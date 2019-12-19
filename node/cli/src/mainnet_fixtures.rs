use authority_discovery_primitives::AuthorityId as AuthorityDiscoveryId;
use aura_primitives::ed25519::AuthorityId as AuraId;
use edgeware_primitives::{AccountId, Balance};
use grandpa::AuthorityId as GrandpaId;
use im_online::ed25519::{AuthorityId as ImOnlineId};
use primitives::crypto::UncheckedInto;
use hex_literal::hex;

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
	];
}

/// This is the allocation that will fit into the "balances" module
pub const STASH_ENDOWMENT: Balance = 8999990000000000000000000;
pub const CONTROLLER_ENDOWMENT: Balance = 10000000000000000000;
pub fn get_commonwealth_allocation() -> Vec<(AccountId, Balance)> {
	return vec![(
		hex!["1a925251d02fa027882bb1a5834abee8ac4ed06fa735d531cb89ccc478ee5b50"].into(),
		1100000000000000000000000,
	), (
		hex!["5c1aee377fcdced7d6a1d0a1b3f89acfbc0f3c2ed6c4f324434aaf3934a14b4b"].into(),
		1100000000000000000000000,
	), (
		hex!["f67cfe9c4bb11ac43f7926e177af33bff39a095ff344fb2b96f95cf6b648b745"].into(),
		1100000000000000000000000,
	), (
		hex!["ce56ed9d0c61d409610244e91df80be6d29d56f7e8f73a9f8bbb9143e6d3ab2b"].into(),
		1100000000000000000000000,
	), (
		hex!["1280d5fd2fe138d51bdd11f8657ef13261f6623dcc4af20f30eea42b5111340c"].into(),
		1100000000000000000000000,
	), (
		hex!["4cb0922d0fb5217553da0da70bd4076812ad2a5cce860524ff7b5e6e3629f962"].into(),
		3000000000000000000000000,
	), (
		hex!["12d490251399a081935bf731184d2bf37d228bc38d3d68a8e3822933bcf23a09"].into(),
		5500000000000000000000000,
	), (
		hex!["78040adec849fff1c66c16ab8ac1534ed27e37a8a1da8aa3239267a883369566"].into(),
		1500000000000000000000000,
	), (
		hex!["c0a8a737e77f8c6cb62a2ffa8c0b9d8d7191d46d0e09c48c1354dab109ac4c5e"].into(),
		2750000000000000000000000,
	), (
		hex!["f89077e892d9861da018a6cfa4082ccf39cceb83c431cec51b937befb2caa949"].into(),
		2750000000000000000000000,
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
		hex!["c4fff312e8e224c9f1380d235c1a601d9f00213e6b4f4c86250f768563cb6f2a"].into(),
		STASH_ENDOWMENT,
	), (
		hex!["ecd405cf016a268dddeb37db29bc6d59d0a556a6265d27e81557b0766c651128"].into(),
		CONTROLLER_ENDOWMENT,
	), (
		hex!["3a9fcaef453185230c2d777fa7dda16a745f6840b0cbc28f3e8c2ab07e533d3b"].into(),
		STASH_ENDOWMENT,
	), (
		hex!["6ad57998c556494d0ebd1790fdc775a0edc26d54afe7f87e9903a4600d4dac52"].into(),
		CONTROLLER_ENDOWMENT,
	), (
		hex!["e280d0fbfbcbdc070526d0997d0bfec3b3527b036bf5a68f87e6cc0422ced302"].into(),
		STASH_ENDOWMENT,
	), (
		hex!["9a66bd5f4f054e8b9b3dd3feb24d74c9554b8a48574b1d53dd27c01ddfa2b333"].into(),
		CONTROLLER_ENDOWMENT,
	), (
		hex!["865d38926f2344541912b464e2bb910ef72f2c9d447bc26f996b46f67605b85b"].into(),
		STASH_ENDOWMENT,
	), (
		hex!["22b875e383411875a6cd3317e09744e3b97cd1426537e74384759db63bc1437d"].into(),
		CONTROLLER_ENDOWMENT,
	), (
		hex!["6ea38f9aeb405131cfcd8e542d72cd2a3f3e56427b9162298d8e7f3529a6c34e"].into(),
		STASH_ENDOWMENT,
	), (
		hex!["524ad81e880a13d4156dac6aedc1407b9f2be89f0b12323ad75302bfa35c9c4c"].into(),
		CONTROLLER_ENDOWMENT,
	), (
		hex!["968a20a9c04a662831afd93d8ad834f69c6aeb37e9017b4b739fd94502aae543"].into(),
		STASH_ENDOWMENT,
	), (
		hex!["96d03b25c303bb81507c9a875a2d548c0ac329156abc412d466ec24c423c660d"].into(),
		CONTROLLER_ENDOWMENT,
	), (
		hex!["fce2e25267c2f45932ca1cc9834d2db0b5438804af69a8af16bade1ca00c160d"].into(),
		STASH_ENDOWMENT,
	), (
		hex!["7a27bdce4ab2965e81d14ec27191e07b2961ba55e97d19e88ff2e6ee8b0f290a"].into(),
		CONTROLLER_ENDOWMENT,
	), (
		hex!["86cdb7761daa3c8917719b057e83ca0377de8ca599624c4ff7bd541ab056427d"].into(),
		STASH_ENDOWMENT,
	), (
		hex!["8078b8d716f6a968185c80fa463e14d32f863db448e8dc84542ab34a20260706"].into(),
		CONTROLLER_ENDOWMENT,
	), (
		hex!["dc1ec6087624728ec118c4b4cd8e58bfe8c977195aac172473262242fba5d160"].into(),
		STASH_ENDOWMENT,
	), (
		hex!["4892baad519848ce7b030b3c029560b0a682d18ba5bde78d37a3ffff2120a452"].into(),
		CONTROLLER_ENDOWMENT,
	), (
		hex!["a0c53fe2dd2bdcb3b3da9612b5a185243057ae81a7e6092a0788672cc99f8b72"].into(),
		STASH_ENDOWMENT,
	), (
		hex!["f8c36b94295a2ad78361d61a485be3e40184c2304d5bfa15b31de3a4087d971d"].into(),
		CONTROLLER_ENDOWMENT,
	), (
		hex!["3401ae0dc64bf4bae104413d1e68380cd8f9f75b753b870b39352bc25e05c865"].into(),
		STASH_ENDOWMENT + CONTROLLER_ENDOWMENT,
	),(
		hex!["62f118b23e00a02f0e64b2cc5491226c24b7781fd60ca7ac330011173f6d4b68"].into(),
		STASH_ENDOWMENT + CONTROLLER_ENDOWMENT,
	), (
		hex!["0211cc0925ac6dd2d14fbdb63766d235d2a3c159d4c1dd3a260f3f179b05ce41"].into(),
		STASH_ENDOWMENT + CONTROLLER_ENDOWMENT,
	), (
		hex!["d6a6eb11c5bb7b37de7ebefdcb723016bf525f5af016aff323ed8b46e182e063"].into(),
		STASH_ENDOWMENT + CONTROLLER_ENDOWMENT,
	), (
		hex!["2a36057030d2b7034eb0da5188729859780d151bdd3a64295ddcf5514e21a679"].into(),
		STASH_ENDOWMENT - CONTROLLER_ENDOWMENT,
	), (
		hex!["92c3225d346f99794812069a16cb91d979137d1534fff5795dacbaede7369f1d"].into(),
		CONTROLLER_ENDOWMENT,
	), (
		hex!["02456d652d95f78a97ff18e9f1170b3478f3644c4283f58312f2bb98f3ffe74e"].into(),
		CONTROLLER_ENDOWMENT,
	), (
		hex!["5283ca562da42f106978e4773e41c9f2585599f0f63b60c647ef35a5e929c356"].into(),
		92500000000000000000000000,
	), (
		hex!["688421b084a363cb394c5e3a7c79f44482bf2f15f6d86ea37ae110a3af238f07"].into(),
		10000000000000000000000000,
	), (
		hex!["765169492c492ee29f2af9af46f9e1b117aa0283b73a4361ae12ace1c41a6c72"].into(),
		150000000000000000000000000,
	), (
		hex!["6490626e3bde470c449e90b83df92ddb8514f02067a0ddd66f1080b5033dec2d"].into(),
		1475000000000000000002506,
	), (
		hex!["ec80b8b78a2b283f0a48712c8446241cf5f36d2f480559cdc73253981963f402"].into(),
		25000000000000000000000,
	)]
}

/// The mainnet commonwealth validator pubkeys and staked balances. These
/// staked amounts should be less than the balances of the stash accounts.
pub fn get_cw_mainnet_validators() -> Vec<(AccountId, AccountId, Balance, AuraId, GrandpaId, ImOnlineId, AuthorityDiscoveryId)> {
	return vec![(
		hex!["1ec5e3d9a77ac81d6da0290c04d003bbcb04af8c4902bd59dbf9be4dfa47234f"].into(),
		hex!["d6fb4a2f0d5dfc62c37a09e6aac5c3ea4ce2ba021f553c940e63d59dadf0cd24"].into(),
		1000000000000000000000000,
		hex!["532cdeaeb19afd03eb4a57d4dddad09268fb720478d5386263330f5bf86f1cc4"].unchecked_into(),
		hex!["ae5e4e8c3bbb47737a2b4abe79a71289f60271a94a5eaebf6b200f638fcbe332"].unchecked_into(),
		hex!["532cdeaeb19afd03eb4a57d4dddad09268fb720478d5386263330f5bf86f1cc4"].unchecked_into(),
		hex!["5e0755efa5da8b2bb83d2443635268b0be48ba020587fdc3731a5b87b51ff500"].unchecked_into(),
	), (
		hex!["720967cda4c9097924d705695b62dfb6dc6dbeade65b5575abf5c4ca38e50503"].into(),
		hex!["38a58e82baf9df6ec1f9a7064a337f872778649f3dd9002e3fe48df94b475232"].into(),
		1000000000000000000000000,
		hex!["89d7a1fb903a63494696c6d10d76704da7760da7f32dc3b5aa122bfba3f85680"].unchecked_into(),
		hex!["f272b4cdec7e979dd76c7860f7e9dd8cb1da974d74366a266c4de501b2079896"].unchecked_into(),
		hex!["89d7a1fb903a63494696c6d10d76704da7760da7f32dc3b5aa122bfba3f85680"].unchecked_into(),
		hex!["fea24c3e2b57972eead003295fcfc52fb57ffd1bdfedf1dcfb1cd35df11dcc37"].unchecked_into(),
	), (
		hex!["de90c8b070c0a63fbf52655af7492dc8e7d985334a4c60c02bc2f59424ff1430"].into(),
		hex!["0e33e22cd22b272f388bcd41f13942d803089106ec350b8754e1d290ee6ff52b"].into(),
		1000000000000000000000000,
		hex!["98a4f5d4b363447331f54328d83816f773e30799be5979c3d6d9be08f4941799"].unchecked_into(),
		hex!["8634c736497e74b1cedbecec7efe94bdff2ec899f60c97e003e8123f366e14e3"].unchecked_into(),
		hex!["98a4f5d4b363447331f54328d83816f773e30799be5979c3d6d9be08f4941799"].unchecked_into(),
		hex!["bce40bb0b2649d3fa924e050c603dccb5b7468f89924158e9b6d4d048c79dc23"].unchecked_into(),
	), (
		hex!["9665bd715c72b686c2557fe11e6cd54924adef62dc1f52cf43a503f363cf843c"].into(),
		hex!["6aa8f0dd6b6221788d68bf2486126fb14bb65ea710028c11f7ca131e0df10707"].into(),
		1000000000000000000000000,
		hex!["095ca61a04a9bd2fc95b31c1f86b73ef85e1388e75822896b4079cf5bc1c0e14"].unchecked_into(),
		hex!["98d7399dd06f0192a2eb3096aca6df6f2600634d77b92c926bed92d60c50b75d"].unchecked_into(),
		hex!["095ca61a04a9bd2fc95b31c1f86b73ef85e1388e75822896b4079cf5bc1c0e14"].unchecked_into(),
		hex!["8e84ac19afbba97a686f1b20a3288610f76177b3c71cdc1f31df0828ba6acd1c"].unchecked_into(),
	), (
		hex!["464c96a206e310511a27acc92b2e410a14bd83cb8788b522d0cee03f0d285862"].into(),
		hex!["ae5bfe517affa6f7456ad6b9f7465520059e6d7b2a8928673460461abb741c18"].into(),
		1000000000000000000000000,
		hex!["76c5e8b1a4656cf7f174662674c61cb3a8fb67cee15d4f9a85f25235653c2a76"].unchecked_into(),
		hex!["08b8316bbc960b4e3d610724e4f330556356b567fac3a7cb63fe458dbafdf028"].unchecked_into(),
		hex!["76c5e8b1a4656cf7f174662674c61cb3a8fb67cee15d4f9a85f25235653c2a76"].unchecked_into(),
		hex!["a69f1ccc84e40afae38d2d2f87b5910b164a8b451a0d70ac51e117f1222ede65"].unchecked_into(),
	), (
		hex!["34c71b1e42e910b94b8cbb2c960873bd4bf0db6e80afdf41cdc52acd91d6393f"].into(),
		hex!["6a782c02fd24ed538224f3d0bda56146bc6bacd34f9a784c1b5367e19cda456e"].into(),
		1000000000000000000000000,
		hex!["82b6147e4e661551609f04168599ed213c21fa194aba3327fb2fd6247a52b5d2"].unchecked_into(),
		hex!["996b0ef45bd1f90b17ebc5f038cea21ccc1725a2c1daf3a85e774e22e173bb6e"].unchecked_into(),
		hex!["82b6147e4e661551609f04168599ed213c21fa194aba3327fb2fd6247a52b5d2"].unchecked_into(),
		hex!["bef61229205f6027e92e70bcd6b01e352c21d795e38dd64cf87335fd214d994b"].unchecked_into(),
	), (
		hex!["d02002915139ac3e4552c5006f92cccfbf8b02cb4d4ca1993a69d63368cc1f0c"].into(),
		hex!["4864744ab79cd62cbc1094da8e6f54aba8cba7ed6d616bdd8df10572d146c15c"].into(),
		1000000000000000000000000,
		hex!["3cf4ee0a14ea22e82a8953d60fc68f80a62d881e0b56c97445b3ea96adc4d31c"].unchecked_into(),
		hex!["2f9a90f811621556d0dd1bdf324d0154444b34a0501a9f5bd338cf41d94634f9"].unchecked_into(),
		hex!["3cf4ee0a14ea22e82a8953d60fc68f80a62d881e0b56c97445b3ea96adc4d31c"].unchecked_into(),
		hex!["4e85c095e94a47dea48c7c6824d3f4818e1c34df1e00fe658d650845c275e13e"].unchecked_into(),
	), (
		hex!["143f9f9019fa62919ed6da39b8f147cb52501438e1e9e7a82d22d7b49df18e59"].into(),
		hex!["a01bef9c8591ae4155f9483eee09db092fb39bdd580e3715e927333e4aa89d6d"].into(),
		1000000000000000000000000,
		hex!["191decd2b0b7e447a2009d0c8f963b118ee7781adada0e217273ac924514b3a8"].unchecked_into(),
		hex!["a81b12f94160734c3e53ac961a73c796783be50225e4e9b028e8318654f2d876"].unchecked_into(),
		hex!["191decd2b0b7e447a2009d0c8f963b118ee7781adada0e217273ac924514b3a8"].unchecked_into(),
		hex!["38fc0932145f659c13d14d4be0215b3a811738bf713c771b980032d66d10d567"].unchecked_into(),
	), (
		hex!["4e7de9c8f3564fe5cc5057de51c41b40a7da801d22c6ee5aa57f8bb2838ae857"].into(),
		hex!["00e5a14e08930f94148569274ca1e9355938fabf65fffef3b7cb3c3e3edabb23"].into(),
		1000000000000000000000000,
		hex!["d76fca327e6b6c91c220acbe0769d16ede7c96578e7743036a784fe3d528d40d"].unchecked_into(),
		hex!["da5bcd957592d12041bb9777605b3e3aeeac7712c2ba2339a80e33acfc5cf07e"].unchecked_into(),
		hex!["d76fca327e6b6c91c220acbe0769d16ede7c96578e7743036a784fe3d528d40d"].unchecked_into(),
		hex!["e04db33479ca34e1ee304db70edf95b37427c2350b14eff984bccc4d07e8876a"].unchecked_into(),
	), (
		hex!["ce64070e4dffe61183241dca3e922b65ecd509430a3e283fab5c143532f79d3e"].into(),
		hex!["b0492fa7ac84ecb20f9f69e1c328b521fce8f472af2cc13784286d2240e4924c"].into(),
		1000000000000000000000000,
		hex!["5342c923d5c187f4417862556555ee09475c11141cbc0103272b826e0f8cd0b9"].unchecked_into(),
		hex!["a7a35e31a1b49a5ced7f4f6ef214da56318e7ca0dfad50274dbcb88456f35621"].unchecked_into(),
		hex!["5342c923d5c187f4417862556555ee09475c11141cbc0103272b826e0f8cd0b9"].unchecked_into(),
		hex!["504e8b32ad648f3bfaf186840421b7717af828b77b2d512a30dd6cd62060401e"].unchecked_into(),
	)];
}

pub fn get_mainnet_identity_verifiers() -> Vec<AccountId> {
	return vec![
		// 5FP8pEw3pCxTap1GjJh6JVjoLLvnsVyvKtrxu9PSQThMDn1M
		hex!["92c3225d346f99794812069a16cb91d979137d1534fff5795dacbaede7369f1d"].into(),
	];
}

pub fn get_mainnet_election_members() -> Vec<AccountId> {
	return vec![
		// 5C7gaRoByJ99HoiZT9zgJAfx9p3YHLASkdT4Tn3ScgzpX6nX
		hex!["02456d652d95f78a97ff18e9f1170b3478f3644c4283f58312f2bb98f3ffe74e"].into(),
	];
}