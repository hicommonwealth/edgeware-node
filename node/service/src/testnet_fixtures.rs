use edgeware_primitives::{Balance, AuraId, AccountId};
use grandpa::AuthorityId as GrandpaId;
use im_online::ed25519::{AuthorityId as ImOnlineId};
use primitives::crypto::UncheckedInto;
use hex_literal::hex;

pub fn get_testnet_bootnodes() -> Vec<String> {
	return vec![
		"/ip4/149.28.199.42/tcp/30333/p2p/QmWq6BWLR2zguEasWueka31gjo5PvKLDXWBh9YVE5HmA1r".to_string(),
		"/ip4/155.138.142.114/tcp/30333/p2p/QmfSdJ5Ke4XUQkL6L2u5Eq7FLR7xkmeW7ywh6xiJAzfyFT".to_string(),
		"/ip4/209.250.238.44/tcp/30333/p2p/QmNrbVewm9tRFobwVsPdXmEz27CSGSGWE8LLrZTpV5ukbs".to_string(),
		"/ip4/45.76.158.163/tcp/30333/p2p/QmQWCzFd8HuPYxrVCb7T4oV4m4qEtanMSJw4a5N6fXGYfn".to_string(),
	];
}

pub fn get_cw_testnet_validators() -> Vec<(AccountId, AccountId, AuraId, Balance, GrandpaId, ImOnlineId)> {
	return vec![(
		// 5CqbHumekpSu9k4D2gYbixZ1pxM8EQXnBAnUu4t9LFXvHL7p
		hex!["223bf1b243fc526133c9bcdcce87edb097c5c31d5f3b6832ef9ec91e44c5d311"].unchecked_into(),
		// 5DFY8ZiwBYwtgWMASHK8PxKXGbKHDrdCfZSKzPwuiRpqpKdW
		hex!["347f31488e541b855b2d19f063eb5b596c33031363e2753643be7b012f3dab67"].unchecked_into(),
		// 5DTE8SBbXhKA8tReeCHAHprq6PumGn6UiBmyTxukrMixnBVf
		hex!["3d698952bf4bbe590ee52ffa2896a3e04472ea5328dea40dbb1f067fe2c19650"].unchecked_into(),
		50000000000000000000000000,
		// 5CxgEhQCzpy7GEAkKue2RZPZWtUNt541D3ZRQN2E65Ftgbpw
		hex!["27a34e94f043cf72613cb737f2c0c478d310ab9c41ef2db43f1a393f0ccc71d2"].unchecked_into(),
		// 5ENh5FVQr1b4zHyUJjEo6QBQL83HGLDbAvAGEr4UBV7nPGEM
		hex!["66303f85a44f317594135213f22db0bfefdb508f0ddcb3351bbe16eeb899d592"].unchecked_into(),
	), (
		// 5Ci7MNXJcCdtkDCtjq2iWYWMb7NVfKzW16zF5rjzdjkeWNKJ
		hex!["1c872bb879b438b7e92b697e70b4527aa9aa9a06478fad78cd2d338bcfdb8f67"].unchecked_into(),
		// 5HL5cdL8Ejabwx2A4zrQYZuRADRkLkUdedSJA9er6vXQj8Qu
		hex!["e8e7178afeb2b15f1df951480755338fa756c2aa62b56935e43337809daf1326"].unchecked_into(),
		// 5DVfoQALZWpEJWHsBTdmV9jTqGnFXT7TjV9KZz6Lm8XJUgvr
		hex!["3f46723da30f5ea31daa5e2116e6cc3a4ba203622a73a803dffcc461dc72d305"].unchecked_into(),
		50000000000000000000000000,
		// 5DRkx4UHvCnacEC7REG6F6jWLDzPkR2CbdgUPjhzLDCdTPyc
		hex!["3c4acc7d6e9032d20dd3e114b0ee6c49e2de5cee3227062e628290f1257095f4"].unchecked_into(),
		// 5ERH4CmPBmap93sM9sQmcXNBXxusVnfNuGbAuvZd4TtVM7iX
		hex!["682921e387d8fad84803752d78086bc8d1ea386ec1d1efb1815775bcceb3a8f2"].unchecked_into(),
	), (
		// 5DXB33t37BMNnv5y6urwvUqedqZL7nkmk2SPFqWvbrAep2T6
		hex!["406c1b4b6fc4802fc06d13f998a4df9f2d369e684a9451eff5da08a64cc8944d"].unchecked_into(),
		// 5EM67CZKnR1Xc8kSXjvnqMK3jnctMsF83tm7rhR5f3cXeGuJ
		hex!["64f74b9d6aa94598b9451ab104a83d9c8a7b845a0f09d236ddbd6dcc4e8f9730"].unchecked_into(),
		// 5FahewhFnPa1En4bqjsJatXJhUPyvjH6FvUE5vbbaPsA7SVd
		hex!["9b9561c95a1c635e7058ef4332542a9e5e7a92da6ae27abde15963ce1a3ee0c1"].unchecked_into(),
		50000000000000000000000000,
		// 5FwxeG1B1ARF4y17rwz36BsazXz2PAmJXxXdxX1G2gbvD8Z3
		hex!["abcbfd813af3a5786b50c221e220d2f794e7c022a89c73c5968f8d7c0934d796"].unchecked_into(),
		// 5DY6oVsSUCYygHMT2NuUPUjE88vvgvipaXExpAwqnpTsUNuh
		hex!["412119abb2e0907b2f39a7def07883ffe679927905022185cd6226d51dc9c4a7"].unchecked_into(),
	), (
		// 5FKqffNYhKj8fgcBxsBPZpDjuLbTS25D7bkjgxreYcK8Hzd7
		hex!["903fac4b05bc58fc372ce151fe6e491e00683f0f016f385b76e7622a3f02a80a"].unchecked_into(),
		// 5EZrHHsNX2GnR3Cmz9CTpgsnPmqy1BHpjtNkUFzsmgZ1PNG7
		hex!["6eb2f1717997e9dc2bc7c84db096283f3503640d27071e482312ebb4d78f3119"].unchecked_into(),
		// 5EfM5vSEJ53hC4CreJgKLMAZcfmQQQYfZLhzsDbU2KXhGyif
		hex!["72e4220ace25b370e7cbbdcd6e93d7ecde672de3b8cdb0309dda62d52f1f7bfd"].unchecked_into(),
		50000000000000000000000000,
		// 5DHBUhpGNE7fvfzCLuGxiUPXbMwDZjKapZFXqDkdcEaDvk6A
		hex!["35c028efeec43254b2a053e6ee4938e8bfba7d0b3ea9e74a8d53003e8e6d6de8"].unchecked_into(),
		// 5D1AVCitGHgmGr8ejeZEWkCSBHBKWUzrEVaJBeQ5Zm7vj6b7
		hex!["2988e564b9dcb6792adf3063bfd3434b3d6b0ea6375ee5e44e4944ee2b05bcec"].unchecked_into(),
	)];
}

pub fn get_testnet_identity_verifiers() -> Vec<AccountId> {
	return vec![
		// 5FC2u6RCD2j61kDDVJp2pCnJN1946uxyGuZDhUR9htmaDmf5
		hex!["8a4b84c72992c08895cab8f3583f3c13c556ab58e9bbceb6c7f6910221196b78"].unchecked_into(),
	];
}

pub fn get_testnet_root_key() -> AccountId {
	// 5G8jA2TLTQqnofx2jCE1MAtaZNqnJf1ujv7LdZBv2LGznJE2
	return hex!["b4024a048721ca8e7e2d342193bde1e80a269c0ddcf4fedd65303fc7289a1905"].unchecked_into();
}

pub fn get_testnet_election_members() -> Vec<AccountId> {
	return vec![
		// 5EeJqpx6RCQxg13WW2WJt4CPE6w6vSmFSWNBdzYCh2YX7bFU
		hex!["72195640f79f8254ce35db3b5d0b17c0243b0fb4489fa4b04688ed121ba22603"].unchecked_into(),
	];
}
