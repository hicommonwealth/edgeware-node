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

use crate::Cli;
use edgeware_cli_opt::{EthApi as EthApiCmd, RpcConfig};
use edgeware_primitives::Block;
use fc_db::DatabaseSource;
use edgeware_runtime::RuntimeApi;
// use maplit::hashmap;
#[cfg(feature = "frontier-block-import")]
use fc_consensus::FrontierBlockImport;
use sc_client_api::BlockBackend;
use fc_rpc_core::types::{FeeHistoryCache, FilterPool};
use futures::prelude::*;
use sc_consensus_aura::{self, ImportQueueParams, SlotProportion, StartAuraParams};
use sc_network::{Event, NetworkService};
use sc_service::{config::{Configuration, /*PrometheusConfig*/}, error::Error as ServiceError, RpcHandlers,BasePath, ChainSpec, TaskManager};
use sc_telemetry::{Telemetry, TelemetryWorker, TelemetryWorkerHandle};
//use sp_consensus::SlotData;
use sp_core::U256;
use sp_runtime::traits::Block as BlockT;
use std::{
	collections::BTreeMap,
	str::FromStr,
	sync::{Arc, Mutex},
};
use sc_client_api::ExecutorProvider;
// use substrate_prometheus_endpoint::Registry;

type NEWEExecutor = edgeware_executor::NativeElseWasmExecutor<edgeware_executor::EdgewareExecutor>;
type FullClient = sc_service::TFullClient<Block, RuntimeApi, NEWEExecutor>;
type FullBackend = sc_service::TFullBackend<Block>;
type FullSelectChain = sc_consensus::LongestChain<FullBackend, Block>;
type FullGrandpaBlockImport = sc_finality_grandpa::GrandpaBlockImport<FullBackend, Block, FullClient, FullSelectChain>;

/// The transaction pool type defintion.
pub type TransactionPool = sc_transaction_pool::FullPool<Block, FullClient>;

#[cfg(not(feature = "frontier-block-import"))]
pub type ConsensusResult = (
	FullGrandpaBlockImport,
	sc_finality_grandpa::LinkHalf<Block, FullClient, FullSelectChain>,
);

#[cfg(feature = "frontier-block-import")]
pub type ConsensusResult = (
	FrontierBlockImport<
		Block,
		FullGrandpaBlockImport,
		FullClient,
	>,
	sc_finality_grandpa::LinkHalf<Block, FullClient, FullSelectChain>,
);

/// Can be called for a `Configuration` to check if it is a configuration for
/// the `Kusama` network.
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

pub fn frontier_database_dir(config: &Configuration, path: &str) -> std::path::PathBuf {
	let config_dir = config
		.base_path
		.as_ref()
		.map(|base_path| base_path.config_dir(config.chain_spec.id()))
		.unwrap_or_else(|| {
			BasePath::from_project("", "", "moonbeam").config_dir(config.chain_spec.id())
		});
	config_dir.join("frontier").join(path)
}

// TODO This is copied from frontier. It should be imported instead after
// https://github.com/paritytech/frontier/issues/333 is solved
pub fn open_frontier_backend(config: &Configuration) -> Result<Arc<fc_db::Backend<Block>>, String> {
	Ok(Arc::new(fc_db::Backend::<Block>::new(
		&fc_db::DatabaseSettings {
			source: match config.database {
				DatabaseSource::RocksDb { .. } => DatabaseSource::RocksDb {
					path: frontier_database_dir(&config, "db"),
					cache_size: 0,
				},
				DatabaseSource::ParityDb { .. } => DatabaseSource::ParityDb {
					path: frontier_database_dir(&config, "paritydb"),
				},
				DatabaseSource::Auto { .. } => DatabaseSource::Auto {
					rocksdb_path: frontier_database_dir(&config, "db"),
					paritydb_path: frontier_database_dir(&config, "paritydb"),
					cache_size: 0,
				},
				_ => {
					return Err("Supported db sources: `rocksdb` | `paritydb` | `auto`".to_string())
				}
			},
		},
	)?))
}

// // If we're using prometheus, use a registry with a prefix of `edgeware`.
// fn set_prometheus_registry(config: &mut Configuration) -> Result<(), ServiceError> {
// 	if let Some(PrometheusConfig { registry, .. }) = config.prometheus_config.as_mut() {
// 		let labels = hashmap! {
// 			"chain".into() => config.chain_spec.id().into(),
// 		};
// 		*registry = Registry::new_custom(Some("edgeware".into()), Some(labels))?;
// 	}

// 	Ok(())
// }


/// Builds the PartialComponents for development service
///
/// Use this function if you don't actually need the full service, but just the partial in order to
/// be able to perform chain operations.
pub fn new_partial(
	config: &Configuration,
	cli: &Cli,
) -> Result<
	sc_service::PartialComponents<
		FullClient,
		FullBackend,
		FullSelectChain,
		sc_consensus::DefaultImportQueue<Block, FullClient>,
		sc_transaction_pool::FullPool<Block, FullClient>,
		(
			ConsensusResult,
			Option<FilterPool>,
			Arc<fc_db::Backend<Block>>,
			Option<Telemetry>,
			Option<TelemetryWorkerHandle>,
			FeeHistoryCache,
		),
	>,
	ServiceError,
> {
	// set_prometheus_registry(config)?;

	let telemetry = config
		.telemetry_endpoints
		.clone()
		.filter(|x| !x.is_empty())
		.map(|endpoints| -> Result<_, sc_telemetry::Error> {
			let worker = TelemetryWorker::new(16)?;
			let telemetry = worker.handle().new_telemetry(endpoints);
			Ok((worker, telemetry))
		})
		.transpose()?;

	let executor = NEWEExecutor::new(
		config.wasm_method,
		config.default_heap_pages,
		config.max_runtime_instances,
		config.runtime_cache_size,
	);

	let (client, backend, keystore_container, task_manager) =
		sc_service::new_full_parts::<Block, RuntimeApi, _>(
			config,
			telemetry.as_ref().map(|(_, telemetry)| telemetry.handle()),
			executor,
		)?;

	let client = Arc::new(client);

	let telemetry_worker_handle = telemetry.as_ref().map(|(worker, _)| worker.handle());

	let telemetry = telemetry.map(|(worker, telemetry)| {
		task_manager
			.spawn_handle()
			.spawn("telemetry", None, worker.run());
		telemetry
	});

	let select_chain = sc_consensus::LongestChain::new(backend.clone());

	let transaction_pool = sc_transaction_pool::BasicPool::new_full(
		config.transaction_pool.clone(),
		config.role.is_authority().into(),
		config.prometheus_registry(),
		task_manager.spawn_essential_handle(),
		client.clone(),
	);

	let filter_pool: Option<FilterPool> = Some(Arc::new(Mutex::new(BTreeMap::new())));
	let fee_history_cache: FeeHistoryCache = Arc::new(Mutex::new(BTreeMap::new()));

	let frontier_backend = open_frontier_backend(config)?;

	let (grandpa_block_import, grandpa_link) = sc_finality_grandpa::block_import(
		client.clone(),
		&(client.clone() as Arc<_>),
		select_chain.clone(),
		telemetry.as_ref().map(|x| x.handle()),
	)?;

	#[cfg(feature = "frontier-block-import")]
	let frontier_block_import =
		FrontierBlockImport::new(grandpa_block_import.clone(), client.clone(), frontier_backend.clone());

	let justification_import = grandpa_block_import.clone();

	let slot_duration = sc_consensus_aura::slot_duration(&*client)?;//.as_duration();
	let target_gas_price = U256::from(cli.run.target_gas_price);

	let import_queue =
		sc_consensus_aura::import_queue::<sp_consensus_aura::ed25519::AuthorityPair, _, _, _, _, _, _>(
	    ImportQueueParams {
		    #[cfg(feature = "frontier-block-import")]
		    block_import: frontier_block_import.clone(),
		    #[cfg(not(feature = "frontier-block-import"))]
		    block_import: grandpa_block_import.clone(),
		    justification_import: Some(Box::new(justification_import)),
		    client: client.clone(),
		    create_inherent_data_providers: move |_, ()| async move {
			    let timestamp = sp_timestamp::InherentDataProvider::from_system_time();

			    let slot =
				    sp_consensus_aura::inherents::InherentDataProvider::from_timestamp_and_slot_duration(
					    *timestamp,
					    slot_duration,
				    );

    //			let uncles =
    //				sp_authorship::InherentDataProvider::<<Block as BlockT>::Header>::check_inherents();

			    let dynamic_fee =
				    pallet_dynamic_fee::InherentDataProvider(U256::from(target_gas_price));

			    Ok((timestamp, slot, /*uncles,*/ dynamic_fee))
		    },
		    spawner: &task_manager.spawn_essential_handle(),
		    registry: config.prometheus_registry(),
		    can_author_with: sp_consensus::CanAuthorWithNativeVersion::new(client.executor().clone()),
		    check_for_equivocation: Default::default(),
		    telemetry: telemetry.as_ref().map(|x| x.handle()),
		}
	)?;

    #[cfg(feature = "frontier-block-import")]
    let import_setup = (frontier_block_import.clone(), grandpa_link);
    #[cfg(not(feature = "frontier-block-import"))]
    let import_setup = (grandpa_block_import.clone(), grandpa_link);

	Ok(sc_service::PartialComponents {
		client,
		backend,
		task_manager,
		keystore_container,
		select_chain,
		import_queue,
		transaction_pool,
		other: (
			import_setup,
			filter_pool,
			frontier_backend,
			telemetry,
			telemetry_worker_handle,
			fee_history_cache,
		),
	})
}

/// Result of [`new_full_base`].
pub struct NewFullBase {
	/// The task manager of the node.
	pub task_manager: TaskManager,
	/// The client instance of the node.
	pub client: Arc<FullClient>,
	/// The networking service of the node.
	pub network: Arc<NetworkService<Block, <Block as BlockT>::Hash>>,
	/// The transaction pool of the node.
	pub transaction_pool: Arc<TransactionPool>,
	/// The rpc handlers of the node.
	pub rpc_handlers: RpcHandlers,
}

/// Creates a full service from the configuration.
pub fn new_full_base(mut config: Configuration,
	cli: &Cli, 
	rpc_config: RpcConfig
) -> Result<NewFullBase, ServiceError> {
	let sc_service::PartialComponents {
		client,
		backend,
		mut task_manager,
		import_queue,
		keystore_container,
		select_chain,
		transaction_pool,
		other: (import_setup, filter_pool, frontier_backend,
			mut telemetry, _telementry_worker_handle, _fee_history_cache),
	} = new_partial(&config, cli)?;

	let auth_disc_publish_non_global_ips = config.network.allow_non_globals_in_dht;
	let grandpa_protocol_name = sc_finality_grandpa::protocol_standard_name(
		&client.block_hash(0).ok().flatten().expect("Genesis block exists; qed"),
		&config.chain_spec,
	);

	config
		.network
		.extra_sets
		.push(sc_finality_grandpa::grandpa_peers_set_config(grandpa_protocol_name.clone()));
	let warp_sync = Arc::new(sc_finality_grandpa::warp_proof::NetworkProvider::new(
		backend.clone(),
		import_setup.1.shared_authority_set().clone(),
		Vec::default(),
	));

	let (network, system_rpc_tx, network_starter) =
		sc_service::build_network(sc_service::BuildNetworkParams {
			config: &config,
			client: client.clone(),
			transaction_pool: transaction_pool.clone(),
			spawn_handle: task_manager.spawn_handle(),
			import_queue,
			block_announce_validator_builder: None,
			warp_sync: Some(warp_sync),
		})?;

	if config.offchain_worker.enabled {
		sc_service::build_offchain_workers(
			&config,
			task_manager.spawn_handle(),
			client.clone(),
			network.clone(),
		);
	}

	let role = config.role.clone();
	let force_authoring = config.force_authoring;
	let name = config.network.node_name.clone();
	let enable_grandpa = !config.disable_grandpa;
	let prometheus_registry = config.prometheus_registry().cloned();
	let subscription_task_executor =
		sc_rpc::SubscriptionTaskExecutor::new(task_manager.spawn_handle());
	let fee_history_cache: fc_rpc_core::types::FeeHistoryCache = Arc::new(Mutex::new(BTreeMap::new()));
	let fee_history_cache_limit = cli.run.fee_history_limit;
	let overrides = edgeware_rpc::overrides_handle(client.clone());

	let (block_import, grandpa_link) = import_setup;

	edgeware_rpc::spawn_essential_tasks(edgeware_rpc::SpawnTasksParams {
		task_manager: &task_manager,
		client: client.clone(),
		substrate_backend: backend.clone(),
		frontier_backend: frontier_backend.clone(),
		filter_pool: filter_pool.clone(),
		fee_history_cache: fee_history_cache.clone(),
		fee_history_limit: fee_history_cache_limit,
		overrides: overrides.clone(),
	});

	let ethapi_cmd: Vec<_> = cli
		.run
		.ethapi
		.iter()
		.map(|v| EthApiCmd::from_str(&v.to_string()))
		.flatten()
		.collect();

	let justification_stream = grandpa_link.justification_stream();
	let shared_authority_set = grandpa_link.shared_authority_set().clone();
	let shared_voter_state = sc_finality_grandpa::SharedVoterState::empty();
	// let rpc_setup = shared_voter_state.clone();
//	let rpc_setup = (shared_voter_state.clone(), finality_proof_provider.clone());

	let finality_proof_provider = sc_finality_grandpa::FinalityProofProvider::new_for_service(
		backend.clone(),
		Some(shared_authority_set.clone()),
	);

	let client = client.clone();
	let pool = transaction_pool.clone();
	let select_chain = select_chain.clone();
	let keystore = keystore_container.sync_keystore();

	let block_data_cache = Arc::new(fc_rpc::EthBlockDataCacheTask::new(
		task_manager.spawn_handle(),
		overrides.clone(),
		rpc_config.eth_log_block_cache,
		rpc_config.eth_statuses_cache,
		prometheus_registry.clone(),
	));

	let tracing_requesters =
		if ethapi_cmd.contains(&EthApiCmd::Debug) || ethapi_cmd.contains(&EthApiCmd::Trace) {
			edgeware_rpc::tracing::spawn_tracing_tasks(
				&rpc_config,
				edgeware_rpc::SpawnTasksParams {
					task_manager: &task_manager,
					client: client.clone(),
					substrate_backend: backend.clone(),
					frontier_backend: frontier_backend.clone(),
					filter_pool: filter_pool.clone(),
					overrides: overrides.clone(),
					fee_history_cache: fee_history_cache.clone(),
					fee_history_limit: fee_history_cache_limit,
				},
			)
		} else {
			edgeware_rpc::tracing::RpcRequesters {
				debug: None,
				trace: None,
			}
		};

	let enable_dev_signer =  cli.run.enable_dev_signer;
	let is_authority = config.role.is_authority();
	let clt = client.clone();
	let ntw = network.clone();
	let svs = shared_voter_state.clone();
	let rpc_extensions_builder = move |deny_unsafe, subscription_executor| {
		let deps = edgeware_rpc::FullDeps {
			client: clt.clone(),
			pool: pool.clone(),
//			select_chain: select_chain.clone(),
//			chain_spec: chain_spec.cloned_box(),
			deny_unsafe,
			grandpa: edgeware_rpc::GrandpaDeps {
				shared_voter_state: svs.clone(),
				shared_authority_set: shared_authority_set.clone(),
				justification_stream: justification_stream.clone(),
				subscription_executor,
				finality_provider: finality_proof_provider.clone(),
			},
			// Frontier
			graph: pool.pool().clone(),
			is_authority: is_authority,
			enable_dev_signer: enable_dev_signer,
			network: ntw.clone(),
			filter_pool: filter_pool.clone(),
			ethapi_cmd: ethapi_cmd.clone(),
			backend: frontier_backend.clone(),
			max_past_logs: rpc_config.max_past_logs,
			fee_history_cache: fee_history_cache.clone(),
			fee_history_cache_limit: fee_history_cache_limit,
			overrides: overrides.clone(),
			block_data_cache: block_data_cache.clone(),
			command_sink: None, 
		};
		#[allow(unused_mut)]
		let mut io = edgeware_rpc::create_full(deps, subscription_task_executor.clone());
		if ethapi_cmd.contains(&EthApiCmd::Debug) || ethapi_cmd.contains(&EthApiCmd::Trace) {
			edgeware_rpc::tracing::extend_with_tracing(
				clt.clone(),
				tracing_requesters.clone(),
				rpc_config.ethapi_trace_max_count,
				&mut io,
			);
		}
		Ok(io)
	};

	let rpc_handlers = sc_service::spawn_tasks(sc_service::SpawnTasksParams {
		config: config,
		client: client.clone(),
		backend: backend.clone(),
		task_manager: &mut task_manager,
		keystore: keystore,
		transaction_pool: transaction_pool.clone(),
		rpc_extensions_builder: Box::new(rpc_extensions_builder),
		network: network.clone(),
		system_rpc_tx: system_rpc_tx,
		telemetry: telemetry.as_mut(),
	})?;


	let backoff_authoring_blocks: Option<()> = None;

	if let sc_service::config::Role::Authority { .. } = &role {
		let proposer_factory = sc_basic_authorship::ProposerFactory::new(
			task_manager.spawn_handle(),
			client.clone(),
			transaction_pool.clone(),
			prometheus_registry.as_ref(),
			telemetry.as_ref().map(|x| x.handle()),
		);

		let can_author_with =
			sp_consensus::CanAuthorWithNativeVersion::new(client.executor().clone());

		let slot_duration = sc_consensus_aura::slot_duration(&*client)?;
		let raw_slot_duration: sc_consensus_aura::SlotDuration = slot_duration.clone();
		let target_gas_price = U256::from(cli.run.target_gas_price);

		let aura = sc_consensus_aura::start_aura::<sp_consensus_aura::ed25519::AuthorityPair, _, _, _, _, _, _, _, _, _, _, _>(
			StartAuraParams {
				slot_duration: slot_duration,
				client: client.clone(),
				select_chain: select_chain,
				block_import: block_import,
				proposer_factory: proposer_factory,
				sync_oracle: network.clone(),
				justification_sync_link: network.clone(),
				create_inherent_data_providers: move |_, ()| async move {
					let timestamp = sp_timestamp::InherentDataProvider::from_system_time();

					let slot =
						sp_consensus_aura::inherents::InherentDataProvider::from_timestamp_and_slot_duration(
							*timestamp,
							raw_slot_duration,
						);

//					let uncles =
//						sp_authorship::InherentDataProvider::<<Block as BlockT>::Header>::check_inherents();

					let dynamic_fee =
						pallet_dynamic_fee::InherentDataProvider(U256::from(target_gas_price));

					Ok((timestamp, slot, /*uncles,*/ dynamic_fee))
				},

				force_authoring: force_authoring,
				backoff_authoring_blocks: backoff_authoring_blocks,
				keystore: keystore_container.sync_keystore(),
				can_author_with: can_author_with,
				block_proposal_slot_portion: SlotProportion::new(2f32 / 3f32),
				max_block_proposal_slot_portion: None,
				telemetry: telemetry.as_ref().map(|x| x.handle()),
			},
		)?;

		// the AURA authoring task is considered essential, i.e. if it
		// fails we take down the service with it.
		task_manager
			.spawn_essential_handle()
			.spawn_blocking("aura-proposer", None, aura);
	}

	// Spawn authority discovery module.
	if role.is_authority() {
		let authority_discovery_role =
			sc_authority_discovery::Role::PublishAndDiscover(keystore_container.keystore());
		let dht_event_stream =
			network.event_stream("authority-discovery").filter_map(|e| async move {
				match e {
					Event::Dht(e) => Some(e),
					_ => None,
				}
			});
		let (authority_discovery_worker, _service) =
			sc_authority_discovery::new_worker_and_service_with_config(
				sc_authority_discovery::WorkerConfig {
					publish_non_global_ips: auth_disc_publish_non_global_ips,
					..Default::default()
				},
				client.clone(),
				network.clone(),
				Box::pin(dht_event_stream),
				authority_discovery_role,
				prometheus_registry.clone(),
			);

		task_manager.spawn_handle().spawn(
			"authority-discovery-worker",
			Some("networking"),
			authority_discovery_worker.run(),
		);
	}

	// if the node isn't actively participating in consensus then it doesn't
	// need a keystore, regardless of which protocol we use below.
	let keystore =
		if role.is_authority() { Some(keystore_container.sync_keystore()) } else { None };

	let config = sc_finality_grandpa::Config {
		// FIXME #1578 make this available through chainspec
		gossip_duration: std::time::Duration::from_millis(333),
		justification_period: 512,
		name: Some(name),
		observer_enabled: false,
		keystore,
		local_role: role,
		telemetry: telemetry.as_ref().map(|x| x.handle()),
		protocol_name: grandpa_protocol_name,
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
			telemetry: telemetry.as_ref().map(|x| x.handle()),
			voting_rule: sc_finality_grandpa::VotingRulesBuilder::default().build(),
			prometheus_registry,
			shared_voter_state,
		};

		// the GRANDPA voter task is considered infallible, i.e.
		// if it fails we take down the service with it.
		task_manager.spawn_essential_handle().spawn_blocking(
			"grandpa-voter",
			None,
			sc_finality_grandpa::run_grandpa_voter(grandpa_config)?,
		);
	}

	network_starter.start_network();
	Ok(NewFullBase { task_manager, client, network, transaction_pool, rpc_handlers })
}


/// Builds a new service for a full client.
pub fn new_full(config: Configuration, cli: &Cli, rpc_config: RpcConfig) -> Result<TaskManager, ServiceError> {
	new_full_base(config, cli, rpc_config).map(|NewFullBase { task_manager, .. }| task_manager)
}
