// Copyright 2018-2019 Commonwealth Labs, Inc.
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
// along with Edgeware.  If not, see <http://www.gnu.org/licenses/>

//! A set of constant values used in substrate runtime.

/// Money matters.
pub mod currency {
    use edgeware_primitives::Balance;

    pub const MILLICENTS: Balance = 10_000_000_000_000;
    pub const CENTS: Balance = 1_000 * MILLICENTS;    // assume this is worth about a cent.
    pub const DOLLARS: Balance = 100 * CENTS;
}

/// Time.
pub mod time {
    use edgeware_primitives::Moment;
    pub const MILLISECS_PER_BLOCK: Moment = 6000;
    pub const SECS_PER_BLOCK: Moment = MILLISECS_PER_BLOCK / 1000;
    pub const MINUTES: Moment = 60 / SECS_PER_BLOCK;
    pub const HOURS: Moment = MINUTES * 60;
    pub const DAYS: Moment = HOURS * 24;
    pub const SLOT_DURATION: Moment = MILLISECS_PER_BLOCK;
}

// CRITICAL NOTE: The system module maintains two constants: a _maximum_ block weight and a
// _ratio_ of it yielding the portion which is accessible to normal transactions (reserving the rest
// for operational ones). 0`TARGET_BLOCK_FULLNESS` is entirely independent and the system module is
// not aware of if, nor should it care about it. This constant simply denotes on which ratio of the
// _maximum_ block weight we tweak the fees. It does NOT care about the type of the dispatch.
//
// For the system to be configured in a sane way, `TARGET_BLOCK_FULLNESS` should always be less than
// the ratio that `system` module uses to find normal transaction quota.
/// Fee-related.
pub mod fee {
    pub use runtime_primitives::Perbill;

    /// The block saturation level. Fees will be updates based on this value.
    pub const TARGET_BLOCK_FULLNESS: Perbill = Perbill::from_percent(25);
}