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
use std::time::Duration;

use aura::{import_queue, start_aura, AuraImportQueue, SlotDuration};
use client::{self, LongestChain};
use grandpa::{self, FinalityProofProvider as GrandpaFinalityProofProvider};
use edgeware_executor;
use primitives::Pair;
use futures::prelude::*;
use edgeware_primitives::{AuraPair, Block};
use edgeware_runtime::{GenesisConfig, RuntimeApi};
use substrate_service::{
	FactoryFullConfiguration, LightComponents, FullComponents, FullBackend,
	FullClient, LightClient, LightBackend, FullExecutor, LightExecutor,
	error::{Error as ServiceError},
};
use transaction_pool::{self, txpool::{Pool as TransactionPool}};
use inherents::InherentDataProviders;
use network::construct_simple_protocol;
use substrate_service::construct_service_factory;
use log::info;
use substrate_service::TelemetryOnConnect;

pub mod chain_spec;
pub mod fixtures;

construct_simple_protocol! {
	/// Demo protocol attachment for substrate.
	pub struct NodeProtocol where Block = Block { }
}

/// Node specific configuration
pub struct NodeConfig<F: substrate_service::ServiceFactory> {
	/// grandpa connection to import block
	// FIXME #1134 rather than putting this on the config, let's have an actual intermediate setup state
	pub grandpa_import_setup: Option<(grandpa::BlockImportForService<F>, grandpa::LinkHalfForService<F>)>,
	inherent_data_providers: InherentDataProviders,
}

impl<F> Default for NodeConfig<F> where F: substrate_service::ServiceFactory {
	fn default() -> NodeConfig<F> {
		NodeConfig {
			grandpa_import_setup: None,
			inherent_data_providers: InherentDataProviders::new(),
		}
	}
}

construct_service_factory! {
	struct Factory {
		Block = Block,
		RuntimeApi = RuntimeApi,
		NetworkProtocol = NodeProtocol { |config| Ok(NodeProtocol::new()) },
		RuntimeDispatch = edgeware_executor::Executor,
		FullTransactionPoolApi = transaction_pool::ChainApi<client::Client<FullBackend<Self>, FullExecutor<Self>, Block, RuntimeApi>, Block>
			{ |config, client| Ok(TransactionPool::new(config, transaction_pool::ChainApi::new(client))) },
		LightTransactionPoolApi = transaction_pool::ChainApi<client::Client<LightBackend<Self>, LightExecutor<Self>, Block, RuntimeApi>, Block>
			{ |config, client| Ok(TransactionPool::new(config, transaction_pool::ChainApi::new(client))) },
		Genesis = GenesisConfig,
		Configuration = NodeConfig<Self>,
		FullService = FullComponents<Self>
			{ |config: FactoryFullConfiguration<Self>|
				FullComponents::<Factory>::new(config) },
		AuthoritySetup = {
			|mut service: Self::FullService| {
				let (block_import, link_half) = service.config.custom.grandpa_import_setup.take()
					.expect("Link Half and Block Import are present for Full Services or setup failed before. qed");

				if let Some(aura_key) = service.authority_key::<AuraPair>() {
					info!("Using aura key {}", aura_key.public());

					let proposer = Arc::new(substrate_basic_authorship::ProposerFactory {
						client: service.client(),
						transaction_pool: service.transaction_pool(),
					});

					let client = service.client();
					let select_chain = service.select_chain()
						.ok_or(ServiceError::SelectChainRequired)?;

					let aura = start_aura(
						SlotDuration::get_or_compute(&*client)?,
						Arc::new(aura_key),
						client,
						select_chain,
						block_import,
						proposer,
						service.network(),
						service.config.custom.inherent_data_providers.clone(),
						service.config.force_authoring,
					)?;
					let select = aura.select(service.on_exit()).then(|_| Ok(()));
					service.spawn_task(Box::new(select));
				}

				let grandpa_key = if service.config.disable_grandpa {
					None
				} else {
					service.authority_key::<grandpa_primitives::AuthorityPair>()
				};

				let config = grandpa::Config {
					local_key: grandpa_key.map(Arc::new),
					// FIXME #1578 make this available through chainspec
					gossip_duration: Duration::from_millis(333),
					justification_period: 4096,
					name: Some(service.config.name.clone())
				};

				match config.local_key {
					None if !service.config.grandpa_voter => {
						service.spawn_task(Box::new(grandpa::run_grandpa_observer(
							config,
							link_half,
							service.network(),
							service.on_exit(),
						)?));
					},
					// Either config.local_key is set, or user forced voter service via `--grandpa-voter` flag.
					_ => {
						let telemetry_on_connect = TelemetryOnConnect {
							telemetry_connection_sinks: service.telemetry_on_connect_stream(),
						};
						let grandpa_config = grandpa::GrandpaParams {
							config: config,
							link: link_half,
							network: service.network(),
							inherent_data_providers: service.config.custom.inherent_data_providers.clone(),
							on_exit: service.on_exit(),
							telemetry_on_connect: Some(telemetry_on_connect),
						};
						service.spawn_task(Box::new(grandpa::run_grandpa_voter(grandpa_config)?));
					},
				}

				Ok(service)
			}
		},
		LightService = LightComponents<Self>
			{ |config| <LightComponents<Factory>>::new(config) },
		FullImportQueue = AuraImportQueue<Self::Block>
			{ |config: &mut FactoryFullConfiguration<Self> , client: Arc<FullClient<Self>>, select_chain: Self::SelectChain| {
				let slot_duration = SlotDuration::get_or_compute(&*client)?;
				let (block_import, link_half) =
					grandpa::block_import::<_, _, _, RuntimeApi, FullClient<Self>, _>(
						client.clone(), client.clone(), select_chain
					)?;
				let justification_import = block_import.clone();

				config.custom.grandpa_import_setup = Some((block_import.clone(), link_half));

				import_queue::<_, _, AuraPair>(
					slot_duration,
					Box::new(block_import),
					Some(Box::new(justification_import)),
					None,
					client,
					config.custom.inherent_data_providers.clone(),
				).map_err(Into::into)
			}},
		LightImportQueue = AuraImportQueue<Self::Block>
			{ |config: &FactoryFullConfiguration<Self>, client: Arc<LightClient<Self>>| {
				#[allow(deprecated)]
				let fetch_checker = client.backend().blockchain().fetcher()
					.upgrade()
					.map(|fetcher| fetcher.checker().clone())
					.ok_or_else(|| "Trying to start light import queue without active fetch checker")?;
				let block_import = grandpa::light_block_import::<_, _, _, RuntimeApi, LightClient<Self>>(
					client.clone(), Arc::new(fetch_checker), client.clone()
				)?;
				let finality_proof_import = block_import.clone();
				let finality_proof_request_builder = finality_proof_import.create_finality_proof_request_builder();

				import_queue::<_, _, AuraPair>(
					SlotDuration::get_or_compute(&*client)?,
					Box::new(block_import),
					None,
					Some(Box::new(finality_proof_import)),
					client,
					config.custom.inherent_data_providers.clone(),
				).map(|q| (q, finality_proof_request_builder)).map_err(Into::into)
			}},
		SelectChain = LongestChain<FullBackend<Self>, Self::Block>
			{ |config: &FactoryFullConfiguration<Self>, client: Arc<FullClient<Self>>| {
				#[allow(deprecated)]
				Ok(LongestChain::new(client.backend().clone()))
			}
		},
		FinalityProofProvider = { |client: Arc<FullClient<Self>>| {
			Ok(Some(Arc::new(GrandpaFinalityProofProvider::new(client.clone(), client)) as _))
		}},
	}
}
