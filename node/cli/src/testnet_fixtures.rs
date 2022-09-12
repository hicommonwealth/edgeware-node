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

//! Testnet fixtures

use edgeware_primitives::AccountId;
use pallet_im_online::ed25519::AuthorityId as ImOnlineId;
use sc_network::config::MultiaddrWithPeerId;
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_aura::ed25519::AuthorityId as AuraId;
use sp_core::crypto::Ss58Codec;
use sp_finality_grandpa::AuthorityId as GrandpaId;

/// Testnet root key
pub fn get_testnet_root_key() -> AccountId {
	AccountId::from_ss58check("5HbAMQtwKrtUwAx3B22MGxBWpyLeWmEDhBuhXfVLwjAzUPpa").unwrap()
}

/// Beresheet bootnodes
pub fn get_beresheet_bootnodes() -> Vec<MultiaddrWithPeerId> {
	return vec![
		"/dns/beresheet-boot0.jelliedowl.com/tcp/30833/p2p/12D3KooWCD6gCJ4ftNEuxFcKozc2vHBHTvb6JG2bYU6aByefXN5F".parse().unwrap(),
		"/dns/beresheet-boot1.jelliedowl.com/tcp/30833/p2p/12D3KooWAVYNwBrueKcmjksYeXMdQaG75Nd3RUfiz6t1Liw2JQeg".parse().unwrap(),
		"/dns/beresheet-boot2.jelliedowl.com/tcp/30833/p2p/12D3KooWLr5tSkZvs8QB1u36P9LGDstW9qaDV27QFYnqWWf2E4wH".parse().unwrap(),
		"/dns/beresheet-boot3.jelliedowl.com/tcp/30833/p2p/12D3KooWR9Ghmp3q1WCoWTp1TWWXKyjzHvkbAmAxa45Vsi8ikTAU".parse().unwrap(),
		"/dns/beresheet-boot4.jelliedowl.com/tcp/30833/p2p/12D3KooWBAq7Yim8LraTrpaPa8SdU9nHnMyBGt37sRWT6GPwct2X".parse().unwrap(),
	];
}

/// Beresheet initial authorities
pub fn get_beresheet_initial_authorities() -> Vec<(
	AccountId,
	AccountId,
	GrandpaId,
	AuraId,
	ImOnlineId,
	AuthorityDiscoveryId,
)> {
	vec![
		(
			AccountId::from_ss58check("5CMNamTiTYPkkv68w1GLaHPVty8EJWvkg9XMwrWRfTSdHhcU").unwrap(),
			AccountId::from_ss58check("5CMNamTiTYPkkv68w1GLaHPVty8EJWvkg9XMwrWRfTSdHhcU").unwrap(),
			GrandpaId::from_ss58check("5FURkcLw91AeCj9P3ZDj5isM7RFN3byqEhvT2HcSBoirZWk8").unwrap(),
			AuraId::from_ss58check("5GEpMSrNg5AH1tv7Mw5H7rRx4b2eygkevTL1dysm1ZATymTJ").unwrap(),
			ImOnlineId::from_ss58check("5FbVL6FUBrRYTrETYQuMQffdCYQ6HsDcpK5skgnL6qxKAonZ").unwrap(),
			AuthorityDiscoveryId::from_ss58check("5HEg7w5eh568aR4ysw7ZdbuEVKLmRtqW7oy23e39qnjhRGsj").unwrap(),
		),
		(
			AccountId::from_ss58check("5CiP172csqtauU3iN4jPFr4bJQvJLpa8DSN1ggPBJaFzjqaG").unwrap(),
			AccountId::from_ss58check("5CiP172csqtauU3iN4jPFr4bJQvJLpa8DSN1ggPBJaFzjqaG").unwrap(),
			GrandpaId::from_ss58check("5GGtucZrb1ieagbtipYqWckPgSaTpUADbmPFLq1VeaX8m5ce").unwrap(),
			AuraId::from_ss58check("5Fqkra6SSmvRH4vvxaaYaqhYAgekcUJUSVBFPTRz6N7Ka9h4").unwrap(),
			ImOnlineId::from_ss58check("5HQ5K2YCqedhRdxhAzBWvxSgYLjbfBWJWqfcQjcX2QyT21bi").unwrap(),
			AuthorityDiscoveryId::from_ss58check("5HJyS6p1vUjYubWYjyzb4UzFYPkWiz9ST7EsXME6xE8zAGLA").unwrap(),
		),
		(
			AccountId::from_ss58check("5GmwvToG6Sk6wsHXkmwD2vqnSBvi1FMVpp2LBw5LowbgeZhz").unwrap(),
			AccountId::from_ss58check("5FHPJFLDvaTc3Xo6Zi5Ee4XRnX5QRt5uwrk3i7QrFbM9tMrz").unwrap(),
			GrandpaId::from_ss58check("5FcSstksh1ENseKBaPAf2s1wggESPWPcBtew4FtDyRJmxfCs").unwrap(),
			AuraId::from_ss58check("5CutFvKrZpbd3SAVHKixrjGTifHgpzRQM79bGj6sPzoLdUV3").unwrap(),
			ImOnlineId::from_ss58check("5ChcbMkL64LEnkCRrTiofpdmTzkSPxuoVkCNHGeYeJyebKyv").unwrap(),
			AuthorityDiscoveryId::from_ss58check("5E2Ae7Fu3z12Vxkiyh3DUtu9PprDhiFrRYhBDP9AjPbxJzmX").unwrap(),
		),
		(
			AccountId::from_ss58check("5EPBXa8MP8oCZsWjRBquHpaGHLfVTZpGhrcP1UjGzjK8rzTn").unwrap(),
			AccountId::from_ss58check("5GeYerKSvhXDbWHFZrgmSLZ8g6mVqrZKxgVb3ePST1US1eB8").unwrap(),
			GrandpaId::from_ss58check("5GCCwo4jZ1CC11bZQAvinp6Wzqz6k3tqNzQoj1ZWe8oLWg8y").unwrap(),
			AuraId::from_ss58check("5FLqf8scff7jeRZLZXTct1RFU9gh3MdczeFwSzsEi936jxJE").unwrap(),
			ImOnlineId::from_ss58check("5EkXZTxi7651ARtDxjJJkNGrBy4asWys9KbCp2JDQLePYXGM").unwrap(),
			AuthorityDiscoveryId::from_ss58check("5Cg1tL4qeMWzLQ4BS7wiy6Sxj7M4HXjHNhXJxkx9ZqPAtZy8").unwrap(),
		),
	]
}
