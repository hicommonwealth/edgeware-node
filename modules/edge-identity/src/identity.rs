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
extern crate edge_voting as voting;

use codec::Encode;
use rstd::prelude::*;
use runtime_primitives::traits::{Hash};
use runtime_support::dispatch::Result;
use runtime_support::{StorageMap, StorageValue};
use system::ensure_signed;

pub trait Trait: voting::Trait {
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
	pub vote_id: u64,
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event<T>() = default;

		/// Register an identity with the hash of the signature. Ensures that
		/// all identities are unique, so that no duplicate identities can be
		/// registered.
		///
		/// Current implementation suffers from squatter attacks. Additional
		/// implementations could provide a mechanism for a trusted set of
		/// authorities to delete a squatted identity OR implement storage
		/// rent to disincentivize it.
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
				vote_id: 0,
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
			// Get record and check if it is verified
			let record = <IdentityOf<T>>::get(&identity_hash).ok_or("Identity does not exist")?;
			ensure!(record.stage != IdentityStage::Verified, "Identity already verified");
			// Check that original sender and current sender match
			ensure!(record.account == _sender, "Stored identity does not match sender");
			// Reset expiration
			let expiration = <system::Module<T>>::block_number() + Self::expiration_time();
			// create a vote to go along with the attested identity
			let vote_id = <voting::Module<T>>::create_vote(
				_sender.clone(),
				voting::VoteType::Binary,
				false, // not commit-reveal
				voting::TallyType::OneCoin,
				vec![voting::YES_VOTE, voting::NO_VOTE],
			)?;
			// Allow voting upon attestation
			<voting::Module<T>>::advance_stage(record.vote_id)?;
			// TODO: Decide how we want to process proof updates
			// currently this implements no check against updating
			// proof links
			<IdentityOf<T>>::insert(identity_hash, IdentityRecord {
				proof: Some(attestation),
				stage: IdentityStage::Attested,
				expiration_time: Some(expiration),
				vote_id: vote_id,
				..record
			});

			<IdentitiesPending<T>>::mutate(|idents| {
				idents.retain(|(hash, _)| hash != &identity_hash);
				idents.push((identity_hash, expiration))
			});

			Self::deposit_event(RawEvent::Attest(identity_hash, _sender.into()));
			Ok(())
		}

		/// Register and attest to an identity
		pub fn register_and_attest(
			origin,
			identity_type: Vec<u8>,
			identity: Vec<u8>,
			identity_hash: T::Hash,
			attestation: Attestation
		) -> Result {
			Self::register(origin, identity_type, identity);
			Self::attest(origin, identity_hash, attestation);
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
		fn on_finalise(n: T::BlockNumber) {
			let (expired, valid): (Vec<_>, _) = <IdentitiesPending<T>>::get()
				.into_iter()
				.partition(|(_, expiration)| n >= *expiration);
			<IdentitiesPending<T>>::put(valid);

			expired.into_iter().for_each(move |(exp_hash, _)| {
				if let Some(record) = <IdentityOf<T>>::get(exp_hash) {
					match record.stage {
						// If the expired record has only been registered, remove it
						Registered => {
							Self::remove_pending_identity(&exp_hash);
							Self::deposit_event(RawEvent::Expired(exp_hash))
						},
						// If the expired record has been attested, tally it
						Attested => {
							let vote_id = record.vote_id;
							// Voting should be completed once the record is expiring
							<voting::Module<T>>::advance_stage(record.vote_id).unwrap();
							// Tally the vote
							match <voting::Module<T>>::get_winning_outcome(vote_id) {
								Some(outcome) => {
									if outcome == voting::YES_VOTE {
										<IdentityOf<T>>::insert(exp_hash, IdentityRecord {
											stage: IdentityStage::Verified,
											expiration_time: None,
											..record
										});
									} else {
										Self::remove_pending_identity(&exp_hash);
										Self::deposit_event(RawEvent::Expired(exp_hash))
									}
								},
								None => {
									Self::remove_pending_identity(&exp_hash);
									Self::deposit_event(RawEvent::Expired(exp_hash))
								},
							};
						},
						_ => ()
					}
				}
			});
			
		}
	}
}

impl<T: Trait> Module<T> {
	/// Removes all data about a pending identity given the hash of the record
	fn remove_pending_identity(identity_hash: &T::Hash) {
		<Identities<T>>::mutate(|idents| idents.retain(|hash| hash != identity_hash));
		<IdentityOf<T>>::remove(identity_hash);
		<IdentitiesPending<T>>::mutate(|idents| {
			idents.retain(|(hash, _)| hash != identity_hash)
		});
	}
}

/// An event in this module.
decl_event!(
	pub enum Event<T> where <T as system::Trait>::Hash, <T as system::Trait>::AccountId {
		Register(Hash, AccountId),
		Attest(Hash, AccountId),
		Verify(Hash, AccountId, Vec<AccountId>),
		Failed(Hash, AccountId),
		Expired(Hash),
	}
);

// TODO: rename "timeouts" "time limit" to ???
decl_storage! {
	trait Store for Module<T: Trait> as Identity {
		/// The hashed identities.
		pub Identities get(identities): Vec<(T::Hash)>;
		/// Identity types of users
		pub UsedType get(used_type): map T::Hash => bool;
		/// Actual identity for a given hash, if it's current.
		pub IdentityOf get(identity_of): map T::Hash => Option<IdentityRecord<T::AccountId, T::BlockNumber>>;
		/// List of identities awaiting attestation or verification and associated expirations
		pub IdentitiesPending get(identities_pending): Vec<(T::Hash, T::BlockNumber)>;
		/// List of malicious identities who submit failed attestations
		pub FrozenAccounts get(frozen_accounts): Vec<T::AccountId>;
		/// Number of blocks allowed between register/attest or attest/verify.
		pub ExpirationTime get(expiration_time) config(): T::BlockNumber;

	}
}
