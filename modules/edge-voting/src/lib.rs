// Copyright 2018 Commonwealth Labs, Inc.
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

#[cfg(feature = "std")]
extern crate serde;

// Needed for deriving `Serialize` and `Deserialize` for various types.
// We only implement the serde traits for std builds - they're unneeded
// in the wasm runtime.
#[cfg(feature = "std")]
#[macro_use]
extern crate serde_derive;
#[cfg(test)]
#[macro_use]
extern crate hex_literal;
#[macro_use] extern crate parity_codec_derive;
#[macro_use] extern crate srml_support;


extern crate parity_codec as codec;
extern crate substrate_primitives as primitives;
#[cfg_attr(not(feature = "std"), macro_use)]
extern crate sr_std as rstd;
extern crate srml_support as runtime_support;
extern crate sr_primitives as runtime_primitives;
extern crate sr_io as runtime_io;

extern crate srml_balances as balances;
extern crate srml_system as system;
extern crate edge_delegation as delegation;

use rstd::prelude::*;
use runtime_support::dispatch::Result;

pub mod voting;
pub use voting::{Module, Trait, RawEvent, Event};
pub use voting::{VoteStage, VoteType, TallyType, VoteRecord, VoteData};

// Tests for Delegation Module
#[cfg(test)]
mod tests {
	
	use super::*;
	use runtime_io::ed25519::Pair;
	use system::{EventRecord, Phase};
	use runtime_io::with_externalities;
	use primitives::{H256, Blake2Hasher};
	// The testing primitives are very useful for avoiding having to work with signatures
	// or public keys. `u64` is used as the `AccountId` and no `Signature`s are requried.
	use runtime_primitives::{
		BuildStorage, traits::{BlakeTwo256}, testing::{Digest, DigestItem, Header}
	};


	impl_outer_origin! {
		pub enum Origin for Test {}
	}

	impl_outer_event! {
		pub enum Event for Test {
			voting<T>, delegation<T>, balances<T>,
		}
	}

	impl_outer_dispatch! {
		pub enum Call for Test where origin: Origin {}
	}

	// For testing the module, we construct most of a mock runtime. This means
	// first constructing a configuration type (`Test`) which `impl`s each of the
	// configuration traits of modules we want to use.
	#[derive(Clone, Eq, PartialEq)]
	pub struct Test;
	impl system::Trait for Test {
		type Origin = Origin;
		type Index = u64;
		type BlockNumber = u64;
		type Hash = H256;
		type Hashing = BlakeTwo256;
		type Digest = Digest;
		type AccountId = H256;
		type Header = Header;
		type Event = Event;
		type Log = DigestItem;
	}

	impl balances::Trait for Test {
		type Balance = u64;
		type AccountIndex = u64;
		type OnFreeBalanceZero = ();
		type EnsureAccountLiquid = ();
		type Event = Event;
	}

	impl delegation::Trait for Test {
		type Event = Event;
	}

	impl Trait for Test {
		type Event = Event;
	}

	pub type System = system::Module<Test>;
	pub type Delegation = delegation::Module<Test>;
	pub type Voting = Module<Test>;

	// This function basically just builds a genesis storage key/value store according to
	// our desired mockup.
	fn new_test_ext() -> sr_io::TestExternalities<Blake2Hasher> {
		let t = system::GenesisConfig::<Test>::default().build_storage().unwrap().0;
		// We use default for brevity, but you can configure as desired if needed.
		t.into()
	}

	fn create_vote(
		who: H256,
		vote_type: voting::VoteType,
		initialization_time: u64,
		expiration_time: u64,
		is_commit_reveal: bool,
		tally_type: voting::TallyType,
		outcomes: &[[u8; 32]]
	) -> Result {
		Voting::create_vote(Origin::signed(who),
							vote_type,
							initialization_time,
							expiration_time,
							is_commit_reveal,
							tally_type,
							outcomes.to_vec())
	}

	fn commit(who: H256, vote_id: u64, commit: [u8; 32]) -> Result {
		Voting::commit(Origin::signed(who), vote_id, commit)
	}

	fn reveal(who: H256, vote_id: u64, vote: [u8; 32], secret: Option<[u8; 32]>) -> Result {
		Voting::reveal(Origin::signed(who), vote_id, vote, secret)
	}

	fn get_test_key() -> H256 {
		let pair: Pair = Pair::from_seed(&hex!("9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"));
		let public: H256 = pair.public().0.into();
		return public;
	}

	fn generate_1p1v_public_binary_vote() -> (voting::VoteType, u64, u64, bool, voting::TallyType, [[u8; 32]; 2]) {
		let vote_type = VoteType::Binary;
		let tally_type = TallyType::OnePerson;
		let init_time = 1;
		let expire_time = 1;
		let is_commit_reveal = false;
		let yes_outcome: [u8; 32] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1];
		let no_outcome: [u8; 32] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];

		return (vote_type, init_time, expire_time, is_commit_reveal, tally_type, [yes_outcome, no_outcome]);
	}

	fn generate_1p1v_public_multi_vote() -> (voting::VoteType, u64, u64, bool, voting::TallyType, [[u8; 32]; 4]) {
		let vote_type = VoteType::MultiOption;
		let tally_type = TallyType::OnePerson;
		let init_time = 1;
		let expire_time = 1;
		let is_commit_reveal = false;
		let one_outcome: [u8; 32] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1];
		let two_outcome: [u8; 32] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2];
		let three_outcome: [u8; 32] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,3];
		let four_outcome: [u8; 32] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,4];

		return (vote_type, init_time, expire_time, is_commit_reveal, tally_type, [
			one_outcome,
			two_outcome,
			three_outcome,
			four_outcome
		]);
	}

	fn make_record(
		id: u64,
		author: H256,
		vote_type: voting::VoteType,
		initialization_time: u64,
		expiration_time: u64,
		is_commit_reveal: bool,
		tally_type: voting::TallyType,
		outcomes: &[[u8; 32]],
		create_time: u64,
	) -> VoteRecord<H256, u64> {
		VoteRecord {
			id: id,
			is_commit_reveal: is_commit_reveal,
			commitments: vec![],
			reveals: vec![],
			outcomes: outcomes.to_vec(),
			winning_outcome: None,
			data: VoteData {
				initiator: author,
				stage: VoteStage::PreVoting,
				vote_type: vote_type,
				creation_time: create_time,
				initialization_time: initialization_time + create_time,
				expiration_time: expiration_time + create_time + initialization_time,
				tally_type: tally_type,
			},
		}
	}

	#[test]
	fn create_binary_vote_should_work() {
		with_externalities(&mut new_test_ext(), || {
			System::set_block_number(1);
			let public = get_test_key();
			let vote = generate_1p1v_public_binary_vote();
			assert_ok!(create_vote(public, vote.0, vote.1, vote.2, vote.3, vote.4, &vote.5));
			assert_eq!(Voting::vote_record_count(), 1);
			assert_eq!(
				Voting::vote_records(1),
				Some(make_record(1, public, vote.0, vote.1, vote.2, vote.3, vote.4, &vote.5, 1))
			);
		});
	}

	#[test]
	fn create_multi_vote_should_work() {
		with_externalities(&mut new_test_ext(), || {
			System::set_block_number(1);
			let public = get_test_key();
			let vote = generate_1p1v_public_multi_vote();
			assert_ok!(create_vote(public, vote.0, vote.1, vote.2, vote.3, vote.4, &vote.5));
			assert_eq!(Voting::vote_record_count(), 1);
			assert_eq!(
				Voting::vote_records(1),
				Some(make_record(1, public, vote.0, vote.1, vote.2, vote.3, vote.4, &vote.5, 1))
			);
		});
	}
}
