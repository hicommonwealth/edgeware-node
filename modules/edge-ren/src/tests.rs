//! Unit tests for the renvm bridge module.


#![cfg(test)]

use super::*;
use frame_support::{assert_err, assert_noop, assert_ok, unsigned::ValidateUnsigned};
use hex_literal::hex;
use mock::{EdgeAssets, AccountId, ExtBuilder, Origin, RenVmBridge, RenvmBridgeCall, System};
use sp_runtime::transaction_validity::TransactionValidityError;
use sp_runtime::{DispatchError,traits::Dispatchable};


fn mint_ren_token(
	_ren_token_id: u32,
	who: AccountId,
	p_hash: [u8; 32],
	amount: mock::Balance,
	n_hash: [u8; 32],
	sig: EcdsaSignature,
) -> Result<DispatchResult, TransactionValidityError> {
	<RenVmBridge as ValidateUnsigned>::validate_unsigned(
		TransactionSource::External,
		&RenvmBridgeCall::mint(_ren_token_id, who.clone(), p_hash, amount, n_hash, sig.clone()),
	)?;

	Ok(RenVmBridge::mint(Origin::none(), _ren_token_id, who, p_hash, amount, n_hash, sig))
}


#[test]
fn token_mint_fails_on_bad_init_but_works_after() {
	ExtBuilder::default().build().execute_with(|| {
		
		assert_ok!(mock::Call::EdgeAssets(edge_assets::Call::force_create(
				0,
				super::Module::<mock::Runtime>::account_id().into(),
				u32::max_value(),
				1u128
			)).dispatch(Origin::signed(super::Module::<mock::Runtime>::account_id()))
		);

		assert_eq!(
			edge_assets::Asset::<mock::Runtime>::contains_key(0),
			true
		);

		assert_noop!(
			mint_ren_token(
				0,
				hex!["d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"].into(),
				hex!["67028f26328144de6ef80b8cd3b05e0cefb488762c340d1574c0542f752996cb"],
				93963,
				hex!["f6a75cc370a2dda6dfc8d016529766bb6099d7fa0d787d9fe5d3a7e60c9ac2a0"],
				EcdsaSignature::from_slice(&hex!["defda6eef01da2e2a90ce30ba73e90d32204ae84cae782b485f01d16b69061e0381a69cafed3deb6112af044c42ed0f7c73ee0eec7b533334d31a06db50fc40e1b"]),
			),
			TransactionValidityError::Invalid(InvalidTransaction::BadProof)
		);

		assert_noop!(
			RenVmBridge::add_ren_token(
				Origin::root(),
				0,
				"renBTC".as_bytes().to_vec(),
				hex_literal::hex!["f6b5b360905f856404bd4cf39021b82209908faa44159e68ea207ab8a5e13197"],
				hex_literal::hex!["4b939fc8ade87cb50b78987b1dda927460dc456a"],
				true,
				true,
				0,
				0,
				u32::max_value(),
				1u32.into()
			),
			edge_assets::Error::<mock::Runtime>::InUse
		);


		assert_ok!(mock::Call::EdgeAssets(edge_assets::Call::force_destroy(
				0,
				0,
			)).dispatch(Origin::signed(super::Module::<mock::Runtime>::account_id()))
		);

		assert_ok!(
			RenVmBridge::add_ren_token(
				Origin::root(),
				0,
				"renBTC".as_bytes().to_vec(),
				hex_literal::hex!["f6b5b360905f856404bd4cf39021b82209908faa44159e68ea207ab8a5e13197"],
				hex_literal::hex!["4b939fc8ade87cb50b78987b1dda927460dc456a"],
				true,
				true,
				0,
				0,
				u32::max_value(),
				1u32.into()
			)
		);


		assert_ok!(
			mint_ren_token(
				0,
				hex!["d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"].into(),
				hex!["67028f26328144de6ef80b8cd3b05e0cefb488762c340d1574c0542f752996cb"],
				93963,
				hex!["f6a75cc370a2dda6dfc8d016529766bb6099d7fa0d787d9fe5d3a7e60c9ac2a0"],
				EcdsaSignature::from_slice(&hex!["defda6eef01da2e2a90ce30ba73e90d32204ae84cae782b485f01d16b69061e0381a69cafed3deb6112af044c42ed0f7c73ee0eec7b533334d31a06db50fc40e1b"]),
			)
		);

		assert_eq!(
			EdgeAssets::balance(0, hex!["d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"].into()),
			93963
		);

		assert_ok!(
			RenVmBridge::delete_ren_token(
				Origin::root(),
				0,
				1
			)
		);

		assert_noop!(
			mint_ren_token(
				0,
				hex!["d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"].into(),
				hex!["425673f98610064b76dbd334783f45ea192f0e954db75ba2ae6b6058a8143d67"],
				87266,
				hex!["fe125f912d2de05e3e34b96a0ce8a8e35d9ed883e830b978871f3e1f5d393726"],
				EcdsaSignature::from_slice(&hex!["acd463fa396c54995e444234e96d793d3977e75f445da219c10bc4947c22622f325f24dfc31e8e56ec21f04fc7669e91db861778a8367444bde6dfb5f95e15ed1b"]),
			),
			TransactionValidityError::Invalid(InvalidTransaction::BadProof)
		);

	});

}



#[test]
fn token_mint_works() {
	ExtBuilder::default().build().execute_with(|| {

		assert_ok!(
			RenVmBridge::add_ren_token(
				Origin::root(),
				0,
				"renBTC".as_bytes().to_vec(),
				hex_literal::hex!["f6b5b360905f856404bd4cf39021b82209908faa44159e68ea207ab8a5e13197"],
				hex_literal::hex!["4b939fc8ade87cb50b78987b1dda927460dc456a"],
				false,
				true,
				0,
				0,
				u32::max_value(),
				1u32.into()
			)
		);


		assert_noop!(
			mint_ren_token(
				0,
				hex!["d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"].into(),
				hex!["67028f26328144de6ef80b8cd3b05e0cefb488762c340d1574c0542f752996cb"],
				93963,
				hex!["f6a75cc370a2dda6dfc8d016529766bb6099d7fa0d787d9fe5d3a7e60c9ac2a0"],
				EcdsaSignature::from_slice(&hex!["defda6eef01da2e2a90ce30ba73e90d32204ae84cae782b485f01d16b69061e0381a69cafed3deb6112af044c42ed0f7c73ee0eec7b533334d31a06db50fc40e1b"]),
			).unwrap_or_else(|_| Err(DispatchError::from(Error::<mock::Runtime>::UnexpectedError))),
			Error::<mock::Runtime>::RenTokenMintDisabled
		);

		assert_ok!(
			RenVmBridge::update_ren_token(
				Origin::root(),
				0,
				None,
				None,
				None,
				Some(true),
				None,
				None,
				None
			)
		);


		assert_ok!(
			mint_ren_token(
				0,
				hex!["d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"].into(),
				hex!["67028f26328144de6ef80b8cd3b05e0cefb488762c340d1574c0542f752996cb"],
				93963,
				hex!["f6a75cc370a2dda6dfc8d016529766bb6099d7fa0d787d9fe5d3a7e60c9ac2a0"],
				EcdsaSignature::from_slice(&hex!["defda6eef01da2e2a90ce30ba73e90d32204ae84cae782b485f01d16b69061e0381a69cafed3deb6112af044c42ed0f7c73ee0eec7b533334d31a06db50fc40e1b"]),
			)
		);

		assert_eq!(
			EdgeAssets::balance(0, hex!["d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"].into()),
			93963
		);

		assert_ok!(
			mint_ren_token(
				0,
				hex!["d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"].into(),
				hex!["425673f98610064b76dbd334783f45ea192f0e954db75ba2ae6b6058a8143d67"],
				87266,
				hex!["fe125f912d2de05e3e34b96a0ce8a8e35d9ed883e830b978871f3e1f5d393726"],
				EcdsaSignature::from_slice(&hex!["acd463fa396c54995e444234e96d793d3977e75f445da219c10bc4947c22622f325f24dfc31e8e56ec21f04fc7669e91db861778a8367444bde6dfb5f95e15ed1b"]),
			)
		);

		assert_eq!(
			EdgeAssets::balance(0, hex!["d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"].into()),
			93963 + 87266
		);

		assert_eq!(
			EdgeAssets::balance(0, super::Module::<mock::Runtime>::account_id().into()),
			0
		);

		assert_noop!(
			mint_ren_token(
				0,
				hex!["d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"].into(),
				hex!["67028f26328144de6ef80b8cd3b05e0cefb488762c340d1574c0542f752996cb"],
				93963,
				hex!["f6a75cc370a2dda6dfc8d016529766bb6099d7fa0d787d9fe5d3a7e60c9ac2a0"],
				EcdsaSignature::from_slice(&hex!["000463fa396c54995e444234e96d793d3977e75f445da219c10bc4947c22622f325f24dfc31e8e56ec21f04fc7669e91db861778a8367444bde6dfb5f95e15ed1b"]),
			),
			TransactionValidityError::Invalid(InvalidTransaction::BadProof)
		);

		assert_noop!(
			mint_ren_token(
				0,
				hex!["d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"].into(),
				hex!["425673f98610064b76dbd334783f45ea192f0e954db75ba2ae6b6058a8143d67"],
				87266,
				hex!["fe125f912d2de05e3e34b96a0ce8a8e35d9ed883e830b978871f3e1f5d393726"],
				EcdsaSignature::from_slice(&hex!["acd463fa396c54995e444234e96d793d3977e75f445da219c10bc4947c22622f325f24dfc31e8e56ec21f04fc7669e91db861778a8367444bde6dfb5f95e15ed1b"]),
			),
			TransactionValidityError::Invalid(InvalidTransaction::Stale)
		);

	});

	ExtBuilder::default().build().execute_with(|| {

		assert_ok!(
			RenVmBridge::add_ren_token(
				Origin::root(),
				1,
				"renBTC_withFee".as_bytes().to_vec(),
				hex_literal::hex!["f6b5b360905f856404bd4cf39021b82209908faa44159e68ea207ab8a5e13197"],
				hex_literal::hex!["4b939fc8ade87cb50b78987b1dda927460dc456a"],
				true,
				true,
				100_000,
				0,
				u32::max_value(),
				1u32.into()
			)
		);


		assert_ok!(
			mint_ren_token(
				1,
				hex!["d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"].into(),
				hex!["67028f26328144de6ef80b8cd3b05e0cefb488762c340d1574c0542f752996cb"],
				93963,
				hex!["f6a75cc370a2dda6dfc8d016529766bb6099d7fa0d787d9fe5d3a7e60c9ac2a0"],
				EcdsaSignature::from_slice(&hex!["defda6eef01da2e2a90ce30ba73e90d32204ae84cae782b485f01d16b69061e0381a69cafed3deb6112af044c42ed0f7c73ee0eec7b533334d31a06db50fc40e1b"]),
			)
		);

		assert_eq!(
			EdgeAssets::balance(1, hex!["d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"].into()),
			93963 - (93963/10)
		);

		assert_ok!(
			mint_ren_token(
				1,
				hex!["d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"].into(),
				hex!["425673f98610064b76dbd334783f45ea192f0e954db75ba2ae6b6058a8143d67"],
				87266,
				hex!["fe125f912d2de05e3e34b96a0ce8a8e35d9ed883e830b978871f3e1f5d393726"],
				EcdsaSignature::from_slice(&hex!["acd463fa396c54995e444234e96d793d3977e75f445da219c10bc4947c22622f325f24dfc31e8e56ec21f04fc7669e91db861778a8367444bde6dfb5f95e15ed1b"]),
			)
		);

		assert_eq!(
			EdgeAssets::balance(1, hex!["d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"].into()),
			93963 + 87266 - ((93963 + 87266)/10)
		);

		assert_eq!(
			EdgeAssets::balance(1, super::Module::<mock::Runtime>::account_id().into()),
			((93963 + 87266)/10)
		);

	});
}

#[test]
fn token_spend_works() {
	ExtBuilder::default().build().execute_with(|| {

		assert_ok!(
			RenVmBridge::add_ren_token(
				Origin::root(),
				1,
				"renBTC_withFee".as_bytes().to_vec(),
				hex_literal::hex!["f6b5b360905f856404bd4cf39021b82209908faa44159e68ea207ab8a5e13197"],
				hex_literal::hex!["4b939fc8ade87cb50b78987b1dda927460dc456a"],
				true,
				true,
				100_000,
				0,
				u32::max_value(),
				1u32.into()
			)
		);


		assert_ok!(
			mint_ren_token(
				1,
				hex!["d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"].into(),
				hex!["67028f26328144de6ef80b8cd3b05e0cefb488762c340d1574c0542f752996cb"],
				93963,
				hex!["f6a75cc370a2dda6dfc8d016529766bb6099d7fa0d787d9fe5d3a7e60c9ac2a0"],
				EcdsaSignature::from_slice(&hex!["defda6eef01da2e2a90ce30ba73e90d32204ae84cae782b485f01d16b69061e0381a69cafed3deb6112af044c42ed0f7c73ee0eec7b533334d31a06db50fc40e1b"]),
			)
		);

		assert_eq!(
			EdgeAssets::balance(1, hex!["d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"].into()),
			93963 - (93963/10)
		);

		assert_ok!(
			mint_ren_token(
				1,
				hex!["d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"].into(),
				hex!["425673f98610064b76dbd334783f45ea192f0e954db75ba2ae6b6058a8143d67"],
				87266,
				hex!["fe125f912d2de05e3e34b96a0ce8a8e35d9ed883e830b978871f3e1f5d393726"],
				EcdsaSignature::from_slice(&hex!["acd463fa396c54995e444234e96d793d3977e75f445da219c10bc4947c22622f325f24dfc31e8e56ec21f04fc7669e91db861778a8367444bde6dfb5f95e15ed1b"]),
			)
		);

		assert_eq!(
			EdgeAssets::balance(1, hex!["d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"].into()),
			93963 + 87266 - ((93963 + 87266)/10)
		);

		assert_eq!(
			EdgeAssets::balance(1, super::Module::<mock::Runtime>::account_id().into()),
			((93963 + 87266)/10)
		);

		assert_ok!(
			RenVmBridge::spend_tokens(
				Origin::root(),
				1,
				hex!["d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"].into(),
				2000,
			)
		);

		assert_eq!(
			EdgeAssets::balance(1, hex!["d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"].into()),
			93963 + 87266 - ((93963 + 87266)/10) + 2000
		);

		assert_eq!(
			EdgeAssets::balance(1, super::Module::<mock::Runtime>::account_id().into()),
			((93963 + 87266)/10) - 2000
		);

	});
}

#[test]
fn token_crud_works() {
	ExtBuilder::default().build().execute_with(|| {

		assert_ok!(
			RenVmBridge::add_ren_token(
				Origin::root(),
				0,
				"renBTC".as_bytes().to_vec(),
				hex_literal::hex!["f6b5b360905f856404bd4cf39021b82209908faa44159e68ea207ab8a5e13197"],
				hex_literal::hex!["4b939fc8ade87cb50b78987b1dda927460dc456a"],
				true,
				true,
				0,
				0,
				u32::max_value(),
				1u32.into()
			)
		);

		assert_eq!(
			RenVmBridge::ren_token_registry(0).map_or_else(||{[0u8;32]}, |x| {x.ren_token_renvm_id}),
			hex_literal::hex!["f6b5b360905f856404bd4cf39021b82209908faa44159e68ea207ab8a5e13197"]
		);

		assert_eq!(
			edge_assets::Asset::<mock::Runtime>::contains_key(0),
			true
		);

		assert_ok!(
			RenVmBridge::update_ren_token(
				Origin::root(),
				0,
				Some("edgeRenBTC".as_bytes().to_vec()),
				None,
				None,
				None,
				None,
				None,
				None
			)
		);

		assert_eq!(
			RenVmBridge::ren_token_registry(0).map_or_else(||{Default::default()}, |x| {x.ren_token_name}),
			"edgeRenBTC".as_bytes().to_vec()
		);

		assert_ok!(
			RenVmBridge::delete_ren_token(
				Origin::root(),
				0,
				0
			)
		);

		assert_eq!(
			RenVmBridge::ren_token_registry(0),
			None
		);

		assert_eq!(
			edge_assets::Asset::<mock::Runtime>::contains_key(0),
			false
		);

	});
}

#[test]
fn verify_signature_works() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(
			RenVmBridge::add_ren_token(
				Origin::root(),
				0,
				"renBTC".as_bytes().to_vec(),
				hex_literal::hex!["f6b5b360905f856404bd4cf39021b82209908faa44159e68ea207ab8a5e13197"],
				hex_literal::hex!["4b939fc8ade87cb50b78987b1dda927460dc456a"],
				true,
				true,
				0,
				0,
				u32::max_value(),
				1u32.into()
			)
		);


		assert_ok!(
			RenVmBridge::verify_signature(
				0,
				&hex!["67028f26328144de6ef80b8cd3b05e0cefb488762c340d1574c0542f752996cb"],
				93963,
				&hex!["d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"],
				&hex!["f6a75cc370a2dda6dfc8d016529766bb6099d7fa0d787d9fe5d3a7e60c9ac2a0"],
				&hex!["defda6eef01da2e2a90ce30ba73e90d32204ae84cae782b485f01d16b69061e0381a69cafed3deb6112af044c42ed0f7c73ee0eec7b533334d31a06db50fc40e1b"]
			)
		);

		assert_ok!(
			RenVmBridge::verify_signature(
				0,
				&hex!["ad8fae51f70e3a013962934614201466076fec72eb60f74183f3059d6ad2c4c1"],
				86129,
				&hex!["d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"],
				&hex!["1cdb2d4388e10ce8f89613f06a0d03a2d3fbcfd334d81d4564f7e1bfc5ebc9bb"],
				&hex!["87f068a20cfaf7752151320dcfde3994f2861cb4dd36aa73a947f23f92f135507607c997b450053914f2e9313ea2d1abf3326a7984341fdf47e4e21f33b54cda1b"]
			)
		);

		assert_ok!(
			RenVmBridge::verify_signature(
				0,
				&hex!["1a98ccc4004f71c29c3ae3ee3a8fe51ece4a0eda383443aa8aaafeec4fd55247"],
				80411,
				&hex!["d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"],
				&hex!["d45761c6d5123a10c5f707472613451de1e738b544acfbdd4b2680754ed2008a"],
				&hex!["1281893709fd7f4e1d65147a948d9884adf65bb9bcb587ea32e2f3b633fa1e1f2d82488ae89105004a301eda66ef8e5f036b705716f1df42d357647e09dd3e581c"]
			)
		);

		assert_ok!(
			RenVmBridge::verify_signature(
				0,
				&hex!["425673f98610064b76dbd334783f45ea192f0e954db75ba2ae6b6058a8143d67"],
				87266,
				&hex!["d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"],
				&hex!["fe125f912d2de05e3e34b96a0ce8a8e35d9ed883e830b978871f3e1f5d393726"],
				&hex!["acd463fa396c54995e444234e96d793d3977e75f445da219c10bc4947c22622f325f24dfc31e8e56ec21f04fc7669e91db861778a8367444bde6dfb5f95e15ed1b"]
			)
		);

		assert_ok!(
			RenVmBridge::verify_signature(
				0,
				&hex!["046076abc0c7e2bd8cc15b9e22ed97deff2d8e2acfe3bec1ffbbd0255b2a094c"],
				87403,
				&hex!["d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"],
				&hex!["64962866cd5245005a06b8a10ac57626f176bc1c8e340a008c4a765a56aa4a6f"],
				&hex!["63f68adcda25db1de27b0edeb0439f7d971a22afeebb5ddb07ed05d4b07ac4fd1f78e5ecd4f2d6a21aabcc73027e8b977f9a688ae16db5aaf6c0d0021e85e3f41b"]
			)
		);

		assert_err!(
			RenVmBridge::verify_signature(
				0,
				&hex!["046076abc0c7e2bd8cc15b9e22ed97deff2d8e2acfe3bec1ffbbd0255b2a094c"],
				87403,
				&hex!["d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"],
				&hex!["64962866cd5245005a06b8a10ac57626f176bc1c8e340a008c4a765a56aa4a6f"],
				&hex!["63f68adcda25db1de27b0edeb0439f7d971a22afeebb5ddb07ed05d4b07ac4fd1f78e5ecd4f2d6a21aabcc73027e8b977f9a688ae16db5aaf6c0d0021e85e3f41a"]
			),
			Error::<mock::Runtime>::InvalidMintSignature
		);

	});
}


#[test]
fn token_burn_works() {
	ExtBuilder::default().build().execute_with(|| {

		assert_ok!(
			RenVmBridge::add_ren_token(
				Origin::root(),
				0,
				"renBTC".as_bytes().to_vec(),
				hex_literal::hex!["f6b5b360905f856404bd4cf39021b82209908faa44159e68ea207ab8a5e13197"],
				hex_literal::hex!["4b939fc8ade87cb50b78987b1dda927460dc456a"],
				true,
				false,
				0,
				200_000,
				u32::max_value(),
				1u32.into()
			)
		);


		assert_ok!(
			mint_ren_token(
				0,
				hex!["d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"].into(),
				hex!["67028f26328144de6ef80b8cd3b05e0cefb488762c340d1574c0542f752996cb"],
				93963,
				hex!["f6a75cc370a2dda6dfc8d016529766bb6099d7fa0d787d9fe5d3a7e60c9ac2a0"],
				EcdsaSignature::from_slice(&hex!["defda6eef01da2e2a90ce30ba73e90d32204ae84cae782b485f01d16b69061e0381a69cafed3deb6112af044c42ed0f7c73ee0eec7b533334d31a06db50fc40e1b"]),
			)
		);

		assert_eq!(
			EdgeAssets::balance(0, hex!["d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"].into()),
			93963
		);


		assert_eq!(RenVmBridge::burn_events(0), None);

		assert_noop!(
			RenVmBridge::burn(
				Origin::signed(hex!["d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"].into()),
				0,
				"17VZNX1SN5NtKa8UQFxwQbFeFc3iqRYhem".as_bytes().to_vec(),
				1000
			),
			Error::<mock::Runtime>::RenTokenBurnDisabled
		);

		assert_ok!(
			RenVmBridge::update_ren_token(
				Origin::root(),
				0,
				None,
				None,
				None,
				None,
				Some(true),
				None,
				None
			)
		);

		assert_ok!(
			RenVmBridge::burn(
				Origin::signed(hex!["d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"].into()),
				0,
				"17VZNX1SN5NtKa8UQFxwQbFeFc3iqRYhem".as_bytes().to_vec(),
				1000
			)
		);


		assert_eq!(
			EdgeAssets::balance(0, hex!["d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"].into()),
			92963
		);

		assert_eq!(
			EdgeAssets::balance(0, super::Module::<mock::Runtime>::account_id().into()),
			1000*20/100
		);

		assert_eq!(RenVmBridge::burn_events(0), Some((0, 0, "17VZNX1SN5NtKa8UQFxwQbFeFc3iqRYhem".as_bytes().to_vec(), 1000*80/100)));
		assert_eq!(RenVmBridge::next_burn_event_id(), 1);

		System::set_block_number(15);

		assert_ok!(
			RenVmBridge::burn(
				Origin::signed(hex!["d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"].into()),
				0,
				"3EktnHQD7RiAE6uzMj2ZifT9YgRrkSgzQX".as_bytes().to_vec(),
				2000
			)
		);

		assert_eq!(
			EdgeAssets::balance(0, hex!["d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"].into()),
			90963
		);

		assert_eq!(
			EdgeAssets::balance(0, super::Module::<mock::Runtime>::account_id().into()),
			1000*20/100 + 2000*20/100
		);

		assert_eq!(RenVmBridge::burn_events(1), Some((0, 15, "3EktnHQD7RiAE6uzMj2ZifT9YgRrkSgzQX".as_bytes().to_vec(), 2000*80/100)));
		assert_eq!(RenVmBridge::next_burn_event_id(), 2);

	});
}
