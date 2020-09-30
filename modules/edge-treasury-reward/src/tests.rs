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

use super::*;
use sp_staking::SessionIndex;
use sp_runtime::traits::OpaqueKeys;
use sp_runtime::curve::PiecewiseLinear;
use sp_runtime::testing::{UintAuthorityId, TestXt};

#[cfg(feature = "std")]
use std::{collections::HashSet, cell::RefCell};

use sp_core::{H256, crypto::key_types};

use frame_support::{parameter_types, impl_outer_origin, impl_outer_dispatch, weights::Weight};
use frame_support::{traits::{Contains, ContainsLengthBound, OnFinalize}};

use sp_runtime::{
	Perbill, Permill, KeyTypeId, ModuleId,
	testing::{Header}, Percent,
	traits::{BlakeTwo256, IdentityLookup, One},
};

use crate::GenesisConfig;

/// The AccountId alias in this test module.
pub type AccountId = u64;
pub type Balance = u128;

/// Simple structure that exposes how u64 currency can be represented as... u64.
pub struct CurrencyToVoteHandler;
impl sp_runtime::traits::Convert<u64, u64> for CurrencyToVoteHandler {
	fn convert(x: u64) -> u64 { x }
}
impl sp_runtime::traits::Convert<u128, u64> for CurrencyToVoteHandler {
	fn convert(x: u128) -> u64 { x as u64 }
}
impl sp_runtime::traits::Convert<u128, u128> for CurrencyToVoteHandler {
	fn convert(x: u128) -> u128 { x }
}
impl sp_runtime::traits::Convert<u64, u128> for CurrencyToVoteHandler {
	fn convert(x: u64) -> u128 { x as u128 }
}

thread_local! {
	static SESSION: RefCell<(Vec<AccountId>, HashSet<AccountId>)> = RefCell::new(Default::default());
	static EXISTENTIAL_DEPOSIT: RefCell<u64> = RefCell::new(0);
}

pub struct TestSessionHandler;
impl pallet_session::SessionHandler<AccountId> for TestSessionHandler {
	const KEY_TYPE_IDS: &'static [KeyTypeId] = &[key_types::DUMMY];

	fn on_genesis_session<Ks: OpaqueKeys>(_validators: &[(AccountId, Ks)]) {}

	fn on_new_session<Ks: OpaqueKeys>(
		_changed: bool,
		validators: &[(AccountId, Ks)],
		_queued_validators: &[(AccountId, Ks)],
	) {
		SESSION.with(|x|
			*x.borrow_mut() = (validators.iter().map(|x| x.0.clone()).collect(), HashSet::new())
		);
	}

	fn on_disabled(validator_index: usize) {
		SESSION.with(|d| {
			let mut d = d.borrow_mut();
			let value = d.0[validator_index];
			d.1.insert(value);
		})
	}
}

impl_outer_origin! {
	pub enum Origin for Test  where system = frame_system { }
}

impl_outer_dispatch! {
	pub enum Call for Test where origin: Origin {
		pallet_staking::Staking,
	}
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Test;

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: Weight = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::one();
	pub const MaximumExtrinsicWeight: Weight = 1024;
}

impl frame_system::Trait for Test {
	type BaseCallFilter = ();
	type Origin = Origin;
	type Index = u64;
	type BlockNumber = u64;
	type Call = Call;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = ();
	type BlockHashCount = BlockHashCount;
	type MaximumBlockWeight = MaximumBlockWeight;
	type DbWeight = ();
	type BlockExecutionWeight = ();
	type ExtrinsicBaseWeight = ();
	type MaximumBlockLength = MaximumBlockLength;
	type AvailableBlockRatio = AvailableBlockRatio;
	type Version = ();
	type ModuleToIndex = ();
	type AccountData = pallet_balances::AccountData<u128>;
	type OnNewAccount = ();
	type MigrateAccount = ();
	type OnKilledAccount = ();
	type MaximumExtrinsicWeight = MaximumExtrinsicWeight;
}

parameter_types! {
	pub const ExistentialDeposit: u128 = 1;
}

impl pallet_balances::Trait for Test {
	type Balance = u128;
	type DustRemoval = ();
	type Event = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = frame_system::Module<Test>;
}

parameter_types! {
	pub const Period: u64 = 1;
	pub const Offset: u64 = 0;
	pub const UncleGenerations: u64 = 0;
	pub const DisabledValidatorsThreshold: Perbill = Perbill::from_percent(25);
}

impl pallet_session::Trait for Test {
	type Keys = UintAuthorityId;
	type ShouldEndSession = pallet_session::PeriodicSessions<Period, Offset>;
	type SessionHandler = TestSessionHandler;
	type Event = ();
	type ValidatorId = AccountId;
	type ValidatorIdOf = pallet_staking::StashOf<Test>;
	type DisabledValidatorsThreshold = DisabledValidatorsThreshold;
	type SessionManager = Staking;
	type NextSessionRotation = pallet_session::PeriodicSessions<Period, Offset>;
}

impl pallet_session::historical::Trait for Test {
	type FullIdentification = pallet_staking::Exposure<AccountId, Balance>;
	type FullIdentificationOf = pallet_staking::ExposureOf<Test>;
}

parameter_types! {
	pub const MinimumPeriod: u64 = 5;
}
impl pallet_timestamp::Trait for Test {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = MinimumPeriod;
}

pallet_staking_reward_curve::build! {
	const I_NPOS: PiecewiseLinear<'static> = curve!(
		min_inflation: 0_025_000,
		max_inflation: 0_100_000,
		ideal_stake: 0_800_000,
		falloff: 0_050_000,
		max_piece_count: 40,
		test_precision: 0_005_000,
	);
}

impl<LocalCall> frame_system::offchain::SendTransactionTypes<LocalCall> for Test where
	Call: From<LocalCall>,
{
	type OverarchingCall = Call;
	type Extrinsic = Extrinsic;
}

pub type Extrinsic = TestXt<Call, ()>;

parameter_types! {
	pub const SessionsPerEra: SessionIndex = 3;
	pub const BondingDuration: pallet_staking::EraIndex = 3;
	pub const RewardCurve: &'static PiecewiseLinear<'static> = &I_NPOS;
	pub const MaxNominatorRewardedPerValidator: u32 = 64;
	pub MinSolutionScoreBump: Perbill = Perbill::from_rational_approximation(5u32, 10_000);
}

impl pallet_staking::Trait for Test {
	type Currency = pallet_balances::Module<Self>;
	type UnixTime = pallet_timestamp::Module<Self>;
	type CurrencyToVote = CurrencyToVoteHandler;
	type RewardRemainder = ();
	type Event = ();
	type Slash = ();
	type Reward = ();
	type SessionsPerEra = SessionsPerEra;
	type SlashDeferDuration = ();
	type SlashCancelOrigin = frame_system::EnsureRoot<Self::AccountId>;
	type BondingDuration = BondingDuration;
	type SessionInterface = Self;
	type RewardCurve = RewardCurve;
	type NextNewSession = pallet_session::Module<Test>;
	type ElectionLookahead = ();
	type Call = Call;
	type MaxIterations = ();
	type MaxNominatorRewardedPerValidator = MaxNominatorRewardedPerValidator;
	type UnsignedPriority = ();
	type MinSolutionScoreBump = MinSolutionScoreBump;
}

thread_local! {
	static TEN_TO_FOURTEEN: RefCell<Vec<u64>> = RefCell::new(vec![10,11,12,13,14]);
}
pub struct TenToFourteen;
impl Contains<u64> for TenToFourteen {
	fn sorted_members() -> Vec<u64> {
		TEN_TO_FOURTEEN.with(|v| {
			v.borrow().clone()
		})
	}
	#[cfg(feature = "runtime-benchmarks")]
	fn add(new: &u64) {
		TEN_TO_FOURTEEN.with(|v| {
			let mut members = v.borrow_mut();
			members.push(*new);
			members.sort();
		})
	}
}
impl ContainsLengthBound for TenToFourteen {
	fn max_len() -> usize {
		TEN_TO_FOURTEEN.with(|v| v.borrow().len())
	}
	fn min_len() -> usize { 0 }
}

parameter_types! {
	pub const ProposalBond: Permill = Permill::from_percent(5);
	pub const ProposalBondMinimum: u64 = 1;
	pub const SpendPeriod: u64 = 2;
	pub const Burn: Permill = Permill::from_percent(50);
	pub const TipCountdown: u64 = 1;
	pub const TipFindersFee: Percent = Percent::from_percent(20);
	pub const TipReportDepositBase: u64 = 1;
	pub const TipReportDepositPerByte: u64 = 1;
	pub const TreasuryModuleId: ModuleId = ModuleId(*b"py/trsry");
}
impl pallet_treasury::Trait for Test {
	type ModuleId = TreasuryModuleId;
	type Currency = pallet_balances::Module<Test>;
	type ApproveOrigin = frame_system::EnsureRoot<u64>;
	type RejectOrigin = frame_system::EnsureRoot<u64>;
	type Tippers = TenToFourteen;
	type TipCountdown = TipCountdown;
	type TipFindersFee = TipFindersFee;
	type TipReportDepositBase = TipReportDepositBase;
	type TipReportDepositPerByte = TipReportDepositPerByte;
	type Event = ();
	type ProposalRejection = ();
	type ProposalBond = ProposalBond;
	type ProposalBondMinimum = ProposalBondMinimum;
	type SpendPeriod = SpendPeriod;
	type Burn = Burn;
}


parameter_types! {
	pub const MinimumTreasuryPct: Percent = Percent::from_percent(50);
	pub const MaximumRecipientPct: Percent = Percent::from_percent(50);
}

impl Trait for Test {
	type Event = ();
	type Currency = Balances;
}

pub type Balances = pallet_balances::Module<Test>;
pub type System = frame_system::Module<Test>;
pub type Staking = pallet_staking::Module<Test>;
pub type Treasury = pallet_treasury::Module<Test>;
pub type TreasuryReward = Module<Test>;

pub struct ExtBuilder {
	existential_deposit: u64,
}

impl Default for ExtBuilder {
	fn default() -> Self {
		Self {
			existential_deposit: 0,
		}
	}
}

impl ExtBuilder {
	fn build(self) -> sp_io::TestExternalities {
		let balance_factor = if self.existential_deposit > 0 {
			256
		} else {
			1
		};
		let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
		pallet_balances::GenesisConfig::<Test> {
			balances: vec![
					(1, 10000000000 * balance_factor),
					(2, 10000000000 * balance_factor),
					(3, 10000000000 * balance_factor),
					(4, 10000000000 * balance_factor),
					(10, 10000000000 * balance_factor),
					(11, 10000000000 * balance_factor),
					(20, 10000000000 * balance_factor),
					(21, 10000000000 * balance_factor),
					(30, 10000000000 * balance_factor),
					(31, 10000000000 * balance_factor),
					(40, 10000000000 * balance_factor),
					(41, 10000000000 * balance_factor),
					(100, 10000000000 * balance_factor),
					(101, 10000000000 * balance_factor),
					// This allow us to have a total_payout different from 0.
					(999, 1_000_000_000_000),
			],
		}.assimilate_storage(&mut t).unwrap();
		
		pallet_staking::GenesisConfig::<Test> {
			stakers: vec![],
			validator_count: 2,
			minimum_validator_count: 0,
			invulnerables: vec![],
			slash_reward_fraction: Perbill::from_percent(10),
			.. Default::default()
		}.assimilate_storage(&mut t).unwrap();
		GenesisConfig::<Test> {
			current_payout: 9500000,
			minting_interval: One::one(),
		}.assimilate_storage(&mut t).unwrap();
		t.into()
	}
}

#[test]
fn basic_setup_works() {
	// Verifies initial conditions of mock
	ExtBuilder::default().build().execute_with(|| {
		// Initial Era and session
		let treasury_address = Treasury::account_id();
		System::set_block_number(1);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(1);
		System::set_block_number(2);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(2);
		System::set_block_number(100);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(101);
		System::set_block_number(101);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(102);
		System::set_block_number(102);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(103);
		System::set_block_number(103);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(104);
		assert_eq!(Balances::free_balance(treasury_address) > 0, true);
	});
}

#[test]
fn setting_treasury_block_reward () {
	// Verifies initial conditions of mock
	ExtBuilder::default().build().execute_with(|| {
		// Initial Era and session
		let treasury_address = Treasury::account_id();
		System::set_block_number(1);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(1);
		assert_eq!(Balances::free_balance(treasury_address)==9500000, true);
		System::set_block_number(2);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(2);
		assert_eq!(Balances::free_balance(treasury_address)==19000000, true);

		<TreasuryReward>::set_current_payout(frame_system::RawOrigin::Root.into(),95).unwrap();
		<TreasuryReward>::set_minting_interval(frame_system::RawOrigin::Root.into(),2).unwrap();
		
		System::set_block_number(3);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(3);
		assert_eq!(Balances::free_balance(treasury_address)==19000000, true);
		System::set_block_number(4);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(4);
		assert_eq!(Balances::free_balance(treasury_address)==19000095, true);

		<TreasuryReward>::set_current_payout(frame_system::RawOrigin::Root.into(),0).unwrap();

		System::set_block_number(5);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(5);
		assert_eq!(Balances::free_balance(treasury_address)==19000095, true);
		System::set_block_number(6);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(6);
		assert_eq!(Balances::free_balance(treasury_address)==19000095, true);

		<TreasuryReward>::set_current_payout(frame_system::RawOrigin::Root.into(),105).unwrap();

		System::set_block_number(7);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(7);
		assert_eq!(Balances::free_balance(treasury_address)==19000095, true);
		System::set_block_number(8);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(8);
		assert_eq!(Balances::free_balance(treasury_address)==19000200, true);

		<TreasuryReward>::set_minting_interval(frame_system::RawOrigin::Root.into(),1).unwrap();
		<TreasuryReward>::set_current_payout(frame_system::RawOrigin::Root.into(),10).unwrap();

		System::set_block_number(9);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(9);
		assert_eq!(Balances::free_balance(treasury_address)==19000210, true);
		System::set_block_number(10);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(10);
		assert_eq!(Balances::free_balance(treasury_address)==19000220, true);
	});
}
