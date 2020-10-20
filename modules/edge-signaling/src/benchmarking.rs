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
use frame_support::{
	IterableStorageMap,
	traits::{Currency, Get, EnsureOrigin, OnInitialize, UnfilteredDispatchable, schedule::DispatchTime},
};
use frame_system::{RawOrigin, Module as System, self, EventRecord};
use sp_runtime::traits::Bounded;

use crate::Module as Signaling;

const SEED: u32 = 0;
const YES_VOTE: voting::VoteOutcome = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1];
const NO_VOTE: voting::VoteOutcome = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
const MAX_PROPOSALS: u32 = 99;

fn funded_account<T: Trait>(name: &'static str, index: u32) -> T::AccountId {
	let caller: T::AccountId = account(name, index, SEED);
	T::Currency::make_free_balance_be(&caller, BalanceOf::<T>::max_value());
	caller
}

benchmarks! {
	_ { }

	// Benchmark `create_proposal` extrinsic
	create_proposal {
		let proposer = funded_account::<T>("proposer", 0);
		whitelist_account!(proposer);

		let title: &[u8] = b"Edgeware";
		let contents = (10 as u32).to_be_bytes();
		let outcomes = vec![YES_VOTE, NO_VOTE];

		let mut buf = Vec::new();
		buf.extend_from_slice(&proposer.encode());
		buf.extend_from_slice(&contents.as_ref());
		let hash = T::Hashing::hash(&buf[..]);
	}: _(RawOrigin::Signed(proposer), title.into(), contents.to_vec(), outcomes, VoteType::Binary, TallyType::OneCoin)
	verify {
		assert!(Signaling::<T>::proposal_of(hash).is_some());
		assert_eq!(Signaling::<T>::inactive_proposals().len(), 1 as usize, "Proposals not created");
	}

	// Benchmark `advance_proposal` extrinsic
	// advance_proposal {

	// }
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::tests_composite::{ExtBuilder, Test};
	use frame_support::assert_ok;

	fn test_benchmarks() {
		new_test_ext().execute_with(|| {
			assert_ok!(test_benchmark_create_proposal::<Test>());
			// assert_ok!(test_benchmark_advance_proposal::<Test>());
		});
	}
}