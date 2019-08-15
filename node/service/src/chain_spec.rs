// Copyright 2018 Commonwealth Labs, Inc.
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
// along with Edgeware.  If not, see <http://www.gnu.org/licenses/>

use primitives::{ed25519, sr25519, Pair};
use edgeware_primitives::{AccountId, AuraId, Balance};
use edgeware_runtime::{
	GrandpaConfig, BalancesConfig, ContractsConfig, ElectionsConfig, DemocracyConfig, CouncilConfig,
	AuraConfig, IndicesConfig, SessionConfig, StakingConfig, SudoConfig, TreasuryRewardConfig,
	SystemConfig, ImOnlineConfig, WASM_BINARY, Perbill, SessionKeys, StakerStatus,
};
use edgeware_runtime::constants::{time::DAYS, currency::DOLLARS, currency::MILLICENTS};
use edgeware_runtime::{IdentityConfig, SignalingConfig};
pub use edgeware_runtime::GenesisConfig;
use substrate_service;
use substrate_telemetry::TelemetryEndpoints;
use grandpa::AuthorityId as GrandpaId;
use crate::fixtures::*;
use sr_primitives::{
	traits::{One},
};

const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
const DEFAULT_PROTOCOL_ID: &str = "edg";

/// Specialised `ChainSpec`.
pub type ChainSpec = substrate_service::ChainSpec<GenesisConfig>;

pub fn edgeware_config() -> ChainSpec {
	match ChainSpec::from_json_file(std::path::PathBuf::from("testnets/v0.6.0/edgeware.json")) {
		Ok(spec) => spec,
		Err(e) => panic!(e),
	}
}

pub fn edgeware_testnet_config_gensis() -> GenesisConfig {
	let commonwealth_authorities: Vec<(AccountId, AccountId, AuraId, Balance)> = get_commonwealth_validators();
	let grandpa_nodes = get_grandpa_nodes();
	let spec = get_spec_allocation().unwrap();
	let lockdrop_balances = spec.0;
	let lockdrop_vesting = spec.1;
	let lockdrop_validators = spec.2;
	let root_key = get_root_key();
	// Add controller accounts to endowed accounts
	let endowed_accounts = get_more_endowed();
	let identity_verifiers = get_identity_verifiers();
	const ENDOWMENT: Balance = 10 * DOLLARS;
	GenesisConfig {
		system: Some(SystemConfig {
			code: WASM_BINARY.to_vec(),
			changes_trie_config: Default::default(),
		}),
		balances: Some(BalancesConfig {
			balances: endowed_accounts.iter().cloned()
				.map(|k| (k, ENDOWMENT))
				// give authorities their balances
				.chain(commonwealth_authorities.iter().map(|x| (x.0.clone(), x.3.clone())))
				// give controllers an endowment
				.chain(commonwealth_authorities.iter().map(|x| (x.1.clone(), ENDOWMENT)))
				// give lockdropers their balances
				.chain(lockdrop_balances.iter().map(|x| (x.0.clone(), x.1.clone())))
				.collect(),
			vesting: lockdrop_vesting,
		}),
		indices: Some(IndicesConfig {
			ids: endowed_accounts.iter().cloned()
				.chain(commonwealth_authorities.iter().map(|x| x.0.clone()))
				.chain(commonwealth_authorities.iter().map(|x| x.1.clone()))
				.chain(lockdrop_balances.iter().map(|x| x.0.clone()))
				.collect::<Vec<_>>(),
		}),
		session: Some(SessionConfig {
			keys: commonwealth_authorities.iter().map(|x| (x.0.clone(), session_keys(x.2.clone())))
				.chain(lockdrop_validators.iter().map(|x| (x.0.clone(), session_keys(x.2.clone()))))
				.collect::<Vec<_>>(),
		}),
		staking: Some(StakingConfig {
			current_era: 0,
			offline_slash: Perbill::from_parts(1_000_000),
			validator_count: 100,
			offline_slash_grace: 10,
			minimum_validator_count: 10,
			stakers: commonwealth_authorities.iter().map(|x| (x.0.clone(), x.1.clone(), x.3.clone(), StakerStatus::Validator))
				.chain(lockdrop_validators.iter().map(|x| (x.0.clone(), x.1.clone(), x.3.clone(), StakerStatus::Validator)))
				.collect(),
			invulnerables: commonwealth_authorities.iter().map(|x| x.0.clone())
				.chain(lockdrop_validators.iter().map(|x| x.0.clone()))
				.collect(),
		}),
		democracy: Some(DemocracyConfig::default()),
		collective_Instance1: Some(CouncilConfig {
			members: commonwealth_authorities.iter().map(|x| x.1.clone())
				.chain(endowed_accounts.iter().map(|x| x.clone()))
				.collect(),
			phantom: Default::default(),
		}),
		elections: Some(ElectionsConfig {
			members: commonwealth_authorities.iter().map(|x| (x.1.clone(), 1000000))
				.chain(endowed_accounts.iter().map(|x| (x.clone(), 1000000)))
				.collect(),
			desired_seats: 13,
			presentation_duration: 2 * DAYS,
			term_duration: 3 * DAYS,
		}),
		contracts: Some(ContractsConfig {
			current_schedule: Default::default(),
			gas_price: 1 * MILLICENTS,
		}),
		sudo: Some(SudoConfig {
			key: root_key,
		}),
		im_online: Some(ImOnlineConfig {
			gossip_at: 0,
			last_new_era_start: 0,
		}),
		aura: Some(AuraConfig {
			authorities: commonwealth_authorities.iter().map(|x| x.2.clone())
				.chain(lockdrop_validators.iter().map(|x| x.2.clone()))
				.collect(),
		}),
		grandpa: Some(GrandpaConfig {
			authorities: grandpa_nodes.iter().map(|x| (x.clone(), 1)).collect(),
		}),
		identity: Some(IdentityConfig {
			verifiers: identity_verifiers,
			expiration_length: 1 * DAYS, // 1 days
			registration_bond: 1 * DOLLARS,
		}),
		signaling: Some(SignalingConfig {
			voting_length: 3 * DAYS, // 7 days
			proposal_creation_bond: 100 * DOLLARS,
		}),
		treasury_reward: Some(TreasuryRewardConfig {
			minting_interval: One::one(),
		}),
	}
}

/// Edgeware testnet generator
pub fn edgeware_testnet_config() -> Result<ChainSpec, String> {
	let boot_nodes = vec![
		"/ip4/157.230.218.41/tcp/30333/p2p/QmNYiKrVuztYuL42gs5kHLTqvKsmEnE3GvJQ8ewcvwtSVF".to_string(),
		"/ip4/18.223.143.102/tcp/30333/p2p/QmdHoon1jbjeJfTdifknGefGrJHUNYgDDpnJBLLW1Pdt13".to_string(),
		"/ip4/206.189.33.216/tcp/30333/p2p/QmNc7rakvWY1QL6LL9ssTfKTWUhHUfzMvygYdyMLpLQCR7".to_string(),
		"/ip4/157.230.125.18/tcp/30333/p2p/QmTqM3sPbeaE7R2WaveJNg1Ma86dSFPTBHXxYSNjwcii1x".to_string(),
	];

	Ok(ChainSpec::from_genesis(
		"Edgeware Testnet",
		"edgeware_testnet",
		edgeware_testnet_config_gensis,
		boot_nodes,
		Some(TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])),
		Some(DEFAULT_PROTOCOL_ID),
		None,
		None
	))
}

fn session_keys(key: ed25519::Public) -> SessionKeys {
	SessionKeys { ed25519: key }
}

/// Helper function to generate AccountId from seed
pub fn get_account_id_from_seed(seed: &str) -> AccountId {
	sr25519::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Helper function to generate AuraId from seed
pub fn get_aura_id_from_seed(seed: &str) -> AuraId {
	ed25519::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Helper function to generate GrandpaId from seed
pub fn get_grandpa_id_from_seed(seed: &str) -> GrandpaId {
	ed25519::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Helper function to generate stash, controller and session key from seed
pub fn get_authority_keys_from_seed(seed: &str) -> (AccountId, AccountId, AuraId, GrandpaId) {
	(
		get_account_id_from_seed(&format!("{}//stash", seed)),
		get_account_id_from_seed(seed),
		get_aura_id_from_seed(seed),
		get_grandpa_id_from_seed(seed)
	)
}

/// Helper function to create GenesisConfig for testing
pub fn development_genesis(
	initial_authorities: Vec<(AccountId, AccountId, AuraId, GrandpaId)>,
	root_key: AccountId,
	endowed_accounts: Option<Vec<AccountId>>,
	initial_verifiers: Option<Vec<AccountId>>,
) -> GenesisConfig {
	let initial_verifiers: Vec<AccountId> = initial_verifiers.unwrap_or_else(|| {
		vec![
			get_account_id_from_seed("Alice"),
			get_account_id_from_seed("Bob"),
		]
	});

	let endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
		vec![
			get_account_id_from_seed("Alice"),
			get_account_id_from_seed("Bob"),
			get_account_id_from_seed("Charlie"),
			get_account_id_from_seed("Dave"),
			get_account_id_from_seed("Eve"),
			get_account_id_from_seed("Ferdie"),
			get_account_id_from_seed("Alice//stash"),
			get_account_id_from_seed("Bob//stash"),
			get_account_id_from_seed("Charlie//stash"),
			get_account_id_from_seed("Dave//stash"),
			get_account_id_from_seed("Eve//stash"),
			get_account_id_from_seed("Ferdie//stash"),
		]
	});

	const ENDOWMENT: Balance = 10_000_000 * DOLLARS;
	const STASH: Balance = 100 * DOLLARS;
	let desired_seats: u32 = (endowed_accounts.len() / 2 - initial_authorities.len()) as u32;

	GenesisConfig {
		system: Some(SystemConfig {
			code: WASM_BINARY.to_vec(),
			changes_trie_config: Default::default(),
		}),
		balances: Some(BalancesConfig {
			balances: endowed_accounts.iter().cloned()
				.map(|k| (k, ENDOWMENT))
				.chain(initial_authorities.iter().map(|x| (x.0.clone(), STASH)))
				.collect(),
			vesting: vec![],
		}),
		indices: Some(IndicesConfig {
			ids: endowed_accounts.iter().cloned()
				.chain(initial_authorities.iter().map(|x| x.0.clone()))
				.collect::<Vec<_>>(),
		}),
		session: Some(SessionConfig {
			keys: initial_authorities.iter().map(|x| (x.0.clone(), session_keys(x.2.clone()))).collect::<Vec<_>>(),
		}),
		staking: Some(StakingConfig {
			current_era: 0,
			offline_slash: Perbill::from_parts(1_000_000),
			validator_count: 7,
			offline_slash_grace: 4,
			minimum_validator_count: 4,
			stakers: initial_authorities.iter().map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator)).collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
		}),
		democracy: Some(DemocracyConfig::default()),
		collective_Instance1: Some(CouncilConfig {
			members: initial_authorities.iter().map(|x| x.1.clone()).collect(),
			phantom: Default::default(),
		}),
		elections: Some(ElectionsConfig {
			members: initial_authorities.iter().map(|x| (x.1.clone(), 1000000)).collect(),
			desired_seats: desired_seats,
			presentation_duration: 1 * DAYS,
			term_duration: 28 * DAYS,
		}),
		contracts: Some(ContractsConfig {
			current_schedule: Default::default(),
			gas_price: 1 * MILLICENTS,
		}),
		sudo: Some(SudoConfig {
			key: root_key,
		}),
		im_online: Some(ImOnlineConfig {
			gossip_at: 0,
			last_new_era_start: 0,
		}),
		aura: Some(AuraConfig {
			authorities: initial_authorities.iter().map(|x| x.2.clone()).collect(),
		}),
		grandpa: Some(GrandpaConfig {
			authorities: initial_authorities.iter().map(|x| (x.3.clone(), 1)).collect(),
		}),
		identity: Some(IdentityConfig {
			verifiers: initial_verifiers,
			expiration_length: 7 * DAYS, // 7 days
			registration_bond: 1 * DOLLARS,
		}),
		signaling: Some(SignalingConfig {
			voting_length: 7 * DAYS, // 7 days
			proposal_creation_bond: 1 * DOLLARS,
		}),
		treasury_reward: Some(TreasuryRewardConfig {
			minting_interval: One::one(),
		}),
	}
}

fn development_config_genesis() -> GenesisConfig {
	development_genesis(
		vec![
			get_authority_keys_from_seed("Alice"),
		],
		get_account_id_from_seed("Alice"),
		None,
		None,
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
		Some(DEFAULT_PROTOCOL_ID),
		None,
		None)
}

fn local_development_genesis() -> GenesisConfig {
	development_genesis(
		vec![
			get_authority_keys_from_seed("Alice"),
			get_authority_keys_from_seed("Bob"),
		],
		get_account_id_from_seed("Alice"),
		None,
		None,
	)
}

/// Local testnet config (multivalidator Alice + Bob)
pub fn local_testnet_config() -> ChainSpec {
	ChainSpec::from_genesis(
		"Local Testnet",
		"local_testnet",
		local_development_genesis,
		vec![],
		None,
		Some(DEFAULT_PROTOCOL_ID),
		None,
		None)
}
