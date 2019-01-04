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
#[macro_use]
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

use rstd::prelude::*;
use runtime_support::dispatch::Result;

pub mod delegation;
pub use delegation::{Module, Trait, RawEvent, Event};

// Tests for Delegation Module
#[cfg(test)]
mod tests {
    use super::*;

    use system::{EventRecord, Phase};
    use runtime_io::with_externalities;
    use primitives::{H256, Blake2Hasher};
    // The testing primitives are very useful for avoiding having to work with signatures
    // or public keys. `u64` is used as the `AccountId` and no `Signature`s are requried.
    use runtime_primitives::{
        BuildStorage, traits::{BlakeTwo256}, testing::{Digest, DigestItem, Header}
    };


    impl_outer_origin! {
        pub enum Origin for Test {}
    }

    impl_outer_event! {
        pub enum Event for Test {
            delegation<T>, balances<T>,
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
        type Header = Header;
        type Event = Event;
        type Log = DigestItem;
    }

    impl balances::Trait for Test {
        type Balance = u64;
        type AccountIndex = u64;
        type OnFreeBalanceZero = ();
        type EnsureAccountLiquid = ();
        type Event = Event;
    }

    impl Trait for Test {
        type Event = Event;
    }

    pub type System = system::Module<Test>;
    pub type Delegation = Module<Test>;

    // This function basically just builds a genesis storage key/value store according to
    // our desired mockup.
    fn new_test_ext() -> sr_io::TestExternalities<Blake2Hasher> {
        let t = system::GenesisConfig::<Test>::default().build_storage().unwrap().0;
        // We use default for brevity, but you can configure as desired if needed.
        t.into()
    }

    fn delegate_to(who: H256, to_account: H256) -> super::Result {
        Delegation::delegate_to(Origin::signed(who), to_account)
    }

    fn undelegate_from(who: H256, from_account: H256) -> super::Result {
        Delegation::undelegate_from(Origin::signed(who), from_account)
    }

    #[test]
    fn unit_delegate_should_work() {
        with_externalities(&mut new_test_ext(), || {
            System::set_block_number(1);
            let a : Vec<H256> = (1..3).map(|v| H256::from(v)).collect();

            assert_ok!(delegate_to(a[0], a[1]));
            assert_eq!(System::events(), vec![
                EventRecord {
                    phase: Phase::ApplyExtrinsic(0),
                    event: Event::delegation(RawEvent::Delegated(a[0], a[1]))
                }]
            );
            assert_eq!(Delegation::tally_delegation(a.clone()),
                       vec![(a[0], a[1]), (a[1], a[1])]);
        });
    }

    #[test]
    fn multistep_delegate_should_work() {
        with_externalities(&mut new_test_ext(), || {
            System::set_block_number(1);
            let a : Vec<H256> = (1..5).map(|v| H256::from(v)).collect();
            
            assert_ok!(delegate_to(a[0], a[1]));
            assert_eq!(System::events(), vec![
                EventRecord {
                    phase: Phase::ApplyExtrinsic(0),
                    event: Event::delegation(RawEvent::Delegated(a[0], a[1]))
                }]
            );

            assert_ok!(delegate_to(a[1], a[2]));
            assert_eq!(System::events(), vec![
                EventRecord {
                    phase: Phase::ApplyExtrinsic(0),
                    event: Event::delegation(RawEvent::Delegated(a[0], a[1]))
                },
                EventRecord {
                    phase: Phase::ApplyExtrinsic(0),
                    event: Event::delegation(RawEvent::Delegated(a[1], a[2]))
                }]
            );
            assert_eq!(Delegation::tally_delegation(a.clone()),
                       vec![(a[0], a[2]), (a[1], a[2]), (a[2], a[2]), (a[3], a[3])]);
        });
    }

    #[test]
    fn self_delegate_should_fail() {
        with_externalities(&mut new_test_ext(), || {
            System::set_block_number(1);
            let a = H256::from(1);
            assert_eq!(delegate_to(a, a), Err("Invalid delegation"));
            assert_eq!(System::events(), vec![]);
            assert_eq!(Delegation::tally_delegation(vec![a]), vec![(a, a)]);
        });
    }

    #[test]
    fn cycle_delegate_should_fail() {
        with_externalities(&mut new_test_ext(), || {
            System::set_block_number(1);
            let a : Vec<H256> = (1..3).map(|v| H256::from(v)).collect();
            
            assert_ok!(delegate_to(a[0], a[1]));
            assert_eq!(System::events(), vec![
                EventRecord {
                    phase: Phase::ApplyExtrinsic(0),
                    event: Event::delegation(RawEvent::Delegated(a[0], a[1]))
                }]
            );
            assert_eq!(delegate_to(a[1], a[0]), Err("Invalid delegation"));
            
            // ensure failure did not add event
            assert_eq!(System::events(), vec![
                EventRecord {
                    phase: Phase::ApplyExtrinsic(0),
                    event: Event::delegation(RawEvent::Delegated(a[0], a[1]))
                }]
            );
            assert_eq!(Delegation::tally_delegation(a.clone()),
                       vec![(a[0], a[1]), (a[1], a[1])])
        });
    }

    #[test]
    fn unit_undelegate_should_work() {
        with_externalities(&mut new_test_ext(), || {
            System::set_block_number(1);

            assert_ok!(delegate_to(H256::from(1), H256::from(2)));
            assert_ok!(undelegate_from(H256::from(1), H256::from(2)));
            assert_eq!(System::events(), vec![
                EventRecord {
                    phase: Phase::ApplyExtrinsic(0),
                    event: Event::delegation(RawEvent::Delegated(H256::from(1), H256::from(2)))
                },
                EventRecord {
                    phase: Phase::ApplyExtrinsic(0),
                    event: Event::delegation(RawEvent::Undelegated(H256::from(1), H256::from(2)))
                }]
            );
        });
    }

    // TODO: write undelegate tests that should fail
}
