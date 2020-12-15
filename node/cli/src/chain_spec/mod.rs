use pallet_im_online::ed25519::AuthorityId as ImOnlineId;
use sc_chain_spec::ChainSpecExtension;
use serde::{Deserialize, Serialize};
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_aura::ed25519::AuthorityId as AuraId;
use sp_core::{sr25519, Pair, Public,};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::Verify;
use serde_json::Result;
use std::fs::File;
use std::path::Path;
use edgeware_primitives::Block;

#[cfg(feature = "with-mainnet-runtime")]
pub mod mainnet;
#[cfg(feature = "with-testnet-runtime")]
pub mod testnet;
#[cfg(feature = "with-development-runtime")]
pub mod dev;

pub type AccountPublic = <Signature as Verify>::Signer;

pub const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
pub const DEFAULT_PROTOCOL_ID: &str = "edg";

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
	/// Block numbers with known hashes.
	pub fork_blocks: sc_client_api::ForkBlocks<Block>,
	/// Known bad block hashes.
	pub bad_blocks: sc_client_api::BadBlocks<Block>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Allocation {
	balances: Vec<(String, String)>,
	vesting: Vec<(String, BlockNumber, BlockNumber, String)>,
}

pub fn get_lockdrop_participants_allocation() -> Result<Allocation> {
	let path = Path::new("node/cli/lockdrop/lockdrop-allocation.json");
	let mut file = File::open(&path).unwrap();
	let mut data = String::new();
	file.read_to_string(&mut data).unwrap();
	let a: Allocation = serde_json::from_str(&data)?;
	return Ok(a);
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed
pub fn get_authority_keys_from_seed(seed: &str) -> (
	AccountId,
	AccountId,
	GrandpaId,
	AuraId,
	ImOnlineId,
	AuthorityDiscoveryId,
) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
		get_account_id_from_seed::<sr25519::Public>(seed),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<AuraId>(seed),
		get_from_seed::<ImOnlineId>(seed),
		get_from_seed::<AuthorityDiscoveryId>(seed),
	)
}
