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

#![allow(unused_parens)]
#![allow(unused_imports)]

use sp_std::marker::PhantomData;
use frame_support::weights::{Weight, constants::RocksDbWeight as DbWeight};


pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> crate::WeightInfo for SubstrateWeight<T> {
	fn create_proposal(p: u32, b: u32, ) -> Weight {
		(12_424_000 as Weight)
			.saturating_add((983_000 as Weight).saturating_mul(p as Weight))
			.saturating_add((3_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(DbWeight::get().reads(6 as Weight))
			.saturating_add(DbWeight::get().writes(5 as Weight))
	}
	fn advance_proposal(p: u32, ) -> Weight {
		(58_164_000 as Weight)
			.saturating_add((846_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(DbWeight::get().reads(5 as Weight))
			.saturating_add(DbWeight::get().writes(4 as Weight))
	}
}


impl crate::WeightInfo for () {
	fn create_proposal(p: u32, b: u32, ) -> Weight {
		(12_424_000 as Weight)
			.saturating_add((983_000 as Weight).saturating_mul(p as Weight))
			.saturating_add((3_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(DbWeight::get().reads(6 as Weight))
			.saturating_add(DbWeight::get().writes(5 as Weight))
	}
	fn advance_proposal(p: u32, ) -> Weight {
		(58_164_000 as Weight)
			.saturating_add((846_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(DbWeight::get().reads(5 as Weight))
			.saturating_add(DbWeight::get().writes(4 as Weight))
	}
}
