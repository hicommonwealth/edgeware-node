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
extern crate srml_timestamp as timestamp;

use rstd::prelude::*;
use runtime_primitives::traits::{Zero, Hash};
use runtime_support::dispatch::Result;
use runtime_support::{StorageMap, StorageValue};
use system::ensure_signed;
use codec::Encode;

pub trait Trait: timestamp::Trait {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

pub type Attestation = Vec<u8>;
pub type IdentityType = Vec<u8>;
pub type Identity = Vec<u8>;

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
pub struct IdentityRecord<AccountId, Moment> {
	pub account: AccountId,
	pub identity_type: IdentityType,
	pub identity: Identity,
	pub stage: IdentityStage,
	pub expiration_time: Moment,
	pub proof: Option<Attestation>,
	pub metadata: Option<MetadataRecord>,
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event<T>() = default;

		/// A function that registers an identity_type and identity for a user
		///
		/// Checks whether the (identity_type, identity) pair exists and creates
		/// the record if now. The record is indexed by the hash of the pair.
		pub fn register(origin, identity_type: IdentityType, identity: Identity) -> Result {
			let _sender = ensure_signed(origin)?;
			ensure!(!<UsedTypes<T>>::get(_sender.clone()).iter().any(|i| i == &identity_type), "Identity type already used");
			let mut buf = Vec::new();
			buf.extend_from_slice(&identity_type.encode());
			buf.extend_from_slice(&identity.encode());
			let hash = T::Hashing::hash(&buf[..]);
			ensure!(!<IdentityOf<T>>::exists(hash), "Identity already exists");
			return Self::register_identity(_sender, identity_type, identity, hash);
		}

		/// A function that creates an identity attestation
		///
		/// Attestation is only valid if the identity is in the attestation phase
		/// and is verified off-chain using an off-chain worker node. Current
		/// implementation overwrites all proofs if safety checks pass.
		pub fn attest(origin, identity_hash: T::Hash, attestation: Attestation) -> Result {
			let _sender = ensure_signed(origin)?;
			// Grab record
			let record = <IdentityOf<T>>::get(&identity_hash).ok_or("Identity does not exist")?;
			// Ensure the record is not verified
			ensure!(record.stage != IdentityStage::Verified, "Already verified");
			// Ensure the record isn't expired if it still exists
			ensure!(<timestamp::Module<T>>::get() <= record.expiration_time, "Identity expired");
			// Check that original sender and current sender match
			ensure!(record.account == _sender, "Stored identity does not match sender");


			return Self::attest_for(_sender, identity_hash, attestation);
		}

		/// A function that registers and attests to an identity simultaneously.
		///
		/// Allows more efficient registration and attestation processing since it
		/// requires only 1 transaction.
		pub fn register_and_attest(origin, identity_type: IdentityType, identity: Identity, attestation: Attestation) -> Result {
			let _sender = ensure_signed(origin)?;
			// Check hash
			let mut buf = Vec::new();
			buf.extend_from_slice(&identity_type.encode());
			buf.extend_from_slice(&identity.encode());
			let hash = T::Hashing::hash(&buf[..]);
			ensure!(!<IdentityOf<T>>::exists(hash), "Identity already exists");
			// Register identity
			Self::register_identity(_sender.clone(), identity_type, identity, hash).unwrap();
			// Grab record
			let record = <IdentityOf<T>>::get(&hash).ok_or("Identity does not exist")?;
			// Ensure the record is not verified
			ensure!(record.stage != IdentityStage::Verified, "Already verified");
			// Ensure the record isn't expired if it still exists
			ensure!(<timestamp::Module<T>>::get() <= record.expiration_time, "Identity expired");
			// Check that original sender and current sender match
			ensure!(record.account == _sender.clone(), "Stored identity does not match sender");
			return Self::attest_for(_sender, hash, attestation);
		}

		/// A function that verifies or denies an identity attestation.
		/// 
		/// The verification is handled by a set of seeded verifiers who run
		/// the off-chain worker node to verify attestations.
		pub fn verify_or_deny(origin, identity_hash: T::Hash, approve: bool, verifier_index: usize) -> Result {
			let _sender = ensure_signed(origin)?;
			ensure!(verifier_index < Self::verifiers().len(), "Verifier index out of bounds");
			ensure!(Self::verifiers()[verifier_index] == _sender.clone(), "Sender is not a verifier");
			return Self::verify_or_deny_identity(_sender, &identity_hash, approve);
		}

		/// Deny many verification requests
		pub fn verify_or_deny_many(origin, identity_hashes: Vec<T::Hash>, approvals: Vec<bool>, verifier_index: usize) -> Result {
			let _sender = ensure_signed(origin)?;
			ensure!(verifier_index < Self::verifiers().len(), "Verifier index out of bounds");
			ensure!(Self::verifiers()[verifier_index] == _sender.clone(), "Sender is not a verifier");
			
			for i in 0..identity_hashes.len() {
				Self::verify_or_deny_identity(_sender.clone(), &identity_hashes[i], approvals[i]).unwrap();
			}

			Ok(())
		}

		/// Add metadata to sender's account.
		// TODO: make all options and only updated provided?
		// TODO: limit the max length of these user-submitted types?
		pub fn add_metadata(origin, identity_hash: T::Hash, avatar: Vec<u8>, display_name: Vec<u8>, tagline: Vec<u8>) -> Result {
			let _sender = ensure_signed(origin)?;
			let record = <IdentityOf<T>>::get(&identity_hash).ok_or("Identity does not exist")?;

			// Check that original sender and current sender match
			ensure!(record.account == _sender, "Stored identity does not match sender");
			ensure!(<timestamp::Module<T>>::get() <= record.expiration_time, "Identity expired");

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

		/// Check all pending identities for expiration when each block is
		/// finalised. Once an identity expires, it is deleted from storage.
		/// TODO: We may want to limit how many identities will be purged each block.
		fn on_finalise(_n: T::BlockNumber) {
			let (expired, valid): (Vec<_>, _) = <IdentitiesPending<T>>::get()
				.into_iter()
				.partition(|(_, exp)| (<timestamp::Module<T>>::get() > *exp) && (*exp > T::Moment::zero()));

			expired.into_iter().for_each(move |(exp_hash, _)| {
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
		ensure!(<timestamp::Module<T>>::get() <= record.expiration_time, "Identity expired");
		match record.stage {
			IdentityStage::Registered => return Err("No attestation to verify"),
			IdentityStage::Verified => return Err("Already verified"),
			IdentityStage::Attested => ()
		}

		let id_type = record.identity_type.encode().clone();
		let id = record.identity.encode().clone();
		if approve {
			<IdentitiesPending<T>>::mutate(|idents| idents.retain(|(hash, _)| hash != identity_hash));
			Self::deposit_event(RawEvent::Verify(*identity_hash, sender, id_type, id));
			<IdentityOf<T>>::insert(identity_hash, IdentityRecord {
				stage: IdentityStage::Verified,
				expiration_time: T::Moment::zero(),
				..record
			});
		} else {
			Self::remove_pending_identity(&identity_hash);
			Self::deposit_event(RawEvent::Denied(*identity_hash, sender, id_type, id));
		}

		Ok(())
	}

	/// Helper function for executing the registration of identities
	fn register_identity(sender: T::AccountId, identity_type: IdentityType, identity: Identity, identity_hash: T::Hash) -> Result {
		// Hash the identity type with the identity to use as a key for the mapping
		let mut types = <UsedTypes<T>>::get(sender.clone());
		types.push(identity_type.clone());
		<UsedTypes<T>>::insert(sender.clone(), types);

		// Set expiration time of identity
		let expiration = <timestamp::Module<T>>::get() + Self::expiration_time();
		// Add identity record
		<Identities<T>>::mutate(|idents| idents.push(identity_hash.clone()));
		<IdentityOf<T>>::insert(identity_hash, IdentityRecord {
			account: sender.clone(),
			identity_type: identity_type,
			identity: identity,
			stage: IdentityStage::Registered,
			expiration_time: expiration.clone(),
			proof: None,
			metadata: None,
		});
		<IdentitiesPending<T>>::mutate(|idents| idents.push((identity_hash, expiration.clone())));
		// Fire register event
		Self::deposit_event(RawEvent::Register(identity_hash, sender.into(), expiration));
		Ok(())
	}

	/// Helper function for executing the attestation of identities
	fn attest_for(sender: T::AccountId, identity_hash: T::Hash, attestation: Attestation) -> Result {
		// Grab record
		let record = <IdentityOf<T>>::get(&identity_hash).ok_or("Identity does not exist")?;
		let id_type = record.identity_type.clone();
		let identity = record.identity.clone();
		let expiration = <timestamp::Module<T>>::get() + Self::expiration_time();

		// TODO: Decide how we want to process proof updates
		// currently this implements no check against updating
		// proof links
		<IdentityOf<T>>::insert(identity_hash, IdentityRecord {
			proof: Some(attestation.clone()),
			stage: IdentityStage::Attested,
			expiration_time: expiration.clone(),
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
							<T as timestamp::Trait>::Moment {
		/// (record_hash, creator, expiration) when an account is registered
		Register(Hash, AccountId, Moment),
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
		pub IdentityOf get(identity_of): map T::Hash => Option<IdentityRecord<T::AccountId, T::Moment>>;
		/// List of identities awaiting attestation or verification and associated expirations
		pub IdentitiesPending get(identities_pending): Vec<(T::Hash, T::Moment)>;
		/// Number of blocks allowed between register/attest or attest/verify.
		pub ExpirationTime get(expiration_time) config(): T::Moment;
		/// Identity types of users
		pub UsedTypes get(used_types): map T::AccountId => Vec<IdentityType>;
		/// Verifier set
		pub Verifiers get(verifiers) config(): Vec<T::AccountId>;
	}
}
