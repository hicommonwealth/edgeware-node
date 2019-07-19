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

//! The Edgeware runtime. This can be compiled with `#[no_std]`, ready for Wasm.

#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit="256"]

use edge_governance::governance;
use edge_identity::identity;
use edge_voting::voting;

use rstd::prelude::*;
use support::{
	construct_runtime, parameter_types, traits::{SplitTwoWays, Currency, OnUnbalanced}
};
use substrate_primitives::u32_trait::{_1, _2, _3, _4};
use edgeware_primitives::{
	AccountId, AccountIndex, AuraId, Balance, BlockNumber, Hash, Index,
	Moment, Signature,
};
use grandpa::fg_primitives::{self, ScheduledChange};
use client::{
	block_builder::api::{self as block_builder_api, InherentData, CheckInherentsResult},
	runtime_api as client_api, impl_runtime_apis
};
use runtime_primitives::{ApplyResult, impl_opaque_keys, generic, create_runtime_str, key_types};
use runtime_primitives::transaction_validity::TransactionValidity;
use runtime_primitives::traits::{
	BlakeTwo256, Block as BlockT, DigestFor, NumberFor, StaticLookup,
};
use version::RuntimeVersion;
use elections::VoteIndex;
#[cfg(any(feature = "std", test))]
use version::NativeVersion;
use substrate_primitives::OpaqueMetadata;
use grandpa::{AuthorityId as GrandpaId, AuthorityWeight as GrandpaWeight};
use finality_tracker::{DEFAULT_REPORT_LATENCY, DEFAULT_WINDOW_SIZE};

#[cfg(any(feature = "std", test))]
pub use runtime_primitives::BuildStorage;
pub use timestamp::Call as TimestampCall;
pub use balances::Call as BalancesCall;
pub use contracts::Gas;
pub use runtime_primitives::{Permill, Perbill};
pub use support::StorageValue;
pub use staking::StakerStatus;

use runtime_primitives::weights::{Weight, WeightMultiplier, MAX_TRANSACTIONS_WEIGHT, IDEAL_TRANSACTIONS_WEIGHT};
use runtime_primitives::traits::{Convert, Saturating};
use runtime_primitives::Fixed64;

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

/// This runtime version.
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: create_runtime_str!("edgeware"),
	impl_name: create_runtime_str!("edgeware-node"),
	authoring_version: 10,
	// Per convention: if the runtime behavior changes, increment spec_version and set impl_version
	// to equal spec_version. If only runtime implementation changes and behavior does not, then
	// leave spec_version as is and increment impl_version.
	spec_version: 12,
	impl_version: 14,
	apis: RUNTIME_API_VERSIONS,
};

/// Native version.
#[cfg(any(feature = "std", test))]
pub fn native_version() -> NativeVersion {
	NativeVersion {
		runtime_version: VERSION,
		can_author_with: Default::default(),
	}
}

/// Struct that handles the conversion of Balance -> `u64`. This is used for staking's election
/// calculation.
pub struct CurrencyToVoteHandler;

impl CurrencyToVoteHandler {
	fn factor() -> Balance { (Balances::total_issuance() / u64::max_value() as Balance).max(1) }
}

impl Convert<Balance, u64> for CurrencyToVoteHandler {
	fn convert(x: Balance) -> u64 { (x / Self::factor()) as u64 }
}

impl Convert<u128, Balance> for CurrencyToVoteHandler {
	fn convert(x: u128) -> Balance { x * Self::factor() }
}

/// A struct that updates the weight multiplier based on the saturation level of the previous block.
/// This should typically be called once per-block.
///
/// This assumes that weight is a numeric value in the u32 range.
///
/// Formula:
///   diff = (ideal_weight - current_block_weight)
///   v = 0.00004
///   next_weight = weight * (1 + (v . diff) + (v . diff)^2 / 2)
///
/// https://research.web3.foundation/en/latest/polkadot/Token%20Economics/#relay-chain-transaction-fees
pub struct WeightMultiplierUpdateHandler;
impl Convert<(Weight, WeightMultiplier), WeightMultiplier> for WeightMultiplierUpdateHandler {
	fn convert(previous_state: (Weight, WeightMultiplier)) -> WeightMultiplier {
		let (block_weight, multiplier) = previous_state;
		let ideal = IDEAL_TRANSACTIONS_WEIGHT as u128;
		let block_weight = block_weight as u128;

		// determines if the first_term is positive
		let positive = block_weight >= ideal;
		let diff_abs = block_weight.max(ideal) - block_weight.min(ideal);
		// diff is within u32, safe.
		let diff = Fixed64::from_rational(diff_abs as i64, MAX_TRANSACTIONS_WEIGHT as u64);
		let diff_squared = diff.saturating_mul(diff);

		// 0.00004 = 4/100_000 = 40_000/10^9
		let v = Fixed64::from_rational(4, 100_000);
		// 0.00004^2 = 16/10^10 ~= 2/10^9. Taking the future /2 into account, then it is just 1 parts
		// from a billionth.
		let v_squared_2 = Fixed64::from_rational(1, 1_000_000_000);

		let first_term = v.saturating_mul(diff);
		// It is very unlikely that this will exist (in our poor perbill estimate) but we are giving
		// it a shot.
		let second_term = v_squared_2.saturating_mul(diff_squared);

		if positive {
			let excess = first_term.saturating_add(second_term);
			multiplier.saturating_add(WeightMultiplier::from_fixed(excess))
		} else {
			// first_term > second_term
			let negative = first_term - second_term;
			multiplier.saturating_sub(WeightMultiplier::from_fixed(negative))
				// despite the fact that apply_to saturates weight (final fee cannot go below 0)
				// it is crucially important to stop here and don't further reduce the weight fee
				// multiplier. While at -1, it means that the network is so un-congested that all
				// transactions are practically free. We stop here and only increase if the network
				// became more busy.
				.max(WeightMultiplier::from_rational(-1, 1))
		}
	}
}

pub const MILLICENTS: Balance = 1_000_000_000;
pub const CENTS: Balance = 1_000 * MILLICENTS;    // assume this is worth about a cent.
pub const DOLLARS: Balance = 100 * CENTS;

type NegativeImbalance = <Balances as Currency<AccountId>>::NegativeImbalance;

pub struct Author;

impl OnUnbalanced<NegativeImbalance> for Author {
	fn on_unbalanced(amount: NegativeImbalance) {
		Balances::resolve_creating(&Authorship::author(), amount);
	}
}

pub type DealWithFees = SplitTwoWays<
	Balance,
	NegativeImbalance,
	_4, Treasury,   // 4 parts (80%) goes to the treasury.
	_1, Author,     // 1 part (20%) goes to the block author.
>;

pub const SECS_PER_BLOCK: Moment = 6;
pub const MINUTES: Moment = 60 / SECS_PER_BLOCK;
pub const HOURS: Moment = MINUTES * 60;
pub const DAYS: Moment = HOURS * 24;

parameter_types! {
	pub const BlockHashCount: BlockNumber = 250;
}

impl system::Trait for Runtime {
	type Origin = Origin;
	type Index = Index;
	type BlockNumber = BlockNumber;
	type Hash = Hash;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = Indices;
	type Header = generic::Header<BlockNumber, BlakeTwo256>;
	type WeightMultiplierUpdate = WeightMultiplierUpdateHandler;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
}

impl aura::Trait for Runtime {
	type HandleReport = aura::StakingSlasher<Runtime>;
	type AuthorityId = AuraId;
}

impl indices::Trait for Runtime {
	type AccountIndex = AccountIndex;
	type IsDeadAccount = Balances;
	type ResolveHint = indices::SimpleResolveHint<Self::AccountId, Self::AccountIndex>;
	type Event = Event;
}

parameter_types! {
	pub const ExistentialDeposit: Balance = 1 * DOLLARS;
	pub const TransferFee: Balance = 1 * CENTS;
	pub const CreationFee: Balance = 1 * CENTS;
	pub const TransactionBaseFee: Balance = 1 * CENTS;
	pub const TransactionByteFee: Balance = 10 * MILLICENTS;
}

impl balances::Trait for Runtime {
	type Balance = Balance;
	type OnFreeBalanceZero = ((Staking, Contracts), Session);
	type OnNewAccount = Indices;
	type Event = Event;
	type TransactionPayment = DealWithFees;
	type DustRemoval = ();
	type TransferPayment = ();
	type ExistentialDeposit = ExistentialDeposit;
	type TransferFee = TransferFee;
	type CreationFee = CreationFee;
	type TransactionBaseFee = TransactionBaseFee;
	type TransactionByteFee = TransactionByteFee;
}

parameter_types! {
	pub const MinimumPeriod: u64 = SECS_PER_BLOCK / 2;
}
impl timestamp::Trait for Runtime {
	type Moment = Moment;
	type OnTimestampSet = Aura;
	type MinimumPeriod = MinimumPeriod;
}

parameter_types! {
	pub const UncleGenerations: u64 = 0;
}

// TODO: #2986 implement this properly
impl authorship::Trait for Runtime {
	type FindAuthor = ();
	type UncleGenerations = UncleGenerations;
	type FilterUncle = ();
	type EventHandler = ();
}

parameter_types! {
	pub const Period: BlockNumber = 10 * MINUTES;
	pub const Offset: BlockNumber = 0;
}

type SessionHandlers = (Grandpa, Aura);

impl_opaque_keys! {
	pub struct SessionKeys {
		#[id(key_types::ED25519)]
		pub ed25519: GrandpaId,
	}
}

// NOTE: `SessionHandler` and `SessionKeys` are co-dependent: One key will be used for each handler.
// The number and order of items in `SessionHandler` *MUST* be the same number and order of keys in
// `SessionKeys`.
// TODO: Introduce some structure to tie these together to make it a bit less of a footgun. This
// should be easy, since OneSessionHandler trait provides the `Key` as an associated type. #2858

impl session::Trait for Runtime {
	type OnSessionEnding = Staking;
	type SessionHandler = SessionHandlers;
	type ShouldEndSession = session::PeriodicSessions<Period, Offset>;
	type Event = Event;
	type Keys = SessionKeys;
	type ValidatorId = AccountId;
	type ValidatorIdOf = staking::StashOf<Self>;
	type SelectInitialValidators = Staking;
}

impl session::historical::Trait for Runtime {
	type FullIdentification = staking::Exposure<AccountId, Balance>;
	type FullIdentificationOf = staking::ExposureOf<Runtime>;
}

parameter_types! {
	pub const SessionsPerEra: session::SessionIndex = 6;
	pub const BondingDuration: staking::EraIndex = 24 * 28;
}

impl staking::Trait for Runtime {
	type Currency = Balances;
	type CurrencyToVote = CurrencyToVoteHandler;
	type OnRewardMinted = Treasury;
	type Event = Event;
	type Slash = ();
	type Reward = ();
	type SessionsPerEra = SessionsPerEra;
	type BondingDuration = BondingDuration;
	type SessionInterface = Self;
}

parameter_types! {
	pub const LaunchPeriod: BlockNumber = 28 * 24 * 60 * MINUTES;
	pub const VotingPeriod: BlockNumber = 28 * 24 * 60 * MINUTES;
	pub const EmergencyVotingPeriod: BlockNumber = 3 * 24 * 60 * MINUTES;
	pub const MinimumDeposit: Balance = 100 * DOLLARS;
	pub const EnactmentPeriod: BlockNumber = 30 * 24 * 60 * MINUTES;
	pub const CooloffPeriod: BlockNumber = 30 * 24 * 60 * MINUTES;
}

impl democracy::Trait for Runtime {
	type Proposal = Call;
	type Event = Event;
	type Currency = Balances;
	type EnactmentPeriod = EnactmentPeriod;
	type LaunchPeriod = LaunchPeriod;
	type VotingPeriod = VotingPeriod;
	type EmergencyVotingPeriod = EmergencyVotingPeriod;
	type MinimumDeposit = MinimumDeposit;
	type ExternalOrigin = collective::EnsureProportionAtLeast<_1, _2, AccountId, CouncilInstance>;
	type ExternalMajorityOrigin = collective::EnsureProportionAtLeast<_2, _3, AccountId, CouncilInstance>;
	type ExternalPushOrigin = collective::EnsureProportionAtLeast<_2, _3, AccountId, TechnicalInstance>;
	type EmergencyOrigin = collective::EnsureProportionAtLeast<_1, _1, AccountId, CouncilInstance>;
	type CancellationOrigin = collective::EnsureProportionAtLeast<_2, _3, AccountId, CouncilInstance>;
	type VetoOrigin = collective::EnsureMember<AccountId, CouncilInstance>;
	type CooloffPeriod = CooloffPeriod;
}

type CouncilInstance = collective::Instance1;
impl collective::Trait<CouncilInstance> for Runtime {
	type Origin = Origin;
	type Proposal = Call;
	type Event = Event;
}

parameter_types! {
	pub const CandidacyBond: Balance = 10 * DOLLARS;
	pub const VotingBond: Balance = 1 * DOLLARS;
	pub const VotingFee: Balance = 2 * DOLLARS;
	pub const PresentSlashPerVoter: Balance = 1 * CENTS;
	pub const CarryCount: u32 = 6;
	// one additional vote should go by before an inactive voter can be reaped.
	pub const InactiveGracePeriod: VoteIndex = 1;
	pub const ElectionsVotingPeriod: BlockNumber = 2 * DAYS;
	pub const DecayRatio: u32 = 0;
}

impl elections::Trait for Runtime {
	type Event = Event;
	type Currency = Balances;
	type BadPresentation = ();
	type BadReaper = ();
	type BadVoterIndex = ();
	type LoserCandidate = ();
	type ChangeMembers = Council;
	type CandidacyBond = CandidacyBond;
	type VotingBond = VotingBond;
	type VotingFee = VotingFee;
	type PresentSlashPerVoter = PresentSlashPerVoter;
	type CarryCount = CarryCount;
	type InactiveGracePeriod = InactiveGracePeriod;
	type VotingPeriod = ElectionsVotingPeriod;
	type DecayRatio = DecayRatio;
}

type TechnicalInstance = collective::Instance2;
impl collective::Trait<TechnicalInstance> for Runtime {
	type Origin = Origin;
	type Proposal = Call;
	type Event = Event;
}

parameter_types! {
	pub const ProposalBond: Permill = Permill::from_percent(5);
	pub const ProposalBondMinimum: Balance = 1 * DOLLARS;
	pub const SpendPeriod: BlockNumber = 1 * DAYS;
	pub const Burn: Permill = Permill::from_percent(50);
}

impl treasury::Trait for Runtime {
	type Currency = Balances;
	type ApproveOrigin = collective::EnsureMembers<_4, AccountId, CouncilInstance>;
	type RejectOrigin = collective::EnsureMembers<_2, AccountId, CouncilInstance>;
	type Event = Event;
	type MintedForSpending = ();
	type ProposalRejection = ();
	type ProposalBond = ProposalBond;
	type ProposalBondMinimum = ProposalBondMinimum;
	type SpendPeriod = SpendPeriod;
	type Burn = Burn;
}

parameter_types! {
	pub const ContractTransferFee: Balance = 1 * CENTS;
	pub const ContractCreationFee: Balance = 1 * CENTS;
	pub const ContractTransactionBaseFee: Balance = 1 * CENTS;
	pub const ContractTransactionByteFee: Balance = 10 * MILLICENTS;
	pub const ContractFee: Balance = 1 * CENTS;
}

impl contracts::Trait for Runtime {
	type Currency = Balances;
	type Call = Call;
	type Event = Event;
	type DetermineContractAddress = contracts::SimpleAddressDeterminator<Runtime>;
	type ComputeDispatchFee = contracts::DefaultDispatchFeeComputor<Runtime>;
	type TrieIdGenerator = contracts::TrieIdFromParentCounter<Runtime>;
	type GasPayment = ();
	type SignedClaimHandicap = contracts::DefaultSignedClaimHandicap;
	type TombstoneDeposit = contracts::DefaultTombstoneDeposit;
	type StorageSizeOffset = contracts::DefaultStorageSizeOffset;
	type RentByteFee = contracts::DefaultRentByteFee;
	type RentDepositOffset = contracts::DefaultRentDepositOffset;
	type SurchargeReward = contracts::DefaultSurchargeReward;
	type TransferFee = ContractTransferFee;
	type CreationFee = ContractCreationFee;
	type TransactionBaseFee = ContractTransactionBaseFee;
	type TransactionByteFee = ContractTransactionByteFee;
	type ContractFee = ContractFee;
	type CallBaseFee = contracts::DefaultCallBaseFee;
	type CreateBaseFee = contracts::DefaultCreateBaseFee;
	type MaxDepth = contracts::DefaultMaxDepth;
	type MaxValueSize = contracts::DefaultMaxValueSize;
	type BlockGasLimit = contracts::DefaultBlockGasLimit;
}

impl sudo::Trait for Runtime {
	type Event = Event;
	type Proposal = Call;
}

impl grandpa::Trait for Runtime {
	type Event = Event;
}

parameter_types! {
	pub const WindowSize: BlockNumber = DEFAULT_WINDOW_SIZE.into();
	pub const ReportLatency: BlockNumber = DEFAULT_REPORT_LATENCY.into();
}

impl finality_tracker::Trait for Runtime {
	type OnFinalizationStalled = Grandpa;
	type WindowSize = WindowSize;
	type ReportLatency = ReportLatency;
}

impl voting::Trait for Runtime {
	type Event = Event;
}

impl governance::Trait for Runtime {
	type Event = Event;
	type Currency = Balances;
}

impl identity::Trait for Runtime {
	type Event = Event;
	type Currency = Balances;
}

construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = edgeware_primitives::Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: system::{Module, Call, Storage, Config, Event},
		Aura: aura::{Module, Call, Storage, Config<T>, Inherent(Timestamp)},
		Timestamp: timestamp::{Module, Call, Storage, Inherent},
		Authorship: authorship::{Module, Call, Storage},
		Indices: indices,
		Balances: balances,
		Staking: staking::{default, OfflineWorker},
		Session: session::{Module, Call, Storage, Event, Config<T>},
		Democracy: democracy::{Module, Call, Storage, Config, Event<T>},
		Council: collective::<Instance1>::{Module, Call, Storage, Origin<T>, Event<T>, Config<T>},
		TechnicalCommittee: collective::<Instance2>::{Module, Call, Storage, Origin<T>, Event<T>, Config<T>},
		Elections: elections::{Module, Call, Storage, Event<T>, Config<T>},
		FinalityTracker: finality_tracker::{Module, Call, Inherent},
		Grandpa: grandpa::{Module, Call, Storage, Config, Event},
		Treasury: treasury::{Module, Call, Storage, Event<T>},
		Contracts: contracts,
		Sudo: sudo,
		Identity: identity::{Module, Call, Storage, Config<T>, Event<T>},
		Voting: voting::{Module, Call, Storage, Event<T>},
		Governance: governance::{Module, Call, Storage, Config<T>, Event<T>},
	}
);

/// The address format for describing accounts.
pub type Address = <Indices as StaticLookup>::Source;
/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// A Block signed with a Justification
pub type SignedBlock = generic::SignedBlock<Block>;
/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;
/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic = generic::UncheckedMortalCompactExtrinsic<Address, Index, Call, Signature>;
/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, Index, Call>;
/// Executive: handles dispatch to the various modules.
pub type Executive = executive::Executive<Runtime, Block, system::ChainContext<Runtime>, Balances, Runtime, AllModules>;

impl_runtime_apis! {
	impl client_api::Core<Block> for Runtime {
		fn version() -> RuntimeVersion {
			VERSION
		}

		fn execute_block(block: Block) {
			Executive::execute_block(block)
		}

		fn initialize_block(header: &<Block as BlockT>::Header) {
			Executive::initialize_block(header)
		}
	}

	impl client_api::Metadata<Block> for Runtime {
		fn metadata() -> OpaqueMetadata {
			Runtime::metadata().into()
		}
	}

	impl block_builder_api::BlockBuilder<Block> for Runtime {
		fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyResult {
			Executive::apply_extrinsic(extrinsic)
		}

		fn finalize_block() -> <Block as BlockT>::Header {
			Executive::finalize_block()
		}

		fn inherent_extrinsics(data: InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
			data.create_extrinsics()
		}

		fn check_inherents(block: Block, data: InherentData) -> CheckInherentsResult {
			data.check_extrinsics(&block)
		}

		fn random_seed() -> <Block as BlockT>::Hash {
			System::random_seed()
		}
	}

	impl client_api::TaggedTransactionQueue<Block> for Runtime {
		fn validate_transaction(tx: <Block as BlockT>::Extrinsic) -> TransactionValidity {
			Executive::validate_transaction(tx)
		}
	}

	impl offchain_primitives::OffchainWorkerApi<Block> for Runtime {
		fn offchain_worker(number: NumberFor<Block>) {
			Executive::offchain_worker(number)
		}
	}

	impl fg_primitives::GrandpaApi<Block> for Runtime {
		fn grandpa_pending_change(digest: &DigestFor<Block>)
			-> Option<ScheduledChange<NumberFor<Block>>>
		{
			Grandpa::pending_change(digest)
		}

		fn grandpa_forced_change(digest: &DigestFor<Block>)
			-> Option<(NumberFor<Block>, ScheduledChange<NumberFor<Block>>)>
		{
			Grandpa::forced_change(digest)
		}

		fn grandpa_authorities() -> Vec<(GrandpaId, GrandpaWeight)> {
			Grandpa::grandpa_authorities()
		}
	}

	impl consensus_aura::AuraApi<Block, AuraId> for Runtime {
		fn slot_duration() -> u64 {
			Aura::slot_duration()
		}
		fn authorities() -> Vec<AuraId> {
			Aura::authorities()
		}
	}
}