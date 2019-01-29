//! The Edgeware runtime. This can be compiled with `#[no_std]`, ready for Wasm.

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(alloc))]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]

extern crate sr_io as runtime_io;
extern crate sr_std as rstd;
#[macro_use]
extern crate substrate_client as client;
#[macro_use]
extern crate srml_support;
#[macro_use]
extern crate sr_primitives as runtime_primitives;
#[cfg(feature = "std")]
#[macro_use]
extern crate serde_derive;
extern crate parity_codec;
extern crate substrate_primitives as primitives;
#[macro_use]
extern crate parity_codec_derive;
#[macro_use]
extern crate sr_version as version;
extern crate edge_delegation;
extern crate edge_governance;
extern crate edge_identity;
extern crate edge_voting;
extern crate srml_aura as aura;
extern crate srml_balances as balances;
extern crate srml_consensus as consensus;
extern crate srml_contract as contract;
extern crate srml_executive as executive;
extern crate srml_grandpa as grandpa;
extern crate srml_session as session;
extern crate srml_staking as staking;
extern crate srml_system as system;
extern crate srml_timestamp as timestamp;
extern crate srml_treasury as treasury;
extern crate srml_upgrade_key as upgrade_key;
extern crate node_primitives;
extern crate substrate_consensus_aura_primitives as consensus_aura;

use edge_delegation::delegation;
use edge_governance::governance;
use edge_identity::identity;
use edge_voting::voting;

use client::{block_builder::api as block_builder_api, runtime_api};
use consensus_aura::api as aura_api;
use primitives::OpaqueMetadata;
use node_primitives::{
	AccountId, AccountIndex, Balance, BlockNumber, Hash, Index, SessionKey, Signature
};
use rstd::prelude::*;
use runtime_primitives::{
	generic,
	traits::{BlakeTwo256, Block as BlockT, Convert, ProvideInherent, NumberFor, DigestFor},
	transaction_validity::TransactionValidity,
	ApplyResult, BasicInherentData, CheckInherentError,
};
use grandpa::fg_primitives::{self, ScheduledChange};
#[cfg(feature = "std")]
use version::NativeVersion;
use version::RuntimeVersion;

// A few exports that help ease life for downstream crates.
pub use balances::Call as BalancesCall;
pub use consensus::Call as ConsensusCall;
#[cfg(any(feature = "std", test))]
pub use runtime_primitives::BuildStorage;
pub use runtime_primitives::{Perbill, Permill};
pub use srml_support::{RuntimeMetadata, StorageValue};
pub use timestamp::BlockPeriod;
pub use timestamp::Call as TimestampCall;

/// This runtime version.
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: create_runtime_str!("edgeware"),
	impl_name: create_runtime_str!("edgeware"),
	authoring_version: 1,
	spec_version: 1,
	impl_version: 0,
	apis: RUNTIME_API_VERSIONS,
};

/// The version infromation used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
	NativeVersion {
		runtime_version: VERSION,
		can_author_with: Default::default(),
	}
}

impl system::Trait for Runtime {
	/// The identifier used to distinguish between accounts.
	type AccountId = AccountId;
	/// The index type for storing how many extrinsics an account has signed.
	type Index = Index;
	/// The index type for blocks.
	type BlockNumber = BlockNumber;
	/// The type for hashing blocks and tries.
	type Hash = Hash;
	/// The hashing algorithm used.
	type Hashing = BlakeTwo256;
	/// The header digest type.
	type Digest = generic::Digest<Log>;
	/// The header type.
	type Header = generic::Header<BlockNumber, BlakeTwo256, Log>;
	/// The ubiquitous event type.
	type Event = Event;
	/// The ubiquitous log type.
	type Log = Log;
	/// The ubiquitous origin type.
	type Origin = Origin;
}

impl aura::Trait for Runtime {
	type HandleReport = aura::StakingSlasher<Runtime>;
}

impl consensus::Trait for Runtime {
	/// The position in the block's extrinsics that the note-offline inherent must be placed.
	const NOTE_OFFLINE_POSITION: u32 = 1;
	/// The identifier we use to refer to authorities.
	type SessionKey = SessionKey;
	// The aura module handles offline-reports internally
	// rather than using an explicit report system.
	type InherentOfflineReport = ();
	/// The ubiquitous log type.
	type Log = Log;
}

impl timestamp::Trait for Runtime {
	/// The position in the block's extrinsics that the timestamp-set inherent must be placed.
	const TIMESTAMP_SET_POSITION: u32 = 0;
	/// A timestamp: seconds since the unix epoch.
	type Moment = u64;
	type OnTimestampSet = Aura;
}

/// Session key conversion.
pub struct SessionKeyConversion;
impl Convert<AccountId, SessionKey> for SessionKeyConversion {
	fn convert(a: AccountId) -> SessionKey {
		a.to_fixed_bytes().into()
	}
}

impl session::Trait for Runtime {
	type ConvertAccountIdToSessionKey = SessionKeyConversion;
	type OnSessionChange = (Staking, grandpa::SyncedAuthorities<Runtime>);
	type Event = Event;
}

impl balances::Trait for Runtime {
	/// The type for recording an account's balance.
	type Balance = Balance;
	/// The type for recording indexing into the account enumeration.
	type AccountIndex = AccountIndex;
	/// What to do if an account's free balance gets zeroed.
	type OnFreeBalanceZero = ((Staking, Contract), ());
	/// Restrict whether an account can transfer funds. We don't place any further restrictions.
	type EnsureAccountLiquid = Staking;
	/// The uniquitous event type.
	type Event = Event;
}

impl upgrade_key::Trait for Runtime {
	/// The uniquitous event type.
	type Event = Event;
}

impl staking::Trait for Runtime {
	type OnRewardMinted = Treasury;
	type Event = Event;
}

// TODO: replace ApproveOrigin and RejectOrigin with voting-related origins
impl treasury::Trait for Runtime {
	type ApproveOrigin = system::EnsureRoot<AccountId>;
	type RejectOrigin = system::EnsureRoot<AccountId>;
	type Event = Event;
}

impl contract::Trait for Runtime {
	type Gas = u64;
	type DetermineContractAddress = contract::SimpleAddressDeterminator<Runtime>;
	type Event = Event;
}

impl grandpa::Trait for Runtime {
	type SessionKey = SessionKey;
	type Log = Log;
	type Event = Event;
}

impl delegation::Trait for Runtime {
	/// The uniquitous event type.
	type Event = Event;
}

impl voting::Trait for Runtime {
	/// The uniquitous event type.
	type Event = Event;
}

impl governance::Trait for Runtime {
	/// The uniquitous event type.
	type Event = Event;
}

impl identity::Trait for Runtime {
	/// The type for making a claim to an identity.
	type Claim = Vec<u8>;
	/// The uniquitous event type.
	type Event = Event;
}

construct_runtime!(
	pub enum Runtime with Log(InternalLog: DigestItem<Hash, SessionKey>) where
		Block = Block,
		NodeBlock = node_primitives::Block,
		InherentData = BasicInherentData
	{
		System: system::{default, Log(ChangesTrieRoot)},
		Timestamp: timestamp::{Module, Call, Storage, Config<T>, Inherent},
		Consensus: consensus::{Module, Call, Storage, Config<T>, Log(AuthoritiesChange), Inherent},
		Aura: aura::{Module},
		Balances: balances,
		Session: session,
		Staking: staking,
		UpgradeKey: upgrade_key,
		Grandpa: grandpa::{Module, Call, Storage, Config<T>, Log(), Event<T>},
		Contract: contract::{Module, Call, Config<T>, Event<T>},
		Treasury: treasury,
		Delegation: delegation::{Module, Call, Storage, Config<T>, Event<T>},
		Voting: voting::{Module, Call, Storage, Event<T>},
		Governance: governance::{Module, Call, Storage, Config<T>, Event<T>},
		Identity: identity::{Module, Call, Storage, Config<T>, Event<T>},
	}
);

/// The type used as a helper for interpreting the sender of transactions.
type Context = balances::ChainContext<Runtime>;
/// The address format for describing accounts.
type Address = balances::Address<Runtime>;
/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256, Log>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;
/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic =
	generic::UncheckedMortalCompactExtrinsic<Address, Index, Call, Signature>;
/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, Index, Call>;
/// Executive: handles dispatch to the various modules.
pub type Executive = executive::Executive<Runtime, Block, Context, Balances, AllModules>;

// Implement our runtime API endpoints. This is just a bunch of proxying.
impl_runtime_apis! {
	impl runtime_api::Core<Block> for Runtime {
		fn version() -> RuntimeVersion {
			VERSION
		}

		fn authorities() -> Vec<SessionKey> {
			Consensus::authorities()
		}

		fn execute_block(block: Block) {
			Executive::execute_block(block)
		}

		fn initialise_block(header: <Block as BlockT>::Header) {
			Executive::initialise_block(&header)
		}
	}

	impl runtime_api::Metadata<Block> for Runtime {
		fn metadata() -> OpaqueMetadata {
			Runtime::metadata().into()
		}
	}

	impl block_builder_api::BlockBuilder<Block, BasicInherentData> for Runtime {
		fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyResult {
			Executive::apply_extrinsic(extrinsic)
		}

		fn finalise_block() -> <Block as BlockT>::Header {
			Executive::finalise_block()
		}

		fn inherent_extrinsics(data: BasicInherentData) -> Vec<<Block as BlockT>::Extrinsic> {
			let mut inherent = Vec::new();

			inherent.extend(
				Timestamp::create_inherent_extrinsics(data.timestamp)
					.into_iter()
					.map(|v| (v.0, UncheckedExtrinsic::new_unsigned(Call::Timestamp(v.1))))
			);

			inherent.extend(
				Consensus::create_inherent_extrinsics(data.consensus)
					.into_iter()
					.map(|v| (v.0, UncheckedExtrinsic::new_unsigned(Call::Consensus(v.1))))
			);

			inherent.as_mut_slice().sort_unstable_by_key(|v| v.0);
			inherent.into_iter().map(|v| v.1).collect()
		}

		fn check_inherents(block: Block, data: BasicInherentData) -> Result<(), CheckInherentError> {
			Runtime::check_inherents(block, data)
		}

		fn random_seed() -> <Block as BlockT>::Hash {
			System::random_seed()
		}
	}

	impl runtime_api::TaggedTransactionQueue<Block> for Runtime {
		fn validate_transaction(tx: <Block as BlockT>::Extrinsic) -> TransactionValidity {
			Executive::validate_transaction(tx)
		}
	}

	impl fg_primitives::GrandpaApi<Block> for Runtime {
		fn grandpa_pending_change(digest: DigestFor<Block>)
			-> Option<ScheduledChange<NumberFor<Block>>>
		{
			for log in digest.logs.iter().filter_map(|l| match l {
				Log(InternalLog::grandpa(grandpa_signal)) => Some(grandpa_signal),
				_=> None
			}) {
				if let Some(change) = Grandpa::scrape_digest_change(log) {
					return Some(change);
				}
			}
			None
		}

		fn grandpa_authorities() -> Vec<(SessionKey, u64)> {
			Grandpa::grandpa_authorities()
		}
	}

	impl aura_api::AuraApi<Block> for Runtime {
		fn slot_duration() -> u64 {
			Aura::slot_duration()
		}
	}
}
