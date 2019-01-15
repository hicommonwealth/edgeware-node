use edgeware_runtime::{
	Permill, Perbill,
	BalancesConfig, ConsensusConfig, GenesisConfig, ContractConfig, SessionConfig,
	TimestampConfig, TreasuryConfig, StakingConfig, UpgradeKeyConfig, GrandpaConfig,
	IdentityConfig, GovernanceConfig
};
use node_primitives::AccountId;
use primitives::{ed25519, Ed25519AuthorityId};
use substrate_service;

type SessionKey = Ed25519AuthorityId;

// Note this is the URL for the telemetry server
const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

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
							ed25519::Pair::from_seed(b"Alice                           ")
								.public()
								.into(),
						],
						vec![
							ed25519::Pair::from_seed(b"Alice                           ")
								.public()
								.0
								.into(),
						],
						ed25519::Pair::from_seed(b"Alice                           ")
							.public()
							.0
							.into(),
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
							ed25519::Pair::from_seed(b"Alice                           ")
								.public()
								.into(),
							ed25519::Pair::from_seed(b"Bob                             ")
								.public()
								.into(),
						],
						vec![
							ed25519::Pair::from_seed(b"Alice                           ")
								.public()
								.0
								.into(),
							ed25519::Pair::from_seed(b"Bob                             ")
								.public()
								.0
								.into(),
							ed25519::Pair::from_seed(b"Charlie                         ")
								.public()
								.0
								.into(),
							ed25519::Pair::from_seed(b"Dave                            ")
								.public()
								.0
								.into(),
							ed25519::Pair::from_seed(b"Eve                             ")
								.public()
								.0
								.into(),
							ed25519::Pair::from_seed(b"Ferdie                          ")
								.public()
								.0
								.into(),
						],
						ed25519::Pair::from_seed(b"Alice                           ")
							.public()
							.0
							.into(),
					)
				},
				vec![],
				None,
				None,
				None,
				None,
			),
			Alternative::Edgeware => {
				match ChainSpec::from_json_file(std::path::PathBuf::from("edgeware_testnet.json")) {
					Ok(spec) => spec,
					Err(_) => panic!(),
				}
			}
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
	initial_authorities: Vec<SessionKey>,
	endowed_accounts: Vec<AccountId>,
	upgrade_key: AccountId,
) -> GenesisConfig {
	GenesisConfig {
		consensus: Some(ConsensusConfig {
			code: include_bytes!("../runtime/wasm/target/wasm32-unknown-unknown/release/edgeware_runtime.compact.wasm").to_vec(),
			authorities: initial_authorities.clone(),
		}),
		system: None,
		timestamp: Some(TimestampConfig {
			period: 5,					// 5 second block time.
		}),
		balances: Some(BalancesConfig {
			transaction_base_fee: 1,
			transaction_byte_fee: 0,
			existential_deposit: 500,
			transfer_fee: 0,
			creation_fee: 0,
			reclaim_rebate: 0,
			balances: endowed_accounts.iter().map(|&k|(k, (1 << 60))).collect(),
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
			key: upgrade_key,
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
			expiration_time: 10000,
			claims_issuers: initial_authorities.iter().cloned().map(Into::into).collect(),
		}),
		governance: Some(GovernanceConfig {
			voting_time: 10000,
		}),
	}
}
