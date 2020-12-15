//! Mocks for the airdrop module.

#![cfg(test)]

use super::*;
use frame_support::{impl_outer_event, impl_outer_origin, parameter_types};
use sp_core::H256;
use sp_runtime::{testing::Header, traits::IdentityLookup, Perbill};

pub type AccountId = H256;
pub type BlockNumber = u64;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Test;

mod renvm {
	pub use super::super::*;
}

impl_outer_origin! {
	pub enum Origin for Test {}
}

impl_outer_event! {
	pub enum TestEvent for Test {
		frame_system<T>,
		pallet_balances<T>,
		pallet_assets<T>,
		renvm<T>,
	}
}

pub type RenvmBridgeCall = super::Call<Test>;

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: u32 = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::one();
}

impl frame_system::Config for Test {
	type Origin = Origin;
	type Index = u64;
	type BlockNumber = BlockNumber;
	type Call = ();
	type Hash = H256;
	type Hashing = ::sp_Test::traits::BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = TestEvent;
	type BlockHashCount = BlockHashCount;
	type MaximumBlockWeight = MaximumBlockWeight;
	type MaximumBlockLength = MaximumBlockLength;
	type AvailableBlockRatio = AvailableBlockRatio;
	type Version = ();
	type PalletInfo = ();
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type DbWeight = ();
	type BlockExecutionWeight = ();
	type ExtrinsicBaseWeight = ();
	type MaximumExtrinsicWeight = ();
	type BaseCallFilter = ();
	type SystemWeightInfo = ();
}

parameter_types! {
	pub const ExistentialDeposit: Balance = 0;
	pub const RenVmPublicKey: [u8; 20] = hex_literal::hex!["4b939fc8ade87cb50b78987b1dda927460dc456a"];
	pub const RENBTCIdentifier: [u8; 32] = hex_literal::hex!["f6b5b360905f856404bd4cf39021b82209908faa44159e68ea207ab8a5e13197"];
}

impl pallet_balances::Config for Test {
	type Balance = Balance;
	type DustRemoval = ();
	type Event = TestEvent;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = frame_system::Module<Test>;
	type MaxLocks = ();
	type WeightInfo = ();
}
pub type Balances = pallet_balances::Module<Test>;

parameter_types! {
	pub const UnsignedPriority: u64 = 1 << 20;
}

pub type AdaptedBasicCurrency = orml_currencies::BasicCurrencyAdapter<Test, Balances, Amount, BlockNumber>;

parameter_type_with_key! {
	pub ExistentialDeposits: |currency_id: CurrencyId| -> Balance {
		Default::default()
	};
}

impl pallet_balances::Config for Test {
	type MaxLocks = ();
	type Balance = Balance;
	type Event = ();
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
}

parameter_types! {
	pub const AssetDepositBase: u64 = 1;
	pub const AssetDepositPerZombie: u64 = 1;
}

impl pallet_assets::Config for Test {
	type Currency = Balances;
	type Event = Event;
	type Balance = u64;
	type AssetId = u32;
	type ForceOrigin = frame_system::EnsureRoot<u64>;
	type AssetDepositBase = AssetDepositBase;
	type AssetDepositPerZombie = AssetDepositPerZombie;
	type WeightInfo = ();
}

impl Config for Test {
	type Event = TestEvent;
	type Currency = Balances;
	type PublicKey = RenVmPublicKey;
	type CurrencyIdentifier = RENBTCIdentifier;
	type UnsignedPriority = UnsignedPriority;
}
pub type RenVmBridge = Module<Test>;
pub type System = frame_system::Module<Test>;
pub type Balances = pallet_balances::Module<Test>;
pub type Assets = pallet_assets::Module<Test>;

pub struct ExtBuilder();

impl Default for ExtBuilder {
	fn default() -> Self {
		Self()
	}
}

impl ExtBuilder {
	pub fn build(self) -> sp_io::TestExternalities {
		let t = frame_system::GenesisConfig::default()
			.build_storage::<Test>()
			.unwrap();
		t.into()
	}
}