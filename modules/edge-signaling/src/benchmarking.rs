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

//! signaling pallet benchmarking.

#![cfg(feature = "runtime-benchmarks")]

use super::*;

use frame_benchmarking::{benchmarks, account, whitelist_account};
use frame_support::traits::Currency;
use frame_system::{EventRecord, RawOrigin, self};
use sp_runtime::traits::Bounded;

use crate::Module as Signaling;

const SEED: u32 = 0;
const YES_VOTE: voting::VoteOutcome = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1];
const NO_VOTE: voting::VoteOutcome = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
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
const MAX_PROPOSALS: u32 = 99;
const MAX_BYTES: u32 = 16_384;

fn funded_account<T: Trait>(name: &'static str, index: u32) -> T::AccountId {
	let caller: T::AccountId = account(name, index, SEED);
	T::Currency::make_free_balance_be(&caller, BalanceOf::<T>::max_value());
	caller
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

	// Benchmark `create_proposal` extrinsic
	create_proposal {
		let p in 1 .. MAX_PROPOSALS;
		let b in 1 .. MAX_BYTES;
		// TODO: benchmark max outcomes as well

		let proposer = funded_account::<T>("proposer", 0);
		whitelist_account!(proposer);

		let title: &[u8] = b"Edgeware";

		let contents = vec![1; b as usize];
		let outcomes = vec![YES_VOTE, NO_VOTE];

		let mut buf = Vec::new();
		buf.extend_from_slice(&proposer.encode());
		buf.extend_from_slice(&contents.as_ref());
		let hash = T::Hashing::hash(&buf[..]);
	}: _(RawOrigin::Signed(proposer.clone()), title.into(), contents, outcomes, VoteType::Binary, TallyType::OneCoin)
	verify {
		assert_last_event::<T>(Event::<T>::NewProposal(proposer, hash).into());
		assert!(Signaling::<T>::proposal_of(hash).is_some());
	}

	// Benchmark `advance_proposal` extrinsic
	advance_proposal {
		let p in 1 .. MAX_PROPOSALS;
		// TODO: benchmark max outcomes as well

		let proposer = funded_account::<T>("proposer", 0);
		whitelist_account!(proposer);
		let origin: <T as frame_system::Trait>::Origin = RawOrigin::Signed(proposer.clone()).into();

		let title: &[u8] = b"Edgeware";

		let contents = p.to_le_bytes().to_vec();
		let outcomes = vec![YES_VOTE, NO_VOTE];

		let mut buf = Vec::new();
		buf.extend_from_slice(&proposer.encode());
		buf.extend_from_slice(&contents.as_ref());
		let hash = T::Hashing::hash(&buf[..]);
		Signaling::<T>::create_proposal(origin, title.into(), contents, outcomes, VoteType::Binary, TallyType::OneCoin)?;
	}: _(RawOrigin::Signed(proposer), hash)
	verify {
		// TODO: fix this assert
		// assert_last_event::<T>(Event::<T>::VotingStarted(hash, p, 1).into());
		assert!(Signaling::<T>::proposal_of(hash).is_some());
		assert_eq!(Signaling::<T>::proposal_of(hash).unwrap().stage, VoteStage::Voting);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::tests_composite::{ExtBuilder, Test};
	use frame_support::assert_ok;

	fn test_benchmarks() {
		new_test_ext().execute_with(|| {
			assert_ok!(test_benchmark_create_proposal::<Test>());
			assert_ok!(test_benchmark_advance_proposal::<Test>());
		});
	}
}