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

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
pub mod mock;

#[cfg(test)]
pub mod tests;

use frame_support::traits::Currency;
pub type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_runtime::traits::Zero;

	/// The pallet's configuration trait.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_balances::Config + pallet_treasury::Config {
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The account balance
		type Currency: Currency<Self::AccountId>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(crate) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Treasury minting event
		TreasuryMinting(T::Balance, T::BlockNumber, T::AccountId),
	}

	/// The next tree identifier up for grabs
	#[pallet::storage]
	#[pallet::getter(fn minting_interval)]
	pub type MintingInterval<T: Config> = StorageValue<_, T::BlockNumber, ValueQuery>;

	/// The next tree identifier up for grabs
	#[pallet::storage]
	#[pallet::getter(fn current_payout)]
	pub type CurrentPayout<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

	#[pallet::pallet]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		// tokens to create + min_balance for that token
		pub minting_interval: T::BlockNumber,
		// endowed accounts for a token + their balances
		pub current_payout: BalanceOf<T>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			GenesisConfig {
				minting_interval: Zero::zero(),
				current_payout: Zero::zero(),
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			MintingInterval::<T>::put(self.minting_interval);
			CurrentPayout::<T>::put(self.current_payout);
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {
		fn on_finalize(_n: T::BlockNumber) {
			if <frame_system::Pallet<T>>::block_number() % Self::minting_interval() == Zero::zero() {
				let reward = Self::current_payout();
				if reward.is_zero() {
					return;
				}
				<T as Config>::Currency::deposit_creating(&<pallet_treasury::Pallet<T>>::account_id(), reward);
				Self::deposit_event(Event::TreasuryMinting(
					<pallet_balances::Pallet<T>>::free_balance(<pallet_treasury::Pallet<T>>::account_id()),
					<frame_system::Pallet<T>>::block_number(),
					<pallet_treasury::Pallet<T>>::account_id(),
				));
			}
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Sets the fixed treasury payout per minting interval.
		#[pallet::weight(5_000_000)]
		pub fn set_current_payout(origin: OriginFor<T>, payout: BalanceOf<T>) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;
			<CurrentPayout<T>>::put(payout);
			Ok(().into())
		}

		/// Sets the treasury minting interval.
		#[pallet::weight(5_000_000)]
		pub fn set_minting_interval(origin: OriginFor<T>, interval: T::BlockNumber) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;
			<MintingInterval<T>>::put(interval);
			Ok(().into())
		}
	}
}
