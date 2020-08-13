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

//! Genesis Configuration.

use crate::keyring::*;
use edgeware_runtime::constants::currency::*;
use edgeware_runtime::{
	BalancesConfig, ContractsConfig, GenesisConfig, GrandpaConfig, IndicesConfig, SessionConfig,
	StakingConfig, SystemConfig, WASM_BINARY,
};
use sp_core::ChangesTrieConfiguration;
use sp_keyring::{Ed25519Keyring, Sr25519Keyring};
use sp_runtime::Perbill;

/// Create genesis runtime configuration for tests.
pub fn config(support_changes_trie: bool, code: Option<&[u8]>) -> GenesisConfig {
	let endowed = vec![
		(alice(), 111 * DOLLARS),
		(bob(), 100 * DOLLARS),
		(charlie(), 100_000_000 * DOLLARS),
		(dave(), 111 * DOLLARS),
		(eve(), 101 * DOLLARS),
		(ferdie(), 100 * DOLLARS),
	];

	GenesisConfig {
		frame_system: Some(SystemConfig {
			changes_trie_config: if support_changes_trie {
				Some(ChangesTrieConfiguration {
					digest_interval: 2,
					digest_levels: 2,
				})
			} else {
				None
			},
			code: code
				.map(|x| x.to_vec())
				.unwrap_or_else(|| WASM_BINARY.to_vec()),
		}),
		pallet_indices: Some(IndicesConfig { indices: vec![] }),
		pallet_balances: Some(BalancesConfig { balances: endowed }),
		pallet_session: Some(SessionConfig {
			keys: vec![
				(
					dave(),
					alice(),
					to_session_keys(&Ed25519Keyring::Alice, &Sr25519Keyring::Alice),
				),
				(
					eve(),
					bob(),
					to_session_keys(&Ed25519Keyring::Bob, &Sr25519Keyring::Bob),
				),
				(
					ferdie(),
					charlie(),
					to_session_keys(&Ed25519Keyring::Charlie, &Sr25519Keyring::Charlie),
				),
			],
		}),
		pallet_staking: Some(StakingConfig {
			stakers: vec![
				(
					dave(),
					alice(),
					111 * DOLLARS,
					pallet_staking::StakerStatus::Validator,
				),
				(
					eve(),
					bob(),
					100 * DOLLARS,
					pallet_staking::StakerStatus::Validator,
				),
				(
					ferdie(),
					charlie(),
					100 * DOLLARS,
					pallet_staking::StakerStatus::Validator,
				),
			],
			validator_count: 3,
			minimum_validator_count: 0,
			slash_reward_fraction: Perbill::from_percent(10),
			invulnerables: vec![alice(), bob(), charlie()],
			..Default::default()
		}),
		pallet_aura: Some(Default::default()),
		pallet_grandpa: Some(GrandpaConfig {
			authorities: vec![],
		}),
		pallet_im_online: Some(Default::default()),
		pallet_authority_discovery: Some(Default::default()),
		pallet_democracy: Some(Default::default()),
		pallet_collective_Instance1: Some(Default::default()),
		pallet_treasury: Some(Default::default()),
		pallet_elections_phragmen: Some(Default::default()),
		pallet_sudo: Some(Default::default()),
		pallet_vesting: Some(Default::default()),
		// pallet_evm: Some(Default::default()),
		signaling: Some(Default::default()),
		treasury_reward: Some(Default::default()),
	}
}
