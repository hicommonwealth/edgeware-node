use edgeware_runtime::{
	Permill, Perbill,
	BalancesConfig, ConsensusConfig, GenesisConfig, ContractConfig, SessionConfig,
	TimestampConfig, TreasuryConfig, StakingConfig, UpgradeKeyConfig, GrandpaConfig,
	IdentityConfig, GovernanceConfig, DelegationConfig,
	CouncilSeatsConfig, CouncilVotingConfig, DemocracyConfig, IndicesConfig,
};
use node_primitives::AccountId;
use primitives::{ed25519, Ed25519AuthorityId};
use substrate_service;

use substrate_keystore::pad_seed;

/// Specialised `ChainSpec`. This is a specialisation of the general Substrate ChainSpec type.
pub type ChainSpec = substrate_service::ChainSpec<GenesisConfig>;

/// The chain specification option. This is expected to come in from the CLI and
/// is little more than one of a number of alternatives which can easily be converted
/// from a string (`--chain=...`) into a `ChainSpec`.
#[derive(Clone, Debug)]
pub enum Alternative {
	/// Whatever the current runtime is, with just Alice as an auth.
	Development,
	/// Whatever the current runtime is, with simple Alice/Bob auths.
	LocalTestnet,
	/// Whatever the current runtime is, with all lock droppers and valid authorities
	Edgeware,
}

/// Helper function to generate AuthorityID from seed
pub fn get_authority_id_from_seed(seed: &str) -> Ed25519AuthorityId {
	let padded_seed = pad_seed(seed);
	// NOTE from ed25519 impl:
	// prefer pkcs#8 unless security doesn't matter -- this is used primarily for tests.
	ed25519::Pair::from_seed(&padded_seed).public().0.into()
}

pub fn get_testnet_pubkeys() -> Vec<Ed25519AuthorityId> {
	let pubkeys = vec![
		ed25519::Public::from_raw(hex!("df291854c27a22c50322344604076e8b2dc3ffe11dbdcd886adba9e0d6c9f950") as [u8; 32]).into(),
		ed25519::Public::from_raw(hex!("3bd15363a31eac0e5ecd067731d8a4561185347fc804c50b507025abc29c2ba1") as [u8; 32]).into(),
		ed25519::Public::from_raw(hex!("65b118b4ae7fe642a59316fc5f0ad9b75cdb9f5ab52733165004f7602755bcfd") as [u8; 32]).into(),
		ed25519::Public::from_raw(hex!("68128017e34fe40f4ed40f79c24dc7f5a531afc82fc6b71e8092c903627a9133") as [u8; 32]).into(),
		ed25519::Public::from_raw(hex!("dc746491a214053440d8b9df6774587da105661cc58ed703dc36965359c666a6") as [u8; 32]).into()
	];

	return pubkeys;
}

impl Alternative {
	/// Get an actual chain config from one of the alternatives.
	pub(crate) fn load(self) -> Result<ChainSpec, String> {
		Ok(match self {
			Alternative::Development => ChainSpec::from_genesis(
				"Development",
				"development",
				|| {
					testnet_genesis(
						vec![
							get_authority_id_from_seed("Alice"),
						],
						get_authority_id_from_seed("Alice").into(),
						None,
					)
				},
				vec![],
				None,
				None,
				None,
				None,
			),
			Alternative::LocalTestnet => ChainSpec::from_genesis(
				"Local Testnet",
				"local_testnet",
				|| {
					testnet_genesis(
						vec![
							get_authority_id_from_seed("Alice"),
							get_authority_id_from_seed("Bob"),
						],
						get_authority_id_from_seed("Alice").into(),
						None,
					)
				},
				vec![],
				None,
				None,
				None,
				None,
			),
			Alternative::Edgeware => ChainSpec::from_genesis(
				"Edgeware",
				"edgeware-testnet",
				|| {
					testnet_genesis(
						get_testnet_pubkeys(),
						get_testnet_pubkeys()[0].into(),
						Some(get_testnet_pubkeys()),
					)
				},
				vec![],
				None,
				None,
				None,
				None,
			),
		})
	}

	pub(crate) fn from(s: &str) -> Option<Self> {
		match s {
			"dev" => Some(Alternative::Development),
			"local" => Some(Alternative::LocalTestnet),
			"edgeware" => Some(Alternative::Edgeware),
			_ => None,
		}
	}
}

fn testnet_genesis(
	initial_authorities: Vec<Ed25519AuthorityId>,
	root_key: AccountId,
	endowed_accounts: Option<Vec<Ed25519AuthorityId>>,
) -> GenesisConfig {
	let endowed_accounts = endowed_accounts.unwrap_or_else(|| {
		vec![
			get_authority_id_from_seed("Alice"),
			get_authority_id_from_seed("Bob"),
			get_authority_id_from_seed("Charlie"),
			get_authority_id_from_seed("Dave"),
			get_authority_id_from_seed("Eve"),
			get_authority_id_from_seed("Ferdie"),
		]
	});

	GenesisConfig {
		consensus: Some(ConsensusConfig {
			code: include_bytes!("../runtime/wasm/target/wasm32-unknown-unknown/release/edgeware_runtime.wasm").to_vec(),
			authorities: initial_authorities.clone(),
		}),
		system: None,
		indices: Some(IndicesConfig {
			ids: endowed_accounts.iter().map(|x| x.0.into()).collect(),
		}),
		timestamp: Some(TimestampConfig {
			period: 5,					// 5 second block time.
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
			.filter(|a| initial_authorities.iter().find(|&b| a.0 == b.0).is_none())
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
		balances: Some(BalancesConfig {
			transaction_base_fee: 1,
			transaction_byte_fee: 0,
			existential_deposit: 500,
			transfer_fee: 0,
			creation_fee: 0,
			balances: endowed_accounts.iter().map(|&k| (k.into(), (1 << 60))).collect(),
			vesting: vec![],
		}),
		session: Some(SessionConfig {
			validators: initial_authorities.iter().cloned().map(Into::into).collect(),
			session_length: 10,
		}),
		staking: Some(StakingConfig {
			current_era: 0,
			intentions: initial_authorities.iter().cloned().map(Into::into).collect(),
			minimum_validator_count: 1,
			validator_count: 2,
			sessions_per_era: 5,
			bonding_duration: 2 * 60 * 12,
			offline_slash: Perbill::zero(),
			session_reward: Perbill::zero(),
			current_offline_slash: 0,
			current_session_reward: 0,
			offline_slash_grace: 0,
			invulnerables: initial_authorities.iter().cloned().map(Into::into).collect(),
		}),
		upgrade_key: Some(UpgradeKeyConfig {
			key: root_key,
		}),
		grandpa: Some(GrandpaConfig {
			authorities: initial_authorities.clone().into_iter().map(|k| (k, 1)).collect(),
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
		treasury: Some(TreasuryConfig {
			proposal_bond: Permill::from_percent(5),
			proposal_bond_minimum: 1_000_000,
			spend_period: 12 * 60 * 24,
			burn: Permill::from_percent(50),
		}),
		identity: Some(IdentityConfig {
			verifiers: initial_authorities.iter().cloned().map(Into::into).collect(),
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
