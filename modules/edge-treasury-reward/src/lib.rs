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

use frame_system::ensure_root;
use frame_support::traits::{Get, Currency};
use sp_std::prelude::*;
use sp_runtime::{Percent, PerThing};
use sp_runtime::traits::{Zero, Saturating};

use frame_support::{decl_event, decl_module, decl_storage};
use frame_system::{self as system};

pub type BalanceOf<T> = <<T as pallet_staking::Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;

pub trait Trait: pallet_staking::Trait + pallet_treasury::Trait + pallet_balances::Trait {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
	/// The account balance
	type Currency: Currency<Self::AccountId>;
	/// Fraction of a treasury reward that goes to the Treasury account itself
	type TreasuryRewardPercentage: Get<Percent>;
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		/// Adds a new recipient to the recipients list and assigns them
		/// the submitted percentage of the leftover treasury reward.
		/// As a result, the other recipients' reward percentages will be
		/// diluted. This should be taken into account when submitting and
		/// voting on add_recipient proposals.
		fn add_recipient(origin, recipient: T::AccountId, pct: Percent) {
			ensure_root(origin)?;
			assert!(!Self::recipients().contains(&recipient), "Duplicate recipients not allowed");
			let mut recipients = Self::recipients();
			// Get dilulted existing percentages for each existing recipient;
			let new_pcts = Self::dilute_percentages(pct);
			// Dilute all percentages before adding new recipient
			for i in 0..recipients.len() {
				<RecipientPercentages<T>>::insert(recipients[i].clone(), new_pcts[i].clone());
			}
			// Add the new recipient
			recipients.push(recipient.clone());
			<Recipients<T>>::put(recipients);
			// Add the recipients percentage
			<RecipientPercentages<T>>::insert(recipient.clone(), pct.clone());
			Self::deposit_event(RawEvent::RecipientAdded(recipient, pct));
		}

		fn remove_recipient(origin, recipient: T::AccountId) {
			ensure_root(origin)?;
			assert!(Self::recipients().contains(&recipient), "Invalid recipient");
			let mut recipients = Self::recipients();
			// Find recipient index and remove them
			let index = recipients.iter().position(|x| *x == recipient).unwrap();
			recipients.remove(index);
			// Put recipients back
			<Recipients<T>>::put(recipients.clone());
			// Get removed recipient percentrage and calculate augmented percentages.
			let pct = <RecipientPercentages<T>>::get(recipient.clone()).unwrap();
			// Calculation occurs over updated set of recipients since we put it back.
			let new_pcts = Self::augment_percentages(pct);
			// Remove the removed recipient's percentage from the map
			<RecipientPercentages<T>>::remove(recipient.clone());
			// Update all recipients percentages in the mapping
			for i in 0..recipients.len() {
				<RecipientPercentages<T>>::insert(recipients[i].clone(), new_pcts[i].clone());
			}
			Self::deposit_event(RawEvent::RecipientRemoved(recipient));
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
		assert!(recipients.len() != pcts.len(), "There must be a one-to-one mapping between recipients and percentages");
		<Recipients<T>>::put(recipients.clone());
		// Sum all percentages to ensure they're bounded by 100
		let sum = Self::sum_percentages(pcts.clone());
		assert!(sum.deconstruct() <= 100, "Percentages must sum to at most 100");
		// Get leftover treasury reward percentage after the treasury's percentage
		let leftover = Self::get_leftover(T::TreasuryRewardPercentage::get());
		for i in 0..recipients.clone().len() {
			// Get recipients percentage of leftover percentage and add to mapping
			let leftover_pct = leftover.saturating_mul(pcts[i]);
			<RecipientPercentages<T>>::insert(recipients[i].clone(), leftover_pct);
		}	
	}

	fn dilute_percentages(new_pct: Percent) -> Vec<Percent> {
		let dilution_frac = Self::get_leftover(new_pct);
		// multiple all percetages by dilution fraction
		Self::get_percentages().iter().map(|p| p.saturating_mul(dilution_frac)).collect()
	}

	fn augment_percentages(old_pct: Percent) -> Vec<Percent> {
		let augment_frac = Self::get_leftover(old_pct);
		// divide all percetages by augment fraction
		Self::get_percentages().iter().map(|p| *p / augment_frac).collect()
	}

	fn get_percentages() -> Vec<Percent> {
		let mut pcts = vec![];
		let recipients = Self::recipients();
		for i in 0..recipients.len() {
			if let Some(pct) = Self::recipient_percentages(recipients[i].clone()) {
				pcts.push(pct);
			}
		}
		
		pcts
	}

	fn sum_percentages(pcts: Vec<Percent>) -> Percent {
		let pct = Percent::from_percent(0);
		for i in 0..pcts.len() {
			pct.saturating_add(pcts[i].clone());
		}

		pct
	}

	fn get_leftover(pct: Percent) -> Percent {
		Percent::from_percent(100).saturating_sub(pct)
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
		pub RecipientPercentages get(fn recipient_percentages): map hasher(twox_64_concat) T::AccountId => Option<Percent>;
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
