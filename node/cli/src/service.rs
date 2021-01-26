// Copyright 2018-2020 Commonwealth Labs, Inc.
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
// along with Edgeware.  If not, see <http://www.gnu.org/licenses/>.

#![warn(unused_extern_crates)]

//! Service implementation. Specialized wrapper over substrate service.

use sc_service::ChainSpec;
use std::{sync::{Arc, Mutex}, collections::HashMap};
use fc_rpc_core::types::PendingTransactions;
use sc_client_api::{ExecutorProvider, RemoteBackend, BlockchainEvents};
use sc_consensus_aura;
use sc_finality_grandpa::{self, FinalityProofProvider as GrandpaFinalityProofProvider};
use fc_consensus::FrontierBlockImport;
use edgeware_primitives::Block;
use edgeware_runtime::RuntimeApi;
use sc_service::{
	config::{Configuration}, error::{Error as ServiceError},
	RpcHandlers, TaskManager,
};
use sp_inherents::InherentDataProviders;
use sc_network::{Event, NetworkService};
use sp_runtime::traits::Block as BlockT;
use futures::prelude::*;
use edgeware_executor::Executor;

type FullClient = sc_service::TFullClient<Block, RuntimeApi, Executor>;
type FullBackend = sc_service::TFullBackend<Block>;
type FullSelectChain = sc_consensus::LongestChain<FullBackend, Block>;
type FullGrandpaBlockImport =
	sc_finality_grandpa::GrandpaBlockImport<FullBackend, Block, FullClient, FullSelectChain>;
type LightClient = sc_service::TLightClient<Block, RuntimeApi, Executor>;

/// Can be called for a `Configuration` to check if it is a configuration for the `Kusama` network.
pub trait IdentifyVariant {
	/// Returns if this is a configuration for the `Kusama` network.
	fn is_mainnet(&self) -> bool;

	/// Returns if this is a configuration for the `Westend` network.
	fn is_beresheet(&self) -> bool;
}

impl IdentifyVariant for Box<dyn ChainSpec> {
	fn is_mainnet(&self) -> bool {
		self.id().starts_with("edgeware") || self.id().starts_with("edg")
	}

	fn is_beresheet(&self) -> bool {
		self.id().starts_with("beresheet") || self.id().starts_with("tedg")
	}
}

pub fn new_partial(config: &Configuration) -> Result<sc_service::PartialComponents<
	FullClient, FullBackend, FullSelectChain,
	sp_consensus::DefaultImportQueue<Block, FullClient>,
	sc_transaction_pool::FullPool<Block, FullClient>,
	(
		sc_consensus_aura::AuraBlockImport<
			Block,
			FullClient,
			FrontierBlockImport<
				Block,
				FullGrandpaBlockImport,
				FullClient
			>,
			sp_consensus_aura::ed25519::AuthorityPair
		>,
		sc_finality_grandpa::LinkHalf<Block, FullClient, FullSelectChain>,
		PendingTransactions,
	)
>, ServiceError> {
	let (client, backend, keystore_container, task_manager) =
		sc_service::new_full_parts::<Block, RuntimeApi, Executor>(&config)?;
	let client = Arc::new(client);

	let select_chain = sc_consensus::LongestChain::new(backend.clone());

	let transaction_pool = sc_transaction_pool::BasicPool::new_full(
		config.transaction_pool.clone(),
		config.prometheus_registry(),
		task_manager.spawn_handle(),
		client.clone(),
	);

	let pending_transactions: PendingTransactions
		= Some(Arc::new(Mutex::new(HashMap::new())));

	let (grandpa_block_import, grandpa_link) = sc_finality_grandpa::block_import(
		client.clone(),
		&(client.clone() as Arc<_>),
		select_chain.clone(),
	)?;

	let frontier_block_import = FrontierBlockImport::new(
		grandpa_block_import.clone(),
		client.clone(),
		true
	);

	let aura_block_import =
		sc_consensus_aura::AuraBlockImport::<_, _, _, sp_consensus_aura::ed25519::AuthorityPair>::new(
			frontier_block_import,
			client.clone()
		);

	let inherent_data_providers = sp_inherents::InherentDataProviders::new();

	let import_queue = sc_consensus_aura::import_queue::<_, _, _, sp_consensus_aura::ed25519::AuthorityPair, _, _>(
		sc_consensus_aura::slot_duration(&*client)?,
		aura_block_import.clone(),
		Some(Box::new(grandpa_block_import.clone())),
		client.clone(),
		inherent_data_providers.clone(),
		&task_manager.spawn_handle(),
		config.prometheus_registry(),
		sp_consensus::CanAuthorWithNativeVersion::new(client.executor().clone()),
	)?;

	Ok(sc_service::PartialComponents {
		client, backend, task_manager, import_queue, keystore_container, select_chain, transaction_pool,
		inherent_data_providers,
		other: (aura_block_import.clone(), grandpa_link, pending_transactions,)
	})
}

/// Creates a full service from the configuration.
pub fn new_full_base(
	mut config: Configuration,
	enable_dev_signer: bool,
	with_startup_data: impl FnOnce(
		&sc_consensus_aura::AuraBlockImport<
			Block,
			FullClient,
			FrontierBlockImport<
				Block,
				FullGrandpaBlockImport,
				FullClient
			>,
			sp_consensus_aura::ed25519::AuthorityPair
		>,
		&sc_finality_grandpa::LinkHalf<Block, FullClient, FullSelectChain>,
	),
) -> Result<NewFullBase, ServiceError> {
	let sc_service::PartialComponents {
		client, backend, mut task_manager, import_queue, keystore_container,
		select_chain, transaction_pool, inherent_data_providers,
		other: (block_import, grandpa_link, pending_transactions),
	} = new_partial(&config)?;

	let (network, network_status_sinks, system_rpc_tx, network_starter) =
		sc_service::build_network(sc_service::BuildNetworkParams {
			config: &config,
			client: client.clone(),
			transaction_pool: transaction_pool.clone(),
			spawn_handle: task_manager.spawn_handle(),
			import_queue,
			on_demand: None,
			block_announce_validator_builder: None,
		})?;

	if config.offchain_worker.enabled {
		sc_service::build_offchain_workers(
			&config, backend.clone(), task_manager.spawn_handle(), client.clone(), network.clone(),
		);
	}

	let role = config.role.clone();
	let force_authoring = config.force_authoring;
	let name = config.network.node_name.clone();
	let enable_grandpa = !config.disable_grandpa;
	let prometheus_registry = config.prometheus_registry().cloned();
	let telemetry_connection_sinks = sc_service::TelemetryConnectionSinks::default();

	let (rpc_extensions_builder, rpc_setup) = {
		let justification_stream = grandpa_link.justification_stream();
		let shared_authority_set = grandpa_link.shared_authority_set().clone();
		let shared_voter_state = sc_finality_grandpa::SharedVoterState::empty();
		let finality_proof_provider =
			GrandpaFinalityProofProvider::new_for_service(backend.clone(), client.clone());

		let rpc_setup = (shared_voter_state.clone(), finality_proof_provider.clone());
		let client = client.clone();
		let pool = transaction_pool.clone();
		let select_chain = select_chain.clone();
		let network = network.clone();
		let pending = pending_transactions.clone();
		let is_authority = config.role.clone().is_authority();
		let _keystore = keystore_container.sync_keystore();
		let subscription_executor = sc_rpc::SubscriptionTaskExecutor::new(task_manager.spawn_handle());

		let rpc_extensions_builder = move |deny_unsafe, _| {
			let deps = edgeware_rpc::FullDeps {
				client: client.clone(),
				pool: pool.clone(),
				select_chain: select_chain.clone(),
				network: network.clone(),
				pending_transactions: pending.clone(),
				is_authority,
				enable_dev_signer,
				deny_unsafe,
				grandpa: edgeware_rpc::GrandpaDeps {
					shared_voter_state: shared_voter_state.clone(),
					shared_authority_set: shared_authority_set.clone(),
					justification_stream: justification_stream.clone(),
					subscription_executor: subscription_executor.clone(),
					finality_provider: finality_proof_provider.clone(),
				},
			};

			edgeware_rpc::create_full(deps, subscription_executor.clone())
		};

		(rpc_extensions_builder, rpc_setup)
	};

	config.network.extra_sets.push(sc_finality_grandpa::grandpa_peers_set_config());

	sc_service::spawn_tasks(sc_service::SpawnTasksParams {
		config,
		backend: backend.clone(),
		client: client.clone(),
		keystore: keystore_container.sync_keystore(),
		network: network.clone(),
		rpc_extensions_builder: Box::new(rpc_extensions_builder),
		transaction_pool: transaction_pool.clone(),
		task_manager: &mut task_manager,
		on_demand: None,
		remote_blockchain: None,
		telemetry_connection_sinks: telemetry_connection_sinks.clone(),
		network_status_sinks: network_status_sinks.clone(),
		system_rpc_tx,
	})?;

	// Spawn Frontier pending transactions maintenance task (as essential, otherwise we leak).
	if pending_transactions.is_some() {
		use futures::StreamExt;
		use fp_consensus::{FRONTIER_ENGINE_ID, ConsensusLog};
		use sp_runtime::generic::OpaqueDigestItemId;

		const TRANSACTION_RETAIN_THRESHOLD: u64 = 5;
		task_manager.spawn_essential_handle().spawn(
			"frontier-pending-transactions",
			client.import_notification_stream().for_each(move |notification| {

				if let Ok(locked) = &mut pending_transactions.clone().unwrap().lock() {
					// As pending transactions have a finite lifespan anyway
					// we can ignore MultiplePostRuntimeLogs error checks.
					let mut frontier_log: Option<_> = None;
					for log in notification.header.digest.logs {
						let log = log.try_to::<ConsensusLog>(OpaqueDigestItemId::Consensus(&FRONTIER_ENGINE_ID));
						if let Some(log) = log {
							frontier_log = Some(log);
						}
					}

					let imported_number: u64 = notification.header.number as u64;

					if let Some(ConsensusLog::EndBlock {
						block_hash: _, transaction_hashes,
					}) = frontier_log {
						// Retain all pending transactions that were not
						// processed in the current block.
						locked.retain(|&k, _| !transaction_hashes.contains(&k));
					}
					locked.retain(|_, v| {
						// Drop all the transactions that exceeded the given lifespan.
						let lifespan_limit = v.at_block + TRANSACTION_RETAIN_THRESHOLD;
						lifespan_limit > imported_number
					});
				}
				futures::future::ready(())
			})
		);
	}

	let (shared_voter_state, _finality_proof_provider) = rpc_setup;

	(with_startup_data)(&block_import, &grandpa_link);

	if let sc_service::config::Role::Authority { .. } = &role {
		let proposer = sc_basic_authorship::ProposerFactory::new(
			task_manager.spawn_handle(),
			client.clone(),
			transaction_pool.clone(),
			prometheus_registry.as_ref(),
		);

		let can_author_with =
			sp_consensus::CanAuthorWithNativeVersion::new(client.executor().clone());

		let backoff_authoring_blocks: Option<()> = None;
		let aura = sc_consensus_aura::start_aura::<_, _, _, _, _, sp_consensus_aura::ed25519::AuthorityPair, _, _, _, _>(
			sc_consensus_aura::slot_duration(&*client)?,
			client.clone(),
			select_chain,
			block_import,
			proposer,
			network.clone(),
			inherent_data_providers.clone(),
			force_authoring,
			backoff_authoring_blocks,
			keystore_container.sync_keystore(),
			can_author_with,
		)?;

		// the AURA authoring task is considered essential, i.e. if it
		// fails we take down the service with it.
		task_manager.spawn_essential_handle().spawn_blocking("aura-proposer", aura);
	}

	// Spawn authority discovery module.
	if role.is_authority() {
		let authority_discovery_role = sc_authority_discovery::Role::PublishAndDiscover(
			keystore_container.keystore(),
		);
		let dht_event_stream = network.event_stream("authority-discovery")
			.filter_map(|e| async move { match e {
				Event::Dht(e) => Some(e),
				_ => None,
			}});
		let (authority_discovery_worker, _service) = sc_authority_discovery::new_worker_and_service(
			client.clone(),
			network.clone(),
			Box::pin(dht_event_stream),
			authority_discovery_role,
			prometheus_registry.clone(),
		);

		task_manager.spawn_handle().spawn("authority-discovery-worker", authority_discovery_worker.run());
	}

	// if the node isn't actively participating in consensus then it doesn't
	// need a keystore, regardless of which protocol we use below.
	let keystore = if role.is_authority() {
		Some(keystore_container.sync_keystore())
	} else {
		None
	};

	let config = sc_finality_grandpa::Config {
		// FIXME #1578 make this available through chainspec
		gossip_duration: std::time::Duration::from_millis(333),
		justification_period: 512,
		name: Some(name),
		observer_enabled: false,
		keystore,
		is_authority: role.is_network_authority(),
	};

	if enable_grandpa {
		// start the full GRANDPA voter
		// NOTE: non-authorities could run the GRANDPA observer protocol, but at
		// this point the full voter should provide better guarantees of block
		// and vote data availability than the observer. The observer has not
		// been tested extensively yet and having most nodes in a network run it
		// could lead to finality stalls.
		let grandpa_config = sc_finality_grandpa::GrandpaParams {
			config,
			link: grandpa_link,
			network: network.clone(),
			telemetry_on_connect: Some(telemetry_connection_sinks.on_connect_stream()),
			voting_rule: sc_finality_grandpa::VotingRulesBuilder::default().build(),
			prometheus_registry,
			shared_voter_state,
		};

		// the GRANDPA voter task is considered infallible, i.e.
		// if it fails we take down the service with it.
		task_manager.spawn_essential_handle().spawn_blocking(
			"grandpa-voter",
			sc_finality_grandpa::run_grandpa_voter(grandpa_config)?
		);
	}

	network_starter.start_network();
	Ok(NewFullBase {
		task_manager, inherent_data_providers, client, network, network_status_sinks,
		transaction_pool,
	})
}

pub struct NewFullBase {
	pub task_manager: TaskManager,
	pub inherent_data_providers: InherentDataProviders,
	pub client: Arc<FullClient>,
	pub network: Arc<NetworkService<Block, <Block as BlockT>::Hash>>,
	pub network_status_sinks: sc_service::NetworkStatusSinks<Block>,
	pub transaction_pool: Arc<sc_transaction_pool::FullPool<Block, FullClient>>,
}

/// Builds a new service for a full client.
pub fn new_full(config: Configuration, enable_dev_signer: bool)
-> Result<TaskManager, ServiceError> {
	new_full_base(config, enable_dev_signer, |_, _| ()).map(|NewFullBase { task_manager, .. }| {
		task_manager
	})
}

pub fn new_light_base(config: Configuration) -> Result<(
	TaskManager, RpcHandlers, Arc<LightClient>,
	Arc<NetworkService<Block, <Block as BlockT>::Hash>>,
	Arc<sc_transaction_pool::LightPool<Block, LightClient, sc_network::config::OnDemand<Block>>>
), ServiceError> {
	let (client, backend, keystore_container, mut task_manager, on_demand) =
		sc_service::new_light_parts::<Block, RuntimeApi, Executor>(&config)?;

	let select_chain = sc_consensus::LongestChain::new(backend.clone());

	let transaction_pool = Arc::new(sc_transaction_pool::BasicPool::new_light(
		config.transaction_pool.clone(),
		config.prometheus_registry(),
		task_manager.spawn_handle(),
		client.clone(),
		on_demand.clone(),
	));

	let (grandpa_block_import, _) = sc_finality_grandpa::block_import(
		client.clone(),
		&(client.clone() as Arc<_>),
		select_chain.clone(),
	)?;

	let aura_block_import = sc_consensus_aura::AuraBlockImport::<_, _, _, sp_consensus_aura::ed25519::AuthorityPair>::new(
		grandpa_block_import.clone(),
		client.clone(),
	);

	let import_queue = sc_consensus_aura::import_queue::<_, _, _, sp_consensus_aura::ed25519::AuthorityPair, _, _>(
		sc_consensus_aura::slot_duration(&*client)?,
		aura_block_import,
		Some(Box::new(grandpa_block_import)),
		client.clone(),
		InherentDataProviders::new(),
		&task_manager.spawn_handle(),
		config.prometheus_registry(),
		sp_consensus::NeverCanAuthor,
	)?;

	let (network, network_status_sinks, system_rpc_tx, network_starter) =
		sc_service::build_network(sc_service::BuildNetworkParams {
			config: &config,
			client: client.clone(),
			transaction_pool: transaction_pool.clone(),
			spawn_handle: task_manager.spawn_handle(),
			import_queue,
			on_demand: Some(on_demand.clone()),
			block_announce_validator_builder: None,
		})?;
	network_starter.start_network();

	if config.offchain_worker.enabled {
		sc_service::build_offchain_workers(
			&config, backend.clone(), task_manager.spawn_handle(), client.clone(), network.clone(),
		);
	}

	let light_deps = edgeware_rpc::LightDeps {
		remote_blockchain: backend.remote_blockchain(),
		fetcher: on_demand.clone(),
		client: client.clone(),
		pool: transaction_pool.clone(),
	};

	let rpc_extensions = edgeware_rpc::create_light(light_deps);

	let rpc_handlers =
		sc_service::spawn_tasks(sc_service::SpawnTasksParams {
			on_demand: Some(on_demand),
			remote_blockchain: Some(backend.remote_blockchain()),
			rpc_extensions_builder: Box::new(sc_service::NoopRpcExtensionBuilder(rpc_extensions)),
			client: client.clone(),
			transaction_pool: transaction_pool.clone(),
			keystore: keystore_container.sync_keystore(),
			config, backend, network_status_sinks, system_rpc_tx,
			network: network.clone(),
			telemetry_connection_sinks: sc_service::TelemetryConnectionSinks::default(),
			task_manager: &mut task_manager,
		})?;

	Ok((task_manager, rpc_handlers, client, network, transaction_pool))
}

/// Builds a new service for a light client.
pub fn new_light(config: Configuration) -> Result<TaskManager, ServiceError> {
	new_light_base(config).map(|(task_manager, _, _, _, _)| {
		task_manager
	})
}

