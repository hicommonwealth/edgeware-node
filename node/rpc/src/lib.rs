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

//! A collection of node-specific RPC methods.
//!
//! Since `substrate` core functionality makes no assumptions
//! about the modules used inside the runtime, so do
//! RPC methods defined in `substrate-rpc` crate.
//! It means that `client/rpc` can't have any methods that
//! need some strong assumptions about the particular runtime.
//!
//! The RPCs available in this crate however can make some assumptions
//! about how the runtime is constructed and what `SRML` modules
//! are part of it. Therefore all node-runtime-specific RPCs can
//! be placed here or imported from corresponding `SRML` RPC definitions.

#![warn(missing_docs)]

pub use edgeware_opts as opts;
use edgeware_opts::EthApi as EthApiCmd;
use edgeware_primitives::{AccountId, Balance, Block, BlockNumber, Hash, Index};
use fc_rpc::{OverrideHandle, RuntimeApiStorageOverride, SchemaV1Override, StorageOverride};
use fc_rpc_core::types::{FilterPool, PendingTransactions};
use jsonrpc_pubsub::manager::SubscriptionManager;
use merkle_rpc::{MerkleApi, MerkleClient};
use pallet_ethereum::EthereumStorageSchema;
use sc_client_api::{
	backend::{AuxStore, Backend, StateBackend, StorageProvider},
	client::BlockchainEvents,
	BlockOf,
};
use fc_mapping_sync::MappingSyncWorker;

use sc_service::TaskManager;
use sc_finality_grandpa::{FinalityProofProvider, GrandpaJustificationStream, SharedAuthoritySet, SharedVoterState};
use sc_finality_grandpa_rpc::GrandpaRpcHandler;
use sc_network::NetworkService;
use sc_rpc::SubscriptionTaskExecutor;
pub use sc_rpc_api::DenyUnsafe;
use sc_transaction_graph::{ChainApi, Pool};
use sp_api::{HeaderT, ProvideRuntimeApi};
use sp_core::H256;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};
use sp_consensus::SelectChain;
use sp_transaction_pool::TransactionPool;
use std::{
	collections::{BTreeMap},
	sync::{Arc},
	time::Duration,
};
use fc_rpc::EthTask;
use edgeware_rpc_trace::CacheTask;
use edgeware_rpc_debug::DebugHandler;
use sp_runtime::traits::{BlakeTwo256, Block as BlockT};
use tokio::sync::Semaphore;
use edgeware_rpc_primitives_debug::DebugRuntimeApi;
use fp_rpc::EthereumRuntimeRPCApi;
use edgeware_rpc_debug::{Debug, DebugRequester, DebugServer};
use edgeware_rpc_trace::{CacheRequester as TraceFilterCacheRequester, Trace, TraceServer};
use edgeware_rpc_txpool::{TxPool, TxPoolServer};
use futures::StreamExt;


/// RPC Client
pub mod client;
use client::RuntimeApiCollection;

/// Public io handler for exporting into other modules
pub type IoHandler = jsonrpc_core::IoHandler<sc_rpc::Metadata>;

/// Light client extra dependencies.
pub struct LightDeps<C, F, P> {
	/// The client instance to use.
	pub client: Arc<C>,
	/// Transaction pool instance.
	pub pool: Arc<P>,
	/// Remote access to the blockchain (async).
	pub remote_blockchain: Arc<dyn sc_client_api::light::RemoteBlockchain<Block>>,
	/// Fetcher instance.
	pub fetcher: Arc<F>,
}

/// Extra dependencies for GRANDPA
pub struct GrandpaDeps<B> {
	/// Voting round info.
	pub shared_voter_state: SharedVoterState,
	/// Authority set info.
	pub shared_authority_set: SharedAuthoritySet<Hash, BlockNumber>,
	/// Receives notifications about justification events from Grandpa.
	pub justification_stream: GrandpaJustificationStream<Block>,
	/// Executor to drive the subscription manager in the Grandpa RPC handler.
	pub subscription_executor: SubscriptionTaskExecutor,
	/// Finality proof provider.
	pub finality_provider: Arc<FinalityProofProvider<B, Block>>,
}

/// Full client dependencies.
pub struct FullDeps<C, P, SC, B, A: ChainApi> {
	/// The client instance to use.
	pub client: Arc<C>,
	/// Transaction pool instance.
	pub pool: Arc<P>,
	/// Graph pool instance.
	pub graph: Arc<Pool<A>>,
	/// The SelectChain Strategy
	pub select_chain: SC,
	/// The Node authority flag
	pub is_authority: bool,
	/// Whether to enable dev signer
	pub enable_dev_signer: bool,
	/// Network service
	pub network: Arc<NetworkService<Block, Hash>>,
	/// Whether to deny unsafe calls
	pub deny_unsafe: DenyUnsafe,
	/// GRANDPA specific dependencies.
	pub grandpa: GrandpaDeps<B>,
	/// Ethereum pending transactions.
	pub pending_transactions: PendingTransactions,
	/// EthFilterApi pool.
	pub filter_pool: Option<FilterPool>,
	/// Backend.
	pub backend: Arc<fc_db::Backend<Block>>,
	/// Maximum number of logs in a query.
	pub max_past_logs: u32,
	/// The list of optional RPC extensions.
	pub ethapi_cmd: Vec<EthApiCmd>,
	/// Debug server requester.
	pub debug_requester: Option<DebugRequester>,
	/// Trace filter cache server requester.
	pub trace_filter_requester: Option<TraceFilterCacheRequester>,
	/// Trace filter max count.
	pub trace_filter_max_count: u32,
}

/// Instantiate all Full RPC extensions.
pub fn create_full<C, P, SC, B, A>(
	deps: FullDeps<C, P, SC, B, A>,
	subscription_task_executor: SubscriptionTaskExecutor,
) -> jsonrpc_core::IoHandler<sc_rpc_api::Metadata>
where
	C: ProvideRuntimeApi<Block> + StorageProvider<Block, B> + AuxStore,
	C: HeaderBackend<Block> + HeaderMetadata<Block, Error = BlockChainError> + 'static,
	C: Send + Sync + 'static,
	C: BlockchainEvents<Block>,
	C::Api: pallet_contracts_rpc::ContractsRuntimeApi<Block, AccountId, Balance, BlockNumber, Hash>,
	C::Api: BlockBuilder<Block>,
	C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
	C::Api: fp_rpc::EthereumRuntimeRPCApi<Block>,
	C::Api: merkle::MerkleApi<Block>,
	C::Api: BlockBuilder<Block>,
	P: TransactionPool<Block = Block> + 'static,
	SC: SelectChain<Block> + 'static,
	B: sc_client_api::Backend<Block> + Send + Sync + 'static,
	B::State: sc_client_api::backend::StateBackend<sp_runtime::traits::HashFor<Block>>,
	A: ChainApi<Block = Block> + 'static,
	C::Api: RuntimeApiCollection<StateBackend = B::State>,
{
	use fc_rpc::{
		EthApi, EthApiServer, EthDevSigner, EthFilterApi, EthFilterApiServer, EthPubSubApi, EthPubSubApiServer,
		EthSigner, HexEncodedIdProvider, NetApi, NetApiServer, Web3Api, Web3ApiServer,
	};
	use pallet_contracts_rpc::{Contracts, ContractsApi};
	use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApi};
	use substrate_frame_rpc_system::{FullSystem, SystemApi};

	let mut io = jsonrpc_core::IoHandler::default();
	let FullDeps {
		client,
		pool,
		graph,
		select_chain: _,
		enable_dev_signer,
		is_authority,
		network,
		deny_unsafe,
		grandpa,
		pending_transactions,
		filter_pool,
		backend,
		max_past_logs,
		debug_requester,
		trace_filter_requester,
		trace_filter_max_count,
		ethapi_cmd,
	} = deps;
	let GrandpaDeps {
		shared_voter_state,
		shared_authority_set,
		justification_stream,
		subscription_executor,
		finality_provider,
	} = grandpa;

	io.extend_with(SystemApi::to_delegate(FullSystem::new(
		client.clone(),
		pool.clone(),
		deny_unsafe,
	)));
	// Making synchronous calls in light client freezes the browser currently,
	// more context: https://github.com/paritytech/substrate/pull/3480
	// These RPCs should use an asynchronous caller instead.
	io.extend_with(ContractsApi::to_delegate(Contracts::new(client.clone())));
	io.extend_with(TransactionPaymentApi::to_delegate(TransactionPayment::new(
		client.clone(),
	)));

	let mut signers = Vec::new();
	if enable_dev_signer {
		signers.push(Box::new(EthDevSigner::new()) as Box<dyn EthSigner>);
	}
	let mut overrides_map = BTreeMap::new();
	overrides_map.insert(
		EthereumStorageSchema::V1,
		Box::new(SchemaV1Override::new(client.clone())) as Box<dyn StorageOverride<_> + Send + Sync>,
	);

	let overrides = Arc::new(OverrideHandle {
		schemas: overrides_map,
		fallback: Box::new(RuntimeApiStorageOverride::new(client.clone())),
	});

	io.extend_with(EthApiServer::to_delegate(EthApi::new(
		client.clone(),
		pool.clone(),
		edgeware_runtime::TransactionConverter,
		network.clone(),
		pending_transactions.clone(),
		signers,
		overrides.clone(),
		backend,
		is_authority,
		max_past_logs,
	)));

	if let Some(filter_pool) = filter_pool {
		io.extend_with(EthFilterApiServer::to_delegate(EthFilterApi::new(
			client.clone(),
			filter_pool.clone(),
			500 as usize, // max stored filters
			overrides.clone(),
			max_past_logs,
		)));
	}

	io.extend_with(NetApiServer::to_delegate(NetApi::new(
		client.clone(),
		network.clone(),
		// Whether to format the `peer_count` response as Hex (default) or not.
		true,
	)));

	io.extend_with(Web3ApiServer::to_delegate(Web3Api::new(client.clone())));
	io.extend_with(EthPubSubApiServer::to_delegate(EthPubSubApi::new(
		pool.clone(),
		client.clone(),
		network.clone(),
		SubscriptionManager::<HexEncodedIdProvider>::with_id_provider(
			HexEncodedIdProvider::default(),
			Arc::new(subscription_task_executor),
		),
		overrides,
	)));

	io.extend_with(MerkleApi::to_delegate(MerkleClient::new(client.clone())));
	io.extend_with(sc_finality_grandpa_rpc::GrandpaApi::to_delegate(
		GrandpaRpcHandler::new(
			shared_authority_set,
			shared_voter_state,
			justification_stream,
			subscription_executor,
			finality_provider,
		),
	));

	if ethapi_cmd.contains(&EthApiCmd::Txpool) {
		io.extend_with(TxPoolServer::to_delegate(TxPool::new(Arc::clone(&client), graph)));
	}

	if let Some(trace_filter_requester) = trace_filter_requester {
		io.extend_with(TraceServer::to_delegate(Trace::new(
			client,
			trace_filter_requester,
			trace_filter_max_count,
		)));
	}

	if let Some(debug_requester) = debug_requester {
		io.extend_with(DebugServer::to_delegate(Debug::new(debug_requester)));
	}

	io
}

/// Instantiate all Light RPC extensions.
pub fn create_light<C, P, M, F>(deps: LightDeps<C, F, P>) -> jsonrpc_core::IoHandler<M>
where
	C: sp_blockchain::HeaderBackend<Block>,
	C: Send + Sync + 'static,
	F: sc_client_api::light::Fetcher<Block> + 'static,
	P: TransactionPool + 'static,
	M: jsonrpc_core::Metadata + Default,
{
	use substrate_frame_rpc_system::{LightSystem, SystemApi};

	let LightDeps {
		client,
		pool,
		remote_blockchain,
		fetcher,
	} = deps;
	let mut io = jsonrpc_core::IoHandler::default();
	io.extend_with(SystemApi::<Hash, AccountId, Index>::to_delegate(LightSystem::new(
		client,
		remote_blockchain,
		fetcher,
		pool,
	)));

	io
}

/// Parameters for various rpc utilities
pub struct RpcRequesters {
	/// debug
	pub debug: Option<DebugRequester>,
	/// trace
	pub trace: Option<TraceFilterCacheRequester>,
}

/// Parameters for various services to start
pub struct SpawnTasksParams<'a, B: BlockT, C, BE> {
	/// task manager
	pub task_manager: &'a TaskManager,
	/// substrate client
	pub client: Arc<C>,
	/// substrate backend
	pub substrate_backend: Arc<BE>,
	/// frontier backend
	pub frontier_backend: Arc<fc_db::Backend<B>>,
	/// pending txes
	pub pending_transactions: PendingTransactions,
	/// ethereum filter pool
	pub filter_pool: Option<FilterPool>,
}

/// Spawn the tasks that are required to run Moonbeam.
pub fn spawn_tasks<B, C, BE>(
	rpc_config: &edgeware_opts::RpcConfig,
	params: SpawnTasksParams<B, C, BE>,
) -> RpcRequesters
where
	C: ProvideRuntimeApi<B> + BlockOf,
	C: HeaderBackend<B> + HeaderMetadata<B, Error = BlockChainError> + 'static,
	C: BlockchainEvents<B>,
	C: Send + Sync + 'static,
	C::Api: EthereumRuntimeRPCApi<B> + DebugRuntimeApi<B> + DebugRuntimeApi<B>,
	C::Api: BlockBuilder<B>,
	B: BlockT<Hash = H256> + Send + Sync + 'static,
	B::Header: HeaderT<Number = u32>,
	BE: Backend<B> + 'static,
	BE::State: StateBackend<BlakeTwo256>,
{
	let permit_pool = Arc::new(Semaphore::new(rpc_config.ethapi_max_permits as usize));

	let (trace_filter_task, trace_filter_requester) =
		if rpc_config.ethapi.contains(&EthApiCmd::Trace) {
			let (trace_filter_task, trace_filter_requester) = CacheTask::create(
				Arc::clone(&params.client),
				Arc::clone(&params.substrate_backend),
				Duration::from_secs(rpc_config.ethapi_trace_cache_duration),
				Arc::clone(&permit_pool),
			);
			(Some(trace_filter_task), Some(trace_filter_requester))
		} else {
			(None, None)
		};

	let (debug_task, debug_requester) = if rpc_config.ethapi.contains(&EthApiCmd::Debug) {
		let (debug_task, debug_requester) = DebugHandler::task(
			Arc::clone(&params.client),
			Arc::clone(&params.substrate_backend),
			Arc::clone(&params.frontier_backend),
			Arc::clone(&permit_pool),
		);
		(Some(debug_task), Some(debug_requester))
	} else {
		(None, None)
	};

	// Frontier offchain DB task. Essential.
	// Maps emulated ethereum data to substrate native data.
	params.task_manager.spawn_essential_handle().spawn(
		"frontier-mapping-sync-worker",
		MappingSyncWorker::new(
			params.client.import_notification_stream(),
			Duration::new(6, 0),
			params.client.clone(),
			params.substrate_backend.clone(),
			params.frontier_backend.clone(),
		)
		.for_each(|()| futures::future::ready(())),
	);

	// `trace_filter` cache task. Essential.
	// Proxies rpc requests to it's handler.
	if let Some(trace_filter_task) = trace_filter_task {
		params
			.task_manager
			.spawn_essential_handle()
			.spawn("trace-filter-cache", trace_filter_task);
	}

	// `debug` task if enabled. Essential.
	// Proxies rpc requests to it's handler.
	if let Some(debug_task) = debug_task {
		params
			.task_manager
			.spawn_essential_handle()
			.spawn("ethapi-debug", debug_task);
	}

	// Frontier `EthFilterApi` maintenance.
	// Manages the pool of user-created Filters.
	if let Some(filter_pool) = params.filter_pool {
		// Each filter is allowed to stay in the pool for 100 blocks.
		const FILTER_RETAIN_THRESHOLD: u64 = 100;
		params.task_manager.spawn_essential_handle().spawn(
			"frontier-filter-pool",
			EthTask::filter_pool_task(
				Arc::clone(&params.client),
				filter_pool,
				FILTER_RETAIN_THRESHOLD,
			),
		);
	}

	// Frontier pending transactions task. Essential.
	// Maintenance for the Frontier-specific pending transaction pool.
	if let Some(pending_transactions) = params.pending_transactions {
		const TRANSACTION_RETAIN_THRESHOLD: u64 = 100;
		params.task_manager.spawn_essential_handle().spawn(
			"frontier-pending-transactions",
			EthTask::pending_transaction_task(
				Arc::clone(&params.client),
				pending_transactions,
				TRANSACTION_RETAIN_THRESHOLD,
			),
		);
	}

	RpcRequesters {
		debug: debug_requester,
		trace: trace_filter_requester,
	}
}
