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

//! Service and ServiceFactory implementation. Specialized wrapper over Substrate service.

#![warn(unused_extern_crates)]

use std::sync::Arc;
use std::time::Duration;

use transaction_pool::{self, txpool::{Pool as TransactionPool}};
use substrate_inherents::InherentDataProviders;
use node_primitives::{Block};
use edgeware_runtime::{self, GenesisConfig, RuntimeApi};
use substrate_service::{
	FactoryFullConfiguration, LightComponents, FullComponents, FullBackend,
	FullClient, LightClient, LightBackend, FullExecutor, LightExecutor,
	TaskExecutor,
};
use node_executor;
use consensus::{import_queue, start_aura, AuraImportQueue, SlotDuration, NothingExtra};
use client;
use grandpa;
use primitives::ed25519::Pair;

pub use substrate_executor::NativeExecutor;
// Our native executor instance.
native_executor_instance!(
	pub Executor,
	edgeware_runtime::api::dispatch,
	edgeware_runtime::native_version,
	include_bytes!("../runtime/wasm/target/wasm32-unknown-unknown/release/edgeware_runtime.wasm")
);

construct_simple_protocol! {
	/// Demo protocol attachment for substrate.
	pub struct NodeProtocol where Block = Block { }
}

/// Node specific configuration
pub struct NodeConfig<F: substrate_service::ServiceFactory> {
	/// grandpa connection to import block
	// FIXME #1134 rather than putting this on the config, let's have an actual intermediate setup state
	pub grandpa_import_setup: Option<(Arc<grandpa::BlockImportForService<F>>, grandpa::LinkHalfForService<F>)>,
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
		RuntimeDispatch = node_executor::Executor,
		FullTransactionPoolApi = transaction_pool::ChainApi<client::Client<FullBackend<Self>, FullExecutor<Self>, Block, RuntimeApi>, Block>
			{ |config, client| Ok(TransactionPool::new(config, transaction_pool::ChainApi::new(client))) },
		LightTransactionPoolApi = transaction_pool::ChainApi<client::Client<LightBackend<Self>, LightExecutor<Self>, Block, RuntimeApi>, Block>
			{ |config, client| Ok(TransactionPool::new(config, transaction_pool::ChainApi::new(client))) },
		Genesis = GenesisConfig,
		Configuration = NodeConfig<Self>,
		FullService = FullComponents<Self>
			{ |config: FactoryFullConfiguration<Self>, executor: TaskExecutor|
				FullComponents::<Factory>::new(config, executor) },
		AuthoritySetup = {
			|mut service: Self::FullService, executor: TaskExecutor, local_key: Option<Arc<Pair>>| {
				let (block_import, link_half) = service.config.custom.grandpa_import_setup.take()
					.expect("Link Half and Block Import are present for Full Services or setup failed before. qed");

				if let Some(ref key) = local_key {
					info!("Using authority key {}", key.public());
					let proposer = Arc::new(substrate_basic_authorship::ProposerFactory {
						client: service.client(),
						transaction_pool: service.transaction_pool(),
					});

					let client = service.client();
					executor.spawn(start_aura(
						SlotDuration::get_or_compute(&*client)?,
						key.clone(),
						client,
						block_import.clone(),
						proposer,
						service.network(),
						service.on_exit(),
						service.config.custom.inherent_data_providers.clone(),
					)?);

					info!("Running Grandpa session as Authority {}", key.public());
				}

				executor.spawn(grandpa::run_grandpa(
					grandpa::Config {
						local_key,
						// FIXME #1578 make this available through chainspec
						gossip_duration: Duration::new(4, 0),
						justification_period: 4096,
						name: Some(service.config.name.clone())
					},
					link_half,
					grandpa::NetworkBridge::new(service.network()),
					service.config.custom.inherent_data_providers.clone(),
					service.on_exit(),
				)?);

				Ok(service)
			}
		},
		LightService = LightComponents<Self>
			{ |config, executor| <LightComponents<Factory>>::new(config, executor) },
		FullImportQueue = AuraImportQueue<Self::Block>
			{ |config: &mut FactoryFullConfiguration<Self> , client: Arc<FullClient<Self>>| {
				let slot_duration = SlotDuration::get_or_compute(&*client)?;
				let (block_import, link_half) =
					grandpa::block_import::<_, _, _, RuntimeApi, FullClient<Self>>(
						client.clone(), client.clone()
					)?;
				let block_import = Arc::new(block_import);
				let justification_import = block_import.clone();

				config.custom.grandpa_import_setup = Some((block_import.clone(), link_half));

				import_queue(
					slot_duration,
					block_import,
					Some(justification_import),
					client,
					NothingExtra,
					config.custom.inherent_data_providers.clone(),
				).map_err(Into::into)
			}},
		LightImportQueue = AuraImportQueue<Self::Block>
			{ |config: &FactoryFullConfiguration<Self>, client: Arc<LightClient<Self>>| {
					import_queue(
						SlotDuration::get_or_compute(&*client)?,
						client.clone(),
						None,
						client,
						NothingExtra,
						config.custom.inherent_data_providers.clone(),
					).map_err(Into::into)
				}
			},
	}
}
