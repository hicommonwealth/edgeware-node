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

use rstd::prelude::*;
use system::ensure_signed;
use runtime_support::{StorageValue, StorageMap, Parameter};
use runtime_support::dispatch::Result;
use runtime_primitives::traits::Hash;
use codec::Encode;

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, Copy, Clone, Eq, PartialEq)]
pub enum VoteStage {
	// Before voting stage
	// TODO: Define or remove
	PreVoting,
	// Active voting stage, votes allowed
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
pub struct VoteData<AccountId, BlockNumber> {
	// creator of vote
	pub initiator: AccountId,
	// Stage of the vote
	pub stage: VoteStage,
	// Type of vote defined abovoe
	pub vote_type: VoteType,
	// Creation time of the record
	pub creation_time: BlockNumber,
	// Initialization time of the voting stage
	pub initialization_time: BlockNumber,
	// Expiration time of the voting stage
	pub expiration_time: BlockNumber,
	// Tally metric
	pub tally_type: TallyType,
	// Vote outcomes
}

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, PartialEq)]
pub struct VoteRecord<AccountId, BlockNumber> {
	// Identifier of the vote
	pub id: u64,
	// Flag for commit/reveal voting scheme
	pub is_commit_reveal: bool,
	// Vote commitments
	pub commitments: Vec<(AccountId, [u8; 32])>,
	// Vote reveals with 2^32 possible options
	pub reveals: Vec<(AccountId, [u8; 32])>,
	// Vote data record
	pub data: VoteData<AccountId, BlockNumber>,
	// Vote outcomes
	pub outcomes: Vec<[u8; 32]>,
}

pub trait Trait: balances::Trait {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		pub fn create_vote(
			origin,
			vote_type: VoteType,
			initialization_time: T::BlockNumber,
			expiration_time: T::BlockNumber,
			is_commit_reveal: bool,
			tally_type: TallyType,
			outcomes: Vec<[u8; 32]>
		) -> Result {
			let _sender = ensure_signed(origin)?;
			ensure!(vote_type == VoteType::Binary || vote_type == VoteType::MultiOption, "Unsupported vote type");

			let id = Self::vote_record_count() + 1;
			let create_time = <system::Module<T>>::block_number();
			let init_time = <system::Module<T>>::block_number() + initialization_time;
			let expire_time = <system::Module<T>>::block_number() + initialization_time + expiration_time;
			<VoteRecords<T>>::insert(id, VoteRecord {
				id: id,
				is_commit_reveal: is_commit_reveal,
				commitments: vec![],
				reveals: vec![],
				outcomes: outcomes,
				data: VoteData {
					initiator: _sender.clone(),
					stage: VoteStage::PreVoting,
					vote_type: vote_type,
					creation_time: create_time,
					initialization_time: init_time,
					expiration_time: expire_time,
					tally_type: tally_type,
				},
			});

			<VoteRecordCount<T>>::mutate(|i| *i += 1);
			Self::deposit_event(RawEvent::VoteCreated(_sender, vote_type, init_time, expire_time));
			Ok(())
		}

		pub fn commit(origin, vote_id: u64, commit: [u8; 32]) -> Result {
			let _sender = ensure_signed(origin)?;
			ensure!(<VoteRecords<T>>::exists(vote_id), "Vote record does not exist");

			match <VoteRecords<T>>::get(vote_id) {
				Some(mut record) => {
					// Ensure record is a commit reveal record
					ensure!(record.is_commit_reveal, "Commitments are not configured for this vote");
					// Prevent duplicate commitments
					ensure!(!record.commitments.clone().into_iter().any(|c| c.0 == _sender.clone()), "Duplicate votes are not allowed");
					// Add commitment to record
					record.commitments.push((_sender.clone(), commit));
					<VoteRecords<T>>::insert(record.id, record);
				},
				None => { return Err("Vote record does not exist") },
			}
			Ok(())
		}

		pub fn reveal(origin, vote_id: u64, vote: [u8; 32], secret: Option<[u8; 32]>) -> Result {
			let _sender = ensure_signed(origin)?;
			// Check vote record exists
			ensure!(<VoteRecords<T>>::exists(vote_id), "Vote record does not exist");
			// Check vote is for a valid outcome
			ensure!(<VoteRecords<T>>::get(vote_id).unwrap().outcomes.iter().any(|o| o == &vote), "Vote type must be binary");
			
			match <VoteRecords<T>>::get(vote_id) {
				Some(mut record) => {
					// Prevent duplicate reveals
					ensure!(!record.reveals.clone().into_iter().any(|c| c.0 == _sender.clone()), "Duplicate votes are not allowed");
					// Ensure voter committed
					if record.is_commit_reveal {
						ensure!(record.commitments.clone().into_iter().any(|c| c.0 == _sender.clone()), "Duplicate commits are not allowed");
						let commit: (T::AccountId, [u8; 32]) = record.commitments.clone()
							.into_iter()
							.find(|c| c.0 == _sender.clone())
							.unwrap();

						let mut buf = Vec::new();
						buf.extend_from_slice(&_sender.encode());
						buf.extend_from_slice(&secret.unwrap().encode());
						buf.extend_from_slice(&vote);
						let hash = T::Hashing::hash_of(&buf);
						ensure!(hash.encode() == commit.1.encode(), "Commitments do not match");
					}

					record.reveals.push((_sender.clone(), vote));
					<VoteRecords<T>>::insert(record.id, record);
				},
				None => { return Err("Vote record does not exist") },
			}

			Ok(())
		}
	}
}

impl<T: Trait> Module<T> {}

/// An event in this module.
decl_event!(
	pub enum Event<T> where <T as system::Trait>::AccountId, <T as system::Trait>::BlockNumber {
		VoteCreated(AccountId, VoteType, BlockNumber, BlockNumber),
	}
);

decl_storage! {
	trait Store for Module<T: Trait> as Delegation {
		/// The map of all vote records indexed by id
		pub VoteRecords get(vote_records): map u64 => Option<VoteRecord<T::AccountId, T::BlockNumber>>;
		/// The number of vote records that have been created
		pub VoteRecordCount get(vote_record_count): u64;
	}
}
