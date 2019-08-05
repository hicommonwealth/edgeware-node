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

#![no_std]
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
// extern crate num_bigint;
// extern crate num_traits;
// extern crate bellman;
// extern crate sapling_crypto;

// use bellman::pairing::bn256::Bn256;
// use bellman::pairing::bn256::Fr;
// use bellman::pairing::ff::{BitIterator, PrimeField, Field};
// use sapling_crypto::{
//     babyjubjub::{
//         JubjubBn256,
//     },
// };
// use num_traits::Num;
use rstd::prelude::*;
use system::ensure_signed;
use runtime_support::{StorageValue, StorageMap};
use runtime_support::dispatch::Result;

use runtime_primitives::traits::{Zero};
use codec::Encode;


// use bellman::groth16::{Proof, Parameters, verify_proof, prepare_verifying_key};
// use num_bigint::BigInt;

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, PartialEq)]
pub struct MTree<Balance> {
    pub fee: Balance,
    pub depth: u32,
    pub leaf_count: u32
}

const DEFAULT_TREE_DEPTH: u32 = 31;
// TODO: Better estimates/decisions
const MAX_DEPTH: u32 = 31;

pub trait Trait: balances::Trait {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event<T>() = default;

        pub fn create_tree(origin, _fee: Option<T::Balance>, _depth: Option<u32>, _leaves: Option<Vec<Vec<u8>>>) -> Result {
            let _sender = ensure_signed(origin)?;

            let fee = match _fee {
                Some(f) => f,
                None => Zero::zero(),
            };

            let depth = match _depth {
                Some(d) => d,
                None => DEFAULT_TREE_DEPTH,
            };
            ensure!(depth <= MAX_DEPTH, "Fee is too large");

            let ctr = Self::number_of_trees();
            for i in 0..depth {
                <MerkleTreeLevels<T>>::insert((ctr, i), vec![]);
            }

            let mtree = MTree {
                fee: fee,
                depth: depth,
                leaf_count: 0,
            };
            
            <MerkleTreeMetadata<T>>::insert(ctr, mtree);
            <NumberOfTrees<T>>::put(ctr + 1);

            if let Some(leaves) = _leaves {
                for i in 0..leaves.len() {
                    Self::add_leaf_element(ctr, leaves[i].clone());
                }
            }

            Ok(())
        }

        pub fn add_leaf(origin, tree_id: u32, leaf_value: T::Hash) -> Result {
            let _sender = ensure_signed(origin)?;
            let tree = <MerkleTreeMetadata<T>>::get(tree_id).ok_or("Tree doesn't exist")?;
            ensure!(<balances::Module<T>>::free_balance(_sender.clone()) >= tree.fee, "Insufficient balance from sender");    
            ensure!(tree.leaf_count < (1 << 31), "Insufficient capacity in tree");
            Self::add_leaf_element(tree_id, leaf_value);
            Ok(())
        }
	}
}

impl<T: Trait> Module<T> {
    fn add_leaf_element(key: u32, leaf: T::Hash) {
        let mut tree = <MerkleTreeMetadata<T>>::get(key).ok_or("Tree doesn't exist").unwrap();
        // Add element
        let leaf_index = tree.leaf_count;
        tree.leaf_count += 1;
        if let Some(mut mt_level) = <MerkleTreeLevels<T>>::get((key, tree.depth - 1)) {
            mt_level.push(leaf);
            <MerkleTreeLevels<T>>::insert((key, tree.depth - 1), mt_level);
        }

        let mut curr_index = leaf_index as usize;
        // Update the tree
        for i in 0..(tree.depth - 1) {
            let mut left: T::Hash;
            let mut right: T::Hash;
            let next_index = curr_index / 2;
            let level = <MerkleTreeLevels<T>>::get((key, tree.depth - i - 1));
            if curr_index % 2 == 0 {
                left = &level.clone().unwrap()[curr_index].clone();
                // Get leaf if exists or use precomputed hash
                right = {
                    let mut temp = vec![];
                    if level.len() >= curr_index + 2 {
                        temp = level.clone().unwrap()[curr_index + 1]
                    }
                    // returns precompute for an index or the node
                    Self::get_unique_node(temp, i as usize)
                };
            } else {
                left = Self::get_unique_node(&level.clone().unwrap()[curr_index - 1], i as usize);
                right = &level.clone().unwrap()[curr_index];
            }

            if let Some(mut next_level) = <MerkleTreeLevels<T>>::get((key, tree.depth - i - 2)) {
                println!("Next level {:?}", tree.depth - i - 2);
                let new_node = Self::hash_from_halves(left, right, Some(i as usize));
                println!("{:?}", new_node);
                if next_level.len() >= next_index + 1 {
                    next_level[next_index] = new_node;
                } else {
                    next_level.push(new_node);
                }
                println!("{:?}", next_level);

                <MerkleTreeLevels<T>>::insert((key, tree.depth - i - 2), next_level);
            }

            curr_index = next_index;
        }

        <MerkleTreeMetadata<T>>::insert(key, tree);
    }

    pub fn hash_from_halves(left: T::Hash, right: T::Hash, index: Option<usize>) -> T::Hash {
        // let params = &JubjubBn256::new();
        // let mut lhs: Vec<bool> = BitIterator::new(left.into_repr()).collect();
        // let mut rhs: Vec<bool> = BitIterator::new(right.into_repr()).collect();
        // lhs.reverse();
        // rhs.reverse();

        // // Split on whether it is leaf node hash or intermediate node hash
        // let personalization = if index.is_none() {
        //     sapling_crypto::baby_pedersen_hash::Personalization::NoteCommitment
        // } else {
        //     sapling_crypto::baby_pedersen_hash::Personalization::MerkleTree(index.unwrap())
        // };

        // let hash = sapling_crypto::baby_pedersen_hash::pedersen_hash::<Bn256, _>(
        //     personalization,
        //     lhs.into_iter()
        //        .take(Fr::NUM_BITS as usize)
        //        .chain(rhs.into_iter().take(Fr::NUM_BITS as usize)),
        //     params
        // ).into_xy().0;
        let mut buf = Vec::new();
        buf.extend_from_slice(&left.encode());
        buf.extend_from_slice(&right.as_ref());
        let hash = T::Hashing::hash(&buf[..]);
        return hash;
    }

    pub fn get_unique_node(leaf: Vec<u8>, index: usize) -> Vec<u8> {
        if leaf != vec![] {
            return leaf;
        } else {
            return BLAKE_PRECOMPILES[index].encode();
        }

    }
}

decl_event!(
	pub enum Event<T> where <T as system::Trait>::Hash {
		/// new vote (id, creator, type of vote)
		NewLeaf(Hash, Hash),
	}
);

decl_storage! {
	trait Store for Module<T: Trait> as MerkleTree {
		pub NumberOfTrees get(number_of_trees): u32;
		pub MerkleTreeMetadata get(merkle_tree_metadata): map u32 => Option<MTree<T::Balance>>;
        pub MerkleTreeLevels get(merkle_tree_level): map (u32, u32) => Option<Vec<T::Hash>>;
        pub UsedNullifiers get(used_nullifiers): map Vec<u8> => bool;
	}
}


const BLAKE_PRECOMPILES: [[u8; 32]; 31] = [];
// const PEDERSEN_PRECOMPILES: [[u8; 64]; 31] = [
//     [48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48],
//     [48, 49, 56, 52, 57, 50, 98, 56, 101, 99, 56, 102, 101, 97, 101, 102, 99, 53, 54, 101, 57, 49, 55, 49, 54, 51, 102, 100, 50, 56, 56, 54, 55, 97, 102, 49, 57, 55, 52, 50, 100, 50, 100, 97, 49, 98, 98, 56, 52, 102, 52, 98, 54, 102, 48, 49, 54, 97, 101, 55, 101, 52, 100, 97],
//     [50, 53, 51, 102, 52, 98, 55, 101, 48, 48, 55, 50, 49, 53, 57, 55, 51, 55, 50, 102, 50, 102, 49, 56, 98, 98, 98, 53, 49, 56, 51, 97, 55, 50, 49, 55, 98, 56, 102, 49, 57, 52, 101, 49, 51, 102, 53, 57, 48, 97, 99, 52, 52, 100, 49, 52, 53, 98, 52, 52, 56, 55, 55, 98],
//     [48, 97, 55, 53, 98, 56, 56, 53, 53, 97, 51, 101, 99, 99, 54, 51, 53, 102, 52, 100, 54, 56, 51, 54, 52, 48, 49, 48, 53, 101, 53, 50, 97, 49, 53, 55, 50, 98, 54, 57, 54, 102, 52, 101, 99, 99, 54, 54, 55, 57, 100, 50, 49, 56, 57, 52, 57, 55, 101, 55, 53, 97, 99, 53],
//     [48, 53, 53, 50, 53, 99, 51, 98, 52, 51, 102, 48, 102, 57, 100, 102, 102, 102, 52, 50, 102, 54, 54, 52, 48, 53, 55, 98, 98, 101, 52, 53, 99, 101, 50, 101, 55, 48, 54, 56, 100, 102, 54, 50, 48, 57, 54, 48, 98, 101, 54, 53, 49, 99, 101, 51, 100, 100, 56, 56, 49, 102, 56, 53],
//     [50, 101, 99, 100, 100, 51, 100, 53, 56, 52, 101, 49, 48, 48, 99, 54, 55, 100, 101, 50, 49, 101, 55, 54, 48, 48, 52, 101, 101, 97, 55, 100, 99, 100, 102, 101, 53, 55, 102, 53, 56, 98, 50, 53, 97, 51, 54, 99, 54, 51, 56, 56, 97, 52, 54, 100, 102, 52, 55, 56, 101, 102, 97, 97],
//     [49, 53, 54, 48, 56, 49, 56, 57, 56, 55, 50, 54, 50, 99, 49, 98, 50, 54, 48, 49, 98, 48, 97, 101, 50, 101, 57, 57, 55, 102, 51, 100, 49, 53, 51, 97, 102, 99, 57, 100, 50, 52, 51, 48, 55, 53, 97, 56, 100, 54, 100, 48, 100, 48, 54, 50, 53, 97, 57, 51, 51, 56, 50, 53],
//     [48, 99, 54, 56, 53, 97, 48, 98, 99, 101, 57, 99, 48, 54, 99, 98, 56, 99, 49, 55, 55, 97, 56, 102, 57, 51, 97, 50, 52, 99, 102, 52, 48, 101, 51, 50, 53, 49, 49, 49, 102, 56, 100, 56, 97, 52, 57, 49, 97, 54, 48, 57, 99, 99, 98, 55, 100, 53, 51, 99, 52, 55, 57, 48],
//     [50, 99, 48, 49, 57, 97, 57, 97, 56, 50, 54, 54, 52, 57, 52, 48, 57, 101, 98, 49, 48, 101, 54, 99, 101, 102, 54, 57, 55, 49, 97, 50, 98, 99, 98, 57, 99, 57, 48, 98, 97, 102, 48, 98, 98, 98, 56, 53, 99, 101, 101, 102, 100, 54, 101, 57, 57, 56, 53, 102, 56, 101, 97, 53],
//     [49, 48, 97, 102, 98, 97, 49, 48, 54, 54, 100, 54, 102, 51, 52, 100, 101, 49, 48, 52, 102, 54, 49, 98, 51, 48, 56, 56, 55, 52, 51, 97, 97, 51, 102, 56, 54, 102, 57, 52, 101, 57, 50, 102, 101, 49, 49, 55, 57, 98, 102, 50, 97, 56, 100, 51, 98, 98, 100, 98, 101, 55, 55, 55],
//     [48, 52, 101, 99, 56, 55, 53, 50, 100, 101, 49, 97, 50, 55, 52, 98, 57, 55, 102, 49, 51, 102, 56, 102, 54, 50, 52, 100, 101, 54, 50, 49, 49, 55, 98, 50, 98, 101, 53, 50, 99, 54, 52, 57, 101, 99, 52, 51, 100, 49, 53, 97, 51, 55, 51, 49, 102, 54, 55, 53, 53, 98, 50, 57],
//     [49, 51, 53, 54, 54, 99, 50, 101, 51, 53, 101, 48, 55, 99, 55, 53, 49, 99, 100, 53, 48, 57, 98, 100, 52, 57, 101, 57, 99, 100, 52, 53, 48, 52, 48, 97, 54, 57, 52, 57, 102, 54, 51, 100, 53, 52, 98, 48, 97, 55, 50, 54, 53, 53, 100, 97, 55, 57, 57, 52, 48, 57, 53, 102],
//     [49, 49, 56, 102, 55, 101, 52, 55, 53, 50, 53, 50, 100, 50, 99, 53, 57, 55, 97, 54, 51, 50, 49, 102, 101, 54, 101, 57, 56, 50, 54, 54, 48, 50, 97, 49, 52, 101, 54, 51, 52, 54, 55, 48, 101, 55, 51, 50, 52, 52, 50, 50, 56, 51, 49, 49, 97, 100, 55, 56, 100, 49, 53, 53],
//     [50, 53, 54, 98, 50, 100, 49, 48, 56, 55, 53, 53, 99, 51, 56, 53, 102, 100, 98, 101, 52, 97, 102, 99, 50, 102, 57, 49, 99, 99, 51, 56, 48, 52, 99, 53, 55, 97, 49, 98, 101, 100, 48, 53, 54, 52, 49, 53, 56, 100, 51, 97, 50, 56, 100, 97, 101, 102, 57, 100, 49, 51, 54, 50],
//     [49, 55, 53, 102, 101, 56, 57, 51, 50, 52, 54, 56, 53, 48, 99, 54, 52, 55, 55, 51, 101, 48, 51, 52, 56, 49, 54, 51, 98, 49, 48, 99, 99, 51, 53, 48, 54, 55, 49, 49, 101, 53, 49, 56, 99, 97, 57, 55, 101, 48, 48, 55, 50, 56, 57, 56, 100, 52, 97, 53, 101, 101, 55, 53],
//     [48, 56, 48, 56, 100, 56, 53, 56, 101, 53, 98, 52, 98, 99, 101, 98, 54, 55, 57, 97, 54, 51, 49, 101, 48, 100, 56, 54, 97, 55, 49, 100, 49, 51, 53, 98, 99, 48, 56, 100, 52, 49, 57, 51, 101, 55, 97, 50, 102, 57, 50, 100, 52, 48, 57, 49, 52, 51, 56, 53, 51, 53, 53, 98],
//     [49, 52, 53, 97, 57, 53, 50, 55, 48, 49, 54, 52, 57, 56, 49, 99, 48, 54, 55, 52, 101, 97, 101, 98, 53, 98, 49, 99, 48, 50, 52, 54, 56, 56, 51, 97, 51, 53, 100, 50, 100, 99, 48, 97, 51, 57, 99, 100, 100, 53, 52, 98, 55, 99, 56, 50, 100, 102, 99, 55, 51, 100, 51, 50],
//     [49, 49, 56, 53, 99, 49, 50, 99, 51, 54, 54, 101, 56, 102, 50, 98, 100, 55, 48, 53, 101, 50, 50, 56, 100, 98, 99, 50, 53, 51, 98, 48, 50, 102, 50, 50, 51, 57, 98, 101, 50, 98, 56, 97, 55, 99, 57, 100, 49, 102, 55, 102, 51, 56, 52, 99, 56, 101, 101, 99, 52, 97, 50, 53],
//     [50, 49, 102, 49, 99, 97, 56, 57, 48, 54, 56, 54, 48, 102, 100, 102, 98, 49, 99, 54, 56, 101, 102, 102, 102, 49, 98, 54, 55, 102, 51, 100, 48, 99, 56, 50, 101, 102, 48, 55, 102, 98, 100, 102, 53, 55, 98, 53, 55, 97, 98, 54, 50, 57, 48, 100, 53, 56, 50, 56, 50, 97, 48, 49],
//     [48, 49, 49, 50, 102, 98, 100, 49, 51, 101, 100, 53, 53, 97, 50, 51, 97, 55, 53, 97, 55, 102, 52, 98, 56, 55, 52, 54, 55, 57, 52, 55, 55, 52, 52, 101, 56, 54, 57, 53, 97, 53, 98, 52, 51, 52, 48, 50, 99, 98, 53, 55, 102, 54, 48, 52, 102, 50, 100, 101, 48, 98, 51, 52],
//     [49, 102, 50, 100, 50, 54, 50, 53, 51, 57, 50, 98, 55, 99, 48, 55, 55, 100, 52, 50, 56, 100, 100, 51, 101, 57, 54, 49, 97, 54, 99, 100, 100, 48, 52, 52, 98, 48, 50, 51, 98, 55, 50, 52, 100, 102, 52, 97, 97, 98, 98, 50, 49, 99, 57, 51, 54, 51, 102, 99, 52, 56, 49, 50],
//     [50, 98, 54, 49, 52, 97, 55, 53, 51, 99, 49, 52, 99, 51, 52, 56, 57, 48, 101, 102, 55, 50, 51, 50, 102, 48, 56, 56, 53, 98, 52, 49, 100, 48, 55, 51, 48, 98, 53, 53, 99, 51, 100, 101, 101, 52, 102, 56, 57, 48, 102, 98, 98, 49, 54, 57, 49, 99, 102, 48, 51, 54, 100, 102],
//     [50, 57, 51, 101, 48, 50, 56, 52, 98, 57, 100, 52, 51, 49, 57, 48, 51, 53, 50, 48, 52, 97, 99, 55, 102, 54, 52, 48, 53, 98, 55, 97, 101, 97, 49, 57, 50, 98, 57, 54, 53, 55, 48, 52, 100, 49, 101, 50, 53, 48, 56, 57, 51, 97, 56, 52, 48, 57, 57, 54, 56, 99, 57, 57],
//     [50, 57, 55, 98, 48, 102, 102, 57, 100, 101, 52, 100, 53, 98, 55, 48, 98, 54, 48, 100, 101, 99, 102, 99, 54, 53, 54, 100, 57, 55, 53, 54, 101, 101, 53, 53, 49, 55, 102, 101, 97, 100, 53, 56, 101, 57, 51, 102, 57, 53, 50, 52, 54, 51, 55, 101, 98, 100, 98, 52, 54, 49, 98, 102],
//     [50, 52, 55, 52, 52, 98, 56, 99, 52, 48, 53, 101, 99, 52, 57, 56, 97, 100, 97, 50, 54, 49, 55, 48, 97, 56, 53, 55, 102, 101, 57, 102, 102, 48, 48, 51, 56, 48, 52, 56, 102, 97, 54, 53, 53, 53, 102, 99, 48, 56, 54, 54, 54, 98, 56, 50, 49, 52, 56, 55, 101, 48, 55, 49],
//     [50, 49, 56, 101, 51, 97, 54, 97, 99, 52, 53, 55, 52, 98, 102, 102, 98, 54, 101, 53, 97, 52, 100, 98, 55, 52, 55, 56, 102, 53, 52, 56, 55, 98, 54, 53, 51, 51, 50, 97, 50, 98, 55, 51, 102, 56, 97, 100, 54, 50, 97, 53, 99, 101, 99, 51, 54, 49, 57, 53, 48, 52, 97, 48],
//     [50, 99, 57, 99, 100, 48, 102, 54, 48, 52, 57, 100, 100, 53, 57, 101, 102, 49, 98, 102, 57, 52, 52, 97, 54, 50, 98, 101, 100, 99, 49, 97, 100, 54, 97, 48, 55, 49, 101, 55, 52, 100, 99, 100, 99, 53, 52, 54, 48, 55, 101, 48, 99, 51, 99, 48, 98, 99, 101, 51, 50, 54, 101, 49],
//     [50, 99, 101, 53, 98, 51, 51, 100, 99, 51, 100, 99, 99, 102, 53, 53, 50, 52, 98, 56, 56, 101, 48, 57, 52, 102, 55, 101, 50, 100, 57, 48, 100, 57, 98, 101, 50, 99, 102, 51, 99, 100, 52, 99, 101, 50, 102, 54, 51, 52, 48, 56, 49, 56, 99, 48, 57, 101, 48, 102, 55, 97, 55, 56],
//     [50, 98, 54, 49, 57, 99, 99, 57, 50, 56, 54, 54, 49, 57, 54, 53, 57, 102, 101, 54, 50, 97, 48, 102, 98, 54, 53, 102, 51, 55, 97, 50, 54, 98, 56, 99, 52, 49, 55, 97, 56, 51, 52, 51, 48, 56, 50, 100, 49, 99, 56, 100, 51, 98, 101, 53, 98, 98, 53, 53, 55, 102, 57, 57],
//     [50, 101, 51, 101, 54, 98, 97, 50, 98, 53, 53, 55, 51, 52, 56, 56, 100, 101, 52, 50, 56, 56, 101, 57, 100, 98, 99, 50, 48, 54, 54, 50, 51, 54, 99, 51, 51, 98, 102, 102, 102, 98, 57, 98, 56, 51, 48, 55, 100, 54, 98, 99, 51, 54, 57, 55, 52, 53, 97, 50, 101, 48, 56, 51],
//     [50, 100, 102, 48, 49, 53, 56, 51, 100, 54, 102, 51, 51, 53, 100, 48, 100, 55, 52, 101, 57, 50, 49, 49, 55, 48, 100, 57, 100, 99, 51, 55, 50, 50, 57, 56, 98, 50, 53, 48, 97, 55, 48, 53, 50, 98, 51, 102, 98, 50, 53, 98, 57, 97, 56, 56, 53, 50, 49, 52, 99, 51, 98, 53],
// ];