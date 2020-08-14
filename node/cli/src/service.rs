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

use std::sync::Arc;

use sc_consensus_aura;
use sc_finality_grandpa::{
	self, FinalityProofProvider as GrandpaFinalityProofProvider, StorageAndProofProvider,
};
use edgeware_primitives::Block;
use edgeware_runtime::RuntimeApi;
use sc_service::{
	config::{Role, Configuration}, error::{Error as ServiceError},
	RpcHandlers, ServiceComponents, TaskManager,
};
use sp_inherents::InherentDataProviders;
use sc_network::{Event, NetworkService};
use sp_runtime::traits::Block as BlockT;
use futures::prelude::*;
use sc_client_api::{ExecutorProvider, RemoteBackend};
use sp_core::traits::BareCryptoStorePtr;
use edgeware_executor::Executor;


type FullClient = sc_service::TFullClient<Block, RuntimeApi, Executor>;
type FullBackend = sc_service::TFullBackend<Block>;
type FullSelectChain = sc_consensus::LongestChain<FullBackend, Block>;
type FullGrandpaBlockImport =
	sc_finality_grandpa::GrandpaBlockImport<FullBackend, Block, FullClient, FullSelectChain>;
type LightClient = sc_service::TLightClient<Block, RuntimeApi, Executor>;

pub fn new_full_params(config: Configuration) -> Result<(
	sc_service::ServiceParams<
		Block, FullClient,
		sc_consensus_aura::AuraImportQueue<Block, FullClient>,
		sc_transaction_pool::FullPool<Block, FullClient>,
		(), FullBackend,
	>,
	(
		sc_consensus_aura::AuraBlockImport<Block, FullClient, FullGrandpaBlockImport, sp_consensus_aura::ed25519::AuthorityPair>,
		sc_finality_grandpa::LinkHalf<Block, FullClient, FullSelectChain>,
	),
	sc_finality_grandpa::SharedVoterState,
	FullSelectChain,
	sp_inherents::InherentDataProviders,
), ServiceError> {
	let inherent_data_providers = sp_inherents::InherentDataProviders::new();
	let (client, backend, keystore, task_manager) =
		sc_service::new_full_parts::<Block, RuntimeApi, Executor>(&config)?;
	let client = Arc::new(client);

	let select_chain = sc_consensus::LongestChain::new(backend.clone());

	let pool_api = sc_transaction_pool::FullChainApi::new(
		client.clone(), config.prometheus_registry(),
	);
	let transaction_pool = sc_transaction_pool::BasicPool::new_full(
		config.transaction_pool.clone(),
		std::sync::Arc::new(pool_api),
		config.prometheus_registry(),
		task_manager.spawn_handle(),
		client.clone(),
	);

	let (grandpa_block_import, grandpa_link) = sc_finality_grandpa::block_import(
		client.clone(), &(client.clone() as Arc<_>), select_chain.clone(),
	)?;
	let justification_import = grandpa_block_import.clone();

	let aura_block_import =
		sc_consensus_aura::AuraBlockImport::<_, _, _, sp_consensus_aura::ed25519::AuthorityPair>::new(
			justification_import.clone(),
			client.clone()
		);

	let import_queue = sc_consensus_aura::import_queue::<_, _, _, sp_consensus_aura::ed25519::AuthorityPair, _>(
		sc_consensus_aura::slot_duration(&*client)?,
		aura_block_import,
		Some(Box::new(grandpa_block_import.clone())),
		None,
		client.clone(),
		inherent_data_providers.clone(),
		&task_manager.spawn_handle(),
		config.prometheus_registry(),
	)?;

	let import_setup = (aura_block_import, grandpa_link);

	let (rpc_extensions_builder, rpc_setup) = {
		let (_, grandpa_link) = &import_setup;

		let shared_authority_set = grandpa_link.shared_authority_set().clone();
		let shared_voter_state = sc_finality_grandpa::SharedVoterState::empty();

		let rpc_setup = shared_voter_state.clone();

		let client = client.clone();
		let pool = transaction_pool.clone();
		let select_chain = select_chain.clone();
		let keystore = keystore.clone();

		let rpc_extensions_builder = Box::new(move |deny_unsafe| {
			let deps = edgeware_rpc::FullDeps {
				client: client.clone(),
				pool: pool.clone(),
				select_chain: select_chain.clone(),
				deny_unsafe,
				grandpa: edgeware_rpc::GrandpaDeps {
					shared_voter_state: shared_voter_state.clone(),
					shared_authority_set: shared_authority_set.clone(),
				},
				is_authority: config.role.clone().is_authority(),
			};

			edgeware_rpc::create_full(deps)
		});

		(rpc_extensions_builder, rpc_setup)
	};

	let provider = client.clone() as Arc<dyn sc_finality_grandpa::StorageAndProofProvider<_, _>>;
	let finality_proof_provider =
		Arc::new(sc_finality_grandpa::FinalityProofProvider::new(backend.clone(), provider));

	let params = sc_service::ServiceParams {
		backend, client, import_queue, keystore, task_manager, transaction_pool,
		config,
		block_announce_validator_builder: None,
		finality_proof_request_builder: None,
		finality_proof_provider: Some(finality_proof_provider),
		on_demand: None,
		remote_blockchain: None,
		rpc_extensions_builder: Box::new(|_| ()),
	};

	Ok((params, import_setup, rpc_setup, select_chain, inherent_data_providers))
}

/// Creates a full service from the configuration.
pub fn new_full_base(
	config: Configuration,
	with_startup_data: impl FnOnce(
		&sc_consensus_aura::AuraBlockImport<Block, FullClient, FullGrandpaBlockImport, sp_consensus_aura::ed25519::AuthorityPair>,
		&sc_finality_grandpa::LinkHalf<Block, FullClient, FullSelectChain>,
	)
) -> Result<(
	TaskManager, InherentDataProviders, Arc<FullClient>,
	Arc<NetworkService<Block, <Block as BlockT>::Hash>>,
	Arc<sc_transaction_pool::FullPool<Block, FullClient>>,
), ServiceError> {
	let (params, import_setup, rpc_setup, select_chain, inherent_data_providers)
		= new_full_params(config)?;

	let (
		role, force_authoring, name, enable_grandpa, prometheus_registry,
		client, transaction_pool, keystore,
	) = {
		let sc_service::ServiceParams {
			config, client, transaction_pool, keystore, ..
		} = &params;

		(
			config.role.clone(),
			config.force_authoring,
			config.network.node_name.clone(),
			!config.disable_grandpa,
			config.prometheus_registry().cloned(),

			client.clone(), transaction_pool.clone(), keystore.clone(),
		)
	};

	let ServiceComponents {
		task_manager, network, telemetry_on_connect_sinks, ..
	} = sc_service::build(params)?;
	
	let (block_import, grandpa_link) = import_setup;
	let shared_voter_state = rpc_setup;

	(with_startup_data)(&block_import, &grandpa_link);

	if let sc_service::config::Role::Authority { .. } = &role {
		let proposer = sc_basic_authorship::ProposerFactory::new(
			client.clone(),
			transaction_pool.clone(),
			prometheus_registry.as_ref(),
		);

		let can_author_with =
			sp_consensus::CanAuthorWithNativeVersion::new(client.executor().clone());

		let aura = sc_consensus_aura::start_aura::<_, _, _, _, _, sp_consensus_aura::ed25519::AuthorityPair, _, _, _>(
			sc_consensus_aura::slot_duration(&*client)?,
			client.clone(),
			select_chain,
			block_import,
			proposer,
			network.clone(),
			inherent_data_providers.clone(),
			force_authoring,
			keystore.clone(),
			can_author_with,
		)?;

		// the AURA authoring task is considered essential, i.e. if it
		// fails we take down the service with it.
		task_manager.spawn_essential_handle().spawn_blocking("aura-proposer", aura);
	}

	// Spawn authority discovery module.
	if matches!(role, Role::Authority{..} | Role::Sentry {..}) {
		let (sentries, authority_discovery_role) = match role {
			sc_service::config::Role::Authority { ref sentry_nodes } => (
				sentry_nodes.clone(),
				sc_authority_discovery::Role::Authority (
					keystore.clone(),
				),
			),
			sc_service::config::Role::Sentry {..} => (
				vec![],
				sc_authority_discovery::Role::Sentry,
			),
			_ => unreachable!("Due to outer matches! constraint; qed.")
		};

		let dht_event_stream = network.event_stream("authority-discovery")
			.filter_map(|e| async move { match e {
				Event::Dht(e) => Some(e),
				_ => None,
			}}).boxed();
		let authority_discovery = sc_authority_discovery::AuthorityDiscovery::new(
			client.clone(),
			network.clone(),
			sentries,
			dht_event_stream,
			authority_discovery_role,
			prometheus_registry.clone(),
		);

		task_manager.spawn_handle().spawn("authority-discovery", authority_discovery);
	}

	// if the node isn't actively participating in consensus then it doesn't
	// need a keystore, regardless of which protocol we use below.
	let keystore = if role.is_authority() {
		Some(keystore as BareCryptoStorePtr)
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
			inherent_data_providers: inherent_data_providers.clone(),
			telemetry_on_connect: Some(telemetry_on_connect_sinks.on_connect_stream()),
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
	} else {
		sc_finality_grandpa::setup_disabled_grandpa(
			client.clone(),
			&inherent_data_providers,
			network.clone(),
		)?;
	}

	Ok((task_manager, inherent_data_providers, client, network, transaction_pool))
}

/// Builds a new service for a full client.
pub fn new_full(config: Configuration)
-> Result<TaskManager, ServiceError> {
	new_full_base(config, |_, _| ()).map(|(task_manager, _, _, _, _)| {
		task_manager
	})
}

pub fn new_light_base(config: Configuration) -> Result<(
	TaskManager, Arc<RpcHandlers>, Arc<LightClient>,
	Arc<NetworkService<Block, <Block as BlockT>::Hash>>,
	Arc<sc_transaction_pool::LightPool<Block, LightClient, sc_network::config::OnDemand<Block>>>
), ServiceError> {
	let (client, backend, keystore, task_manager, on_demand) =
		sc_service::new_light_parts::<Block, RuntimeApi, Executor>(&config)?;

	let select_chain = sc_consensus::LongestChain::new(backend.clone());

	let transaction_pool_api = Arc::new(sc_transaction_pool::LightChainApi::new(
		client.clone(),
		on_demand.clone(),
	));
	let transaction_pool = Arc::new(sc_transaction_pool::BasicPool::new_light(
		config.transaction_pool.clone(),
		transaction_pool_api,
		config.prometheus_registry(),
		task_manager.spawn_handle(),
	));

	let grandpa_block_import = sc_finality_grandpa::light_block_import(
		client.clone(), backend.clone(), &(client.clone() as Arc<_>),
		Arc::new(on_demand.checker().clone()),
	)?;

	let finality_proof_import = grandpa_block_import.clone();
	let finality_proof_request_builder =
		finality_proof_import.create_finality_proof_request_builder();

	let import_queue = sc_consensus_aura::import_queue::<_, _, _, sp_consensus_aura::ed25519::AuthorityPair, _>(
		sc_consensus_aura::slot_duration(&*client)?,
		grandpa_block_import,
		None,
		Some(Box::new(finality_proof_import)),
		client.clone(),
		InherentDataProviders::new(),
		&task_manager.spawn_handle(),
		config.prometheus_registry(),
	)?;

	// GenesisAuthoritySetProvider is implemented for StorageAndProofProvider
	let provider = client.clone() as Arc<dyn StorageAndProofProvider<_, _>>;
	let finality_proof_provider =
		Arc::new(GrandpaFinalityProofProvider::new(backend.clone(), provider));

	let light_deps = edgeware_rpc::LightDeps {
		remote_blockchain: backend.remote_blockchain(),
		fetcher: on_demand.clone(),
		client: client.clone(),
		pool: transaction_pool.clone(),
	};

	let rpc_extensions = edgeware_rpc::create_light(light_deps);

	let ServiceComponents { task_manager, rpc_handlers, network, .. } =
		sc_service::build(sc_service::ServiceParams {	
			block_announce_validator_builder: None,
			finality_proof_request_builder: Some(finality_proof_request_builder),
			finality_proof_provider: Some(finality_proof_provider),
			on_demand: Some(on_demand),
			remote_blockchain: Some(backend.remote_blockchain()),
			rpc_extensions_builder: Box::new(sc_service::NoopRpcExtensionBuilder(rpc_extensions)),
			client: client.clone(),
			transaction_pool: transaction_pool.clone(),
			config, import_queue, keystore, backend, task_manager,
		})?;
	
	Ok((task_manager, rpc_handlers, client, network, transaction_pool))
}

/// Builds a new service for a light client.
pub fn new_light(config: Configuration) -> Result<TaskManager, ServiceError> {	
	new_light_base(config).map(|(task_manager, _, _, _, _)| {
		task_manager
	})
}
