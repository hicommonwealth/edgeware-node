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
extern crate srml_system as system;

use rstd::prelude::*;
use system::ensure_signed;
use runtime_support::{StorageValue, StorageMap, Parameter};
use runtime_support::dispatch::Result;
use runtime_primitives::traits::Hash;
use codec::Encode;

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, PartialEq, Clone, Copy)]
pub enum ProposalStage {
    PreVoting,
    Voting,
    Completed,
}

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, PartialEq, Clone, Copy)]
pub enum ProposalCategory {
    Signaling,
    Funding(u32), // TODO: convert this into a Balance
    Upgrade,
}

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, PartialEq)]
pub struct ProposalRecord<AccountId> {
    pub index: u32,
    pub author: AccountId,
    pub stage: ProposalStage,
    pub category: ProposalCategory,
    pub title: Vec<u8>,
    pub contents: Vec<u8>,
    // TODO: separate comments into different object, for storage reasons
    pub comments: Vec<(Vec<u8>, AccountId)>,
}

pub trait Trait: system::Trait {
    /// The overarching event type
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        pub fn create_proposal(origin, title: Vec<u8>, contents: Vec<u8>, category: ProposalCategory) -> Result {
            let _sender = ensure_signed(origin)?;
            ensure!(!title.is_empty(), "Proposal must have title");
            ensure!(!contents.is_empty(), "Proposal must not be empty");

            // construct hash(origin + proposal) and check existence
            // TODO: include title/category/etc?
            let mut buf = Vec::new();
            buf.extend_from_slice(&_sender.encode());
            buf.extend_from_slice(&contents.as_ref());
            let hash = T::Hashing::hash(&buf[..]);
            ensure!(<ProposalOf<T>>::get(&hash) == None, "Proposal already exists");

            // construct proposal
            let index = <ProposalCount<T>>::get();
            <ProposalCount<T>>::mutate(|i| *i += 1);
            let record = ProposalRecord { index: index,
                                          author: _sender.clone(),
                                          stage: ProposalStage::PreVoting,
                                          category: category,
                                          title: title,
                                          contents: contents,
                                          comments: vec![] };

            // add new record to storage
            <ProposalOf<T>>::insert(&hash, record);
            let mut proposals = Self::proposals();
            proposals.push(hash.clone());
            <Proposals<T>>::put(proposals);
            Self::deposit_event(RawEvent::NewProposal(_sender, hash));
            Ok(())
        }

        // TODO: give comments unique numbers/ids?
        pub fn add_comment(origin, proposal_hash: T::Hash, comment: Vec<u8>) -> Result {
            let _sender = ensure_signed(origin)?;
            // TODO: can we mut borrow somehow?
            let record = <ProposalOf<T>>::get(&proposal_hash).ok_or("Proposal does not exist")?;
            let mut new_record = record;
            new_record.comments.push((comment, _sender.clone()));
            <ProposalOf<T>>::insert(&proposal_hash, new_record);
            Self::deposit_event(RawEvent::NewComment(_sender, proposal_hash));
            Ok(())
        }

        pub fn advance_proposal(origin, proposal_hash: T::Hash) -> Result {
            let _sender = ensure_signed(origin)?;
            let record = <ProposalOf<T>>::get(&proposal_hash).ok_or("Proposal does not exist")?;

            // only permit original author to advance
            ensure!(record.author == _sender, "Proposal must be advanced by author");
            let next_stage = match record.stage {
                ProposalStage::PreVoting => ProposalStage::Voting,
                ProposalStage::Voting    => ProposalStage::Completed,
                ProposalStage::Completed => { return Err("Proposal already completed") },
            };
            let mut new_record = record;
            new_record.stage = next_stage;
            <ProposalOf<T>>::insert(&proposal_hash, new_record);
            if next_stage == ProposalStage::Voting {
                Self::deposit_event(RawEvent::VotingStarted(proposal_hash));
            } else if next_stage == ProposalStage::Completed {
                Self::deposit_event(RawEvent::VotingCompleted(proposal_hash));
            }
            Ok(())
        }

        pub fn submit_vote(origin, proposal_hash: T::Hash, vote: bool) -> Result {
            let _sender = ensure_signed(origin)?;
            let record = <ProposalOf<T>>::get(&proposal_hash).ok_or("Proposal does not exist")?;
            ensure!(record.stage == ProposalStage::Voting, "Proposal not in voting stage");

            // TODO: This does not allow updating one's vote; should we support this?
            if Self::vote_of((proposal_hash, _sender.clone())).is_none() {
                <ProposalVoters<T>>::mutate(proposal_hash, |voters| voters.push(_sender.clone()));
            }
            <VoteOf<T>>::insert((proposal_hash.clone(), _sender.clone()), vote);
            Self::deposit_event(RawEvent::VoteSubmitted(proposal_hash, _sender, vote));
            Ok(())
        }
    }
}

decl_event!(
    pub enum Event<T> where <T as system::Trait>::Hash,
                            <T as system::Trait>::AccountId {
        NewProposal(AccountId, Hash),
        NewComment(AccountId, Hash),
        VotingStarted(Hash),
        VoteSubmitted(Hash, AccountId, bool),
        VotingCompleted(Hash),
    }
);

decl_storage! {
    trait Store for Module<T: Trait> as Governance {
        pub ProposalCount get(proposal_count) : u32;
        pub Proposals get(proposals): Vec<T::Hash>;
        pub ProposalOf get(proposal_of): map T::Hash => Option<ProposalRecord<T::AccountId>>;
        pub ProposalVoters get(proposal_voters): map T::Hash => Vec<T::AccountId>;
        pub VoteOf get(vote_of): map (T::Hash, T::AccountId) => Option<bool>;
    }
}
