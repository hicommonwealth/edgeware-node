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
	fn create_proposal(p: u32, b: u32, _o: u32, ) -> Weight {
		(141_283_000 as Weight)
			.saturating_add((552_000 as Weight).saturating_mul(p as Weight))
			.saturating_add((2_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	fn advance_proposal(p: u32, _o: u32, ) -> Weight {
		(86_050_000 as Weight)
			.saturating_add((314_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
}
