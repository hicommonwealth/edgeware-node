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

use edgeware_beresheet_runtime::constants::currency::*;
use edgeware_beresheet_runtime::{
	AuraConfig, AuthorityDiscoveryConfig, BalancesConfig, CouncilConfig,
	DemocracyConfig, GrandpaConfig, ImOnlineConfig, IndicesConfig, SessionConfig,
	SessionKeys, SignalingConfig, StakerStatus, StakingConfig, SudoConfig, SystemConfig,
	TreasuryRewardConfig, VestingConfig, wasm_binary_unwrap, EVMConfig,
};
use pallet_im_online::ed25519::AuthorityId as ImOnlineId;
use sc_service::ChainType;
use sc_telemetry::TelemetryEndpoints;
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_aura::ed25519::AuthorityId as AuraId;
use sp_core::{sr25519, Pair, Public, U256, H160,};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::{
	traits::{IdentifyAccount, One, Verify},
	Perbill,
};
use std::collections::BTreeMap;
use edgeware_primitives::{AccountId, Balance, BlockNumber, Signature};
use edgeware_beresheet_runtime::constants::time::*;
use edgeware_beresheet_runtime::GenesisConfig;

/// Specialized `ChainSpec`.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;

/// Beresheet Testnet configuration
pub fn edgeware_beresheet_official() -> ChainSpec {
	match ChainSpec::from_json_bytes(&include_bytes!("../res/beresheet.chainspec.json")[..]) {
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
	SessionKeys {
		grandpa,
		aura,
		im_online,
		authority_discovery,
	}
}

/// Helper function to create GenesisConfig for testing
pub fn testnet_genesis(
	initial_authorities: Vec<(AccountId, AccountId, GrandpaId, AuraId, ImOnlineId, AuthorityDiscoveryId)>,
	_root_key: AccountId,
	endowed_accounts: Option<Vec<AccountId>>,
	_enable_println: bool,
	balances: Vec<(AccountId, Balance)>,
	vesting: Vec<(AccountId, BlockNumber, BlockNumber, Balance)>,
	founder_allocation: Vec<(AccountId, Balance)>,
) -> GenesisConfig {
	let alice_evm_account_id = H160::from_str("19e7e376e7c213b7e7e7e46cc70a5dd086daff2a").unwrap();
	let mut evm_accounts = BTreeMap::new();
	evm_accounts.insert(
		alice_evm_account_id,
		pallet_evm::GenesisAccount {
			nonce: 0.into(),
			balance: U256::from(123456_123_000_000_000_000_000u128),
			storage: BTreeMap::new(),
			code: vec![],
		},
	);

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

	const STASH: Balance = 100000000 * DOLLARS;

	GenesisConfig {
		frame_system: Some(SystemConfig {
			code: wasm_binary_unwrap().to_vec(),
			changes_trie_config: Default::default(),
		}),
		pallet_balances: Some(BalancesConfig {
			balances: endowed_accounts
				.iter()
				.cloned()
				.map(|k| (k, STASH))
				.chain(
					founder_allocation
						.iter()
						.map(|x| (x.0.clone(), x.1.clone())),
				)
				.chain(initial_authorities.iter().map(|x| (x.0.clone(), STASH)))
				.chain(initial_authorities.iter().map(|x| (x.1.clone(), STASH)))
				.chain(balances.clone())
				.collect(),
		}),
		pallet_indices: Some(IndicesConfig { indices: vec![] }),
		pallet_session: Some(SessionConfig {
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
		}),
		pallet_staking: Some(StakingConfig {
			validator_count: 20,
			minimum_validator_count: initial_authorities.len() as u32,
			stakers: initial_authorities
				.iter()
				.map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator))
				.collect(),
			invulnerables: [].to_vec(),
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		}),
		pallet_democracy: Some(DemocracyConfig::default()),
		pallet_collective_Instance1: Some(CouncilConfig {
			members: crate::testnet_fixtures::get_testnet_election_members(),
			phantom: Default::default(),
		}),
		pallet_aura: Some(AuraConfig {
			authorities: vec![],
		}),
		pallet_im_online: Some(ImOnlineConfig { keys: vec![] }),
		pallet_authority_discovery: Some(AuthorityDiscoveryConfig { keys: vec![] }),
		pallet_grandpa: Some(GrandpaConfig {
			authorities: vec![],
		}),
		pallet_treasury: Some(Default::default()),
		pallet_elections_phragmen: Some(Default::default()),
		pallet_sudo: Some(SudoConfig { key: _root_key }),
		pallet_vesting: Some(VestingConfig { vesting: vesting }),
		pallet_evm: Some(EVMConfig { accounts: evm_accounts }),
		pallet_contracts: Some(Default::default()),
		pallet_ethereum: Some(Default::default()),
		signaling: Some(SignalingConfig {
			voting_length: 7 * DAYS,
			proposal_creation_bond: 1 * DOLLARS,
		}),
		treasury_reward: Some(TreasuryRewardConfig {
			current_payout: 95 * DOLLARS,
			minting_interval: One::one(),
		}),
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

	let initial_authorities = crate::testnet_fixtures::get_mtestnet_initial_authorities();

	testnet_genesis(
		initial_authorities,
		crate::testnet_fixtures::get_testnet_root_key(),
		None,
		true,
		balances,
		vesting,
		crate::mainnet_fixtures::get_commonwealth_allocation(),
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
	let boot_nodes = crate::testnet_fixtures::get_mtestnet_bootnodes();

	ChainSpec::from_genesis(
		&testnet_name,
		&testnet_node_name,
		ChainType::Development,
		edgeware_testnet_config_genesis,
		boot_nodes,
		Some(
			TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])
				.expect("Staging telemetry url is valid; qed"),
		),
		Some(DEFAULT_PROTOCOL_ID),
		properties,
		Default::default(),
	)
}
