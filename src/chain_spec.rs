use primitives::{AuthorityId, ed25519};
use edgeware_runtime::{
	AccountId,
	GenesisConfig,
	ConsensusConfig,
  SessionConfig,
	TimestampConfig,
	BalancesConfig,
	UpgradeKeyConfig,
	IdentityConfig,
};
use substrate_service;

// Note this is the URL for the telemetry server
//const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

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
				|| testnet_genesis(vec![
					ed25519::Pair::from_seed(b"Alice                           ").public().into(),
				], vec![
					ed25519::Pair::from_seed(b"Alice                           ").public().0.into(),
				],
					ed25519::Pair::from_seed(b"Alice                           ").public().0.into()
				),
				vec![],
				None,
				None,
				None,
        None
			),
			Alternative::LocalTestnet => ChainSpec::from_genesis(
				"Local Testnet",
				"local_testnet",
				|| testnet_genesis(vec![
					ed25519::Pair::from_seed(b"Alice                           ").public().into(),
					ed25519::Pair::from_seed(b"Bob                             ").public().into(),
				], vec![
					ed25519::Pair::from_seed(b"Alice                           ").public().0.into(),
					ed25519::Pair::from_seed(b"Bob                             ").public().0.into(),
					ed25519::Pair::from_seed(b"Charlie                         ").public().0.into(),
					ed25519::Pair::from_seed(b"Dave                            ").public().0.into(),
					ed25519::Pair::from_seed(b"Eve                             ").public().0.into(),
					ed25519::Pair::from_seed(b"Ferdie                          ").public().0.into(),
				],
					ed25519::Pair::from_seed(b"Alice                           ").public().0.into()
				),
				vec![],
				None,
				None,
				None,
        None
			),
            Alternative::Edgeware => {
                // Read LockDrop data in from local JSON file
                let mut file = File::open("lockdrop.json").unwrap();
                let mut data = String::new();
                file.read_to_string(&mut data).unwrap();

                // Read data from temp file generated from LockDrop data
                let json = Json::from_str(&data).unwrap();
                let authorities: Vec<AuthorityId> = json.find_path(&["Authorities"]).unwrap();
                let lockers: Vec<AuthorityId> = json.find_path(&["LockDroppers"]).unwrap();
                let upgrading_key = AccountId = json.find_path(&["UpgradeKey"]).unwrap();

                // Create chainspec using LockDrop data
                return ChainSpec::from_genesis(
                    "Edgeware Testnet",
                    "edgeware_testnet",
                    || testnet_genesis(
                        authorities,
                        lock,
                        upgrading_key,
                        vec![],
                        None,
                        None,
                        None,
                        None
                    )
                );
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

fn testnet_genesis(initial_authorities: Vec<AuthorityId>, endowed_accounts: Vec<AccountId>, upgrade_key: AccountId) -> GenesisConfig {
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
		upgrade_key: Some(UpgradeKeyConfig {
			key: upgrade_key,
		}),
		identity: Some(IdentityConfig {
			claims_issuers: initial_authorities.iter().cloned().map(Into::into).collect(),
		}),
	}
}
