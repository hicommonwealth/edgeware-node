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
#[macro_use]
extern crate srml_support;
extern crate parity_codec as codec;
extern crate substrate_primitives as primitives;
extern crate sr_std as rstd;
extern crate srml_support as runtime_support;
extern crate sr_primitives as runtime_primitives;
extern crate sr_io as runtime_io;

extern crate srml_balances as balances;
extern crate srml_system as system;
extern crate edge_voting as voting;

pub mod signaling;
pub use signaling::{Module, Trait, RawEvent, Event, ProposalRecord};
pub use voting::voting::{VoteType, VoteOutcome, VoteStage, TallyType};

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
		Perbill,
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
			voting<T>, balances<T>, signaling<T>,
		}
	}
	
	#[derive(Clone, PartialEq, Eq, Debug)]
	pub struct Test;

	parameter_types! {
		pub const BlockHashCount: u64 = 250;
		pub const MaximumBlockWeight: u32 = 1024;
		pub const MaximumBlockLength: u32 = 2 * 1024;
		pub const AvailableBlockRatio: Perbill = Perbill::one();
	}

	impl system::Trait for Test {
		type Origin = Origin;
		type Call = ();
		type Index = u64;
		type BlockNumber = u64;
		type Hash = H256;
		type Hashing = BlakeTwo256;
		type AccountId = u64;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Header = Header;
		type Event = Event;
		type WeightMultiplierUpdate = ();
		type BlockHashCount = BlockHashCount;
		type MaximumBlockWeight = MaximumBlockWeight;
		type MaximumBlockLength = MaximumBlockLength;
		type AvailableBlockRatio = AvailableBlockRatio;
		type Version = ();
	}

	parameter_types! {
		pub const ExistentialDeposit: u64 = 0;
		pub const TransferFee: u64 = 0;
		pub const CreationFee: u64 = 0;
		pub const TransactionBaseFee: u64 = 0;
		pub const TransactionByteFee: u64 = 0;
	}
	impl balances::Trait for Test {
		type Balance = u64;
		type OnNewAccount = ();
		type OnFreeBalanceZero = ();
		type Event = Event;
		type TransactionPayment = ();
		type TransferPayment = ();
		type DustRemoval = ();
		type ExistentialDeposit = ExistentialDeposit;
		type TransferFee = TransferFee;
		type CreationFee = CreationFee;
		type TransactionBaseFee = TransactionBaseFee;
		type TransactionByteFee = TransactionByteFee;
		type WeightToFee = ();
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
	pub type Signaling = Module<Test>;

	const BOND: u64 = 10;
	const YES_VOTE: voting::voting::VoteOutcome = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1];
	const NO_VOTE: voting::voting::VoteOutcome = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];

	fn new_test_ext() -> sr_io::TestExternalities<Blake2Hasher> {
		let mut t = system::GenesisConfig::default().build_storage::<Test>().unwrap();
		// We use default for brevity, but you can configure as desired if needed.
		t.0.extend(
			signaling::GenesisConfig::<Test> {
				voting_length: 10000,
				proposal_creation_bond: BOND,
			}.build_storage().unwrap().0,
		);
		t.0.extend(
			balances::GenesisConfig::<Test> {
				balances: vec![
					(1, 100),
					(2, 100),
					(3, 100),
					(4, 100),
				],
				vesting: vec![],
			}.build_storage().unwrap().0,
		);
		t.into()
	}

	fn propose(
		who: u64,
		title: &[u8],
		proposal: &[u8],
		outcomes: Vec<VoteOutcome>,
		vote_type: VoteType,
		tally_type: TallyType
	) -> Result {
		Signaling::create_proposal(
			Origin::signed(who),
			title.to_vec(),
			proposal.to_vec(),
			outcomes,
			vote_type,
			tally_type)
	}

	fn advance_proposal(who: u64, proposal_hash: H256) -> Result {
		Signaling::advance_proposal(Origin::signed(who), proposal_hash)
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
		contents: &[u8])
		-> ProposalRecord<u64, u64> {
			ProposalRecord {
				index: 0,
				author: author,
				stage: VoteStage::PreVoting,
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
			let (title, proposal) = generate_proposal();
			let hash = build_proposal_hash(public, &proposal);
			let outcomes = vec![YES_VOTE, NO_VOTE];
			assert_ok!(propose(public, title, proposal, outcomes, VoteType::Binary, TallyType::OneCoin));
			let vote_id = Signaling::proposal_of(hash).unwrap().vote_id;
			assert_eq!(System::events(), vec![
				EventRecord {
					phase: Phase::ApplyExtrinsic(0),
					event: Event::voting(voting::RawEvent::VoteCreated(vote_id, public, VoteType::Binary)),
					topics: vec![],
				},
				EventRecord {
					phase: Phase::ApplyExtrinsic(0),
					event: Event::signaling(RawEvent::NewProposal(public, hash)),
					topics: vec![],
				}]
			);

			let title2: &[u8] = b"Proposal 2";
			let proposal2: &[u8] = b"Proposal 2";
			let hash2 = build_proposal_hash(public, &proposal2);
			let outcomes = vec![YES_VOTE, NO_VOTE];
			assert_ok!(propose(public, title2, proposal2, outcomes, VoteType::Binary, TallyType::OneCoin));
			let vote_id2 = Signaling::proposal_of(hash2).unwrap().vote_id;
			assert_eq!(System::events(), vec![
				EventRecord {
					phase: Phase::ApplyExtrinsic(0),
					event: Event::voting(voting::RawEvent::VoteCreated(vote_id, public, VoteType::Binary)),
					topics: vec![],
				},
				EventRecord {
					phase: Phase::ApplyExtrinsic(0),
					event: Event::signaling(RawEvent::NewProposal(public, hash)),
					topics: vec![],
				},
				EventRecord {
					phase: Phase::ApplyExtrinsic(0),
					event: Event::voting(voting::RawEvent::VoteCreated(vote_id2, public, VoteType::Binary)),
					topics: vec![],
				},
				EventRecord {
					phase: Phase::ApplyExtrinsic(0),
					event: Event::signaling(RawEvent::NewProposal(public, hash2)),
					topics: vec![],
				},]
			);
			assert_eq!(Signaling::proposal_count(), 2);
			assert_eq!(Signaling::inactive_proposals(), vec![(hash, 10001), (hash2, 10001)]);
			assert_eq!(Signaling::active_proposals(), vec![]);
			assert_eq!(
				Signaling::proposal_of(hash),
				Some(ProposalRecord {
					transition_time: 10001,
					..make_record(public, title, proposal)
				})
			);
			assert_eq!(
				Signaling::proposal_of(hash2),
				Some(ProposalRecord {
					index: 1,
					vote_id: vote_id2,
					transition_time: 10001,
					..make_record(public, title2, proposal2)
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
			let outcomes = vec![YES_VOTE, NO_VOTE];
			assert_ok!(propose(public, title, proposal, outcomes.clone(), VoteType::Binary, TallyType::OneCoin));
			assert_eq!(propose(public, title, proposal, outcomes, VoteType::Binary, TallyType::OneCoin), Err("Proposal already exists"));
			assert_eq!(Signaling::proposal_count(), 1);
			assert_eq!(Signaling::inactive_proposals(), vec![(hash, 10001)]);
			assert_eq!(
				Signaling::proposal_of(hash),
				Some(ProposalRecord {
					transition_time: 10001,
					..make_record(public, title, proposal)
				})
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
			let outcomes = vec![YES_VOTE, NO_VOTE];
			assert_eq!(propose(public, title, &proposal, outcomes, VoteType::Binary, TallyType::OneCoin), Err("Proposal must not be empty"));
			assert_eq!(Signaling::proposal_count(), 0);
			assert_eq!(Signaling::inactive_proposals(), vec![]);
			assert_eq!(Signaling::proposal_of(hash), None);
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
			let outcomes = vec![YES_VOTE, NO_VOTE];
			assert_eq!(propose(public, &title, proposal, outcomes, VoteType::Binary, TallyType::OneCoin), Err("Proposal must have title"));
			assert_eq!(Signaling::proposal_count(), 0);
			assert_eq!(Signaling::inactive_proposals(), vec![]);
			assert_eq!(Signaling::proposal_of(hash), None);
		});
	}

	#[test]
	fn advance_proposal_should_work() {
		with_externalities(&mut new_test_ext(), || {
			System::set_block_number(1);
			let public = get_test_key();
			let (title, proposal) = generate_proposal();
			let hash = build_proposal_hash(public, &proposal);
			let outcomes = vec![YES_VOTE, NO_VOTE];
			assert_ok!(propose(public, title, proposal, outcomes, VoteType::Binary, TallyType::OneCoin));
			let vote_id = Signaling::proposal_of(hash).unwrap().vote_id;
			assert_eq!(vote_id, 1);
			assert_ok!(advance_proposal(public, hash));

			let vote_time = Signaling::voting_length();
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
					event: Event::signaling(RawEvent::NewProposal(public, hash)),
					topics: vec![],
				},
				EventRecord {
					phase: Phase::ApplyExtrinsic(0),
					event: Event::voting(voting::RawEvent::VoteAdvanced(vote_id, VoteStage::PreVoting, VoteStage::Voting)),
					topics: vec![],
				},
				EventRecord {
					phase: Phase::ApplyExtrinsic(0),
					event: Event::signaling(RawEvent::VotingStarted(hash, vote_id, vote_ends_at)),
					topics: vec![],
				},]
			);
			assert_eq!(Signaling::active_proposals(), vec![(hash, 10001)]);
			assert_eq!(
				Signaling::proposal_of(hash),
				Some(ProposalRecord {
					stage: VoteStage::Voting,
					transition_time: 10001,
					..make_record(public, title, proposal)
				})
			);
		});
	}

	#[test]
	fn advance_proposal_if_voting_should_fail() {
		with_externalities(&mut new_test_ext(), || {
			System::set_block_number(1);
			let public = get_test_key();
			let (title, proposal) = generate_proposal();
			let hash = build_proposal_hash(public, &proposal);
			let outcomes = vec![YES_VOTE, NO_VOTE];
			assert_ok!(propose(public, title, proposal, outcomes, VoteType::Binary, TallyType::OneCoin));
			assert_ok!(advance_proposal(public, hash));
			assert_err!(advance_proposal(public, hash),
									"Proposal not in pre-voting or commit stage");
			assert_eq!(Signaling::active_proposals(), vec![(hash, 10001)]);
			assert_eq!(
				Signaling::proposal_of(hash),
				Some(ProposalRecord {
					stage: VoteStage::Voting,
					transition_time: 10001,
					..make_record(public, title, proposal)
				})
			);
		});
	}

	#[test]
	fn voting_proposal_should_advance() {
		with_externalities(&mut new_test_ext(), || {
			System::set_block_number(1);
			let public = get_test_key();
			let (title, proposal) = generate_proposal();
			let hash = build_proposal_hash(public, &proposal);
			let outcomes = vec![YES_VOTE, NO_VOTE];
			assert_ok!(propose(public, title, proposal, outcomes, VoteType::Binary, TallyType::OneCoin));
			let vote_id = Signaling::proposal_of(hash).unwrap().vote_id;
			assert_eq!(vote_id, 1);
			assert_ok!(advance_proposal(public, hash));

			let vote_time = Signaling::voting_length();
			let now = System::block_number();
			let vote_ends_at = now + vote_time;

			assert_eq!(Signaling::active_proposals(), vec![(hash, 10001)]);
			assert_eq!(
				Signaling::proposal_of(hash),
				Some(ProposalRecord {
					stage: VoteStage::Voting,
					transition_time: 10001,
					..make_record(public, title, proposal)
				})
			);

			System::set_block_number(2);
			<Signaling as OnFinalize<u64>>::on_finalize(2);
			System::set_block_number(3);

			assert_eq!(System::events(), vec![
				EventRecord {
					phase: Phase::ApplyExtrinsic(0),
					event: Event::voting(voting::RawEvent::VoteCreated(vote_id, public, VoteType::Binary)),
					topics: vec![],
				},
				EventRecord {
					phase: Phase::ApplyExtrinsic(0),
					event: Event::signaling(RawEvent::NewProposal(public, hash)),
					topics: vec![],
				},
				EventRecord {
					phase: Phase::ApplyExtrinsic(0),
					event: Event::voting(voting::RawEvent::VoteAdvanced(vote_id, VoteStage::PreVoting, VoteStage::Voting)),
					topics: vec![],
				},
				EventRecord {
					phase: Phase::ApplyExtrinsic(0),
					event: Event::signaling(RawEvent::VotingStarted(hash, vote_id, vote_ends_at)),
					topics: vec![],
				},]
			);

			System::set_block_number(10002);
			<Signaling as OnFinalize<u64>>::on_finalize(10002);
			System::set_block_number(10003);

			assert_eq!(System::events(), vec![
				EventRecord {
					phase: Phase::ApplyExtrinsic(0),
					event: Event::voting(voting::RawEvent::VoteCreated(vote_id, public, VoteType::Binary)),
					topics: vec![],
				},
				EventRecord {
					phase: Phase::ApplyExtrinsic(0),
					event: Event::signaling(RawEvent::NewProposal(public, hash)),
					topics: vec![],
				},
				EventRecord {
					phase: Phase::ApplyExtrinsic(0),
					event: Event::voting(voting::RawEvent::VoteAdvanced(vote_id, VoteStage::PreVoting, VoteStage::Voting)),
					topics: vec![],
				},
				EventRecord {
					phase: Phase::ApplyExtrinsic(0),
					event: Event::signaling(RawEvent::VotingStarted(hash, vote_id, vote_ends_at)),
					topics: vec![],
				},
				EventRecord {
					phase: Phase::ApplyExtrinsic(0),
					event: Event::voting(voting::RawEvent::VoteAdvanced(vote_id, VoteStage::Voting, VoteStage::Completed)),
					topics: vec![],
				},
				EventRecord {
					phase: Phase::ApplyExtrinsic(0),
					event: Event::signaling(RawEvent::VotingCompleted(hash, vote_id)),
					topics: vec![],
				}]
			);

			assert_eq!(Signaling::active_proposals(), vec![]);
			assert_eq!(
				Signaling::proposal_of(hash),
				Some(ProposalRecord {
					stage: VoteStage::Completed,
					transition_time: 20002,
					..make_record(public, title, proposal)
				})
			);
		});
	}

	#[test]
	fn advance_proposal_if_completed_should_fail() {
		with_externalities(&mut new_test_ext(), || {
			System::set_block_number(1);
			let public = get_test_key();
			let (title, proposal) = generate_proposal();
			let hash = build_proposal_hash(public, &proposal);
			let outcomes = vec![YES_VOTE, NO_VOTE];
			assert_ok!(propose(public, title, proposal, outcomes, VoteType::Binary, TallyType::OneCoin));
			assert_ok!(advance_proposal(public, hash));
			assert_eq!(Signaling::active_proposals(), vec![(hash, 10001)]);
			System::set_block_number(10002);
			<Signaling as OnFinalize<u64>>::on_finalize(10002);
			System::set_block_number(10003);
			assert_err!(advance_proposal(public, hash), "Proposal not in pre-voting or commit stage");
			assert_eq!(Signaling::active_proposals(), vec![]);
			assert_eq!(
				Signaling::proposal_of(hash),
				Some(ProposalRecord {
					stage: VoteStage::Completed,
					transition_time: 20002,
					..make_record(public, title, proposal)
				})
			);
		});
	}

	#[test]
	fn non_author_advance_should_fail() {
		with_externalities(&mut new_test_ext(), || {
			System::set_block_number(1);
			let public = get_test_key();
			let (title, proposal) = generate_proposal();
			let hash = build_proposal_hash(public, &proposal);

			let other_public = 2_u64;
			let outcomes = vec![YES_VOTE, NO_VOTE];
			assert_ok!(propose(public, title, proposal, outcomes, VoteType::Binary, TallyType::OneCoin));
			assert_err!(advance_proposal(other_public, hash), "Proposal must be advanced by author");
			assert_eq!(Signaling::active_proposals(), vec![]);
			assert_eq!(
				Signaling::proposal_of(hash),
				Some(ProposalRecord{
					transition_time: 10001,
					..make_record(public, title, proposal)
				})
			);
		});
	}

	#[test]
	fn creating_proposal_with_insufficient_balance_fails() {
		with_externalities(&mut new_test_ext(), || {
			System::set_block_number(1);
			let public = 100_u64;
			let (title, proposal) = generate_proposal();
			let outcomes = vec![YES_VOTE, NO_VOTE];

			assert_err!(
				propose(public, title, proposal, outcomes, VoteType::Binary, TallyType::OneCoin),
				"Not enough currency for reserve bond");
		});
	}

	#[test]
	fn completed_proposal_should_return_creation_bond() {
		with_externalities(&mut new_test_ext(), || {
			System::set_block_number(1);
			let public = get_test_key();
			let (title, proposal) = generate_proposal();
			let outcomes = vec![YES_VOTE, NO_VOTE];
			let hash = build_proposal_hash(public, &proposal);
			let balance = Balances::free_balance(public);
			assert_ok!(propose(public, title, proposal, outcomes, VoteType::Binary, TallyType::OneCoin));
			let after_propose_balance = Balances::free_balance(public);
			assert_eq!(balance - BOND, after_propose_balance);
			assert_ok!(advance_proposal(public, hash));
			println!("{:?}", Signaling::proposal_of(hash));
			System::set_block_number(10002);
			<Signaling as OnFinalize<u64>>::on_finalize(10002);
			System::set_block_number(10003);

			let after_completion_balance = Balances::free_balance(public);
			assert_eq!(balance, after_completion_balance);
		});
	}

	#[test]
	fn expired_inactive_proposal_should_return_creation_bond() {
		with_externalities(&mut new_test_ext(), || {
			System::set_block_number(1);
			let public = get_test_key();
			let (title, proposal) = generate_proposal();
			let outcomes = vec![YES_VOTE, NO_VOTE];
			let hash = build_proposal_hash(public, &proposal);
			let balance = Balances::free_balance(public);
			assert_ok!(propose(public, title, proposal, outcomes, VoteType::Binary, TallyType::OneCoin));
			let after_propose_balance = Balances::free_balance(public);
			assert_eq!(balance - BOND, after_propose_balance);
			System::set_block_number(10002);
			<Signaling as OnFinalize<u64>>::on_finalize(10002);
			System::set_block_number(10003);

			let after_completion_balance = Balances::free_balance(public);
			assert_eq!(balance, after_completion_balance);
			assert_eq!(
				Signaling::proposal_of(hash),
				None
			);
		});
	}

	#[test]
	fn completed_proposal_should_remain_before_deletion() {
		with_externalities(&mut new_test_ext(), || {
			System::set_block_number(1);
			let public = get_test_key();
			let (title, proposal) = generate_proposal();
			let outcomes = vec![YES_VOTE, NO_VOTE];
			let hash = build_proposal_hash(public, &proposal);
			assert_ok!(propose(public, title, proposal, outcomes, VoteType::Binary, TallyType::OneCoin));
			assert_ok!(advance_proposal(public, hash));
			System::set_block_number(10002);
			<Signaling as OnFinalize<u64>>::on_finalize(10002);
			System::set_block_number(10003);
			assert_eq!(
				Signaling::proposal_of(hash),
				Some(ProposalRecord{
					stage: VoteStage::Completed,
					transition_time: 20002,
					..make_record(public, title, proposal)
				})
			);
			System::set_block_number(20003);
			<Signaling as OnFinalize<u64>>::on_finalize(20003);
			System::set_block_number(20004);
			assert_eq!(
				Signaling::proposal_of(hash),
				None
			);
		});
	}
}
