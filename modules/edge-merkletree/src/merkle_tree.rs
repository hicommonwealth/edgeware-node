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
//#[cfg(feature = "std")]

extern crate parity_codec as codec;
extern crate substrate_primitives as primitives;
extern crate sr_std as rstd;
extern crate srml_support as runtime_support;
extern crate sr_primitives as runtime_primitives;
extern crate sr_io as runtime_io;
extern crate srml_balances as balances;
extern crate srml_system as system;
extern crate edge_delegation as delegation;

use rstd::prelude::*;
use system::ensure_signed;
use runtime_support::{StorageValue, StorageMap};
use runtime_support::dispatch::Result;
use runtime_primitives::traits::Hash;
use runtime_primitives::traits::{Zero};
use codec::Encode;

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, PartialEq)]
pub struct MTree<Hash, Balance> {
	pub root: Hash,
    pub leaves: Option<Vec<Hash>>,
    pub fee: Balance,
}


pub trait Trait: balances::Trait + delegation::Trait {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event<T>() = default;

        pub fn create_tree(origin) -> Result {
            let _sender = ensure_signed(origin)?;
            let ctr = Self::number_of_trees();
            <NumberOfTrees<T>>::put(ctr + 1);
            <MerkleTrees<T>>::insert(ctr, MTree {
                root: T::Hashing::hash_of(b"0"),
                leaves: None,
                fee: Zero::zero(),
            });
            Ok(())
        }

        pub fn add_leaf(origin, tree_id: u32, leaf_value: T::Hash) -> Result {
            let _sender = ensure_signed(origin)?;
            let tree = <MerkleTrees<T>>::get(tree_id).ok_or("Tree doesn't exist")?;
            ensure!(<balances::Module<T>>::free_balance(_sender.clone()) > tree.fee, "Insufficient balance from sender");    
            
            if let Some(mut leaves) = tree.leaves {
                leaves.push(leaf_value);
                let new_root = Self::compute_new_root(leaves.clone(), None);
                <MerkleTrees<T>>::insert(tree_id, MTree {
                    root: new_root,
                    leaves: Some(leaves),
                    ..tree
                });
            }

            
            Ok(())
        }

        pub fn prove_membership(origin, tree_id: u32, leaf_value: T::Hash, path: Vec<(T::Hash, bool)>) -> Result {
			let _sender = ensure_signed(origin)?;
            let tree = <MerkleTrees<T>>::get(tree_id).ok_or("Tree doesn't exist")?;

            let mut hash = leaf_value.clone();
            for (elt, side) in path.into_iter() {
            	let mut buf = Vec::new();
            	if side {
	            	buf.extend_from_slice(&hash.encode());
	            	buf.extend_from_slice(&elt.encode());            		
            	} else {
	            	buf.extend_from_slice(&elt.encode());
	            	buf.extend_from_slice(&hash.encode());
            	}
            	hash = T::Hashing::hash_of(&buf);
            }

            ensure!(hash == tree.root, "Invalid merkle path proof");
            Ok(())
        }

        pub fn prove_with_zk(origin, tree_id: u32, nullifier: T::Hash, proof: Vec<u8>) -> Result {
        	Ok(())
        }
	}
}

impl<T: Trait> Module<T> {
    fn compute_new_root(leaves: Vec<T::Hash>, length: Option<usize>) -> T::Hash {
        let mut limit = 0;
        if let Some(len) = length {
            if len == 2 {
                let mut buf = Vec::new();
                buf.extend_from_slice(&leaves[0].encode());
                buf.extend_from_slice(&leaves[1].encode());
                return T::Hashing::hash_of(&buf);
            }

            limit = len;
        }

        let zero_hash = T::Hashing::hash_of(b"0");
        let mut result: Vec<T::Hash> = Vec::new();
        let mut lf_len = 0;
        if length.is_none() {
            lf_len = leaves.len();
            limit = Self::get_upper_count(lf_len);
        }

        let mut ctr = 0;
        while ctr < limit {
            // Pull elements from array, check for even or odd
            let mut fst = zero_hash;
            let mut snd = zero_hash;

            if ctr < lf_len {
                fst = leaves[ctr];
                if ctr + 1 < lf_len { snd = leaves[ctr + 1] }
            }

            let mut buf = Vec::new();
            buf.extend_from_slice(&fst.encode());
            buf.extend_from_slice(&snd.encode());
            result.push(T::Hashing::hash_of(&buf));
            ctr += 2;
        }

        return Self::compute_new_root(result, Some(limit / 2));
    }

    fn get_upper_count(length: usize) -> usize {
        let mut ctr = 1;
        while (1 << ctr) < length {
            ctr += 1
        }

        (1 << ctr)
    }    
}

/// An event in this module.
decl_event!(
	pub enum Event<T> where <T as system::Trait>::Hash {
		/// new vote (id, creator, type of vote)
		NewLeaf(Hash, Hash),
	}
);

decl_storage! {
	trait Store for Module<T: Trait> as MerkleTree {
		pub NumberOfTrees get(number_of_trees): u32;
		pub MerkleTrees get(merkle_trees): map u32 => Option<MTree<T::Hash, T::Balance>>;
	}
}
