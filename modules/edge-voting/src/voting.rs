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
#[derive(Encode, Decode, PartialEq)]
pub struct VoteRecord<AccountId, BlockNumber> {
	// Identifier of the vote
	pub id: u64,
	// creator of vote
	pub initiator: AccountId,
	// participating voters (still preserves anonymity if this is anonymity set)
	pub voters: Vec<AccountId>,
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
}

pub trait Trait: balances::Trait {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		pub fn create_vote(origin, vote_type: VoteType, length_in_blocks: T::BlockNumber) -> Result {
			let _sender = ensure_signed(origin)?;
			let id = Self::vote_record_count() + 1;
			
			<VoteRecords<T>>::insert(id, VoteRecord {
				id: id,
				initiator: _sender,
				voters: vec![],
				stage: VoteStage::PreVoting,
				vote_type: vote_type,
				creation_time: <
			});
		}
	}
}

impl<T: Trait> Module<T> {}

/// An event in this module.
decl_event!(
	pub enum Event<T> where <T as system::Trait>::AccountId, <T as system::Trait>::BlockNumber {
		VoteCreated(AccountId, BlockNumber),
	}
);

decl_storage! {
	trait Store for Module<T: Trait> as Delegation {
		/// The map of all vote records indexed by id
		pub VoteRecords get(vote_records): map u64 => VoteRecord<T::AccountId, T::BlockNumber>>;
		/// The number of vote records that have been created
		pub VoteRecordCount get(vote_record_count): u64
	}
}
