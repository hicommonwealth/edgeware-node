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

use frame_benchmarking::{benchmarks, account, whitelisted_caller};
use frame_support::{
	IterableStorageMap,
	traits::{Currency, Get, EnsureOrigin, OnInitialize, UnfilteredDispatchable, schedule::DispatchTime},
};
use frame_system::{RawOrigin, Module as System, self, EventRecord};
use sp_runtime::traits::Bounded;

use crate::Module as Signaling;

const YES_VOTE: voting::VoteOutcome = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1];
const NO_VOTE: voting::VoteOutcome = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
const MAX_PROPOSALS: u32 = 99;

fn funded_account<T: Trait>(name: &'static str, index: u32) -> T::AccountId {
	let caller: T::AccountId = account(name, index, SEED);
	T::Currency::make_free_balance_be(&caller, BalanceOf::<T>::max_value());
	caller
}

fn propose<T: Trait>(n: u32) -> Result<T::Hash, &'static str> {
	let proposer = funded_account::<T>("proposer", n);
	
	let title: &[u8] = b"Edgeware";
	let contents: &[u8] = n[..];
	let outcomes = vec![YES_VOTE, NO_VOTE];

	let mut buf = Vec::new();
	buf.extend_from_slice(&proposer.encode());
	buf.extend_from_slice(&contents.as_ref());
	let hash = T::Hashing::hash(&buf[..]);
	
	Signaling::<T>::create_proposal(
		RawOrigin::Signed(proposer.clone()),
		title,
		contents,
		VoteType::Binary,
		TallyType::OneCoin,
	)?;

	assert!(Signaling::<T>::proposal_of(hash).is_some());
	Ok(hash)
}

benchmarks! {
	_ { }

	// Benchmark `create_proposal` extrinsic
	create_proposal {
		let p in 1 .. MAX_REFERENDUMS;
		for i in 0 .. p {
			propose::<T>(i)?;
		}

		let proposer = funded_account::<T>("proposer", 0);
		let proposal_hash = Signaling::<T>::active_proposals()[0][0];
		whitelist_account!(caller);
	}: _(RawOrigin::Signed(proposer), proposal_hash)
	verify {
		assert_eq(Signaling::<T>::active_proposals().len(), p as usize, "Proposals not created");
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