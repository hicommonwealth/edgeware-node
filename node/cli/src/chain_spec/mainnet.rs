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

use edgeware_mainnet_runtime::constants::currency::*;
use edgeware_mainnet_runtime::{
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
use edgeware_mainnet_runtime::constants::time::*;
use edgeware_mainnet_runtime::GenesisConfig;


/// Specialized `ChainSpec`.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;

/// Mainnet configuration
pub fn edgeware_mainnet_official() -> ChainSpec {
	match ChainSpec::from_json_bytes(&include_bytes!("../res/mainnet.chainspec.json")[..]) {
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
		frame_system: Some(SystemConfig {
			code: wasm_binary_unwrap().to_vec(),
			changes_trie_config: Default::default(),
		}),
		pallet_balances: Some(BalancesConfig {
			balances: founder_allocation
				.iter()
				.map(|x| (x.0.clone(), x.1.clone()))
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
						session_keys(x.4.clone(), x.3.clone(), x.5.clone(), x.6.clone()),
					)
				})
				.collect::<Vec<_>>(),
		}),
		pallet_staking: Some(StakingConfig {
			validator_count: 60,
			minimum_validator_count: initial_authorities.len() as u32,
			stakers: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.1.clone(),
						x.2.clone(),
						StakerStatus::Validator,
					)
				})
				.collect(),
			invulnerables: vec![],
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		}),
		pallet_democracy: Some(DemocracyConfig::default()),
		pallet_collective_Instance1: Some(CouncilConfig {
			members: crate::mainnet_fixtures::get_mainnet_election_members(),
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
		pallet_sudo: Some(SudoConfig {
			key: crate::mainnet_fixtures::get_mainnet_root_key(),
		}),
		pallet_vesting: Some(VestingConfig { vesting: vesting }),
		pallet_evm: Some(Default::default()),
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
		"Edgeware",
		"edgeware",
		ChainType::Live,
		edgeware_mainnet_config_genesis,
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
