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

use primitives::{Pair, Public};
use edgeware_primitives::{AccountId, AuraId, Balance};
use im_online::ed25519::{AuthorityId as ImOnlineId};
use edgeware_runtime::{
	GrandpaConfig, BalancesConfig, ContractsConfig, ElectionsConfig, DemocracyConfig, CouncilConfig,
	AuraConfig, IndicesConfig, SessionConfig, StakingConfig, SudoConfig, TreasuryRewardConfig,
	SystemConfig, ImOnlineConfig, WASM_BINARY, Perbill, SessionKeys, StakerStatus, AuthorityDiscoveryConfig,
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
use core::convert::TryInto;
use rand::{thread_rng};

const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
const DEFAULT_PROTOCOL_ID: &str = "edg";

/// Specialised `ChainSpec`.
pub type ChainSpec = substrate_service::ChainSpec<GenesisConfig>;

pub fn edgeware_testnet_v8_config() -> ChainSpec {
	match ChainSpec::from_json_file(std::path::PathBuf::from("testnets/v0.8.0/edgeware.json")) {
		Ok(spec) => spec,
		Err(e) => panic!(e),
	}
}

pub fn edgeware_testnet_v9_config() -> ChainSpec {
	match ChainSpec::from_json_file(std::path::PathBuf::from("testnets/v0.9.0/edgeware.json")) {
		Ok(spec) => spec,
		Err(e) => panic!(e),
	}
}

pub fn edgeware_testnet_config_gensis() -> GenesisConfig {
	let commonwealth_authorities: Vec<(
		AccountId,
		AccountId,
		AuraId,
		Balance,
		GrandpaId,
		ImOnlineId
	)> = get_testnet_commonwealth_validators();
	let spec = get_spec_allocation().unwrap();
	let lockdrop_balances = spec.0;
	let lockdrop_vesting = spec.1;
	let lockdrop_validators = spec.2;
	let root_key = get_root_key();
	// Add controller accounts to endowed accounts
	let endowed_accounts = get_more_endowed();
	let identity_verifiers = get_identity_verifiers();
	const ENDOWMENT: Balance = 10 * DOLLARS;
	// randomize the session keys
	let mut rng = thread_rng();
	let extras = vec![
		get_authority_keys_from_seed("Alice"),
		get_authority_keys_from_seed("Bob"),
		get_authority_keys_from_seed("Charlie"),
		get_authority_keys_from_seed("Dave"),
		get_authority_keys_from_seed("Eve"),
		get_authority_keys_from_seed("Ferdie"),
	];

    let mut session_keys = extras.iter().map(|x| (x.0.clone(), session_keys(x.2.clone(), x.3.clone(), x.4.clone())))
    	// .chain(commonwealth_authorities.iter().map(|x| (x.0.clone(), session_keys(x.2.clone(), x.4.clone(), x.5.clone()))))
		// .chain(lockdrop_validators.iter().map(|x| (x.0.clone(), session_keys(x.2.clone(), x.4.clone(), x.5.clone()))))
		.collect::<Vec<_>>();
	// session_keys.shuffle(&mut rng);



	GenesisConfig {
		system: Some(SystemConfig {
			code: WASM_BINARY.to_vec(),
			changes_trie_config: Default::default(),
		}),
		balances: Some(BalancesConfig {
			balances: endowed_accounts.iter().cloned()
				.map(|k| (k, ENDOWMENT))
				.chain(extras.iter().map(|x| (x.0.clone(), ENDOWMENT)))
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
			keys: session_keys,
		}),
		staking: Some(StakingConfig {
			current_era: 0,
			validator_count: 6,
			minimum_validator_count: 0,
			stakers: extras.iter().map(|x| (x.0.clone(), x.1.clone(), ENDOWMENT, StakerStatus::Validator))
				// .chain(commonwealth_authorities.iter().map(|x| (x.0.clone(), x.1.clone(), x.3.clone(), StakerStatus::Validator)))
				// .chain(lockdrop_validators.iter().map(|x| (x.0.clone(), x.1.clone(), x.3.clone(), StakerStatus::Validator)))
				.collect(),
			invulnerables: vec![],
				// extras.iter().map(|x| (x.0.clone()))
				// .chain(commonwealth_authorities.iter().map(|x| x.0.clone()))
				// .chain(lockdrop_validators.iter().map(|x| x.0.clone()))
				// .collect(),
			slash_reward_fraction: Perbill::from_percent(0),
			.. Default::default()
		}),
		democracy: Some(DemocracyConfig::default()),
		collective_Instance1: Some(CouncilConfig {
			members: extras.iter().map(|x| x.1.clone())
				// .chain(commonwealth_authorities.iter().map(|x| x.1.clone()))
				// .chain(endowed_accounts.iter().map(|x| x.clone()))
				.collect(),
			phantom: Default::default(),
		}),
		elections: Some(ElectionsConfig {
			members: extras.iter().map(|x| (x.1.clone(), 1000000))
				// .chain(commonwealth_authorities.iter().map(|x| (x.1.clone(), 1000000)))
				// .chain(endowed_accounts.iter().map(|x| (x.clone(), 1000000)))
				.collect(),
			desired_seats: 13,
			presentation_duration: (2 * DAYS).try_into().unwrap(),
			term_duration: (3 * DAYS).try_into().unwrap(),
		}),
		contracts: Some(ContractsConfig {
			current_schedule: Default::default(),
			gas_price: 1 * MILLICENTS,
		}),
		sudo: Some(SudoConfig {
			key: root_key,
		}),
		im_online: Some(ImOnlineConfig {
			keys: vec![],
		}),
		aura: Some(AuraConfig {
			authorities: vec![],
		}),
		grandpa: Some(GrandpaConfig {
			authorities: vec![],
		}),
		authority_discovery: Some(AuthorityDiscoveryConfig{
			keys: vec![],
		}),
		identity: Some(IdentityConfig {
			verifiers: identity_verifiers,
			expiration_length: (1 * DAYS).try_into().unwrap(), // 1 days
			registration_bond: 1 * DOLLARS,
		}),
		signaling: Some(SignalingConfig {
			voting_length: (3 * DAYS).try_into().unwrap(), // 7 days
			proposal_creation_bond: 100 * DOLLARS,
		}),
		treasury_reward: Some(TreasuryRewardConfig {
			current_payout: 158 * DOLLARS,
			minting_interval: One::one(),
		}),
	}
}

/// Edgeware testnet generator
pub fn edgeware_testnet_config() -> Result<ChainSpec, String> {
	let boot_nodes = vec![
	    // "/ip4/108.61.209.73/tcp/30333/p2p/QmeufKtv4KgAQUAUcWvagyFy7skrmSavKYNWYsP1MkJg2N".to_string(),
	    // "/ip4/45.77.93.189/tcp/30333/p2p/QmZh6bVocFNJznraiETnjDVXvmiBDwhMRjc36Ej677L3sf".to_string(),
	    // "/ip4/45.76.17.97/tcp/30333/p2p/QmdcteLq4pnzxikcGUrTNrdZT4cepMMBb8r4CBtwC7q9ZK".to_string(),
	    // "/ip4/44.202.84.209/tcp/30333/p2p/QmRuqvRv3Uudf81vXtSmVfD6bwWTgmjwLd4PG6PSh2Q2oP".to_string(),
	    // "/ip4/96.30.192.236/tcp/30333/p2p/QmUkAMzwRrxD3gzMWSXgoGxPkWbnLk72KoTmENVVbBneZB".to_string(),
	    // "/ip4/45.77.78.68/tcp/30333/p2p/QmeFzPbhp6HZEZkwnkAyoiUFN7Kr3NcDdD3HEPWq5XSNH3".to_string(),
	    // "/ip4/45.32.139.96/tcp/30333/p2p/QmNmmJGNah4RCNaiwPuv19T4haJGLcbKnU3Rx6phJHq5c6".to_string(),
	    // "/ip4/66.42.79.81/tcp/30333/p2p/QmW9PtASCawTG2SzsTnqEasaAv1FVjM7HJ56dZFyBKBuiC".to_string(),
	    // "/ip4/45.77.108.5/tcp/30333/p2p/QmT1LLofb3p8LJBwMmenzoDgvUzs18hnciXEvTsEi71AQn".to_string(),
	    // "/ip4/144.202.84.209/tcp/30333/p2p/QmRuqvRv3Uudf81vXtSmVfD6bwWTgmjwLd4PG6PSh2Q2oP".to_string(),
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

fn session_keys(aura: AuraId, grandpa: GrandpaId, im_online: ImOnlineId) -> SessionKeys {
	SessionKeys { aura, grandpa, im_online }
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}


/// Helper function to generate stash, controller and session key from seed
pub fn get_authority_keys_from_seed(seed: &str) -> (AccountId, AccountId, AuraId, GrandpaId, ImOnlineId) {
	(
		get_from_seed::<AccountId>(&format!("{}//stash", seed)),
		get_from_seed::<AccountId>(seed),
		get_from_seed::<AuraId>(seed),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<ImOnlineId>(seed),
	)
}

/// Helper function to create GenesisConfig for testing
pub fn development_genesis(
	initial_authorities: Vec<(AccountId, AccountId, AuraId, GrandpaId, ImOnlineId)>,
	root_key: AccountId,
	endowed_accounts: Option<Vec<AccountId>>,
	initial_verifiers: Option<Vec<AccountId>>,
) -> GenesisConfig {
	let initial_verifiers: Vec<AccountId> = initial_verifiers.unwrap_or_else(|| {
		vec![
			get_authority_keys_from_seed("Alice").1,
		]
	});

	let endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
		vec![
			get_authority_keys_from_seed("Alice").1,
			get_authority_keys_from_seed("Bob").1,
			get_authority_keys_from_seed("Charlie").1,
			get_authority_keys_from_seed("Dave").1,
			get_authority_keys_from_seed("Eve").1,
			get_authority_keys_from_seed("Ferdie").1,
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
			keys: initial_authorities.iter().map(|x|
				(x.0.clone(), session_keys(x.2.clone(), x.3.clone(), x.4.clone()))
			).collect::<Vec<_>>(),
		}),
		staking: Some(StakingConfig {
			current_era: 0,
			validator_count: 7,
			minimum_validator_count: 4,
			stakers: initial_authorities.iter().map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator)).collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			.. Default::default()
		}),
		democracy: Some(DemocracyConfig::default()),
		collective_Instance1: Some(CouncilConfig {
			members: initial_authorities.iter().map(|x| x.1.clone()).collect(),
			phantom: Default::default(),
		}),
		elections: Some(ElectionsConfig {
			members: initial_authorities.iter().map(|x| (x.1.clone(), 1000000)).collect(),
			desired_seats: desired_seats,
			presentation_duration: (1 * DAYS).try_into().unwrap(),
			term_duration: (28 * DAYS).try_into().unwrap(),
		}),
		contracts: Some(ContractsConfig {
			current_schedule: Default::default(),
			gas_price: 1 * MILLICENTS,
		}),
		sudo: Some(SudoConfig {
			key: root_key,
		}),
		im_online: Some(ImOnlineConfig {
			keys: vec![],
		}),
		aura: Some(AuraConfig {
			authorities: vec![],
		}),
		grandpa: Some(GrandpaConfig {
			authorities: vec![],
		}),
		authority_discovery: Some(AuthorityDiscoveryConfig{
			keys: vec![],
		}),
		identity: Some(IdentityConfig {
			verifiers: initial_verifiers,
			expiration_length: (1 * DAYS).try_into().unwrap(), // 1 days
			registration_bond: 1 * DOLLARS,
		}),
		signaling: Some(SignalingConfig {
			voting_length: (3 * DAYS).try_into().unwrap(), // 7 days
			proposal_creation_bond: 100 * DOLLARS,
		}),
		treasury_reward: Some(TreasuryRewardConfig {
			current_payout: 158 * DOLLARS,
			minting_interval: One::one(),
		}),
	}
}

fn development_config_genesis() -> GenesisConfig {
	development_genesis(
		vec![
			get_authority_keys_from_seed("Alice"),
		],
		get_authority_keys_from_seed("Alice").0,
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
		get_authority_keys_from_seed("Alice").0,
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
