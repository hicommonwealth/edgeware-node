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

use substrate_primitives::{ed25519, sr25519, Pair, crypto::UncheckedInto};
use edgeware_primitives::{AccountId, AuthorityId};
use edgeware_runtime::{ConsensusConfig, CouncilSeatsConfig, DemocracyConfig,
	SessionConfig, StakingConfig, StakerStatus, TimestampConfig, BalancesConfig, TreasuryConfig,
	SudoConfig, ContractConfig, GrandpaConfig, IndicesConfig, Permill, Perbill,
	IdentityConfig, GovernanceConfig};
pub use edgeware_runtime::GenesisConfig;
use substrate_service;
use hex_literal::{hex, hex_impl};
use substrate_telemetry::TelemetryEndpoints;

const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
const DEFAULT_PROTOCOL_ID: &str = "edg";

/// Specialised `ChainSpec`.
pub type ChainSpec = substrate_service::ChainSpec<GenesisConfig>;

pub fn edgeware_config() -> ChainSpec {
	match ChainSpec::from_json_file(std::path::PathBuf::from("testnets/v0.2.0/edgeware.json")) {
		Ok(spec) => spec,
		Err(e) => panic!(e),
	}
}

pub fn edgeware_testnet_config_gensis() -> GenesisConfig {
	let initial_authorities: Vec<(AccountId, AccountId, AuthorityId)> = vec![(
		hex!["fcf5ef308894c9686b8302a23416ff57f4f92049b58ed3711a897d4627c56c94"].unchecked_into(), // 5HnNyuEyvLfTarQ4LLTSK4PpacMi2BTzmEYg4iKbVtSEU53C
		hex!["78ca2addb982a4a39262d68ed63fb4a8b0dfd73b7a38e0190f77b0cf155f94e0"].unchecked_into(), // 5Eo5fD9gBaMrLoLTeABXY4KoDL7mb4QtrzSQBMzFDJhhw57q
		hex!["e320e0bec84e02f4789170ad0126175d89f64257c504c599249df7b1fe90d688"].unchecked_into(), // 5HCWVbHiXgkHKgm2rqqbMx4UCDUz3BBWAY8oTRKDkahAaMEu
	),(
		hex!["5049a5a4f220aad7edc78bf3311710e05070b622cdd55bec8adf506a1d90ef73"].unchecked_into(), // 5DsyZmCaTGmKaiaFChifPRB4X4jcP1vWHuEbjxsF3NerzDgd
		hex!["4f8f200b881db35caaa041356f5ff454162cb930693983d09bafdc4abbc3f879"].unchecked_into(), // 5Ds2A4mjhSQSG7fFrZhhgj4fCDpdUEpm1L9bKzEzmrzRYNXX
		hex!["ada6a4726da7c73108cdf2686e6735dd3740dce19f2be7eee9460152cfb4a691"].unchecked_into(), // 5FzPeKoqxxKDbKT9QmNLeFP1CVSBqKpvV1P4maubvyyQSLqU
	),(
		hex!["efffffdcc35e2bb8eb1b274b19aa0e84db49e519083e9bdd5d06ee9b62b80c56"].unchecked_into(), // 5HVPMDQ5wzhKBnZtVSc1YGH8HvNYqzFfk4yFxdSCYuq3iTgK
		hex!["d2d25e48253d9b16f81323de36eb1c51d2fc07d9b89dafdcef4cb61bff52800c"].unchecked_into(), // 5Gq8QTxo9Kedv33bTCdWFz6DSrB46Ben5saEWEYrpkpBF5rF
		hex!["f07586a2147e4f63dd416b0b8363ed56f08f9cacf0dd35fcb6b5574f3b465d6c"].unchecked_into(), // 5HVzG9Yb7nCPRpLY7ypWJGKx5mJVtLkUFCDCtR9rQnVEm5ej
	),(
		hex!["d857fce59cca864637d5bc879eef6b23acc166ff4e01b6fc602bdf9c3f33fc18"].unchecked_into(), // 5GxNLZgAzKdC3MGRfZjbhFTazjfxrq4ZaMFq987rrMJ8mPJo
		hex!["b68ea4881c12b90205de5037283252b1a2dce85c819950a460d0d89945cad5a0"].unchecked_into(), // 5GC4wpJ3pQFfp1pDu6bXMvduZq3c6rRkYbgvQthT7M1TRBWb
		hex!["c39aabb19d05e62b7f743d737e53c6b5f892808a10224ccf8921a146b805c57a"].unchecked_into(), // 5GVB9A9W8q7aPBTxVMw5bmD8jgtu4kT93nVVK2wRU8eHP3QG
	)];

	testnet_genesis(
		initial_authorities, // authorities
		hex!["fcf5ef308894c9686b8302a23416ff57f4f92049b58ed3711a897d4627c56c94"].unchecked_into(), // root key
		Some(vec![ // endowed accounts
			hex!["561f4a7512b6c4a0c9708dd60ca76bb85e2f4e35ff0c5d0c5a63b4148c44e476"].unchecked_into()
		]),
		Some(vec![ // identity verifiers
			hex!["6d4b9f54cc2b3f16d17a1cbe641592ef1e9ce280c5e466c21cc6bcca11b6b5eb"].unchecked_into()
		]),
		false,
	)
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
	initial_verifiers: Option<Vec<AccountId>>,
	enable_println: bool,
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

	const STASH: u128 = 1 << 20;
	const ENDOWMENT: u128 = 1 << 20;

	let council_desired_seats = (endowed_accounts.len() / 2 - initial_authorities.len()) as u32;
	let mut contract_config = ContractConfig {
		signed_claim_handicap: 2,
		rent_byte_price: 4,
		rent_deposit_offset: 1000,
		storage_size_offset: 8,
		surcharge_reward: 150,
		tombstone_deposit: 16,
		transaction_base_fee: 1,
		transaction_byte_fee: 0,
		transfer_fee: 0,
		creation_fee: 0,
		contract_fee: 21,
		call_base_fee: 135,
		create_base_fee: 175,
		gas_price: 1,
		max_depth: 1024,
		block_gas_limit: 10_000_000,
		current_schedule: Default::default(),
	};
	// this should only be enabled on development chains
	contract_config.current_schedule.enable_println = enable_println;

	GenesisConfig {
		consensus: Some(ConsensusConfig {
			code: include_bytes!("../../runtime/wasm/target/wasm32-unknown-unknown/release/edgeware_runtime.compact.wasm").to_vec(),    // FIXME change once we have #1252
			authorities: initial_authorities.iter().map(|x| x.2.clone()).collect(),
		}),
		system: None,
		indices: Some(IndicesConfig {
			ids: endowed_accounts.clone(),
		}),
		balances: Some(BalancesConfig {
			transaction_base_fee: 1,
			transaction_byte_fee: 0,
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
			bonding_duration: 12,
			offline_slash: Perbill::zero(),
			session_reward: Perbill::zero(),
			current_session_reward: 0,
			offline_slash_grace: 0,
			stakers: initial_authorities.iter().map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator)).collect(),
			invulnerables: initial_authorities.iter().map(|x| x.1.clone()).collect(),
		}),
		democracy: Some(DemocracyConfig::default()),
		council_seats: Some(CouncilSeatsConfig {
			active_council: endowed_accounts.iter()
				.filter(|&endowed| initial_authorities.iter().find(|&(_, controller, _)| controller == endowed).is_none())
				.map(|a| (a.clone(), 1000000)).collect(),
			candidacy_bond: 10,
			voter_bond: 2,
			voting_fee: 5,
			present_slash_per_voter: 1,
			carry_count: 4,
			presentation_duration: 10,
			approval_voting_period: 20,
			term_duration: 1000000,
			desired_seats: council_desired_seats,
			decay_ratio: council_desired_seats / 3,
			inactive_grace_period: 1,
		}),
		timestamp: Some(TimestampConfig {
			minimum_period: 2,                    // 2*2=4 second block time.
		}),
		treasury: Some(TreasuryConfig {
			proposal_bond: Permill::from_percent(5),
			proposal_bond_minimum: 1_000_000,
			spend_period: 12 * 60 * 24,
			burn: Permill::from_percent(50),
		}),
		contract: Some(contract_config),
		sudo: Some(SudoConfig {
			key: root_key,
		}),
		grandpa: Some(GrandpaConfig {
			authorities: initial_authorities.iter().map(|x| (x.2.clone(), 1)).collect(),
		}),
		identity: Some(IdentityConfig {
			verifiers: initial_verifiers,
			expiration_length: 604800, // 7 days
			registration_bond: 1_000_000,
		}),
		governance: Some(GovernanceConfig {
			voting_length: 604800, // 7 days
			proposal_creation_bond: 1_000_000,
		}),
	}
}

/// Helper function to create GenesisConfig for commonwealth CI testing
pub fn cwci_testnet_genesis(
	initial_authorities: Vec<(AccountId, AccountId, AuthorityId)>,
	root_key: AccountId,
	endowed_accounts: Option<Vec<AccountId>>,
	initial_verifiers: Option<Vec<AccountId>>,
	_enable_println: bool,
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

	const MILLICENTS: u128 = 1_000_000_000;
	const CENTS: u128 = 1_000 * MILLICENTS;    // assume this is worth about a cent.
	const DOLLARS: u128 = 100 * CENTS;

	const SECS_PER_BLOCK: u64 = 6;
	const MINUTES: u64 = 60 / SECS_PER_BLOCK;
	const HOURS: u64 = MINUTES * 60;
	const DAYS: u64 = HOURS * 24;

	const ENDOWMENT: u128 = 10_000_000 * DOLLARS;
	const STASH: u128 = 100 * DOLLARS;
	GenesisConfig {
		consensus: Some(ConsensusConfig {
			code: include_bytes!("../../runtime/wasm/target/wasm32-unknown-unknown/release/edgeware_runtime.compact.wasm").to_vec(),    // FIXME change once we have #1252
			authorities: initial_authorities.iter().map(|x| x.2.clone()).collect(),
		}),
		system: None,
		balances: Some(BalancesConfig {
			transaction_base_fee: 1 * CENTS,
			transaction_byte_fee: 10 * MILLICENTS,
			balances: endowed_accounts.iter().cloned()
				.map(|k| (k, ENDOWMENT))
				.chain(initial_authorities.iter().map(|x| (x.0.clone(), STASH)))
				.collect(),
			existential_deposit: 1 * DOLLARS,
			transfer_fee: 1 * CENTS,
			creation_fee: 1 * CENTS,
			vesting: vec![],
		}),
		indices: Some(IndicesConfig {
			ids: endowed_accounts.iter().cloned()
				.chain(initial_authorities.iter().map(|x| x.0.clone()))
				.collect::<Vec<_>>(),
		}),
		session: Some(SessionConfig {
			validators: initial_authorities.iter().map(|x| x.1.clone()).collect(),
			session_length: 5 * MINUTES,
			keys: initial_authorities.iter().map(|x| (x.1.clone(), x.2.clone())).collect::<Vec<_>>(),
		}),
		staking: Some(StakingConfig {
			current_era: 0,
			offline_slash: Perbill::from_parts(1_000_000),
			session_reward: Perbill::from_parts(2_065),
			current_session_reward: 0,
			validator_count: 7,
			sessions_per_era: 12,
			bonding_duration: 12,
			offline_slash_grace: 4,
			minimum_validator_count: 4,
			stakers: initial_authorities.iter().map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator)).collect(),
			invulnerables: initial_authorities.iter().map(|x| x.1.clone()).collect(),
		}),
		democracy: Some(DemocracyConfig::default()),
		council_seats: Some(CouncilSeatsConfig {
			active_council: vec![],
			candidacy_bond: 10 * DOLLARS,
			voter_bond: 1 * DOLLARS,
			voting_fee: 2 * DOLLARS,
			present_slash_per_voter: 1 * CENTS,
			carry_count: 6,
			presentation_duration: 1 * DAYS,
			approval_voting_period: 2 * DAYS,
			term_duration: 28 * DAYS,
			desired_seats: 0,
			decay_ratio: 0,
			inactive_grace_period: 1,    // one additional vote should go by before an inactive voter can be reaped.
		}),
		timestamp: Some(TimestampConfig {
			minimum_period: SECS_PER_BLOCK / 2, // due to the nature of aura the slots are 2*period
		}),
		treasury: Some(TreasuryConfig {
			proposal_bond: Permill::from_percent(5),
			proposal_bond_minimum: 1 * DOLLARS,
			spend_period: 1 * DAYS,
			burn: Permill::from_percent(50),
		}),
		contract: Some(ContractConfig {
			signed_claim_handicap: 2,
			rent_byte_price: 4,
			rent_deposit_offset: 1000,
			storage_size_offset: 8,
			surcharge_reward: 150,
			tombstone_deposit: 16,
			transaction_base_fee: 1 * CENTS,
			transaction_byte_fee: 10 * MILLICENTS,
			transfer_fee: 1 * CENTS,
			creation_fee: 1 * CENTS,
			contract_fee: 1 * CENTS,
			call_base_fee: 1000,
			create_base_fee: 1000,
			gas_price: 1 * MILLICENTS,
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
		identity: Some(IdentityConfig {
			verifiers: initial_verifiers,
			expiration_length: 604800, // 7 days
			registration_bond: 100,
		}),
		governance: Some(GovernanceConfig {
			voting_length: 4,
			proposal_creation_bond: 100,

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
		None,
		true,
	)
}

/// Development config (single validator Alice)
pub fn development_config() -> ChainSpec {
	ChainSpec::from_genesis("Development", "dev", development_config_genesis, vec![], None, Some(DEFAULT_PROTOCOL_ID), None, None)
}

fn local_testnet_genesis() -> GenesisConfig {
	testnet_genesis(
		vec![
			get_authority_keys_from_seed("Alice"),
			get_authority_keys_from_seed("Bob"),
		],
		get_account_id_from_seed("Alice"),
		None,
		None,
		true,
	)
}

/// Local testnet config (multivalidator Alice + Bob)
pub fn local_testnet_config() -> ChainSpec {
	ChainSpec::from_genesis("Local Testnet", "local_testnet", local_testnet_genesis, vec![], None, Some(DEFAULT_PROTOCOL_ID), None, None)
}


fn cwci_config_genesis() -> GenesisConfig {
	cwci_testnet_genesis(
		vec![
			get_authority_keys_from_seed("Alice"),
			get_authority_keys_from_seed("Bob"),
		],
		get_account_id_from_seed("Alice"),
		None,
		None,
		false,
	)
}

/// Local testnet config (multivalidator Alice + Bob)
pub fn cwci_testnet_config() -> ChainSpec {
	ChainSpec::from_genesis("Commonwealth CI Testnet", "cwci_testnet", cwci_config_genesis, vec![], None, Some(DEFAULT_PROTOCOL_ID), None, None)
}

#[cfg(test)]
pub(crate) mod tests {
	use super::*;
	use service_test;
	use crate::Factory;

	fn local_testnet_genesis_instant() -> GenesisConfig {
		let mut genesis = local_testnet_genesis();
		genesis.timestamp = Some(TimestampConfig { minimum_period: 1 });
		genesis
	}

	fn local_testnet_genesis_instant_single() -> GenesisConfig {
		let mut genesis = testnet_genesis(
			vec![
				get_authority_keys_from_seed("Alice"),
			],
			get_account_id_from_seed("Alice"),
			None,
			None,
			false,
		);
		genesis.timestamp = Some(TimestampConfig { minimum_period: 1 });
		genesis
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
			None,
		)
	}

	/// Local testnet config (multivalidator Alice + Bob)
	pub fn integration_test_config_with_two_authorities() -> ChainSpec {
		ChainSpec::from_genesis("Integration Test", "test", local_testnet_genesis_instant, vec![], None, None, None, None)
	}

	#[test]
	#[ignore]
	fn test_connectivity() {
		service_test::connectivity::<Factory>(integration_test_config_with_two_authorities());
	}
}
