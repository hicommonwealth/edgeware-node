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

#![recursion_limit="128"]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod tests;

use sp_std::prelude::*;
use sp_std::result;
use frame_system::{self as system, ensure_signed};
use frame_support::{dispatch::DispatchResult, traits::Get, weights::Weight};
use codec::{Decode, Encode};

use sp_runtime::RuntimeDebug;
use sp_runtime::traits::{
	Hash
};

use frame_support::{decl_event, decl_module, decl_storage, decl_error, ensure, StorageMap};

/// A potential outcome of a vote, with 2^32 possible options
pub type VoteOutcome = [u8; 32];

#[derive(Encode, Decode, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, RuntimeDebug)]
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

#[derive(Encode, Decode, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, RuntimeDebug)]
pub enum VoteType {
	// Binary decision vote, i.e. 2 outcomes
	Binary,
	// Multi option decision vote, i.e. > 2 possible outcomes
	MultiOption,
	// Ranked choice voting
	RankedChoice,
}

#[derive(Encode, Decode, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, RuntimeDebug)]
pub enum TallyType {
	// 1 person 1 vote, i.e. 1 account 1 vote
	OnePerson,
	// 1 coin 1 vote, i.e. by balances
	OneCoin,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, Ord, PartialOrd, RuntimeDebug)]
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

#[derive(Encode, Decode, Clone, Eq, PartialEq, Ord, PartialOrd, RuntimeDebug)]
pub struct VoteRecord<AccountId> {
	// Identifier of the vote
	pub id: u64,
	// Vote commitments
	pub commitments: Vec<(AccountId, VoteOutcome)>,
	// Vote reveals
	pub reveals: Vec<(AccountId, Vec<VoteOutcome>)>,
	// Vote data record
	pub data: VoteData<AccountId>,
	// Vote outcomes
	pub outcomes: Vec<VoteOutcome>,
}

pub trait Trait: frame_system::Trait {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

decl_error! {
    pub enum Error for Module<T: Trait> {
        VoteCompleted
    }
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = Error<T>;
		fn deposit_event() = default;

		fn on_runtime_upgrade() -> Weight {
			migration::migrate::<T>();
			T::MaximumBlockWeight::get()
		}

		/// A function for commit-reveal voting schemes that adds a vote commitment.
		///
		/// A vote commitment is formatted using the native hash function. There
		/// are currently no cryptoeconomic punishments against not revealing the
		/// commitment.
		#[weight = 0]
		pub fn commit(origin, vote_id: u64, commit: VoteOutcome) -> DispatchResult {
			let _sender = ensure_signed(origin)?;
			let mut record = <VoteRecords<T>>::get(vote_id).ok_or("Vote record does not exist")?;
			ensure!(record.data.is_commit_reveal, "Commitments are not configured for this vote");
			ensure!(record.data.stage == VoteStage::Commit, "Vote is not in commit stage");
			// No changing of commitments once placed
			ensure!(!record.commitments.iter().any(|c| &c.0 == &_sender), "Duplicate commits are not allowed");

			// Add commitment to record
			record.commitments.push((_sender.clone(), commit));
			let id = record.id;
			<VoteRecords<T>>::insert(id, record);
			Self::deposit_event(RawEvent::VoteCommitted(id, _sender));
			Ok(())
		}

		/// A function that reveals a vote commitment or serves as the general vote function.
		///
		/// There are currently no cryptoeconomic incentives for revealing commited votes.
		#[weight = 0]
		pub fn reveal(origin, vote_id: u64, vote: Vec<VoteOutcome>, secret: Option<VoteOutcome>) -> DispatchResult {
			let _sender = ensure_signed(origin)?;
			let mut record = <VoteRecords<T>>::get(vote_id).ok_or("Vote record does not exist")?;
			ensure!(record.data.stage == VoteStage::Voting, "Vote is not in voting stage");
			// Check vote is for valid outcomes
			if record.data.vote_type == VoteType::RankedChoice {
				ensure!(Self::is_ranked_choice_vote_valid(
					vote.clone(),
					record.outcomes.clone()
				), "Ranked choice vote invalid");
			} else {
				ensure!(Self::is_valid_vote(
					vote.clone(),
					record.outcomes.clone()
				), "Vote outcome is not valid");
			}
			// Ensure ranked choice votes have same number of votes as outcomes
			if record.data.vote_type == VoteType::RankedChoice {
				ensure!(record.outcomes.len() == vote.len(), "Vote must rank all outcomes in order");
			}
			// Reject vote or reveal changes
			ensure!(!record.reveals.iter().any(|c| &c.0 == &_sender), "Duplicate votes are not allowed");
			// Ensure voter committed
			if record.data.is_commit_reveal {
				// Ensure secret is passed in
				ensure!(secret.is_some(), "Secret is invalid");
				// Ensure the current sender has already committed previously
				ensure!(record.commitments.iter().any(|c| &c.0 == &_sender), "Sender not yet committed");
				let commit: (T::AccountId, VoteOutcome) = record.commitments
					.iter()
					.find(|c| &c.0 == &_sender)
					.unwrap()
					.clone();
				// Create commitment hash using reported secret and ranked choice ordering
				let mut buf = Vec::new();
				buf.extend_from_slice(&_sender.encode());
				buf.extend_from_slice(&secret.unwrap().encode());
				for i in 0..vote.len() {
					buf.extend_from_slice(&vote[i]);
				}
				let hash = T::Hashing::hash_of(&buf);
				// Ensure the hashes match
				ensure!(hash.encode() == commit.1.encode(), "Commitments do not match");
			}
			// Record the revealed vote and emit an event
			let id = record.id;
			record.reveals.push((_sender.clone(), vote.clone()));
			<VoteRecords<T>>::insert(id, record);
			Self::deposit_event(RawEvent::VoteRevealed(id, _sender, vote));
			Ok(())
		}
	}
}

impl<T: Trait> Module<T> {
	/// A helper function for creating a new vote/ballot.
	pub fn create_vote(
		sender: T::AccountId,
		vote_type: VoteType,
		is_commit_reveal: bool,
		tally_type: TallyType,
		outcomes: Vec<VoteOutcome>
	) -> result::Result<u64, &'static str> {
		if vote_type == VoteType::Binary { ensure!(outcomes.len() == 2, "Invalid binary outcomes") }
		if vote_type == VoteType::MultiOption { ensure!(outcomes.len() > 2, "Invalid multi option outcomes") }
		if vote_type == VoteType::RankedChoice { ensure!(outcomes.len() > 2, "Invalid ranked choice outcomes") }

		let id = Self::vote_record_count() + 1;
		<VoteRecords<T>>::insert(id, VoteRecord {
			id: id,
			commitments: vec![],
			reveals: vec![],
			outcomes: outcomes,
			data: VoteData {
				initiator: sender.clone(),
				stage: VoteStage::PreVoting,
				vote_type: vote_type,
				tally_type: tally_type,
				is_commit_reveal: is_commit_reveal,
			},
		});

		<VoteRecordCount>::mutate(|i| *i += 1);
		Self::deposit_event(RawEvent::VoteCreated(id, sender, vote_type));
		return Ok(id);
	}

	/// A helper function for advancing the stage of a vote, as a state machine
	pub fn advance_stage(vote_id: u64) -> DispatchResult {
		let mut record = <VoteRecords<T>>::get(vote_id).ok_or("Vote record does not exist")?;
		let curr_stage = record.data.stage;
		let next_stage = match curr_stage {
			VoteStage::PreVoting if record.data.is_commit_reveal => VoteStage::Commit,
			VoteStage::PreVoting | VoteStage::Commit => VoteStage::Voting,
			VoteStage::Voting => VoteStage::Completed,
			VoteStage::Completed => return Err(Error::<T>::VoteCompleted)?,
		};
		record.data.stage = next_stage;
		<VoteRecords<T>>::insert(record.id, record);
		Self::deposit_event(RawEvent::VoteAdvanced(vote_id, curr_stage, next_stage));
		Ok(())
	}

	pub fn is_ranked_choice_vote_valid(mut vote: Vec<VoteOutcome>, mut outcomes: Vec<VoteOutcome>) -> bool {
		// check length equality
		if vote.len() == outcomes.len() {
			// sort both sets
			vote.sort();
			outcomes.sort();
			// check element wise equality
			for i in 0..vote.len() {
				if vote[i] == outcomes[i] { continue; }
				else { return false }
			}

			true
		} else {
			false
		}
	}

	pub fn is_valid_vote(vote: Vec<VoteOutcome>, outcomes: Vec<VoteOutcome>) -> bool {
		for i in 0..vote.len() {
			if outcomes.iter().any(|o| o == &vote[i]) {
				continue;
			} else {
				return false;
			}
		}

		true
	}

	pub fn get_vote_record(vote_id: u64) -> Option<VoteRecord<T::AccountId>> {
		return <VoteRecords<T>>::get(vote_id);
	}
}

decl_event!(
	pub enum Event<T> where <T as frame_system::Trait>::AccountId {
		/// new vote (id, creator, type of vote)
		VoteCreated(u64, AccountId, VoteType),
		/// vote stage transition (id, old stage, new stage)
		VoteAdvanced(u64, VoteStage, VoteStage),
		/// user commits
		VoteCommitted(u64, AccountId),
		/// user reveals a vote
		VoteRevealed(u64, AccountId, Vec<VoteOutcome>),
	}
);

decl_storage! {
	trait Store for Module<T: Trait> as Voting {
		/// The map of all vote records indexed by id
		pub VoteRecords get(fn vote_records): map hasher(twox_64_concat) u64 => Option<VoteRecord<T::AccountId>>;
		/// The number of vote records that have been created
		pub VoteRecordCount get(fn vote_record_count): u64;
	}
}

mod migration {
	use super::*;

	pub fn migrate<T: Trait>() {
		for idx in 0..(VoteRecordCount::get() + 1) {
			VoteRecords::<T>::migrate_key_from_blake(idx);
		}
	}
}
