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

//! voting pallet benchmarking.

#![cfg(feature = "runtime-benchmarks")]

use super::*;

use frame_benchmarking::{benchmarks, account, whitelist_account};
use frame_support::traits::Currency;
use frame_system::{EventRecord, RawOrigin, self};
use sp_runtime::traits::Bounded;

use crate::Module as Voting;

const SEED: u32 = 0;
const YES_VOTE: VoteOutcome = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1];
const NO_VOTE: VoteOutcome = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
const MAX_VOTERS: u32 = 100;

fn funded_account<T: Trait>(name: &'static str, index: u32) -> T::AccountId {
	let caller: T::AccountId = account(name, index, SEED);
	// T::Currency::make_free_balance_be(&caller, BalanceOf::<T>::max_value());
	caller
}

fn add_simple_binary_vote<T: Trait>(n: u32) -> Result<u64, &'static str> {
	let other = funded_account::<T>("proposer", n);
	let id = Voting::<T>::create_vote(
		other, VoteType::Binary, false, TallyType::OneCoin, vec![YES_VOTE, NO_VOTE]
	)?;
	Voting::<T>::advance_stage(id)?;
	Ok(id)
}

fn assert_last_event<T: Trait>(generic_event: <T as Trait>::Event) {
	let events = frame_system::Module::<T>::events();
	let system_event: <T as frame_system::Trait>::Event = generic_event.into();
	// compare to the last event record
	let EventRecord { event, .. } = &events[events.len() - 1];
	assert_eq!(event, &system_event);
}

benchmarks! {
	_ { }

	reveal {
		let s in 1 .. MAX_VOTERS;

		let caller = funded_account::<T>("caller", 0);
		let id = add_simple_binary_vote::<T>(s)?;

		// Create s existing "seconds"
		for i in 0 .. s {
			let seconder = funded_account::<T>("seconder", i);
			Voting::<T>::reveal(RawOrigin::Signed(seconder).into(), id, vec![YES_VOTE], None)?;
		}

		let record = Voting::<T>::vote_records(id).ok_or("Proposal not created")?;
		assert_eq!(record.reveals.len(), s as usize, "Votes not recorded");
		whitelist_account!(caller);
	}: _(RawOrigin::Signed(caller), id, vec![YES_VOTE], None)
	verify {
		let record = Voting::<T>::vote_records(id).ok_or("Proposal not created")?;
		assert_eq!(record.reveals.len(), (s + 1) as usize, "Vote not recorded");
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::tests_composite::{ExtBuilder, Test};
	use frame_support::assert_ok;

	fn test_benchmarks() {
		new_test_ext().execute_with(|| {
			assert_ok!(test_benchmark_commit::<Test>());
			assert_ok!(test_benchmark_reveal::<Test>());
		});
	}
}