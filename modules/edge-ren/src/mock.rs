//! Mocks for the airdrop module.

// #![cfg(test)]

use super::*;
use frame_support::{impl_outer_dispatch, impl_outer_event, impl_outer_origin, parameter_types};
use sp_core::H256;
use sp_runtime::{testing::Header, traits::IdentityLookup, Perbill};
use frame_system::{EnsureRoot};

pub type AccountId = H256;
pub type BlockNumber = u64;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Runtime;

mod edge_ren {
	pub use super::super::*;
}

impl_outer_dispatch! {
	pub enum Call for Runtime where origin: Origin {
		frame_system::System,
		pallet_assets::AssetsPallet,
		pallet_balances::Balances,
		edge_ren::RenVmBridge,
	}
}


impl_outer_origin! {
	pub enum Origin for Runtime {}
}

impl_outer_event! {
	pub enum TestEvent for Runtime {
		pallet_assets<T>,
		frame_system<T>,
		pallet_balances<T>,
		edge_ren<T>,
	}
}

pub type RenvmBridgeCall = super::Call<Runtime>;

parameter_types! {
	pub const BlockHashCount: u64 = 250;
}

impl frame_system::Config for Runtime {
	type Origin = Origin;
	type Index = u64;
	type BlockNumber = BlockNumber;
	type Call = Call;
	type Hash = H256;
	type Hashing = ::sp_runtime::traits::BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = TestEvent;
	type BlockHashCount = BlockHashCount;
	type BlockWeights = ();
	type BlockLength = ();
	type Version = ();
	type PalletInfo = ();
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type DbWeight = ();
	type BaseCallFilter = ();
	type SystemWeightInfo = ();
}

parameter_types! {
	pub const ExistentialDeposit: Balance = 0;
}

impl pallet_balances::Config for Runtime {
	type MaxLocks = ();
	type Balance = Balance;
	type DustRemoval = ();
	type Event = TestEvent;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = frame_system::Module<Runtime>;
	type WeightInfo = ();
}
pub type Balances = pallet_balances::Module<Runtime>;


parameter_types! {
	pub const RenvmBridgeUnsignedPriority: u64 = 1 << 20;

	pub const RenVMModuleId: ModuleId = ModuleId(*b"RenToken");
}

impl Config for Runtime {
	type Event = TestEvent;
	type RenvmBridgeUnsignedPriority = RenvmBridgeUnsignedPriority;
	type ControllerOrigin= EnsureRenVM<Runtime>;
	type ModuleId= RenVMModuleId;
	type Assets = AssetsPallet;
}
pub type RenVmBridge = Module<Runtime>;

parameter_types! {
	pub const AssetDepositBase: u64 = 1;
	pub const AssetDepositPerZombie: u64 = 1;
	pub const AssetsAllowFreezing: bool = true;
	pub const AssetsAllowBurning: bool = true;
	pub const AssetsAllowMinting: bool = true;
}

impl pallet_assets::Config for Runtime {
	type Currency = Balances;
	type Event = TestEvent;
	type Balance = Balance;
	type AssetId = u32;
	type ForceOrigin = EnsureRenVM<Runtime>;
	type AssetDepositBase = AssetDepositBase;
	type AssetDepositPerZombie = AssetDepositPerZombie;
	type WeightInfo = ();
	type AllowFreezing = AssetsAllowFreezing;
	type AllowBurning = AssetsAllowBurning;
	type AllowMinting = AssetsAllowMinting;
}


pub type AssetsPallet = pallet_assets::Module<Runtime>;
pub type System = frame_system::Module<Runtime>;

pub struct ExtBuilder();

impl Default for ExtBuilder {
	fn default() -> Self {
		Self()
	}
}

impl ExtBuilder {
	pub fn build(self) -> sp_io::TestExternalities {
		let t = frame_system::GenesisConfig::default()
			.build_storage::<Runtime>()
			.unwrap();
		t.into()
	}
}
