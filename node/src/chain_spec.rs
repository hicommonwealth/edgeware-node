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

use node_primitives::AccountId;
use primitives::{Ed25519AuthorityId as AuthorityId, ed25519};
use edgeware_runtime::{
	Permill, Perbill,
	BalancesConfig, ConsensusConfig, GenesisConfig, ContractConfig, SessionConfig,
	TimestampConfig, TreasuryConfig, StakingConfig, UpgradeKeyConfig, GrandpaConfig,
	IdentityConfig, GovernanceConfig, DelegationConfig, FeesConfig,
	CouncilSeatsConfig, CouncilVotingConfig, DemocracyConfig, IndicesConfig,
};
use substrate_service;
use substrate_telemetry::TelemetryEndpoints;
use substrate_keystore::pad_seed;

const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialised `ChainSpec`.
pub type ChainSpec = substrate_service::ChainSpec<GenesisConfig>;

/// Edgeware testnet generator
pub fn edgeware_config() -> Result<ChainSpec, String> {
	ChainSpec::from_embedded(include_bytes!("../../testnets/v0.1.4/edgeware.json"))
}

/// Helper function to generate AuthorityID from seed
pub fn get_authority_id_from_seed(seed: &str) -> AuthorityId {
	let padded_seed = pad_seed(seed);
	// NOTE from ed25519 impl:
	// prefer pkcs#8 unless security doesn't matter -- this is used primarily for tests.
	ed25519::Pair::from_seed(&padded_seed).public().0.into()
}

pub fn get_testnet_pubkeys() -> Vec<AuthorityId> {
	let pubkeys = vec![
		ed25519::Public::from_raw(hex!("df291854c27a22c50322344604076e8b2dc3ffe11dbdcd886adba9e0d6c9f950") as [u8; 32]).into(),
		ed25519::Public::from_raw(hex!("3bd15363a31eac0e5ecd067731d8a4561185347fc804c50b507025abc29c2ba1") as [u8; 32]).into(),
		ed25519::Public::from_raw(hex!("65b118b4ae7fe642a59316fc5f0ad9b75cdb9f5ab52733165004f7602755bcfd") as [u8; 32]).into(),
		ed25519::Public::from_raw(hex!("68128017e34fe40f4ed40f79c24dc7f5a531afc82fc6b71e8092c903627a9133") as [u8; 32]).into(),
		ed25519::Public::from_raw(hex!("dc746491a214053440d8b9df6774587da105661cc58ed703dc36965359c666a6") as [u8; 32]).into()
	];

	return pubkeys;
}

fn staging_testnet_config_genesis() -> GenesisConfig {
	// stash, controller, session-key
	let initial_authorities: Vec<(AccountId, AccountId, AuthorityId)> = vec![(
		hex!["d52793d9a0c5c7c82275f6d20cb314045e3ec4b93d73d828d9219f6b21ccdd51"].into(),
		hex!["dcb34453b417badcef2fb93efb855380a783289319389952622e4cea7d743145"].into(),
		hex!["6b72796524ee3fe2b7b4a3e65dde1e9057eeb0026e899fbecfee92ef9a8280ac"].into(),
	), (
		hex!["148cac52642973fc4a61bb434e6421681e67b64974d04a172e7c2514d26d5366"].into(),
		hex!["afcc85986335783a13907a12f273335b270a05bd99444a618ef3a1c4c4d92186"].into(),
		hex!["bdf3930a4e93e29200e1b1f81f1de011d2a9e02f774e54f67e6e52dfaeda9f23"].into(),
	), (
		hex!["7e0837ed15ea8198b099502570f356f9ec0b49fa2b961ec35e8ebd5498647335"].into(),
		hex!["faa71637adec464e2217464cdfa6e3a891257a814c222fd4360480bf0b36855c"].into(),
		hex!["b0278d6f62876202aa70e8133b21f49daa303ee8818b98988630d05235f7b798"].into(),
	), (
		hex!["eb708c619cdd7634c0ba7238c1f13a9c5cb4816d5af02c3fdde717f1966d05ad"].into(),
		hex!["db34823044cf58c96df8dec73896d4014ba03328c1fef29e8f447c003625d2ac"].into(),
		hex!["c0d496cc1c37fdc276e005960199e22c051a94bb611f93c537c463ff4b7dd61f"].into(),
	), (
		hex!["df291854c27a22c50322344604076e8b2dc3ffe11dbdcd886adba9e0d6c9f950"].into(),
		hex!["7ef7449d0d0224e0d9cabc66fe29aeff73dc923e10c8e199cb5aab0afb69d0e5"].into(),
		hex!["be3e3264a06a61d9c5c8055807bce41a71e2497257ee72f8745d251429014a2b"].into(),
	), (
		hex!["3bd15363a31eac0e5ecd067731d8a4561185347fc804c50b507025abc29c2ba1"].into(),
		hex!["619473a7bd9f608bfdfa93582b53cc8867245e91c9fe5026fee379d47c94dd09"].into(),
		hex!["ab66295ab4f3015a6108e391181f8ac13e40b437cedfa87983688c7e5065bb70"].into(),
	), (
		hex!["65b118b4ae7fe642a59316fc5f0ad9b75cdb9f5ab52733165004f7602755bcfd"].into(),
		hex!["01489c5e4c7d0cc8af9fba72c72b95785357e2db50fd8c5ae907ac799a66d9dd"].into(),
		hex!["e48e7a2b1c381a7a0821d61791daaa695bfd070815dd9fe02b51f60f81f0e034"].into(),
	), (
		hex!["68128017e34fe40f4ed40f79c24dc7f5a531afc82fc6b71e8092c903627a9133"].into(),
		hex!["8510ba4363ac9a70b34fd586a5a6a1335e3484ec4767617f49db060865e899c4"].into(),
		hex!["83191772bc526b7625ee6ca197a63f984ca10afc2231ad87865d71a6fda0b84d"].into(),
	), (
		hex!["dc746491a214053440d8b9df6774587da105661cc58ed703dc36965359c666a6"].into(),
		hex!["d04fa941c18fef1461da631b36766e410ef0017817a06f5c8728e3b23d87f660"].into(),
		hex!["b30d0b164273c00050d4c2e1eb1cc8be6ade9ac9078abbb692c649c81b4c21b4"].into(),
	)];

	// Test accounts with balances
	let endowed_accounts: Vec<AccountId> = vec![
		get_account_id_from_seed("Alice"),
		get_account_id_from_seed("Bob"),
		get_account_id_from_seed("Charlie"),
		get_account_id_from_seed("Dave"),
		get_account_id_from_seed("Eve"),
		get_account_id_from_seed("Ferdie"),
	];

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
			code: include_bytes!("../runtime/wasm/target/wasm32-unknown-unknown/release/edgeware_runtime.compact.wasm").to_vec(),    // FIXME change once we have #1252
			authorities: initial_authorities.iter().map(|x| x.2.clone()).collect(),
		}),
		system: None,
		balances: Some(BalancesConfig {
			balances: endowed_accounts.iter()
				.map(|&k| (k, ENDOWMENT))
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
			validators: initial_authorities.iter().map(|x| x.1.into()).collect(),
			session_length: 5 * MINUTES,
			keys: initial_authorities.iter().map(|x| (x.1.clone(), x.2.clone())).collect::<Vec<_>>(),
		}),
		staking: Some(StakingConfig {
			current_era: 0,
			offline_slash: Perbill::from_billionths(1_000_000),
			session_reward: Perbill::from_billionths(2_065),
			current_offline_slash: 0,
			current_session_reward: 0,
			validator_count: 7,
			sessions_per_era: 12,
			bonding_duration: 60 * MINUTES,
			offline_slash_grace: 4,
			minimum_validator_count: 4,
			stakers: initial_authorities.iter().map(|x| (x.0.into(), x.1.into(), STASH)).collect(),
			invulnerables: initial_authorities.iter().map(|x| x.1.into()).collect(),
		}),
		democracy: Some(DemocracyConfig {
			launch_period: 10 * MINUTES,    // 1 day per public referendum
			voting_period: 10 * MINUTES,    // 3 days to discuss & vote on an active referendum
			minimum_deposit: 50 * DOLLARS,    // 12000 as the minimum deposit for a referendum
			public_delay: 10 * MINUTES,
			max_lock_periods: 6,
		}),
		council_seats: Some(CouncilSeatsConfig {
			active_council: vec![],
			candidacy_bond: 10 * DOLLARS,
			voter_bond: 1 * DOLLARS,
			present_slash_per_voter: 1 * CENTS,
			carry_count: 6,
			presentation_duration: 1 * DAYS,
			approval_voting_period: 2 * DAYS,
			term_duration: 28 * DAYS,
			desired_seats: 0,
			inactive_grace_period: 1,    // one additional vote should go by before an inactive voter can be reaped.
		}),
		council_voting: Some(CouncilVotingConfig {
			cooloff_period: 4 * DAYS,
			voting_period: 1 * DAYS,
			enact_delay_period: 0,
		}),
		timestamp: Some(TimestampConfig {
			period: SECS_PER_BLOCK / 2, // due to the nature of aura the slots are 2*period
		}),
		treasury: Some(TreasuryConfig {
			proposal_bond: Permill::from_percent(5),
			proposal_bond_minimum: 1 * DOLLARS,
			spend_period: 1 * DAYS,
			burn: Permill::from_percent(50),
		}),
		contract: Some(ContractConfig {
			contract_fee: 1 * CENTS,
			call_base_fee: 1000,
			create_base_fee: 1000,
			gas_price: 1 * MILLICENTS,
			max_depth: 1024,
			block_gas_limit: 10_000_000,
			current_schedule: Default::default(),
		}),
		grandpa: Some(GrandpaConfig {
			authorities: initial_authorities.iter().map(|x| (x.2.clone(), 1)).collect(),
		}),
		fees: Some(FeesConfig {
			transaction_base_fee: 1 * CENTS,
			transaction_byte_fee: 10 * MILLICENTS,
		}),
		upgrade_key: Some(UpgradeKeyConfig {
			key: get_testnet_pubkeys()[0].0.into(),
		}),
		identity: Some(IdentityConfig {
			verifiers: get_testnet_pubkeys().iter().map(|x| x.0.into()).collect(),
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
		None,
	)
}

/// Helper function to generate AuthorityId from seed
pub fn get_account_id_from_seed(seed: &str) -> AccountId {
	let padded_seed = pad_seed(seed);
	// NOTE from ed25519 impl:
	// prefer pkcs#8 unless security doesn't matter -- this is used primarily for tests.
	ed25519::Pair::from_seed(&padded_seed).public().0.into()
}

/// Helper function to generate stash, controller and session key from seed
pub fn get_authority_keys_from_seed(seed: &str) -> (AccountId, AccountId, AuthorityId) {
	let padded_seed = pad_seed(seed);
	// NOTE from ed25519 impl:
	// prefer pkcs#8 unless security doesn't matter -- this is used primarily for tests.
	(
		get_account_id_from_seed(&format!("{}-stash", seed)),
		get_account_id_from_seed(seed),
		ed25519::Pair::from_seed(&padded_seed).public().0.into()
	)
}

/// Helper function to create GenesisConfig for testing
pub fn testnet_genesis(
	mut init_authorities: Vec<(AccountId, AccountId, AuthorityId)>,
	root_key: AccountId,
	endowed_accounts: Option<Vec<AccountId>>,
) -> GenesisConfig {
	// stash, controller, session-key
	let mut initial_authorities: Vec<(AccountId, AccountId, AuthorityId)> = vec![(
		hex!["d52793d9a0c5c7c82275f6d20cb314045e3ec4b93d73d828d9219f6b21ccdd51"].into(),
		hex!["dcb34453b417badcef2fb93efb855380a783289319389952622e4cea7d743145"].into(),
		hex!["6b72796524ee3fe2b7b4a3e65dde1e9057eeb0026e899fbecfee92ef9a8280ac"].into(),
	), (
		hex!["148cac52642973fc4a61bb434e6421681e67b64974d04a172e7c2514d26d5366"].into(),
		hex!["afcc85986335783a13907a12f273335b270a05bd99444a618ef3a1c4c4d92186"].into(),
		hex!["bdf3930a4e93e29200e1b1f81f1de011d2a9e02f774e54f67e6e52dfaeda9f23"].into(),
	), (
		hex!["7e0837ed15ea8198b099502570f356f9ec0b49fa2b961ec35e8ebd5498647335"].into(),
		hex!["faa71637adec464e2217464cdfa6e3a891257a814c222fd4360480bf0b36855c"].into(),
		hex!["b0278d6f62876202aa70e8133b21f49daa303ee8818b98988630d05235f7b798"].into(),
	), (
		hex!["eb708c619cdd7634c0ba7238c1f13a9c5cb4816d5af02c3fdde717f1966d05ad"].into(),
		hex!["db34823044cf58c96df8dec73896d4014ba03328c1fef29e8f447c003625d2ac"].into(),
		hex!["c0d496cc1c37fdc276e005960199e22c051a94bb611f93c537c463ff4b7dd61f"].into(),
	), (
		hex!["df291854c27a22c50322344604076e8b2dc3ffe11dbdcd886adba9e0d6c9f950"].into(),
		hex!["7ef7449d0d0224e0d9cabc66fe29aeff73dc923e10c8e199cb5aab0afb69d0e5"].into(),
		hex!["be3e3264a06a61d9c5c8055807bce41a71e2497257ee72f8745d251429014a2b"].into(),
	), (
		hex!["3bd15363a31eac0e5ecd067731d8a4561185347fc804c50b507025abc29c2ba1"].into(),
		hex!["619473a7bd9f608bfdfa93582b53cc8867245e91c9fe5026fee379d47c94dd09"].into(),
		hex!["ab66295ab4f3015a6108e391181f8ac13e40b437cedfa87983688c7e5065bb70"].into(),
	), (
		hex!["65b118b4ae7fe642a59316fc5f0ad9b75cdb9f5ab52733165004f7602755bcfd"].into(),
		hex!["01489c5e4c7d0cc8af9fba72c72b95785357e2db50fd8c5ae907ac799a66d9dd"].into(),
		hex!["e48e7a2b1c381a7a0821d61791daaa695bfd070815dd9fe02b51f60f81f0e034"].into(),
	), (
		hex!["68128017e34fe40f4ed40f79c24dc7f5a531afc82fc6b71e8092c903627a9133"].into(),
		hex!["8510ba4363ac9a70b34fd586a5a6a1335e3484ec4767617f49db060865e899c4"].into(),
		hex!["83191772bc526b7625ee6ca197a63f984ca10afc2231ad87865d71a6fda0b84d"].into(),
	), (
		hex!["dc746491a214053440d8b9df6774587da105661cc58ed703dc36965359c666a6"].into(),
		hex!["d04fa941c18fef1461da631b36766e410ef0017817a06f5c8728e3b23d87f660"].into(),
		hex!["b30d0b164273c00050d4c2e1eb1cc8be6ade9ac9078abbb692c649c81b4c21b4"].into(),
	)];

	initial_authorities.append(&mut init_authorities);


	let mut endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
		vec![
			get_account_id_from_seed("Alice"),
			get_account_id_from_seed("Bob"),
			get_account_id_from_seed("Charlie"),
			get_account_id_from_seed("Dave"),
			get_account_id_from_seed("Eve"),
			get_account_id_from_seed("Ferdie"),
		]
	});

	endowed_accounts.append(&mut vec![
		hex!["d52793d9a0c5c7c82275f6d20cb314045e3ec4b93d73d828d9219f6b21ccdd51"].into(),
		hex!["148cac52642973fc4a61bb434e6421681e67b64974d04a172e7c2514d26d5366"].into(),
		hex!["7e0837ed15ea8198b099502570f356f9ec0b49fa2b961ec35e8ebd5498647335"].into(),
		hex!["eb708c619cdd7634c0ba7238c1f13a9c5cb4816d5af02c3fdde717f1966d05ad"].into(),
		hex!["df291854c27a22c50322344604076e8b2dc3ffe11dbdcd886adba9e0d6c9f950"].into(),
		hex!["3bd15363a31eac0e5ecd067731d8a4561185347fc804c50b507025abc29c2ba1"].into(),
		hex!["65b118b4ae7fe642a59316fc5f0ad9b75cdb9f5ab52733165004f7602755bcfd"].into(),
		hex!["68128017e34fe40f4ed40f79c24dc7f5a531afc82fc6b71e8092c903627a9133"].into(),
	]);

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
			balances: endowed_accounts.iter().map(|&k| (k.into(), ENDOWMENT)).collect(),
			vesting: vec![],
		}),
		session: Some(SessionConfig {
			validators: initial_authorities.iter().map(|x| x.1.into()).collect(),
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
			stakers: initial_authorities.iter().map(|x| (x.0.into(), x.1.into(), STASH)).collect(),
			invulnerables: initial_authorities.iter().map(|x| x.1.into()).collect(),
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
				.map(|a| (a.clone().into(), 1000000)).collect(),
			candidacy_bond: 10,
			voter_bond: 2,
			present_slash_per_voter: 1,
			carry_count: 4,
			presentation_duration: 10,
			approval_voting_period: 20,
			term_duration: 1000000,
			desired_seats: (endowed_accounts.len() - initial_authorities.len()) as u32,
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
		grandpa: Some(GrandpaConfig {
			authorities: initial_authorities.iter().map(|x| (x.2.clone(), 1)).collect(),
		}),
		fees: Some(FeesConfig {
			transaction_base_fee: 1,
			transaction_byte_fee: 0,
		}),
		upgrade_key: Some(UpgradeKeyConfig {
			key: root_key,
		}),
		identity: Some(IdentityConfig {
			verifiers: get_testnet_pubkeys().iter().map(|x| x.0.into()).collect(),
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
		get_account_id_from_seed("Alice").into(),
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
		get_account_id_from_seed("Alice").into(),
		None,
	)
}

/// Local testnet config (multivalidator Alice + Bob)
pub fn local_testnet_config() -> ChainSpec {
	ChainSpec::from_genesis("Local Testnet", "local_testnet", local_testnet_genesis, vec![], None, None, None, None)
}
