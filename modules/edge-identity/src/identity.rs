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
use runtime_primitives::traits::{Hash, MaybeSerializeDebug, Zero};
use runtime_support::dispatch::Result;
use runtime_support::{Parameter, StorageMap, StorageValue};
use system::ensure_signed;

pub trait Trait: system::Trait + timestamp::Trait {
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
	pub identity: Vec<u8>,
	pub stage: IdentityStage,
	pub expiration_time: Moment,
	pub proof: Option<Attestation>,
	pub verifications: [u128; 2],
	pub metadata: Option<MetadataRecord>,
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
		pub fn register(origin, identity: Vec<u8>) -> Result {
			let _sender = ensure_signed(origin)?;
			ensure!(!<FrozenAccounts<T>>::get(&_sender.clone()), "Sender account is frozen");

			let hash = T::Hashing::hash_of(&identity);
			ensure!(!<IdentityOf<T>>::exists(hash), "Identity already exists");

			let expiration = <timestamp::Module<T>>::get() + Self::expiration_time();

			<IdentityOf<T>>::insert(hash, IdentityRecord {
				account: _sender.clone(),
				identity: identity,
				stage: IdentityStage::Registered,
				expiration_time: expiration.clone(),
				proof: None,
				verifications: [0, 0],
				metadata: None,
			});

			<PendingIdentities<T>>::mutate(|identities| identities.push(hash));

			Self::deposit_event(RawEvent::Register(hash, _sender.into(), expiration));
			Ok(())
		}

		/// Attest that the sender is the original publisher of said identity
		/// by linking to an external proof.
		///
		/// Current implementation overwrites all proofs if safety checks
		/// pass.
		pub fn attest(origin, identity_hash: T::Hash, attestation: Attestation) -> Result {
			let _sender = ensure_signed(origin)?;
			ensure!(!<FrozenAccounts<T>>::get(&_sender.clone()), "Sender account is frozen");

			let record = <IdentityOf<T>>::get(&identity_hash).ok_or("Identity does not exist")?;
			ensure!(<timestamp::Module<T>>::get() <= record.expiration_time, "Identity expired");

			if record.stage == IdentityStage::Verified {
				return Err("Already verified");
			}

			// Check that original sender and current sender match
			ensure!(record.account == _sender, "Stored identity does not match sender");

			let expiration = <timestamp::Module<T>>::get() + Self::expiration_time();

			// TODO: Decide how we want to process proof updates
			// currently this implements no check against updating
			// proof links
			<IdentityOf<T>>::insert(identity_hash, IdentityRecord {
				proof: Some(attestation),
				stage: IdentityStage::Attested,
				expiration_time: expiration.clone(),
				..record
			});

			Self::deposit_event(RawEvent::Attest(identity_hash, _sender.into(), expiration));
			Ok(())
		}

		/// Verify an existing identity based on its attested proof. Sender
		/// be specified on the pre-selected list of verifiers.
		pub fn verify(origin, identity_hash: T::Hash, vote: bool, verifier_index: usize) -> Result {
			let _sender = ensure_signed(origin)?;

			let verifiers = Self::verifiers();
			ensure!(verifiers[verifier_index] == _sender, "Sender not a verifier");
			let mut record = <IdentityOf<T>>::get(&identity_hash).ok_or("Identity does not exist")?;
			match record.stage {
				IdentityStage::Registered => return Err("No attestation to verify"),
				IdentityStage::Verified => return Err("Already verified"),
				IdentityStage::Attested => ()
			}

			ensure!(<timestamp::Module<T>>::get() <= record.expiration_time, "Identity expired");

			let current_vote_res = <VerifiedIdentityBy<T>>::get((identity_hash, _sender.clone()));
			match current_vote_res {
				Some(current_vote) => {
					ensure!(vote != current_vote, "Already verified with specified vote");
					record.verifications[current_vote as usize] -= 1;
				},
				None => (),
			}

			<VerifiedIdentityBy<T>>::insert((identity_hash, _sender.clone()), vote);
			record.verifications[vote as usize] += 1;

			let yes_majority = record.verifications[1] * 3 >= 2 * verifiers.len() as u128;
			let no_majority = record.verifications[0] * 3 >= 2 * verifiers.len() as u128;
			let end_of_vote = record.verifications[0] + record.verifications[1] == verifiers.len() as u128;
			if yes_majority || no_majority || end_of_vote {
				return Self::finalize_verification(identity_hash, record, yes_majority, no_majority);
			} else {
				<IdentityOf<T>>::insert(identity_hash, IdentityRecord {
					..record
				});
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

		/// Add a claim as a claims issuer. Ensures that the sender is currently
		/// an active claims issuer. Ensures that the identity exists by checking
		/// hash exists in the Identities map.
		pub fn add_claim(origin, identity_hash: T::Hash, claim: T::Claim, issuer_index: usize) -> Result {
			let _sender = ensure_signed(origin)?;

			let issuers: Vec<T::AccountId> = Self::claims_issuers();
			ensure!(issuers[issuer_index] == _sender, "Invalid claims issuer");
			ensure!(<IdentityOf<T>>::exists(identity_hash), "Invalid identity record");

			let mut claims = Self::claims(identity_hash);
			claims.push((_sender.clone(), claim));
			<Claims<T>>::insert(identity_hash, claims);
			Ok(())
		}

		/// Remove a claim as a claims issuer. Ensures that the sender is an active
		/// claims issuer. Ensures that the sender has issued a claim over the
		/// identity provided to the module.
		pub fn remove_claim(origin, identity_hash: T::Hash, claim_index: usize) -> Result {
			let _sender = ensure_signed(origin)?;

			ensure!(<IdentityOf<T>>::exists(identity_hash), "Invalid identity record");

			let mut claims = Self::claims(identity_hash);
			ensure!(claims[claim_index].0 == _sender.clone(), "No existing claim under issuer");

			claims.remove(claim_index);
			<Claims<T>>::insert(identity_hash, claims);

			Ok(())
		}

		fn on_finalise(_n: T::BlockNumber) {
			let mut pending_identities = Self::pending_identities();
			pending_identities.retain(|hash| {
				match <IdentityOf<T>>::get(hash) {
					Some(record) => {
						if (<timestamp::Module<T>>::get() > record.expiration_time) && (record.expiration_time > T::Moment::zero()) {
							<IdentityOf<T>>::remove(hash);
							Self::deposit_event(RawEvent::Expired(*hash));
							return false;
						}
						if record.stage == IdentityStage::Verified {
							return false;
						}
						return true;
					},
					None => return false,
				};
			});
			<PendingIdentities<T>>::put(pending_identities);
		}
	}
}

impl<T: Trait> Module<T> {
	fn finalize_verification(identity_hash: T::Hash, record: IdentityRecord<T::AccountId, T::Moment>, yes_majority: bool, no_majority: bool) -> Result {
		let acct = record.account.clone();
		if yes_majority {
			let stage = IdentityStage::Verified;
			let expiration_time: T::Moment = T::Moment::zero();

			<IdentityOf<T>>::insert(identity_hash, IdentityRecord {
				stage,
				expiration_time,
				..record
			});

			Self::deposit_event(RawEvent::Verify(identity_hash, acct.clone().into(), record.verifications[1]));
		} else {
			if no_majority {
				<FrozenAccounts<T>>::insert(acct.clone(), true);
			}
			<IdentityOf<T>>::remove(identity_hash);
			Self::deposit_event(RawEvent::Failed(identity_hash, acct.into()));
		}
		Ok(())
	}
}

/// An event in this module.
decl_event!(
	pub enum Event<T> where <T as system::Trait>::Hash,
							<T as system::Trait>::AccountId,
							<T as Trait>::Claim,
							<T as timestamp::Trait>::Moment {
		Register(Hash, AccountId, Moment),
		Attest(Hash, AccountId, Moment),
		Verify(Hash, AccountId, u128),
		Failed(Hash, AccountId),
		Expired(Hash),
		AddedClaim(Hash, Claim, AccountId),
		RemovedClaim(Hash, Claim, AccountId),
	}
);

// TODO: rename "timeouts" "time limit" to ???
decl_storage! {
	trait Store for Module<T: Trait> as Identity {
		/// Actual identity for a given hash, if it's current.
		pub IdentityOf get(identity_of): map T::Hash => Option<IdentityRecord<T::AccountId, T::Moment>>;
		/// List of identities awaiting attestation or verification and associated expirations
		pub PendingIdentities get(pending_identities): Vec<T::Hash>;
		// Makes sure that one validator can validate identity only once
		pub VerifiedIdentityBy get(verified_identity_by): map (T::Hash, T::AccountId) => Option<bool>;
		/// List of malicious identities who submit failed attestations
		pub FrozenAccounts get(frozen_accounts): map T::AccountId => bool;
		/// Timestamp allowed between register/attest or attest/verify.
		pub ExpirationTime get(expiration_time) config(): T::Moment;
		/// Accounts granted power to verify identities
		pub Verifiers get(verifiers) config(): Vec<T::AccountId>;
		/// The set of active claims issuers
		pub ClaimsIssuers get(claims_issuers) config(): Vec<T::AccountId>;
		/// The claims mapping for identity records: (claims_issuer, claim)
		pub Claims get(claims): map T::Hash => Vec<(T::AccountId, T::Claim)>;

	}
}