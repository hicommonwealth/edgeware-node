use edgeware_primitives::{BlockNumber, Balance, AuraId, AccountId};
use grandpa::AuthorityId as GrandpaId;
use im_online::ed25519::{AuthorityId as ImOnlineId};
use primitives::crypto::UncheckedInto;
use hex_literal::hex;
use serde::{Deserialize, Serialize};
use serde_json::{Result};
use std::fs::File;
use std::io::Read;
use hex::FromHex;

#[derive(Serialize, Deserialize)]
struct Spec {
    balances: Vec<(String, String)>,
    vesting: Vec<(String, u32, u32, String)>,
    validators: Vec<(String, String, String, String)>,
}

pub fn get_spec_allocation() 
	-> Result<(
		Vec<(AccountId, Balance)>,
		Vec<(AccountId, BlockNumber, BlockNumber, Balance)>,
		Vec<(AccountId, AccountId, AuraId, Balance, GrandpaId, ImOnlineId)>
	)> {
    let mut file = File::open("node/service/src/genesis.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

	let json: Spec = serde_json::from_str(&data)?;;
	let balances_json = json.balances;
	let vesting_json = json.vesting;
	let validators_json = json.validators;

	let balances: Vec<(AccountId, Balance)> = balances_json.into_iter().map(|e| {
		return (
			<[u8; 32]>::from_hex(e.0).unwrap().unchecked_into(),
			e.1.to_string().parse::<Balance>().unwrap(),
		);
	}).collect();
	let vesting: Vec<(AccountId, BlockNumber, BlockNumber, Balance)> = vesting_json.into_iter().map(|e| {
		return (
			<[u8; 32]>::from_hex(e.0).unwrap().unchecked_into(),
			e.1.to_string().parse::<BlockNumber>().unwrap(),
			e.2.to_string().parse::<BlockNumber>().unwrap(),
			e.3.to_string().parse::<Balance>().unwrap()
		);
	}).collect();
	let validators: Vec<(AccountId, AccountId, AuraId, Balance, GrandpaId, ImOnlineId)> = validators_json.into_iter().map(|e| {
		return (
			<[u8; 32]>::from_hex(e.0).unwrap().unchecked_into(),
			<[u8; 32]>::from_hex(e.1).unwrap().unchecked_into(),
			<[u8; 32]>::from_hex(e.2.clone()).unwrap().unchecked_into(),
			e.3.to_string().parse::<Balance>().unwrap(),
			<[u8; 32]>::from_hex(e.2.clone()).unwrap().unchecked_into(),
			<[u8; 32]>::from_hex(e.2).unwrap().unchecked_into(),
		);
	}).collect();
	Ok((balances, vesting, validators))
}

pub fn get_commonwealth_validators() -> -> Vec<(AccountId, AccountId, AuraId, Balance, GrandpaId, ImOnlineId)> {
	return vec![];
}

pub fn get_testnet_commonwealth_validators() -> Vec<(AccountId, AccountId, AuraId, Balance, GrandpaId, ImOnlineId)> {
	return vec![(
		// 5DvWxEcMP66DgHigGm2eTTg4pPueDMMDS5F67ixK2WpCTKMU
		hex!["5239cc265b2d7ac6dad6b640a28a64ce5e09b7de22fd0549c2d282d461da260e"].unchecked_into(),
		// 5EkGSct2SPojcFF6fX6EvF3xbaW5aq3oEW2ujJLjTKk8pexP
		hex!["76a4bad1d5fe37ba60dcc9160f3b0fb1822c64f0e92f2171c358a0b7e59aef37"].unchecked_into(),
		// 5D8sbkeQpqoAXt7E4WcNTBEK3sn4CGp67HRBJRQsqXFfsXB5
		hex!["2f6a032ba0dbdcac7fa68607533971ba399a9a06002978b4c071f87334d153b0"].unchecked_into(),
		50000000000000000000000000,
		// 5GywTGqF81sdGsynZG7hr8DgibrCDsdvNN9mGCQhf7CNqHpv
		hex!["d98ab5ea66c0ee4d443b3b6f896daf9c7fefb4d1d2eeca7653ffae84557cf5f3"].unchecked_into(),
		// 5D8sbkeQpqoAXt7E4WcNTBEK3sn4CGp67HRBJRQsqXFfsXB5
		hex!["2f6a032ba0dbdcac7fa68607533971ba399a9a06002978b4c071f87334d153b0"].unchecked_into(),
	), (
		// 5ECSwHL89ShGsfBt34HyjCHK7gkd6vGT5A4gTa5yd4mKPhYe
		hex!["5e603412d1c84d56f590423a78050aebd3ec34e6d3bc775ca87d19216eb91911"].unchecked_into(),
		// 5C7rVE4qA7GvruqzHjc9RYnoNBP5hbCCKqpEjCm5KEmfvHir
		hex!["0266c9d3e063215ef8f35fc87ccd50489b3c6a2356aac39f89d0667145fab16b"].unchecked_into(),
		// 5HPtGdWoRmiReRYE16AQitm4MG8s47ngfHLGUeKHZuo1Cdry
		hex!["ebcde238597379c874dd51fcca5e0f651972b218c46aa21c471167474e089c86"].unchecked_into(),
		50000000000000000000000000,
		// 5GCJ3HKN5MaseCwqwNJ4pUbpJqRbfAmZXWB8SCMJM6FMyM9B
		hex!["b6bab8caa7be249400b5062d17908c59c0e40dcbe2bd1c818098a5dae916a869"].unchecked_into(),
		// 5HPtGdWoRmiReRYE16AQitm4MG8s47ngfHLGUeKHZuo1Cdry
		hex!["ebcde238597379c874dd51fcca5e0f651972b218c46aa21c471167474e089c86"].unchecked_into(),
	), (
		// 5HVcs12oqxzPBZg13DM2BRqnwVxfgTbyFSFzLmygDpjyfBKG
		hex!["f02d7d7c48f4e042ca7cd3c10a741f2bbc181d913cc8f0bfb5caad17bf1a6c51"].unchecked_into(),
		// 5EHZY4syPYQ3bxs5MUahKsvWfYAHddWt3dim5MVFAnAonHxc
		hex!["6246a5ac798b7c27e061bcc04f37ddad8635face9231d4ae60b2577db6de9c45"].unchecked_into(),
		// 5HWaNteWt4qUKVRY4bF6nbbX6BwJmpp2dgggXVnEPX8ksB4K
		hex!["f0e85f197f2e4f9ebff4bf0896ddaf59cb42acce608507f43e2a14741f467163"].unchecked_into(),
		50000000000000000000000000,
		// 5HNBgaLkwhDNb798Xe2j7Z1PBJSxNKzZT16uKHwQLQrN9ftn
		hex!["ea82027e6819e1b2e85127cec6b94da2fad2bb9dbcdd8a9449b7871fad7942b7"].unchecked_into(),
		// 5HWaNteWt4qUKVRY4bF6nbbX6BwJmpp2dgggXVnEPX8ksB4K
		hex!["f0e85f197f2e4f9ebff4bf0896ddaf59cb42acce608507f43e2a14741f467163"].unchecked_into(),
	), (
		// 5EHvssibZkZHkZwoABzBqbb21rsaJjQg8KfW75TjVZmzbEh9
		hex!["628e7d34e61faa51f4aac5c400406646876c7189575d84eb6d4e4f5ecec8e672"].unchecked_into(),
		// 5DscYaYpUohKFeRJRvKYGU1XuDvLNo4XKuN6gDzeKSxF95eB
		hex!["5002e2d414c9c0dc6753b54499077da71b8abe348ab0e745a78d1ca5e70cd752"].unchecked_into(),
		// 5GnSM7vxsa5weU2EFTFi3qBRtxB66g4MtbaRpgCBRfEzA1G9
		hex!["d0c50804164d9e79b3899df678d6de83a226b581fc972f8b8bdc74070ae7e8af"].unchecked_into(),
		50000000000000000000000000,
		// 5DU7imzCeBoaWPkw6dqVpMUj8zzkgKom3uG3RJtPLNQpVhzk
		hex!["3e1735fcc35cf289761f00cddabc74e91a9b565b9838a205f0027e23d06e76b1"].unchecked_into(),
		// 5GnSM7vxsa5weU2EFTFi3qBRtxB66g4MtbaRpgCBRfEzA1G9
		hex!["d0c50804164d9e79b3899df678d6de83a226b581fc972f8b8bdc74070ae7e8af"].unchecked_into(),
	), (
		// 5CwKvDp9JTo3fLW9Q6NrEZ7PaCCjeLCmGbTnN2hEfs9WfRM7
		hex!["269ba9c9b8a209acdb1858501a649ac20ea2331a519c9104dbdda40356e3723f"].unchecked_into(),
		// 5E6xrSDyaARfbwkYQfDqsP2xA1wzLMoFRiXrpmWiuuV8GuZm
		hex!["5a31704dfdb8e263a15b4af4ddd1c0b14e675377126c3bcddcb9cba0570c040f"].unchecked_into(),
		// 5DDtisexsMoEG94f4tr5qRaSKJ42f1H1kBxEHKgq5Kocvsdq
		hex!["333e04dd11f60ebf3037e2615be6d63b01f310b920f8022fb1d6737a2c73dfa5"].unchecked_into(),
		50000000000000000000000000,
		// 5CzV3FMTHzQxtF3TSkVcp2CNJHnuwUCJjhTsuYEUGxizRAUq
		hex!["29041a9d8ca43fd99a9c0e2447c6d137e7ba991d8475c790cbf78744636f9915"].unchecked_into(),
		// 5DDtisexsMoEG94f4tr5qRaSKJ42f1H1kBxEHKgq5Kocvsdq
		hex!["333e04dd11f60ebf3037e2615be6d63b01f310b920f8022fb1d6737a2c73dfa5"].unchecked_into(),
	), (
		// 5DRQpsFg1BgziDA5oMfwVyWzF8CkwwnxsguSu1utbgRNQFrK
		hex!["3c070e2721a02249bd35a0677e1a3b1a8b9a07c25a902e7e9373b4e4d0378a54"].unchecked_into(),
		// 5GNyVHKVDEh4FfAQNhdkTWKuPT42j9ExsFJHfP7RjDo4s8qB
		hex!["bedff87aaf3154ee73dae8754f9af11e033a0cbba09a8e91f4dde82d3d6bed20"].unchecked_into(),
		// 5ETqvphCpj22HfKcKRc4zS1VCdZXKtaF1ArhHAyQU73ceVm5
		hex!["6a1e4860a31716685e0e0f49f3333c5799bbdab5dd3bb1e674134f6f9b2be689"].unchecked_into(),
		50000000000000000000000000,
		// 5HfcYNfpoch9w88CzAqH9PuWHUzwbSJHBA1v3BF9WsRoLht7
		hex!["f7ccdcf57cd3ecd9e56c3324ad95a1484e6f21b0b6f3540a09ada389499cab9d"].unchecked_into(),
		// 5ETqvphCpj22HfKcKRc4zS1VCdZXKtaF1ArhHAyQU73ceVm5
		hex!["6a1e4860a31716685e0e0f49f3333c5799bbdab5dd3bb1e674134f6f9b2be689"].unchecked_into(),
	), (
		// 5E5JcTyK2jJdwQkTx3dysADVxgYaH2YitKcSRXPHL42UFNQD
		hex!["58ed768c260aa671af835ba9b0de0f911840d8346f3857ec13fd4024eed8e621"].unchecked_into(),
		// 5HQsBQKSbxfEjWX76oerhbZQJQnaej9CKGx2rFpsWpYubTm3
		hex!["ec8d75997fc3d2989d6115f8238e0184fca60e1b458595ad278005ef526c245b"].unchecked_into(),
		// 5He2hStVecXboJfsQvJDgPTtWpExQBw9N4ER2FMSyeMqyzGn
		hex!["f697b0a9e1401161380c5df2b3147646beb2bc0132f1c4b8bd8bdeb5f9532b2e"].unchecked_into(),
		50000000000000000000000000,
		// 5FERuAqE9SBT4jmiivs6bWd37riU3svikz4LnndykvJvTn6b
		hex!["8c1f7054a919f65fc2eaa15f73b5e7912416c3526ec9d8e8005341921a1b0c58"].unchecked_into(),
		// 5He2hStVecXboJfsQvJDgPTtWpExQBw9N4ER2FMSyeMqyzGn
		hex!["f697b0a9e1401161380c5df2b3147646beb2bc0132f1c4b8bd8bdeb5f9532b2e"].unchecked_into(),
	), (
		// 5GRgeaoDUAxYhb14xaYSoMN2G4S7MgoGucmjzHZBUCXS8XQf
		hex!["c0f105033e9e77aa5d67a2bc8f1bfbb15f30c20ffbff463b09f7db8c31ffdc34"].unchecked_into(),
		// 5EWo159vVquevNxJU8h76y9a5URiXVvVSiYggcP5GCYF3dby
		hex!["6c5e2aaaa992fd3cff46a1df494106ac5c410639fb068d36c49344c245392768"].unchecked_into(),
		// 5EBoJbE1WRvocBwGfYydXenBsJ8eKvUkxNduf6qWgx2bndvK
		hex!["5de1854fccd7bd5bf8abfcfd3e50bbeb05649c827a0cdefbd8ec02005b1ee4ef"].unchecked_into(),
		50000000000000000000000000,
		// 5HoL4YjE6hNbsFCFNo369i4tDHB19qSQDeh6KaeKnCwgrkgJ
		hex!["fdaf59bb00991a4a412a1cb73cf592e187064dd65dd8bfdbcc9f32645e52c57d"].unchecked_into(),
		// 5EBoJbE1WRvocBwGfYydXenBsJ8eKvUkxNduf6qWgx2bndvK
		hex!["5de1854fccd7bd5bf8abfcfd3e50bbeb05649c827a0cdefbd8ec02005b1ee4ef"].unchecked_into(),
	), (
		// 5GH23iTUJtvz9KGDK36nXWHtrkm6E83ZZjVPFhb8DKQk3cv3
		hex!["ba551cfbf9e91da337f21658276dfbd56ba43be852395db10a89a64e07978f31"].unchecked_into(),
		// 5HQwfPfmbuWt3fKyEK3SSuDneVtF4MwHbK1afsXPHxAfogyj
		hex!["ec9c8c8d80eab0b1fc4733e25af31137fb656390c595bb1c7536f73b201ede57"].unchecked_into(),
		// 5Ec2hh96mXEavdu2C866hgUC4joBYpBVujXQJgQsDWfUMmM1
		hex!["705c8360296c7b6af2f842e7a0804492c86a855aaa605fdf419f577f1f4fecbe"].unchecked_into(),
		50000000000000000000000000,
		// 5GokhX8Ce1VrMaWFt5RMdAq2EkzoBxdUerFoMzRLDYNPyS2M
		hex!["d1c60ddadc9a3f65da208c5c50e7fc9ed0ab069e79553d08dcc36a401948fa1a"].unchecked_into(),
		// 5Ec2hh96mXEavdu2C866hgUC4joBYpBVujXQJgQsDWfUMmM1
		hex!["705c8360296c7b6af2f842e7a0804492c86a855aaa605fdf419f577f1f4fecbe"].unchecked_into(),
	), (
		// 5G3h5wZrWiKwnYUbSCuM5EDzrSjyYWzjbocJYxH4Q83Q4VfD
		hex!["b02b17ab762a49749169c2bba23ea7e381dfd3daccd0ed4dd557f1de25ee0e71"].unchecked_into(),
		// 5E7hMvzKjcG31jjK7GhD5Xz9ZrKJSRGcct65tNDhccZkehMF
		hex!["5ac08904abd6cb603e582180dc8106898eb1c8a401cf89a06d221f61fee5df46"].unchecked_into(),
		// 5G2yXYTi1JsgbvkRcnzMe6EABrBpi3g1w4QtWe1W4XHmJxpW
		hex!["af9f319aa910050d7bed99f5ee2ba4e25429ac9e7746b94edcdf154b8a901a3c"].unchecked_into(),
		50000000000000000000000000,
		// 5EJbF7phraRrRmSi6a9P9RKVfHZTtwC29qdu7TdaZxE9vJdZ
		hex!["630fa435592579438ffe7a5e6c074617c972a09ff3850bdc25cebaeca40b5c13"].unchecked_into(),
		// 5G2yXYTi1JsgbvkRcnzMe6EABrBpi3g1w4QtWe1W4XHmJxpW
		hex!["af9f319aa910050d7bed99f5ee2ba4e25429ac9e7746b94edcdf154b8a901a3c"].unchecked_into(),
	)];
}

pub fn get_more_endowed() -> Vec<AccountId> {
	return vec![
		// 5GbmRGiV3roZkekSVHigiRonQnauNg5SB3DEnjF1a3VaW2uY
		hex!["c8a18852ebde806e011b4df37861a7b9b3960eea3383d8c5938a2feb6585ca60"].unchecked_into(),
		// 5DaJBK1GzL8ppRvGFGZzCtvXGUzEuqed2dQCp9bQmSWBcovH
		hex!["42cde2ea2ebad078c18be12d567bde59fe019243ddf670c9b46b30cbe0210d44"].unchecked_into(),
		// 5CexFLkSDHFn6jWSaiQs6s3QURig5rPoTCWecdysVxWRD8Jo
		hex!["1a1ec9b100da17f4062f5b23d4c442d6845ac913ff7d6d1ef5357be688b4ef16"].unchecked_into(),
		// 5HNVCZustH928S8mWbQWPcNfEQ5zipccQTiCDbYDrdSDno4f
		hex!["eabcfa4431091ab5742e21164a2e24b0e0d6f3ab96018c002b0188c213272d47"].unchecked_into(),
		// 5CrLFRMME6MbaFXrTUuHZ6VcS3RFHNwtqiP52f8BnEQTARVB
		hex!["22CC8CEE58420B7FF445DC9D6AFAEFC33B658A5F3A26322BA8DAB2D3FB6D2F1F"].unchecked_into(),
	];
}

pub fn get_identity_verifiers() -> Vec<AccountId> {
	return vec![
		// 5DXp7vdd8uS5HnodyNzXhE7oGrJWh7PpVa3DzwoAtRH66SMv
		hex!["40e8f152a7015fb3867e7c108514029942ef9004602d0f3a5f8061a54dfa6f35"].unchecked_into(),
	];
}

pub fn get_root_key() -> AccountId {
	// 5DXp7vdd8uS5HnodyNzXhE7oGrJWh7PpVa3DzwoAtRH66SMv
	return hex!["40e8f152a7015fb3867e7c108514029942ef9004602d0f3a5f8061a54dfa6f35"].unchecked_into();
}

