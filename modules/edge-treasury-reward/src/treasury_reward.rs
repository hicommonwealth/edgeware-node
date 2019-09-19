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

use sr_primitives as runtime_primitives;
use srml_system as system;
use srml_treasury as treasury;
use srml_staking as staking;
use srml_balances as balances;

use runtime_primitives::{traits::{Zero}};

use rstd::prelude::*;
use srml_support::{
	decl_module, decl_event,
	decl_storage, traits::{
		Currency
	}
};

pub type BalanceOf<T> = <<T as staking::Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;

pub trait Trait: staking::Trait + treasury::Trait + balances::Trait {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
	/// The account balance
	type Currency: Currency<Self::AccountId>;
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;
		/// Mint money for the treasury!
		fn on_finalize(_n: T::BlockNumber) {
			if <system::Module<T>>::block_number() % Self::minting_interval() == Zero::zero() {
				let reward = Self::current_payout();
				<T as staking::Trait>::Currency::deposit_creating(&<treasury::Module<T>>::account_id(), reward);
				<Pot<T>>::put(<balances::Module<T>>::free_balance(&<treasury::Module<T>>::account_id()));
				Self::deposit_event(RawEvent::TreasuryMinting(
					Self::pot(),
					reward,
					<system::Module<T>>::block_number())
				);
			}
		}
	}
}

decl_event!(
	pub enum Event<T> where <T as system::Trait>::BlockNumber,
							Balance2 = <<T as staking::Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance,
							Balance = <T as balances::Trait>::Balance
							{
		TreasuryMinting(Balance, Balance2, BlockNumber),
	}
);

decl_storage! {
	trait Store for Module<T: Trait> as TreasuryReward {
		/// Interval in number of blocks to reward treasury
		pub MintingInterval get(minting_interval) config(): T::BlockNumber;
		/// Current payout of module
		pub CurrentPayout get(current_payout) config(): BalanceOf<T>;
		/// Current pot
		pub Pot get(pot): T::Balance;
	}
}
