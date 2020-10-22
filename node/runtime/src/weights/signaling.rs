//! Weights for signaling
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 2.0.0
//! DATE: 2020-10-21, STEPS: [50], REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: None, WASM-EXECUTION: Interpreted, CHAIN: None, DB CACHE: 128
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Trait> signaling::WeightInfo for WeightInfo<T> {
	fn create_proposal(p: u32, b: u32, ) -> Weight {
		(12_424_000 as Weight)
			.saturating_add((983_000 as Weight).saturating_mul(p as Weight))
			.saturating_add((3_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	fn advance_proposal(p: u32, ) -> Weight {
		(58_164_000 as Weight)
			.saturating_add((846_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
}
