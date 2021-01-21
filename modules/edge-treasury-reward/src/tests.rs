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

use sp_runtime::traits::BlakeTwo256;
use super::*;
use frame_support::{
	impl_outer_origin, parameter_types,
	traits::{OnFinalize},
};

use sp_core::{H256};

use sp_runtime::{
	Permill, ModuleId,
	testing::{Header}, Percent,
	traits::{IdentityLookup, One},
};

use crate::GenesisConfig;

impl_outer_origin! {
	pub enum Origin for Test where system = frame_system {}
}

// Workaround for https://github.com/rust-lang/rust/issues/26925 . Remove when sorted.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Test;

parameter_types! {
	pub const BlockHashCount: u64 = 250;
}
impl frame_system::Config for Test {
	type BaseCallFilter = ();
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Index = u64;
	type Call = ();
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = ();
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = ();
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
}

impl pallet_balances::Config for Test {
	type MaxLocks = ();
	type Balance = u64;
	type Event = ();
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
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
impl pallet_treasury::Config for Test {
	type ModuleId = TreasuryModuleId;
	type Currency = Balances;
	type ApproveOrigin = frame_system::EnsureRoot<u64>;
	type RejectOrigin = frame_system::EnsureRoot<u64>;
	type Event = ();
	type OnSlash = ();
	type ProposalBond = ProposalBond;
	type ProposalBondMinimum = ProposalBondMinimum;
	type SpendPeriod = SpendPeriod;
	type Burn = Burn;
	type BurnDestination = ();
	type SpendFunds = ();
	type WeightInfo = ();
}

impl Config for Test {
	type Event = ();
	type Currency = Balances;
}

pub type Balances = pallet_balances::Module<Test>;
pub type System = frame_system::Module<Test>;
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
