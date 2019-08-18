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
use grandpa_primitives::AuthorityPair as GrandpaPair;
use aura_primitives::sr25519::AuthorityPair as AuraAuthorityPair;

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
	/// Tasks that were created by previous setup steps and should be spawned.
	pub tasks_to_spawn: Option<Vec<Box<dyn Future<Item = (), Error = ()> + Send>>>,
	inherent_data_providers: InherentDataProviders,
}

impl<F> Default for NodeConfig<F> where F: substrate_service::ServiceFactory {
	fn default() -> NodeConfig<F> {
		NodeConfig {
			grandpa_import_setup: None,
			inherent_data_providers: InherentDataProviders::new(),
			tasks_to_spawn: None,
		}
	}
}

construct_service_factory! {
	struct Factory {
		Block = Block,
		RuntimeApi = RuntimeApi,
		NetworkProtocol = NodeProtocol { |config| Ok(NodeProtocol::new()) },
		RuntimeDispatch = edgeware_executor::Executor,
		FullTransactionPoolApi = transaction_pool::ChainApi<
			client::Client<FullBackend<Self>, FullExecutor<Self>, Block, RuntimeApi>,
			Block
		> {
			|config, client| Ok(TransactionPool::new(config, transaction_pool::ChainApi::new(client)))
		},
		LightTransactionPoolApi = transaction_pool::ChainApi<
			client::Client<LightBackend<Self>, LightExecutor<Self>, Block, RuntimeApi>,
			Block
		> {
			|config, client| Ok(TransactionPool::new(config, transaction_pool::ChainApi::new(client)))
		},
		Genesis = GenesisConfig,
		Configuration = NodeConfig<Self>,
		FullService = FullComponents<Self>
			{ |config: FactoryFullConfiguration<Self>|
				FullComponents::<Factory>::new(config) },
		AuthoritySetup = {
			|mut service: Self::FullService| {
				let (block_import, link_half) = service.config().custom.grandpa_import_setup.take()
					.expect("Link Half and Block Import are present for Full Services or setup failed before. qed");

				// spawn any futures that were created in the previous setup steps
				if let Some(tasks) = service.config_mut().custom.tasks_to_spawn.take() {
					for task in tasks {
						service.spawn_task(
							task.select(service.on_exit())
								.map(|_| ())
								.map_err(|_| ())
						);
					}
				}

				if service.config().roles.is_authority() {
					// info!("Using aura key {}", aura_key.public());

					let proposer = Arc::new(substrate_basic_authorship::ProposerFactory {
						client: service.client(),
						transaction_pool: service.transaction_pool(),
					});

					let client = service.client();
					let select_chain = service.select_chain()
						.ok_or(ServiceError::SelectChainRequired)?;

					let aura = start_aura::<_, _, _, _, _, AuraAuthorityPair, _, _, _>(
						SlotDuration::get_or_compute(&*client)?,
						client.clone(),
						select_chain,
						client,
						proposer,
						service.network(),
						service.config().custom.inherent_data_providers.clone(),
						service.config().force_authoring,
						Some(service.keystore()),
					)?;
					let select = aura.select(service.on_exit()).then(|_| Ok(()));
					service.spawn_task(Box::new(select));
				}

				let grandpa_key = if service.config().disable_grandpa {
					None
				} else {
					service.authority_key()
				};

				let config = grandpa::Config {
					// FIXME #1578 make this available through chainspec
					gossip_duration: Duration::from_millis(333),
					justification_period: 4096,
					name: Some(service.config().name.clone()),
					keystore: Some(service.keystore()),
				};

				match (service.config().roles.is_authority(), service.config().disable_grandpa) {
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
						let telemetry_on_connect = TelemetryOnConnect {
							telemetry_connection_sinks: service.telemetry_on_connect_stream(),
						};
						let grandpa_config = grandpa::GrandpaParams {
							config: config,
							link: link_half,
							network: service.network(),
							inherent_data_providers:
								service.config().custom.inherent_data_providers.clone(),
							on_exit: service.on_exit(),
							telemetry_on_connect: Some(telemetry_on_connect),
						};

						// the GRANDPA voter task is considered infallible, i.e.
						// if it fails we take down the service with it.
						service.spawn_essential_task(grandpa::run_grandpa_voter(grandpa_config)?);
					},
					(_, true) => {
						grandpa::setup_disabled_grandpa(
							service.client(),
							&service.config().custom.inherent_data_providers,
							service.network(),
						)?;
					},
				}

				Ok(service)
			}
		},
		LightService = LightComponents<Self>
			{ |config| <LightComponents<Factory>>::new(config) },
		FullImportQueue = AuraImportQueue<Self::Block>
			{
				|
					config: &mut FactoryFullConfiguration<Self>,
					client: Arc<FullClient<Self>>,
					select_chain: Self::SelectChain,
					transaction_pool: Option<Arc<TransactionPool<Self::FullTransactionPoolApi>>>,
				|
			{
				let (block_import, link_half) =
					grandpa::block_import::<_, _, _, RuntimeApi, FullClient<Self>, _>(
						client.clone(), client.clone(), select_chain
					)?;
				let justification_import = block_import.clone();

				let (import_queue, ..) = import_queue::<_, _, AuraAuthorityPair, _>(
					SlotDuration::get_or_compute(&*client)?,
					block_import,
					Some(Box::new(justification_import)),
					None,
					client.clone(),
					config.custom.inherent_data_providers.clone(),
					transaction_pool,
				)?;

				Ok(import_queue)
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
				let finality_proof_request_builder =
					finality_proof_import.create_finality_proof_request_builder();

				// FIXME: pruning task isn't started since light client doesn't do `AuthoritySetup`.
				let (import_queue, ..) = import_queue::<_, _, _, _, _, _, TransactionPool<Self::FullTransactionPoolApi>>(
					SlotDuration::get_or_compute(&*client)?,
					block_import,
					None,
					Some(Box::new(finality_proof_import)),
					client.clone(),
					client,
					config.custom.inherent_data_providers.clone(),
					None,
				)?;

				Ok((import_queue, finality_proof_request_builder))
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
