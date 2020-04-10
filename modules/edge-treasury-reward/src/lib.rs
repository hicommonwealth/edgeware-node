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

use codec::*;
use frame_system::ensure_root;
use frame_support::traits::{Get, Currency};
use sp_std::prelude::*;
use sp_runtime::{Percent, PerThing, RuntimeDebug};
use sp_runtime::traits::{Zero, Saturating};

use frame_support::{decl_event, decl_module, decl_storage};
use frame_system::{self as system};

pub type BalanceOf<T> = <<T as pallet_staking::Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;

pub trait Trait: pallet_staking::Trait + pallet_treasury::Trait + pallet_balances::Trait {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
	/// The account balance
	type Currency: Currency<Self::AccountId>;
	/// Minimum fraction of a treasury reward that goes to the Treasury account itself
	type MinimumTreasuryPct: Get<Percent>;
	/// Maximum fraction of a treasury reward that goes to an individual non-Treasury recipient itself
	type MaximumRecipientPct: Get<Percent>;
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, Ord, PartialOrd, RuntimeDebug)]
pub struct RecipientAllocation {
	pub proposed: Percent,
	pub current: Percent,
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		/// Adds a new recipient to the recipients list and assigns them
		/// the submitted percentage of the leftover treasury reward.
		/// If there is no leftover allocation, the other recipients'
		/// reward percentages will be diluted. This should be taken into
		/// account when submitting and voting on add recipient proposals.
		fn add(origin, recipient: T::AccountId, pct: Percent) {
			ensure_root(origin)?;
			assert!(!Self::recipients().contains(&recipient), "Duplicate recipients not allowed");
			let leftover_allocation = Self::get_available_recipient_alloc();
			// Dilute current allocations by overflowed percentage
			if pct.deconstruct() > leftover_allocation.deconstruct() {
				let diff = pct.saturating_sub(leftover_allocation);
				Self::dilute_percentages(diff);
			}
			// Add new recipient
			Self::add_recipient(recipient, pct);
		}

		fn remove(origin, recipient: T::AccountId) {
			ensure_root(origin)?;
			assert!(Self::recipients().contains(&recipient), "Recipient doesn't exist");
			// Get removed recipient percentrage and calculate augmented percentages.
			let pct = <RecipientPercentages<T>>::get(recipient.clone()).unwrap();
			// Remove recipient from pool and the mapping to their allocation
			Self::remove_recipient(recipient.clone());
			// Calculation occurs over updated set of recipients since we put it back.
			Self::augment_percentages(pct.current);
		}

		fn update_minting_interval(origin, interval: T::BlockNumber) {
			ensure_root(origin)?;
			<MintingInterval<T>>::put(interval);
			Self::deposit_event(RawEvent::MintingIntervalUpdate(interval));
		}

		fn update_reward_payout(origin, amount: BalanceOf<T>) {
			ensure_root(origin)?;
			<CurrentPayout<T>>::put(amount);
			Self::deposit_event(RawEvent::RewardPayoutUpdate(amount));
		}

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
	}
}

impl<T: Trait> Module<T> {
	fn initialize_recipients(recipients: Vec<T::AccountId>, pcts: Vec<Percent>) {
		assert!(recipients.len() == pcts.len(), "There must be a one-to-one mapping between recipients and percentages");
		<Recipients<T>>::put(recipients.clone());
		// Sum all percentages to ensure they're bounded by 100
		let sum = Self::sum_percentages(pcts.clone());
		assert!(sum <= 100, "Percentages must sum to at most 100");
		for i in 0..recipients.clone().len() {
			<RecipientPercentages<T>>::insert(recipients[i].clone(), RecipientAllocation {
				current: pcts[i],
				proposed: pcts[i],
			});
		}	
	}

	fn dilute_percentages(new_pct: Percent) {
		let recipients = Self::recipients();
		let dilution_frac = Self::get_leftover(new_pct);
		// multiply all percentages by dilution fraction
		for i in 0..recipients.len() {
			if let Some(mut alloc) = Self::recipient_percentages(recipients[i].clone()) {
				alloc.current = alloc.current.saturating_mul(dilution_frac);
				<RecipientPercentages<T>>::insert(recipients[i].clone(), alloc);
			}
		}
	}

	fn augment_percentages(old_pct: Percent) {
		let recipients = Self::recipients();
		let augment_frac = Self::get_leftover(old_pct);
		// divide all percetages by augment fraction
		for i in 0..recipients.len() {
			if let Some(mut alloc) = Self::recipient_percentages(recipients[i].clone()) {
				alloc.current = alloc.current / augment_frac;
				// Ensure augmenting never leads to higher than proposed allocation 
				if alloc.current.deconstruct() > alloc.proposed.deconstruct() {
					alloc.current = alloc.proposed;
				}
				<RecipientPercentages<T>>::insert(recipients[i].clone(), alloc);
			}
		}
	}

	fn sum_percentages(pcts: Vec<Percent>) -> u8 {
		let mut pct = 0;
		for i in 0..pcts.len() {
			pct += pcts[i].deconstruct();
		}

		pct
	}

	fn get_leftover(pct: Percent) -> Percent {
		Percent::from_percent(100).saturating_sub(pct)
	}

	fn get_available_recipient_alloc() -> Percent {
		let recipients = Self::recipients();
		let mut pct_sum = Percent::from_percent(0);
		for i in 0..recipients.len() {
			if let Some(alloc) = Self::recipient_percentages(recipients[i].clone()) {
				pct_sum = pct_sum.saturating_add(alloc.current);
			}
		}

		return Self::get_leftover(pct_sum);
	}

	fn add_recipient(recipient: T::AccountId, pct: Percent) {
		let mut recipients = Self::recipients();
		// Add the new recipient to the pool
		recipients.push(recipient.clone());
		<Recipients<T>>::put(recipients);
		// Add the recipients percentage
		<RecipientPercentages<T>>::insert(recipient.clone(), RecipientAllocation {
			current: pct,
			proposed: pct,
		});
		Self::deposit_event(RawEvent::RecipientAdded(recipient, pct));
	}

	fn remove_recipient(recipient: T::AccountId) {
		let mut recipients = Self::recipients();
		// Find recipient index and remove them
		let index = recipients.iter().position(|x| *x == recipient).unwrap();
		recipients.remove(index);
		// Put recipients back
		<Recipients<T>>::put(recipients.clone());
		// Remove the removed recipient's percentage from the map
		<RecipientPercentages<T>>::remove(recipient.clone());
		Self::deposit_event(RawEvent::RecipientRemoved(recipient));
	}
}

decl_event!(
	pub enum Event<T> where <T as frame_system::Trait>::BlockNumber,
							<T as frame_system::Trait>::AccountId,
							Balance = <T as pallet_balances::Trait>::Balance,
							Payout = <<T as pallet_staking::Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance {
		TreasuryMinting(Balance, BlockNumber, AccountId),
		RecipientAdded(AccountId, Percent),
		RecipientRemoved(AccountId),
		MintingIntervalUpdate(BlockNumber),
		RewardPayoutUpdate(Payout),
	}
);

decl_storage! {
	trait Store for Module<T: Trait> as TreasuryReward {
		/// Interval in number of blocks to reward treasury
		pub MintingInterval get(fn minting_interval) config(): T::BlockNumber;
		/// Current payout of module
		pub CurrentPayout get(fn current_payout) config(): BalanceOf<T>;
		/// Treasury reward recipients
		pub Recipients get(fn recipients): Vec<T::AccountId>;
		/// Treasury reward percentages mapping
		pub RecipientPercentages get(fn recipient_percentages): map hasher(twox_64_concat) T::AccountId => Option<RecipientAllocation>;
	}
	add_extra_genesis {
		config(recipients): Vec<T::AccountId>;
		config(recipient_percentages): Vec<Percent>;
		build(|config| Module::<T>::initialize_recipients(
			config.recipients.clone(),
			config.recipient_percentages.clone(),
		))
	}
}
