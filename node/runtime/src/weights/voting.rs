//! Weights for voting
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 2.0.0
//! DATE: 2020-10-21, STEPS: [20], REPEAT: 3, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: None, WASM-EXECUTION: Interpreted, CHAIN: None, DB CACHE: 128
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Trait> voting::WeightInfo for WeightInfo<T> {
	fn commit(p: u32, s: u32, ) -> Weight {
		(9_056_000 as Weight)
			.saturating_add((241_000 as Weight).saturating_mul(p as Weight))
			.saturating_add((523_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn reveal(p: u32, s: u32, ) -> Weight {
		(30_054_000 as Weight)
			.saturating_add((111_000 as Weight).saturating_mul(p as Weight))
			.saturating_add((3_036_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}
