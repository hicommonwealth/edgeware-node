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
// along with Edgeware.  If not, see <http://www.gnu.org/licenses/>

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate serde;

// Needed for deriving `Serialize` and `Deserialize` for various types.
// We only implement the serde traits for std builds - they're unneeded
// in the wasm runtime.
#[cfg(feature = "std")]
extern crate serde_derive;
#[cfg(test)]
extern crate hex_literal;
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
extern crate edge_voting as voting;

pub mod governance;
pub use governance::{
	Module, Trait, RawEvent, Event,
	ProposalStage, ProposalCategory, ProposalRecord
};

#[cfg(test)]
mod tests {
	use super::*;
	use rstd::prelude::*;
	use codec::Encode;
	use runtime_support::dispatch::Result;
	use system::{EventRecord, Phase};
	use runtime_io::with_externalities;
	use primitives::{H256, Blake2Hasher, Hasher};
	// The testing primitives are very useful for avoiding having to work with signatures
	// or public keys. `u64` is used as the `AccountId` and no `Signature`s are requried.
	use runtime_primitives::{
		BuildStorage,
		traits::{BlakeTwo256, OnFinalize, IdentityLookup},
		testing::{Header}
	};
	use voting::{VoteStage, VoteType};
	use voting::voting::{VoteOutcome, TallyType};

	impl_outer_origin! {
		pub enum Origin for Test {}
	}

	impl_outer_event! {
		pub enum Event for Test {
			voting<T>, balances<T>, governance<T>,
		}
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
		type AccountId = u64;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Header = Header;
		type Event = Event;
	}

	impl balances::Trait for Test {
		type Balance = u64;
		type OnFreeBalanceZero = ();
		type OnNewAccount = ();
		type Event = Event;
		type TransactionPayment = ();
		type TransferPayment = ();
		type DustRemoval = ();
	}

	impl voting::Trait for Test {
		type Event = Event;
	}

	impl Trait for Test {
		type Event = Event;
		type Currency = balances::Module<Self>;
	}

	pub type Balances = balances::Module<Test>;
	pub type System = system::Module<Test>;
	pub type Governance = Module<Test>;

	const BOND: u64 = 10;

	fn new_test_ext() -> sr_io::TestExternalities<Blake2Hasher> {
		let mut t = system::GenesisConfig::<Test>::default().build_storage().unwrap().0;
		// We use default for brevity, but you can configure as desired if needed.
		t.extend(
			governance::GenesisConfig::<Test> {
				voting_length: 10000,
				proposal_creation_bond: BOND,
			}.build_storage().unwrap().0,
		);
		t.extend(
			balances::GenesisConfig::<Test> {
				balances: vec![
					(1, 100),
					(2, 100),
					(3, 100),
					(4, 100),
				],
				transaction_base_fee: 0,
				transaction_byte_fee: 0,
				existential_deposit: 0,
				transfer_fee: 0,
				creation_fee: 0,
				vesting: vec![],
			}.build_storage().unwrap().0,
		);
		t.into()
	}

	fn propose(
		who: u64,
		title: &[u8],
		proposal: &[u8],
		category: governance::ProposalCategory,
		outcomes: Vec<VoteOutcome>,
		vote_type: VoteType,
		tally_type: TallyType
	) -> Result {
		Governance::create_proposal(
			Origin::signed(who),
			title.to_vec(),
			proposal.to_vec(),
			category,
			outcomes,
			vote_type,
			tally_type)
	}

	fn advance_proposal(who: u64, proposal_hash: H256) -> Result {
		Governance::advance_proposal(Origin::signed(who), proposal_hash)
	}

	fn build_proposal_hash(who: u64, proposal: &[u8]) -> H256 {
			let mut buf = Vec::new();
			buf.extend_from_slice(&who.encode());
			buf.extend_from_slice(proposal.as_ref());
			return Blake2Hasher::hash(&buf[..]);
	}

	fn get_test_key() -> u64 {
		let public = 1_u64;
		return public;
	}

	fn generate_proposal() -> (&'static[u8], &'static[u8]) {
		let title: &[u8] = b"Make Edgeware Free";
		let proposal: &[u8] = b"Simple: make Edgeware free for everyone";
		return (title, proposal);
	}

	fn make_record(
		author: u64,
		title: &[u8],
		contents: &[u8],
		category: ProposalCategory)
		-> ProposalRecord<u64, u64> {
			ProposalRecord {
				index: 0,
				author: author,
				stage: ProposalStage::PreVoting,
				category: category,
				transition_time: 0,
				title: title.to_vec(),
				contents: contents.to_vec(),
				vote_id: 1,
			}
	}

	#[test]
	fn propose_should_work() {
		with_externalities(&mut new_test_ext(), || {
			System::set_block_number(1);
			let public = get_test_key();
			let category = governance::ProposalCategory::Signaling;
			let (title, proposal) = generate_proposal();
			let hash = build_proposal_hash(public, &proposal);
			let outcomes = vec![governance::YES_VOTE, governance::NO_VOTE];
			assert_ok!(propose(public, title, proposal, category, outcomes, VoteType::Binary, TallyType::OneCoin));
			let vote_id = Governance::proposal_of(hash).unwrap().vote_id;
			assert_eq!(System::events(), vec![
				EventRecord {
					phase: Phase::ApplyExtrinsic(0),
					event: Event::voting(voting::RawEvent::VoteCreated(vote_id, public, VoteType::Binary)),
					topics: vec![],
				},
				EventRecord {
					phase: Phase::ApplyExtrinsic(0),
					event: Event::governance(RawEvent::NewProposal(public, hash)),
					topics: vec![],
				}]
			);

			let title2: &[u8] = b"Proposal 2";
			let proposal2: &[u8] = b"Proposal 2";
			let hash2 = build_proposal_hash(public, &proposal2);
			let outcomes = vec![governance::YES_VOTE, governance::NO_VOTE];
			assert_ok!(propose(public, title2, proposal2, category, outcomes, VoteType::Binary, TallyType::OneCoin));
			let vote_id2 = Governance::proposal_of(hash2).unwrap().vote_id;
			assert_eq!(System::events(), vec![
				EventRecord {
					phase: Phase::ApplyExtrinsic(0),
					event: Event::voting(voting::RawEvent::VoteCreated(vote_id, public, VoteType::Binary)),
					topics: vec![],
				},
				EventRecord {
					phase: Phase::ApplyExtrinsic(0),
					event: Event::governance(RawEvent::NewProposal(public, hash)),
					topics: vec![],
				},
				EventRecord {
					phase: Phase::ApplyExtrinsic(0),
					event: Event::voting(voting::RawEvent::VoteCreated(vote_id2, public, VoteType::Binary)),
					topics: vec![],
				},
				EventRecord {
					phase: Phase::ApplyExtrinsic(0),
					event: Event::governance(RawEvent::NewProposal(public, hash2)),
					topics: vec![],
				},]
			);
			assert_eq!(Governance::proposal_count(), 2);
			assert_eq!(Governance::proposals(), vec![hash, hash2]);
			assert_eq!(Governance::active_proposals(), vec![]);
			assert_eq!(
				Governance::proposal_of(hash),
				Some(make_record(public, title, proposal, category))
			);
			assert_eq!(
				Governance::proposal_of(hash2),
				Some(ProposalRecord {
					index: 1,
					vote_id: vote_id2,
					..make_record(public, title2, proposal2, category)
				})
			);
		});
	}

	#[test]
	fn propose_duplicate_should_fail() {
		with_externalities(&mut new_test_ext(), || {
			System::set_block_number(1);
			let public = get_test_key();
			let (title, proposal) = generate_proposal();
			let hash = build_proposal_hash(public, &proposal);
			let category = governance::ProposalCategory::Signaling;
			let outcomes = vec![governance::YES_VOTE, governance::NO_VOTE];
			assert_ok!(propose(public, title, proposal, category, outcomes.clone(), VoteType::Binary, TallyType::OneCoin));
			assert_eq!(propose(public, title, proposal, category, outcomes, VoteType::Binary, TallyType::OneCoin), Err("Proposal already exists"));
			assert_eq!(Governance::proposal_count(), 1);
			assert_eq!(Governance::proposals(), vec![hash]);
			assert_eq!(
				Governance::proposal_of(hash),
				Some(make_record(public, title, proposal, category))
			);
		});
	}

	#[test]
	fn propose_empty_should_fail() {
		with_externalities(&mut new_test_ext(), || {
			System::set_block_number(1);
			let public = get_test_key();
			let (title, _) = generate_proposal();
			let proposal = vec![];
			let hash = build_proposal_hash(public, &proposal);
			let category = governance::ProposalCategory::Signaling;
			let outcomes = vec![governance::YES_VOTE, governance::NO_VOTE];
			assert_eq!(propose(public, title, &proposal, category, outcomes, VoteType::Binary, TallyType::OneCoin), Err("Proposal must not be empty"));
			assert_eq!(Governance::proposal_count(), 0);
			assert_eq!(Governance::proposals(), vec![]);
			assert_eq!(Governance::proposal_of(hash), None);
		});
	}

	#[test]
	fn propose_empty_title_should_fail() {
		with_externalities(&mut new_test_ext(), || {
			System::set_block_number(1);
			let public = get_test_key();
			let (_, proposal) = generate_proposal();
			let hash = build_proposal_hash(public, &proposal);
			let title = vec![];
			let category = governance::ProposalCategory::Signaling;
			let outcomes = vec![governance::YES_VOTE, governance::NO_VOTE];
			assert_eq!(propose(public, &title, proposal, category, outcomes, VoteType::Binary, TallyType::OneCoin), Err("Proposal must have title"));
			assert_eq!(Governance::proposal_count(), 0);
			assert_eq!(Governance::proposals(), vec![]);
			assert_eq!(Governance::proposal_of(hash), None);
		});
	}

	#[test]
	fn advance_proposal_should_work() {
		with_externalities(&mut new_test_ext(), || {
			System::set_block_number(1);
			let public = get_test_key();
			let category = governance::ProposalCategory::Signaling;
			let (title, proposal) = generate_proposal();
			let hash = build_proposal_hash(public, &proposal);
			let outcomes = vec![governance::YES_VOTE, governance::NO_VOTE];
			assert_ok!(propose(public, title, proposal, category, outcomes, VoteType::Binary, TallyType::OneCoin));
			let vote_id = Governance::proposal_of(hash).unwrap().vote_id;
			assert_eq!(vote_id, 1);
			assert_ok!(advance_proposal(public, hash));

 			let vote_time = Governance::voting_length();
			let now = System::block_number();
			let vote_ends_at = now + vote_time;

			assert_eq!(System::events(), vec![
				EventRecord {
					phase: Phase::ApplyExtrinsic(0),
					event: Event::voting(voting::RawEvent::VoteCreated(vote_id, public, VoteType::Binary)),
					topics: vec![],
				},
				EventRecord {
					phase: Phase::ApplyExtrinsic(0),
					event: Event::governance(RawEvent::NewProposal(public, hash)),
					topics: vec![],
				},
				EventRecord {
					phase: Phase::ApplyExtrinsic(0),
					event: Event::voting(voting::RawEvent::VoteAdvanced(vote_id, VoteStage::PreVoting, VoteStage::Voting)),
					topics: vec![],
				},
				EventRecord {
					phase: Phase::ApplyExtrinsic(0),
					event: Event::governance(RawEvent::VotingStarted(hash, vote_id, vote_ends_at)),
					topics: vec![],
				},]
			);
			assert_eq!(Governance::active_proposals(), vec![(hash, 10001)]);
			assert_eq!(
				Governance::proposal_of(hash),
				Some(ProposalRecord {
					stage: ProposalStage::Voting,
					transition_time: 10001,
					..make_record(public, title, proposal, category)
				})
			);
		});
	}

	#[test]
	fn advance_proposal_if_voting_should_fail() {
		with_externalities(&mut new_test_ext(), || {
			System::set_block_number(1);
			let public = get_test_key();
			let category = governance::ProposalCategory::Signaling;
			let (title, proposal) = generate_proposal();
			let hash = build_proposal_hash(public, &proposal);
			let outcomes = vec![governance::YES_VOTE, governance::NO_VOTE];
			assert_ok!(propose(public, title, proposal, category, outcomes, VoteType::Binary, TallyType::OneCoin));
			assert_ok!(advance_proposal(public, hash));
			assert_err!(advance_proposal(public, hash),
									"Proposal not in pre-voting stage");
			assert_eq!(Governance::active_proposals(), vec![(hash, 10001)]);
			assert_eq!(
				Governance::proposal_of(hash),
				Some(ProposalRecord {
					stage: ProposalStage::Voting,
					transition_time: 10001,
					..make_record(public, title, proposal, category)
				})
			);
		});
	}

	#[test]
	fn voting_proposal_should_advance() {
		with_externalities(&mut new_test_ext(), || {
			System::set_block_number(1);
			let public = get_test_key();
			let category = governance::ProposalCategory::Signaling;
			let (title, proposal) = generate_proposal();
			let hash = build_proposal_hash(public, &proposal);
			let outcomes = vec![governance::YES_VOTE, governance::NO_VOTE];
			assert_ok!(propose(public, title, proposal, category, outcomes, VoteType::Binary, TallyType::OneCoin));
			let vote_id = Governance::proposal_of(hash).unwrap().vote_id;
			assert_eq!(vote_id, 1);
			assert_ok!(advance_proposal(public, hash));

 			let vote_time = Governance::voting_length();
			let now = System::block_number();
			let vote_ends_at = now + vote_time;

			assert_eq!(Governance::active_proposals(), vec![(hash, 10001)]);
			assert_eq!(
				Governance::proposal_of(hash),
				Some(ProposalRecord {
					stage: ProposalStage::Voting,
					transition_time: 10001,
					..make_record(public, title, proposal, category)
				})
			);

			System::set_block_number(2);
			<Governance as OnFinalize<u64>>::on_finalize(2);
			System::set_block_number(3);

			assert_eq!(System::events(), vec![
				EventRecord {
					phase: Phase::ApplyExtrinsic(0),
					event: Event::voting(voting::RawEvent::VoteCreated(vote_id, public, VoteType::Binary)),
					topics: vec![],
				},
				EventRecord {
					phase: Phase::ApplyExtrinsic(0),
					event: Event::governance(RawEvent::NewProposal(public, hash)),
					topics: vec![],
				},
				EventRecord {
					phase: Phase::ApplyExtrinsic(0),
					event: Event::voting(voting::RawEvent::VoteAdvanced(vote_id, VoteStage::PreVoting, VoteStage::Voting)),
					topics: vec![],
				},
				EventRecord {
					phase: Phase::ApplyExtrinsic(0),
					event: Event::governance(RawEvent::VotingStarted(hash, vote_id, vote_ends_at)),
					topics: vec![],
				},]
			);

			System::set_block_number(10002);
			<Governance as OnFinalize<u64>>::on_finalize(10002);
			System::set_block_number(10003);

			assert_eq!(System::events(), vec![
				EventRecord {
					phase: Phase::ApplyExtrinsic(0),
					event: Event::voting(voting::RawEvent::VoteCreated(vote_id, public, VoteType::Binary)),
					topics: vec![],
				},
				EventRecord {
					phase: Phase::ApplyExtrinsic(0),
					event: Event::governance(RawEvent::NewProposal(public, hash)),
					topics: vec![],
				},
				EventRecord {
					phase: Phase::ApplyExtrinsic(0),
					event: Event::voting(voting::RawEvent::VoteAdvanced(vote_id, VoteStage::PreVoting, VoteStage::Voting)),
					topics: vec![],
				},
				EventRecord {
					phase: Phase::ApplyExtrinsic(0),
					event: Event::governance(RawEvent::VotingStarted(hash, vote_id, vote_ends_at)),
					topics: vec![],
				},
				EventRecord {
					phase: Phase::ApplyExtrinsic(0),
					event: Event::voting(voting::RawEvent::VoteAdvanced(vote_id, VoteStage::Voting, VoteStage::Completed)),
					topics: vec![],
				},
				EventRecord {
					phase: Phase::ApplyExtrinsic(0),
					event: Event::governance(RawEvent::VotingCompleted(hash, vote_id)),
					topics: vec![],
				}]
			);

			assert_eq!(Governance::active_proposals(), vec![]);
			assert_eq!(
				Governance::proposal_of(hash),
				Some(ProposalRecord {
					stage: ProposalStage::Completed,
					transition_time: 0,
					..make_record(public, title, proposal, category)
				})
			);
		});
	}

	#[test]
	fn advance_proposal_if_completed_should_fail() {
		with_externalities(&mut new_test_ext(), || {
			System::set_block_number(1);
			let public = get_test_key();
			let category = governance::ProposalCategory::Signaling;
			let (title, proposal) = generate_proposal();
			let hash = build_proposal_hash(public, &proposal);
			let outcomes = vec![governance::YES_VOTE, governance::NO_VOTE];
			assert_ok!(propose(public, title, proposal, category, outcomes, VoteType::Binary, TallyType::OneCoin));
			assert_ok!(advance_proposal(public, hash));
			
			System::set_block_number(10002);
			<Governance as OnFinalize<u64>>::on_finalize(10002);
			System::set_block_number(10003);

			assert_err!(advance_proposal(public, hash), "Proposal not in pre-voting stage");
			assert_eq!(Governance::active_proposals(), vec![]);
			assert_eq!(
				Governance::proposal_of(hash),
				Some(ProposalRecord {
					stage: ProposalStage::Completed,
					transition_time: 0,
					..make_record(public, title, proposal, category)
				})
			);
		});
	}

	#[test]
	fn non_author_advance_should_fail() {
		with_externalities(&mut new_test_ext(), || {
			System::set_block_number(1);
			let public = get_test_key();
			let category = governance::ProposalCategory::Signaling;
			let (title, proposal) = generate_proposal();
			let hash = build_proposal_hash(public, &proposal);

			let other_public = 2_u64;
			let outcomes = vec![governance::YES_VOTE, governance::NO_VOTE];
			assert_ok!(propose(public, title, proposal, category, outcomes, VoteType::Binary, TallyType::OneCoin));
			assert_err!(advance_proposal(other_public, hash), "Proposal must be advanced by author");
			assert_eq!(Governance::active_proposals(), vec![]);
			assert_eq!(
				Governance::proposal_of(hash),
				Some(make_record(public, title, proposal, category))
			);
		});
	}

	#[test]
	fn creating_proposal_with_insufficient_balance_fails() {
		with_externalities(&mut new_test_ext(), || {
			System::set_block_number(1);
			let public = 100_u64;
			let category = governance::ProposalCategory::Signaling;
			let (title, proposal) = generate_proposal();
			let outcomes = vec![governance::YES_VOTE, governance::NO_VOTE];

			assert_err!(
				propose(public, title, proposal, category, outcomes, VoteType::Binary, TallyType::OneCoin),
				"Not enough currency for reserve bond");
		});
	}

	#[test]
	fn completed_proposal_should_return_creation_bond() {
		with_externalities(&mut new_test_ext(), || {
			System::set_block_number(1);
			let public = get_test_key();
			let category = governance::ProposalCategory::Signaling;
			let (title, proposal) = generate_proposal();
			let outcomes = vec![governance::YES_VOTE, governance::NO_VOTE];
			let hash = build_proposal_hash(public, &proposal);
			let balance = Balances::free_balance(public);
			assert_ok!(propose(public, title, proposal, category, outcomes, VoteType::Binary, TallyType::OneCoin));
			let after_propose_balance = Balances::free_balance(public);
			assert_eq!(balance - BOND, after_propose_balance);
			assert_ok!(advance_proposal(public, hash));

			System::set_block_number(10002);
			<Governance as OnFinalize<u64>>::on_finalize(10002);
			System::set_block_number(10003);

			let after_completion_balance = Balances::free_balance(public);
			assert_eq!(balance, after_completion_balance);
		});
	}
} 
