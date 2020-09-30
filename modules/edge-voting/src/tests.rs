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

use sp_runtime::{
	Perbill,
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};
use sp_core::H256;
use frame_support::{
	parameter_types, impl_outer_origin, assert_err, assert_ok, weights::Weight,
	traits::{OnFinalize}
};

use super::*;
use crate::{Trait, Module, VoteType, TallyType};

static SECRET: [u8; 32] = [1,0,1,0,1,0,1,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,4];

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

impl frame_system::Trait for Test {
	type BaseCallFilter = ();
	type Origin = Origin;
	type Index = u64;
	type BlockNumber = u64;
	type Call = ();
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
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

impl Trait for Test {
	type Event = ();
}

pub type System = frame_system::Module<Test>;
pub type Voting = Module<Test>;

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
fn new_test_ext() -> sp_io::TestExternalities {
	let t = system::GenesisConfig::default().build_storage::<Test>().unwrap();
	// We use default for brevity, but you can configure as desired if needed.
	t.into()
}

fn create_vote(
	who: u64,
	vote_type: VoteType,
	is_commit_reveal: bool,
	tally_type: TallyType,
	outcomes: &[[u8; 32]]
) -> result::Result<u64, &'static str> {
	Voting::create_vote(who,
						vote_type,
						is_commit_reveal,
						tally_type,
						outcomes.to_vec())
}

fn commit(who: u64, vote_id: u64, commit: [u8; 32]) -> DispatchResult {
	Voting::commit(Origin::signed(who), vote_id, commit)
}

fn reveal(who: u64, vote_id: u64, vote: Vec<[u8; 32]>, secret: Option<[u8; 32]>) -> DispatchResult {
	Voting::reveal(Origin::signed(who), vote_id, vote, secret)
}

fn advance_stage(vote_id: u64) -> DispatchResult {
	Voting::advance_stage(vote_id)
}

fn get_test_key() -> u64 {
	let public = 1_u64;
	return public;
}

fn get_test_key_2() -> u64 {
	let public = 2_u64;
	return public;		
}

fn generate_1p1v_public_binary_vote() -> (VoteType, bool, TallyType, [[u8; 32]; 2]) {
	let vote_type = VoteType::Binary;
	let tally_type = TallyType::OnePerson;
	let is_commit_reveal = false;
	let yes_outcome: [u8; 32] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1];
	let no_outcome: [u8; 32] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];

	return (vote_type, is_commit_reveal, tally_type, [yes_outcome, no_outcome]);
}

fn generate_1p1v_commit_reveal_binary_vote() -> (VoteType, bool, TallyType, [[u8; 32]; 2]) {
	let vote_type = VoteType::Binary;
	let tally_type = TallyType::OnePerson;
	let is_commit_reveal = true;
	let yes_outcome: [u8; 32] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1];
	let no_outcome: [u8; 32] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];

	return (vote_type, is_commit_reveal, tally_type, [yes_outcome, no_outcome]);
}

fn generate_1p1v_public_multi_vote() -> (VoteType, bool, TallyType, [[u8; 32]; 4]) {
	let vote_type = VoteType::MultiOption;
	let tally_type = TallyType::OnePerson;
	let is_commit_reveal = false;
	let one_outcome: [u8; 32] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1];
	let two_outcome: [u8; 32] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2];
	let three_outcome: [u8; 32] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,3];
	let four_outcome: [u8; 32] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,4];

	return (vote_type, is_commit_reveal, tally_type, [
		one_outcome,
		two_outcome,
		three_outcome,
		four_outcome
	]);
}

fn generate_1p1v_public_ranked_choice_vote() -> (VoteType, bool, TallyType, [[u8; 32]; 4]) {
	let vote_type = VoteType::RankedChoice;
	let tally_type = TallyType::OnePerson;
	let is_commit_reveal = false;
	let one_outcome: [u8; 32] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1];
	let two_outcome: [u8; 32] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2];
	let three_outcome: [u8; 32] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,3];
	let four_outcome: [u8; 32] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,4];

	return (vote_type, is_commit_reveal, tally_type, [
		one_outcome,
		two_outcome,
		three_outcome,
		four_outcome
	]);
}

fn generate_1p1v_commit_reveal_ranked_choice_vote() -> (VoteType, bool, TallyType, [[u8; 32]; 4]) {
	let vote_type = VoteType::RankedChoice;
	let tally_type = TallyType::OnePerson;
	let is_commit_reveal = true;
	let one_outcome: [u8; 32] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1];
	let two_outcome: [u8; 32] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2];
	let three_outcome: [u8; 32] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,3];
	let four_outcome: [u8; 32] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,4];

	return (vote_type, is_commit_reveal, tally_type, [
		one_outcome,
		two_outcome,
		three_outcome,
		four_outcome
	]);
}

fn make_record(
	id: u64,
	author: u64,
	vote_type: VoteType,
	is_commit_reveal: bool,
	tally_type: TallyType,
	outcomes: &[[u8; 32]],
	stage: VoteStage
) -> VoteRecord<u64> {
	VoteRecord {
		id: id,
		commitments: vec![],
		reveals: vec![],
		outcomes: outcomes.to_vec(),
		data: VoteData {
			initiator: author,
			stage: stage,
			vote_type: vote_type,
			tally_type: tally_type,
			is_commit_reveal: is_commit_reveal,
		},
	}
}

#[test]
fn create_binary_vote_should_work() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let public = get_test_key();
		let vote = generate_1p1v_public_binary_vote();
		assert_eq!(Ok(1), create_vote(public, vote.0, vote.1, vote.2, &vote.3));
		assert_eq!(Voting::vote_record_count(), 1);
		assert_eq!(
			Voting::vote_records(1),
			Some(make_record(1, public, vote.0, vote.1, vote.2, &vote.3, VoteStage::PreVoting))
		);
	});
}

#[test]
fn create_binary_vote_with_multi_options_should_not_work() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let public = get_test_key();
		let vote = generate_1p1v_public_binary_vote();
		let multi_vote = generate_1p1v_public_multi_vote();
		assert_err!(create_vote(public, vote.0, vote.1, vote.2, &multi_vote.3), "Invalid binary outcomes");
		assert_eq!(Voting::vote_record_count(), 0);
		assert_eq!(Voting::vote_records(1), None);
	});
}

#[test]
fn create_multi_vote_should_work() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let public = get_test_key();
		let vote = generate_1p1v_public_multi_vote();
		assert_eq!(Ok(1), create_vote(public, vote.0, vote.1, vote.2, &vote.3));
		assert_eq!(Voting::vote_record_count(), 1);
		assert_eq!(
			Voting::vote_records(1),
			Some(make_record(1, public, vote.0, vote.1, vote.2, &vote.3, VoteStage::PreVoting))
		);
	});
}

#[test]
fn create_multi_vote_with_binary_options_should_not_work() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let public = get_test_key();
		let vote = generate_1p1v_public_binary_vote();
		let multi_vote = generate_1p1v_public_multi_vote();
		assert_err!(create_vote(public, multi_vote.0, multi_vote.1, multi_vote.2, &vote.3), "Invalid multi option outcomes");
		assert_eq!(Voting::vote_record_count(), 0);
		assert_eq!(Voting::vote_records(1), None);
	});
}

#[test]
fn create_vote_with_one_outcome_should_not_work() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let public = get_test_key();
		let vote = generate_1p1v_public_multi_vote();
		let outcome: [u8; 32] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,4];
		assert_err!(create_vote(public, vote.0, vote.1, vote.2, &[outcome]), "Invalid multi option outcomes");
		assert_eq!(Voting::vote_record_count(), 0);
		assert_eq!(Voting::vote_records(1), None);
	});
}

#[test]
fn commit_to_nonexistent_record_should_not_work() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let public = get_test_key();
		let commit_value = SECRET;
		assert_err!(commit(public, 1, commit_value), "Vote record does not exist");
	});
}

#[test]
fn commit_to_non_commit_record_should_not_work() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let public = get_test_key();
		let vote = generate_1p1v_public_binary_vote();
		assert_eq!(Ok(1), create_vote(public, vote.0, vote.1, vote.2, &vote.3));
		let commit_value = SECRET;
		assert_err!(commit(public, 1, commit_value), "Commitments are not configured for this vote");
	});
}

#[test]
fn reveal_to_nonexistent_record_should_not_work() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let public = get_test_key();
		let commit_value = SECRET;
		assert_err!(reveal(public, 1, vec![commit_value], Some(commit_value)), "Vote record does not exist");
	});
}

#[test]
fn reveal_to_record_before_voting_period_should_not_work() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let public = get_test_key();
		let vote = generate_1p1v_public_binary_vote();
		assert_eq!(Ok(1), create_vote(public, vote.0, vote.1, vote.2, &vote.3));
		let vote_outcome = vote.3[0];
		assert_err!(reveal(public, 1, vec![vote_outcome], Some(vote_outcome)), "Vote is not in voting stage");
	});
}

#[test]
fn advance_from_initiator_should_work() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let public = get_test_key();
		let vote = generate_1p1v_public_binary_vote();
		assert_eq!(Ok(1), create_vote(public, vote.0, vote.1, vote.2, &vote.3));
		assert_ok!(advance_stage(1));
		assert_eq!(
			Voting::vote_records(1),
			Some(make_record(1, public, vote.0, vote.1, vote.2, &vote.3, VoteStage::Voting))
		);
	});
}

#[test]
fn reveal_should_work() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let public = get_test_key();
		let vote = generate_1p1v_public_binary_vote();
		assert_eq!(Ok(1), create_vote(public, vote.0, vote.1, vote.2, &vote.3));
		assert_ok!(advance_stage(1));
		let public2 = get_test_key_2();
		assert_ok!(reveal(public2, 1, vec![vote.3[0]], Some(vote.3[0])));
		assert_eq!(
			Voting::vote_records(1).unwrap().reveals,
			vec![(public2, vec![vote.3[0]])]
		);
	});
}

#[test]
fn reveal_invalid_outcome_should_not_work() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let public = get_test_key();
		let vote = generate_1p1v_public_binary_vote();
		assert_eq!(Ok(1), create_vote(public, vote.0, vote.1, vote.2, &vote.3));
		assert_ok!(advance_stage(1));
		let public2 = get_test_key_2();
		let invalid_outcome = SECRET;
		assert_err!(reveal(public2, 1, vec![invalid_outcome], None), "Vote outcome is not valid");
	});
}

#[test]
fn reveal_multi_outcome_should_work() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let public = get_test_key();
		let vote = generate_1p1v_public_multi_vote();
		assert_eq!(Ok(1), create_vote(public, vote.0, vote.1, vote.2, &vote.3));
		assert_ok!(advance_stage(1));

		
		for i in 0..vote.3.len() {
			assert_ok!(reveal(i as u64, 1, vec![vote.3[i]], None));
		}
	});
}

#[test]
fn complete_after_reveal_should_work() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let public = get_test_key();
		let vote = generate_1p1v_public_binary_vote();
		assert_eq!(Ok(1), create_vote(public, vote.0, vote.1, vote.2, &vote.3));
		assert_ok!(advance_stage(1));
		let public2 = get_test_key_2();
		assert_ok!(reveal(public2, 1, vec![vote.3[0]], Some(vote.3[0])));
		assert_ok!(advance_stage(1));
		assert_eq!(
			Voting::vote_records(1).unwrap().data.stage,
			VoteStage::Completed
		);
	});
}

#[test]
fn transition_to_commit_should_work() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let public = get_test_key();
		let vote = generate_1p1v_commit_reveal_binary_vote();
		assert_eq!(Ok(1), create_vote(public, vote.0, vote.1, vote.2, &vote.3));
		assert_eq!(
			Voting::vote_records(1).unwrap().data.is_commit_reveal,
			true
		);
		assert_ok!(advance_stage(1));
		assert_eq!(
			Voting::vote_records(1).unwrap().data.stage,
			VoteStage::Commit
		);
	});
}

#[test]
fn reveal_before_commit_should_not_work() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let public = get_test_key();
		let vote = generate_1p1v_commit_reveal_binary_vote();
		assert_eq!(Ok(1), create_vote(public, vote.0, vote.1, vote.2, &vote.3));
		assert_eq!(
			Voting::vote_records(1).unwrap().data.is_commit_reveal,
			true
		);
		let public2 = get_test_key_2();
		assert_err!(reveal(public2, 1, vec![vote.3[0]], Some(vote.3[0])), "Vote is not in voting stage");
	});
}

#[test]
fn reveal_commit_before_stage_change_should_not_work() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let public = get_test_key();
		let vote = generate_1p1v_commit_reveal_binary_vote();
		assert_eq!(Ok(1), create_vote(public, vote.0, vote.1, vote.2, &vote.3));
		assert_ok!(advance_stage(1));
		let public2 = get_test_key_2();
		let secret = SECRET;
		let mut buf = Vec::new();
		buf.extend_from_slice(&public2.encode());
		buf.extend_from_slice(&secret);
		buf.extend_from_slice(&vote.3[0]);
		let commit_hash: [u8; 32] = BlakeTwo256::hash_of(&buf).into();
		assert_ok!(commit(public2, 1, commit_hash));
		assert_eq!(
			Voting::vote_records(1).unwrap().commitments,
			vec![(public2, commit_hash)]
		);

		assert_err!(reveal(public2, 1, vec![vote.3[0]], Some(secret)), "Vote is not in voting stage");
	});
}

#[test]
fn reveal_commit_should_work() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let public = get_test_key();
		let vote = generate_1p1v_commit_reveal_binary_vote();
		assert_eq!(Ok(1), create_vote(public, vote.0, vote.1, vote.2, &vote.3));
		assert_ok!(advance_stage(1));
		let public2 = get_test_key_2();
		let secret = SECRET;
		let mut buf = Vec::new();
		buf.extend_from_slice(&public2.encode());
		buf.extend_from_slice(&secret);
		buf.extend_from_slice(&vote.3[0]);
		let commit_hash: [u8; 32] = BlakeTwo256::hash_of(&buf).into();
		assert_ok!(commit(public2, 1, commit_hash));
		assert_eq!(
			Voting::vote_records(1).unwrap().commitments,
			vec![(public2, commit_hash)]
		);

		assert_ok!(advance_stage(1));
		assert_ok!(reveal(public2, 1, vec![vote.3[0]], Some(secret)));
	});
}

#[test]
fn create_public_ranked_choice_vote_should_work() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let public = get_test_key();
		let vote = generate_1p1v_public_ranked_choice_vote();
		assert_eq!(Ok(1), create_vote(public, vote.0, vote.1, vote.2, &vote.3));
		assert_eq!(Voting::vote_record_count(), 1);
		assert_eq!(
			Voting::vote_records(1),
			Some(make_record(1, public, vote.0, vote.1, vote.2, &vote.3, VoteStage::PreVoting))
		);

		assert_ok!(advance_stage(1));
		assert_eq!(
			Voting::vote_records(1),
			Some(make_record(1, public, vote.0, vote.1, vote.2, &vote.3, VoteStage::Voting))
		);
	});
}

#[test]
fn reveal_public_ranked_choice_vote_should_work() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let public = get_test_key();
		let vote = generate_1p1v_public_ranked_choice_vote();
		assert_eq!(Ok(1), create_vote(public, vote.0, vote.1, vote.2, &vote.3));
		assert_ok!(advance_stage(1));
		assert_ok!(reveal(public, 1, vote.3.to_vec(), None));
	});
}

#[test]
fn reveal_incorrect_outcomes_ranked_choice_should_fail() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let public = get_test_key();
		let vote = generate_1p1v_public_ranked_choice_vote();
		assert_eq!(Ok(1), create_vote(public, vote.0, vote.1, vote.2, &vote.3));
		assert_ok!(advance_stage(1));
		assert_err!(reveal(public, 1, vec![vote.3[0]], None), "Ranked choice vote invalid");
	});
}

#[test]
fn commit_reveal_ranked_choice_vote_should_work() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let public = get_test_key();
		let vote = generate_1p1v_commit_reveal_ranked_choice_vote();
		assert_eq!(Ok(1), create_vote(public, vote.0, vote.1, vote.2, &vote.3));
		assert_ok!(advance_stage(1));

		let mut buf = vec![];
		buf.extend_from_slice(&public.encode());
		buf.extend_from_slice(&SECRET.encode());
		for i in 0..vote.3.len() {
			buf.extend_from_slice(&vote.3[i].encode());
		}
		let hash = BlakeTwo256::hash_of(&buf);
		assert_ok!(commit(public, 1, hash.into()));
		assert_ok!(advance_stage(1));
		assert_ok!(reveal(public, 1, vote.3.to_vec(), Some(SECRET)));
	});
}

#[test]
fn change_hasher_migration() {
	mod deprecated {
		use sp_std::prelude::*;

		use codec::{Encode, Decode};
		use frame_support::{decl_module, decl_storage};

		use crate::{Trait, VoteRecord};

		decl_module! {
			pub struct Module<T: Trait> for enum Call where origin: T::Origin { }
		}
		decl_storage! {
			trait Store for Module<T: Trait> as Voting {
				pub VoteRecords get(fn vote_records): map hasher(opaque_blake2_256)
					u64 => Option<VoteRecord<T::AccountId>>;
			}
		}
	}

	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		// build vote record
		let public = get_test_key();
		let yes_vote: [u8; 32] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1];
		let no_vote: [u8; 32] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
		let outcomes = vec![yes_vote, no_vote];
		let id = VoteRecordCount::get() + 1;
		let record = VoteRecord {
			id: id,
			commitments: vec![],
			reveals: vec![],
			outcomes: outcomes,
			data: VoteData {
				initiator: public.clone(),
				stage: VoteStage::PreVoting,
				vote_type: VoteType::Binary,
				tally_type: TallyType::OneCoin,
				is_commit_reveal: false,
			},
		};

		// insert the record with the old hasher
		deprecated::VoteRecords::<Test>::insert(id, &record);
		VoteRecordCount::mutate(|i| *i += 1);
		assert!(
			Voting::vote_records(id).is_none(),
			"proposal should not (yet) be available with the new hasher"
		);
		// do the migration
		crate::migration::migrate::<Test>();
		let maybe_vote = Voting::vote_records(id);
		// check that it was successfull
		assert!(maybe_vote.is_some());
		let vote = maybe_vote.unwrap();
		assert_eq!(vote, record);
	});
}
