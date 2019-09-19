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

#[cfg(feature = "std")]
extern crate serde;

// Needed for deriving `Serialize` and `Deserialize` for various types.
// We only implement the serde traits for std builds - they're unneeded
// in the wasm runtime.
#[cfg(feature = "std")]
extern crate parity_codec as codec;
extern crate sr_io as runtime_io;
extern crate sr_primitives as runtime_primitives;
extern crate sr_std as rstd;
extern crate srml_support as runtime_support;
extern crate substrate_primitives as primitives;

extern crate srml_system as system;
extern crate srml_balances as balances;


use srml_support::traits::{Currency, ReservableCurrency};

use rstd::prelude::*;
use runtime_primitives::traits::{Zero, Hash};
use runtime_support::dispatch::Result;
use runtime_support::{StorageMap};
use system::ensure_signed;
use codec::{Encode, Decode};

pub trait Trait: balances::Trait {
    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
    /// The account balance.
    type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
}

pub type Attestation = Vec<u8>;
pub type IdentityType = Vec<u8>;
pub type Identity = Vec<u8>;
type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, PartialEq)]
pub struct MetadataRecord {
    pub avatar: Vec<u8>,
    pub display_name: Vec<u8>,
    pub tagline: Vec<u8>,
}

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, Copy, Clone, Eq, PartialEq)]
pub enum IdentityStage {
    Registered,
    Attested,
    Verified,
}

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, PartialEq)]
pub struct IdentityRecord<AccountId, BlockNumber> {
    pub account: AccountId,
    pub identity_type: IdentityType,
    pub identity: Identity,
    pub stage: IdentityStage,
    pub expiration_length: BlockNumber,
    pub proof: Option<Attestation>,
    pub metadata: Option<MetadataRecord>,
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        /// A function that registers an identity_type and identity for a user
        ///
        /// Checks whether the (identity_type, identity) pair exists and creates
        /// the record if now. The record is indexed by the hash of the pair.
        pub fn register(origin, identity_type: IdentityType, identity: Identity) -> Result {
            let _sender = ensure_signed(origin)?;
            // create hash
            let mut buf = Vec::new();
            buf.extend_from_slice(&identity_type.encode());
            buf.extend_from_slice(&identity.encode());
            let hash = T::Hashing::hash(&buf[..]);
            return Self::do_register_identity(_sender, identity_type, identity, hash);
        }

        /// A function that creates an identity attestation
        ///
        /// Attestation is only valid if the identity is in the attestation phase
        /// and is verified off-chain using an off-chain worker node. Current
        /// implementation overwrites all proofs if safety checks pass.
        pub fn attest(origin, identity_hash: T::Hash, attestation: Attestation) -> Result {
            let _sender = ensure_signed(origin)?;
            return Self::do_attest(_sender, identity_hash, attestation);
        }

        /// A function that registers and attests to an identity simultaneously.
        ///
        /// Allows more efficient registration and attestation processing since it
        /// requires only 1 transaction.
        pub fn register_and_attest(origin, identity_type: IdentityType, identity: Identity, attestation: Attestation) -> Result {
            let _sender = ensure_signed(origin)?;
            // create hash
            let mut buf = Vec::new();
            buf.extend_from_slice(&identity_type.encode());
            buf.extend_from_slice(&identity.encode());
            let hash = T::Hashing::hash(&buf[..]);
            Self::do_register_identity(_sender.clone(), identity_type, identity, hash)?;
            return Self::do_attest(_sender, hash, attestation);
        }

        /// A function that verifies an identity attestation.
        ///
        /// The verification is handled by a set of seeded verifiers who run
        /// the off-chain worker node to verify attestations.
        pub fn verify(origin, identity_hash: T::Hash, verifier_index: u32) -> Result {
            let _sender = ensure_signed(origin)?;
            ensure!((verifier_index as usize) < Self::verifiers().len(), "Verifier index out of bounds");
            ensure!(Self::verifiers()[verifier_index as usize] == _sender.clone(), "Sender is not a verifier");
            return Self::verify_or_deny_identity(_sender, &identity_hash, true);
        }

        /// A function that denies an identity attestation.
        ///
        /// The verification is handled by a set of seeded verifiers who run
        /// the off-chain worker node to verify attestations.
        pub fn deny(origin, identity_hash: T::Hash, verifier_index: u32) -> Result {
            let _sender = ensure_signed(origin)?;
            ensure!((verifier_index as usize) < Self::verifiers().len(), "Verifier index out of bounds");
            ensure!(Self::verifiers()[verifier_index as usize] == _sender.clone(), "Sender is not a verifier");
            return Self::verify_or_deny_identity(_sender, &identity_hash, false);
        }

        /// Verify many verification requests
        pub fn verify_many(origin, identity_hashes: Vec<T::Hash>, verifier_index: u32) -> Result {
            let _sender = ensure_signed(origin)?;
            ensure!((verifier_index as usize) < Self::verifiers().len(), "Verifier index out of bounds");
            ensure!(Self::verifiers()[verifier_index as usize] == _sender.clone(), "Sender is not a verifier");

            for i in 0..identity_hashes.len() {
                Self::verify_or_deny_identity(_sender.clone(), &identity_hashes[i], true)?;
            }

            Ok(())
        }

        /// Deny many verification requests
        pub fn deny_many(origin, identity_hashes: Vec<T::Hash>, verifier_index: u32) -> Result {
            let _sender = ensure_signed(origin)?;
            ensure!((verifier_index as usize) < Self::verifiers().len(), "Verifier index out of bounds");
            ensure!(Self::verifiers()[verifier_index as usize] == _sender.clone(), "Sender is not a verifier");

            for i in 0..identity_hashes.len() {
                Self::verify_or_deny_identity(_sender.clone(), &identity_hashes[i], false)?;
            }

            Ok(())
        }

        /// Add metadata to sender's account.
        pub fn add_metadata(origin, identity_hash: T::Hash, avatar: Vec<u8>, display_name: Vec<u8>, tagline: Vec<u8>) -> Result {
            let _sender = ensure_signed(origin)?;
            let record = <IdentityOf<T>>::get(&identity_hash).ok_or("Identity does not exist")?;
            // Check that original sender and current sender match
            ensure!(record.account == _sender, "Stored identity does not match sender");
            ensure!(<system::Module<T>>::block_number() <= record.expiration_length, "Identity expired");
            // Replace all metadata
            let mut new_record = record;
            new_record.metadata = Some(MetadataRecord {
                avatar: avatar,
                display_name: display_name,
                tagline: tagline,
            });
            <IdentityOf<T>>::insert(identity_hash, new_record);
            Ok(())
        }

        /// Revoke an identity from the creator/sender of such an identity
        pub fn revoke(origin, identity_hash: T::Hash) -> Result {
            let _sender = ensure_signed(origin)?;
            let record = <IdentityOf<T>>::get(&identity_hash).ok_or("Identity does not exist")?;
            // Check that original sender and current sender match
            ensure!(record.account == _sender, "Stored identity does not match sender");
            Self::remove_pending_identity(&identity_hash);
            Ok(())
        }

        /// Check all pending identities for expiration when each block is
        /// finalised. Once an identity expires, it is deleted from storage.
        fn on_finalize(_n: T::BlockNumber) {
            let (expired, valid): (Vec<_>, _) = <IdentitiesPending<T>>::get()
                .into_iter()
                .partition(|(_, exp)| (_n > *exp) && (*exp > T::BlockNumber::zero()));

            expired.into_iter().for_each(move |(exp_hash, _)| {
                if let Some(id_record) = <IdentityOf<T>>::get(exp_hash) {
                    let mut types = <UsedTypes<T>>::get(id_record.account.clone());
                    types.retain(|t| *t != id_record.identity_type.clone());
                    <UsedTypes<T>>::insert(id_record.account, types);
                }

                <Identities<T>>::mutate(|idents| idents.retain(|hash| hash != &exp_hash));
                <IdentityOf<T>>::remove(exp_hash);
                Self::deposit_event(RawEvent::Expired(exp_hash))
            });
            <IdentitiesPending<T>>::put(valid);
        }
    }
}

impl<T: Trait> Module<T> {
    /// Removes all data about a pending identity given the hash of the record
    pub fn remove_pending_identity(identity_hash: &T::Hash) {
        <Identities<T>>::mutate(|idents| idents.retain(|hash| hash != identity_hash));
        <IdentityOf<T>>::remove(identity_hash);
        <IdentitiesPending<T>>::mutate(|idents| idents.retain(|(hash, _)| hash != identity_hash));
    }

    /// Helper function for executing the verification of identities
    fn verify_or_deny_identity(sender: T::AccountId, identity_hash: &T::Hash, approve: bool) -> Result {
        let record = <IdentityOf<T>>::get(identity_hash).ok_or("Identity does not exist")?;
        ensure!(<system::Module<T>>::block_number() <= record.expiration_length, "Identity expired");
        match record.stage {
            IdentityStage::Registered => return Err("No attestation to verify"),
            IdentityStage::Verified => return Err("Already verified"),
            IdentityStage::Attested => ()
        }

        let id_type = record.identity_type.encode().clone();
        let id = record.identity.encode().clone();
        if approve {
            // Add the registration bond amount on behalf of a successful verification
            T::Currency::unreserve(&record.account, Self::registration_bond());
            // Remove identity from list of pending identities
            <IdentitiesPending<T>>::mutate(|idents| idents.retain(|(hash, _)| hash != identity_hash));
            Self::deposit_event(RawEvent::Verify(*identity_hash, sender, id_type, id));
            <IdentityOf<T>>::insert(identity_hash, IdentityRecord {
                stage: IdentityStage::Verified,
                expiration_length: T::BlockNumber::zero(),
                ..record
            });
        } else {
            T::Currency::slash_reserved(&record.account, Self::registration_bond());
            Self::remove_pending_identity(&identity_hash);
            Self::deposit_event(RawEvent::Denied(*identity_hash, sender, id_type, id));
        }

        Ok(())
    }

    /// Helper function for executing the registration of identities
    fn do_register_identity(sender: T::AccountId, identity_type: IdentityType, identity: Identity, identity_hash: T::Hash) -> Result {
        ensure!(!<UsedTypes<T>>::get(sender.clone()).iter().any(|i| i == &identity_type), "Identity type already used");
        ensure!(!<IdentityOf<T>>::exists(identity_hash), "Identity already exists");
        // Reserve the registration bond amount
        T::Currency::reserve(&sender, Self::registration_bond()).map_err(|_| "Not enough currency for reserve bond")?;

        // reserve identity type
        let mut types = <UsedTypes<T>>::get(sender.clone());
        types.push(identity_type.clone());
        <UsedTypes<T>>::insert(sender.clone(), types);

        // Set expiration time of identity
        let now = <system::Module<T>>::block_number();
        let expiration = now + Self::expiration_length();
        // Add identity record
        <Identities<T>>::mutate(|idents| idents.push(identity_hash.clone()));
        <IdentityOf<T>>::insert(identity_hash, IdentityRecord {
            account: sender.clone(),
            identity_type: identity_type,
            identity: identity,
            stage: IdentityStage::Registered,
            expiration_length: expiration.clone(),
            proof: None,
            metadata: None,
        });
        <IdentitiesPending<T>>::mutate(|idents| idents.push((identity_hash, expiration.clone())));
        // Fire register event
        Self::deposit_event(RawEvent::Register(identity_hash, sender.into(), expiration));
        Ok(())
    }

    /// Helper function for executing the attestation of identities
    fn do_attest(sender: T::AccountId, identity_hash: T::Hash, attestation: Attestation) -> Result {
        let record = <IdentityOf<T>>::get(&identity_hash).ok_or("Identity does not exist")?;
        // Ensure the record is not verified
        ensure!(record.stage != IdentityStage::Verified, "Already verified");
        // Ensure the record isn't expired if it still exists
        ensure!(<system::Module<T>>::block_number() <= record.expiration_length, "Identity expired");
        // Check that original sender and current sender match
        ensure!(record.account == sender, "Stored identity does not match sender");

        // update record
        let id_type = record.identity_type.clone();
        let identity = record.identity.clone();
        let now = <system::Module<T>>::block_number();
        let expiration = now + Self::expiration_length();
        // Currently this implements no check against updating proof links
        <IdentityOf<T>>::insert(identity_hash, IdentityRecord {
            proof: Some(attestation.clone()),
            stage: IdentityStage::Attested,
            expiration_length: expiration.clone(),
            ..record
        });

        <IdentitiesPending<T>>::mutate(|idents| {
            idents.retain(|(hash, _)| hash != &identity_hash);
            idents.push((identity_hash, expiration.clone()))
        });

        Self::deposit_event(RawEvent::Attest(attestation, identity_hash, sender.into(), id_type, identity));
        Ok(())
    }
}

decl_event!(
    pub enum Event<T> where <T as system::Trait>::Hash,
                            <T as system::Trait>::AccountId,
                            <T as system::Trait>::BlockNumber {
        /// (record_hash, creator, expiration) when an account is registered
        Register(Hash, AccountId, BlockNumber),
        /// (attestation, record_hash, creator, identity_type, identity) when an account creator submits an attestation
        Attest(Attestation, Hash, AccountId, IdentityType, Identity),
        /// (record_hash, verifier, id_type, identity) when a verifier approves an account
        Verify(Hash, AccountId, IdentityType, Identity),
        /// (record_hash) when an account is expired and deleted
        Expired(Hash),
        /// (identity_hash, verifier, id_type, identity) when a valid verifier denies a batch of registration/attestations
        Denied(Hash, AccountId, IdentityType, Identity),
    }
);

decl_storage! {
    trait Store for Module<T: Trait> as Identity {
        /// The hashed identities.
        pub Identities get(identities): Vec<(T::Hash)>;
        /// Actual identity for a given hash, if it's current.
        pub IdentityOf get(identity_of): map T::Hash => Option<IdentityRecord<T::AccountId, T::BlockNumber>>;
        /// List of identities awaiting attestation or verification and associated expirations
        pub IdentitiesPending get(identities_pending): Vec<(T::Hash, T::BlockNumber)>;
        /// Number of blocks allowed between register/attest or attest/verify.
        pub ExpirationLength get(expiration_length) config(): T::BlockNumber;
        /// Identity types of users
        pub UsedTypes get(used_types): map T::AccountId => Vec<IdentityType>;
        /// Verifier set
        pub Verifiers get(verifiers) config(): Vec<T::AccountId>;
        /// Registration bond
        pub RegistrationBond get(registration_bond) config(): BalanceOf<T>;
    }
}
