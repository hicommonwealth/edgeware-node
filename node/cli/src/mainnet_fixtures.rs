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

//! Mainnet fixtures

use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_aura::ed25519::AuthorityId as AuraId;
use edgeware_primitives::{AccountId, Balance};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use pallet_im_online::ed25519::{AuthorityId as ImOnlineId};
use sp_core::crypto::UncheckedInto;
use hex_literal::hex;

/// Bootnodes for mainnet launch
pub fn get_mainnet_bootnodes() -> Vec<String> {
	return vec![
		"/ip4/144.202.61.115/tcp/30333/p2p/QmXTb6R2AvA6FrvD4w2YRD2oj9WQk2f9Dg1dTqGsdxgwuD".to_string(),
		"/ip4/107.191.48.39/tcp/30333/p2p/QmdFq4WXvgokUi5MAcGvzcV4PZmo6fZN2fxcEbcPQioGcK".to_string(),
		"/ip4/66.42.113.164/tcp/30333/p2p/Qmawkfqh4y4vnPWiy87pBnWpgsyy8QrQmUFprDTktgatSm".to_string(),
		"/ip4/144.202.58.79/tcp/30333/p2p/QmXWhRta7P3xW43WbJ6CDH9ZsHwVxFhLJNjpBa6J3jaAqj".to_string(),
		"/ip4/207.148.13.203/tcp/30333/p2p/QmRgKnmZNYVCznVd4ao5UHCHGWieT3sePB5g8v7PSGofD2".to_string(),
		"/ip4/207.148.11.222/tcp/30333/p2p/QmbzrqjbDcwhhX1oiKndxTjK1ULjqVw36QvrEuRKSZjgLY".to_string(),
		"/ip4/149.28.120.45/tcp/30333/p2p/QmfB4F7TeUcuZZ4AMT3nvvfPVME4eWyJUUdWkXeus3AThe".to_string(),
		"/ip4/149.28.115.253/tcp/30333/p2p/QmQvAPW1bBpx5N7YJLcBhHNqANw4dxVmBTiJNeuC8FoYeR".to_string(),
		"/ip4/66.42.116.197/tcp/30333/p2p/QmU1g7NFj1cd46T69ZXig9c7Xc6RLGwjZm4Ur6d4JPBDh2".to_string(),
		"/ip4/104.207.139.151/tcp/30333/p2p/QmPuU4VY2nckAodyWXv3VyCwavk5FF9yqVWB4G1LtNf9v9".to_string(),
		"/ip4/45.77.238.189/tcp/30333/p2p/Qme2d6D8WGGhsymJLmhBwEadW8sFhXZZ9HKEzhbueunTZc".to_string(),
		"/ip4/209.250.227.147/tcp/30333/p2p/QmfWEPyZmrZeS6dgsi7hjvKX1u6AxGGrvEH6BZsXBe68Eo".to_string(),
		"/ip4/202.182.103.213/tcp/30333/p2p/QmTxiQDFQykWEWpywtPiNDJK8EFnGumSYHJDuUJ8uhBWDu".to_string(),
		"/ip4/207.148.77.158/tcp/30333/p2p/Qmcswgc3oYLdyWZmTo9Kf3pr3fnLdaA34fpJRPtYTKwpPS".to_string(),
		"/ip4/140.82.54.194/tcp/30333/p2p/QmTzLQJn7MCkJZEcucSgoBnUgzF85YC5RK5wRq7oRupSFR".to_string(),
		"/ip4/155.138.133.37/tcp/30333/p2p/QmYJtBiHi9s6nghnW4PqgKAab4Gq6BQikmR66rk5yoiY9f".to_string(),
		"/ip4/45.32.171.175/tcp/30333/p2p/QmYi3K6422sZ9RqNjcCJ4s3DoUdyK6C2U9RDH4kwQC1KXB".to_string(),
		"/ip4/45.77.105.248/tcp/30333/p2p/QmQzjXREKk4QbfrkwtRsEEdrQA85k1ERvcUPvTaSC2FTnX".to_string(),
		"/ip4/144.202.19.214/tcp/30333/p2p/QmRybov4ZYCcgm6QyRkH2TgSjRngQuL4m3td5MQxmgqWHu".to_string(),
		"/ip4/207.246.98.108/tcp/30333/p2p/QmfH6NCWUcRwfVcJsDocNF5ZUpGUex8xWba64rHvomyweV".to_string(),
	];
}


/// Split endowment amount for Commonwealth
pub const COMMONWEALTH_ENDOWMENT: Balance = 75_000_000_000_000_000_000_000_000;
/// Split endowment amount for stash
pub const STASH_ENDOWMENT: Balance = 10_000_000_000_000_000_000;
/// Split endowment amount for controllers
pub const CONTROLLER_ENDOWMENT: Balance = 10_000_000_000_000_000_000;
/// Genesis allocation that will fit into the "balances" module for Commonwealth/Founders
pub fn get_commonwealth_allocation() -> Vec<(AccountId, Balance)> {
	return vec![(
		hex!["14ad3d151938d63a4e02454f034a3158c719ed9de2e233dd0843c2d81ddba53d"].into(),
		5_500_000_000_000_000_000_000_000,
	), (
		hex!["12d490251399a081935bf731184d2bf37d228bc38d3d68a8e3822933bcf23a09"].into(),
		5_500_000_000_000_000_000_000_000,
	), (
		hex!["a87d1f2e04d8e95499f8a6f18214355bcb2fd2c9370ab5c19f379dd9d3167075"].into(),
		5_500_000_000_000_000_000_000_000,
	), (
		hex!["4cb0922d0fb5217553da0da70bd4076812ad2a5cce860524ff7b5e6e3629f962"].into(),
		3_000_000_000_000_000_000_000_000,
	), (
		hex!["78040adec849fff1c66c16ab8ac1534ed27e37a8a1da8aa3239267a883369566"].into(),
		1_500_000_000_000_000_000_000_000,
	), (
		hex!["cc3d67fe87c81b5895ed89cfb1c44cc29c3798bac93368487dfc11364d6e3068"].into(),
		COMMONWEALTH_ENDOWMENT,
	), (
		hex!["eeb7482b9cce124538b1aeea1a7935d313b9f01cc6192fb4cc6bdf1b0f6b4430"].into(),
		COMMONWEALTH_ENDOWMENT,
	), (
		hex!["765e19400b3f7d44e5677d24d9914ae8cabb1bf3ef81ebc1ca72ad99d312af46"].into(),
		COMMONWEALTH_ENDOWMENT,
	), (
		hex!["ca91588bb9258ade926d0c0631798d7e3f17c4581fae56283287d54883244a55"].into(),
		CONTROLLER_ENDOWMENT,
	), (
		hex!["1ec5e3d9a77ac81d6da0290c04d003bbcb04af8c4902bd59dbf9be4dfa47234f"].into(),
		STASH_ENDOWMENT,
	), (
		hex!["d6fb4a2f0d5dfc62c37a09e6aac5c3ea4ce2ba021f553c940e63d59dadf0cd24"].into(),
		CONTROLLER_ENDOWMENT,
	), (
		hex!["720967cda4c9097924d705695b62dfb6dc6dbeade65b5575abf5c4ca38e50503"].into(),
		STASH_ENDOWMENT,
	), (
		hex!["38a58e82baf9df6ec1f9a7064a337f872778649f3dd9002e3fe48df94b475232"].into(),
		CONTROLLER_ENDOWMENT,
	), (
		hex!["de90c8b070c0a63fbf52655af7492dc8e7d985334a4c60c02bc2f59424ff1430"].into(),
		STASH_ENDOWMENT,
	), (
		hex!["0e33e22cd22b272f388bcd41f13942d803089106ec350b8754e1d290ee6ff52b"].into(),
		CONTROLLER_ENDOWMENT,
	), (
		hex!["9665bd715c72b686c2557fe11e6cd54924adef62dc1f52cf43a503f363cf843c"].into(),
		STASH_ENDOWMENT,
	), (
		hex!["6aa8f0dd6b6221788d68bf2486126fb14bb65ea710028c11f7ca131e0df10707"].into(),
		CONTROLLER_ENDOWMENT,
	), (
		hex!["464c96a206e310511a27acc92b2e410a14bd83cb8788b522d0cee03f0d285862"].into(),
		STASH_ENDOWMENT,
	), (
		hex!["ae5bfe517affa6f7456ad6b9f7465520059e6d7b2a8928673460461abb741c18"].into(),
		CONTROLLER_ENDOWMENT,
	), (
		hex!["34c71b1e42e910b94b8cbb2c960873bd4bf0db6e80afdf41cdc52acd91d6393f"].into(),
		STASH_ENDOWMENT,
	), (
		hex!["6a782c02fd24ed538224f3d0bda56146bc6bacd34f9a784c1b5367e19cda456e"].into(),
		CONTROLLER_ENDOWMENT,
	), (
		hex!["d02002915139ac3e4552c5006f92cccfbf8b02cb4d4ca1993a69d63368cc1f0c"].into(),
		STASH_ENDOWMENT,
	), (
		hex!["4864744ab79cd62cbc1094da8e6f54aba8cba7ed6d616bdd8df10572d146c15c"].into(),
		CONTROLLER_ENDOWMENT,
	), (
		hex!["143f9f9019fa62919ed6da39b8f147cb52501438e1e9e7a82d22d7b49df18e59"].into(),
		STASH_ENDOWMENT,
	), (
		hex!["a01bef9c8591ae4155f9483eee09db092fb39bdd580e3715e927333e4aa89d6d"].into(),
		CONTROLLER_ENDOWMENT,
	), (
		hex!["4e7de9c8f3564fe5cc5057de51c41b40a7da801d22c6ee5aa57f8bb2838ae857"].into(),
		STASH_ENDOWMENT,
	), (
		hex!["00e5a14e08930f94148569274ca1e9355938fabf65fffef3b7cb3c3e3edabb23"].into(),
		CONTROLLER_ENDOWMENT,
	), (
		hex!["ce64070e4dffe61183241dca3e922b65ecd509430a3e283fab5c143532f79d3e"].into(),
		STASH_ENDOWMENT,
	), (
		hex!["b0492fa7ac84ecb20f9f69e1c328b521fce8f472af2cc13784286d2240e4924c"].into(),
		CONTROLLER_ENDOWMENT,
	), (
		hex!["58e8d8750021d11f5bf1106966235ed293a4288511016af7f3b2e81a84ead342"].into(),
		92_500_000_000_000_000_000_000_000,
	), (
		hex!["688421b084a363cb394c5e3a7c79f44482bf2f15f6d86ea37ae110a3af238f07"].into(),
		10_000_000_000_000_000_000_000_000,
	), (
		hex!["765169492c492ee29f2af9af46f9e1b117aa0283b73a4361ae12ace1c41a6c72"].into(),
		150_000_000_000_000_000_000_000_000,
	), (
		hex!["6490626e3bde470c449e90b83df92ddb8514f02067a0ddd66f1080b5033dec2d"].into(),
		1474_790_000_000_000_000_002_506,
	), (
		hex!["ec80b8b78a2b283f0a48712c8446241cf5f36d2f480559cdc73253981963f402"].into(),
		25_000_000_000_000_000_000_000,
	)]
}

/// 5 EDG stake amount for Commonwealth validators
pub const STAKED_ENDOWMENT: Balance = 5_000_000_000_000_000_000;
/// The mainnet commonwealth validator pubkeys and staked balances. These
/// staked amounts should be less than the balances of the stash accounts.
pub fn get_cw_mainnet_validators() -> Vec<(AccountId, AccountId, Balance, AuraId, GrandpaId, ImOnlineId, AuthorityDiscoveryId)> {
	return vec![(
		hex!["1ec5e3d9a77ac81d6da0290c04d003bbcb04af8c4902bd59dbf9be4dfa47234f"].into(),
		hex!["d6fb4a2f0d5dfc62c37a09e6aac5c3ea4ce2ba021f553c940e63d59dadf0cd24"].into(),
		STAKED_ENDOWMENT,
		hex!["532cdeaeb19afd03eb4a57d4dddad09268fb720478d5386263330f5bf86f1cc4"].unchecked_into(),
		hex!["ae5e4e8c3bbb47737a2b4abe79a71289f60271a94a5eaebf6b200f638fcbe332"].unchecked_into(),
		hex!["532cdeaeb19afd03eb4a57d4dddad09268fb720478d5386263330f5bf86f1cc4"].unchecked_into(),
		hex!["5e0755efa5da8b2bb83d2443635268b0be48ba020587fdc3731a5b87b51ff500"].unchecked_into(),
	), (
		hex!["720967cda4c9097924d705695b62dfb6dc6dbeade65b5575abf5c4ca38e50503"].into(),
		hex!["38a58e82baf9df6ec1f9a7064a337f872778649f3dd9002e3fe48df94b475232"].into(),
		STAKED_ENDOWMENT,
		hex!["89d7a1fb903a63494696c6d10d76704da7760da7f32dc3b5aa122bfba3f85680"].unchecked_into(),
		hex!["f272b4cdec7e979dd76c7860f7e9dd8cb1da974d74366a266c4de501b2079896"].unchecked_into(),
		hex!["89d7a1fb903a63494696c6d10d76704da7760da7f32dc3b5aa122bfba3f85680"].unchecked_into(),
		hex!["fea24c3e2b57972eead003295fcfc52fb57ffd1bdfedf1dcfb1cd35df11dcc37"].unchecked_into(),
	), (
		hex!["de90c8b070c0a63fbf52655af7492dc8e7d985334a4c60c02bc2f59424ff1430"].into(),
		hex!["0e33e22cd22b272f388bcd41f13942d803089106ec350b8754e1d290ee6ff52b"].into(),
		STAKED_ENDOWMENT,
		hex!["98a4f5d4b363447331f54328d83816f773e30799be5979c3d6d9be08f4941799"].unchecked_into(),
		hex!["8634c736497e74b1cedbecec7efe94bdff2ec899f60c97e003e8123f366e14e3"].unchecked_into(),
		hex!["98a4f5d4b363447331f54328d83816f773e30799be5979c3d6d9be08f4941799"].unchecked_into(),
		hex!["bce40bb0b2649d3fa924e050c603dccb5b7468f89924158e9b6d4d048c79dc23"].unchecked_into(),
	), (
		hex!["9665bd715c72b686c2557fe11e6cd54924adef62dc1f52cf43a503f363cf843c"].into(),
		hex!["6aa8f0dd6b6221788d68bf2486126fb14bb65ea710028c11f7ca131e0df10707"].into(),
		STAKED_ENDOWMENT,
		hex!["095ca61a04a9bd2fc95b31c1f86b73ef85e1388e75822896b4079cf5bc1c0e14"].unchecked_into(),
		hex!["98d7399dd06f0192a2eb3096aca6df6f2600634d77b92c926bed92d60c50b75d"].unchecked_into(),
		hex!["095ca61a04a9bd2fc95b31c1f86b73ef85e1388e75822896b4079cf5bc1c0e14"].unchecked_into(),
		hex!["8e84ac19afbba97a686f1b20a3288610f76177b3c71cdc1f31df0828ba6acd1c"].unchecked_into(),
	), (
		hex!["464c96a206e310511a27acc92b2e410a14bd83cb8788b522d0cee03f0d285862"].into(),
		hex!["ae5bfe517affa6f7456ad6b9f7465520059e6d7b2a8928673460461abb741c18"].into(),
		STAKED_ENDOWMENT,
		hex!["76c5e8b1a4656cf7f174662674c61cb3a8fb67cee15d4f9a85f25235653c2a76"].unchecked_into(),
		hex!["08b8316bbc960b4e3d610724e4f330556356b567fac3a7cb63fe458dbafdf028"].unchecked_into(),
		hex!["76c5e8b1a4656cf7f174662674c61cb3a8fb67cee15d4f9a85f25235653c2a76"].unchecked_into(),
		hex!["a69f1ccc84e40afae38d2d2f87b5910b164a8b451a0d70ac51e117f1222ede65"].unchecked_into(),
	), (
		hex!["34c71b1e42e910b94b8cbb2c960873bd4bf0db6e80afdf41cdc52acd91d6393f"].into(),
		hex!["6a782c02fd24ed538224f3d0bda56146bc6bacd34f9a784c1b5367e19cda456e"].into(),
		STAKED_ENDOWMENT,
		hex!["82b6147e4e661551609f04168599ed213c21fa194aba3327fb2fd6247a52b5d2"].unchecked_into(),
		hex!["996b0ef45bd1f90b17ebc5f038cea21ccc1725a2c1daf3a85e774e22e173bb6e"].unchecked_into(),
		hex!["82b6147e4e661551609f04168599ed213c21fa194aba3327fb2fd6247a52b5d2"].unchecked_into(),
		hex!["bef61229205f6027e92e70bcd6b01e352c21d795e38dd64cf87335fd214d994b"].unchecked_into(),
	), (
		hex!["d02002915139ac3e4552c5006f92cccfbf8b02cb4d4ca1993a69d63368cc1f0c"].into(),
		hex!["4864744ab79cd62cbc1094da8e6f54aba8cba7ed6d616bdd8df10572d146c15c"].into(),
		STAKED_ENDOWMENT,
		hex!["3cf4ee0a14ea22e82a8953d60fc68f80a62d881e0b56c97445b3ea96adc4d31c"].unchecked_into(),
		hex!["2f9a90f811621556d0dd1bdf324d0154444b34a0501a9f5bd338cf41d94634f9"].unchecked_into(),
		hex!["3cf4ee0a14ea22e82a8953d60fc68f80a62d881e0b56c97445b3ea96adc4d31c"].unchecked_into(),
		hex!["4e85c095e94a47dea48c7c6824d3f4818e1c34df1e00fe658d650845c275e13e"].unchecked_into(),
	), (
		hex!["143f9f9019fa62919ed6da39b8f147cb52501438e1e9e7a82d22d7b49df18e59"].into(),
		hex!["a01bef9c8591ae4155f9483eee09db092fb39bdd580e3715e927333e4aa89d6d"].into(),
		STAKED_ENDOWMENT,
		hex!["191decd2b0b7e447a2009d0c8f963b118ee7781adada0e217273ac924514b3a8"].unchecked_into(),
		hex!["a81b12f94160734c3e53ac961a73c796783be50225e4e9b028e8318654f2d876"].unchecked_into(),
		hex!["191decd2b0b7e447a2009d0c8f963b118ee7781adada0e217273ac924514b3a8"].unchecked_into(),
		hex!["38fc0932145f659c13d14d4be0215b3a811738bf713c771b980032d66d10d567"].unchecked_into(),
	), (
		hex!["4e7de9c8f3564fe5cc5057de51c41b40a7da801d22c6ee5aa57f8bb2838ae857"].into(),
		hex!["00e5a14e08930f94148569274ca1e9355938fabf65fffef3b7cb3c3e3edabb23"].into(),
		STAKED_ENDOWMENT,
		hex!["d76fca327e6b6c91c220acbe0769d16ede7c96578e7743036a784fe3d528d40d"].unchecked_into(),
		hex!["da5bcd957592d12041bb9777605b3e3aeeac7712c2ba2339a80e33acfc5cf07e"].unchecked_into(),
		hex!["d76fca327e6b6c91c220acbe0769d16ede7c96578e7743036a784fe3d528d40d"].unchecked_into(),
		hex!["e04db33479ca34e1ee304db70edf95b37427c2350b14eff984bccc4d07e8876a"].unchecked_into(),
	), (
		hex!["ce64070e4dffe61183241dca3e922b65ecd509430a3e283fab5c143532f79d3e"].into(),
		hex!["b0492fa7ac84ecb20f9f69e1c328b521fce8f472af2cc13784286d2240e4924c"].into(),
		STAKED_ENDOWMENT,
		hex!["5342c923d5c187f4417862556555ee09475c11141cbc0103272b826e0f8cd0b9"].unchecked_into(),
		hex!["a7a35e31a1b49a5ced7f4f6ef214da56318e7ca0dfad50274dbcb88456f35621"].unchecked_into(),
		hex!["5342c923d5c187f4417862556555ee09475c11141cbc0103272b826e0f8cd0b9"].unchecked_into(),
		hex!["504e8b32ad648f3bfaf186840421b7717af828b77b2d512a30dd6cd62060401e"].unchecked_into(),
	)];
}

/// Commonwealth election member
pub fn get_mainnet_election_members() -> Vec<AccountId> {
	return vec![
		hex!["ca91588bb9258ade926d0c0631798d7e3f17c4581fae56283287d54883244a55"].into(),
	];
}

/// Mainnet root key
pub fn get_mainnet_root_key() -> AccountId {
	return hex!["0000000000000000000000000000000000000000000000000000000000000000"].into();
}
