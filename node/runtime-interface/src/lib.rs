// Copyright 2018-2020 Commonwealth Labs, Inc.
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
#![recursion_limit = "128"]

use sp_runtime_interface::runtime_interface;

#[runtime_interface]
pub trait Storage {
	fn child_storage_kill(_a: u64, _b: u64, _c: u32) {
		return;
	}

	fn child_root(_a: u64) -> Option<u64> {
		return Some(_a);
	}

	fn child_get(_a: u64, _b: u64, _c: u32, _d: u64) -> Option<u64> {
		return Some(_a);
	}

	fn child_storage_key_or_panic(_a: u64) -> u64 {
		return _a;
	}

	fn child_read(_a: u64, _b: u64, _c: u32, _d: u64, _e: u64, _f: u32) -> Option<u32> {
		return Some(_c);
	}

	fn child_clear(_a: u64, _b: u64, _c: u32, _d: u64) {
		return;
	}

	fn child_set(_a: u64, _b: u64, _c: u32, _d: u64, _e: u64) {
		return;
	}

	/*
		Not sure if these will be needed in the future but
		commenting out since combing the git history is annoying
	*/

	// fn child_exists(_a: u64, _b: u64, _c: u32, _d: u64) -> bool {
	//     return false;
	// }

	// fn child_clear_prefix(_a: u64, _b: u64, _c: u32, _d: u64) {
	//     return;
	// }

	// fn child_next_key(_a: u64, _b: u64, _c: u32, _d: u64) -> Option<u64> {
	//     return;
	// }
}
