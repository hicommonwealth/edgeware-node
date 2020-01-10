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

use super::*;
use sp_runtime::{
	Perbill,
	testing::Header,
	traits::{IdentityLookup, OnFinalize},
};
use sp_core::H256;
use frame_support::{parameter_types, impl_outer_origin, assert_err};

use frame_support::{
	assert_ok, dispatch::DispatchResult
};

use sp_core::{Blake2Hasher, Hasher};

pub use crate::{Event, Module, RawEvent, Trait, IdentityStage, MetadataRecord, IdentityRecord, GenesisConfig};

impl_outer_origin! {
	pub enum Origin for Test {}
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Test;

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: u32 = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::one();
}

impl frame_system::Trait for Test {
	type Origin = Origin;
	type Index = u64;
	type BlockNumber = u64;
	type Call = ();
	type Hash = H256;
	type Hashing = ::sp_runtime::traits::BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = ();
	type BlockHashCount = BlockHashCount;
	type MaximumBlockWeight = MaximumBlockWeight;
	type MaximumBlockLength = MaximumBlockLength;
	type AvailableBlockRatio = AvailableBlockRatio;
	type Version = ();
	type ModuleToIndex = ();
}

parameter_types! {
	pub const ExistentialDeposit: u128 = 0;
	pub const TransferFee: u128 = 0;
	pub const CreationFee: u128 = 0;
}

impl pallet_balances::Trait for Test {
	/// The type for recording an account's balance.
	type Balance = u128;
	/// What to do if an account's free balance gets zeroed.
	type OnFreeBalanceZero = ();
	/// What to do if a new account is created.
	type OnNewAccount = ();
	/// The ubiquitous event type.
	type Event = ();
	type DustRemoval = ();
	type TransferPayment = ();
	type ExistentialDeposit = ExistentialDeposit;
	type TransferFee = TransferFee;
	type CreationFee = CreationFee;
}

impl Trait for Test {
	type Event = ();
	type Currency = pallet_balances::Module<Self>;
}

type Balances = pallet_balances::Module<Test>;
type System = frame_system::Module<Test>;
type Identity = Module<Test>;

const BOND: u128 = 10;
// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	// We use default for brevity, but you can configure as desired if needed.
	GenesisConfig::<Test> {
		expiration_length: 10000,
		verifiers: vec![1_u64],
		registration_bond: BOND,
	}.assimilate_storage(&mut t).unwrap();
	pallet_balances::GenesisConfig::<Test> {
		balances: vec![
			(1, 100),
			(2, 100),
			(3, 100),
			(4, 100),
		],
		vesting: vec![],
	}.assimilate_storage(&mut t).unwrap();
	t.into()
}

fn register_identity(who: u64, identity_type: &[u8], identity: &[u8]) -> DispatchResult {
	Identity::register(Origin::signed(who), identity_type.to_vec(), identity.to_vec())
}

fn attest_to_identity(who: u64, identity_hash: H256, attestation: &[u8]) -> DispatchResult {
	Identity::attest(Origin::signed(who), identity_hash, attestation.to_vec())
}

fn register_and_attest(who: u64, identity_type: &[u8], identity: &[u8], attestation: &[u8]) -> DispatchResult {
	Identity::register_and_attest(Origin::signed(who), identity_type.to_vec(), identity.to_vec(), attestation.to_vec())
}

fn verify_identity(who: u64, identity_hash: H256, verifier_index: u32) -> DispatchResult {
	Identity::verify(Origin::signed(who), identity_hash, verifier_index)
}

fn deny_many(who: u64, identity_hashes: &[H256], verifier_index: u32) -> DispatchResult {
	Identity::deny_many(Origin::signed(who), identity_hashes.to_vec(), verifier_index)
}

fn revoke(who: u64, identity_hash: H256) -> DispatchResult {
	Identity::revoke(Origin::signed(who), identity_hash)
}

fn add_metadata_to_account(
	who: u64,
	identity_hash: H256,
	avatar: &[u8],
	display_name: &[u8],
	tagline: &[u8],
) -> DispatchResult {
	Identity::add_metadata(
		Origin::signed(who),
		identity_hash,
		avatar.to_vec(),
		display_name.to_vec(),
		tagline.to_vec(),
	)
}

fn default_identity_record(public: u64, identity_type: &[u8], identity: &[u8]) -> IdentityRecord<u64, u64> {
	IdentityRecord {
		account: public,
		identity_type: identity_type.to_vec(),
		identity: identity.to_vec(),
		stage: IdentityStage::Registered,
		expiration_length: 10001,
		proof: None,
		metadata: None
	}
}

fn build_identity_hash(identity_type: &[u8], identity: &[u8]) -> H256 {
		let mut buf = Vec::new();
		buf.extend_from_slice(&identity_type.to_vec().encode());
		buf.extend_from_slice(&identity.to_vec().encode());
		return Blake2Hasher::hash(&buf[..]);
}

#[test]
fn register_should_work() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let identity_type: &[u8] = b"github";
		let identity: &[u8] = b"drewstone";
		let identity_hash = build_identity_hash(identity_type, identity);

		let public = 1_u64;

		let expiration_length = Identity::expiration_length();
		let now = System::block_number();
		let _expires_at = now + expiration_length;

		let balance = Balances::free_balance(public);
		assert_ok!(register_identity(public, identity_type, identity));
		let after_register_balance = Balances::free_balance(public);
		assert_eq!(balance - BOND, after_register_balance);
		assert_eq!(Identity::identities(), vec![identity_hash]);
		assert_eq!(Identity::identities_pending(), vec![(identity_hash, 10001)]);
		assert_eq!(
			Identity::identity_of(identity_hash),
			Some(default_identity_record(public, identity_type, identity))
		);
	});
}

#[test]
fn register_twice_should_not_work() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let identity_type: &[u8] = b"github";
		let identity: &[u8] = b"drewstone";
		let identity_hash = build_identity_hash(identity_type, identity);
		let public = 1_u64;

		assert_ok!(register_identity(public, identity_type, identity));
		assert_err!(
			register_identity(public, identity_type, identity),
			"Identity type already used"
		);
		assert_eq!(Identity::identities(), vec![identity_hash]);
		assert_eq!(Identity::identities_pending(), vec![(identity_hash, 10001)]);
		assert_eq!(
			Identity::identity_of(identity_hash),
			Some(default_identity_record(public, identity_type, identity))
		);
	});
}

#[test]
fn register_existing_identity_should_not_work() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let identity_type: &[u8] = b"github";
		let identity: &[u8] = b"drewstone";
		let identity_hash = build_identity_hash(identity_type, identity);
		let public = 1_u64;
		let public2 = 2_u64;

		assert_ok!(register_identity(public, identity_type, identity));
		assert_err!(
			register_identity(public2, identity_type, identity),
			"Identity already exists"
		);
		assert_eq!(Identity::identities(), vec![identity_hash]);
		assert_eq!(Identity::identities_pending(), vec![(identity_hash, 10001)]);
		assert_eq!(
			Identity::identity_of(identity_hash),
			Some(default_identity_record(public, identity_type, identity))
		);
	});
}

#[test]
fn register_same_type_should_not_work() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let identity_type: &[u8] = b"github";
		let identity: &[u8] = b"drewstone";
		let identity_hash = build_identity_hash(identity_type, identity);
		let public = 1_u64;

		assert_ok!(register_identity(public, identity_type, identity));

		let new_github: &[u8] = b"drstone";
		assert_err!(
			register_identity(public, identity_type, new_github),
			"Identity type already used"
		);
		assert_eq!(Identity::identities(), vec![identity_hash]);
		assert_eq!(Identity::identities_pending(), vec![(identity_hash, 10001)]);
		assert_eq!(
			Identity::identity_of(identity_hash),
			Some(default_identity_record(public, identity_type, identity))
		);
	});
}

#[test]
fn register_and_attest_should_work() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let identity_type: &[u8] = b"github";
		let identity: &[u8] = b"drewstone";
		let identity_hash = build_identity_hash(identity_type, identity);

		let public = 1_u64;

		assert_ok!(register_identity(public, identity_type, identity));

		let mut expiration_length = Identity::expiration_length();
		let mut now = System::block_number();
		let _register_expires_at = now + expiration_length;

		let attestation: &[u8] = b"www.proof.com/attest_of_extra_proof";
		assert_ok!(attest_to_identity(public, identity_hash, attestation));

		expiration_length = Identity::expiration_length();
		now = System::block_number();
		let _attest_expires_at = now + expiration_length;
		assert_eq!(Identity::identities(), vec![identity_hash]);
		assert_eq!(Identity::identities_pending(), vec![(identity_hash, 10001)]);
		assert_eq!(
			Identity::identity_of(identity_hash),
			Some(IdentityRecord {
				stage: IdentityStage::Attested,
				proof: Some(attestation.to_vec()),
				..default_identity_record(public, identity_type, identity)
			})
		);
	});
}


#[test]
fn register_and_attest_as_one_should_work() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let identity_type: &[u8] = b"github";
		let identity: &[u8] = b"drewstone";
		let identity_hash = build_identity_hash(identity_type, identity);

		let public = 1_u64;

		let mut expiration_length = Identity::expiration_length();
		let mut now = System::block_number();
		let _register_expires_at = now + expiration_length;

		let attestation: &[u8] = b"www.proof.com/attest_of_extra_proof";
		assert_ok!(register_and_attest(public, identity_type, identity, attestation));

		expiration_length = Identity::expiration_length();
		now = System::block_number();
		let _attest_expires_at = now + expiration_length;

		assert_eq!(Identity::identities(), vec![identity_hash]);
		assert_eq!(Identity::identities_pending(), vec![(identity_hash, 10001)]);
		assert_eq!(
			Identity::identity_of(identity_hash),
			Some(IdentityRecord {
				stage: IdentityStage::Attested,
				proof: Some(attestation.to_vec()),
				..default_identity_record(public, identity_type, identity)
			})
		);
	});
}

#[test]
fn attest_without_register_should_not_work() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let identity_type: &[u8] = b"github";
		let identity: &[u8] = b"drewstone";
		let identity_hash = build_identity_hash(identity_type, identity);
		let public = 1_u64;

		let attestation: &[u8] = b"www.proof.com/attest_of_extra_proof";
		assert_err!(
			attest_to_identity(public, identity_hash, attestation),
			"Identity does not exist"
		);
		assert_eq!(Identity::identities(), vec![]);
		assert_eq!(Identity::identities_pending(), vec![]);
		assert_eq!(Identity::identity_of(identity_hash), None);
	});
}

#[test]
fn attest_from_different_account_should_not_work() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		let identity_type: &[u8] = b"github";
		let identity: &[u8] = b"drewstone";
		let identity_hash = build_identity_hash(identity_type, identity);
		let public = 1_u64;
		let other_pub = 2_u64;

		assert_ok!(register_identity(public, identity_type, identity));
		let attestation: &[u8] = b"www.proof.com/attest_of_extra_proof";
		assert_err!(
			attest_to_identity(other_pub, identity_hash, attestation),
			"Stored identity does not match sender"
		);
		assert_eq!(Identity::identities(), vec![identity_hash]);
		assert_eq!(Identity::identities_pending(), vec![(identity_hash, 10001)]);
		assert_eq!(
			Identity::identity_of(identity_hash),
			Some(default_identity_record(public, identity_type, identity))
		);
	});
}

#[test]
fn register_attest_and_verify_should_work() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let identity_type: &[u8] = b"github";
		let identity: &[u8] = b"drewstone";
		let identity_hash = build_identity_hash(identity_type, identity);

		let public = 1_u64;

		let balance = Balances::free_balance(public);
		println!("{:?}", balance);
		assert_ok!(register_identity(public, identity_type, identity));
		let after_register_balance = Balances::free_balance(public);
		println!("{:?}", after_register_balance);
		assert_eq!(balance - BOND, after_register_balance);

		let mut expiration_length = Identity::expiration_length();
		let mut now = System::block_number();
		let _register_expires_at = now + expiration_length;

		let attestation: &[u8] = b"www.proof.com/attest_of_extra_proof";
		assert_ok!(attest_to_identity(public, identity_hash, attestation));

		expiration_length = Identity::expiration_length();
		now = System::block_number();
		let _attest_expires_at = now + expiration_length;

		System::set_block_number(2);
		let verifier = 1_u64;
		assert_ok!(verify_identity(verifier, identity_hash, 0));
		let balance_after_verify = Balances::free_balance(public);
		assert_eq!(balance, balance_after_verify);

		assert_eq!(Identity::identities(), vec![identity_hash]);
		assert_eq!(Identity::identities_pending(), vec![]);
		assert_eq!(
			Identity::identity_of(identity_hash),
			Some(IdentityRecord {
				stage: IdentityStage::Verified,
				expiration_length: 0,
				proof: Some(attestation.to_vec()),
				..default_identity_record(public, identity_type, identity)
			})
		);
	});
}

#[test]
fn deny_many_should_work() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let mut id_hashes = vec![];
		let _test_id_type: &[u8] = b"github";
		let _test_id: Vec<u8> = "drewstone 4".as_bytes().to_vec();
		for i in 1..5 {
			let identity_type: &[u8] = b"github";
			let identity: Vec<u8> = format!("drewstone {}", i).as_bytes().to_vec();
			let identity_hash = build_identity_hash(identity_type, &identity);	
			assert_ok!(register_identity(i as u64, identity_type, &identity));
			let attestation: &[u8] = b"this_is_a_fake_attestation";
			assert_ok!(attest_to_identity(i as u64, identity_hash, attestation));
			id_hashes.push(identity_hash);
		}

		let verifier = 1_u64;
		assert_ok!(deny_many(verifier, &id_hashes, 0));
	});
}

#[test]
fn attest_after_verify_should_not_work() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let identity_type: &[u8] = b"github";
		let identity: &[u8] = b"drewstone";
		let identity_hash = build_identity_hash(identity_type, identity);

		let public = 1_u64;

		assert_ok!(register_identity(public, identity_type, identity));

		let attestation: &[u8] = b"www.proof.com/attest_of_extra_proof";
		assert_ok!(attest_to_identity(public, identity_hash, attestation));

		let verifier = 1_u64;
		assert_ok!(verify_identity(verifier, identity_hash, 0));
		assert_err!(
			attest_to_identity(public, identity_hash, attestation),
			"Already verified"
		);
		assert_eq!(Identity::identities(), vec![identity_hash]);
		assert_eq!(Identity::identities_pending(), vec![]);
		assert_eq!(
			Identity::identity_of(identity_hash),
			Some(IdentityRecord {
				stage: IdentityStage::Verified,
				expiration_length: 0,
				proof: Some(attestation.to_vec()),
				..default_identity_record(public, identity_type, identity)
			})
		);
	});
}

#[test]
fn verify_from_nonverifier_should_not_work() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let identity_type: &[u8] = b"github";
		let identity: &[u8] = b"drewstone";
		let identity_hash = build_identity_hash(identity_type, identity);

		let public = 2_u64;

		assert_ok!(register_identity(public, identity_type, identity));

		let attestation: &[u8] = b"www.proof.com/attest_of_extra_proof";
		assert_ok!(attest_to_identity(public, identity_hash, attestation));

		assert_err!(
			verify_identity(public, identity_hash, 0),
			"Sender is not a verifier"
		);
		assert_eq!(Identity::identities(), vec![identity_hash]);
		assert_eq!(Identity::identities_pending(), vec![(identity_hash, 10001)]);
		assert_eq!(
			Identity::identity_of(identity_hash),
			Some(IdentityRecord {
				stage: IdentityStage::Attested,
				proof: Some(attestation.to_vec()),
				..default_identity_record(public, identity_type, identity)
			})
		);
	});
}

#[test]
fn verify_from_wrong_index_should_not_work() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let identity_type: &[u8] = b"github";
		let identity: &[u8] = b"drewstone";
		let identity_hash = build_identity_hash(identity_type, identity);

		let public = 1_u64;

		assert_ok!(register_identity(public, identity_type, identity));

		let attestation: &[u8] = b"www.proof.com/attest_of_extra_proof";
		assert_ok!(attest_to_identity(public, identity_hash, attestation));

		assert_err!(
			verify_identity(public, identity_hash, 1),
			"Verifier index out of bounds"
		);
		assert_eq!(Identity::identities(), vec![identity_hash]);
		assert_eq!(Identity::identities_pending(), vec![(identity_hash, 10001)]);
		assert_eq!(
			Identity::identity_of(identity_hash),
			Some(IdentityRecord {
				stage: IdentityStage::Attested,
				proof: Some(attestation.to_vec()),
				..default_identity_record(public, identity_type, identity)
			})
		);
	});
}

#[test]
fn register_should_expire() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let identity_type: &[u8] = b"github";
		let identity: &[u8] = b"drewstone";
		let identity_hash = build_identity_hash(identity_type, identity);

		let public = 1_u64;

		assert_ok!(register_identity(public, identity_type, identity));

		let expiration_length = Identity::expiration_length();
		let now = System::block_number();
		let _expires_at = now + expiration_length;

		System::set_block_number(10002);
		<Identity as OnFinalize<u64>>::on_finalize(10002);
		System::set_block_number(10003);

		let attestation: &[u8] = b"www.proof.com/attest_of_extra_proof";
		assert_err!(
			attest_to_identity(public, identity_hash, attestation),
			"Identity does not exist"
		);

		assert_eq!(Identity::identities(), vec![]);
		assert_eq!(Identity::identities_pending(), vec![]);
		assert_eq!(Identity::identity_of(identity_hash), None);
	});
}

#[test]
fn add_metadata_should_work() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let identity_type: &[u8] = b"github";
		let identity: &[u8] = b"drewstone";
		let identity_hash = build_identity_hash(identity_type, identity);

		let public = 1_u64;

		let avatar: &[u8] = b"avatars3.githubusercontent.com/u/13153687";
		let display_name: &[u8] = b"drewstone";
		let tagline: &[u8] = b"hello world!";

		assert_ok!(register_identity(public, identity_type, identity));
		assert_ok!(add_metadata_to_account(
			public,
			identity_hash,
			avatar,
			display_name,
			tagline
		));
		assert_eq!(Identity::identities(), vec![identity_hash]);
		assert_eq!(Identity::identities_pending(), vec![(identity_hash, 10001)]);
		let default_record = default_identity_record(public, identity_type, identity);
		assert_eq!(
			Identity::identity_of(identity_hash),
			Some(IdentityRecord {
				metadata: Some(MetadataRecord {
				avatar: avatar.to_vec(),
				display_name: display_name.to_vec(),
				tagline: tagline.to_vec(),
				}),
				..default_record
			})
		);
	});
}

#[test]
fn add_metadata_without_register_should_not_work() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let identity_type: &[u8] = b"github";
		let identity: &[u8] = b"drewstone";
		let identity_hash = build_identity_hash(identity_type, identity);
		let public = 1_u64;

		let avatar: &[u8] = b"avatars3.githubusercontent.com/u/13153687";
		let display_name: &[u8] = b"drewstone";
		let tagline: &[u8] = b"hello world!";
		assert_err!(
			add_metadata_to_account(public, identity_hash, avatar, display_name, tagline),
			"Identity does not exist"
		);
		assert_eq!(Identity::identities(), vec![]);
		assert_eq!(Identity::identities_pending(), vec![]);
		assert_eq!(Identity::identity_of(identity_hash), None);
	});
}

#[test]
fn add_metadata_from_different_account_should_not_work() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		let identity_type: &[u8] = b"github";
		let identity: &[u8] = b"drewstone";
		let identity_hash = build_identity_hash(identity_type, identity);
		let public = 1_u64;
		let other_pub = 2_u64;

		let avatar: &[u8] = b"avatars3.githubusercontent.com/u/13153687";
		let display_name: &[u8] = b"drewstone";
		let tagline: &[u8] = b"hello world!";

		assert_ok!(register_identity(public, identity_type, identity));
		assert_err!(
			add_metadata_to_account(other_pub, identity_hash, avatar, display_name, tagline),
			"Stored identity does not match sender"
		);
		assert_eq!(Identity::identities(), vec![identity_hash]);
		assert_eq!(Identity::identities_pending(), vec![(identity_hash, 10001)]);
		assert_eq!(
			Identity::identity_of(identity_hash),
			Some(default_identity_record(public, identity_type, identity))
		);
	});
}

#[test]
fn revoke_verified_should_work() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let identity_type: &[u8] = b"github";
		let identity: &[u8] = b"drewstone";
		let identity_hash = build_identity_hash(identity_type, identity);
		let public = 1_u64;
		// Register
		assert_ok!(register_identity(public, identity_type, identity));
		// Attest
		let attestation: &[u8] = b"www.proof.com/attest_of_extra_proof";
		assert_ok!(attest_to_identity(public, identity_hash, attestation));

		System::set_block_number(2);
		let verifier = 1_u64;
		assert_ok!(verify_identity(verifier, identity_hash, 0));
		assert_eq!(
			Identity::identity_of(identity_hash),
			Some(IdentityRecord {
				stage: IdentityStage::Verified,
				expiration_length: 0,
				proof: Some(attestation.to_vec()),
				..default_identity_record(public, identity_type, identity)
			})
		);
		assert_ok!(revoke(public, identity_hash));
		assert_eq!(
			Identity::identity_of(identity_hash),
			None,
		);
	});
}

#[test]
fn revoke_from_wrong_sender_should_not_work() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let identity_type: &[u8] = b"github";
		let identity: &[u8] = b"drewstone";
		let identity_hash = build_identity_hash(identity_type, identity);
		let public = 1_u64;
		// Register
		assert_ok!(register_identity(public, identity_type, identity));
		// Attest
		let attestation: &[u8] = b"www.proof.com/attest_of_extra_proof";
		assert_ok!(attest_to_identity(public, identity_hash, attestation));

		System::set_block_number(2);
		let verifier = 1_u64;
		assert_ok!(verify_identity(verifier, identity_hash, 0));
		assert_err!(revoke(2_u64, identity_hash), "Stored identity does not match sender");
	});
}

#[test]
fn revoke_at_registered_should_work() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let identity_type: &[u8] = b"github";
		let identity: &[u8] = b"drewstone";
		let identity_hash = build_identity_hash(identity_type, identity);
		let public = 1_u64;
		// Register
		assert_ok!(register_identity(public, identity_type, identity));
		assert_eq!(
			Identity::identity_of(identity_hash),
			Some(default_identity_record(public, identity_type, identity))
		);
		assert_ok!(revoke(public, identity_hash));
		assert_eq!(
			Identity::identity_of(identity_hash),
			None,
		);
	});
}

#[test]
fn revoke_at_attested_should_work() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let identity_type: &[u8] = b"github";
		let identity: &[u8] = b"drewstone";
		let identity_hash = build_identity_hash(identity_type, identity);
		let public = 1_u64;
		// Register
		assert_ok!(register_identity(public, identity_type, identity));
		// Attest
		let attestation: &[u8] = b"www.proof.com/attest_of_extra_proof";
		assert_ok!(attest_to_identity(public, identity_hash, attestation));
		assert_eq!(
			Identity::identity_of(identity_hash),
			Some(IdentityRecord {
				stage: IdentityStage::Attested,
				proof: Some(attestation.to_vec()),
				..default_identity_record(public, identity_type, identity)
			})
		);
		assert_ok!(revoke(public, identity_hash));
		assert_eq!(
			Identity::identity_of(identity_hash),
			None,
		);
	});
}

#[test]
fn register_should_expire_and_work_again() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let identity_type: &[u8] = b"github";
		let identity: &[u8] = b"drewstone";
		let identity_hash = build_identity_hash(identity_type, identity);

		let public = 1_u64;

		assert_ok!(register_identity(public, identity_type, identity));
		System::set_block_number(10002);
		<Identity as OnFinalize<u64>>::on_finalize(10002);
		System::set_block_number(10003);
		assert_eq!(Identity::identity_of(identity_hash), None);
		assert_ok!(register_identity(public, identity_type, identity));
	});
}