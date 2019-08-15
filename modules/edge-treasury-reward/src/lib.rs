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
#[macro_use]
use srml_support;

use substrate_primitives as primitives;

use srml_support as runtime_support;
use sr_primitives as runtime_primitives;

use srml_balances as balances;
use srml_system as system;
use srml_staking as staking;
use srml_session as session;
use srml_timestamp as timestamp;
use srml_treasury as treasury;
use srml_authorship as authorship;

use runtime_support::{
	impl_outer_origin, parameter_types,
	traits::{FindAuthor},
};
use staking::{EraIndex, StakerStatus, ValidatorPrefs};
pub mod treasury_reward;
pub mod inflation;
pub use treasury_reward::{Module, Trait, RawEvent, Event};

use srml_staking::StashOf;
use srml_staking::Exposure;
use srml_staking::ExposureOf;

use primitives::{H256, Blake2Hasher};

#[cfg(test)]
mod tests {
	#[cfg(feature = "std")]
	use std::{collections::HashSet, cell::RefCell};
	use sr_io::with_externalities;
	use srml_staking::Validators;
	use srml_staking::StakingLedger;
	use srml_staking::IndividualExposure;
use super::*;
	// The testing primitives are very useful for avoiding having to work with signatures
	// or public keys. `u64` is used as the `AccountId` and no `Signature`s are requried.
	use runtime_primitives::{
		Perbill, Permill,
		traits::{BlakeTwo256, IdentityLookup, Convert, OpaqueKeys, One, OnFinalize},
		testing::{Header, UintAuthorityId}
	};
	
	/// The AccountId alias in this test module.
	pub type AccountId = u64;
	pub type BlockNumber = u64;
	pub type Balance = u64;

	/// Simple structure that exposes how u64 currency can be represented as... u64.
	pub struct CurrencyToVoteHandler;
	impl Convert<u64, u64> for CurrencyToVoteHandler {
		fn convert(x: u64) -> u64 { x }
	}
	impl Convert<u128, u64> for CurrencyToVoteHandler {
		fn convert(x: u128) -> u64 {
			x as u64
		}
	}

	thread_local! {
		static SESSION: RefCell<(Vec<AccountId>, HashSet<AccountId>)> = RefCell::new(Default::default());
		static EXISTENTIAL_DEPOSIT: RefCell<u64> = RefCell::new(0);
	}

	pub struct TestSessionHandler;
	impl session::SessionHandler<AccountId> for TestSessionHandler {
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

	pub fn is_disabled(validator: AccountId) -> bool {
		let stash = Staking::ledger(&validator).unwrap().stash;
		SESSION.with(|d| d.borrow().1.contains(&stash))
	}

	/// Author of block is always 11
	pub struct Author11;
	impl FindAuthor<u64> for Author11 {
		fn find_author<'a, I>(_digests: I) -> Option<u64>
			where I: 'a + IntoIterator<Item=(srml_support::ConsensusEngineId, &'a [u8])>
		{
			Some(11)
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
		type Hash = H256;
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
	}

	parameter_types! {
		pub const ExistentialDeposit: u64 = 0;
		pub const TransferFee: u64 = 0;
		pub const CreationFee: u64 = 0;
		pub const TransactionBaseFee: u64 = 0;
		pub const TransactionByteFee: u64 = 0;
	}

	impl balances::Trait for Test {
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

	parameter_types! {
		pub const Period: BlockNumber = 1;
		pub const Offset: BlockNumber = 0;
		pub const UncleGenerations: u64 = 0;
	}

	impl session::Trait for Test {
		type OnSessionEnding = session::historical::NoteHistoricalRoot<Test, Staking>;
		type Keys = UintAuthorityId;
		type ShouldEndSession = session::PeriodicSessions<Period, Offset>;
		type SessionHandler = TestSessionHandler;
		type Event = ();
		type ValidatorId = AccountId;
		type ValidatorIdOf = crate::StashOf<Test>;
		type SelectInitialValidators = Staking;
	}

	impl session::historical::Trait for Test {
		type FullIdentification = crate::Exposure<AccountId, Balance>;
		type FullIdentificationOf = crate::ExposureOf<Test>;
	}
	impl authorship::Trait for Test {
		type FindAuthor = Author11;
		type UncleGenerations = UncleGenerations;
		type FilterUncle = ();
		type EventHandler = staking::Module<Test>;
	}

	parameter_types! {
		pub const MinimumPeriod: u64 = 5;
	}
	impl timestamp::Trait for Test {
		type Moment = u64;
		type OnTimestampSet = ();
		type MinimumPeriod = MinimumPeriod;
	}
	parameter_types! {
		pub const SessionsPerEra: session::SessionIndex = 3;
		pub const BondingDuration: EraIndex = 3;
	}
	impl staking::Trait for Test {
		type Currency = balances::Module<Self>;
		type Time = timestamp::Module<Self>;
		type CurrencyToVote = CurrencyToVoteHandler;
		type OnRewardMinted = ();
		type Event = ();
		type Slash = ();
		type Reward = ();
		type SessionsPerEra = SessionsPerEra;
		type BondingDuration = BondingDuration;
		type SessionInterface = Self;
	}

	parameter_types! {
		pub const ProposalBond: Permill = Permill::from_percent(5);
		pub const ProposalBondMinimum: u64 = 1;
		pub const SpendPeriod: u64 = 2;
		pub const Burn: Permill = Permill::from_percent(50);
	}

	impl treasury::Trait for Test {
		type Currency = balances::Module<Test>;
		type ApproveOrigin = system::EnsureRoot<u64>;
		type RejectOrigin = system::EnsureRoot<u64>;
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
		type Currency = balances::Module<Self>;
		type Time = timestamp::Module<Self>;
	}

	pub type Balances = balances::Module<Test>;
	pub type System = system::Module<Test>;
	pub type Staking = staking::Module<Test>;
	pub type Timestamp = timestamp::Module<Test>;
	pub type Treasury = treasury::Module<Test>;
	pub type TreasuryReward = Module<Test>;

	pub struct ExtBuilder {
		existential_deposit: u64,
		validator_pool: bool,
		nominate: bool,
		validator_count: u32,
		minimum_validator_count: u32,
		fair: bool,
		num_validators: Option<u32>,
	}

	impl Default for ExtBuilder {
		fn default() -> Self {
			Self {
				existential_deposit: 0,
				validator_pool: false,
				nominate: true,
				validator_count: 2,
				minimum_validator_count: 0,
				fair: true,
				num_validators: None,
			}
		}
	}

	impl ExtBuilder {
		pub fn existential_deposit(mut self, existential_deposit: u64) -> Self {
			self.existential_deposit = existential_deposit;
			self
		}
		pub fn validator_pool(mut self, validator_pool: bool) -> Self {
			self.validator_pool = validator_pool;
			self
		}
		pub fn nominate(mut self, nominate: bool) -> Self {
			self.nominate = nominate;
			self
		}
		pub fn validator_count(mut self, count: u32) -> Self {
			self.validator_count = count;
			self
		}
		pub fn minimum_validator_count(mut self, count: u32) -> Self {
			self.minimum_validator_count = count;
			self
		}
		pub fn fair(mut self, is_fair: bool) -> Self {
			self.fair = is_fair;
			self
		}
		pub fn num_validators(mut self, num_validators: u32) -> Self {
			self.num_validators = Some(num_validators);
			self
		}
		pub fn set_associated_consts(&self) {
			EXISTENTIAL_DEPOSIT.with(|v| *v.borrow_mut() = self.existential_deposit);
		}

		fn build(self) -> sr_io::TestExternalities<Blake2Hasher> {
			self.set_associated_consts();
			let balance_factor = if self.existential_deposit > 0 {
				256
			} else {
				1
			};
			let mut t = system::GenesisConfig::default().build_storage::<Test>().unwrap().0;
			t.extend(
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

			let stake_21 = if self.fair { 1000 } else { 2000 };
			let stake_31 = if self.validator_pool { balance_factor * 1000 } else { 1 };
			let status_41 = if self.validator_pool {
				StakerStatus::<AccountId>::Validator
			} else {
				StakerStatus::<AccountId>::Idle
			};
			let nominated = if self.nominate { vec![11, 21] } else { vec![] };
			t.extend(
				staking::GenesisConfig::<Test> {
					current_era: 0,
					stakers: vec![
						// (11, 10, balance_factor * 1000, StakerStatus::<AccountId>::Validator),
						// (21, 20, stake_21, StakerStatus::<AccountId>::Validator),
						// (31, 30, stake_31, StakerStatus::<AccountId>::Validator),
						// (41, 40, balance_factor * 1000, status_41),
						// (101, 100, balance_factor * 500, StakerStatus::<AccountId>::Nominator(nominated))
					],
					validator_count: self.validator_count,
					minimum_validator_count: self.minimum_validator_count,
					offline_slash: Perbill::from_percent(5),
					offline_slash_grace: 0,
					invulnerables: vec![],
				}.build_storage().unwrap().0,
			);
			t.extend(
				treasury_reward::GenesisConfig::<Test> {
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
			println!("{:?}", Balances::free_balance(treasury_address));
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
			println!("{:?}", Balances::free_balance(treasury_address));
		});
	}

}