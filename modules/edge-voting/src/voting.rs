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

extern crate parity_codec as codec;
extern crate substrate_primitives as primitives;
extern crate sr_std as rstd;
extern crate srml_support as runtime_support;
extern crate sr_primitives as runtime_primitives;
extern crate sr_io as runtime_io;

extern crate srml_balances as balances;
extern crate srml_system as system;
extern crate edge_delegation as delegation;

use std::collections::HashMap;

use rstd::prelude::*;
use system::ensure_signed;
use runtime_support::{StorageValue, StorageMap};
use runtime_support::dispatch::Result;
use runtime_primitives::traits::Hash;
use runtime_primitives::traits::{Zero, One};
use runtime_primitives::traits::{CheckedAdd, CheckedSub};
use codec::Encode;

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, Copy, Clone, Eq, PartialEq)]
pub enum VoteStage {
	// Before voting stage, no votes accepted
	PreVoting,
	// Commit stage, only for commit-reveal-type elections
	Commit,
	// Active voting stage, votes (reveals) allowed
	Voting,
	// Completed voting stage, no more votes allowed
	Completed,
}

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, Copy, Clone, Eq, PartialEq)]
pub enum VoteType {
	// Binary decision vote, i.e. 2 outcomes
	Binary,
	// Multi option decision vote, i.e. > 2 possible outcomes
	// TODO: Add support for this type
	MultiOption,
	// Anonymous vote using ring signatures
	// TODO: Add support for this type
	AnonymousRing,
	// Anonymous vote using merkle tree accumulators
	// TODO: Add support for this type
	AnonymousMerkle,
}

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, Copy, Clone, Eq, PartialEq)]
pub enum TallyType {
	// 1 person 1 vote, i.e. 1 account 1 vote
	OnePerson,
	// 1 coin 1 vote, i.e. by balances
	OneCoin,
}

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, PartialEq)]
pub struct VoteData<AccountId> {
	// creator of vote
	pub initiator: AccountId,
	// Stage of the vote
	pub stage: VoteStage,
	// Type of vote defined abovoe
	pub vote_type: VoteType,
	// Tally metric
	pub tally_type: TallyType,
	// Flag for commit/reveal voting scheme
	pub is_commit_reveal: bool,
}

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, PartialEq)]
pub struct VoteRecord<AccountId, Balance> {
	// Identifier of the vote
	pub id: u64,
	// Vote commitments
	pub commitments: Vec<(AccountId, [u8; 32])>,
	// Vote reveals with 2^32 possible options
	pub reveals: Vec<(AccountId, [u8; 32])>,
	// Vote data record
	pub data: VoteData<AccountId>,
	// Vote outcomes
	pub outcomes: Vec<[u8; 32]>,
	// Winning outcome
	pub winning_outcome: Option<[u8; 32]>,
	// Final tally
	pub tally: Vec<([u8; 32], Balance)>,
}

pub trait Trait: balances::Trait + delegation::Trait {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event<T>() = default;

		pub fn create_vote(
			origin,
			vote_type: VoteType,
			is_commit_reveal: bool,
			tally_type: TallyType,
			outcomes: Vec<[u8; 32]>
		) -> Result {
			let _sender = ensure_signed(origin)?;
			ensure!(vote_type == VoteType::Binary || vote_type == VoteType::MultiOption, "Unsupported vote type");

			if vote_type == VoteType::Binary { ensure!(outcomes.len() == 2, "Invalid binary outcomes") }
			if vote_type  == VoteType::MultiOption { ensure!(outcomes.len() > 2, "Invalid multi option outcomes") }

			let id = Self::vote_record_count() + 1;
			<VoteRecords<T>>::insert(id, VoteRecord {
				id: id,
				commitments: vec![],
				reveals: vec![],
				outcomes: outcomes,
				winning_outcome: None,
				tally: vec![],
				data: VoteData {
					initiator: _sender.clone(),
					stage: VoteStage::PreVoting,
					vote_type: vote_type,
					tally_type: tally_type,
					is_commit_reveal: is_commit_reveal,
				},
			});

			<VoteRecordCount<T>>::mutate(|i| *i += 1);
			Self::deposit_event(RawEvent::VoteCreated(id, _sender, vote_type));
			Ok(())
		}

		pub fn commit(origin, vote_id: u64, commit: [u8; 32]) -> Result {
			let _sender = ensure_signed(origin)?;
			let mut record = <VoteRecords<T>>::get(vote_id).ok_or("Vote record does not exist")?;
			ensure!(record.data.is_commit_reveal, "Commitments are not configured for this vote");
			ensure!(record.data.stage == VoteStage::Commit, "Vote is not in commit stage");
			// TODO: Allow changing of commits before commit stage ends
			ensure!(!record.commitments.iter().any(|c| &c.0 == &_sender), "Duplicate commits are not allowed");

			// Add commitment to record
			record.commitments.push((_sender.clone(), commit));
			<VoteRecords<T>>::insert(record.id, record);
			Ok(())
		}

		pub fn reveal(origin, vote_id: u64, vote: [u8; 32], secret: Option<[u8; 32]>) -> Result {
			let _sender = ensure_signed(origin)?;
			let mut record = <VoteRecords<T>>::get(vote_id).ok_or("Vote record does not exist")?;
			ensure!(record.data.stage == VoteStage::Voting, "Vote is not in voting stage");
			// Check vote is for a valid outcome
			ensure!(record.outcomes.iter().any(|o| o == &vote), "Vote type must be binary");
			// TODO: Allow changing of votes
			ensure!(!record.reveals.iter().any(|c| &c.0 == &_sender), "Duplicate votes are not allowed");

			// Ensure voter committed
			if record.data.is_commit_reveal {
				ensure!(record.commitments.iter().any(|c| &c.0 == &_sender), "Duplicate commits are not allowed");
				let commit: (T::AccountId, [u8; 32]) = record.commitments
					.iter()
					.find(|c| &c.0 == &_sender)
					.unwrap()
					.clone();

				let mut buf = Vec::new();
				buf.extend_from_slice(&_sender.encode());
				buf.extend_from_slice(&secret.unwrap().encode());
				buf.extend_from_slice(&vote);
				let hash = T::Hashing::hash_of(&buf);
				ensure!(hash.encode() == commit.1.encode(), "Commitments do not match");
			}

			record.reveals.push((_sender.clone(), vote));
			<VoteRecords<T>>::insert(record.id, record);
			Ok(())
		}

		pub fn advance_stage_as_initiator(origin, vote_id: u64) -> Result {
			let _sender = ensure_signed(origin)?;
			let record = <VoteRecords<T>>::get(vote_id).ok_or("Vote record does not exist")?;
			ensure!(record.data.initiator == _sender, "Invalid advance attempt by non-owner");
			return Self::advance_stage(vote_id);
		}

		pub fn tally_as_initiator(origin, vote_id: u64) -> Result {
			let _sender = ensure_signed(origin)?;
			let record = <VoteRecords<T>>::get(vote_id).ok_or("Vote record does not exist")?;
			ensure!(record.data.initiator == _sender, "Invalid advance attempt by non-owner");
			ensure!(record.data.stage == VoteStage::Completed, "Vote is not in completed stage");

			if let Some(tally) = Self::tally(vote_id) {
				<VoteRecords<T>>::insert(record.id, VoteRecord {
					tally: tally,
					..record
				});
			}
			Ok(())
		}
	}
}


impl<T: Trait> Module<T> {
	pub fn advance_stage(vote_id: u64) -> Result {
		let mut record = <VoteRecords<T>>::get(vote_id).ok_or("Vote record does not exist")?;
		let curr_stage = record.data.stage;
		let next_stage = match curr_stage {
			VoteStage::PreVoting if record.data.is_commit_reveal => VoteStage::Commit,
			VoteStage::PreVoting | VoteStage::Commit => VoteStage::Voting,
			VoteStage::Voting => VoteStage::Completed,
			VoteStage::Completed => return Err("Vote already completed"),
		};
		record.data.stage = next_stage;
		<VoteRecords<T>>::insert(record.id, record);
		Self::deposit_event(RawEvent::VoteAdvanced(vote_id, curr_stage, next_stage));
		Ok(())
	}

	// for a given account, finds the voter representing them, aka their
	// closest voting ancestor on the delegation graph (incl self)
	fn find_rep(voters: &HashMap<T::Hash, [u8; 32]>, acct: T::AccountId) -> Option<T::AccountId> {
		if voters.contains_key(&T::Hashing::hash_of(&acct.encode())) {
			return Some(acct);
		} else if let Some(parent) = <delegation::Module<T>>::delegate_of(acct) {
			return Self::find_rep(voters, parent);
		} else {
			return None;
		}
	}

	// constructs a mapping of accounts to their representatives
	fn build_rep_map(reps: &mut HashMap<T::Hash, (T::AccountId, T::AccountId)>, voters: &HashMap<T::Hash, [u8; 32]>, acct: T::AccountId) {
		// if we haven't seen this account yet, find its voting parent
		let hash = T::Hashing::hash_of(&acct.encode());
		if reps.contains_key(&hash) {
			return;
		} else {
			if let Some(voter) = Self::find_rep(voters, acct.clone()) {
				reps.insert(hash, (acct.clone(), voter));
			}
		}

		// recurse to children
		if let Some(ds) = <delegation::Module<T>>::delegates_to(acct) {
			ds.into_iter().for_each(|d| {
				Self::build_rep_map(reps, voters, d);
			});
		};
	}
	
	pub fn tally(vote_id: u64) -> Option<Vec<([u8; 32], T::Balance)>> {
		let mut voters: HashMap<T::Hash, [u8; 32]> = HashMap::new();
		let mut reps: HashMap<T::Hash, (T::AccountId, T::AccountId)> = HashMap::new();

		if let Some(record) = <VoteRecords<T>>::get(vote_id) {
			// build mapping of voters to their votes
			record.reveals.clone().into_iter().for_each(|(acct, choice)| {
				// build a mapping of voters to their votes
				voters.insert(T::Hashing::hash_of(&acct.encode()), choice);
			});

			// populate the map
			record.reveals.clone().into_iter()
				.for_each(|r| Self::build_rep_map(&mut reps, &voters, r.0));

			// tally up the vote
			let mut outcomes: Vec<([u8; 32], T::Balance)> = record.outcomes
				.clone()
				.into_iter()
				.map(|o| (o, Zero::zero()))
				.collect();

			for (_, (account, rep)) in reps.iter() {
				let weight: T::Balance = match record.data.tally_type {
					TallyType::OnePerson => One::one(),
					TallyType::OneCoin => <balances::Module<T>>::total_balance(account),
				};

				// use the representative's choice and the voter's weight
				let selection = voters.get(&T::Hashing::hash_of(&rep.encode())).unwrap();
				let index: usize = outcomes.iter().position(|&o| &o.0 == selection).unwrap();
				outcomes[index].1 = outcomes[index].1.checked_add(&weight).unwrap();
			}

			return Some(outcomes);
		}

		return None;
	}
}

/// An event in this module.
decl_event!(
	pub enum Event<T> where <T as system::Trait>::AccountId {
		/// new vote (id, creator, type of vote)
		VoteCreated(u64, AccountId, VoteType),
		/// vote stage transition (id, old stage, new stage)
		VoteAdvanced(u64, VoteStage, VoteStage),
	}
);

decl_storage! {
	trait Store for Module<T: Trait> as Delegation {
		/// The map of all vote records indexed by id
		pub VoteRecords get(vote_records): map u64 => Option<VoteRecord<T::AccountId, T::Balance>>;
		/// The number of vote records that have been created
		pub VoteRecordCount get(vote_record_count): u64;
	}
}
