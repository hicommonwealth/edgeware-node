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

//! Substrate chain configurations.

use edgeware_runtime::{
	constants::currency::*, wasm_binary_unwrap, AuraConfig, AuthorityDiscoveryConfig, BalancesConfig, Block,
	CouncilConfig, DemocracyConfig, EVMConfig, GrandpaConfig, ImOnlineConfig, IndicesConfig, SessionConfig,
	SessionKeys, StakerStatus, StakingConfig, SudoConfig, SystemConfig, TreasuryRewardConfig, VestingConfig,
};
use pallet_im_online::ed25519::AuthorityId as ImOnlineId;
use sc_chain_spec::ChainSpecExtension;
use sc_service::ChainType;
use sc_telemetry::TelemetryEndpoints;
use serde::{Deserialize, Serialize};
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_aura::ed25519::AuthorityId as AuraId;
use sp_core::{sr25519, Pair, Public, H160};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::{
	traits::{IdentifyAccount, One, Verify},
	Perbill,
};
pub use edgeware_primitives::{AccountId, Balance, BlockNumber, Signature};
pub use edgeware_runtime::{constants::time::*, GenesisConfig};
use fp_evm::GenesisAccount;

use hex::FromHex;
use serde_json::Result;
use std::{collections::BTreeMap, fs::File, io::Read, path::Path, str::FromStr};

type AccountPublic = <Signature as Verify>::Signer;

const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
const DEFAULT_PROTOCOL_ID: &str = "edg";

#[derive(Serialize, Deserialize, Debug)]
struct Allocation {
	balances: Vec<(String, String)>,
	vesting: Vec<(String, BlockNumber, BlockNumber, String)>,
}

fn get_lockdrop_participants_allocation() -> Result<Allocation> {
	let path = Path::new("node/cli/lockdrop/lockdrop-allocation.json");
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
#[serde(rename_all = "camelCase")]
pub struct Extensions {
	/// Block numbers with known hashes.
	pub fork_blocks: sc_client_api::ForkBlocks<Block>,
	/// Known bad block hashes.
	pub bad_blocks: sc_client_api::BadBlocks<Block>,
}

/// Specialized `ChainSpec`.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;

/// Mainnet configuration
pub fn edgeware_mainnet_official() -> ChainSpec {
	match ChainSpec::from_json_bytes(&include_bytes!("../res/mainnet.chainspec.raw.json")[..]) {
		Ok(spec) => spec,
		Err(e) => panic!("{}", e),
	}
}

/// Beresheet Testnet configuration for runtime spec 46 (fast node)
pub fn edgeware_beresheet_v46_fast() -> ChainSpec {
	match ChainSpec::from_json_bytes(&include_bytes!("../res/beresheet_v46_fast.raw.json")[..]) {
		Ok(spec) => spec,
		Err(e) => panic!("{}", e),
	}
}

/// Beresheet Testnet configuration for runtime spec 46
pub fn edgeware_beresheet_v46() -> ChainSpec {
	match ChainSpec::from_json_bytes(&include_bytes!("../res/beresheet_v46.raw.json")[..]) {
		Ok(spec) => spec,
		Err(e) => panic!("{}", e),
	}
}

/// Beresheet Testnet configuration
pub fn edgeware_beresheet_official() -> ChainSpec {
	match ChainSpec::from_json_bytes(&include_bytes!("../res/beresheet.chainspec.json")[..]) {
		Ok(spec) => spec,
		Err(e) => panic!("{}", e),
	}
}

fn session_keys(
	grandpa: GrandpaId,
	aura: AuraId,
	im_online: ImOnlineId,
	authority_discovery: AuthorityDiscoveryId,
) -> SessionKeys {
	SessionKeys {
		grandpa,
		aura,
		im_online,
		authority_discovery,
	}
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed
pub fn get_authority_keys_from_seed(
	seed: &str,
) -> (
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
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		GrandpaId,
		AuraId,
		ImOnlineId,
		AuthorityDiscoveryId,
	)>,
	root_key: AccountId,
	endowed_accounts: Option<Vec<AccountId>>,
	_enable_println: bool,
	_balances: Vec<(AccountId, Balance)>,
	vesting: Vec<(AccountId, BlockNumber, BlockNumber, Balance)>,
	_founder_allocation: Vec<(AccountId, Balance)>,
	create_evm_alice: bool,
) -> GenesisConfig {
	let alice_evm_account_id = H160::from_str("19e7e376e7c213b7e7e7e46cc70a5dd086daff2a").unwrap();
	let mut evm_accounts = BTreeMap::new();
	if create_evm_alice {
		evm_accounts.insert(alice_evm_account_id, GenesisAccount {
			balance: ethereum_types::U256::from(123456_123_000_000_000_000_000u128),
			code: Default::default(),
			nonce: Default::default(),
			storage: Default::default(),
		});
		evm_accounts.insert(
			// H160 address of Alice dev account
			// Derived from SS58 (42 prefix) address
			// SS58: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
			// hex: 0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
			// Using the full hex key, truncating to the first 20 bytes (the first 40 hex chars)
			H160::from_str("d43593c715fdd31c61141abd04a99fd6822c8558")
				.expect("internal H160 is valid; qed"),
			GenesisAccount {
				balance: ethereum_types::U256::from_str("0xffffffffffffffffffffffffffffffff")
					.expect("internal U256 is valid; qed"),
				code: Default::default(),
				nonce: Default::default(),
				storage: Default::default(),
			},
		);
	}
	let mut endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
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
	if !endowed_accounts.contains(&root_key) {
		endowed_accounts.push(root_key.clone());
	}

	initial_authorities.iter().for_each(|x| {
		if !endowed_accounts.contains(&x.0) {
			endowed_accounts.push(x.0.clone());
		}
		if !endowed_accounts.contains(&x.1) {
			endowed_accounts.push(x.1.clone());
		}
	});

	const STASH: Balance = 100000000 * DOLLARS;
	let endowed_balances: Vec<(AccountId, Balance)> = endowed_accounts.iter().map(|k| (k.clone(), STASH)).collect();

	GenesisConfig {
		system: SystemConfig {
			code: wasm_binary_unwrap().to_vec(),
			// changes_trie_config: Default::default(),
		},
		balances: BalancesConfig {
			balances: endowed_balances,
		},
		indices: IndicesConfig { indices: vec![] },
		session: SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						session_keys(x.2.clone(), x.3.clone(), x.4.clone(), x.5.clone()),
					)
				})
				.collect::<Vec<_>>(),
		},
		staking: StakingConfig {
			validator_count: 7,
			minimum_validator_count: initial_authorities.len() as u32,
			stakers: initial_authorities
				.iter()
				.map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator))
				.collect(),
			invulnerables: [].to_vec(),
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		},
		democracy: DemocracyConfig::default(),
		council: Default::default(),
		aura: AuraConfig { authorities: vec![] },
		im_online: ImOnlineConfig { keys: vec![] },
		authority_discovery: AuthorityDiscoveryConfig { keys: vec![] },
		grandpa: GrandpaConfig { authorities: vec![] },
		treasury: Default::default(),
		phragmen_election: Default::default(),
		sudo: SudoConfig { key: Some(root_key) },
		vesting: VestingConfig { vesting },
		assets: Default::default(),
		transaction_payment: Default::default(),
		dynamic_fee: Default::default(),
		ethereum: Default::default(),
		base_fee: Default::default(),
		evm: EVMConfig { accounts: evm_accounts },
		treasury_reward: TreasuryRewardConfig {
			current_payout: 95 * DOLLARS,
			minting_interval: One::one(),
		},
	}
}

fn edgeware_testnet_config_genesis() -> GenesisConfig {
	let allocation = get_lockdrop_participants_allocation().unwrap();
	let balances = allocation
		.balances
		.iter()
		.map(|b| {
			let balance = b.1.to_string().parse::<Balance>().unwrap();
			return (<[u8; 32]>::from_hex(b.0.clone()).unwrap().into(), balance);
		})
		.filter(|b| b.1 > 0)
		.collect();

	let initial_authorities = crate::testnet_fixtures::get_beresheet_initial_authorities();

	testnet_genesis(
		initial_authorities,
		crate::testnet_fixtures::get_testnet_root_key(),
		Some(vec![]),
		true,
		balances,
		vec![],
		crate::mainnet_fixtures::get_commonwealth_allocation(),
		false,
	)
}

/// Edgeware config (8 validators)
pub fn edgeware_testnet_config(testnet_name: String, testnet_node_name: String) -> ChainSpec {
	let data = r#"
		{
			"ss58Format": 42,
			"tokenDecimals": 18,
			"tokenSymbol": "tEDG"
		}"#;
	let properties = serde_json::from_str(data).unwrap();
	let boot_nodes = crate::testnet_fixtures::get_beresheet_bootnodes();

	ChainSpec::from_genesis(
		&testnet_name, //name
		&testnet_node_name, // id
		ChainType::Development,
		edgeware_testnet_config_genesis,
		boot_nodes,
		Some(
			TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])
				.expect("Staging telemetry url is valid; qed"),
		),
		Some(DEFAULT_PROTOCOL_ID),
		Default::default(),
		properties,
		Default::default(),
	)
}

/// Development config with 6 node validator set
fn multi_development_config_genesis() -> GenesisConfig {
	testnet_genesis(
		vec![
			get_authority_keys_from_seed("Alice"),
			get_authority_keys_from_seed("Bob"),
			get_authority_keys_from_seed("Charlie"),
			get_authority_keys_from_seed("Dave"),
			get_authority_keys_from_seed("Eve"),
			get_authority_keys_from_seed("Ferdie"),
		],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
		true,
		vec![],
		vec![],
		vec![],
		true,
	)
}

/// Development config with single node validator set
pub fn development_config_genesis() -> GenesisConfig {
	testnet_genesis(
		vec![get_authority_keys_from_seed("Alice")],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
		true,
		vec![],
		vec![],
		vec![],
		true,
	)
}

/// Development config (single validator Alice)
pub fn development_config() -> ChainSpec {
	let data = r#"
		{
			"ss58Format": 42,
			"tokenDecimals": 18,
			"tokenSymbol": "tEDG"
		}"#;
	let properties = serde_json::from_str(data).unwrap();

	ChainSpec::from_genesis(
		"Development",
		"dev",
		ChainType::Development,
		development_config_genesis,
		vec![],
		None,
		None,
		Default::default(),
		properties,
		Default::default(),
	)
}

/// Development config (6 validators Alice, Bob, Charlie, Dave, Eve, Ferdie)
pub fn multi_development_config() -> ChainSpec {
	let data = r#"
		{
			"ss58Format": 42,
			"tokenDecimals": 18,
			"tokenSymbol": "tEDG"
		}"#;
	let properties = serde_json::from_str(data).unwrap();
	ChainSpec::from_genesis(
		"Multi Development",
		"multi-dev",
		ChainType::Development,
		multi_development_config_genesis,
		vec![],
		None,
		None,
		Default::default(),
		properties,
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
		vec![],
		true,
	)
}

/// Local testnet config (multivalidator Alice + Bob)
pub fn local_testnet_config() -> ChainSpec {
	ChainSpec::from_genesis(
		"Local Testnet",
		"local_testnet",
		ChainType::Development,
		local_testnet_genesis,
		vec![],
		None,
		None,
		Default::default(),
		None,
		Default::default(),
	)
}

/// Helper function to create GenesisConfig for testing
pub fn mainnet_genesis(
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		Balance,
		AuraId,
		GrandpaId,
		ImOnlineId,
		AuthorityDiscoveryId,
	)>,
	founder_allocation: Vec<(AccountId, Balance)>,
	balances: Vec<(AccountId, Balance)>,
	vesting: Vec<(AccountId, BlockNumber, BlockNumber, Balance)>,
) -> GenesisConfig {
	GenesisConfig {
		system: SystemConfig {
			code: wasm_binary_unwrap().to_vec(),
			//changes_trie_config: Default::default(),
		},
		balances: BalancesConfig {
			balances: founder_allocation
				.iter()
				.map(|x| (x.0.clone(), x.1.clone()))
				.chain(balances.clone())
				.collect(),
		},
		indices: IndicesConfig { indices: vec![] },
		session: SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						session_keys(x.4.clone(), x.3.clone(), x.5.clone(), x.6.clone()),
					)
				})
				.collect::<Vec<_>>(),
		},
		staking: StakingConfig {
			validator_count: 60,
			minimum_validator_count: initial_authorities.len() as u32,
			stakers: initial_authorities
				.iter()
				.map(|x| (x.0.clone(), x.1.clone(), x.2.clone(), StakerStatus::Validator))
				.collect(),
			invulnerables: vec![],
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		},
		democracy: DemocracyConfig::default(),
		council: CouncilConfig {
			members: crate::mainnet_fixtures::get_mainnet_election_members(),
			phantom: Default::default(),
		},
		aura: AuraConfig { authorities: vec![] },
		im_online: ImOnlineConfig { keys: vec![] },
		authority_discovery: AuthorityDiscoveryConfig { keys: vec![] },
		grandpa: GrandpaConfig { authorities: vec![] },
		treasury: Default::default(),
		phragmen_election: Default::default(),
		sudo: SudoConfig {
			key: Some(crate::mainnet_fixtures::get_mainnet_root_key()),
		},
		vesting: VestingConfig { vesting },
		assets: Default::default(),
		transaction_payment: Default::default(),
		dynamic_fee: Default::default(),
		ethereum: Default::default(),
		base_fee: Default::default(),
		evm: Default::default(),
		treasury_reward: TreasuryRewardConfig {
			current_payout: 95 * DOLLARS,
			minting_interval: One::one(),
		},
	}
}

/// Mainnet config
fn edgeware_mainnet_config_genesis() -> GenesisConfig {
	let allocation = get_lockdrop_participants_allocation().unwrap();
	let balances = allocation
		.balances
		.iter()
		.map(|b| {
			let balance = b.1.to_string().parse::<Balance>().unwrap();
			return (<[u8; 32]>::from_hex(b.0.clone()).unwrap().into(), balance);
		})
		.filter(|b| b.1 > 0)
		.collect();
	let vesting = allocation
		.vesting
		.iter()
		.map(|b| {
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

	mainnet_genesis(
		crate::mainnet_fixtures::get_cw_mainnet_validators(),
		crate::mainnet_fixtures::get_commonwealth_allocation(),
		balances,
		vesting,
	)
}

/// Edgeware config (8 validators)
pub fn edgeware_mainnet_config() -> ChainSpec {
	let data = r#"
		{
			"ss58Format": 7,
			"tokenDecimals": 18,
			"tokenSymbol": "EDG"
		}"#;
	let properties = serde_json::from_str(data).unwrap();
	let boot_nodes = crate::mainnet_fixtures::get_mainnet_bootnodes();
	ChainSpec::from_genesis(
		"Edgeware",// name
		"edgeware", // chain id
		ChainType::Live, //chain type
		edgeware_mainnet_config_genesis, //constructor
		boot_nodes, //boot noodes Vec<MultiaddrWithPeerId>
		Some(
			TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])
				.expect("Staging telemetry url is valid; qed"),
		), // telemetry in Option
		Some(DEFAULT_PROTOCOL_ID), //protocol_id
		Default::default(), // fork_id
		properties,//Properties
		Default::default(), //extensions
	)
}
