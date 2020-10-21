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
use frame_system::{RawOrigin, self};
use sp_runtime::traits::Bounded;

use crate::Module as Signaling;

const SEED: u32 = 0;
const YES_VOTE: voting::VoteOutcome = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1];
const NO_VOTE: voting::VoteOutcome = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
const MAX_PROPOSALS: u32 = 99;
const MAX_BYTES: u32 = 16_384;

fn funded_account<T: Trait>(name: &'static str, index: u32) -> T::AccountId {
	let caller: T::AccountId = account(name, index, SEED);
	T::Currency::make_free_balance_be(&caller, BalanceOf::<T>::max_value());
	caller
}

benchmarks! {
	_ { }

	// Benchmark `create_proposal` extrinsic
	// NOTES:
	//	- In past benchmarks of outcome size, they had no effect on weight, so we do not include them here.
	//			Number of outcomes should still be capped to a reasonable number.
	//	- Since no branching logic depends on tally type or vote type, we omit these from consideration.
	create_proposal {
		let p in 1 .. MAX_PROPOSALS;
		let b in 2 .. MAX_BYTES;

		let proposer = funded_account::<T>("proposer", 0);
		whitelist_account!(proposer);
		let origin: <T as frame_system::Trait>::Origin = RawOrigin::Signed(proposer.clone()).into();

		let title: &[u8] = b"Edgeware";

		// Create p existing proposals
		for i in 0 .. p {
			let contents = vec![i.to_le_bytes()[0]; b as usize];
			let outcomes = vec![YES_VOTE, NO_VOTE];
			Signaling::<T>::create_proposal(origin.clone(), title.into(), contents, outcomes, VoteType::Binary, TallyType::OneCoin, VotingScheme::Simple)?;	
		}
		assert_eq!(Signaling::<T>::inactive_proposals().len(), p as usize);

		// create new proposal
		let contents = vec![p.to_le_bytes()[0]; b as usize];
		let outcomes = vec![YES_VOTE, NO_VOTE];

		let mut buf = Vec::new();
		buf.extend_from_slice(&proposer.encode());
		buf.extend_from_slice(&contents.as_ref());
		let hash = T::Hashing::hash(&buf[..]);
	}: _(RawOrigin::Signed(proposer.clone()), title.into(), contents, outcomes, VoteType::Binary, TallyType::OneCoin, VotingScheme::Simple)
	verify {
		assert!(Signaling::<T>::proposal_of(hash).is_some());
		assert_eq!(Signaling::<T>::inactive_proposals().len(), (p+1) as usize);
	}

	// Benchmark `advance_proposal` extrinsic
	// NOTES:
	// - See above note regarding outcomes.
	// - No branching logic depends on commit-reveal, so we use the simple voting scheme for this benchmark.
	advance_proposal {
		let p in 1 .. MAX_PROPOSALS;

		let proposer = funded_account::<T>("proposer", 0);
		whitelist_account!(proposer);
		let origin: <T as frame_system::Trait>::Origin = RawOrigin::Signed(proposer.clone()).into();

		let title: &[u8] = b"Edgeware";

		// Create p existing proposals
		for i in 0 .. p {
			let contents = vec![i.to_le_bytes()[0]; MAX_BYTES as usize];
			let outcomes = vec![YES_VOTE, NO_VOTE];

			let mut buf = Vec::new();
			buf.extend_from_slice(&proposer.encode());
			buf.extend_from_slice(&contents.as_ref());
			let hash = T::Hashing::hash(&buf[..]);
			Signaling::<T>::create_proposal(origin.clone(), title.into(), contents, outcomes, VoteType::Binary, TallyType::OneCoin, VotingScheme::Simple)?;	
			Signaling::<T>::advance_proposal(origin.clone(), hash)?;
		}
		assert_eq!(Signaling::<T>::active_proposals().len(), p as usize);

		let contents = vec![p.to_le_bytes()[0]; MAX_BYTES as usize];
		let outcomes = vec![YES_VOTE, NO_VOTE];

		let mut buf = Vec::new();
		buf.extend_from_slice(&proposer.encode());
		buf.extend_from_slice(&contents.as_ref());
		let hash = T::Hashing::hash(&buf[..]);
		Signaling::<T>::create_proposal(origin, title.into(), contents, outcomes, VoteType::Binary, TallyType::OneCoin, VotingScheme::Simple)?;
	}: _(RawOrigin::Signed(proposer), hash)
	verify {
		assert!(Signaling::<T>::proposal_of(hash).is_some());
		assert_eq!(Signaling::<T>::proposal_of(hash).unwrap().stage, VoteStage::Voting);
		assert_eq!(Signaling::<T>::active_proposals().len(), (p+1) as usize);
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