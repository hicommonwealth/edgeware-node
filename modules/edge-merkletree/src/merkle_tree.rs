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
extern crate bellman;
extern crate ff;
extern crate num_bigint;
extern crate num_traits;
extern crate sapling_crypto;

use sapling_crypto::{
    babyjubjub::{
        JubjubBn256,
    },
};
use num_traits::Num;
use ff::{BitIterator, PrimeField};
use pairing::{bn256::{Bn256, Fr}};
use rstd::prelude::*;
use system::ensure_signed;
use runtime_support::{StorageValue, StorageMap};
use runtime_support::dispatch::Result;
use runtime_primitives::traits::Hash;
use runtime_primitives::traits::{Zero};
use codec::Encode;

use bellman::groth16::{Proof, Parameters, verify_proof, prepare_verifying_key};
use num_bigint::BigInt;

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, PartialEq)]
pub struct MTree<Balance> {
	pub root: Vec<u8>,
    pub leaves: Option<Vec<Vec<u8>>>,
    pub fee: Balance,
    pub depth: u32,
    pub upper_pow: u32,
}

const DEFAULT_TREE_DEPTH: u32 = 32;

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
                root: T::Hashing::hash_of(b"0").encode(),
                leaves: None,
                fee: Zero::zero(),
                depth: DEFAULT_TREE_DEPTH,
                upper_pow: 1,
            });
            Ok(())
        }

        pub fn add_leaf(origin, tree_id: u32, leaf_value: T::Hash) -> Result {
            let _sender = ensure_signed(origin)?;
            let mut tree = <MerkleTrees<T>>::get(tree_id).ok_or("Tree doesn't exist")?;
            ensure!(<balances::Module<T>>::free_balance(_sender.clone()) > tree.fee, "Insufficient balance from sender");    
            ensure!(tree.upper_pow <= tree.depth, "Tree has insufficient capacity");

            if let Some(mut leaves) = tree.leaves {
                if leaves.len() == 2_i32.pow(tree.upper_pow) as usize {
                    tree.upper_pow += 1;
                }
                leaves.push(leaf_value.encode());
                let mut aux_leaves = leaves.clone();
                for _ in aux_leaves.len()..(2_i32.pow(tree.upper_pow) as usize) {
                    aux_leaves.push("0".as_bytes().to_vec());
                }

                // TODO: Optimize recomputing hash without recomputing entire tree?
                let new_root = Self::compute_new_root(aux_leaves.clone());
                <MerkleTrees<T>>::insert(tree_id, MTree {
                    root: new_root.encode(),
                    leaves: Some(leaves),
                    ..tree
                });
            }

            
            Ok(())
        }

        pub fn verify_zkproof(origin, tree_id: u32, _params: Vec<u8>, _proof: Vec<u8>, _nullifier_hex: Vec<u8>, _root_hex: Vec<u8>) -> Result {
            let _sender = ensure_signed(origin)?;
            let params = String::from_utf8(_params).expect("Found invalid UTF-8");
            let proof = String::from_utf8(_proof).expect("Found invalid UTF-8");
            let nullifier_hex = String::from_utf8(_nullifier_hex.clone()).expect("Found invalid UTF-8");
            // let root_hex = String::from_utf8(_root_hex).expect("Found invalid UTF-8");
            let tree = <MerkleTrees<T>>::get(tree_id).ok_or("Tree doesn't exist")?;
            let tree_root = tree.root.encode();
            let root_hex = String::from_utf8(tree_root).expect("Invalid root");

            let params_hex = hex::decode(params).expect("Decoding params failed");
            let de_params = Parameters::read(&params_hex[..], true).expect("Param bellman decode failed");


            let pvk = prepare_verifying_key::<Bn256>(&de_params.vk);
            // Nullifier
            let nullifier_big = BigInt::from_str_radix(&nullifier_hex, 16).expect("Nullfier decode failed");
            let nullifier_raw = &nullifier_big.to_str_radix(10);
            let nullifier = Fr::from_str(nullifier_raw).ok_or("couldn't parse Fr")?;
            // Root hash

            let root_big = BigInt::from_str_radix(&root_hex, 16).expect("Root decode failed");
            let root_raw = &root_big.to_str_radix(10);
            let root = Fr::from_str(root_raw).ok_or("couldn't parse Fr")?;
            let _result = verify_proof(
                &pvk,
                &Proof::read(&hex::decode(proof).expect("Proof hex decode failed")[..]).expect("Proof decode failed"),
                &[
                    nullifier,
                    root
                ]).expect("Verify proof failed");

            if _result {
                <UsedNullifiers<T>>::insert(_nullifier_hex, true);    
            }
            
            Ok(())
        }
	}
}

impl<T: Trait> Module<T> {
    pub fn hash_from_halves(left_bytes: Vec<u8>, right_bytes: Vec<u8>) -> Vec<u8> {
        let params = &JubjubBn256::new();
        let left_pt_str = String::from_utf8(left_bytes.encode()).expect("Found invalid UTF-8");
        let left_big = BigInt::from_str_radix(&left_pt_str, 16).expect("Nullfier decode failed");
        let left_raw = &left_big.to_str_radix(10);
        let left = Fr::from_str(left_raw).ok_or("couldn't parse Fr").unwrap();

        let right_pt_str = String::from_utf8(right_bytes.encode()).expect("Found invalid UTF-8");
        let right_big = BigInt::from_str_radix(&right_pt_str, 16).expect("Nullfier decode failed");
        let right_raw = &right_big.to_str_radix(10);
        let right = Fr::from_str(right_raw).ok_or("couldn't parse Fr").unwrap();

        let mut lhs: Vec<bool> = BitIterator::new(left.into_repr()).collect();
        let mut rhs: Vec<bool> = BitIterator::new(right.into_repr()).collect();
        lhs.reverse();
        rhs.reverse();
        let hash = sapling_crypto::baby_pedersen_hash::pedersen_hash::<Bn256, _>(
            sapling_crypto::baby_pedersen_hash::Personalization::NoteCommitment,
            lhs.into_iter()
               .take(Fr::NUM_BITS as usize)
               .chain(rhs.into_iter().take(Fr::NUM_BITS as usize)),
            params
        ).into_xy().0;
        
        return hash.to_string().as_bytes().to_vec();
    }

    fn compute_new_root(mut nodes: Vec<Vec<u8>>) -> Vec<u8> {
        if nodes.len() == 2 {
            let l = nodes.remove(0);
            let r = nodes.remove(0);
            return Self::hash_from_halves(l, r);
        } else {
            let left_nodes = nodes[..(nodes.len() / 2)].to_vec();
            let right_nodes = nodes[(nodes.len() / 2)..].to_vec();
            return Self::hash_from_halves(
                Self::compute_new_root(left_nodes),
                Self::compute_new_root(right_nodes),
            );
        }
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
		pub MerkleTrees get(merkle_trees): map u32 => Option<MTree<T::Balance>>;
        pub UsedNullifiers get(used_nullifiers): map Vec<u8> => bool;
	}
}
