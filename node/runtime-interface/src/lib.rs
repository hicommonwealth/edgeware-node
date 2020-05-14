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

use sp_runtime_interface::runtime_interface;

#[runtime_interface]
pub trait Storage {
    fn child_storage_kill(a: u64, b: u64, c: u32) {
        return;
    }

    fn child_root(a: u64) -> Option<u64> {
        return Some(a);
    }

    fn child_get(a: u64, b: u64, c: u32, d: u64) -> Option<u64> {
        return Some(a);
    }

    fn child_storage_key_or_panic(a: u64) -> u64 {
        return a;
    }

    fn child_read(a: u64, b: u64, c: u32, d: u64, e: u64, f: u32) -> Option<u32> {
        return Some(c);
    }

    fn child_clear(a: u64, b: u64, c: u32, d: u64) {
        return;
    }
}
