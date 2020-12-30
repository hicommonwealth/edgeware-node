#![cfg_attr(not(feature = "std"), no_std)]

mod mock;

use sp_std::prelude::*;
use frame_system::{RawOrigin, Module as System, Config as SystemConfig};
use pallet_balances::{Config as BalancesConfig};
use pallet_assets::{Config as AssetsPalletConfig, Module as AssetsPallet};
use edge_ren::{Config as RenVmBridgeConfig, Module as RenVmBridge};
use sp_runtime::{traits::{ StaticLookup}};
use pallet_assets::Call;
use sp_runtime::{DispatchError,traits::Dispatchable};


use frame_benchmarking::{benchmarks};

pub struct Module<T: Config>(RenVmBridge<T>);

pub trait Config:
	SystemConfig
	+ AssetsPalletConfig
	+ BalancesConfig
	+ RenVmBridgeConfig
{}


const SEED: u32 = 0;


benchmarks! {
	_ { }

	add_ren_token {

		// let call = Call::<T>::force_create(
		// 		Default::default(),
		// 		T::Lookup::unlookup(Default::default()),
		// 		u32::max_value(),
		// 		Default::default()
		// 	);

		// frame_system::Call::<T>(pallet_assets::Call::<T>::force_create(
		// 		Default::default(),
		// 		T::Lookup::unlookup(Default::default()),
		// 		u32::max_value(),
		// 		Default::default()
		// 	));
		AssetsPallet::<T>::force_create(
				RawOrigin::Root.into(),
				Default::default(),
				T::Lookup::unlookup(Default::default()),
				u32::max_value(),
				Default::default()
			);

	}:
	{let _ = RenVmBridge::<T>::add_ren_token(RawOrigin::Root.into(),Default::default(),"renBTC".into(),[0u8; 32],[0u8; 20],true,true,0u32.into(),0u32.into());}
	// {call.dispatch();}
	verify {

	}

}

#[cfg(test)]
mod tests {
	use super::*;
	pub use mock::{Runtime, ExtBuilder, AccountId, Balances, Origin, RenVmBridge, RenvmBridgeCall, System, AssetsPallet, Call};
	use frame_support::assert_ok;

	fn test_benchmarks() {
		ExtBuilder::default().build().execute_with(|| {
			assert_ok!(test_benchmark_add_ren_token::<Runtime>());
		});
	}
}
