
use super::*;
use sp_std::prelude::*;
use sp_runtime::traits::Bounded;
use frame_system::RawOrigin as SystemOrigin;
use frame_benchmarking::{benchmarks, account, whitelisted_caller};
use hex_literal::hex;


use crate::Module as EdgeRen;

const SEED: u32 = 0;



//
// fn create_default_minted_ren_token<T: Config>(max_zombies: u32, amount: T::Balance)
// 	-> (T::AccountId, <T::Lookup as StaticLookup>::Source)
// {
// 	let (caller, caller_lookup)  = create_default_asset::<T>(max_zombies);
// 	assert!(Assets::<T>::mint(
// 		SystemOrigin::Signed(caller.clone()).into(),
// 		Default::default(),
// 		caller_lookup.clone(),
// 		amount,
// 	).is_ok());
// 	(caller, caller_lookup)
// }
//

fn ren_token_add_zombies<T: Config>(n: u32) {
	for i in 0..n {
		let target = account("zombie", i, SEED);
		assert!(T::Assets::mint(Default::default(), target, 100u32.into()).is_ok());
	}
}


fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
	let events = frame_system::Module::<T>::events();
	let system_event: <T as frame_system::Config>::Event = generic_event.into();
	// compare to the last event record
	let frame_system::EventRecord { event, .. } = &events[events.len() - 1];
	assert_eq!(event, &system_event);
}

benchmarks! {
	_ { }

	add_ren_token{

	}: _(
		SystemOrigin::Root,
		Default::default(),
		Default::default(),
		Default::default(),
		Default::default(),
		true,
		true,
		0,
		0,
		u32::max_value(),
		1u32.into()
		)
	verify{
		assert_last_event::<T>(RawEvent::RenTokenAdded(Default::default()).into());
	}

	update_ren_token{
		assert!(EdgeRen::<T>::add_ren_token(
			SystemOrigin::Root.into(),
			Default::default(),
			Default::default(),
			Default::default(),
			Default::default(),
			true,
			true,
			0,
			0,
			u32::max_value(),
			1u32.into()
		).is_ok());
	}: _(
		SystemOrigin::Root,
		Default::default(),
		Some("renBTC".as_bytes().to_vec()),
		Some(hex_literal::hex!["f6b5b360905f856404bd4cf39021b82209908faa44159e68ea207ab8a5e13197"]),
		Some(hex_literal::hex!["4b939fc8ade87cb50b78987b1dda927460dc456a"]),
		Some(true),
		Some(true),
		Some(0),
		Some(0)
	)
	verify{
		assert_last_event::<T>(RawEvent::RenTokenUpdated(Default::default()).into());
	}


	delete_ren_token{
		let z in 0 .. 10_000;
		assert!(EdgeRen::<T>::add_ren_token(
			SystemOrigin::Root.into(),
			Default::default(),
			Default::default(),
			Default::default(),
			Default::default(),
			true,
			true,
			0,
			0,
			u32::max_value(),
			1u32.into()
		).is_ok());
		ren_token_add_zombies::<T>(z);
	}: _(SystemOrigin::Root, Default::default(), u32::max_value().into())
	verify{
		assert_last_event::<T>(RawEvent::RenTokenDeleted(Default::default()).into());
	}

	spend_tokens{
		assert!(EdgeRen::<T>::add_ren_token(
			SystemOrigin::Root.into(),
			Default::default(),
			Default::default(),
			Default::default(),
			Default::default(),
			true,
			true,
			0,
			0,
			u32::max_value(),
			1u32.into()
		).is_ok());
		assert!(T::Assets::mint(Default::default(), Module::<T>::account_id(), 100_000u32.into()).is_ok());
		let to_acc: T::AccountId = account("to_acc", 0, SEED);
	}: _(SystemOrigin::Root, Default::default(), to_acc.clone(), 50_000u32.into())
	verify{
		assert_last_event::<T>(RawEvent::RenTokenSpent(Default::default(), to_acc, 50_000u32.into()).into());
	}


	mint{
		assert!(EdgeRen::<T>::add_ren_token(
			SystemOrigin::Root.into(),
			Default::default(),
			"renBTC".as_bytes().to_vec(),
			hex_literal::hex!["f6b5b360905f856404bd4cf39021b82209908faa44159e68ea207ab8a5e13197"],
			hex_literal::hex!["4b939fc8ade87cb50b78987b1dda927460dc456a"],
			true,
			true,
			20,
			20,
			u32::max_value(),
			1u32.into()
		).is_ok());
		let target: T::AccountId = account("target", 0, SEED);
	}: _(
		SystemOrigin::None,
		Default::default(),
		target.clone(),
		hex!["67028f26328144de6ef80b8cd3b05e0cefb488762c340d1574c0542f752996cb"],
		93963.into(),
		hex!["f6a75cc370a2dda6dfc8d016529766bb6099d7fa0d787d9fe5d3a7e60c9ac2a0"],
		EcdsaSignature::from_slice(&hex!["defda6eef01da2e2a90ce30ba73e90d32204ae84cae782b485f01d16b69061e0381a69cafed3deb6112af044c42ed0f7c73ee0eec7b533334d31a06db50fc40e1b"])
		)
	verify{
		assert_last_event::<T>(RawEvent::RenTokenMinted(Default::default(), target, 93963.into()).into());
	}

	burn{
		assert!(EdgeRen::<T>::add_ren_token(
			SystemOrigin::Root.into(),
			Default::default(),
			"renBTC".as_bytes().to_vec(),
			hex_literal::hex!["f6b5b360905f856404bd4cf39021b82209908faa44159e68ea207ab8a5e13197"],
			hex_literal::hex!["4b939fc8ade87cb50b78987b1dda927460dc456a"],
			true,
			true,
			20,
			20,
			u32::max_value(),
			1u32.into()
		).is_ok());
		let target: T::AccountId = account("target", 0, SEED);
		assert!(EdgeRen::<T>::mint(
			SystemOrigin::None.into(),
			Default::default(),
			target.clone(),
			hex!["67028f26328144de6ef80b8cd3b05e0cefb488762c340d1574c0542f752996cb"],
			93963.into(),
			hex!["f6a75cc370a2dda6dfc8d016529766bb6099d7fa0d787d9fe5d3a7e60c9ac2a0"],
			EcdsaSignature::from_slice(&hex!["defda6eef01da2e2a90ce30ba73e90d32204ae84cae782b485f01d16b69061e0381a69cafed3deb6112af044c42ed0f7c73ee0eec7b533334d31a06db50fc40e1b"])
		).is_ok());
	}: _(
		SystemOrigin::Signed(target.clone()),
		Default::default(),
		"17VZNX1SN5NtKa8UQFxwQbFeFc3iqRYhem".as_bytes().to_vec(),
		10000.into()
		)
	verify{
		assert_last_event::<T>(RawEvent::RenTokenBurnt(Default::default(), target, "17VZNX1SN5NtKa8UQFxwQbFeFc3iqRYhem".as_bytes().to_vec(), (10000*19/20).into()).into());
	}

}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::mock::{Runtime, ExtBuilder};
	use frame_support::assert_ok;

	#[test]
	fn test_benchmarks() {
		ExtBuilder::default().build().execute_with(|| {
			assert_ok!(test_benchmark_add_ren_token::<Runtime>());
			assert_ok!(test_benchmark_update_ren_token::<Runtime>());
			assert_ok!(test_benchmark_delete_ren_token::<Runtime>());
			assert_ok!(test_benchmark_spend_tokens::<Runtime>());
			assert_ok!(test_benchmark_mint::<Runtime>());
			// assert_ok!(test_benchmark_burn::<Runtime>());
		});
	}
}
