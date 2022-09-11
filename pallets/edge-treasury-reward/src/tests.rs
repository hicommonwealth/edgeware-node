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

use super::*;
use crate::mock::*;
use mock::last_event;

use frame_support::traits::OnFinalize;

#[test]
fn basic_setup_works() {
	// Verifies initial conditions of mock
	new_test_ext().execute_with(|| {
		// Initial Era and session
		let treasury_address = Treasury::account_id();
		System::set_block_number(1);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(1);
		System::set_block_number(2);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(2);
		System::set_block_number(100);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(101);
		System::set_block_number(101);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(102);
		System::set_block_number(102);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(103);
		System::set_block_number(103);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(104);
		assert_eq!(Balances::free_balance(treasury_address.clone()) > 0, true);
	});
}

#[test]
fn setting_treasury_block_reward() {
	new_test_ext().execute_with(|| {
		// Initial Era and session
		// Minting interval is 1
		let treasury_address = Treasury::account_id();

		// 95 EDG to block 1
		System::set_block_number(1);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(1);
		assert_eq!(Balances::free_balance(treasury_address.clone()) == 9500000, true);
		assert_eq!(
			last_event(),
			mock::Event::TreasuryReward(crate::Event::TreasuryMinting(9500000, 1, treasury_address.clone()))
		);

		// 95 EDG to block 2
		System::set_block_number(2);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(2);
		assert_eq!(Balances::free_balance(treasury_address.clone()) == 19000000, true);
		assert_eq!(
			last_event(),
			mock::Event::TreasuryReward(crate::Event::TreasuryMinting(19000000, 2, treasury_address.clone()))
		);

		// Reduce minting interval to 2
		<TreasuryReward>::set_current_payout(frame_system::RawOrigin::Root.into(), 95).unwrap();
		<TreasuryReward>::set_minting_interval(frame_system::RawOrigin::Root.into(), 2).unwrap();

		// 0 EDG to block 3
		System::set_block_number(3);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(3);
		assert_eq!(Balances::free_balance(treasury_address.clone()) == 19000000, true);
		assert_eq!(
			last_event(),
			mock::Event::TreasuryReward(crate::Event::TreasuryMinting(19000000, 2, treasury_address.clone()))
		);

		// 95 EDG to block 4
		System::set_block_number(4);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(4);
		assert_eq!(Balances::free_balance(treasury_address.clone()) == 19000095, true);
		assert_eq!(
			last_event(),
			mock::Event::TreasuryReward(crate::Event::TreasuryMinting(19000095, 4, treasury_address.clone()))
		);

		// Reduce payout to 0
		<TreasuryReward>::set_current_payout(frame_system::RawOrigin::Root.into(), 0).unwrap();

		// 0 EDG to block 5
		System::set_block_number(5);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(5);
		assert_eq!(Balances::free_balance(treasury_address.clone()) == 19000095, true);
		assert_eq!(
			last_event(),
			mock::Event::TreasuryReward(crate::Event::TreasuryMinting(19000095, 4, treasury_address.clone()))
		);

		// 0 EDG to block 6
		System::set_block_number(6);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(6);
		assert_eq!(Balances::free_balance(treasury_address.clone()) == 19000095, true);
		assert_eq!(
			last_event(),
			mock::Event::TreasuryReward(crate::Event::TreasuryMinting(19000095, 4, treasury_address.clone()))
		);

		// Increase payout to 105
		<TreasuryReward>::set_current_payout(frame_system::RawOrigin::Root.into(), 105).unwrap();

		// 0 EDG to block 7
		System::set_block_number(7);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(7);
		assert_eq!(Balances::free_balance(treasury_address.clone()) == 19000095, true);
		assert_eq!(
			last_event(),
			mock::Event::TreasuryReward(crate::Event::TreasuryMinting(19000095, 4, treasury_address.clone()))
		);

		// 105 EDG to block 8
		System::set_block_number(8);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(8);
		assert_eq!(Balances::free_balance(treasury_address.clone()) == 19000200, true);
		assert_eq!(
			last_event(),
			mock::Event::TreasuryReward(crate::Event::TreasuryMinting(19000200, 8, treasury_address.clone()))
		);

		// Reduce payout to 10
		// Set minting interval to every block
		<TreasuryReward>::set_minting_interval(frame_system::RawOrigin::Root.into(), 1).unwrap();
		<TreasuryReward>::set_current_payout(frame_system::RawOrigin::Root.into(), 10).unwrap();

		// 10 EDG to block 9
		System::set_block_number(9);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(9);
		assert_eq!(Balances::free_balance(treasury_address.clone()) == 19000210, true);
		assert_eq!(
			last_event(),
			mock::Event::TreasuryReward(crate::Event::TreasuryMinting(19000210, 9, treasury_address.clone()))
		);

		// 10 EDG to block 10
		System::set_block_number(10);
		<TreasuryReward as OnFinalize<u64>>::on_finalize(10);
		assert_eq!(Balances::free_balance(treasury_address.clone()) == 19000220, true);
		assert_eq!(
			last_event(),
			mock::Event::TreasuryReward(crate::Event::TreasuryMinting(19000220, 10, treasury_address.clone()))
		);
	});
}
