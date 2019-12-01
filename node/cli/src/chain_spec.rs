// Copyright 2018-2019 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.

//! Substrate chain configurations.

use chain_spec::ChainSpecExtension;
use primitives::{Pair, Public, crypto::UncheckedInto, sr25519};
use serde::{Serialize, Deserialize};
use edgeware_runtime::{
	AuthorityDiscoveryConfig, AuraConfig, BalancesConfig, ContractsConfig, CouncilConfig, DemocracyConfig,
	GrandpaConfig, ImOnlineConfig, IndicesConfig, SessionConfig, SessionKeys, StakerStatus, StakingConfig, SudoConfig,
	SystemConfig, WASM_BINARY,
	IdentityConfig, SignalingConfig, TreasuryRewardConfig,
};
use edgeware_runtime::Block;
use edgeware_runtime::constants::currency::*;
use substrate_service;
use hex_literal::hex;
use substrate_telemetry::TelemetryEndpoints;
use grandpa_primitives::{AuthorityId as GrandpaId};
use aura_primitives::ed25519::AuthorityId as AuraId;
use im_online::sr25519::{AuthorityId as ImOnlineId};
use authority_discovery_primitives::AuthorityId as AuthorityDiscoveryId;
use sr_primitives::{Perbill, traits::{Verify, IdentifyAccount, One}};

pub use edgeware_primitives::{AccountId, Balance, Signature, BlockNumber};
pub use edgeware_runtime::GenesisConfig;
pub use edgeware_runtime::constants::{time::*};

use std::fs::File;
use std::io::Read;
use std::path::Path;
use serde_json::{Result};
use hex::FromHex;

type AccountPublic = <Signature as Verify>::Signer;

const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
const DEFAULT_PROTOCOL_ID: &str = "edg";

#[derive(Serialize, Deserialize, Debug)]
struct Allocation {
    balances: Vec<(String, String)>,
    vesting: Vec<(String, BlockNumber, BlockNumber, String)>,
}

fn get_lockdrop_participants_allocation() -> Result<Allocation>{
	let path = Path::new("node/cli/fixtures/lockdrop-allocation.json");
	let mut file = File::open(&path).unwrap();
	let mut data = String::new();
	file.read_to_string(&mut data).unwrap();
	let a: Allocation = serde_json::from_str(&data)?;
	return Ok(a);
}

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
pub struct Extensions {
	/// Block numbers with known hashes.
	pub fork_blocks: client::ForkBlocks<Block>,
}

/// Specialized `ChainSpec`.
pub type ChainSpec = substrate_service::ChainSpec<
	GenesisConfig,
	Extensions,
>;

/// 0.9.0 Testnet configuration
pub fn edgeware_testnet_v090_config() -> ChainSpec {
	match ChainSpec::from_json_file(std::path::PathBuf::from("chains/testnet-0.9.0.chainspec.json")) {
		Ok(spec) => spec,
		Err(e) => panic!(e),
	}
}

/// 0.9.5 Testnet configuration
pub fn edgeware_testnet_v095_config() -> ChainSpec {
	match ChainSpec::from_json_file(std::path::PathBuf::from("chains/testnet-0.9.5.chainspec.json")) {
		Ok(spec) => spec,
		Err(e) => panic!(e),
	}
}

fn session_keys(
	grandpa: GrandpaId,
	aura: AuraId,
	im_online: ImOnlineId,
	authority_discovery: AuthorityDiscoveryId,
) -> SessionKeys {
	SessionKeys { grandpa, aura, im_online, authority_discovery }
}

fn staging_testnet_config_genesis() -> GenesisConfig {
	// stash, controller, session-key
	// generated with secret:
	// for i in 1 2 3 4 ; do for j in stash controller; do subkey inspect "$secret"/fir/$j/$i; done; done
	// and
	// for i in 1 2 3 4 ; do for j in session; do subkey --ed25519 inspect "$secret"//fir//$j//$i; done; done

	let initial_authorities: Vec<(AccountId, AccountId, GrandpaId, AuraId, ImOnlineId, AuthorityDiscoveryId)> = vec![(
		// 5Fbsd6WXDGiLTxunqeK5BATNiocfCqu9bS1yArVjCgeBLkVy
		hex!["9c7a2ee14e565db0c69f78c7b4cd839fbf52b607d867e9e9c5a79042898a0d12"].into(),
		// 5EnCiV7wSHeNhjW3FSUwiJNkcc2SBkPLn5Nj93FmbLtBjQUq
		hex!["781ead1e2fa9ccb74b44c19d29cb2a7a4b5be3972927ae98cd3877523976a276"].into(),
		// 5Fb9ayurnxnaXj56CjmyQLBiadfRCqUbL2VWNbbe1nZU6wiC
		hex!["9becad03e6dcac03cee07edebca5475314861492cdfc96a2144a67bbe9699332"].unchecked_into(),
		// 5EZaeQ8djPcq9pheJUhgerXQZt9YaHnMJpiHMRhwQeinqUW8
		hex!["6e7e4eb42cbd2e0ab4cae8708ce5509580b8c04d11f6758dbf686d50fe9f9106"].unchecked_into(),
		// 5EZaeQ8djPcq9pheJUhgerXQZt9YaHnMJpiHMRhwQeinqUW8
		hex!["6e7e4eb42cbd2e0ab4cae8708ce5509580b8c04d11f6758dbf686d50fe9f9106"].unchecked_into(),
		// 5EZaeQ8djPcq9pheJUhgerXQZt9YaHnMJpiHMRhwQeinqUW8
		hex!["6e7e4eb42cbd2e0ab4cae8708ce5509580b8c04d11f6758dbf686d50fe9f9106"].unchecked_into(),
	),(
		// 5ERawXCzCWkjVq3xz1W5KGNtVx2VdefvZ62Bw1FEuZW4Vny2
		hex!["68655684472b743e456907b398d3a44c113f189e56d1bbfd55e889e295dfde78"].into(),
		// 5Gc4vr42hH1uDZc93Nayk5G7i687bAQdHHc9unLuyeawHipF
		hex!["c8dc79e36b29395413399edaec3e20fcca7205fb19776ed8ddb25d6f427ec40e"].into(),
		// 5EockCXN6YkiNCDjpqqnbcqd4ad35nU4RmA1ikM4YeRN4WcE
		hex!["7932cff431e748892fa48e10c63c17d30f80ca42e4de3921e641249cd7fa3c2f"].unchecked_into(),
		// 5DhLtiaQd1L1LU9jaNeeu9HJkP6eyg3BwXA7iNMzKm7qqruQ
		hex!["482dbd7297a39fa145c570552249c2ca9dd47e281f0c500c971b59c9dcdcd82e"].unchecked_into(),
		// 5DhLtiaQd1L1LU9jaNeeu9HJkP6eyg3BwXA7iNMzKm7qqruQ
		hex!["482dbd7297a39fa145c570552249c2ca9dd47e281f0c500c971b59c9dcdcd82e"].unchecked_into(),
		// 5DhLtiaQd1L1LU9jaNeeu9HJkP6eyg3BwXA7iNMzKm7qqruQ
		hex!["482dbd7297a39fa145c570552249c2ca9dd47e281f0c500c971b59c9dcdcd82e"].unchecked_into(),
	),(
		// 5DyVtKWPidondEu8iHZgi6Ffv9yrJJ1NDNLom3X9cTDi98qp
		hex!["547ff0ab649283a7ae01dbc2eb73932eba2fb09075e9485ff369082a2ff38d65"].into(),
		// 5FeD54vGVNpFX3PndHPXJ2MDakc462vBCD5mgtWRnWYCpZU9
		hex!["9e42241d7cd91d001773b0b616d523dd80e13c6c2cab860b1234ef1b9ffc1526"].into(),
		// 5E1jLYfLdUQKrFrtqoKgFrRvxM3oQPMbf6DfcsrugZZ5Bn8d
		hex!["5633b70b80a6c8bb16270f82cca6d56b27ed7b76c8fd5af2986a25a4788ce440"].unchecked_into(),
		// 5DhKqkHRkndJu8vq7pi2Q5S3DfftWJHGxbEUNH43b46qNspH
		hex!["482a3389a6cf42d8ed83888cfd920fec738ea30f97e44699ada7323f08c3380a"].unchecked_into(),
		// 5DhKqkHRkndJu8vq7pi2Q5S3DfftWJHGxbEUNH43b46qNspH
		hex!["482a3389a6cf42d8ed83888cfd920fec738ea30f97e44699ada7323f08c3380a"].unchecked_into(),
		// 5DhKqkHRkndJu8vq7pi2Q5S3DfftWJHGxbEUNH43b46qNspH
		hex!["482a3389a6cf42d8ed83888cfd920fec738ea30f97e44699ada7323f08c3380a"].unchecked_into(),
	),(
		// 5HYZnKWe5FVZQ33ZRJK1rG3WaLMztxWrrNDb1JRwaHHVWyP9
		hex!["f26cdb14b5aec7b2789fd5ca80f979cef3761897ae1f37ffb3e154cbcc1c2663"].into(),
		// 5EPQdAQ39WQNLCRjWsCk5jErsCitHiY5ZmjfWzzbXDoAoYbn
		hex!["66bc1e5d275da50b72b15de072a2468a5ad414919ca9054d2695767cf650012f"].into(),
		// 5DMa31Hd5u1dwoRKgC4uvqyrdK45RHv3CpwvpUC1EzuwDit4
		hex!["3919132b851ef0fd2dae42a7e734fe547af5a6b809006100f48944d7fae8e8ef"].unchecked_into(),
		// 5C4vDQxA8LTck2xJEy4Yg1hM9qjDt4LvTQaMo4Y8ne43aU6x
		hex!["00299981a2b92f878baaf5dbeba5c18d4e70f2a1fcd9c61b32ea18daf38f4378"].unchecked_into(),
		// 5C4vDQxA8LTck2xJEy4Yg1hM9qjDt4LvTQaMo4Y8ne43aU6x
		hex!["00299981a2b92f878baaf5dbeba5c18d4e70f2a1fcd9c61b32ea18daf38f4378"].unchecked_into(),
		// 5C4vDQxA8LTck2xJEy4Yg1hM9qjDt4LvTQaMo4Y8ne43aU6x
		hex!["00299981a2b92f878baaf5dbeba5c18d4e70f2a1fcd9c61b32ea18daf38f4378"].unchecked_into(),
	)];

	// generated with secret: subkey inspect "$secret"/fir
	let root_key: AccountId = hex![
		// 5Ff3iXP75ruzroPWRP2FYBHWnmGGBSb63857BgnzCoXNxfPo
		"9ee5e5bdc0ec239eb164f865ecc345ce4c88e76ee002e0f7e318097347471809"
	].into();

	let endowed_accounts: Vec<AccountId> = vec![root_key.clone()];

	testnet_genesis(
		initial_authorities,
		root_key,
		Some(endowed_accounts),
		false,
		vec![],
		vec![]
	)
}

/// Staging testnet config.
pub fn staging_testnet_config() -> ChainSpec {
	let boot_nodes = vec![];
	ChainSpec::from_genesis(
		"Staging Testnet",
		"staging_testnet",
		staging_testnet_config_genesis,
		boot_nodes,
		Some(TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])),
		None,
		None,
		Default::default(),
	)
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed
pub fn get_authority_keys_from_seed(seed: &str) -> (
	AccountId,
	AccountId,
	GrandpaId,
	AuraId,
	ImOnlineId,
	AuthorityDiscoveryId,
) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
		get_account_id_from_seed::<sr25519::Public>(seed),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<AuraId>(seed),
		get_from_seed::<ImOnlineId>(seed),
		get_from_seed::<AuthorityDiscoveryId>(seed),
	)
}

/// Helper function to create GenesisConfig for testing
pub fn testnet_genesis(
	initial_authorities: Vec<(AccountId, AccountId, GrandpaId, AuraId, ImOnlineId, AuthorityDiscoveryId)>,
	root_key: AccountId,
	endowed_accounts: Option<Vec<AccountId>>,
	enable_println: bool,
	balances: Vec<(AccountId, Balance)>,
	vesting: Vec<(AccountId, BlockNumber, BlockNumber, Balance)>,
) -> GenesisConfig {
	let endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
		vec![
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			get_account_id_from_seed::<sr25519::Public>("Bob"),
			get_account_id_from_seed::<sr25519::Public>("Charlie"),
			get_account_id_from_seed::<sr25519::Public>("Dave"),
			get_account_id_from_seed::<sr25519::Public>("Eve"),
			get_account_id_from_seed::<sr25519::Public>("Ferdie"),
			get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
			get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
			get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
			get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
			get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
			get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
		]
	});

	const ENDOWMENT: Balance = 10_000_000 * DOLLARS;
	const STASH: Balance = 100 * DOLLARS;

	GenesisConfig {
		system: Some(SystemConfig {
			code: WASM_BINARY.to_vec(),
			changes_trie_config: Default::default(),
		}),
		balances: Some(BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|k| (k, ENDOWMENT))
				.chain(initial_authorities.iter().map(|x| (x.0.clone(), STASH)))
				.chain(balances.clone())
				.collect(),
			vesting: vesting,
		}),
		indices: Some(IndicesConfig {
			ids: endowed_accounts.iter().cloned()
				.chain(initial_authorities.iter().map(|x| x.0.clone()))
				.chain(balances.iter().map(|x| x.0.clone()))
				.collect::<Vec<_>>(),
		}),
		session: Some(SessionConfig {
			keys: initial_authorities.iter().map(|x| {
				(x.0.clone(), session_keys(x.2.clone(), x.3.clone(), x.4.clone(), x.5.clone()))
			}).collect::<Vec<_>>(),
		}),
		staking: Some(StakingConfig {
			current_era: 0,
			validator_count: initial_authorities.len() as u32 * 2,
			minimum_validator_count: initial_authorities.len() as u32,
			stakers: initial_authorities.iter().map(|x| {
				(x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator)
			}).collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			.. Default::default()
		}),
		democracy: Some(DemocracyConfig::default()),
		collective_Instance1: Some(CouncilConfig {
			members: vec![],
			phantom: Default::default(),
		}),
		contracts: Some(ContractsConfig {
			current_schedule: contracts::Schedule {
				enable_println, // this should only be enabled on development chains
				..Default::default()
			},
			gas_price: 1 * MILLICENTS,
		}),
		sudo: Some(SudoConfig {
			key: root_key,
		}),
		aura: Some(AuraConfig {
			authorities: vec![],
		}),
		im_online: Some(ImOnlineConfig {
			keys: vec![],
		}),
		authority_discovery: Some(AuthorityDiscoveryConfig {
			keys: vec![],
		}),
		grandpa: Some(GrandpaConfig {
			authorities: vec![],
		}),
		treasury: Some(Default::default()),
		identity: Some(IdentityConfig {
			verifiers: vec![initial_authorities[0].0.clone()],
			expiration_length: 1 * DAYS,
			registration_bond: 1 * DOLLARS,
		}),
		signaling: Some(SignalingConfig {
			voting_length: 3 * DAYS,
			proposal_creation_bond: 100 * DOLLARS,
		}),
		treasury_reward: Some(TreasuryRewardConfig {
			current_payout: 158 * DOLLARS,
			minting_interval: One::one(),
		}),
	}
}

fn edgeware_testnet_config_genesis() -> GenesisConfig {
	let allocation = get_lockdrop_participants_allocation().unwrap();
	let balances = allocation.balances.iter().map(|b| {
		let balance = b.1.to_string().parse::<Balance>().unwrap();
		return (
			<[u8; 32]>::from_hex(b.0.clone()).unwrap().into(),
			balance,
		);
	})
	.filter(|b| b.1 > 0)
	.collect();
	let vesting = allocation.vesting.iter().map(|b| {
		let vesting_balance = b.3.to_string().parse::<Balance>().unwrap();
		return (
			(<[u8; 32]>::from_hex(b.0.clone()).unwrap()).into(),
			b.1,
			b.2,
			vesting_balance,
		);
	})
	.filter(|b| b.3 > 0)
	.collect();

	let initial_authorities: Vec<(AccountId, AccountId, GrandpaId, AuraId, ImOnlineId, AuthorityDiscoveryId)> = vec![(
		// 5DRQpsFg1BgziDA5oMfwVyWzF8CkwwnxsguSu1utbgRNQFrK
		hex!["3c070e2721a02249bd35a0677e1a3b1a8b9a07c25a902e7e9373b4e4d0378a54"].into(),
		// 5GNyVHKVDEh4FfAQNhdkTWKuPT42j9ExsFJHfP7RjDo4s8qB
		hex!["bedff87aaf3154ee73dae8754f9af11e033a0cbba09a8e91f4dde82d3d6bed20"].into(),
		// 5HfcYNfpoch9w88CzAqH9PuWHUzwbSJHBA1v3BF9WsRoLht7
		hex!["f7ccdcf57cd3ecd9e56c3324ad95a1484e6f21b0b6f3540a09ada389499cab9d"].unchecked_into(),
		// 5ETqvphCpj22HfKcKRc4zS1VCdZXKtaF1ArhHAyQU73ceVm5
		hex!["6a1e4860a31716685e0e0f49f3333c5799bbdab5dd3bb1e674134f6f9b2be689"].unchecked_into(),
		// 5ETqvphCpj22HfKcKRc4zS1VCdZXKtaF1ArhHAyQU73ceVm5
		hex!["6a1e4860a31716685e0e0f49f3333c5799bbdab5dd3bb1e674134f6f9b2be689"].unchecked_into(),
		// 5ETqvphCpj22HfKcKRc4zS1VCdZXKtaF1ArhHAyQU73ceVm5
		hex!["6a1e4860a31716685e0e0f49f3333c5799bbdab5dd3bb1e674134f6f9b2be689"].unchecked_into(),
	),(
		// 5EHvssibZkZHkZwoABzBqbb21rsaJjQg8KfW75TjVZmzbEh9
		hex!["628e7d34e61faa51f4aac5c400406646876c7189575d84eb6d4e4f5ecec8e672"].into(),
		// 5DscYaYpUohKFeRJRvKYGU1XuDvLNo4XKuN6gDzeKSxF95eB
		hex!["5002e2d414c9c0dc6753b54499077da71b8abe348ab0e745a78d1ca5e70cd752"].into(),
		// 5DU7imzCeBoaWPkw6dqVpMUj8zzkgKom3uG3RJtPLNQpVhzk
		hex!["3e1735fcc35cf289761f00cddabc74e91a9b565b9838a205f0027e23d06e76b1"].unchecked_into(),
		// 5GnSM7vxsa5weU2EFTFi3qBRtxB66g4MtbaRpgCBRfEzA1G9
		hex!["d0c50804164d9e79b3899df678d6de83a226b581fc972f8b8bdc74070ae7e8af"].unchecked_into(),
		// 5GnSM7vxsa5weU2EFTFi3qBRtxB66g4MtbaRpgCBRfEzA1G9
		hex!["d0c50804164d9e79b3899df678d6de83a226b581fc972f8b8bdc74070ae7e8af"].unchecked_into(),
		// 5GnSM7vxsa5weU2EFTFi3qBRtxB66g4MtbaRpgCBRfEzA1G9
		hex!["d0c50804164d9e79b3899df678d6de83a226b581fc972f8b8bdc74070ae7e8af"].unchecked_into(),
	), (
		// 5CwKvDp9JTo3fLW9Q6NrEZ7PaCCjeLCmGbTnN2hEfs9WfRM7
		hex!["269ba9c9b8a209acdb1858501a649ac20ea2331a519c9104dbdda40356e3723f"].into(),
		// 5E6xrSDyaARfbwkYQfDqsP2xA1wzLMoFRiXrpmWiuuV8GuZm
		hex!["5a31704dfdb8e263a15b4af4ddd1c0b14e675377126c3bcddcb9cba0570c040f"].into(),
		// 5CzV3FMTHzQxtF3TSkVcp2CNJHnuwUCJjhTsuYEUGxizRAUq
		hex!["29041a9d8ca43fd99a9c0e2447c6d137e7ba991d8475c790cbf78744636f9915"].unchecked_into(),
		// 5DDtisexsMoEG94f4tr5qRaSKJ42f1H1kBxEHKgq5Kocvsdq
		hex!["333e04dd11f60ebf3037e2615be6d63b01f310b920f8022fb1d6737a2c73dfa5"].unchecked_into(),
		// 5DDtisexsMoEG94f4tr5qRaSKJ42f1H1kBxEHKgq5Kocvsdq
		hex!["333e04dd11f60ebf3037e2615be6d63b01f310b920f8022fb1d6737a2c73dfa5"].unchecked_into(),
		// 5DDtisexsMoEG94f4tr5qRaSKJ42f1H1kBxEHKgq5Kocvsdq
		hex!["333e04dd11f60ebf3037e2615be6d63b01f310b920f8022fb1d6737a2c73dfa5"].unchecked_into(),
	), (
		// 5DvWxEcMP66DgHigGm2eTTg4pPueDMMDS5F67ixK2WpCTKMU
		hex!["5239cc265b2d7ac6dad6b640a28a64ce5e09b7de22fd0549c2d282d461da260e"].into(),
		// 5EkGSct2SPojcFF6fX6EvF3xbaW5aq3oEW2ujJLjTKk8pexP
		hex!["76a4bad1d5fe37ba60dcc9160f3b0fb1822c64f0e92f2171c358a0b7e59aef37"].into(),
		// 5GywTGqF81sdGsynZG7hr8DgibrCDsdvNN9mGCQhf7CNqHpv
		hex!["d98ab5ea66c0ee4d443b3b6f896daf9c7fefb4d1d2eeca7653ffae84557cf5f3"].unchecked_into(),
		// 5D8sbkeQpqoAXt7E4WcNTBEK3sn4CGp67HRBJRQsqXFfsXB5
		hex!["2f6a032ba0dbdcac7fa68607533971ba399a9a06002978b4c071f87334d153b0"].unchecked_into(),
		// 5D8sbkeQpqoAXt7E4WcNTBEK3sn4CGp67HRBJRQsqXFfsXB5
		hex!["2f6a032ba0dbdcac7fa68607533971ba399a9a06002978b4c071f87334d153b0"].unchecked_into(),
		// 5D8sbkeQpqoAXt7E4WcNTBEK3sn4CGp67HRBJRQsqXFfsXB5
		hex!["2f6a032ba0dbdcac7fa68607533971ba399a9a06002978b4c071f87334d153b0"].unchecked_into(),
	), (
		// 5ECSwHL89ShGsfBt34HyjCHK7gkd6vGT5A4gTa5yd4mKPhYe
		hex!["5e603412d1c84d56f590423a78050aebd3ec34e6d3bc775ca87d19216eb91911"].into(),
		// 5C7rVE4qA7GvruqzHjc9RYnoNBP5hbCCKqpEjCm5KEmfvHir
		hex!["0266c9d3e063215ef8f35fc87ccd50489b3c6a2356aac39f89d0667145fab16b"].into(),
		// 5GCJ3HKN5MaseCwqwNJ4pUbpJqRbfAmZXWB8SCMJM6FMyM9B
		hex!["b6bab8caa7be249400b5062d17908c59c0e40dcbe2bd1c818098a5dae916a869"].unchecked_into(),
		// 5HPtGdWoRmiReRYE16AQitm4MG8s47ngfHLGUeKHZuo1Cdry
		hex!["ebcde238597379c874dd51fcca5e0f651972b218c46aa21c471167474e089c86"].unchecked_into(),
		// 5HPtGdWoRmiReRYE16AQitm4MG8s47ngfHLGUeKHZuo1Cdry
		hex!["ebcde238597379c874dd51fcca5e0f651972b218c46aa21c471167474e089c86"].unchecked_into(),
		// 5HPtGdWoRmiReRYE16AQitm4MG8s47ngfHLGUeKHZuo1Cdry
		hex!["ebcde238597379c874dd51fcca5e0f651972b218c46aa21c471167474e089c86"].unchecked_into(),
	), (
		// 5GH23iTUJtvz9KGDK36nXWHtrkm6E83ZZjVPFhb8DKQk3cv3
		hex!["ba551cfbf9e91da337f21658276dfbd56ba43be852395db10a89a64e07978f31"].into(),
		// 5HQwfPfmbuWt3fKyEK3SSuDneVtF4MwHbK1afsXPHxAfogyj
		hex!["ec9c8c8d80eab0b1fc4733e25af31137fb656390c595bb1c7536f73b201ede57"].into(),
		// 5GokhX8Ce1VrMaWFt5RMdAq2EkzoBxdUerFoMzRLDYNPyS2M
		hex!["d1c60ddadc9a3f65da208c5c50e7fc9ed0ab069e79553d08dcc36a401948fa1a"].unchecked_into(),
		// 5Ec2hh96mXEavdu2C866hgUC4joBYpBVujXQJgQsDWfUMmM1
		hex!["705c8360296c7b6af2f842e7a0804492c86a855aaa605fdf419f577f1f4fecbe"].unchecked_into(),
		// 5Ec2hh96mXEavdu2C866hgUC4joBYpBVujXQJgQsDWfUMmM1
		hex!["705c8360296c7b6af2f842e7a0804492c86a855aaa605fdf419f577f1f4fecbe"].unchecked_into(),
		// 5Ec2hh96mXEavdu2C866hgUC4joBYpBVujXQJgQsDWfUMmM1
		hex!["705c8360296c7b6af2f842e7a0804492c86a855aaa605fdf419f577f1f4fecbe"].unchecked_into(),
	)];

	testnet_genesis(
		initial_authorities,
		hex!["f04eaed79cba531626964ba59d727b670524247c92cdd0b5f5da04c8eccb796b"].into(),
		None,
		true,
		balances,
		vesting,
	)
}

/// Edgeware config (8 validators)
pub fn edgeware_testnet_config() -> ChainSpec {
	let data = r#"
		{
			"tokenDecimals": 18,
			"tokenSymbol": "EDG"
		}"#;
	let properties = serde_json::from_str(data).unwrap();
	let boot_nodes = vec![
		"/ip4/45.77.78.68/tcp/30333/p2p/QmVxmq3EtJa7NWae6hQv3PqBNud2UhubUWVXjVewtGeciK".to_string(),
		"/ip4/45.76.17.97/tcp/30333/p2p/QmSpWQuAmrFgiygF2DZd3bzyHCWuzYcFXmLpzcph5mnTu9".to_string(),
		"/ip4/45.77.93.189/tcp/30333/p2p/QmVJfKRBVPcZHnaqfGyRym3aTXZMtrmNn8XYGsowoxUhr9".to_string(),
		"/ip4/108.61.209.73/tcp/30333/p2p/QmQFddsCpMh3rF7xTWuqWAcWy1pqS4bvShTRVe9zbYJQRh".to_string(),
		"/ip4/45.76.248.131/tcp/30333/p2p/QmeDZ3uK2aSv738r7trsQrpGt7HGgDLL3JqTY2SGHxN7qC".to_string(),
		"/ip4/96.30.192.236/tcp/30333/p2p/QmNkioYhmd26XqzHRmRWCnvd5NGoMNKYh9fgxvAy9Zf7mS".to_string(),
	];
	ChainSpec::from_genesis(
		"Edgeware Testnet",
		"edgeware_testnet",
		edgeware_testnet_config_genesis,
		boot_nodes,
		Some(TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])),
		Some(DEFAULT_PROTOCOL_ID),
		None,
		properties,
	)
}

fn development_config_genesis() -> GenesisConfig {
	testnet_genesis(
		vec![
			get_authority_keys_from_seed("Alice"),
		],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
		true,
		vec![],
		vec![],
	)
}

/// Development config (single validator Alice)
pub fn development_config() -> ChainSpec {
	ChainSpec::from_genesis(
		"Development",
		"dev",
		development_config_genesis,
		vec![],
		None,
		None,
		None,
		Default::default(),
	)
}

fn local_testnet_genesis() -> GenesisConfig {
	testnet_genesis(
		vec![
			get_authority_keys_from_seed("Alice"),
			get_authority_keys_from_seed("Bob"),
		],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
		false,
		vec![],
		vec![],
	)
}

/// Local testnet config (multivalidator Alice + Bob)
pub fn local_testnet_config() -> ChainSpec {
	ChainSpec::from_genesis(
		"Local Testnet",
		"local_testnet",
		local_testnet_genesis,
		vec![],
		None,
		None,
		None,
		Default::default(),
	)
}

#[cfg(test)]
pub(crate) mod tests {
	use super::*;
	use crate::service::new_full;
	use substrate_service::Roles;
	use service_test;

	fn local_testnet_genesis_instant_single() -> GenesisConfig {
		testnet_genesis(
			vec![
				get_authority_keys_from_seed("Alice"),
			],
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			None,
			false,
			vec![],
			vec![],
		)
	}

	/// Local testnet config (single validator - Alice)
	pub fn integration_test_config_with_single_authority() -> ChainSpec {
		ChainSpec::from_genesis(
			"Integration Test",
			"test",
			local_testnet_genesis_instant_single,
			vec![],
			None,
			None,
			None,
			Default::default(),
		)
	}

	/// Local testnet config (multivalidator Alice + Bob)
	pub fn integration_test_config_with_two_authorities() -> ChainSpec {
		ChainSpec::from_genesis(
			"Integration Test",
			"test",
			local_testnet_genesis,
			vec![],
			None,
			None,
			None,
			Default::default(),
		)
	}

	#[test]
	#[ignore]
	fn test_connectivity() {
		service_test::connectivity(
			integration_test_config_with_two_authorities(),
			|config| new_full(config),
			|mut config| {
				// light nodes are unsupported
				config.roles = Roles::FULL;
				new_full(config)
			},
			true,
		);
	}
}
