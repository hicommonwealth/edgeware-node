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

#![recursion_limit="128"]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod default_weight;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
pub mod benchmarking;

use frame_support::traits::{Currency, Get, ReservableCurrency,};
use sp_std::prelude::*;

use frame_system::{ensure_signed};
use frame_support::{dispatch::DispatchResult, weights::Weight};
use codec::{Decode, Encode};

use sp_runtime::RuntimeDebug;
use sp_runtime::traits::{Hash};
use frame_support::{decl_event, decl_module, decl_storage, decl_error, ensure, StorageMap};

pub use voting::{VoteType, VoteOutcome, VoteStage, TallyType, VotingScheme};

#[derive(Encode, Decode, Clone, Eq, PartialEq, Ord, PartialOrd, RuntimeDebug)]
pub struct ProposalRecord<AccountId, Moment> {
	pub index: u32,
	pub author: AccountId,
	pub stage: VoteStage,
	pub transition_time: Moment,
	pub title: Vec<u8>,
	pub contents: Vec<u8>,
	pub vote_id: u64,
}

pub type ProposalTitle = Vec<u8>;
pub type ProposalContents = Vec<u8>;
type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

pub trait WeightInfo {
	fn create_proposal(p: u32, b: u32, ) -> Weight;
	fn advance_proposal(p: u32, ) -> Weight;
}

pub trait Config: voting::Config + pallet_balances::Config {
	/// The overarching event type
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;

	/// The account balance.
	type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;

	/// Maxmimum number of proposals allowed on chain.
	type MaxSignalingProposals: Get<u32>;

	/// Maxmimum length of title in bytes.
	type MaxTitleLength: Get<u32>;

	/// Maxmimum length of contents in bytes.
	type MaxContentsLength: Get<u32>;

	/// Weight information for extrinsics in this pallet.
	type WeightInfo: WeightInfo;
}

decl_storage! {
	trait Store for Module<T: Config> as Signaling {
		/// The total number of proposals created thus far.
		pub ProposalCount get(fn proposal_count) : u32;
		/// A list of all extant proposals.
		pub InactiveProposals get(fn inactive_proposals): Vec<(T::Hash, T::BlockNumber)>;
		/// A list of active proposals along with the time at which they complete.
		pub ActiveProposals get(fn active_proposals): Vec<(T::Hash, T::BlockNumber)>;
		/// A list of completed proposals, pending deletion
		pub CompletedProposals get(fn completed_proposals): Vec<(T::Hash, T::BlockNumber)>;
		/// Amount of time a proposal remains in "Voting" stage.
		pub VotingLength get(fn voting_length) config(): T::BlockNumber;
		/// Map for retrieving the information about any proposal from its hash.
		pub ProposalOf get(fn proposal_of): map hasher(twox_64_concat) T::Hash => Option<ProposalRecord<T::AccountId, T::BlockNumber>>;
		/// Registration bond
		pub ProposalCreationBond get(fn proposal_creation_bond) config(): BalanceOf<T>;
	}
}

decl_event!(
	pub enum Event<T> where <T as frame_system::Config>::Hash,
							<T as frame_system::Config>::AccountId,
							<T as frame_system::Config>::BlockNumber {
		/// Emitted at proposal creation: (Creator, ProposalHash)
		NewProposal(AccountId, Hash),
		/// Emitted when commit stage begins: (ProposalHash, VoteId, CommitEndTime)
		CommitStarted(Hash, u64, BlockNumber),
		/// Emitted when voting begins: (ProposalHash, VoteId, VotingEndTime)
		VotingStarted(Hash, u64, BlockNumber),
		/// Emitted when voting is completed: (ProposalHash, VoteId, VoteResults)
		VotingCompleted(Hash, u64),
	}
);

decl_error! {
	pub enum Error for Module<T: Config> {
		/// Corresponding voting for signaling proposal not found
		VoteRecordDoesntExist,
		/// Empty title of proposal
		EmptyProposalTitle,
		/// Empty contents of proposal
		EmptyProposalContents,
		/// Duplicate proposal
		DuplicateProposal,
		/// Not the proposal author
		NotAuthor,
		/// Proposal in wrong stage
		InvalidStage,
		/// Proposal does not exist with given hash
		ProposalMissing,
		/// Insufficient funds to reserve bond
		InsufficientFunds,
		/// Title is longer than max title length
		TitleTooLong,
		/// Contents are longer than max content length
		ContentsTooLong,
		/// Proposal limit reached
		TooManyProposals
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		/// Maxmimum number of proposals allowed on chain.
		const MaxSignalingProposals: u32 = T::MaxSignalingProposals::get();

		/// Maxmimum length of title in bytes.
		const MaxTitleLength: u32 = T::MaxTitleLength::get();

		/// Maxmimum length of contents in bytes.
		const MaxContentsLength: u32 = T::MaxContentsLength::get();

		type Error = Error<T>;
		fn deposit_event() = default;

		fn on_runtime_upgrade() -> Weight {
			migration::migrate::<T>();
			T::MaximumBlockWeight::get()
		}

		/// Creates a new signaling proposal.
		#[weight = <T as Config>::WeightInfo::create_proposal(T::MaxSignalingProposals::get(), T::MaxContentsLength::get())]
		pub fn create_proposal(
			origin,
			title: ProposalTitle,
			contents: ProposalContents,
			outcomes: Vec<VoteOutcome>,
			vote_type: voting::VoteType,
			tally_type: voting::TallyType,
			voting_scheme: voting::VotingScheme,
		) -> DispatchResult {
			let _sender = ensure_signed(origin)?;
			ensure!(!title.is_empty(), Error::<T>::EmptyProposalTitle);
			ensure!(!contents.is_empty(), Error::<T>::EmptyProposalContents);

			ensure!(title.len() < T::MaxTitleLength::get() as usize, Error::<T>::TitleTooLong);
			ensure!(contents.len() < T::MaxContentsLength::get() as usize, Error::<T>::ContentsTooLong);
			let extant_proposal_count =
				Self::inactive_proposals().len() + Self::active_proposals().len() + Self::completed_proposals().len();
			ensure!(extant_proposal_count < T::MaxSignalingProposals::get() as usize, Error::<T>::TooManyProposals);

			// construct hash(origin + proposal) and check existence
			let mut buf = Vec::new();
			buf.extend_from_slice(&_sender.encode());
			buf.extend_from_slice(&contents.as_ref());
			let hash = T::Hashing::hash(&buf[..]);
			ensure!(<ProposalOf<T>>::get(hash) == None, Error::<T>::DuplicateProposal);

			// Reserve the proposal creation bond amount
			T::Currency::reserve(&_sender, Self::proposal_creation_bond()).map_err(|_| Error::<T>::InsufficientFunds)?;
			// create a vote to go along with the proposal
			let vote_id = <voting::Module<T>>::create_vote(
				_sender.clone(),
				vote_type,
				voting_scheme,
				tally_type,
				outcomes,
			)?;

			let index = <ProposalCount>::get();
			let transition_time = <frame_system::Module<T>>::block_number() + Self::voting_length();
			<ProposalCount>::mutate(|i| *i += 1);
			<ProposalOf<T>>::insert(hash, ProposalRecord {
				index: index,
				author: _sender.clone(),
				stage: VoteStage::PreVoting,
				transition_time: transition_time,
				title: title,
				contents: contents,
				vote_id: vote_id,
			});
			<InactiveProposals<T>>::mutate(|proposals| proposals.push((hash, transition_time)));
			Self::deposit_event(RawEvent::NewProposal(_sender, hash));
			Ok(())
		}

		/// Advance a signaling proposal into the "voting" or "commit" stage.
		/// Can only be performed by the original author of the proposal.
		#[weight = <T as Config>::WeightInfo::advance_proposal(T::MaxSignalingProposals::get())]
		pub fn advance_proposal(origin, proposal_hash: T::Hash) -> DispatchResult {
			let _sender = ensure_signed(origin)?;
			let record = <ProposalOf<T>>::get(&proposal_hash).ok_or(Error::<T>::ProposalMissing)?;

			// only permit original author to advance
			ensure!(record.author == _sender, Error::<T>::NotAuthor);
			ensure!(record.stage == VoteStage::PreVoting
				|| record.stage == VoteStage::Commit, Error::<T>::InvalidStage);

			// prevoting -> voting or commit
			<voting::Module<T>>::advance_stage(record.vote_id)?;
			if let Some(vote_record) = <voting::Module<T>>::get_vote_record(record.vote_id) {
				let transition_time = <frame_system::Module<T>>::block_number() + Self::voting_length();
				let vote_id = record.vote_id;
				<ProposalOf<T>>::insert(proposal_hash, ProposalRecord {
					stage: vote_record.data.stage,
					transition_time: transition_time.clone(),
					..record
				});
				<InactiveProposals<T>>::mutate(|proposals| proposals.retain(|x| x.0 != proposal_hash));
				<ActiveProposals<T>>::mutate(|proposals| proposals.push((proposal_hash, transition_time.clone())));

				// emit event for voting if at this stage
				if vote_record.data.stage == VoteStage::Voting {
					Self::deposit_event(RawEvent::VotingStarted(proposal_hash, vote_id, transition_time));
				}

				// emit event for committing if at this stage
				if vote_record.data.stage == VoteStage::Commit {
					Self::deposit_event(RawEvent::CommitStarted(proposal_hash, vote_id, transition_time));
				}
				Ok(())
			} else {
				Err(Error::<T>::VoteRecordDoesntExist)?
			}
		}

		/// Check all active proposals to see if they're completed. If so, update
		/// them in storage and emit an event.
		fn on_finalize(_n: T::BlockNumber) {
			let (finished, mut active): (Vec<_>, _) = <ActiveProposals<T>>::get()
				.into_iter()
				.partition(|(_, exp)| _n > *exp);

			let (completed, mut pending): (Vec<_>, _) = <CompletedProposals<T>>::get()
				.into_iter()
				.partition(|(_, exp)| _n > *exp);

			let (doubly_inactive, still_inactive): (Vec<_>, _) = <InactiveProposals<T>>::get()
				.into_iter()
				.partition(|(_, exp)| _n > *exp);

			finished.into_iter().for_each(|(finished_hash, _)| {
				match <ProposalOf<T>>::get(finished_hash) {
					Some(record) => {
						let vote_id = record.vote_id;
						let _ = <voting::Module<T>>::advance_stage(vote_id);
						if let Some(vote_record) = <voting::Module<T>>::get_vote_record(record.vote_id) {
							// get next transition time
							let transition_time = <frame_system::Module<T>>::block_number() + Self::voting_length();
							// switch on either completed or voting stage from voting or committed stage
							if vote_record.data.stage == VoteStage::Completed {
								// unreserve the proposal creation bond amount
								T::Currency::unreserve(&record.author, Self::proposal_creation_bond());
								// add these completed proposals, to the pending "deletion" collection
								pending.push((finished_hash, transition_time.clone()));
								Self::deposit_event(RawEvent::VotingCompleted(finished_hash, vote_id));
							} else {
								// add the vote record identifier back into the collection
								active.push((finished_hash, transition_time));
								Self::deposit_event(RawEvent::VotingStarted(finished_hash, vote_id, transition_time));
							}
							// edit the proposal record to its respective stage
							<ProposalOf<T>>::insert(finished_hash.clone(), ProposalRecord {
								stage: vote_record.data.stage,
								transition_time: transition_time,
								..record
							});
						}
					},
					None => {}
				}
			});

			// delete all artifacts of "completed", completed proposals
			completed.iter().for_each(|(finished_hash, _)| {
				if let Some(record) = <ProposalOf<T>>::get(finished_hash) {
					<voting::Module<T>>::delete_vote_record(record.vote_id);
				}
				<ProposalOf<T>>::remove(finished_hash);
			});

			// we want to delete the doubly inactive, inactive proposals which never
			// proceeded into a commit or voting stage, always remaining in prevoting,
			// while also returning the bond.
			doubly_inactive.into_iter().for_each(|(hash, _)| {
				if let Some(record) = <ProposalOf<T>>::get(hash) {
					T::Currency::unreserve(&record.author, Self::proposal_creation_bond());
					<voting::Module<T>>::delete_vote_record(record.vote_id);
				}
				<ProposalOf<T>>::remove(hash);
			});

			// put back all active proposals
			<ActiveProposals<T>>::put(active);
			// put back only pending "to-be-deleted", completed proposals
			<CompletedProposals<T>>::put(pending);
			// put back singly, still_inactive, inactive proposals
			<InactiveProposals<T>>::put(still_inactive);
		}
	}
}

mod migration {
	use super::*;

	pub fn migrate<T: Config>() {
		for (hash, _n) in InactiveProposals::<T>::get() {
			ProposalOf::<T>::migrate_key_from_blake(hash);
		}
		for (hash, _n) in ActiveProposals::<T>::get() {
			ProposalOf::<T>::migrate_key_from_blake(hash);
		}
		for (hash, _n) in CompletedProposals::<T>::get() {
			ProposalOf::<T>::migrate_key_from_blake(hash);
		}
	}
}