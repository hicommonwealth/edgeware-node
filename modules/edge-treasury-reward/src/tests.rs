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

use super::*;
use sr_staking_primitives::SessionIndex;
use sr_primitives::traits::OpaqueKeys;
use sr_primitives::curve::PiecewiseLinear;
use sr_primitives::testing::UintAuthorityId;

#[cfg(feature = "std")]
use std::{collections::HashSet, cell::RefCell};

use substrate_primitives::{H256, crypto::key_types};


use support::{parameter_types, impl_outer_origin};

use sr_primitives::{
	Perbill, Permill, KeyTypeId,
	testing::{Header},
	traits::{OnFinalize, IdentityLookup, One},
};

use crate::GenesisConfig;

/// The AccountId alias in this test module.
pub type AccountId = u64;
pub type Balance = u128;

/// Simple structure that exposes how u64 currency can be represented as... u64.
pub struct CurrencyToVoteHandler;
impl sr_primitives::traits::Convert<u64, u64> for CurrencyToVoteHandler {
	fn convert(x: u64) -> u64 { x }
}
impl sr_primitives::traits::Convert<u128, u64> for CurrencyToVoteHandler {
	fn convert(x: u128) -> u64 { x as u64 }
}
impl sr_primitives::traits::Convert<u128, u128> for CurrencyToVoteHandler {
	fn convert(x: u128) -> u128 { x }
}
impl sr_primitives::traits::Convert<u64, u128> for CurrencyToVoteHandler {
	fn convert(x: u64) -> u128 { x as u128 }
}

thread_local! {
	static SESSION: RefCell<(Vec<AccountId>, HashSet<AccountId>)> = RefCell::new(Default::default());
	static EXISTENTIAL_DEPOSIT: RefCell<u64> = RefCell::new(0);
}

pub struct TestSessionHandler;
impl session::SessionHandler<AccountId> for TestSessionHandler {
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
	pub enum Origin for Test {}
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Test;

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: u32 = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::one();
}

impl system::Trait for Test {
	type Origin = Origin;
	type Index = u64;
	type BlockNumber = u64;
	type Call = ();
	type Hash = H256;
	type Hashing = ::sr_primitives::traits::BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = ();
	type BlockHashCount = BlockHashCount;
	type MaximumBlockWeight = MaximumBlockWeight;
	type MaximumBlockLength = MaximumBlockLength;
	type AvailableBlockRatio = AvailableBlockRatio;
	type Version = ();
}

parameter_types! {
	pub const ExistentialDeposit: u128 = 0;
	pub const TransferFee: u128 = 0;
	pub const CreationFee: u128 = 0;
}

impl balances::Trait for Test {
	/// The type for recording an account's balance.
	type Balance = u128;
	/// What to do if an account's free balance gets zeroed.
	type OnFreeBalanceZero = ();
	/// What to do if a new account is created.
	type OnNewAccount = ();
	/// The ubiquitous event type.
	type Event = ();
	type DustRemoval = ();
	type TransferPayment = ();
	type ExistentialDeposit = ExistentialDeposit;
	type TransferFee = TransferFee;
	type CreationFee = CreationFee;
}

parameter_types! {
	pub const Period: u64 = 1;
	pub const Offset: u64 = 0;
	pub const UncleGenerations: u64 = 0;
	pub const DisabledValidatorsThreshold: Perbill = Perbill::from_percent(25);
}

impl session::Trait for Test {
	type OnSessionEnding = session::historical::NoteHistoricalRoot<Test, Staking>;
	type Keys = UintAuthorityId;
	type ShouldEndSession = session::PeriodicSessions<Period, Offset>;
	type SessionHandler = TestSessionHandler;
	type Event = ();
	type ValidatorId = AccountId;
	type ValidatorIdOf = staking::StashOf<Test>;
	type SelectInitialValidators = Staking;
	type DisabledValidatorsThreshold = DisabledValidatorsThreshold;
}

impl session::historical::Trait for Test {
	type FullIdentification = staking::Exposure<AccountId, Balance>;
	type FullIdentificationOf = staking::ExposureOf<Test>;
}

parameter_types! {
	pub const MinimumPeriod: u64 = 5;
}
impl timestamp::Trait for Test {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = MinimumPeriod;
}

staking_reward_curve::build! {
	const I_NPOS: PiecewiseLinear<'static> = curve!(
		min_inflation: 0_025_000,
		max_inflation: 0_100_000,
		ideal_stake: 0_800_000,
		falloff: 0_050_000,
		max_piece_count: 40,
		test_precision: 0_005_000,
	);
}

parameter_types! {
	pub const SessionsPerEra: SessionIndex = 3;
	pub const BondingDuration: staking::EraIndex = 3;
	pub const RewardCurve: &'static PiecewiseLinear<'static> = &I_NPOS;
}

impl staking::Trait for Test {
	type Currency = Balances;
	type Time = Timestamp;
	type CurrencyToVote = CurrencyToVoteHandler;
	type RewardRemainder = ();
	type Event = ();
	type Slash = ();
	type Reward = ();
	type SessionsPerEra = SessionsPerEra;
	type BondingDuration = BondingDuration;
	type SessionInterface = Self;
	type RewardCurve = RewardCurve;
}

parameter_types! {
	pub const ProposalBond: Permill = Permill::from_percent(5);
	pub const ProposalBondMinimum: u64 = 1;
	pub const SpendPeriod: u64 = 2;
	pub const Burn: Permill = Permill::from_percent(50);
}

impl treasury::Trait for Test {
	type Currency = Balances;
	type ApproveOrigin = system::EnsureRoot<u64>;
	type RejectOrigin = system::EnsureRoot<u64>;
	type Event = ();
	type ProposalRejection = ();
	type ProposalBond = ProposalBond;
	type ProposalBondMinimum = ProposalBondMinimum;
	type SpendPeriod = SpendPeriod;
	type Burn = Burn;
}

impl Trait for Test {
	type Event = ();
	type Currency = Balances;
}

pub type Balances = balances::Module<Test>;
pub type System = system::Module<Test>;
pub type Staking = staking::Module<Test>;
pub type Timestamp = timestamp::Module<Test>;
pub type Treasury = treasury::Module<Test>;
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
	fn build(self) -> sr_io::TestExternalities {
		let balance_factor = if self.existential_deposit > 0 {
			256
		} else {
			1
		};
		let mut t = system::GenesisConfig::default().build_storage::<Test>().unwrap();
		t.0.extend(
			balances::GenesisConfig::<Test> {
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
				vesting: vec![],
			}.build_storage().unwrap().0,
		);

		t.0.extend(
			staking::GenesisConfig::<Test> {
				current_era: 0,
				stakers: vec![],
				validator_count: 2,
				minimum_validator_count: 0,
				invulnerables: vec![],
				slash_reward_fraction: Perbill::from_percent(10),
				.. Default::default()
			}.build_storage().unwrap().0,
		);
		t.0.extend(
			GenesisConfig::<Test> {
				current_payout: 9500000,
				minting_interval: One::one(),
			}.build_storage().unwrap().0,
		);
		t.into()
	}
}

#[test]
fn basic_setup_works() {
	// Verifies initial conditions of mock
	ExtBuilder::default().build().execute_with(|| {
		// Initial Era and session
		assert_eq!(Staking::current_era(), 0);
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
