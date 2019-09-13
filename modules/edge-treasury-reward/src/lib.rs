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

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate serde;

// Needed for deriving `Serialize` and `Deserialize` for various types.
// We only implement the serde traits for std builds - they're unneeded
// in the wasm runtime.
#[cfg(feature = "std")]
extern crate serde_derive;

use srml_system as system;
pub mod treasury_reward;
pub use treasury_reward::{Module, Trait, RawEvent, Event};

#[cfg(test)]
mod tests {
	#[cfg(feature = "std")]
	use std::{collections::HashSet, cell::RefCell};
	use sr_io::with_externalities;
	
	
	
	use super::*;
	// The testing primitives are very useful for avoiding having to work with signatures
	// or public keys. `u64` is used as the `AccountId` and no `Signature`s are requried.
	use sr_primitives::{
		Perbill, Permill,
		traits::{BlakeTwo256, IdentityLookup, OpaqueKeys, One, OnFinalize},
		testing::{Header, UintAuthorityId}
	};
	
	/// The AccountId alias in this test module.
	pub type AccountId = u64;
	pub type Balance = u64;

	/// Simple structure that exposes how u64 currency can be represented as... u64.
	pub struct CurrencyToVoteHandler;
	impl sr_primitives::traits::Convert<u64, u64> for CurrencyToVoteHandler {
		fn convert(x: u64) -> u64 { x }
	}
	impl sr_primitives::traits::Convert<u128, u64> for CurrencyToVoteHandler {
		fn convert(x: u128) -> u64 {
			x as u64
		}
	}

	thread_local! {
		static SESSION: RefCell<(Vec<AccountId>, HashSet<AccountId>)> = RefCell::new(Default::default());
		static EXISTENTIAL_DEPOSIT: RefCell<u64> = RefCell::new(0);
	}

	pub struct TestSessionHandler;
	impl srml_session::SessionHandler<AccountId> for TestSessionHandler {
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

	srml_support::impl_outer_origin! {
		pub enum Origin for Test {}
	}
	
	#[derive(Clone, PartialEq, Eq, Debug)]
	pub struct Test;

	srml_support::parameter_types! {
		pub const BlockHashCount: u64 = 250;
		pub const MaximumBlockWeight: u32 = 1024;
		pub const MaximumBlockLength: u32 = 2 * 1024;
		pub const AvailableBlockRatio: Perbill = Perbill::one();
	}

	impl system::Trait for Test {
		type Origin = Origin;
		type Call = ();
		type Index = u64;
		type BlockNumber = u64;
		type Hash = substrate_primitives::H256;
		type Hashing = BlakeTwo256;
		type AccountId = u64;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Header = Header;
		type Event = ();
		type WeightMultiplierUpdate = ();
		type BlockHashCount = BlockHashCount;
		type MaximumBlockWeight = MaximumBlockWeight;
		type MaximumBlockLength = MaximumBlockLength;
		type AvailableBlockRatio = AvailableBlockRatio;
		type Version = ();
	}

	srml_support::parameter_types! {
		pub const ExistentialDeposit: u64 = 0;
		pub const TransferFee: u64 = 0;
		pub const CreationFee: u64 = 0;
		pub const TransactionBaseFee: u64 = 0;
		pub const TransactionByteFee: u64 = 0;
	}

	impl srml_balances::Trait for Test {
		type Balance = u64;
		type OnNewAccount = ();
		type OnFreeBalanceZero = ();
		type Event = ();
		type TransactionPayment = ();
		type TransferPayment = ();
		type DustRemoval = ();
		type ExistentialDeposit = ExistentialDeposit;
		type TransferFee = TransferFee;
		type CreationFee = CreationFee;
		type TransactionBaseFee = TransactionBaseFee;
		type TransactionByteFee = TransactionByteFee;
		type WeightToFee = ();
	}

	srml_support::parameter_types! {
		pub const Period: u64 = 1;
		pub const Offset: u64 = 0;
		pub const UncleGenerations: u64 = 0;
	}

	impl srml_session::Trait for Test {
		type OnSessionEnding = srml_session::historical::NoteHistoricalRoot<Test, Staking>;
		type Keys = UintAuthorityId;
		type ShouldEndSession = srml_session::PeriodicSessions<Period, Offset>;
		type SessionHandler = TestSessionHandler;
		type Event = ();
		type ValidatorId = AccountId;
		type ValidatorIdOf = srml_staking::StashOf<Test>;
		type SelectInitialValidators = Staking;
	}

	impl srml_session::historical::Trait for Test {
		type FullIdentification = srml_staking::Exposure<AccountId, Balance>;
		type FullIdentificationOf = srml_staking::ExposureOf<Test>;
	}

	srml_support::parameter_types! {
		pub const MinimumPeriod: u64 = 5;
	}
	impl srml_timestamp::Trait for Test {
		type Moment = u64;
		type OnTimestampSet = ();
		type MinimumPeriod = MinimumPeriod;
	}
	srml_support::parameter_types! {
		pub const SessionsPerEra: sr_staking_primitives::SessionIndex = 3;
		pub const BondingDuration: srml_staking::EraIndex = 3;
	}
	impl srml_staking::Trait for Test {
		type Currency = Balances;
		type Time = Timestamp;
		type CurrencyToVote = CurrencyToVoteHandler;
		type OnRewardMinted = ();
		type Event = ();
		type Slash = ();
		type Reward = ();
		type SessionsPerEra = SessionsPerEra;
		type BondingDuration = BondingDuration;
		type SessionInterface = Self;
	}

	srml_support::parameter_types! {
		pub const ProposalBond: Permill = Permill::from_percent(5);
		pub const ProposalBondMinimum: u64 = 1;
		pub const SpendPeriod: u64 = 2;
		pub const Burn: Permill = Permill::from_percent(50);
	}

	impl srml_treasury::Trait for Test {
		type Currency = Balances;
		type ApproveOrigin = srml_system::EnsureRoot<u64>;
		type RejectOrigin = srml_system::EnsureRoot<u64>;
		type Event = ();
		type MintedForSpending = ();
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

	pub type Balances = srml_balances::Module<Test>;
	pub type System = srml_system::Module<Test>;
	pub type Staking = srml_staking::Module<Test>;
	pub type Timestamp = srml_timestamp::Module<Test>;
	pub type Treasury = srml_treasury::Module<Test>;
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
		fn build(self) -> sr_io::TestExternalities<substrate_primitives::Blake2Hasher> {
			let balance_factor = if self.existential_deposit > 0 {
				256
			} else {
				1
			};
			let mut t = srml_system::GenesisConfig::default().build_storage::<Test>().unwrap();
			t.0.extend(
				srml_balances::GenesisConfig::<Test> {
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
				srml_staking::GenesisConfig::<Test> {
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
				treasury_reward::GenesisConfig::<Test> {
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
		with_externalities(&mut ExtBuilder::default()
			.build(),
		|| {
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
}
