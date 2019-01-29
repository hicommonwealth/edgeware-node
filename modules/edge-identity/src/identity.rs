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

use rstd::prelude::*;
use runtime_primitives::traits::Hash;
use runtime_support::dispatch::Result;
use runtime_support::{StorageMap, StorageValue};
use runtime_primitives::traits::EnsureOrigin;
use system::ensure_signed;
use codec::Encode;

pub trait Trait: system::Trait {
	/// Origin from which approvals must come.
	type ApproveOrigin: EnsureOrigin<Self::Origin>;

	/// Origin from which rejections must come.
	type RejectOrigin: EnsureOrigin<Self::Origin>;

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
	pub identity: Vec<u8>,
	pub stage: IdentityStage,
	pub expiration_time: Option<BlockNumber>,
	pub proof: Option<Attestation>,
	pub metadata: Option<MetadataRecord>,
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event<T>() = default;

		pub fn register(origin, identity_type: Vec<u8>, identity: Vec<u8>) -> Result {
			let _sender = ensure_signed(origin)?;
			ensure!(!Self::frozen_accounts().iter().any(|i| i == &_sender.clone()), "Sender account is frozen");
			// Check that the sender hasn't used this identity type before
			let mut buf = Vec::new();
			buf.extend_from_slice(&_sender.encode());
			buf.extend_from_slice(&identity_type.encode());
			let type_hash = T::Hashing::hash(&buf[..]);
			ensure!(!<UsedType<T>>::exists(type_hash), "Identity type already used");
			// Hash the identity type with the identity to use as a key for the mapping
			buf = Vec::new();
			buf.extend_from_slice(&identity_type.encode());
			buf.extend_from_slice(&identity.encode());
			let hash = T::Hashing::hash(&buf[..]);
			ensure!(!<IdentityOf<T>>::exists(hash), "Identity already exists");
			// Set expiration time of identity
			let expiration = <system::Module<T>>::block_number() + Self::expiration_time();
			// Add identity record
			<Identities<T>>::mutate(|idents| idents.push(hash.clone()));
			<IdentityOf<T>>::insert(hash, IdentityRecord {
				account: _sender.clone(),
				identity: identity,
				stage: IdentityStage::Registered,
				expiration_time: Some(expiration),
				proof: None,
				metadata: None,
			});
			<IdentitiesPending<T>>::mutate(|idents| idents.push((hash, expiration)));
			// Fire register event
			Self::deposit_event(RawEvent::Register(hash, _sender.into()));
			Ok(())
		}

		/// Attest that the sender is the original publisher of said identity
		/// by linking to an external proof.
		///
		/// Current implementation overwrites all proofs if safety checks
		/// pass.
		pub fn attest(origin, identity_hash: T::Hash, attestation: Attestation) -> Result {
			let _sender = ensure_signed(origin)?;
			ensure!(!Self::frozen_accounts().iter().any(|i| i == &_sender.clone()), "Sender account is frozen");
			// Grab record
			let record = <IdentityOf<T>>::get(&identity_hash).ok_or("Identity does not exist")?;
			// Ensure the record isn't expired if it still exists
			if let Some(time) = record.expiration_time {
				ensure!(time >= <system::Module<T>>::block_number(), "Identity has reached expiration");
			}
			if record.stage == IdentityStage::Verified {
				return Err("Already verified");
			}

			// Check that original sender and current sender match
			ensure!(record.account == _sender, "Stored identity does not match sender");

			let expiration = <system::Module<T>>::block_number() + Self::expiration_time();

			// TODO: Decide how we want to process proof updates
			// currently this implements no check against updating
			// proof links
			<IdentityOf<T>>::insert(identity_hash, IdentityRecord {
				proof: Some(attestation),
				stage: IdentityStage::Attested,
				expiration_time: Some(expiration),
				..record
			});

			<IdentitiesPending<T>>::mutate(|idents| {
				idents.retain(|(hash, _)| hash != &identity_hash);
				idents.push((identity_hash, expiration))
			});

			Self::deposit_event(RawEvent::Attest(identity_hash, _sender.into()));
			Ok(())
		}

		/// Propose verification to be voted upon by the council
		pub fn verify_or_deny(origin, identity_hash: T::Hash, approve: bool, verifier_index: usize) -> Result {
			let _sender = ensure_signed(origin)?;
			ensure!(verifier_index < Self::verifiers().len(), "Verifier index out of bounds");
			ensure!(Self::verifiers()[verifier_index] == _sender.clone(), "Sender is not a verifier");
			let record = <IdentityOf<T>>::get(&identity_hash).ok_or("Identity does not exist")?;
			match record.stage {
				IdentityStage::Registered => return Err("No attestation to verify"),
				IdentityStage::Verified => return Err("Already verified"),
				IdentityStage::Attested => ()
			}

			if approve {
				<IdentityOf<T>>::insert(identity_hash, IdentityRecord {
					stage: IdentityStage::Verified,
					expiration_time: None,
					..record
				});
				<IdentitiesPending<T>>::mutate(|idents| idents.retain(|(hash, _)| hash != &identity_hash));
				Self::deposit_event(RawEvent::Verify(identity_hash, _sender))
			} else {
				Self::remove_pending_identity(&identity_hash);
				Self::deposit_event(RawEvent::Expired(identity_hash))
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
		fn on_finalise(n: T::BlockNumber) {
			let (expired, valid): (Vec<_>, _) = <IdentitiesPending<T>>::get()
				.into_iter()
				.partition(|(_, expiration)| n >= *expiration);

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
		// If triggered by a malicious party's actions, delete all data
		<Identities<T>>::mutate(|idents| idents.retain(|hash| hash != identity_hash));
		<IdentityOf<T>>::remove(identity_hash);
		<IdentitiesPending<T>>::mutate(|idents| idents.retain(|(hash, _)| hash != identity_hash));
	}
}

/// An event in this module.
decl_event!(
	pub enum Event<T> where <T as system::Trait>::Hash, <T as system::Trait>::AccountId {
		Register(Hash, AccountId),
		Attest(Hash, AccountId),
		Verify(Hash, AccountId),
		Failed(Hash, AccountId),
		Expired(Hash),
	}
);

// TODO: rename "timeouts" "time limit" to ???
decl_storage! {
	trait Store for Module<T: Trait> as Identity {
		/// The hashed identities.
		pub Identities get(identities): Vec<(T::Hash)>;
		/// Actual identity for a given hash, if it's current.
		pub IdentityOf get(identity_of): map T::Hash => Option<IdentityRecord<T::AccountId, T::BlockNumber>>;
		/// List of identities awaiting attestation or verification and associated expirations
		pub IdentitiesPending get(identities_pending): Vec<(T::Hash, T::BlockNumber)>;
		/// List of malicious identities who submit failed attestations
		pub FrozenAccounts get(frozen_accounts): Vec<T::AccountId>;
		/// Number of blocks allowed between register/attest or attest/verify.
		pub ExpirationTime get(expiration_time) config(): T::BlockNumber;
		/// Identity types of users
		pub UsedType get(used_type): map T::Hash => bool;
		/// Verifier set
		pub Verifiers get(verifiers) config(): Vec<T::AccountId>;
	}
}
