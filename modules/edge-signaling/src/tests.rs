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

use super::*;

use frame_support::{
	parameter_types, impl_outer_origin, assert_err, assert_ok, weights::Weight,
	traits::{OnFinalize}, Twox128, Blake2_256, StorageHasher,
	storage::{unhashed, generator::StorageMap as GeneratorMap},
};
use sp_core::{H256, Blake2Hasher, Hasher};
use sp_runtime::{
	Perbill,
	traits::{BlakeTwo256, IdentityLookup},
	testing::{Header}
};
pub use crate::{Event, Module, RawEvent, Trait, GenesisConfig};
use voting::{VoteOutcome, TallyType, VoteStage, VoteType};

impl_outer_origin! {
	pub enum Origin for Test {}
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Test;

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: Weight = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::one();
	pub const MaximumExtrinsicWeight: Weight = 1024;
}

type AccountId = u64;
type BlockNumber = u64;

impl frame_system::Trait for Test {
	type BaseCallFilter = ();
	type Origin = Origin;
	type Index = u64;
	type BlockNumber = BlockNumber;
	type Call = ();
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = ();
	type BlockHashCount = BlockHashCount;
	type MaximumBlockWeight = MaximumBlockWeight;
	type DbWeight = ();
	type BlockExecutionWeight = ();
	type ExtrinsicBaseWeight = ();
	type MaximumBlockLength = MaximumBlockLength;
	type AvailableBlockRatio = AvailableBlockRatio;
	type Version = ();
	type ModuleToIndex = ();
	type AccountData = pallet_balances::AccountData<u128>;
	type OnNewAccount = ();
	type MigrateAccount = ();
	type OnKilledAccount = ();
	type MaximumExtrinsicWeight = MaximumExtrinsicWeight;
}

parameter_types! {
	pub const ExistentialDeposit: u128 = 1;
}

impl pallet_balances::Trait for Test {
	type Balance = u128;
	type Event = ();
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = frame_system::Module<Test>;
}

impl voting::Trait for Test {
	type Event = ();
}

impl Trait for Test {
	type Event = ();
	type Currency = pallet_balances::Module<Self>;
}

pub type Balances = pallet_balances::Module<Test>;
pub type System = frame_system::Module<Test>;
pub type Voting = voting::Module<Test>;
pub type Signaling = Module<Test>;

const BOND: u128 = 10;
const YES_VOTE: voting::VoteOutcome = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1];
const NO_VOTE: voting::VoteOutcome = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
const OTHER_VOTE: voting::VoteOutcome = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2];

fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	// We use default for brevity, but you can configure as desired if needed.
	GenesisConfig::<Test> {
		voting_length: 10000,
		proposal_creation_bond: BOND,
	}.assimilate_storage(&mut t).unwrap();
	pallet_balances::GenesisConfig::<Test> {
		balances: vec![
			(1, 100),
			(2, 100),
			(3, 100),
			(4, 100),
		],
	}.assimilate_storage(&mut t).unwrap();
	t.into()
}

fn propose(
	who: u64,
	title: &[u8],
	proposal: &[u8],
	outcomes: Vec<VoteOutcome>,
	vote_type: VoteType,
	tally_type: TallyType
) -> DispatchResult {
	Signaling::create_proposal(
		Origin::signed(who),
		title.to_vec(),
		proposal.to_vec(),
		outcomes,
		vote_type,
		tally_type)
}

fn advance_proposal(who: u64, proposal_hash: H256) -> DispatchResult {
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
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let public = get_test_key();
		let (title, proposal) = generate_proposal();
		let hash = build_proposal_hash(public, &proposal);
		let outcomes = vec![YES_VOTE, NO_VOTE];
		assert_ok!(propose(public, title, proposal, outcomes, VoteType::Binary, TallyType::OneCoin));
		let _vote_id = Signaling::proposal_of(hash).unwrap().vote_id;
		let title2: &[u8] = b"Proposal 2";
		let proposal2: &[u8] = b"Proposal 2";
		let hash2 = build_proposal_hash(public, &proposal2);
		let outcomes = vec![YES_VOTE, NO_VOTE];
		assert_ok!(propose(public, title2, proposal2, outcomes, VoteType::Binary, TallyType::OneCoin));
		let vote_id2 = Signaling::proposal_of(hash2).unwrap().vote_id;

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
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let public = get_test_key();
		let (title, proposal) = generate_proposal();
		let hash = build_proposal_hash(public, &proposal);
		let outcomes = vec![YES_VOTE, NO_VOTE];
		assert_ok!(propose(public, title, proposal, outcomes.clone(), VoteType::Binary, TallyType::OneCoin));
		assert_err!(
			propose(public, title, proposal, outcomes, VoteType::Binary, TallyType::OneCoin),
			"Proposal already exists");
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
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let public = get_test_key();
		let (title, _) = generate_proposal();
		let proposal = vec![];
		let hash = build_proposal_hash(public, &proposal);
		let outcomes = vec![YES_VOTE, NO_VOTE];
		assert_err!(
			propose(public, title, &proposal, outcomes, VoteType::Binary, TallyType::OneCoin),
			"Proposal must not be empty"
		);
		assert_eq!(Signaling::proposal_count(), 0);
		assert_eq!(Signaling::inactive_proposals(), vec![]);
		assert_eq!(Signaling::proposal_of(hash), None);
	});
}

#[test]
fn propose_empty_title_should_fail() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let public = get_test_key();
		let (_, proposal) = generate_proposal();
		let hash = build_proposal_hash(public, &proposal);
		let title = vec![];
		let outcomes = vec![YES_VOTE, NO_VOTE];
		assert_err!(
			propose(public, &title, proposal, outcomes, VoteType::Binary, TallyType::OneCoin),
			"Proposal must have title"
		);
		assert_eq!(Signaling::proposal_count(), 0);
		assert_eq!(Signaling::inactive_proposals(), vec![]);
		assert_eq!(Signaling::proposal_of(hash), None);
	});
}

#[test]
fn advance_proposal_should_work() {
	new_test_ext().execute_with(|| {
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
		let _vote_ends_at = now + vote_time;

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
	new_test_ext().execute_with(|| {
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
	new_test_ext().execute_with(|| {
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
		let _vote_ends_at = now + vote_time;

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

		System::set_block_number(10002);
		<Signaling as OnFinalize<u64>>::on_finalize(10002);
		System::set_block_number(10003);

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
	new_test_ext().execute_with(|| {
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
	new_test_ext().execute_with(|| {
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
	new_test_ext().execute_with(|| {
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
	new_test_ext().execute_with(|| {
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
	new_test_ext().execute_with(|| {
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
	new_test_ext().execute_with(|| {
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

#[test]
fn propose_multichoice_should_work() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let public = get_test_key();
		let (title, proposal) = generate_proposal();
		let hash = build_proposal_hash(public, &proposal);
		let outcomes = vec![YES_VOTE, NO_VOTE, OTHER_VOTE];
		assert_ok!(propose(public, title, proposal, outcomes, VoteType::MultiOption, TallyType::OneCoin));
		let _vote_id = Signaling::proposal_of(hash).unwrap().vote_id;
		let title2: &[u8] = b"Proposal 2";
		let proposal2: &[u8] = b"Proposal 2";
		let hash2 = build_proposal_hash(public, &proposal2);
		let outcomes = vec![YES_VOTE, NO_VOTE, OTHER_VOTE];
		assert_ok!(propose(public, title2, proposal2, outcomes, VoteType::MultiOption, TallyType::OneCoin));
		let vote_id2 = Signaling::proposal_of(hash2).unwrap().vote_id;

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
fn change_hasher_migration() {
	mod deprecated {
		use sp_std::prelude::*;
		
		use codec::{Encode, Decode};
		use frame_support::{decl_module, decl_storage};

		use crate::{Trait, ProposalRecord};

		decl_module! {
			pub struct Module<T: Trait> for enum Call where origin: T::Origin { }
		}
		decl_storage! {
			trait Store for Module<T: Trait> as Signaling {
				pub ProposalOf get(fn proposal_of): map hasher(opaque_blake2_256) 
					T::Hash => Option<ProposalRecord<T::AccountId, T::BlockNumber>>;
			}
		}
	}

	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		// build proposal, vote and record
		let public = get_test_key();
		let (title, proposal) = generate_proposal();
		let hash = build_proposal_hash(public, &proposal);
		let outcomes = vec![YES_VOTE, NO_VOTE];
		let index = ProposalCount::get();
		let vote_id = Voting::create_vote(
			public.clone(),
			VoteType::Binary,
			false, // not commit-reveal
			TallyType::OneCoin,
			outcomes,
		).expect("Voting::create_vote failed");
		let transition_time = System::block_number() + Signaling::voting_length();
		let record = ProposalRecord {
			index: index,
			author: public.clone(),
			stage: VoteStage::PreVoting,
			transition_time: transition_time,
			title: title.to_vec(),
			contents: proposal.to_vec(),
			vote_id: vote_id,
		};
		// insert the record with the old hasher
		deprecated::ProposalOf::<Test>::insert(hash, &record);
		InactiveProposals::<Test>::mutate(|proposals| proposals.push((hash, transition_time)));
		assert!(
			Signaling::proposal_of(hash).is_none(),
			"proposal should not (yet) be available with the new hasher"
		);
		// do the migration
		crate::migration::migrate::<Test>();
		let maybe_prop = Signaling::proposal_of(hash);
		// check that it was successfull
		assert!(maybe_prop.is_some());
		let prop = maybe_prop.unwrap();
		assert_eq!(prop, record);
	});
}