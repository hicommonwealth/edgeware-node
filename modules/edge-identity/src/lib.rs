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
#[cfg_attr(not(feature = "std"), macro_use)]
extern crate sr_std as rstd;
extern crate srml_support as runtime_support;
extern crate substrate_primitives as primitives;

extern crate srml_consensus as consensus;
extern crate srml_system as system;
extern crate srml_timestamp as timestamp;

pub mod identity;
pub use identity::{Event, Module, RawEvent, Trait};

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
        testing::{Digest, DigestItem, Header as TestHeader},
        traits::{BlakeTwo256, Hash, Header},
        BuildStorage,
    };
    use timestamp::OnTimestampSet;

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
        type Header = TestHeader;
        type Event = Event;
        type Log = DigestItem;
    }
    impl consensus::Trait for Test {
        const NOTE_OFFLINE_POSITION: u32 = 1;
        type Log = DigestItem;
        type SessionKey = u64;
        type InherentOfflineReport = ();
    }
    impl timestamp::Trait for Test {
        const TIMESTAMP_SET_POSITION: u32 = 0;
        type Moment = u64;
        type OnTimestampSet = Identity;
    }
    impl Trait for Test {
        type Claim = Vec<u8>;
        type Event = Event;
    }

    type System = system::Module<Test>;
    type Identity = Module<Test>;

    // This function basically just builds a genesis storage key/value store according to
    // our desired mockup.
    fn new_test_ext() -> sr_io::TestExternalities<Blake2Hasher> {
        let mut t = system::GenesisConfig::<Test>::default()
            .build_storage()
            .unwrap()
            .0;
        // We use default for brevity, but you can configure as desired if needed.
        t.extend(
            identity::GenesisConfig::<Test> {
                expiration_time: 1,
                verifiers: [H256::from(9)].to_vec(),
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

    fn verify_identity(who: H256, identity_hash: H256) -> Result {
        Identity::verify(Origin::signed(who), identity_hash)
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

    fn add_claim_to_identity(who: H256, identity_hash: H256, claim: &[u8]) -> Result {
        Identity::add_claim(Origin::signed(who), identity_hash, claim.to_vec())
    }

    fn remove_claim_from_identity(who: H256, identity_hash: H256) -> Result {
        Identity::remove_claim(Origin::signed(who), identity_hash)
    }

    #[test]
    fn register_should_work() {
        with_externalities(&mut new_test_ext(), || {
            System::set_block_number(1);

            let pair: Pair = Pair::from_seed(&hex!(
                "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
            ));
            let identity: &[u8] = b"github.com/drewstone";
            let identity_hash = BlakeTwo256::hash_of(&identity.to_vec());

            let public: H256 = pair.public().0.into();

            assert_ok!(register_identity(public, identity));
            assert_eq!(
                System::events(),
                vec![EventRecord {
                    phase: Phase::ApplyExtrinsic(0),
                    event: Event::identity(RawEvent::Register(identity_hash, public))
                }]
            );
        });
    }

    #[test]
    fn register_twice_should_not_work() {
        with_externalities(&mut new_test_ext(), || {
            System::set_block_number(1);

            let pair: Pair = Pair::from_seed(&hex!(
                "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
            ));
            let identity: &[u8] = b"github.com/drewstone";
            let public: H256 = pair.public().0.into();

            assert_ok!(register_identity(public, identity));
            assert_err!(
                register_identity(public, identity),
                "Identity already exists"
            )
        });
    }

    #[test]
    fn register_and_attest_should_work() {
        with_externalities(&mut new_test_ext(), || {
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
            assert_eq!(
                System::events(),
                vec![
                    EventRecord {
                        phase: Phase::ApplyExtrinsic(0),
                        event: Event::identity(RawEvent::Register(identity_hash, public))
                    },
                    EventRecord {
                        phase: Phase::ApplyExtrinsic(0),
                        event: Event::identity(RawEvent::Attest(identity_hash, public))
                    }
                ]
            );
        });
    }

    #[test]
    fn attest_without_register_should_not_work() {
        with_externalities(&mut new_test_ext(), || {
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
        });
    }

    #[test]
    fn attest_from_different_account_should_not_work() {
        with_externalities(&mut new_test_ext(), || {
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
        });
    }

    #[test]
    fn register_attest_and_verify_should_work() {
        with_externalities(&mut new_test_ext(), || {
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
            assert_ok!(verify_identity(verifier, identity_hash));

            assert_eq!(
                System::events(),
                vec![
                    EventRecord {
                        phase: Phase::ApplyExtrinsic(0),
                        event: Event::identity(RawEvent::Register(identity_hash, public))
                    },
                    EventRecord {
                        phase: Phase::ApplyExtrinsic(0),
                        event: Event::identity(RawEvent::Attest(identity_hash, public))
                    },
                    EventRecord {
                        phase: Phase::ApplyExtrinsic(0),
                        event: Event::identity(RawEvent::Verify(identity_hash, verifier))
                    }
                ]
            );
        });
    }

    #[test]
    fn verify_before_register_should_not_work() {
        with_externalities(&mut new_test_ext(), || {
            System::set_block_number(1);

            let identity: &[u8] = b"github.com/drewstone";
            let identity_hash = BlakeTwo256::hash_of(&identity.to_vec());
            let verifier: H256 = H256::from(9);
            assert_err!(
                verify_identity(verifier, identity_hash),
                "Identity does not exist"
            );
        });
    }

    #[test]
    fn verify_before_attest_should_not_work() {
        with_externalities(&mut new_test_ext(), || {
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
                verify_identity(verifier, identity_hash),
                "No attestation to verify"
            );
        });
    }

    #[test]
    fn verify_twice_should_not_work() {
        with_externalities(&mut new_test_ext(), || {
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
            assert_ok!(verify_identity(verifier, identity_hash));
            assert_err!(verify_identity(verifier, identity_hash), "Already verified");
        });
    }

    #[test]
    fn attest_after_verify_should_not_work() {
        with_externalities(&mut new_test_ext(), || {
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
            assert_ok!(verify_identity(verifier, identity_hash));
            assert_err!(
                attest_to_identity(public, identity_hash, attestation),
                "Already verified"
            );
        });
    }

    #[test]
    fn verify_from_nonverifier_should_not_work() {
        with_externalities(&mut new_test_ext(), || {
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
                verify_identity(public, identity_hash),
                "Sender not a verifier"
            );
        });
    }

    #[test]
    fn register_should_expire() {
        with_externalities(&mut new_test_ext(), || {
            System::initialise(&1, &Default::default(), &Default::default());
            Identity::on_timestamp_set(0);

            let pair: Pair = Pair::from_seed(&hex!(
                "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
            ));
            let identity: &[u8] = b"github.com/drewstone";
            let identity_hash = BlakeTwo256::hash_of(&identity.to_vec());

            let public: H256 = pair.public().0.into();

            assert_ok!(register_identity(public, identity));

            let header = System::finalise();
            System::initialise(&2, &Header::hash(&header), &Default::default());
            Identity::on_timestamp_set(0);

            let attestation: &[u8] = b"www.proof.com/attest_of_extra_proof";
            assert_err!(
                attest_to_identity(public, identity_hash, attestation),
                "Identity does not exist"
            );

            assert_eq!(
                System::events(),
                vec![EventRecord {
                    phase: Phase::ApplyExtrinsic(0),
                    event: Event::identity(RawEvent::Expired(identity_hash))
                },]
            );
        });
    }

    #[test]
    fn attest_should_expire() {
        with_externalities(&mut new_test_ext(), || {
            System::initialise(&1, &Default::default(), &Default::default());
            Identity::on_timestamp_set(0);

            let pair: Pair = Pair::from_seed(&hex!(
                "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
            ));
            let identity: &[u8] = b"github.com/drewstone";
            let identity_hash = BlakeTwo256::hash_of(&identity.to_vec());

            let public: H256 = pair.public().0.into();

            assert_ok!(register_identity(public, identity));

            let attestation: &[u8] = b"www.proof.com/attest_of_extra_proof";
            assert_ok!(attest_to_identity(public, identity_hash, attestation));

            let header = System::finalise();
            System::initialise(&2, &Header::hash(&header), &Default::default());
            Identity::on_timestamp_set(0);

            let verifier: H256 = H256::from(9);
            assert_err!(
                verify_identity(verifier, identity_hash),
                "Identity does not exist"
            );

            assert_eq!(
                System::events(),
                vec![EventRecord {
                    phase: Phase::ApplyExtrinsic(0),
                    event: Event::identity(RawEvent::Expired(identity_hash))
                },]
            );
        });
    }

    #[test]
    fn verify_should_not_expire() {
        with_externalities(&mut new_test_ext(), || {
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
            assert_ok!(verify_identity(verifier, identity_hash));

            let header = System::finalise();
            System::initialise(&2, &Header::hash(&header), &Default::default());
            Identity::on_timestamp_set(0);

            assert_err!(
                register_identity(public, identity),
                "Identity already exists"
            )
        });
    }

    #[test]
    fn add_metadata_should_work() {
        with_externalities(&mut new_test_ext(), || {
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
        });
    }

    #[test]
    fn add_metadata_without_register_should_not_work() {
        with_externalities(&mut new_test_ext(), || {
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
        });
    }

    #[test]
    fn add_metadata_from_different_account_should_not_work() {
        with_externalities(&mut new_test_ext(), || {
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
        });
    }

    #[test]
    fn add_claim_without_valid_identity_should_not_work() {
        with_externalities(&mut new_test_ext(), || {
            System::set_block_number(1);

            let issuer = H256::from(1);
            let identity: &[u8] = b"github.com/drewstone";
            let identity_hash = BlakeTwo256::hash_of(&identity.to_vec());
            let claim: &[u8] = b"is over 25 years of age";

            assert_err!(
                add_claim_to_identity(issuer, identity_hash, claim),
                "Invalid identity record"
            );
        });
    }

    #[test]
    fn add_claim_as_invalid_issuer_should_not_work() {
        with_externalities(&mut new_test_ext(), || {
            System::set_block_number(1);

            let pair: Pair = Pair::from_seed(&hex!(
                "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
            ));
            let public: H256 = pair.public().0.into();
            let identity: &[u8] = b"github.com/drewstone";
            let identity_hash = BlakeTwo256::hash_of(&identity.to_vec());
            let claim: &[u8] = b"is over 25 years of age";

            assert_err!(
                add_claim_to_identity(public, identity_hash, claim),
                "Invalid claims issuer"
            );
        });
    }

    #[test]
    fn add_claim_valid_should_work() {
        with_externalities(&mut new_test_ext(), || {
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
            assert_ok!(add_claim_to_identity(issuer, identity_hash, claim));
        });
    }

    #[test]
    fn remove_claim_without_valid_identity_should_not_work() {
        with_externalities(&mut new_test_ext(), || {
            System::set_block_number(1);

            let issuer = H256::from(1);
            let identity: &[u8] = b"github.com/drewstone";
            let identity_hash = BlakeTwo256::hash_of(&identity.to_vec());

            assert_err!(
                remove_claim_from_identity(issuer, identity_hash),
                "Invalid identity record"
            );
        });
    }

    #[test]
    fn remove_claim_as_invalid_issuer_should_not_work() {
        with_externalities(&mut new_test_ext(), || {
            System::set_block_number(1);

            let pair: Pair = Pair::from_seed(&hex!(
                "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
            ));
            let public: H256 = pair.public().0.into();
            let identity: &[u8] = b"github.com/drewstone";
            let identity_hash = BlakeTwo256::hash_of(&identity.to_vec());

            assert_err!(
                remove_claim_from_identity(public, identity_hash),
                "Invalid claims issuer"
            );
        });
    }

    #[test]
    fn remove_claim_not_issued_should_not_work() {
        with_externalities(&mut new_test_ext(), || {
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
            assert_ok!(add_claim_to_identity(issuer, identity_hash, claim));

            let another_issuer = H256::from(2);
            assert_err!(
                remove_claim_from_identity(another_issuer, identity_hash),
                "No existing claim under issuer"
            );
        });
    }
}
