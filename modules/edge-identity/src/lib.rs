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
#[macro_use]
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

extern crate srml_system as system;

use rstd::prelude::*;
use runtime_support::dispatch::Result;

pub mod identity;
pub use identity::{Event, Module, RawEvent, Trait};

// Tests for Identity Module
#[cfg(test)]
mod tests {
    use super::*;

    use primitives::{Blake2Hasher, Hasher, H256};
    use runtime_io::ed25519::Pair;
    use runtime_io::with_externalities;
    use system::{EventRecord, Phase};
    // The testing primitives are very useful for avoiding having to work with
    // public keys. `u64` is used as the `AccountId` and no `Signature`s are requried.
    use runtime_primitives::{
        testing::{Digest, DigestItem, Header},
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
        pub enum Call for Test where origin: Origin {}
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
                claims_issuers: [H256::from(1), H256::from(2), H256::from(3)].to_vec(),
            }
            .build_storage()
            .unwrap()
            .0,
        );
        t.into()
    }

    fn publish_identity_attestation(who: H256, attestation: &[u8]) -> super::Result {
        Identity::attest(Origin::signed(who), attestation.to_vec())
    }

    fn link_identity_with_proof(
        who: H256,
        identity_hash: H256,
        proof_link: &[u8],
    ) -> super::Result {
        Identity::link(Origin::signed(who), identity_hash, proof_link.to_vec())
    }

    fn add_metadata_to_account(
        who: H256,
        identity_hash: H256,
        avatar: &[u8],
        display_name: &[u8],
        tagline: &[u8],
    ) -> super::Result {
        Identity::add_metadata(
            Origin::signed(who),
            identity_hash,
            avatar.to_vec(),
            display_name.to_vec(),
            tagline.to_vec(),
        )
    }

    fn add_claim_to_identity(who: H256, identity_hash: H256, claim: &[u8]) -> super::Result {
        Identity::add_claim(Origin::signed(who), identity_hash, claim.to_vec())
    }

    fn remove_claim_from_identity(who: H256, identity_hash: H256) -> super::Result {
        Identity::remove_claim(Origin::signed(who), identity_hash)
    }

    #[test]
    fn propose_should_work() {
        with_externalities(&mut new_test_ext(), || {
            System::set_block_number(1);

            let pair: Pair = Pair::from_seed(&hex!(
                "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
            ));
            let message: &[u8] = b"github.com/drewstone";
            let identity_hash = BlakeTwo256::hash_of(&message.to_vec());

            let public: H256 = pair.public().0.into();

            assert_ok!(publish_identity_attestation(public, message));
            assert_eq!(
                System::events(),
                vec![EventRecord {
                    phase: Phase::ApplyExtrinsic(0),
                    event: Event::identity(RawEvent::Attested(identity_hash, public))
                }]
            );
        });
    }

    #[test]
    fn propose_and_link_should_work() {
        with_externalities(&mut new_test_ext(), || {
            System::set_block_number(1);

            let pair: Pair = Pair::from_seed(&hex!(
                "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
            ));
            let message: &[u8] = b"github.com/drewstone";
            let identity_hash = BlakeTwo256::hash_of(&message.to_vec());

            let public: H256 = pair.public().0.into();

            assert_ok!(publish_identity_attestation(public, message));

            let proof_link: &[u8] = b"www.proof.com/link_of_extra_proof";
            assert_ok!(link_identity_with_proof(public, identity_hash, proof_link));
            assert_eq!(
                System::events(),
                vec![
                    EventRecord {
                        phase: Phase::ApplyExtrinsic(0),
                        event: Event::identity(RawEvent::Attested(identity_hash, public))
                    },
                    EventRecord {
                        phase: Phase::ApplyExtrinsic(0),
                        event: Event::identity(RawEvent::Linked(identity_hash, public))
                    }
                ]
            );
        });
    }

    #[test]
    fn link_without_publish_should_not_work() {
        with_externalities(&mut new_test_ext(), || {
            System::set_block_number(1);

            let pair: Pair = Pair::from_seed(&hex!(
                "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
            ));
            let message: &[u8] = b"github.com/drewstone";
            let identity_hash = BlakeTwo256::hash_of(&message.to_vec());
            let public: H256 = pair.public().0.into();

            let proof_link: &[u8] = b"www.proof.com/link_of_extra_proof";
            assert_eq!(
                link_identity_with_proof(public, identity_hash, proof_link),
                Err("Identity does not exist")
            );
        });
    }

    #[test]
    fn link_from_different_account_should_not_work() {
        with_externalities(&mut new_test_ext(), || {
            System::set_block_number(1);

            let pair: Pair = Pair::from_seed(&hex!(
                "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
            ));
            let other: Pair = Pair::from_seed(&hex!(
                "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f61"
            ));
            let message: &[u8] = b"github.com/drewstone";
            let identity_hash = BlakeTwo256::hash_of(&message.to_vec());
            let public: H256 = pair.public().0.into();
            let other_pub: H256 = other.public().0.into();

            assert_ok!(publish_identity_attestation(public, message));
            let proof_link: &[u8] = b"www.proof.com/link_of_extra_proof";
            assert_eq!(
                link_identity_with_proof(other_pub, identity_hash, proof_link),
                Err("Stored identity does not match sender")
            );
        });
    }

    #[test]
    fn add_metadata_should_work() {
        with_externalities(&mut new_test_ext(), || {
            System::set_block_number(1);
            let pair: Pair = Pair::from_seed(&hex!(
                "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
            ));
            let message: &[u8] = b"github.com/drewstone";
            let identity_hash = BlakeTwo256::hash_of(&message.to_vec());

            let public: H256 = pair.public().0.into();

            let avatar: &[u8] = b"avatars3.githubusercontent.com/u/13153687";
            let display_name: &[u8] = b"drewstone";
            let tagline: &[u8] = b"hello world!";

            assert_ok!(publish_identity_attestation(public, message));
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
    fn add_metadata_without_publish_should_not_work() {
        with_externalities(&mut new_test_ext(), || {
            System::set_block_number(1);

            let pair: Pair = Pair::from_seed(&hex!(
                "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
            ));
            let message: &[u8] = b"github.com/drewstone";
            let identity_hash = BlakeTwo256::hash_of(&message.to_vec());
            let public: H256 = pair.public().0.into();

            let avatar: &[u8] = b"avatars3.githubusercontent.com/u/13153687";
            let display_name: &[u8] = b"drewstone";
            let tagline: &[u8] = b"hello world!";
            assert_eq!(
                add_metadata_to_account(public, identity_hash, avatar, display_name, tagline),
                Err("Identity does not exist")
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
            let message: &[u8] = b"github.com/drewstone";
            let identity_hash = BlakeTwo256::hash_of(&message.to_vec());
            let public: H256 = pair.public().0.into();
            let other_pub: H256 = other.public().0.into();

            let avatar: &[u8] = b"avatars3.githubusercontent.com/u/13153687";
            let display_name: &[u8] = b"drewstone";
            let tagline: &[u8] = b"hello world!";

            assert_ok!(publish_identity_attestation(public, message));
            assert_eq!(
                add_metadata_to_account(other_pub, identity_hash, avatar, display_name, tagline),
                Err("Stored identity does not match sender")
            );
        });
    }

    #[test]
    fn add_claim_without_valid_identity_should_not_work() {
        with_externalities(&mut new_test_ext(), || {
            System::set_block_number(1);

            let issuer = H256::from(1);
            let message: &[u8] = b"github.com/drewstone";
            let identity_hash = BlakeTwo256::hash_of(&message.to_vec());
            let claim: &[u8] = b"is over 25 years of age";

            assert_eq!(
                add_claim_to_identity(issuer, identity_hash, claim),
                Err("Invalid identity record")
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
            let message: &[u8] = b"github.com/drewstone";
            let identity_hash = BlakeTwo256::hash_of(&message.to_vec());
            let claim: &[u8] = b"is over 25 years of age";

            assert_eq!(
                add_claim_to_identity(public, identity_hash, claim),
                Err("Invalid claims issuer")
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
            let message: &[u8] = b"github.com/drewstone";
            let identity_hash = BlakeTwo256::hash_of(&message.to_vec());

            let public: H256 = pair.public().0.into();

            assert_ok!(publish_identity_attestation(public, message));

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
            let message: &[u8] = b"github.com/drewstone";
            let identity_hash = BlakeTwo256::hash_of(&message.to_vec());

            assert_eq!(
                remove_claim_from_identity(issuer, identity_hash),
                Err("Invalid identity record")
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
            let message: &[u8] = b"github.com/drewstone";
            let identity_hash = BlakeTwo256::hash_of(&message.to_vec());

            assert_eq!(
                remove_claim_from_identity(public, identity_hash),
                Err("Invalid claims issuer")
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
            let message: &[u8] = b"github.com/drewstone";
            let identity_hash = BlakeTwo256::hash_of(&message.to_vec());
            let public: H256 = pair.public().0.into();

            assert_ok!(publish_identity_attestation(public, message));

            let issuer = H256::from(1);
            let claim: &[u8] = b"is over 25 years of age";
            assert_ok!(add_claim_to_identity(issuer, identity_hash, claim));

            let another_issuer = H256::from(2);
            assert_eq!(
                remove_claim_from_identity(another_issuer, identity_hash),
                Err("No existing claim under issuer")
            );
        });
    }
}
