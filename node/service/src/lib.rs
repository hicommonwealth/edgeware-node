// Copyright 2018 Commonwealth Labs, Inc.
// This file is part of Edgeware.

// Edgeware is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Edgeware is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Edgeware.  If not, see <http://www.gnu.org/licenses/>

#![warn(unused_extern_crates)]
use std::sync::Arc;


use aura::{import_queue, start_aura, SlotDuration};
use client::{self, LongestChain};
use grandpa::{self, FinalityProofProvider as GrandpaFinalityProofProvider};
use edgeware_executor;
use edgeware_primitives::{Block};
use edgeware_runtime::{GenesisConfig, RuntimeApi};
use substrate_service::{
	AbstractService, ServiceBuilder, config::Configuration, error::{Error as ServiceError},
};
use transaction_pool::{self, txpool::{Pool as TransactionPool}};
use inherents::InherentDataProviders;
use network::construct_simple_protocol;

use aura_primitives::ed25519::AuthorityPair as AuraAuthorityPair;

pub mod chain_spec;
pub mod mainnet_fixtures;
pub mod testnet_fixtures;

construct_simple_protocol! {
	/// Demo protocol attachment for substrate.
	pub struct NodeProtocol where Block = Block { }
}

/// Starts a `ServiceBuilder` for a full service.
///
/// Use this macro if you don't actually need the full service, but just the builder in order to
/// be able to perform chain operations.
#[macro_export]
macro_rules! new_full_start {
	($config:expr) => {{
		let mut import_setup = None;
		let inherent_data_providers = inherents::InherentDataProviders::new();

		let builder = substrate_service::ServiceBuilder::new_full::<
			edgeware_primitives::Block, edgeware_runtime::RuntimeApi, edgeware_executor::Executor
		>($config)?
			.with_select_chain(|_config, backend| {
				Ok(client::LongestChain::new(backend.clone()))
			})?
			.with_transaction_pool(|config, client|
				Ok(transaction_pool::txpool::Pool::new(config, transaction_pool::ChainApi::new(client)))
			)?
			.with_import_queue(|_config, client, mut select_chain, transaction_pool| {
				let select_chain = select_chain.take()
					.ok_or_else(|| substrate_service::Error::SelectChainRequired)?;
				let (block_import, link_half) =
					grandpa::block_import::<_, _, _, edgeware_runtime::RuntimeApi, _, _>(
						client.clone(), &*client, select_chain
					)?;
				let justification_import = block_import.clone();

				let import_queue = import_queue::<_, _, AuraAuthorityPair, _>(
					SlotDuration::get_or_compute(&*client)?,
					Box::new(block_import.clone()),
					Some(Box::new(justification_import)),
					None,
					client.clone(),
					inherent_data_providers.clone(),
					Some(transaction_pool),
				)?;

				import_setup = Some((block_import.clone(), link_half));
				Ok(import_queue)
			})?
			.with_rpc_extensions(|client, pool| {
				use edgeware_rpc::accounts::{Accounts, AccountsApi};

				let mut io = jsonrpc_core::IoHandler::<substrate_service::RpcMetadata>::default();
				io.extend_with(
					AccountsApi::to_delegate(Accounts::new(client, pool))
				);
				io
			})?;

		(builder, import_setup, inherent_data_providers)
	}}
}

/// Creates a full service from the configuration.
///
/// We need to use a macro because the test suit doesn't work with an opaque service. It expects
/// concrete types instead.
#[macro_export]
macro_rules! new_full {
	($config:expr) => {{
		use futures::Future;

		let (
			is_authority,
			force_authoring,
			name,
			disable_grandpa
		) = (
			$config.roles.is_authority(),
			$config.force_authoring,
			$config.name.clone(),
			$config.disable_grandpa
		);

		let (builder, mut import_setup, inherent_data_providers) = new_full_start!($config);

		let service = builder.with_network_protocol(|_| Ok(crate::NodeProtocol::new()))?
			.with_finality_proof_provider(|client, backend|
				Ok(Arc::new(grandpa::FinalityProofProvider::new(backend, client)) as _)
			)?
			.build()?;

		let (block_import, link_half) = import_setup.take()
			.expect("Link Half and Block Import are present for Full Services or setup failed before. qed");

		if is_authority {
			let proposer = substrate_basic_authorship::ProposerFactory {
				client: service.client(),
				transaction_pool: service.transaction_pool(),
			};

			let client = service.client();
			let select_chain = service.select_chain()
				.ok_or(substrate_service::Error::SelectChainRequired)?;

			let aura = start_aura::<_, _, _, _, _, AuraAuthorityPair, _, _, _>(
				SlotDuration::get_or_compute(&*client)?,
				client.clone(),
				select_chain,
				block_import,
				proposer,
				service.network(),
				inherent_data_providers.clone(),
				force_authoring,
				service.keystore(),
			)?;
			let select = aura.select(service.on_exit()).then(|_| Ok(()));
			service.spawn_task(Box::new(select));
		}

		let config = grandpa::Config {
			// FIXME #1578 make this available through chainspec
			gossip_duration: std::time::Duration::from_millis(333),
			justification_period: 4096,
			name: Some(name.clone()),
			keystore: Some(service.keystore()),
		};

		match (is_authority, disable_grandpa) {
			(false, false) => {
				// start the lightweight GRANDPA observer
				service.spawn_task(Box::new(grandpa::run_grandpa_observer(
					config,
					link_half,
					service.network(),
					service.on_exit(),
				)?));
			},
			(true, false) => {
				// start the full GRANDPA voter
				let grandpa_config = grandpa::GrandpaParams {
					config: config,
					link: link_half,
					network: service.network(),
					inherent_data_providers: inherent_data_providers.clone(),
					on_exit: service.on_exit(),
					telemetry_on_connect: Some(service.telemetry_on_connect_stream()),
				};
				service.spawn_task(Box::new(grandpa::run_grandpa_voter(grandpa_config)?));
			},
			(_, true) => {
				grandpa::setup_disabled_grandpa(
					service.client(),
					&inherent_data_providers,
					service.network(),
				)?;
			},
		}

		Ok((service, inherent_data_providers))
	}}
}

/// Builds a new service for a full client.
pub fn new_full<C: Send + Default + 'static>(config: Configuration<C, GenesisConfig>)
-> Result<impl AbstractService, ServiceError> {
	new_full!(config).map(|(service, _)| service)
}

/// Builds a new service for a light client.
pub fn new_light<C: Send + Default + 'static>(config: Configuration<C, GenesisConfig>)
-> Result<impl AbstractService, ServiceError> {
	let inherent_data_providers = InherentDataProviders::new();

	ServiceBuilder::new_light::<Block, RuntimeApi, edgeware_executor::Executor>(config)?
		.with_select_chain(|_config, backend| {
			Ok(LongestChain::new(backend.clone()))
		})?
		.with_transaction_pool(|config, client|
			Ok(TransactionPool::new(config, transaction_pool::ChainApi::new(client)))
		)?
		.with_import_queue_and_fprb(|_config, client, backend, fetcher, _select_chain, transaction_pool| {
			let fetch_checker = fetcher
				.map(|fetcher| fetcher.checker().clone())
				.ok_or_else(|| "Trying to start light import queue without active fetch checker")?;
			let block_import = grandpa::light_block_import::<_, _, _, RuntimeApi, _>(
				client.clone(), backend, Arc::new(fetch_checker), client.clone()
			)?;

			let finality_proof_import = block_import.clone();
			let finality_proof_request_builder =
				finality_proof_import.create_finality_proof_request_builder();

			let import_queue = import_queue::<_, _, AuraAuthorityPair, _>(
				SlotDuration::get_or_compute(&*client)?,
				Box::new(block_import),
				None,
				Some(Box::new(finality_proof_import)),
				client,
				inherent_data_providers.clone(),
				Some(transaction_pool),
			)?;

			Ok((import_queue, finality_proof_request_builder))
		})?
		.with_network_protocol(|_| Ok(crate::NodeProtocol::new()))?
		.with_finality_proof_provider(|client, backend|
			Ok(Arc::new(GrandpaFinalityProofProvider::new(backend, client)) as _)
		)?
		.with_rpc_extensions(|client, pool| {
			use edgeware_rpc::accounts::{Accounts, AccountsApi};

			let mut io = jsonrpc_core::IoHandler::default();
			io.extend_with(
				AccountsApi::to_delegate(Accounts::new(client, pool))
			);
			io
		})?
		.build()
}
