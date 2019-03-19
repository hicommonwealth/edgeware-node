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

use primitives::{ed25519::Public as AuthorityId, ed25519, sr25519, Pair};
use node_primitives::AccountId;
use edgeware_runtime::{
	Permill, Perbill,
	BalancesConfig, ConsensusConfig, GenesisConfig, ContractConfig, SessionConfig,
	TimestampConfig, TreasuryConfig, StakingConfig, StakerStatus, SudoConfig, GrandpaConfig,
	IdentityConfig, GovernanceConfig, DelegationConfig, FeesConfig,
	CouncilSeatsConfig, CouncilVotingConfig, DemocracyConfig, IndicesConfig,
};
use substrate_service;
use substrate_telemetry::TelemetryEndpoints;

const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialised `ChainSpec`.
pub type ChainSpec = substrate_service::ChainSpec<GenesisConfig>;

pub fn edgeware_testnet_config() -> ChainSpec {
	match ChainSpec::from_json_file(std::path::PathBuf::from("testnets/v0.1.6/edgeware.json")) {
		Ok(spec) => spec,
		Err(e) => panic!(e),
	}
}

pub fn edgeware_config_gensis() -> GenesisConfig {
	testnet_genesis(
		vec![
			get_authority_keys_from_seed("Alice"),
			get_authority_keys_from_seed("A"),
			get_authority_keys_from_seed("B"),
			get_authority_keys_from_seed("C"),
			get_authority_keys_from_seed("D"),
		],
		get_account_id_from_seed("Alice").into(),
		Some(vec![
			get_account_id_from_seed("Alice"),
			get_account_id_from_seed("Bob"),
			get_account_id_from_seed("Charlie"),
			get_account_id_from_seed("Dave"),
			get_account_id_from_seed("Eve"),
			get_account_id_from_seed("A"),
			get_account_id_from_seed("B"),
			get_account_id_from_seed("C"),
			get_account_id_from_seed("D"),
			get_account_id_from_seed("E"),
			get_account_id_from_seed("F"),
			get_account_id_from_seed("G"),
			get_account_id_from_seed("H"),
			get_account_id_from_seed("I"),
			get_account_id_from_seed("J"),
			get_account_id_from_seed("K"),
			get_account_id_from_seed("L"),
			get_account_id_from_seed("M"),
			get_account_id_from_seed("N"),
			get_account_id_from_seed("O"),
			get_account_id_from_seed("P"),
			get_account_id_from_seed("Q"),
		]),
	)
}

/// Edgeware testnet generator
pub fn edgeware_config() -> Result<ChainSpec, String> {
	let boot_nodes = vec![];
	Ok(ChainSpec::from_genesis(
		"Edgeware",
		"edgeware",
		edgeware_config_gensis,
		boot_nodes,
		Some(TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])),
		None,
		None,
		None
	))
}

/// Helper function to generate AccountId from seed
pub fn get_account_id_from_seed(seed: &str) -> AccountId {
	sr25519::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Helper function to generate AuthorityId from seed
pub fn get_session_key_from_seed(seed: &str) -> AuthorityId {
	ed25519::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Helper function to generate stash, controller and session key from seed
pub fn get_authority_keys_from_seed(seed: &str) -> (AccountId, AccountId, AuthorityId) {
	(
		get_account_id_from_seed(&format!("{}//stash", seed)),
		get_account_id_from_seed(seed),
		get_session_key_from_seed(seed)
	)
}

/// Helper function to create GenesisConfig for testing
pub fn testnet_genesis(
	initial_authorities: Vec<(AccountId, AccountId, AuthorityId)>,
	root_key: AccountId,
	endowed_accounts: Option<Vec<AccountId>>,
) -> GenesisConfig {
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

	const STASH: u128 = 1 << 20;
	const ENDOWMENT: u128 = 1 << 20;

	GenesisConfig {
		consensus: Some(ConsensusConfig {
			code: include_bytes!("../runtime/wasm/target/wasm32-unknown-unknown/release/edgeware_runtime.compact.wasm").to_vec(),
			authorities: initial_authorities.iter().map(|x| x.2.clone()).collect(),
		}),
		system: None,
		indices: Some(IndicesConfig {
			ids: endowed_accounts.clone(),
		}),
		balances: Some(BalancesConfig {
			existential_deposit: 500,
			transfer_fee: 0,
			creation_fee: 0,
			balances: endowed_accounts.iter().map(|k| (k.clone(), ENDOWMENT)).collect(),
			vesting: vec![],
		}),
		session: Some(SessionConfig {
			validators: initial_authorities.iter().map(|x| x.1.clone()).collect(),
			session_length: 10,
			keys: initial_authorities.iter().map(|x| (x.1.clone(), x.2.clone())).collect::<Vec<_>>(),
		}),
		staking: Some(StakingConfig {
			current_era: 0,
			minimum_validator_count: 1,
			validator_count: 2,
			sessions_per_era: 5,
			bonding_duration: 2 * 60 * 12,
			offline_slash: Perbill::zero(),
			session_reward: Perbill::zero(),
			current_offline_slash: 0,
			current_session_reward: 0,
			offline_slash_grace: 0,
			stakers: initial_authorities.iter().map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator)).collect(),
			invulnerables: initial_authorities.iter().map(|x| x.1.clone()).collect(),
		}),
		democracy: Some(DemocracyConfig {
			launch_period: 9,
			voting_period: 18,
			minimum_deposit: 10,
			public_delay: 0,
			max_lock_periods: 6,
		}),
		council_seats: Some(CouncilSeatsConfig {
			active_council: endowed_accounts.iter()
				.filter(|&endowed| initial_authorities.iter().find(|&(_, controller, _)| controller == endowed).is_none())
				.map(|a| (a.clone(), 1000000)).collect(),
			candidacy_bond: 10,
			voter_bond: 2,
			present_slash_per_voter: 1,
			carry_count: 4,
			presentation_duration: 10,
			approval_voting_period: 20,
			term_duration: 1000000,
			desired_seats: (endowed_accounts.len() / 2 - initial_authorities.len()) as u32,
			inactive_grace_period: 1,
		}),
		council_voting: Some(CouncilVotingConfig {
			cooloff_period: 75,
			voting_period: 20,
			enact_delay_period: 0,
		}),
		timestamp: Some(TimestampConfig {
			period: 2,                    // 2*2=4 second block time.
		}),
		treasury: Some(TreasuryConfig {
			proposal_bond: Permill::from_percent(5),
			proposal_bond_minimum: 1_000_000,
			spend_period: 12 * 60 * 24,
			burn: Permill::from_percent(50),
		}),
		contract: Some(ContractConfig {
			contract_fee: 21,
			call_base_fee: 135,
			create_base_fee: 175,
			gas_price: 1,
			max_depth: 1024,
			block_gas_limit: 10_000_000,
			current_schedule: Default::default(),
		}),
		sudo: Some(SudoConfig {
			key: root_key,
		}),
		grandpa: Some(GrandpaConfig {
			authorities: initial_authorities.iter().map(|x| (x.2.clone(), 1)).collect(),
		}),
		fees: Some(FeesConfig {
			transaction_base_fee: 1,
			transaction_byte_fee: 0,
		}),
		identity: Some(IdentityConfig {
			verifiers: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			expiration_time: 604800, // 7 days
		}),
		governance: Some(GovernanceConfig {
			voting_time: 604800, // 7 days
		}),
		delegation: Some(DelegationConfig {
			delegation_depth: 5,
			_genesis_phantom_data: Default::default(),
		}),
	}
}

fn development_config_genesis() -> GenesisConfig {
	testnet_genesis(
		vec![
			get_authority_keys_from_seed("Alice"),
		],
		get_account_id_from_seed("Alice"),
		None,
	)
}

/// Development config (single validator Alice)
pub fn development_config() -> ChainSpec {
	ChainSpec::from_genesis("Development", "dev", development_config_genesis, vec![], None, None, None, None)
}

fn local_testnet_genesis() -> GenesisConfig {
	testnet_genesis(
		vec![
			get_authority_keys_from_seed("Alice"),
			get_authority_keys_from_seed("Bob"),
		],
		get_account_id_from_seed("Alice"),
		None,
	)
}

/// Local testnet config (multivalidator Alice + Bob)
pub fn local_testnet_config() -> ChainSpec {
	ChainSpec::from_genesis("Local Testnet", "local_testnet", local_testnet_genesis, vec![], None, None, None, None)
}
