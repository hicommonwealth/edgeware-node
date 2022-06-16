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
//! RPC methods defined in `sc-rpc` crate.
//! It means that `client/rpc` can't have any methods that
//! need some strong assumptions about the particular runtime.
//!
//! The RPCs available in this crate however can make some assumptions
//! about how the runtime is constructed and what FRAME pallets
//! are part of it. Therefore all node-runtime-specific RPCs can
//! be placed here or imported from corresponding FRAME RPC definitions.

#![warn(missing_docs)]

use edgeware_cli_opt::EthApi as EthApiCmd;
use edgeware_primitives::{AccountId, Balance, Block, BlockNumber, Hash, Index};
use edgeware_rpc_txpool::{TxPool, TxPoolServer};
use sc_finality_grandpa::{
	FinalityProofProvider, GrandpaJustificationStream, SharedAuthoritySet, SharedVoterState,
};
use sc_finality_grandpa_rpc::GrandpaRpcHandler;
use sc_rpc::SubscriptionTaskExecutor;
use sc_rpc_api::DenyUnsafe;
use sc_transaction_pool_api::TransactionPool;
use sp_api::{HeaderT, ProvideRuntimeApi};
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};

use std::{collections::BTreeMap, sync::Arc, time::Duration};

use jsonrpc_pubsub::manager::SubscriptionManager;
// Substrate
use sc_client_api::{
	backend::{AuxStore, Backend, StateBackend, StorageProvider},
	client::BlockchainEvents,
	BlockOf,
};
use sc_consensus_manual_seal::rpc::{ManualSeal, ManualSealApi};
use sc_network::NetworkService;
use sc_transaction_pool::{ChainApi, Pool};
use sp_runtime::traits::{BlakeTwo256, Block as BlockT};
// Frontier
use fc_rpc::{
	EthBlockDataCacheTask, EthTask, OverrideHandle, RuntimeApiStorageOverride, 
	SchemaV1Override, SchemaV2Override, SchemaV3Override, StorageOverride,
};
use fc_rpc_core::types::{FeeHistoryCache, FeeHistoryCacheLimit, FilterPool};
use fp_storage::EthereumStorageSchema;
use fc_mapping_sync::{MappingSyncWorker, SyncStrategy};

use sp_core::H256;
use sc_service::TaskManager;
use futures::StreamExt;
/// RPC Client
pub mod client;
/// Tracing module
pub mod tracing;
use client::RuntimeApiCollection;

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
pub struct FullDeps<C, P, BE, A: ChainApi> {
	/// The client instance to use.
	pub client: Arc<C>,
	/// Transaction pool instance.
	pub pool: Arc<P>,
	/// Whether to deny unsafe calls
	pub deny_unsafe: DenyUnsafe,
	/// GRANDPA specific dependencies.
	pub grandpa: GrandpaDeps<BE>,

	/// Graph pool instance.
	pub graph: Arc<Pool<A>>,	
	/// The Node authority flag
	pub is_authority: bool,
	/// Whether to enable dev signer
	pub enable_dev_signer: bool,
	/// Network service
	pub network: Arc<NetworkService<Block, Hash>>,
	/// EthFilterApi pool.
	pub filter_pool: Option<FilterPool>,
	/// The list of optional RPC extensions.
	pub ethapi_cmd: Vec<EthApiCmd>,
	/// Backend.
	pub backend: Arc<fc_db::Backend<Block>>,
	/// Maximum number of logs in a query.
	pub max_past_logs: u32,
	/// Fee history cache.
	pub fee_history_cache: FeeHistoryCache,
	/// Maximum fee history cache size.
	pub fee_history_cache_limit: FeeHistoryCacheLimit,
	/// Ethereum data access overrides.
	pub overrides: Arc<OverrideHandle<Block>>,
	/// Cache for Ethereum block data.
	pub block_data_cache: Arc<EthBlockDataCacheTask<Block>>,
	/// Manual seal command sink
	pub command_sink:
		Option<futures::channel::mpsc::Sender<sc_consensus_manual_seal::rpc::EngineCommand<Hash>>>,
}

/// A IO handler that uses all Full RPC extensions.
pub type IoHandler = jsonrpc_core::IoHandler<sc_rpc::Metadata>;
///
pub fn overrides_handle<C, BE>(client: Arc<C>) -> Arc<OverrideHandle<Block>>
where
	C: ProvideRuntimeApi<Block> + StorageProvider<Block, BE> + AuxStore,
	C: HeaderBackend<Block> + HeaderMetadata<Block, Error = BlockChainError>,
	C: Send + Sync + 'static,
	C::Api: sp_api::ApiExt<Block>
		+ fp_rpc::EthereumRuntimeRPCApi<Block>,
		// + fp_rpc::ConvertTransactionRuntimeApi<Block>,
	BE: Backend<Block> + 'static,
	BE::State: StateBackend<BlakeTwo256>,
{
	let mut overrides_map = BTreeMap::new();
	overrides_map.insert(
		EthereumStorageSchema::V1,
		Box::new(SchemaV1Override::new(client.clone()))
			as Box<dyn StorageOverride<_> + Send + Sync>,
	);
	overrides_map.insert(
		EthereumStorageSchema::V2,
		Box::new(SchemaV2Override::new(client.clone()))
			as Box<dyn StorageOverride<_> + Send + Sync>,
	);
	overrides_map.insert(
		EthereumStorageSchema::V3,
		Box::new(SchemaV3Override::new(client.clone()))
			as Box<dyn StorageOverride<_> + Send + Sync>,
	);

	Arc::new(OverrideHandle {
		schemas: overrides_map,
		fallback: Box::new(RuntimeApiStorageOverride::new(client.clone())),
	})
}

/// Instantiate all Full RPC extensions.
pub fn create_full<C, P, BE, A>(
	deps: FullDeps<C, P, BE, A>,
	subscription_task_executor: SubscriptionTaskExecutor,
	// backend: Arc<BE>,
) -> jsonrpc_core::IoHandler<sc_rpc::Metadata>
// ) -> Result<jsonrpc_core::IoHandler<sc_rpc_api::Metadata>, Box<dyn std::error::Error + Send + Sync>>
where
	C: ProvideRuntimeApi<Block>
		+ sc_client_api::BlockBackend<Block>
		+ StorageProvider<Block, BE>
		+ HeaderBackend<Block>
		+ AuxStore
		+ HeaderMetadata<Block, Error = BlockChainError>
		+ BlockchainEvents<Block>
		+ Sync
		+ Send
		+ 'static,
	C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Index>,
	C::Api: pallet_contracts_rpc::ContractsRuntimeApi<Block, AccountId, Balance, BlockNumber, Hash>,
	C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
	C::Api: BlockBuilder<Block>,
	C::Api: fp_rpc::ConvertTransactionRuntimeApi<Block>,
	C::Api: fp_rpc::EthereumRuntimeRPCApi<Block>,
	C::Api: RuntimeApiCollection<StateBackend = BE::State>,
	P: TransactionPool<Block = Block> + 'static,
	BE: Backend<Block> + Send + Sync + 'static,
	BE::State: StateBackend<BlakeTwo256>,
	A: ChainApi<Block = Block> + 'static,
{
	use pallet_contracts_rpc::{Contracts, ContractsApi};
	use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApi};
	use sc_rpc::dev::{Dev, DevApi};
	use substrate_frame_rpc_system::{FullSystem, SystemApi};

	use fc_rpc::{
		Eth, EthApi, EthDevSigner, EthFilter, EthFilterApi, EthPubSub, EthPubSubApi, EthSigner,
		HexEncodedIdProvider, Net, NetApi, Web3, Web3Api,
	};

	let mut io = jsonrpc_core::IoHandler::default();
	let FullDeps { client, pool, deny_unsafe, grandpa,
		
		graph, is_authority, enable_dev_signer, network, filter_pool,
		ethapi_cmd, backend, max_past_logs, fee_history_cache, 
		fee_history_cache_limit, overrides, block_data_cache,
		command_sink, } = deps;

	let GrandpaDeps {
		shared_voter_state,
		shared_authority_set,
		justification_stream,
		subscription_executor,
		finality_provider,
	} = grandpa;

	io.extend_with(SystemApi::to_delegate(FullSystem::new(client.clone(), pool.clone(), deny_unsafe)));
	// Making synchronous calls in light client freezes the browser currently,
	// more context: https://github.com/paritytech/substrate/pull/3480
	// These RPCs should use an asynchronous caller instead.
	io.extend_with(ContractsApi::to_delegate(Contracts::new(client.clone())));	
	io.extend_with(TransactionPaymentApi::to_delegate(TransactionPayment::new(client.clone())));

	io.extend_with(sc_finality_grandpa_rpc::GrandpaApi::to_delegate(GrandpaRpcHandler::new(
		shared_authority_set.clone(),
		shared_voter_state,
		justification_stream,
		subscription_executor,
		finality_provider,
	)));
	// io.extend_with(substrate_state_trie_migration_rpc::StateMigrationApi::to_delegate(
	// 	substrate_state_trie_migration_rpc::MigrationRpc::new(client.clone(), backend, deny_unsafe),
	// ));
	// io.extend_with(sc_sync_state_rpc::SyncStateRpcApi::to_delegate(
	// 	sc_sync_state_rpc::SyncStateRpcHandler::new(
	// 		chain_spec,
	// 		client.clone(),
	// 		shared_authority_set,
	// 		shared_epoch_changes,
	// 	)?,
	// ));
	io.extend_with(DevApi::to_delegate(Dev::new(client.clone(), deny_unsafe)));
	let mut signers = Vec::new();
	if enable_dev_signer {
		signers.push(Box::new(EthDevSigner::new()) as Box<dyn EthSigner>);
	}

	io.extend_with(EthApi::to_delegate(Eth::new(
		client.clone(),
		pool.clone(),
		graph.clone(),
		Some(edgeware_runtime::TransactionConverter),
		network.clone(),
		signers,
		overrides.clone(),
		backend.clone(),
		is_authority,
		block_data_cache.clone(),
		fee_history_cache,
		fee_history_cache_limit,
	)));

	if let Some(filter_pool) = filter_pool {
		io.extend_with(EthFilterApi::to_delegate(EthFilter::new(
			client.clone(),
			backend,
			filter_pool.clone(),
			500 as usize, // max stored filters
			max_past_logs,
			block_data_cache.clone(),
		)));
	}

	io.extend_with(NetApi::to_delegate(Net::new(
		client.clone(),
		network.clone(),
		// Whether to format the `peer_count` response as Hex (default) or not.
		true,
	)));

	io.extend_with(Web3Api::to_delegate(Web3::new(client.clone())));

	io.extend_with(EthPubSubApi::to_delegate(EthPubSub::new(
		pool.clone(),
		client.clone(),
		network.clone(),
		SubscriptionManager::<HexEncodedIdProvider>::with_id_provider(
			HexEncodedIdProvider::default(),
			Arc::new(subscription_task_executor),
		),
		overrides,
	)));
	if ethapi_cmd.contains(&EthApiCmd::Txpool) {
		io.extend_with(TxPoolServer::to_delegate(TxPool::new(
			Arc::clone(&client),
			graph,
		)));
	}

	if let Some(command_sink) = command_sink {
		io.extend_with(
			// We provide the rpc handler with the sending end of the channel to allow the rpc
			// send EngineCommands to the background block authorship task.
			ManualSealApi::to_delegate(ManualSeal::new(command_sink)),
		);
	}

	io
}

#[allow(missing_docs)]
pub struct SpawnTasksParams<'a, B: BlockT, C, BE> {
	pub task_manager: &'a TaskManager,
	pub client: Arc<C>,
	pub substrate_backend: Arc<BE>,
	pub frontier_backend: Arc<fc_db::Backend<B>>,
	pub filter_pool: Option<FilterPool>,
	pub overrides: Arc<OverrideHandle<B>>,
	pub fee_history_limit: u64,
	pub fee_history_cache: FeeHistoryCache,
}

/// Spawn the tasks that are required to run Edgeware.
pub fn spawn_essential_tasks<B, C, BE>(params: SpawnTasksParams<B, C, BE>)
where
	C: ProvideRuntimeApi<B> + BlockOf,
	C: HeaderBackend<B> + HeaderMetadata<B, Error = BlockChainError> + 'static,
	C: BlockchainEvents<B> + StorageProvider<B, BE>,
	C: Send + Sync + 'static,
	C::Api: fp_rpc::EthereumRuntimeRPCApi<B>,
	C::Api: BlockBuilder<B>,
	B: BlockT<Hash = H256> + Send + Sync + 'static,
	B::Header: HeaderT<Number = u32>,
	BE: Backend<B> + 'static,
	BE::State: StateBackend<BlakeTwo256>,
{
	// Frontier offchain DB task. Essential.
	// Maps emulated ethereum data to substrate native data.
	params.task_manager.spawn_essential_handle().spawn(
		"frontier-mapping-sync-worker",
		Some("frontier"),
		MappingSyncWorker::new(
			params.client.import_notification_stream(), //Notification stream
			Duration::new(6, 0), // timeout
			params.client.clone(), // client
			params.substrate_backend.clone(), // substrate backend
			params.frontier_backend.clone(), // frontier backend
            3,
            0,
			SyncStrategy::Normal, // Normal Non-parachain
		)
		.for_each(|()| futures::future::ready(())),
	);

	// Frontier `EthFilterApi` maintenance.
	// Manages the pool of user-created Filters.
	if let Some(filter_pool) = params.filter_pool {
		// Each filter is allowed to stay in the pool for 100 blocks.
		const FILTER_RETAIN_THRESHOLD: u64 = 100;
		params.task_manager.spawn_essential_handle().spawn(
			"frontier-filter-pool",
			Some("frontier"),
			EthTask::filter_pool_task(
				Arc::clone(&params.client),
				filter_pool,
				FILTER_RETAIN_THRESHOLD,
			),
		);
	}

	params.task_manager.spawn_essential_handle().spawn(
		"frontier-schema-cache-task",
		Some("frontier"),
		EthTask::ethereum_schema_cache_task(
			Arc::clone(&params.client),
			Arc::clone(&params.frontier_backend),
		),
	);

	// Spawn Frontier FeeHistory cache maintenance task.
	params.task_manager.spawn_essential_handle().spawn(
		"frontier-fee-history",
		Some("frontier"),
		EthTask::fee_history_task(
			Arc::clone(&params.client),
			Arc::clone(&params.overrides),
			params.fee_history_cache,
			params.fee_history_limit,
		),
	);
}
