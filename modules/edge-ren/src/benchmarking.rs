
use super::*;
use sp_std::{prelude::*, convert::TryInto};
use frame_system::RawOrigin as SystemOrigin;
use frame_benchmarking::{benchmarks, account, whitelisted_caller};
use hex_literal::hex;
use sp_io::crypto::{ecdsa_generate, ecdsa_sign};
use sp_core::testing::ECDSA;

use crate::Module as EdgeRen;

const SEED: u32 = 0;

fn sign_paramters_with_ecdsa_pair<T: Config>(p_hash: &[u8; 32], amount: BalanceOf<T>, who: T::AccountId, n_hash: &[u8; 32], token: &[u8; 32])
	-> ([u8;20], [u8;65])
{
	let ecdsa_key = ecdsa_generate(ECDSA, None);
	let msg = Encode::using_encoded(&who, |encoded| {Module::<T>::signable_message(p_hash, amount, encoded, n_hash, token)});
	let sig: [u8;65] = ecdsa_sign(ECDSA, &ecdsa_key, &msg).unwrap().into();
	let signed_message_hash = keccak_256(&msg);
	let recoverd =
		secp256k1_ecdsa_recover(&sig, &signed_message_hash).map_err(|_| "").unwrap();
	let addr_array: [u8; 20] = keccak_256(&recoverd)[12..].try_into().unwrap();
	(addr_array, sig)
}

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

	validate_and_mint{
		let to_acc: T::AccountId = account("to_acc", 0, SEED);
		let (pubkey, sig) = sign_paramters_with_ecdsa_pair::<T>(
			&hex!["67028f26328144de6ef80b8cd3b05e0cefb488762c340d1574c0542f752996cb"],
			93963.into(),
			to_acc.clone(),
			&hex!["f6a75cc370a2dda6dfc8d016529766bb6099d7fa0d787d9fe5d3a7e60c9ac2a0"],
			&hex_literal::hex!["f6b5b360905f856404bd4cf39021b82209908faa44159e68ea207ab8a5e13197"]
		);
		assert!(EdgeRen::<T>::add_ren_token(
			SystemOrigin::Root.into(),
			Default::default(),
			"renBTC".as_bytes().to_vec(),
			hex_literal::hex!["f6b5b360905f856404bd4cf39021b82209908faa44159e68ea207ab8a5e13197"],
			pubkey,
			true,
			true,
			200_000,
			200_000,
			u32::max_value(),
			1u32.into()
		).is_ok());

		let call = Call::<T>::mint(
			Default::default(),
			to_acc.clone(),
			hex!["67028f26328144de6ef80b8cd3b05e0cefb488762c340d1574c0542f752996cb"],
			93963.into(),
			hex!["f6a75cc370a2dda6dfc8d016529766bb6099d7fa0d787d9fe5d3a7e60c9ac2a0"],
			EcdsaSignature::from_slice(&sig)
		);
	}: {
		<Module<T> as frame_support::unsigned::ValidateUnsigned>::validate_unsigned(TransactionSource::InBlock, &call)?;
		EdgeRen::<T>::mint(
			SystemOrigin::None.into(),
			Default::default(),
			to_acc.clone(),
			hex!["67028f26328144de6ef80b8cd3b05e0cefb488762c340d1574c0542f752996cb"],
			93963.into(),
			hex!["f6a75cc370a2dda6dfc8d016529766bb6099d7fa0d787d9fe5d3a7e60c9ac2a0"],
			EcdsaSignature::from_slice(&sig)
		)?;
	}
	verify{
		assert_last_event::<T>(RawEvent::RenTokenMinted(Default::default(), to_acc, 93963.into()).into());
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
			200_000,
			200_000,
			u32::max_value(),
			1u32.into()
		).is_ok());
		let acc: T::AccountId = whitelisted_caller();
		assert!(EdgeRen::<T>::mint(
			SystemOrigin::None.into(),
			Default::default(),
			acc.clone(),
			hex!["67028f26328144de6ef80b8cd3b05e0cefb488762c340d1574c0542f752996cb"],
			93963.into(),
			hex!["f6a75cc370a2dda6dfc8d016529766bb6099d7fa0d787d9fe5d3a7e60c9ac2a0"],
			EcdsaSignature::from_slice(&hex!["defda6eef01da2e2a90ce30ba73e90d32204ae84cae782b485f01d16b69061e0381a69cafed3deb6112af044c42ed0f7c73ee0eec7b533334d31a06db50fc40e1b"])
		).is_ok());
	}: _(
		SystemOrigin::Signed(acc.clone()),
		Default::default(),
		"17VZNX1SN5NtKa8UQFxwQbFeFc3iqRYhem".as_bytes().to_vec(),
		10000.into()
		)
	verify{
		assert_last_event::<T>(RawEvent::RenTokenBurnt(Default::default(), acc, "17VZNX1SN5NtKa8UQFxwQbFeFc3iqRYhem".as_bytes().to_vec(), (10000*80/100).into()).into());
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
			assert_ok!(test_benchmark_validate_and_mint::<Runtime>());
			assert_ok!(test_benchmark_burn::<Runtime>());
		});
	}
}
