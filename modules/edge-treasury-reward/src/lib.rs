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
use frame_support::traits::{Get, Currency};
use sp_std::prelude::*;
use sp_runtime::{Percent, RuntimeDebug};
use sp_runtime::traits::{Zero, Saturating};
use frame_support::{decl_event, decl_module, decl_storage, decl_error, ensure};

use frame_system::{self as system, ensure_root};
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

decl_error! {
	pub enum Error for Module<T: Trait> {
		FailedToAdd,
		FailedToRemove,
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		/// Adds a new recipient to the recipients list and assigns them
		/// the submitted percentage of the leftover treasury reward.
		/// If there is no leftover allocation, the other recipients'
		/// reward percentages will be diluted.
		#[weight = 5_000_000]
		fn add(origin, recipient: T::AccountId, pct: Percent) {
			ensure_root(origin)?;
			ensure!(pct.deconstruct() <= T::MaximumRecipientPct::get().deconstruct(), "Invalid proposed percentage. Too large.");
			ensure!(!Self::recipients().contains(&recipient), "Duplicate recipients not allowed");
			let leftover_allocation = Self::get_available_recipient_alloc();
			// Dilute current allocations by overflowed percentage
			if pct.deconstruct() > leftover_allocation.deconstruct() {
				let diff = pct.saturating_sub(leftover_allocation);
				let first_portion = pct.saturating_sub(diff);
				let second_portion = pct.saturating_sub(first_portion);
				// Add new recipient with diff first, this needs to be diluted with the leftover
				Self::add_recipient(recipient.clone(), pct, first_portion);
				ensure!(Self::sum_percentages(Self::get_recipient_pcts()) <= 100, "Invalid percentage calculation");
				Self::dilute_percentages(diff);
				Self::add_to_allocation(recipient, second_portion);
			} else {
				// Add new recipient
				Self::add_recipient(recipient, pct, pct);
			}
			ensure!(Self::sum_percentages(Self::get_recipient_pcts()) <= 100, "Invalid percentage calculation");
		}

		/// Removes an existing recipient from the active list and dilutes
		/// all remaining participants current percentages by that deleted amount.
		/// Dilution should only occur up until the proposed percentages each
		/// active participant was added to the set with.
		#[weight = 5_000_000]
		fn remove(origin, recipient: T::AccountId) {
			ensure_root(origin)?;
			ensure!(Self::recipients().contains(&recipient), "Recipient doesn't exist");
			// Get removed recipient percentrage and calculate augmented percentages.
			let pct = <RecipientPercentages<T>>::get(recipient.clone()).unwrap();
			// Remove recipient from pool and the mapping to their allocation
			Self::remove_recipient(recipient.clone());
			// Calculation occurs over updated set of recipients since we put it back.
			Self::augment_percentages(pct.proposed);
			ensure!(Self::sum_percentages(Self::get_recipient_pcts()) <= 100, "Invalid percentage calculation");
		}

		/// Updates an existing recipients allocation by removing and adding
		/// them into the set. This will cause a dilution and inflation of the
		/// set and does lose precision in the process.
		#[weight = 5_000_000]
		fn update(origin, recipient: T::AccountId, pct: Percent) {
			ensure_root(origin.clone())?;
			ensure!(pct.deconstruct() <= T::MaximumRecipientPct::get().deconstruct(), "Invalid proposed percentage. Too large.");
			Self::remove(origin.clone(), recipient.clone()).map_err(|_| Error::<T>::FailedToRemove)?;
			Self::add(origin, recipient, pct).map_err(|_| Error::<T>::FailedToAdd)?;
			ensure!(Self::sum_percentages(Self::get_recipient_pcts()) <= 100, "Invalid percentage calculation");
		}

		/// Updates the minting interval of the treasury reward process
		#[weight = 5_000_000]
		fn set_minting_interval(origin, interval: T::BlockNumber) {
			ensure_root(origin)?;
			<MintingInterval<T>>::put(interval);
			Self::deposit_event(RawEvent::MintingIntervalUpdate(interval));
		}

		/// Updates the current payout of the treasury reward process
		#[weight = 5_000_000]
		fn set_current_payout(origin, amount: BalanceOf<T>) {
			ensure_root(origin)?;
			<CurrentPayout<T>>::put(amount);
			Self::deposit_event(RawEvent::RewardPayoutUpdate(amount));
		}

		/// Mint money for the treasury and recipient pool!
		fn on_finalize(_n: T::BlockNumber) {
			if <frame_system::Module<T>>::block_number() % Self::minting_interval() == Zero::zero() {
				let reward = Self::current_payout();
				// get up front treasury reward from minimum amount that is always allocated
				let mut treasury_reward = T::MinimumTreasuryPct::get() * reward;
				// up front allocation being split between recipients, any leftover goes to Treasury
				let leftover_recipient_alloc = Self::get_leftover(T::MinimumTreasuryPct::get());
				// up front reward that gets divided between recipients; the recipients current
				// allocation percentage denotes their fraction of the leftover_recipient_allocation
				let leftover_recipients_reward = leftover_recipient_alloc * reward;
				let recipients = Self::recipients();
				let mut allocated_to_recipients: BalanceOf<T> = 0.into();
				for i in 0..recipients.len() {
					if let Some(alloc) = Self::recipient_percentages(recipients[i].clone()) {
						// calculate fraction for recipient i
						let reward_i = alloc.current * leftover_recipients_reward;
						// reward the recipient
						<T as pallet_staking::Trait>::Currency::deposit_creating(
							&recipients[i].clone(),
							reward_i.clone(),
						);
						// emit event of payout
						Self::deposit_event(RawEvent::TreasuryMinting(
							<pallet_balances::Module<T>>::free_balance(recipients[i].clone()),
							<frame_system::Module<T>>::block_number(),
							recipients[i].clone())
						);
						// track currently allocated amount to recipients
						allocated_to_recipients = allocated_to_recipients + reward_i;
					}
				}

				// update treasury reward with any leftover reward deducted by what was allocated
				// or ensure that if no recipients exist, to provide entire reward to the treasury
				if recipients.len() == 0 {
					treasury_reward = reward;
				} else {
					treasury_reward = treasury_reward + leftover_recipients_reward - allocated_to_recipients;
				}

				// allocate reward to the Treasury
				<T as pallet_staking::Trait>::Currency::deposit_creating(
					&<pallet_treasury::Module<T>>::account_id(),
					treasury_reward,
				);
				// emit event of payout
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

	fn get_recipient_pcts() -> Vec<Percent> {
		let recipients = Self::recipients();
		let mut pcts = vec![];
		for i in 0..recipients.len() {
			if let Some(alloc) = Self::recipient_percentages(recipients[i].clone()) {
				pcts.push(alloc.current);
			}
		}

		return pcts;
	}

	/// Sums a vector of percentages
	fn sum_percentages(pcts: Vec<Percent>) -> u8 {
		let mut pct = 0;
		for i in 0..pcts.len() {
			pct += pcts[i].deconstruct();
		}

		return pct;
	}

	/// Calculates the difference between 100 percent and a provided percentage 
	fn get_leftover(pct: Percent) -> Percent {
		Percent::from_percent(100).saturating_sub(pct)
	}

	/// Calculates the remaining, leftover percentage that can be allocated
	/// to any set of recipients without diluting all the other recipients
	/// allocation
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

	/// Helper function to add a recipient into the module's storage
	fn add_recipient(recipient: T::AccountId, proposed_pct: Percent, current_pct: Percent) {
		let mut recipients = Self::recipients();
		// Add the new recipient to the pool
		recipients.push(recipient.clone());
		<Recipients<T>>::put(recipients);
		// Add the recipients percentage
		<RecipientPercentages<T>>::insert(recipient.clone(), RecipientAllocation {
			current: current_pct,
			proposed: proposed_pct,
		});
		Self::deposit_event(RawEvent::RecipientAdded(recipient, proposed_pct));
	}

	/// Helper function to remove a recipient from the module's storage
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

	/// Adds a percentage increase to a recipients allocation
	fn add_to_allocation(recipient: T::AccountId, pct: Percent) {
		if let Some(mut alloc) = Self::recipient_percentages(recipient.clone()) {
			alloc.current = alloc.current.saturating_add(pct);
			<RecipientPercentages<T>>::insert(recipient, alloc);
		}
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
