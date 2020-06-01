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

#![recursion_limit="128"]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod tests;

use frame_support::traits::Currency;
use sp_std::prelude::*;
use sp_runtime::traits::{Zero};

use frame_support::{decl_event, decl_module, decl_storage};
use frame_system::{self as system, ensure_root};
pub type BalanceOf<T> = <<T as pallet_staking::Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;

pub trait Trait: pallet_staking::Trait + pallet_treasury::Trait + pallet_balances::Trait {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
	/// The account balance
	type Currency: Currency<Self::AccountId>;
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;
		/// Mint money for the treasury!
		fn on_finalize(_n: T::BlockNumber) {
			if <frame_system::Module<T>>::block_number() % Self::minting_interval() == Zero::zero() {
				let reward = Self::current_payout();
				<T as pallet_staking::Trait>::Currency::deposit_creating(&<pallet_treasury::Module<T>>::account_id(), reward);
				Self::deposit_event(RawEvent::TreasuryMinting(
					<pallet_balances::Module<T>>::free_balance(<pallet_treasury::Module<T>>::account_id()),
					<frame_system::Module<T>>::block_number(),
					<pallet_treasury::Module<T>>::account_id())
				);
			}
		}

		/// Sets the fixed treasury payout per minting interval.
		#[weight = 5_000_000]
		fn set_current_payout(origin, payout: BalanceOf<T>) {
			ensure_root(origin)?;
			<CurrentPayout<T>>::put(payout);
		}

		/// Sets the treasury minting interval.
		#[weight = 5_000_000]
		fn set_minting_interval(origin, interval: T::BlockNumber) {
			ensure_root(origin)?;
			<MintingInterval<T>>::put(interval);
		}
	}
}

decl_event!(
	pub enum Event<T> where <T as frame_system::Trait>::BlockNumber,
							<T as frame_system::Trait>::AccountId,
							Balance = <T as pallet_balances::Trait>::Balance {
		TreasuryMinting(Balance, BlockNumber, AccountId),
	}
);

decl_storage! {
	trait Store for Module<T: Trait> as TreasuryReward {
		/// Interval in number of blocks to reward treasury
		pub MintingInterval get(fn minting_interval) config(): T::BlockNumber;
		/// Current payout of module
		pub CurrentPayout get(fn current_payout) config(): BalanceOf<T>;
	}
}
