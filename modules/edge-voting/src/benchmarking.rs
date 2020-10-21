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
use sp_runtime::traits::{BlakeTwo256, Bounded};

use crate::Module as Voting;

const SEED: u32 = 0;
const YES_VOTE: VoteOutcome = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1];
const NO_VOTE: VoteOutcome = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
const MULTI_OUTCOMES: [[u8; 32]; 10] = [
	[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
	[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
	[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2],
	[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,3],
	[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,4],
	[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,5],
	[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,6],
	[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,7],
	[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,8],
	[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,9],
];
const MAX_VOTERS: u32 = 100;

static SECRET: [u8; 32] = [1,0,1,0,1,0,1,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,4];

fn funded_account<T: Trait>(name: &'static str, index: u32) -> T::AccountId {
	let caller: T::AccountId = account(name, index, SEED);
	// T::Currency::make_free_balance_be(&caller, BalanceOf::<T>::max_value());
	caller
}

fn encode_ranked_vote<T: Trait>(voter: T::AccountId) -> VoteOutcome {
	// create hash to commit
	let mut buf = vec![];
	buf.extend_from_slice(&voter.encode());
	buf.extend_from_slice(&SECRET.encode());
	for i in 0..MULTI_OUTCOMES.len() {
		buf.extend_from_slice(&MULTI_OUTCOMES[i].encode());
	}
	BlakeTwo256::hash_of(&buf).into()
}

fn add_simple_binary_vote<T: Trait>(n: u32) -> Result<u64, &'static str> {
	let other = funded_account::<T>("proposer", n);
	let id = Voting::<T>::create_vote(
		other, VoteType::Binary, false, TallyType::OneCoin, vec![YES_VOTE, NO_VOTE]
	)?;
	Voting::<T>::advance_stage(id)?;
	Ok(id)
}

fn add_commit_reveal_ranked_vote<T: Trait>(n: u32) -> Result<u64, &'static str> {
	let other = funded_account::<T>("proposer", n);
	let id = Voting::<T>::create_vote(
		other, VoteType::RankedChoice, true, TallyType::OneCoin, MULTI_OUTCOMES.to_vec(),
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

	commit {
		let s in 1 .. MAX_VOTERS;

		let caller = funded_account::<T>("caller", 0);
		let id = add_commit_reveal_ranked_vote::<T>(s)?;

		// Create s existing "voter"
		for i in 0 .. s {
			let voter = funded_account::<T>("voter", i);
			let hash = encode_ranked_vote::<T>(voter.clone());
			Voting::<T>::commit(RawOrigin::Signed(voter).into(), id, hash.into())?;
		}

		let record = Voting::<T>::vote_records(id).ok_or("Proposal not created")?;
		assert_eq!(record.commitments.len(), s as usize, "Votes not recorded");
		whitelist_account!(caller);
		let hash = encode_ranked_vote::<T>(caller.clone());
	}: _(RawOrigin::Signed(caller), id, hash)
	verify {
		let record = Voting::<T>::vote_records(id).ok_or("Proposal not created")?;
		assert_eq!(record.commitments.len(), (s + 1) as usize, "Vote not recorded");
	}

	reveal {
		let s in 1 .. MAX_VOTERS;

		let caller = funded_account::<T>("caller", 0);
		let id = add_commit_reveal_ranked_vote::<T>(s)?;

		// Create s existing "voters"
		for i in 0 .. s {
			let voter = funded_account::<T>("voter", i);
			let hash = encode_ranked_vote::<T>(voter.clone());
			Voting::<T>::commit(RawOrigin::Signed(voter).into(), id, hash)?;
		}

		whitelist_account!(caller);
		let hash = encode_ranked_vote::<T>(caller.clone());
		Voting::<T>::commit(RawOrigin::Signed(caller.clone()).into(), id, hash)?;

		// move to reveal phase and perform reveals
		Voting::<T>::advance_stage(id)?;

		for i in 0 .. s {
			let voter = funded_account::<T>("voter", i);
			Voting::<T>::reveal(RawOrigin::Signed(voter).into(), id, MULTI_OUTCOMES.to_vec(), Some(SECRET))?;
		}

		let record = Voting::<T>::vote_records(id).ok_or("Proposal not created")?;
		assert_eq!(record.reveals.len(), s as usize, "Votes not recorded");
	}: _(RawOrigin::Signed(caller), id, MULTI_OUTCOMES.to_vec(), Some(SECRET))
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