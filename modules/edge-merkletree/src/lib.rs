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
extern crate serde_derive;
#[cfg(test)]
#[macro_use]
extern crate hex_literal;
#[macro_use] extern crate parity_codec_derive;
#[macro_use] extern crate srml_support;


extern crate parity_codec as codec;
extern crate substrate_primitives as primitives;
#[cfg_attr(not(feature = "std"), macro_use)]
extern crate sr_std as rstd;
extern crate srml_support as runtime_support;
extern crate sr_primitives as runtime_primitives;
extern crate sr_io as runtime_io;

extern crate srml_balances as balances;
extern crate srml_system as system;
extern crate edge_delegation as delegation;

extern crate rand;
extern crate num_bigint;
extern crate num_traits;
extern crate bellman;
extern crate sapling_crypto;

pub mod merkle_tree;
pub use merkle_tree::{Module, Trait, RawEvent, Event};

#[cfg(not(target_arch = "wasm32"))]
use std::time::{Instant};

pub struct Stopwatch {
    start: Instant,
}

impl Stopwatch {
    pub fn start() -> Self {
        Stopwatch {
            start: Instant::now(),
        }
    }

    pub fn finish(self) -> u128 {
        return self.start.elapsed().as_millis();
    }
}

// Tests for Delegation Module
#[cfg(test)]
mod tests {
	use super::*;
	use rstd::prelude::*;
	use runtime_support::dispatch::Result;
	use runtime_io::ed25519::Pair;
	
	use runtime_io::with_externalities;
	use primitives::{H256, Blake2Hasher};
	use bellman::pairing::bn256::Fr;
	use rand::Rand;
	
	
	// The testing primitives are very useful for avoiding having to work with signatures
	// or public keys. `u64` is used as the `AccountId` and no `Signature`s are requried.
	use runtime_primitives::{
		BuildStorage, traits::{BlakeTwo256, IdentityLookup},
		testing::{Digest, DigestItem, Header}
	};

	// static SECRET: [u8; 32] = [1,0,1,0,1,0,1,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,4];

	impl_outer_origin! {
		pub enum Origin for Test {}
	}

	impl_outer_event! {
		pub enum Event for Test {
			merkle_tree<T>, delegation<T>, balances<T>,
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
		type Lookup = IdentityLookup<H256>;
		type Header = Header;
		type Event = Event;
		type Log = DigestItem;
	}

	impl balances::Trait for Test {
		type Balance = u64;
		type OnFreeBalanceZero = ();
		type OnNewAccount = ();
		type Event = Event;
	}

	impl delegation::Trait for Test {
		type Event = Event;
	}

	impl Trait for Test {
		type Event = Event;
	}

	pub type System = system::Module<Test>;
	pub type Delegation = delegation::Module<Test>;
	pub type MerkleTree = Module<Test>;

	// This function basically just builds a genesis storage key/value store according to
	// our desired mockup.
	fn new_test_ext() -> sr_io::TestExternalities<Blake2Hasher> {
		let mut t = system::GenesisConfig::<Test>::default().build_storage().unwrap().0;
		t.extend(
			delegation::delegation::GenesisConfig::<Test> {
				delegation_depth: 5,
				_genesis_phantom_data: Default::default(),
			}.build_storage().unwrap().0,
		);
		// We use default for brevity, but you can configure as desired if needed.
		t.into()
	}

	fn create_tree(who: H256, fee: Option<u64>, depth: Option<u32>, leaves: Option<Vec<Vec<u8>>>) -> Result {
		MerkleTree::create_tree(Origin::signed(who), fee, depth, leaves)
	}

	fn add_leaf(who: H256, tree_id: u32, leaf_hash: Vec<u8>) -> Result {
		MerkleTree::add_leaf(Origin::signed(who), tree_id, leaf_hash)
	}

	#[test]
	fn create_tree_should_work() {
		with_externalities(&mut new_test_ext(), || {
			System::set_block_number(1);
			let pair: Pair = Pair::from_seed(&hex!("9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"));
			let public: H256 = pair.public().0.into();
			assert_ok!(create_tree(public, None, None, None));
			let tree = MerkleTree::merkle_tree_metadata(0).unwrap();
			assert_eq!(tree.fee, 0);
			assert_eq!(tree.depth, 31);
		});
	}

	#[test]
	fn time_functions() {
		with_externalities(&mut new_test_ext(), || {
			let pair: Pair = Pair::from_seed(&hex!("9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"));
			let public: H256 = pair.public().0.into();

			let mut stopwatch = Stopwatch::start();
			
			assert_ok!(create_tree(public, None, None, None));
			let mut millis = stopwatch.finish();
			println!("create tree time elapsed: {}", millis);

			
			for i in 0..32 {
				stopwatch = Stopwatch::start();
				let precompute_i = MerkleTree::get_precomputes(i);
				println!("{:?}, {:?}", precompute_i, MerkleTree::convert_point_to_bytes(precompute_i));
				millis = stopwatch.finish();
				println!("generating precompute of depth {} time elapsed: {}", i, millis);
			}
		});
	}

	#[test]
	fn add_leaf_to_tree_should_work() {
		with_externalities(&mut new_test_ext(), || {
			System::set_block_number(1);
			let pair: Pair = Pair::from_seed(&hex!("9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"));
			let public: H256 = pair.public().0.into();
			assert_ok!(create_tree(public, None, None, None));

			let mut rng = rand::thread_rng();
			let left_pt = Fr::rand(&mut rng);
			let right_pt = Fr::rand(&mut rng);
			let leaf_hash = MerkleTree::hash_from_halves(left_pt, right_pt, None);
			let leaf_bytes = MerkleTree::convert_point_to_bytes(leaf_hash);
			assert_ok!(add_leaf(public, 0, leaf_bytes));
		});
	}
}