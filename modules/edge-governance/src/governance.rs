
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
extern crate edge_voting as voting;

use rstd::prelude::*;
use srml_support::traits::{Currency, ReservableCurrency};
use system::ensure_signed;
use runtime_support::{StorageValue, StorageMap};
use runtime_support::dispatch::Result;
use runtime_primitives::traits::{Zero, Hash};
use codec::Encode;

pub use voting::voting::{VoteType, VoteOutcome, TallyType};

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
}

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, PartialEq)]
pub struct ProposalRecord<AccountId, Moment> {
	pub index: u32,
	pub author: AccountId,
	pub stage: ProposalStage,
	pub transition_time: Moment,
	pub category: ProposalCategory,
	pub title: Vec<u8>,
	pub contents: Vec<u8>,
	// TODO: for actions, we might need more data
	pub vote_id: u64,
}

pub trait Trait: voting::Trait + balances::Trait {
	/// The overarching event type
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
	/// The account balance.
	type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
}

pub type ProposalTitle = Vec<u8>;
pub type ProposalContents = Vec<u8>;
pub static YES_VOTE: voting::voting::VoteOutcome = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1];
pub static NO_VOTE: voting::voting::VoteOutcome = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event<T>() = default;

		/// Creates a new governance proposal in the chosen category.
		pub fn create_proposal(
			origin,
			title: ProposalTitle,
			contents: ProposalContents,
			category: ProposalCategory,
			outcomes: Vec<VoteOutcome>,
			vote_type: voting::VoteType,
			tally_type: voting::TallyType
		) -> Result {
			let _sender = ensure_signed(origin)?;
			ensure!(!title.is_empty(), "Proposal must have title");
			ensure!(!contents.is_empty(), "Proposal must not be empty");

			// construct hash(origin + proposal) and check existence
			// TODO: include title/category/etc?
			let mut buf = Vec::new();
			buf.extend_from_slice(&_sender.encode());
			buf.extend_from_slice(&contents.as_ref());
			let hash = T::Hashing::hash(&buf[..]);
			ensure!(<ProposalOf<T>>::get(hash) == None, "Proposal already exists");

			// Reserve the proposal creation bond amount
			T::Currency::reserve(&_sender, Self::proposal_creation_bond()).map_err(|_| "Not enough currency for reserve bond")?;
			// create a vote to go along with the proposal
			let vote_id = <voting::Module<T>>::create_vote(
				_sender.clone(),
				vote_type,
				false, // not commit-reveal
				tally_type,
				outcomes,
			)?;

			let index = <ProposalCount<T>>::get();
			<ProposalCount<T>>::mutate(|i| *i += 1);
			<ProposalOf<T>>::insert(hash, ProposalRecord {
				index: index,
				author: _sender.clone(),
				stage: ProposalStage::PreVoting,
				category: category,
				transition_time: T::BlockNumber::zero(),
				title: title,
				contents: contents,
				vote_id: vote_id,
			});
			<Proposals<T>>::mutate(|proposals| proposals.push(hash));
			Self::deposit_event(RawEvent::NewProposal(_sender, hash));
			Ok(())
		}

		/// Advance a governance proposal into the "voting" stage. Can only be
		/// performed by the original author of the proposal.
		pub fn advance_proposal(origin, proposal_hash: T::Hash) -> Result {
			let _sender = ensure_signed(origin)?;
			let record = <ProposalOf<T>>::get(&proposal_hash).ok_or("Proposal does not exist")?;

			// only permit original author to advance
			ensure!(record.author == _sender, "Proposal must be advanced by author");
			ensure!(record.stage == ProposalStage::PreVoting, "Proposal not in pre-voting stage");
			
			// prevoting -> voting
			<voting::Module<T>>::advance_stage(record.vote_id)?;
			let transition_time = <system::Module<T>>::block_number() + Self::voting_length();
			let vote_id = record.vote_id;
			<ProposalOf<T>>::insert(proposal_hash, ProposalRecord {
				stage: ProposalStage::Voting,
				transition_time: transition_time.clone(),
				..record
			});
			<ActiveProposals<T>>::mutate(|proposals| proposals.push((proposal_hash, transition_time.clone())));
			Self::deposit_event(RawEvent::VotingStarted(proposal_hash, vote_id, transition_time));
			Ok(())
		}

		/// Check all active proposals to see if they're completed. If so, update
		/// them in storage and emit an event.
		fn on_finalize(_n: T::BlockNumber) {
			let (finished, active): (Vec<_>, _) = <ActiveProposals<T>>::get()
				.into_iter()
				.partition(|(_, exp)| _n > *exp);
			
			<ActiveProposals<T>>::put(active);
			finished.into_iter().for_each(move |(completed_hash, _)| {
				match <ProposalOf<T>>::get(completed_hash) {
					Some(record) => {
						// voting -> completed
						let vote_id = record.vote_id;
						// TODO: handle possible errors from advance_stage?
						let _ = <voting::Module<T>>::advance_stage(vote_id);
						// Unreserve the proposal creation bond amount
						T::Currency::unreserve(&record.author, Self::proposal_creation_bond());
						// Edit the proposal record to completed
						<ProposalOf<T>>::insert(completed_hash, ProposalRecord {
							stage: ProposalStage::Completed,
							transition_time: T::BlockNumber::zero(),
							..record
						});
						Self::deposit_event(RawEvent::VotingCompleted(completed_hash, vote_id));
					},
					None => { } // TODO: emit an error here?
				}
			});
		}
	}
}

decl_event!(
	pub enum Event<T> where <T as system::Trait>::Hash,
							<T as system::Trait>::AccountId,
							<T as system::Trait>::BlockNumber {
		/// Emitted at proposal creation: (Creator, ProposalHash)
		NewProposal(AccountId, Hash),
		/// Emitted when voting begins: (ProposalHash, VoteId, VotingEndTime)
		VotingStarted(Hash, u64, BlockNumber),
		/// Emitted when voting is completed: (ProposalHash, VoteId, VoteResults)
		VotingCompleted(Hash, u64),
	}
);

decl_storage! {
	trait Store for Module<T: Trait> as Governance {
		/// The total number of proposals created thus far.
		pub ProposalCount get(proposal_count) : u32;
		/// A list of all extant proposals.
		pub Proposals get(proposals): Vec<T::Hash>;
		/// A list of active proposals along with the time at which they complete.
		pub ActiveProposals get(active_proposals): Vec<(T::Hash, T::BlockNumber)>;
		/// Amount of time a proposal remains in "Voting" stage.
		pub VotingLength get(voting_length) config(): T::BlockNumber;
		/// Map for retrieving the information about any proposal from its hash. 
		pub ProposalOf get(proposal_of): map T::Hash => Option<ProposalRecord<T::AccountId, T::BlockNumber>>;
		/// Registration bond
		pub ProposalCreationBond get(proposal_creation_bond) config(): BalanceOf<T>;
	}
}
