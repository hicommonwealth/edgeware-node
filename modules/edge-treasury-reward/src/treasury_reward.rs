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
// along with Edgeware.  If not, see <http://www.gnu.org/licenses/>.

use sr_std as rstd;
use srml_support as runtime_support;
use sr_primitives as runtime_primitives;
use srml_system as system;
use srml_treasury as treasury;
use srml_staking as staking;
use srml_balances as balances;

use runtime_support::traits::{Time};
use staking::inflation::compute_total_payout;

use rstd::prelude::*;
use srml_support::{
	StorageValue, decl_module, decl_event,
	decl_storage, traits::{
		Currency
	}
};
use runtime_primitives::traits::{Zero, SaturatedConversion};
pub type BalanceOf<T> = <<T as staking::Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;
pub type MomentOf<T>= <<T as staking::Trait>::Time as Time>::Moment;

pub trait Trait: staking::Trait + treasury::Trait + balances::Trait {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
	/// The account balance
	type Currency: Currency<Self::AccountId>;
	/// Time used for computing era duration.
	type Time: Time;
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;
		/// Mint money for the treasury!
		fn on_finalize(_n: T::BlockNumber) {
			// Set the start of the first era.
			if !<CurrentRewardCycle<T>>::exists() {
				<CurrentRewardCycle<T>>::put(<system::Module<T>>::block_number());
			}

			let previous_era_start = <CurrentRewardCycle<T>>::get();
			let minting_interval = <MintingInterval<T>>::get();
			let validators = <staking::Module<T>>::current_elected();
			let slot_stake = <staking::Module<T>>::slot_stake();
			let validator_len: BalanceOf<T> = (validators.len() as u32).into();
			let total_rewarded_stake = validator_len * slot_stake;
			let now = <system::Module<T>>::block_number();
			let elapsed_time = now.clone() - previous_era_start;

			if now % minting_interval == Zero::zero() && !elapsed_time.is_zero() {
				<CurrentRewardCycle<T>>::put(<system::Module<T>>::block_number());
				let total_payout = compute_total_payout(
					total_rewarded_stake.clone(),
					<T as staking::Trait>::Currency::total_issuance(),
					<BalanceOf<T>>::from(elapsed_time.saturated_into::<u32>()));
				<T as staking::Trait>::Currency::deposit_creating(&<treasury::Module<T>>::account_id(), total_payout);
				<Pot<T>>::put(<balances::Module<T>>::free_balance(&<treasury::Module<T>>::account_id()));
				Self::deposit_event(RawEvent::TreasuryMinting(total_payout, <system::Module<T>>::block_number()));
			}
		}
	}
}

decl_event!(
	pub enum Event<T> where <T as system::Trait>::BlockNumber,
							Balance = <<T as staking::Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance 
							{
		TreasuryMinting(Balance, BlockNumber),
	}
);

decl_storage! {
	trait Store for Module<T: Trait> as TreasuryReward {
		/// Interval in number of blocks to reward treasury
		pub MintingInterval get(minting_interval) config(): T::BlockNumber;
		/// Time of current reward cycle starting point
		pub CurrentRewardCycle get(current_reward_cycle): T::BlockNumber;
		/// Current pot
		pub Pot get(pot): T::Balance;
	}
}
