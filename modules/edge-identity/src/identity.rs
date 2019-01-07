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
extern crate parity_codec as codec;
extern crate sr_io as runtime_io;
extern crate sr_primitives as runtime_primitives;
extern crate sr_std as rstd;
extern crate srml_support as runtime_support;
extern crate substrate_primitives as primitives;

extern crate srml_balances as balances;
extern crate srml_system as system;

use codec::Codec;
use rstd::prelude::*;
use runtime_primitives::traits::{As, Hash, MaybeSerializeDebug, Member, SimpleArithmetic};
use runtime_support::dispatch::Result;
use runtime_support::{Parameter, StorageMap, StorageValue};
use system::ensure_signed;

pub trait Trait: session::Trait {
    /// The claims type
    type Claim: Parameter + MaybeSerializeDebug;
    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

pub type Attestation = Vec<u8>;

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, PartialEq)]
pub struct MetadataRecord {
    pub avatar: Vec<u8>,
    pub display_name: Vec<u8>,
    pub tagline: Vec<u8>,
}

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, Copy, Clone, Eq, PartialEq, Default)]
pub enum IdentityStage {
    Registered,
    Attested,
    Verified
}

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, PartialEq)]
pub struct IdentityRecord<AccountId> {
    pub account: AccountId,
    pub identity: Vec<u8>,
    pub stage: IdentityStage,
    pub proof: Option<Attestation>,
    pub metadata: Option<MetadataRecord>,
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        /// Publish an identity with the hash of the signature. Ensures that
        /// all identities are unique, so that no two identities of the same
        /// can be published.
        ///
        /// Current implementation suffers from squatter attacks. Additional
        /// implementations could provide a mechanism for a trusted set of
        /// authorities to delete a squatted identity OR implement storage
        /// rent to disincentivize it.
        pub fn register(origin, identity: Vec<u8>) -> Result {
            let _sender = ensure_signed(origin)?;
            let hash = T::Hashing::hash_of(&identity);

            ensure!(!<IdentityOf<T>>::exists(hash), "Identity already exists");

            let mut idents = Self::identities();
            idents.push(hash);
            <Identities<T>>::put(idents);

            let record = IdentityRecord {
                account: _sender.clone(),
                identity: identity,
                stage: Registered,
                proof: None,
                metadata: None,
            };
            <IdentityOf<T>>::insert(hash, record);

            // TODO: configure this expiration
            let expiration = <system::Module<T>>::block_number() + 1000;
            let mut timeouts = Self::identity_timeouts();
            timeouts.push((hash, expiration));
            <IdentityTimeouts<T>>::put(timeouts);

            Self::deposit_event(RawEvent::Register(hash, _sender.into()));
            Ok(())
        }

        /// Link an external proof to an existing identity iff the sender
        /// is the original publisher of said identity.
        ///
        /// Current implementation overwrites all proofs if safety checks
        /// pass.
        pub fn attest(origin, identity_hash: T::Hash, attestation: Attestation) -> Result {
            let _sender = ensure_signed(origin)?;
            let record = <IdentityOf<T>>::get(&identity_hash).ok_or("Identity does not exist")?;

            // Check that original sender and current sender match
            ensure!(record.account == _sender, "Stored identity does not match sender");

            // TODO: Decide how we want to process proof updates
            // currently this implements no check against updating
            // proof links
            let mut new_record = record;
            new_record.proof = Some(attestation);
            new_record.stage = Attested;
            <IdentityOf<T>>::insert(identity_hash, new_record);

            // bump timeout before verification
            // TODO: configure this timeout
            let expiration = <system::Module<T>>::block_number() + 1000;
            let mut timeouts = Self::identity_timeouts();
            timeouts.retain(|(hash, _)| hash != identity_hash);
            timeouts.push((identity_hash, expiration));
            <IdentityTimeouts<T>>::put(timeouts);

            Self::deposit_event(RawEvent::Attest(identity_hash, _sender.into()));
            Ok(())
        }

        pub fn verify(origin, identity_hash: T::Hash) -> Result {
            let _sender = ensure_signed(origin)?;
            ensure!(Self::verifiers().contains(&_sender), "Sender not a verifier");

            let record = <IdentityOf<T>>::get(&identity_hash).ok_or("Identity does not exist")?;
            let mut new_record = record;
            new_record.stage = Verified;
            <IdentityOf<T>>::insert(identity_hash, new_record);

            let mut timeouts = Self::identity_timeouts();
            timeouts.retain(|(hash, _)| hash != identity_hash);
            <IdentityTimeouts<T>>::put(timeouts);
            Ok(())
        }

        /// Add metadata to sender's account.
        // TODO: make all options and only updated provided?
        // TODO: limit the max length of these user-submitted types?
        // TODO: add a field relating to verification?
        pub fn add_metadata(origin, identity_hash: T::Hash, avatar: Vec<u8>, display_name: Vec<u8>, tagline: Vec<u8>) -> Result {
            let _sender = ensure_signed(origin)?;
            let record = <IdentityOf<T>>::get(&identity_hash).ok_or("Identity does not exist")?;

            // Check that original sender and current sender match
            ensure!(record.account == _sender, "Stored identity does not match sender");

            // TODO: Decide how to process metadata updates, for now it's all or nothing
            let mut new_record = record;
            new_record.metadata = Some(MetadataRecord {
                avatar: avatar,
                display_name: display_name,
                tagline: tagline,
            });
            <IdentityOf<T>>::insert(identity_hash, new_record);
            // TODO: worth adding an event?
            Ok(())
        }

        /// Add a claim as a claims issuer. Ensures that the sender is currently
        /// an active claims issuer. Ensures that the identity exists by checking
        /// hash exists in the Identities map.
        pub fn add_claim(origin, identity_hash: T::Hash, claim: T::Claim) -> Result {
            let _sender = ensure_signed(origin)?;

            let issuers: Vec<T::AccountId> = Self::claims_issuers();
            ensure!(issuers.iter().any(|id| id == &_sender), "Invalid claims issuer");
            ensure!(<IdentityOf<T>>::exists(identity_hash), "Invalid identity record");

            let mut claims = Self::claims(identity_hash);
            claims.push((_sender.clone(), claim));
            <Claims<T>>::insert(identity_hash, claims);
            Ok(())
        }

        /// Remove a claim as a claims issuer. Ensures that the sender is an active
        /// claims issuer. Ensures that the sender has issued a claim over the
        /// identity provided to the module.
        pub fn remove_claim(origin, identity_hash: T::Hash) -> Result {
            let _sender = ensure_signed(origin)?;

            let issuers: Vec<T::AccountId> = Self::claims_issuers();
            ensure!(issuers.iter().any(|id| id == &_sender), "Invalid claims issuer");
            ensure!(<IdentityOf<T>>::exists(identity_hash), "Invalid identity record");

            let mut claims = Self::claims(identity_hash);
            ensure!(claims.iter().any(|claim| claim.0 == _sender.clone()), "No existing claim under issuer");

            let index = claims.iter().position(|claim| claim.0 == _sender.clone()).unwrap();
            claims.remove(index);
            <Claims<T>>::insert(identity_hash, claims);

            Ok(())
        }
    }
}

impl<T: Trait> Module<T> {
    fn delete_identity(identity_hash: T::Hash) {
        // assumes the identity has already been removed from timeout vectors
        let mut idents = Self::identities();
        idents.retain(|&hash| hash != identity_hash);
        <Identities<T>>::put(idents);
        <IdentityOf<T>>::remove(identity_hash);
    }

    fn new_session() {
        let currBlock = <system::Module<T>>::block_number();
        let (expired, valid) = Self::identity_timeouts()
            .into_iter()
            .partition(|(_, expiration)| currBlock > expiration);
        expired.for_each(move |(hash, _)| Self::delete_identity(hash));
        <IdentityTimeouts<T>>::put(valid);
    }
}

impl<X, T: Trait> session::OnSessionChange<X> for Module<T>
{
    fn on_session_change(_: X, _: bool) {
        Self::new_session();
    }
}

/// An event in this module.
decl_event!(
    pub enum Event<T> where <T as system::Trait>::Hash,
                            <T as system::Trait>::AccountId,
                            <T as Trait>::Claim {
        Register(Hash, AccountId),
        Attest(Hash, AccountId),
        AddedClaim(Hash, Claim, AccountId),
        RemovedClaim(Hash, Claim, AccountId),
    }
);

decl_storage! {
    trait Store for Module<T: Trait> as Identity {
        /// The hashed identities.
        pub Identities get(identities): Vec<(T::Hash)>;
        /// List of identities awaiting attestation or verification.
        pub IdentityTimeouts get(identity_timeouts): Vec<(T::Hash, T::BlockNumber)>;
        /// Actual identity for a given hash, if it's current.
        pub IdentityOf get(identity_of): map T::Hash => Option<IdentityRecord<T::AccountId>>;
        /// Accounts granted power to verify identities
        pub Verifiers get(verifiers) config(): Vec<T::AccountId>;
        /// The set of active claims issuers
        pub ClaimsIssuers get(claims_issuers) config(): Vec<T::AccountId>;
        /// The claims mapping for identity records: (claims_issuer, claim)
        pub Claims get(claims): map T::Hash => Vec<(T::AccountId, T::Claim)>;

    }
}
