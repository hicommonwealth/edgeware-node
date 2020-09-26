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

use std::{sync::Arc, fmt};
use edgeware_primitives::{AccountId, Balance, Block, BlockNumber, Hash, Index};
use jsonrpc_pubsub::manager::SubscriptionManager;
use sc_finality_grandpa::{SharedAuthoritySet, SharedVoterState, GrandpaJustificationStream};
use sc_finality_grandpa_rpc::GrandpaRpcHandler;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};
use sp_consensus::SelectChain;
use sp_transaction_pool::TransactionPool;
pub use sc_rpc_api::DenyUnsafe;
use sp_runtime::traits::BlakeTwo256;
use sc_client_api::backend::{StorageProvider, Backend, StateBackend, AuxStore};

use sp_block_builder::BlockBuilder;

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
pub struct GrandpaDeps {
	/// Voting round info.
	pub shared_voter_state: SharedVoterState,
	/// Authority set info.
	pub shared_authority_set: SharedAuthoritySet<Hash, BlockNumber>,
	/// Receives notifications about justification events from Grandpa.
	pub justification_stream: GrandpaJustificationStream<Block>,
	/// Subscription manager to keep track of pubsub subscribers.
	pub subscriptions: SubscriptionManager,
}

/// Full client dependencies.
pub struct FullDeps<C, P, SC> {
	/// The client instance to use.
	pub client: Arc<C>,
	/// Transaction pool instance.
	pub pool: Arc<P>,
	/// The SelectChain Strategy
	pub select_chain: SC,
	/// Whether to deny unsafe calls
	pub deny_unsafe: DenyUnsafe,
	/// The Node authority flag
	pub is_authority: bool,
	/// GRANDPA specific dependencies.
	pub grandpa: GrandpaDeps,
}


/// Instantiate all Full RPC extensions.
pub fn create_full<C, P, SC, BE>(
	deps: FullDeps<C, P, SC>,
) -> jsonrpc_core::IoHandler<sc_rpc_api::Metadata> where
	BE: Backend<Block> + 'static,
	BE::State: StateBackend<BlakeTwo256>,
	C: ProvideRuntimeApi<Block> + StorageProvider<Block, BE> + AuxStore,
	C: HeaderBackend<Block> + HeaderMetadata<Block, Error=BlockChainError> + 'static,
	C: Send + Sync + 'static,
	C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Index>,	
	C::Api: pallet_contracts_rpc::ContractsRuntimeApi<Block, AccountId, Balance, BlockNumber>,
	C::Api: BlockBuilder<Block>,
	C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
	C::Api: frontier_rpc_primitives::EthereumRuntimeRPCApi<Block>,
	<C::Api as sp_api::ApiErrorExt>::Error: fmt::Debug,
	P: TransactionPool<Block=Block> + 'static,
	SC: SelectChain<Block> +'static,
{
	use substrate_frame_rpc_system::{FullSystem, SystemApi};
	use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApi};
	use frontier_rpc::{EthApi, EthApiServer, NetApi, NetApiServer};
	use pallet_contracts_rpc::{Contracts, ContractsApi};

	let mut io = jsonrpc_core::IoHandler::default();

	let FullDeps {
		client,
		pool,
		select_chain,
		deny_unsafe,
		is_authority,
		// command_sink,
		grandpa
	} = deps;
	let GrandpaDeps {
		shared_voter_state,
		shared_authority_set,
		justification_stream,
		subscriptions,
	} = grandpa;

	io.extend_with(
		SystemApi::to_delegate(FullSystem::new(client.clone(), pool.clone(), deny_unsafe))
	);
	// Making synchronous calls in light client freezes the browser currently,
	// more context: https://github.com/paritytech/substrate/pull/3480
	// These RPCs should use an asynchronous caller instead.
	io.extend_with(
		ContractsApi::to_delegate(Contracts::new(client.clone()))
	);
	io.extend_with(
		TransactionPaymentApi::to_delegate(TransactionPayment::new(client.clone()))
	);
	io.extend_with(
		EthApiServer::to_delegate(EthApi::new(
			client.clone(),
			select_chain.clone(),
			pool.clone(),
			edgeware_runtime::TransactionConverter,
			is_authority,
		))
	);
	io.extend_with(
		NetApiServer::to_delegate(NetApi::new(
			client.clone(),
			select_chain.clone(),
		))
	);

	io.extend_with(
		sc_finality_grandpa_rpc::GrandpaApi::to_delegate(
			GrandpaRpcHandler::new(
				shared_authority_set,
				shared_voter_state,
				justification_stream,
				subscriptions,
			)
		)
	);

	io
}

/// Instantiate all Light RPC extensions.
pub fn create_light<C, P, M, F>(
	deps: LightDeps<C, F, P>,
) -> jsonrpc_core::IoHandler<M> where
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
		fetcher
	} = deps;
	let mut io = jsonrpc_core::IoHandler::default();
	io.extend_with(
		SystemApi::<Hash, AccountId, Index>::to_delegate(LightSystem::new(client, remote_blockchain, fetcher, pool))
	);

	io
}
