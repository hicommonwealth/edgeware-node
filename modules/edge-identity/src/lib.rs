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
extern crate serde_derive;
#[cfg(test)]
#[macro_use]
extern crate hex_literal;
#[macro_use]
extern crate parity_codec_derive;
#[macro_use]
extern crate srml_support;

extern crate parity_codec as codec;
extern crate sr_io as runtime_io;
extern crate sr_primitives as runtime_primitives;
extern crate sr_std as rstd;
extern crate srml_support as runtime_support;
extern crate substrate_primitives as primitives;

extern crate srml_system as system;
extern crate srml_timestamp as timestamp;
extern crate srml_consensus as consensus;

pub mod identity;
pub use identity::{
	Event, Module, RawEvent, Trait,
	IdentityStage, IdentityRecord, MetadataRecord
};

// Tests for Identity Module
#[cfg(test)]
mod tests {
	use super::*;

	use primitives::{Blake2Hasher, H256};
	use rstd::prelude::*;
	use runtime_io::ed25519::Pair;
	use runtime_io::with_externalities;
	use runtime_support::dispatch::Result;
	use system::{EventRecord, Phase};
	// The testing primitives are very useful for avoiding having to work with
	// public keys. `u64` is used as the `AccountId` and no `Signature`s are requried.
	use runtime_primitives::{
		testing::{Digest, DigestItem, Header, UintAuthorityId},
		traits::{BlakeTwo256, Hash},
		BuildStorage,
	};

	impl_outer_origin! {
		pub enum Origin for Test {}
	}

	impl_outer_event! {
		pub enum Event for Test {
			identity<T>,
		}
	}

	impl_outer_dispatch! {
		pub enum Call for Test where origin: Origin { }
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
		type Digest = Digest;
		type AccountId = H256;
		type Header = Header;
		type Event = Event;
		type Log = DigestItem;
	}
	impl consensus::Trait for Test {
		const NOTE_OFFLINE_POSITION: u32 = 1;
		type Log = DigestItem;
		type SessionKey = UintAuthorityId;
		type InherentOfflineReport = ();
	}
	impl timestamp::Trait for Test {
		const TIMESTAMP_SET_POSITION: u32 = 0;
		type Moment = u64;
		type OnTimestampSet = ();
	}
	impl Trait for Test {
		type Claim = Vec<u8>;
		type Event = Event;
	}

	type Timestamp = timestamp::Module<Test>;
	type System = system::Module<Test>;
	type Identity = Module<Test>;

	// This function basically just builds a genesis storage key/value store according to
	// our desired mockup.
	fn new_test_ext(verifiers: Vec<H256>) -> sr_io::TestExternalities<Blake2Hasher> {
		let mut t = system::GenesisConfig::<Test>::default()
			.build_storage()
			.unwrap()
			.0;
		// We use default for brevity, but you can configure as desired if needed.
		t.extend(
			identity::GenesisConfig::<Test> {
				expiration_time: 10000,
				verifiers: verifiers,
				claims_issuers: [H256::from(1), H256::from(2), H256::from(3)].to_vec(),
			}
			.build_storage()
			.unwrap()
			.0,
		);
		t.into()
	}

	fn register_identity(who: H256, identity: &[u8]) -> Result {
		Identity::register(Origin::signed(who), identity.to_vec())
	}

	fn attest_to_identity(who: H256, identity_hash: H256, attestation: &[u8]) -> Result {
		Identity::attest(Origin::signed(who), identity_hash, attestation.to_vec())
	}

	fn verify_identity(who: H256, identity_hash: H256, vote: bool, verifier_index: usize) -> Result {
		Identity::verify(Origin::signed(who), identity_hash, vote, verifier_index)
	}

	fn remove_expired_identity(who: H256, identity_hash: H256) -> Result {
		Identity::remove_expired_identity(Origin::signed(who), identity_hash)
	}

	fn add_metadata_to_account(
		who: H256,
		identity_hash: H256,
		avatar: &[u8],
		display_name: &[u8],
		tagline: &[u8],
	) -> Result {
		Identity::add_metadata(
			Origin::signed(who),
			identity_hash,
			avatar.to_vec(),
			display_name.to_vec(),
			tagline.to_vec(),
		)
	}

	fn add_claim_to_identity(who: H256, identity_hash: H256, claim: &[u8], issuer_index: usize) -> Result {
		Identity::add_claim(Origin::signed(who), identity_hash, claim.to_vec(), issuer_index)
	}

	fn remove_claim_from_identity(who: H256, identity_hash: H256, claim_index: usize) -> Result {
		Identity::remove_claim(Origin::signed(who), identity_hash, claim_index)
	}

	fn default_identity_record(public: H256, identity: &[u8]) -> IdentityRecord<H256, u64> {
		IdentityRecord {
			account: public,
			identity: identity.to_vec(),
			stage: IdentityStage::Registered,
			expiration_time: 10000,
			proof: None,
			verifications: [0, 0],
			metadata: None,
		}
	}

	#[test]
	fn register_should_work() {
		with_externalities(&mut new_test_ext([H256::from(9)].to_vec()), || {
			System::set_block_number(1);

			let pair: Pair = Pair::from_seed(&hex!(
				"9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
			));
			let identity: &[u8] = b"github.com/drewstone";
			let identity_hash = BlakeTwo256::hash_of(&identity.to_vec());

			let public: H256 = pair.public().0.into();

			let expiration_time = Identity::expiration_time();
			let now = Timestamp::get();
			let expires_at = now + expiration_time;

			assert_ok!(register_identity(public, identity));
			assert_eq!(
				System::events(),
				vec![EventRecord {
					phase: Phase::ApplyExtrinsic(0),
					event: Event::identity(RawEvent::Register(identity_hash, public, expires_at))
				}]
			);
			assert_eq!(
				Identity::identity_of(identity_hash),
				Some(default_identity_record(public, identity))
			);
		});
	}

	#[test]
	fn register_twice_should_not_work() {
		with_externalities(&mut new_test_ext([H256::from(9)].to_vec()), || {
			System::set_block_number(1);

			let pair: Pair = Pair::from_seed(&hex!(
				"9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
			));
			let identity: &[u8] = b"github.com/drewstone";
			let identity_hash = BlakeTwo256::hash_of(&identity.to_vec());
			let public: H256 = pair.public().0.into();

			assert_ok!(register_identity(public, identity));
			assert_err!(
				register_identity(public, identity),
				"Identity already exists"
			);
			assert_eq!(
				Identity::identity_of(identity_hash),
				Some(default_identity_record(public, identity))
			);
		});
	}

	#[test]
	fn register_and_attest_should_work() {
		with_externalities(&mut new_test_ext([H256::from(9)].to_vec()), || {
			System::set_block_number(1);

			let pair: Pair = Pair::from_seed(&hex!(
				"9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
			));
			let identity: &[u8] = b"github.com/drewstone";
			let identity_hash = BlakeTwo256::hash_of(&identity.to_vec());

			let public: H256 = pair.public().0.into();

			assert_ok!(register_identity(public, identity));

			let mut expiration_time = Identity::expiration_time();
			let mut now = Timestamp::get();
			let registration_expires_at = now + expiration_time;

			let attestation: &[u8] = b"www.proof.com/attest_of_extra_proof";
			assert_ok!(attest_to_identity(public, identity_hash, attestation));

			expiration_time = Identity::expiration_time();
			now = Timestamp::get();
			let attest_expires_at = now + expiration_time;

			assert_eq!(
				System::events(),
				vec![
					EventRecord {
						phase: Phase::ApplyExtrinsic(0),
						event: Event::identity(RawEvent::Register(identity_hash, public, registration_expires_at))
					},
					EventRecord {
						phase: Phase::ApplyExtrinsic(0),
						event: Event::identity(RawEvent::Attest(identity_hash, public, attest_expires_at))
					}
				]
			);
			assert_eq!(
				Identity::identity_of(identity_hash),
				Some(IdentityRecord {
					stage: IdentityStage::Attested,
					proof: Some(attestation.to_vec()),
					..default_identity_record(public, identity)
				})
			);
		});
	}

	#[test]
	fn attest_without_register_should_not_work() {
		with_externalities(&mut new_test_ext([H256::from(9)].to_vec()), || {
			System::set_block_number(1);

			let pair: Pair = Pair::from_seed(&hex!(
				"9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
			));
			let identity: &[u8] = b"github.com/drewstone";
			let identity_hash = BlakeTwo256::hash_of(&identity.to_vec());
			let public: H256 = pair.public().0.into();

			let attestation: &[u8] = b"www.proof.com/attest_of_extra_proof";
			assert_err!(
				attest_to_identity(public, identity_hash, attestation),
				"Identity does not exist"
			);
			assert_eq!(Identity::identity_of(identity_hash), None);
		});
	}

	#[test]
	fn attest_from_different_account_should_not_work() {
		with_externalities(&mut new_test_ext([H256::from(9)].to_vec()), || {
			System::set_block_number(1);

			let pair: Pair = Pair::from_seed(&hex!(
				"9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
			));
			let other: Pair = Pair::from_seed(&hex!(
				"9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f61"
			));
			let identity: &[u8] = b"github.com/drewstone";
			let identity_hash = BlakeTwo256::hash_of(&identity.to_vec());
			let public: H256 = pair.public().0.into();
			let other_pub: H256 = other.public().0.into();

			assert_ok!(register_identity(public, identity));
			let attestation: &[u8] = b"www.proof.com/attest_of_extra_proof";
			assert_err!(
				attest_to_identity(other_pub, identity_hash, attestation),
				"Stored identity does not match sender"
			);
			assert_eq!(
				Identity::identity_of(identity_hash),
				Some(default_identity_record(public, identity))
			);
		});
	}

	#[test]
	fn register_attest_and_verify_should_work() {
		with_externalities(&mut new_test_ext([H256::from(9)].to_vec()), || {
			System::set_block_number(1);

			let pair: Pair = Pair::from_seed(&hex!(
				"9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
			));
			let identity: &[u8] = b"github.com/drewstone";
			let identity_hash = BlakeTwo256::hash_of(&identity.to_vec());

			let public: H256 = pair.public().0.into();

			assert_ok!(register_identity(public, identity));

			let mut expiration_time = Identity::expiration_time();
			let mut now = Timestamp::get();
			let registration_expires_at = now + expiration_time;

			let attestation: &[u8] = b"www.proof.com/attest_of_extra_proof";
			assert_ok!(attest_to_identity(public, identity_hash, attestation));

			expiration_time = Identity::expiration_time();
			now = Timestamp::get();
			let attest_expires_at = now + expiration_time;

			let verifier = H256::from(9);
			assert_ok!(verify_identity(verifier, identity_hash, true, 0));

			assert_eq!(
				System::events(),
				vec![
					EventRecord {
						phase: Phase::ApplyExtrinsic(0),
						event: Event::identity(RawEvent::Register(identity_hash, public, registration_expires_at))
					},
					EventRecord {
						phase: Phase::ApplyExtrinsic(0),
						event: Event::identity(RawEvent::Attest(identity_hash, public, attest_expires_at))
					},
					EventRecord {
						phase: Phase::ApplyExtrinsic(0),
						event: Event::identity(RawEvent::Verify(identity_hash, public, 1))
					}
				]
			);
			assert_eq!(
				Identity::identity_of(identity_hash),
				Some(IdentityRecord {
					stage: IdentityStage::Verified,
					expiration_time: 0,
					proof: Some(attestation.to_vec()),
					verifications: [0, 1],
					..default_identity_record(public, identity)
				})
			);
		});
	}

	#[test]
	fn verify_before_register_should_not_work() {
		with_externalities(&mut new_test_ext([H256::from(9)].to_vec()), || {
			System::set_block_number(1);

			let identity: &[u8] = b"github.com/drewstone";
			let identity_hash = BlakeTwo256::hash_of(&identity.to_vec());
			let verifier: H256 = H256::from(9);
			assert_err!(
				verify_identity(verifier, identity_hash, true, 0),
				"Identity does not exist"
			);
			assert_eq!(Identity::identity_of(identity_hash), None);
		});
	}

	#[test]
	fn verify_before_attest_should_not_work() {
		with_externalities(&mut new_test_ext([H256::from(9)].to_vec()), || {
			System::set_block_number(1);

			let pair: Pair = Pair::from_seed(&hex!(
				"9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
			));
			let identity: &[u8] = b"github.com/drewstone";
			let identity_hash = BlakeTwo256::hash_of(&identity.to_vec());

			let public: H256 = pair.public().0.into();

			assert_ok!(register_identity(public, identity));

			let verifier = H256::from(9);
			assert_err!(
				verify_identity(verifier, identity_hash, true, 0),
				"No attestation to verify"
			);
			assert_eq!(
				Identity::identity_of(identity_hash),
				Some(default_identity_record(public, identity))
			);
		});
	}

	#[test]
	fn verify_twice_should_not_work() {
		with_externalities(&mut new_test_ext([H256::from(9)].to_vec()), || {
			System::set_block_number(1);

			let pair: Pair = Pair::from_seed(&hex!(
				"9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
			));
			let identity: &[u8] = b"github.com/drewstone";
			let identity_hash = BlakeTwo256::hash_of(&identity.to_vec());

			let public: H256 = pair.public().0.into();

			assert_ok!(register_identity(public, identity));

			let attestation: &[u8] = b"www.proof.com/attest_of_extra_proof";
			assert_ok!(attest_to_identity(public, identity_hash, attestation));

			let verifier = H256::from(9);
			assert_ok!(verify_identity(verifier, identity_hash, true, 0));
			assert_err!(verify_identity(verifier, identity_hash, true, 0), "Already verified");
			assert_eq!(
				Identity::identity_of(identity_hash),
				Some(IdentityRecord {
					stage: IdentityStage::Verified,
					expiration_time: 0,
					proof: Some(attestation.to_vec()),
					verifications: [0, 1],
					..default_identity_record(public, identity)
				})
			);
		});
	}

	#[test]
	fn attest_after_verify_should_not_work() {
		with_externalities(&mut new_test_ext([H256::from(9)].to_vec()), || {
			System::set_block_number(1);

			let pair: Pair = Pair::from_seed(&hex!(
				"9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
			));
			let identity: &[u8] = b"github.com/drewstone";
			let identity_hash = BlakeTwo256::hash_of(&identity.to_vec());

			let public: H256 = pair.public().0.into();

			assert_ok!(register_identity(public, identity));

			let attestation: &[u8] = b"www.proof.com/attest_of_extra_proof";
			assert_ok!(attest_to_identity(public, identity_hash, attestation));

			let verifier = H256::from(9);
			assert_ok!(verify_identity(verifier, identity_hash, true, 0));
			assert_err!(
				attest_to_identity(public, identity_hash, attestation),
				"Already verified"
			);
			assert_eq!(
				Identity::identity_of(identity_hash),
				Some(IdentityRecord {
					stage: IdentityStage::Verified,
					expiration_time: 0,
					proof: Some(attestation.to_vec()),
					verifications: [0, 1],
					..default_identity_record(public, identity)
				})
			);
		});
	}

	#[test]
	fn verify_from_nonverifier_should_not_work() {
		with_externalities(&mut new_test_ext([H256::from(9)].to_vec()), || {
			System::set_block_number(1);

			let pair: Pair = Pair::from_seed(&hex!(
				"9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
			));
			let identity: &[u8] = b"github.com/drewstone";
			let identity_hash = BlakeTwo256::hash_of(&identity.to_vec());

			let public: H256 = pair.public().0.into();

			assert_ok!(register_identity(public, identity));

			let attestation: &[u8] = b"www.proof.com/attest_of_extra_proof";
			assert_ok!(attest_to_identity(public, identity_hash, attestation));

			assert_err!(
				verify_identity(public, identity_hash, true, 0),
				"Sender not a verifier"
			);
			assert_eq!(
				Identity::identity_of(identity_hash),
				Some(IdentityRecord {
					stage: IdentityStage::Attested,
					proof: Some(attestation.to_vec()),
					..default_identity_record(public, identity)
				})
			);
		});
	}

	#[test]
	fn register_should_expire() {
		with_externalities(&mut new_test_ext([H256::from(9)].to_vec()), || {
			System::set_block_number(1);

			let pair: Pair = Pair::from_seed(&hex!(
				"9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
			));
			let identity: &[u8] = b"github.com/drewstone";
			let identity_hash = BlakeTwo256::hash_of(&identity.to_vec());

			let public: H256 = pair.public().0.into();

			assert_ok!(register_identity(public, identity));

			let expiration_time = Identity::expiration_time();
			let now = Timestamp::get();
			let registration_expires_at = now + expiration_time;

			Timestamp::set_timestamp(10001);

			assert_ok!(remove_expired_identity(public, identity_hash));

			let attestation: &[u8] = b"www.proof.com/attest_of_extra_proof";
			assert_err!(
				attest_to_identity(public, identity_hash, attestation),
				"Identity does not exist"
			);

			assert_eq!(
				System::events(),
				vec![
					EventRecord {
						phase: Phase::ApplyExtrinsic(0),
						event: Event::identity(RawEvent::Register(identity_hash, public, registration_expires_at))
					},
					EventRecord {
						phase: Phase::ApplyExtrinsic(0),
						event: Event::identity(RawEvent::Expired(identity_hash))
					},
				]
			);
			assert_eq!(Identity::identity_of(identity_hash), None);
		});
	}

	#[test]
	fn attest_should_expire() {
		with_externalities(&mut new_test_ext([H256::from(9)].to_vec()), || {
			System::set_block_number(1);

			let pair: Pair = Pair::from_seed(&hex!(
				"9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
			));
			let identity: &[u8] = b"github.com/drewstone";
			let identity_hash = BlakeTwo256::hash_of(&identity.to_vec());

			let public: H256 = pair.public().0.into();

			assert_ok!(register_identity(public, identity));

			let mut expiration_time = Identity::expiration_time();
			let mut now = Timestamp::get();
			let registration_expires_at = now + expiration_time;

			let attestation: &[u8] = b"www.proof.com/attest_of_extra_proof";
			assert_ok!(attest_to_identity(public, identity_hash, attestation));

			expiration_time = Identity::expiration_time();
			now = Timestamp::get();
			let attest_expires_at = now + expiration_time;

			Timestamp::set_timestamp(10001);

			assert_ok!(remove_expired_identity(public, identity_hash));

			let verifier: H256 = H256::from(9);
			assert_err!(
				verify_identity(verifier, identity_hash, true, 0),
				"Identity does not exist"
			);

			assert_eq!(
				System::events(),
				vec![
					EventRecord {
						phase: Phase::ApplyExtrinsic(0),
						event: Event::identity(RawEvent::Register(identity_hash, public, registration_expires_at))
					},
					EventRecord {
						phase: Phase::ApplyExtrinsic(0),
						event: Event::identity(RawEvent::Attest(identity_hash, public, attest_expires_at))
					},
					EventRecord {
						phase: Phase::ApplyExtrinsic(0),
						event: Event::identity(RawEvent::Expired(identity_hash))
					},
				]
			);
			assert_eq!(Identity::identity_of(identity_hash), None);
		});
	}

	#[test]
	fn verify_should_not_expire() {
		with_externalities(&mut new_test_ext([H256::from(9)].to_vec()), || {
			System::set_block_number(1);

			let pair: Pair = Pair::from_seed(&hex!(
				"9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
			));
			let identity: &[u8] = b"github.com/drewstone";
			let identity_hash = BlakeTwo256::hash_of(&identity.to_vec());

			let public: H256 = pair.public().0.into();

			assert_ok!(register_identity(public, identity));

			let attestation: &[u8] = b"www.proof.com/attest_of_extra_proof";
			assert_ok!(attest_to_identity(public, identity_hash, attestation));

			let verifier = H256::from(9);
			assert_ok!(verify_identity(verifier, identity_hash, true, 0));

			assert_err!(
				register_identity(public, identity),
				"Identity already exists"
			);
			assert_eq!(
				Identity::identity_of(identity_hash),
				Some(IdentityRecord {
					stage: IdentityStage::Verified,
					expiration_time: 0,
					proof: Some(attestation.to_vec()),
					verifications: [0, 1],
					..default_identity_record(public, identity)
				})
			);
		});
	}

	#[test]
	fn malicious_attest_should_lock() {
		with_externalities(&mut new_test_ext([H256::from(9)].to_vec()), || {
			System::set_block_number(1);

			let pair: Pair = Pair::from_seed(&hex!(
				"9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
			));
			let identity: &[u8] = b"github.com/drewstone";
			let identity_hash = BlakeTwo256::hash_of(&identity.to_vec());

			let public: H256 = pair.public().0.into();

			assert_ok!(register_identity(public, identity));

			let attestation: &[u8] = b"www.proof.com/attest_of_extra_proof";
			assert_ok!(attest_to_identity(public, identity_hash, attestation));

			let verifier = H256::from(9);
			assert_ok!(verify_identity(verifier, identity_hash, false, 0));

			let new_identity: &[u8] = b"github.com/drstone";
			assert_err!(
				register_identity(public, new_identity),
				"Sender account is frozen"
			);
			assert_eq!(Identity::frozen_accounts(public), true);
			assert_eq!(Identity::identity_of(identity_hash), None);
		});
	}

	#[test]
	fn verify_with_two_thirds_should_work() {
		with_externalities(&mut new_test_ext([
			H256::from(9), H256::from(10), H256::from(11),
		].to_vec()), || {
			System::set_block_number(1);

			let pair: Pair = Pair::from_seed(&hex!(
				"9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
			));
			let identity: &[u8] = b"github.com/drewstone";
			let identity_hash = BlakeTwo256::hash_of(&identity.to_vec());

			let public: H256 = pair.public().0.into();

			assert_ok!(register_identity(public, identity));

			let mut expiration_time = Identity::expiration_time();
			let mut now = Timestamp::get();
			let registration_expires_at = now + expiration_time;

			let attestation: &[u8] = b"www.proof.com/attest_of_extra_proof";
			assert_ok!(attest_to_identity(public, identity_hash, attestation));

			expiration_time = Identity::expiration_time();
			now = Timestamp::get();
			let attest_expires_at = now + expiration_time;

			let verifier_1 = H256::from(9);
			assert_ok!(verify_identity(verifier_1, identity_hash, true, 0));

			let verifier_2 = H256::from(10);
			assert_ok!(verify_identity(verifier_2, identity_hash, true, 1));

			assert_eq!(
				System::events(),
				vec![
					EventRecord {
						phase: Phase::ApplyExtrinsic(0),
						event: Event::identity(RawEvent::Register(identity_hash, public, registration_expires_at))
					},
					EventRecord {
						phase: Phase::ApplyExtrinsic(0),
						event: Event::identity(RawEvent::Attest(identity_hash, public, attest_expires_at))
					},
					EventRecord {
						phase: Phase::ApplyExtrinsic(0),
						event: Event::identity(RawEvent::Verify(identity_hash, public, 2))
					},
				]
			);
			assert_eq!(
				Identity::identity_of(identity_hash),
				Some(IdentityRecord {
					stage: IdentityStage::Verified,
					expiration_time: 0,
					proof: Some(attestation.to_vec()),
					verifications: [0, 2],
					..default_identity_record(public, identity)
				})
			);
		});
	}

	#[test]
	fn add_metadata_should_work() {
		with_externalities(&mut new_test_ext([H256::from(9)].to_vec()), || {
			System::set_block_number(1);

			let pair: Pair = Pair::from_seed(&hex!(
				"9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
			));
			let identity: &[u8] = b"github.com/drewstone";
			let identity_hash = BlakeTwo256::hash_of(&identity.to_vec());

			let public: H256 = pair.public().0.into();

			let avatar: &[u8] = b"avatars3.githubusercontent.com/u/13153687";
			let display_name: &[u8] = b"drewstone";
			let tagline: &[u8] = b"hello world!";

			assert_ok!(register_identity(public, identity));
			assert_ok!(add_metadata_to_account(
				public,
				identity_hash,
				avatar,
				display_name,
				tagline
			));
			let default_record = default_identity_record(public, identity);
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
		with_externalities(&mut new_test_ext([H256::from(9)].to_vec()), || {
			System::set_block_number(1);

			let pair: Pair = Pair::from_seed(&hex!(
				"9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
			));
			let identity: &[u8] = b"github.com/drewstone";
			let identity_hash = BlakeTwo256::hash_of(&identity.to_vec());
			let public: H256 = pair.public().0.into();

			let avatar: &[u8] = b"avatars3.githubusercontent.com/u/13153687";
			let display_name: &[u8] = b"drewstone";
			let tagline: &[u8] = b"hello world!";
			assert_err!(
				add_metadata_to_account(public, identity_hash, avatar, display_name, tagline),
				"Identity does not exist"
			);
			assert_eq!(Identity::identity_of(identity_hash), None);
		});
	}

	#[test]
	fn add_metadata_from_different_account_should_not_work() {
		with_externalities(&mut new_test_ext([H256::from(9)].to_vec()), || {
			System::set_block_number(1);

			let pair: Pair = Pair::from_seed(&hex!(
				"9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
			));
			let other: Pair = Pair::from_seed(&hex!(
				"9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f61"
			));
			let identity: &[u8] = b"github.com/drewstone";
			let identity_hash = BlakeTwo256::hash_of(&identity.to_vec());
			let public: H256 = pair.public().0.into();
			let other_pub: H256 = other.public().0.into();

			let avatar: &[u8] = b"avatars3.githubusercontent.com/u/13153687";
			let display_name: &[u8] = b"drewstone";
			let tagline: &[u8] = b"hello world!";

			assert_ok!(register_identity(public, identity));
			assert_err!(
				add_metadata_to_account(other_pub, identity_hash, avatar, display_name, tagline),
				"Stored identity does not match sender"
			);
			assert_eq!(
				Identity::identity_of(identity_hash),
				Some(default_identity_record(public, identity))
			);
		});
	}

	#[test]
	fn add_claim_without_valid_identity_should_not_work() {
		with_externalities(&mut new_test_ext([H256::from(9)].to_vec()), || {
			System::set_block_number(1);

			let issuer = H256::from(1);
			let identity: &[u8] = b"github.com/drewstone";
			let identity_hash = BlakeTwo256::hash_of(&identity.to_vec());
			let claim: &[u8] = b"is over 25 years of age";
			let issuers = Identity::claims_issuers();
			let issuer_index: usize = issuers.iter().position(|id| id == &issuer).unwrap();

			assert_err!(
				add_claim_to_identity(issuer, identity_hash, claim, issuer_index),
				"Invalid identity record"
			);
			assert_eq!(Identity::claims(identity_hash), vec![]);
		});
	}

	#[test]
	fn add_claim_as_invalid_issuer_should_not_work() {
		with_externalities(&mut new_test_ext([H256::from(9)].to_vec()), || {
			System::set_block_number(1);

			let pair: Pair = Pair::from_seed(&hex!(
				"9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
			));
			let public: H256 = pair.public().0.into();
			let identity: &[u8] = b"github.com/drewstone";
			let identity_hash = BlakeTwo256::hash_of(&identity.to_vec());
			let claim: &[u8] = b"is over 25 years of age";

			assert_err!(
				add_claim_to_identity(public, identity_hash, claim, 0),
				"Invalid claims issuer"
			);
			assert_eq!(Identity::claims(identity_hash), vec![]);
		});
	}

	#[test]
	fn add_claim_valid_should_work() {
		with_externalities(&mut new_test_ext([H256::from(9)].to_vec()), || {
			System::set_block_number(1);

			let pair: Pair = Pair::from_seed(&hex!(
				"9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
			));
			let identity: &[u8] = b"github.com/drewstone";
			let identity_hash = BlakeTwo256::hash_of(&identity.to_vec());

			let public: H256 = pair.public().0.into();

			assert_ok!(register_identity(public, identity));

			let issuer = H256::from(1);
			let claim: &[u8] = b"is over 25 years of age";

			let issuers = Identity::claims_issuers();
			let issuer_index: usize = issuers.iter().position(|id| id == &issuer).unwrap();
			assert_ok!(add_claim_to_identity(issuer, identity_hash, claim, issuer_index));
			assert_eq!(Identity::claims(identity_hash), vec![(issuer, claim.to_vec())]);
		});
	}

	#[test]
	fn remove_claim_without_valid_identity_should_not_work() {
		with_externalities(&mut new_test_ext([H256::from(9)].to_vec()), || {
			System::set_block_number(1);

			let issuer = H256::from(1);
			let identity: &[u8] = b"github.com/drewstone";
			let identity_hash = BlakeTwo256::hash_of(&identity.to_vec());

			assert_err!(
				remove_claim_from_identity(issuer, identity_hash, 0),
				"Invalid identity record"
			);
			assert_eq!(Identity::claims(identity_hash), vec![]);
		});
	}

	#[test]
	fn remove_claim_as_invalid_issuer_should_not_work() {
		with_externalities(&mut new_test_ext([H256::from(9)].to_vec()), || {
			System::set_block_number(1);

			let pair: Pair = Pair::from_seed(&hex!(
				"9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
			));
			let public: H256 = pair.public().0.into();
			let identity: &[u8] = b"github.com/drewstone";
			let identity_hash = BlakeTwo256::hash_of(&identity.to_vec());

			assert_ok!(register_identity(public, identity));

			let issuer = H256::from(1);
			let claim: &[u8] = b"is over 25 years of age";
			let issuers = Identity::claims_issuers();
			let issuer_index: usize = issuers.iter().position(|id| id == &issuer).unwrap();
			assert_ok!(add_claim_to_identity(issuer, identity_hash, claim, issuer_index));

			assert_err!(
				remove_claim_from_identity(public, identity_hash, 0),
				"No existing claim under issuer"
			);
			assert_eq!(Identity::claims(identity_hash), vec![(issuer, claim.to_vec())]);
		});
	}

	#[test]
	fn remove_claim_not_issued_should_not_work() {
		with_externalities(&mut new_test_ext([H256::from(9)].to_vec()), || {
			System::set_block_number(1);

			let pair: Pair = Pair::from_seed(&hex!(
				"9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
			));
			let identity: &[u8] = b"github.com/drewstone";
			let identity_hash = BlakeTwo256::hash_of(&identity.to_vec());
			let public: H256 = pair.public().0.into();

			assert_ok!(register_identity(public, identity));

			let issuer = H256::from(1);
			let claim: &[u8] = b"is over 25 years of age";
			let issuers = Identity::claims_issuers();
			let issuer_index: usize = issuers.iter().position(|id| id == &issuer).unwrap();
			assert_ok!(add_claim_to_identity(issuer, identity_hash, claim, issuer_index));

			let another_issuer = H256::from(2);
			assert_err!(
				remove_claim_from_identity(another_issuer, identity_hash, 0),
				"No existing claim under issuer"
			);
			assert_eq!(Identity::claims(identity_hash), vec![(issuer, claim.to_vec())]);
		});
	}

	#[test]
	fn remove_identity_if_majority_vote_is_not_reached() {
		with_externalities(&mut new_test_ext([H256::from(9), H256::from(5), H256::from(4), H256::from(3)].to_vec()), || {
			System::set_block_number(1);

			let pair: Pair = Pair::from_seed(&hex!(
				"9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
			));
			let identity: &[u8] = b"github.com/drewstone";
			let identity_hash = BlakeTwo256::hash_of(&identity.to_vec());

			let public: H256 = pair.public().0.into();

			assert_ok!(register_identity(public, identity));

			let mut expiration_time = Identity::expiration_time();
			let mut now = Timestamp::get();
			let registration_expires_at = now + expiration_time;

			let attestation: &[u8] = b"www.proof.com/attest_of_extra_proof";
			assert_ok!(attest_to_identity(public, identity_hash, attestation));

			expiration_time = Identity::expiration_time();
			now = Timestamp::get();
			let attest_expires_at = now + expiration_time;

			let mut verifier = H256::from(9);
			assert_ok!(verify_identity(verifier, identity_hash, true, 0));

			verifier = H256::from(5);
			assert_ok!(verify_identity(verifier, identity_hash, true, 1));

			verifier = H256::from(4);
			assert_ok!(verify_identity(verifier, identity_hash, false, 2));

			verifier = H256::from(3);
			assert_ok!(verify_identity(verifier, identity_hash, false, 3));

			assert_eq!(
				System::events(),
				vec![
					EventRecord {
						phase: Phase::ApplyExtrinsic(0),
						event: Event::identity(RawEvent::Register(identity_hash, public, registration_expires_at))
					},
					EventRecord {
						phase: Phase::ApplyExtrinsic(0),
						event: Event::identity(RawEvent::Attest(identity_hash, public, attest_expires_at))
					},
					EventRecord {
						phase: Phase::ApplyExtrinsic(0),
						event: Event::identity(RawEvent::Failed(identity_hash, public))
					},
				]
			);

			assert_eq!(Identity::frozen_accounts(public), false);
			assert_eq!(Identity::identity_of(identity_hash), None);
		});
	}

	#[test]
	fn change_verification_vote_before_finalization() {
		with_externalities(&mut new_test_ext([H256::from(9), H256::from(5), H256::from(4)].to_vec()), || {
			System::set_block_number(1);

			let pair: Pair = Pair::from_seed(&hex!(
				"9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
			));
			let identity: &[u8] = b"github.com/drewstone";
			let identity_hash = BlakeTwo256::hash_of(&identity.to_vec());

			let public: H256 = pair.public().0.into();

			assert_ok!(register_identity(public, identity));

			let attestation: &[u8] = b"www.proof.com/attest_of_extra_proof";
			assert_ok!(attest_to_identity(public, identity_hash, attestation));

			let verifier = H256::from(9);
			assert_ok!(verify_identity(verifier, identity_hash, true, 0));

			let mut vote = Identity::verified_identity_by((identity_hash, verifier));
			assert_eq!(vote, Some(true));
			
			assert_ok!(verify_identity(verifier, identity_hash, false, 0));
			vote = Identity::verified_identity_by((identity_hash, verifier));
			assert_eq!(vote, Some(false));
		});
	}
}
