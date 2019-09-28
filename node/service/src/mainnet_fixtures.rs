use edgeware_primitives::{Balance, AuraId, AccountId};
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

/// This is the allocation that will fit into the "balances" collection
/// of the "balances" module. The total should be 5e26 - 1e26 or 400 million EDG,
/// since we have also allocated 1e26 to the validators below.
pub fn get_commonwealth_allocation() -> Vec<(AccountId, Balance)> {
	return vec![(
		hex!["1a925251d02fa027882bb1a5834abee8ac4ed06fa735d531cb89ccc478ee5b50"].unchecked_into(),
		1100000000000000000000000,
	), (
		hex!["5c1aee377fcdced7d6a1d0a1b3f89acfbc0f3c2ed6c4f324434aaf3934a14b4b"].unchecked_into(),
		1100000000000000000000000,
	), (
		hex!["f67cfe9c4bb11ac43f7926e177af33bff39a095ff344fb2b96f95cf6b648b745"].unchecked_into(),
		1100000000000000000000000,
	), (
		hex!["ce56ed9d0c61d409610244e91df80be6d29d56f7e8f73a9f8bbb9143e6d3ab2b"].unchecked_into(),
		1100000000000000000000000,
	), (
		hex!["1280d5fd2fe138d51bdd11f8657ef13261f6623dcc4af20f30eea42b5111340c"].unchecked_into(),
		1100000000000000000000000,
	), (
		hex!["4cb0922d0fb5217553da0da70bd4076812ad2a5cce860524ff7b5e6e3629f962"].unchecked_into(),
		3000000000000000000000000,
	), (
		hex!["12d490251399a081935bf731184d2bf37d228bc38d3d68a8e3822933bcf23a09"].unchecked_into(),
		5500000000000000000000000,
	), (
		hex!["78040adec849fff1c66c16ab8ac1534ed27e37a8a1da8aa3239267a883369566"].unchecked_into(),
		1500000000000000000000000,
	), (
		hex!["c0a8a737e77f8c6cb62a2ffa8c0b9d8d7191d46d0e09c48c1354dab109ac4c5e"].unchecked_into(),
		2750000000000000000000000,
	), (
		hex!["f89077e892d9861da018a6cfa4082ccf39cceb83c431cec51b937befb2caa949"].unchecked_into(),
		2750000000000000000000000,
	), (
		hex!["1ec5e3d9a77ac81d6da0290c04d003bbcb04af8c4902bd59dbf9be4dfa47234f"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["720967cda4c9097924d705695b62dfb6dc6dbeade65b5575abf5c4ca38e50503"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["de90c8b070c0a63fbf52655af7492dc8e7d985334a4c60c02bc2f59424ff1430"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["9665bd715c72b686c2557fe11e6cd54924adef62dc1f52cf43a503f363cf843c"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["464c96a206e310511a27acc92b2e410a14bd83cb8788b522d0cee03f0d285862"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["34c71b1e42e910b94b8cbb2c960873bd4bf0db6e80afdf41cdc52acd91d6393f"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["d02002915139ac3e4552c5006f92cccfbf8b02cb4d4ca1993a69d63368cc1f0c"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["143f9f9019fa62919ed6da39b8f147cb52501438e1e9e7a82d22d7b49df18e59"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["4e7de9c8f3564fe5cc5057de51c41b40a7da801d22c6ee5aa57f8bb2838ae857"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["ce64070e4dffe61183241dca3e922b65ecd509430a3e283fab5c143532f79d3e"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["c4fff312e8e224c9f1380d235c1a601d9f00213e6b4f4c86250f768563cb6f2a"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["3a9fcaef453185230c2d777fa7dda16a745f6840b0cbc28f3e8c2ab07e533d3b"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["e280d0fbfbcbdc070526d0997d0bfec3b3527b036bf5a68f87e6cc0422ced302"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["865d38926f2344541912b464e2bb910ef72f2c9d447bc26f996b46f67605b85b"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["6ea38f9aeb405131cfcd8e542d72cd2a3f3e56427b9162298d8e7f3529a6c34e"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["968a20a9c04a662831afd93d8ad834f69c6aeb37e9017b4b739fd94502aae543"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["fce2e25267c2f45932ca1cc9834d2db0b5438804af69a8af16bade1ca00c160d"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["86cdb7761daa3c8917719b057e83ca0377de8ca599624c4ff7bd541ab056427d"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["dc1ec6087624728ec118c4b4cd8e58bfe8c977195aac172473262242fba5d160"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["a0c53fe2dd2bdcb3b3da9612b5a185243057ae81a7e6092a0788672cc99f8b72"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["3401ae0dc64bf4bae104413d1e68380cd8f9f75b753b870b39352bc25e05c865"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["62f118b23e00a02f0e64b2cc5491226c24b7781fd60ca7ac330011173f6d4b68"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["0211cc0925ac6dd2d14fbdb63766d235d2a3c159d4c1dd3a260f3f179b05ce41"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["d6a6eb11c5bb7b37de7ebefdcb723016bf525f5af016aff323ed8b46e182e063"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["2a36057030d2b7034eb0da5188729859780d151bdd3a64295ddcf5514e21a679"].unchecked_into(),
		4999980000000000000000000,
	), (
		hex!["92c3225d346f99794812069a16cb91d979137d1534fff5795dacbaede7369f1d"].unchecked_into(),
		10000000000000000000,
	), (
		hex!["02456d652d95f78a97ff18e9f1170b3478f3644c4283f58312f2bb98f3ffe74e"].unchecked_into(),
		10000000000000000000,
	), (
		hex!["5283ca562da42f106978e4773e41c9f2585599f0f63b60c647ef35a5e929c356"].unchecked_into(),
		92500000000000000000000000,
	), (
		hex!["688421b084a363cb394c5e3a7c79f44482bf2f15f6d86ea37ae110a3af238f07"].unchecked_into(),
		10000000000000000000000000,
	), (
		hex!["765169492c492ee29f2af9af46f9e1b117aa0283b73a4361ae12ace1c41a6c72"].unchecked_into(),
		150000000000000000000000000,
	), (
		hex!["6490626e3bde470c449e90b83df92ddb8514f02067a0ddd66f1080b5033dec2d"].unchecked_into(),
		1475000000000000000002363,
	), (
		hex!["ec80b8b78a2b283f0a48712c8446241cf5f36d2f480559cdc73253981963f402"].unchecked_into(),
		25000000000000000000000,
	)]
}

/// The mainnet commonwealth validator pubkeys and staked balances.
/// We give each of these 9999990000000000000000000 balance, which
/// is 1e19 less than 1e25, so that we reserve 1e19 for the controllers
/// of these accounts (enough balance for txs and existential balance)
///
/// The total of the Commonwealth Mainnet validators balances is thus 1e26 or 20%
/// i.e 10 * ((1e25 - 1e19) + 1e19) = 1e26 or 20% of 5e26
pub const CONTROLLER_ENDOWMENT: Balance = 10000000000000000000;
pub fn get_cw_mainnet_validators() -> Vec<(AccountId, AccountId, AuraId, Balance, GrandpaId, ImOnlineId)> {
	return vec![(
		// 5H8nV9caoeAW9Nuz3qtbnu2z4uDVKbycG3SiYovEBbvMwXVh
		hex!["e049bb3d3369ddf42e07a51e8eeadb660a618fc3cea63dc728a2b79683873855"].unchecked_into(),
		// 5H99BGAaAjcMsy6fqzrN5hkenVBGwU2cVbCYCxwc8oFB3X1q
		hex!["e08f62b32adc2a8c2063c30111c27398d303009c3363342775c8b35100d5c87b"].unchecked_into(),
		// 5DwmAGvL8yaHFEm4NrnK8n6Wxocx83Qiy5du1MbRzCt7Nqcj
		hex!["532cdeaeb19afd03eb4a57d4dddad09268fb720478d5386263330f5bf86f1cc4"].unchecked_into(),
		// Initial bonded stake
		9999990000000000000000000,
		// 5G1LCnmoNuvZan28WJBH1Sxta1FTJxGquy9fRYbbm6xMYi7J
		hex!["ae5e4e8c3bbb47737a2b4abe79a71289f60271a94a5eaebf6b200f638fcbe332"].unchecked_into(),
		// 5DwmAGvL8yaHFEm4NrnK8n6Wxocx83Qiy5du1MbRzCt7Nqcj
		hex!["532cdeaeb19afd03eb4a57d4dddad09268fb720478d5386263330f5bf86f1cc4"].unchecked_into(),
	), (
		// 5G67pPxDrwpRhRQJp4FVLkebg1NCWDzvFaLNxNSSD2vw7XCq
		hex!["b204d6aa8c13f1f274f5f849cb09ef4f3f2641acc3dec99f905e444784fbb247"].unchecked_into(),
		// 5D282YS7mojiwsKJBNebusWdvpvp1jynANTZgmQM6XgDANDz
		hex!["2a43dc8cf92f75f87735149afc8cc93a91bf1e0ea692258e83260c99dedd246a"].unchecked_into(),
		// 5FBSURgNK7jPkt24bSJWFqSmizLVTuep4taaezx3YFBnJzKJ
		hex!["89d7a1fb903a63494696c6d10d76704da7760da7f32dc3b5aa122bfba3f85680"].unchecked_into(),
		// Initial bonded stake
		9999990000000000000000000,
		// 5HYbX7zmVKbNHR8w8Lbc31UAMtBjjyfeGAK6MNThG9SicznB
		hex!["f272b4cdec7e979dd76c7860f7e9dd8cb1da974d74366a266c4de501b2079896"].unchecked_into(),
		// 5FBSURgNK7jPkt24bSJWFqSmizLVTuep4taaezx3YFBnJzKJ
		hex!["89d7a1fb903a63494696c6d10d76704da7760da7f32dc3b5aa122bfba3f85680"].unchecked_into(),
	), (
		// 5F53A2ykziF7JteGC5FEfgMNf5jGZ2GGMSNmSPT2cCZ1NSXj
		hex!["84f5ab01391781936ac0447d2e801fa6531cbe478ae96c5583fef9e101001123"].unchecked_into(),
		// 5Fy7qtUWhdw3hejrabs9r1W3as2sqQ4EztzQTgtcmBJMLpiV
		hex!["acae35110120ca529fe3a42d1a8b8d568899516d2e5b637fd9f1743e37ac1069"].unchecked_into(),
		// 5FWr926Yqmt17vABPddJyX925X7g4ngJKbXkFsZF7RTJCxKs
		hex!["98a4f5d4b363447331f54328d83816f773e30799be5979c3d6d9be08f4941799"].unchecked_into(),
		// Initial bonded stake
		9999990000000000000000000,
		// 5F6fxB8LPBXYo7n1Jf7S81JJEdPXY5CZQsaL3FPqZDunAuXH
		hex!["8634c736497e74b1cedbecec7efe94bdff2ec899f60c97e003e8123f366e14e3"].unchecked_into(),
		// 5FWr926Yqmt17vABPddJyX925X7g4ngJKbXkFsZF7RTJCxKs
		hex!["98a4f5d4b363447331f54328d83816f773e30799be5979c3d6d9be08f4941799"].unchecked_into(),
	), (
		// 5EZWgjN7MqMgWTKs7Ax71w3shs3FGkwFihD4gAyLC84VrLqP
		hex!["6e70fa41746fd3e728e2abe2bed9c957c9ef1a5e67304ff0a0841551bcd4d621"].unchecked_into(),
		// 5HMaNDyhZYRSr99fY82A3Kc9izayBAcwwetfgt3qnxgXb5AU
		hex!["ea0b1fc82793114e5ddbf318e1eadc37280d43ad06fcffe235823b2dc1a0a234"].unchecked_into(),
		// 5CGyoy2eMFYaKKhMLhXuuAaqnBg75v7YVppyEHdcpj534NuY
		hex!["095ca61a04a9bd2fc95b31c1f86b73ef85e1388e75822896b4079cf5bc1c0e14"].unchecked_into(),
		// Initial bonded stake
		9999990000000000000000000,
		// 5FX754YGBq12zbrxPdr7b96DerYQN5wBcWRZhz43M6JJ7Yrn
		hex!["98d7399dd06f0192a2eb3096aca6df6f2600634d77b92c926bed92d60c50b75d"].unchecked_into(),
		// 5CGyoy2eMFYaKKhMLhXuuAaqnBg75v7YVppyEHdcpj534NuY
		hex!["095ca61a04a9bd2fc95b31c1f86b73ef85e1388e75822896b4079cf5bc1c0e14"].unchecked_into(),
	), (
		// 5G1hbiwXNJ8PwChY7d5LYUx8hGq32UcbRsz5MhT6Hr4nG5nv
		hex!["aea65488616f3fa5dc0bb7b4a3a803185bf05a8e89c076f9abc18021694ebb38"].unchecked_into(),
		// 5H4PWKakssFT3rxh66oY6DCpSnq7gW1kS8vN4few33GJJuht
		hex!["dcef6494fd74ad78df343f358d6e408282c9bfc8003cb7edfca8d2c57b9b6e7d"].unchecked_into(),
		// 5EkSJHQYANf3em5ekJaS7Cvn7TZGLdmX6EowZXwL9KmXSQy6
		hex!["76c5e8b1a4656cf7f174662674c61cb3a8fb67cee15d4f9a85f25235653c2a76"].unchecked_into(),
		// Initial bonded stake
		9999990000000000000000000,
		// 5CG8xTcS6ezpAYk9nqY2u1o9nBgSJUQsLBKHNxmdgo8hp4YY
		hex!["08b8316bbc960b4e3d610724e4f330556356b567fac3a7cb63fe458dbafdf028"].unchecked_into(),
		// 5EkSJHQYANf3em5ekJaS7Cvn7TZGLdmX6EowZXwL9KmXSQy6
		hex!["76c5e8b1a4656cf7f174662674c61cb3a8fb67cee15d4f9a85f25235653c2a76"].unchecked_into(),
	), (
		// 5Eh825dytw82xbN3yBuNufN8YEACgCyzB1oRwfFw534qJEgq
		hex!["743ea391454673ddacfd7b344790b7889ab8702cd06208347fb3c24ab9dcbe14"].unchecked_into(),
		// 5EcL14PVD3hDazhcQT18WEAFucHZCy79Fv8ZEAkqos1tNxmW
		hex!["7096bf8b9cfe0600e7504f402dfde3dc0e45d4eff772c4159d21673a95d6dd56"].unchecked_into(),
		// 5F26AtKiBJiwMoUgq5M32mzAaXXHfSKyzQtW8fRdHe7dVQQQ
		hex!["82b6147e4e661551609f04168599ed213c21fa194aba3327fb2fd6247a52b5d2"].unchecked_into(),
		// Initial bonded stake
		9999990000000000000000000,
		// 5FXrzAt8C6s8konBf8BTCkNpvgZtgMfcLzEq7rxBA9KbQVyd
		hex!["996b0ef45bd1f90b17ebc5f038cea21ccc1725a2c1daf3a85e774e22e173bb6e"].unchecked_into(),
		// 5F26AtKiBJiwMoUgq5M32mzAaXXHfSKyzQtW8fRdHe7dVQQQ
		hex!["82b6147e4e661551609f04168599ed213c21fa194aba3327fb2fd6247a52b5d2"].unchecked_into(),
	), (
		// 5F4QGshDAgZy1B1UtHqhWyXVN5J1ckfVNgrFh8rbBtNfPeoc
		hex!["84798309f2d264c5bb5f3c651ddf5080c3494013b201859e62bd9352b32c1148"].unchecked_into(),
		// 5HpyJDaJGqN1qnSZgLG2nffSLw8meLySL4cEnxfnMkuiaX1M
		hex!["feeff14174042f8450cf9a599ba10ac10e2c2ed0985a48169e0f65b07bb7704a"].unchecked_into(),
		// 5DSdVMH9LLJZazwE2wKytMBb8tnH4dAWL6h7kSao46ePMkQ1
		hex!["3cf4ee0a14ea22e82a8953d60fc68f80a62d881e0b56c97445b3ea96adc4d31c"].unchecked_into(),
		// Initial bonded stake
		9999990000000000000000000,
		// 5D982KNVdYHaefG9NxsVnKysDGMQd24B18GkqeGcZYmGEi7d
		hex!["2f9a90f811621556d0dd1bdf324d0154444b34a0501a9f5bd338cf41d94634f9"].unchecked_into(),
		// 5DSdVMH9LLJZazwE2wKytMBb8tnH4dAWL6h7kSao46ePMkQ1
		hex!["3cf4ee0a14ea22e82a8953d60fc68f80a62d881e0b56c97445b3ea96adc4d31c"].unchecked_into(),
	), (
		// 5DU4uisTgSAeRdzkX7kVZRB8RTZEEUoa7xqL6maMNPQGF4Qg
		hex!["3e0dbf4f6e126100f2cc8c6b4c62fffa263387edd2894e7ab7fa3a7f876a9437"].unchecked_into(),
		// 5DNxWy6gTLQh8ySyPMBM1Rb6XuMhTUBZQyoJTnctByCRHJVQ
		hex!["3a2800768efd0e12e4310fecb617833a15b8b5c4e40d10bada06c8759fab065a"].unchecked_into(),
		// 5CddwhVFGKSWquc6CESCq7hNeYZfvdCPsfLNNcjNiYBw8pJR
		hex!["191decd2b0b7e447a2009d0c8f963b118ee7781adada0e217273ac924514b3a8"].unchecked_into(),
		// Initial bonded stake
		9999990000000000000000000,
		// 5Fs7wjVeGS7sPaeBPhDRvw3TkjB8CnVVVaozLjVZxQ1z4WYc
		hex!["a81b12f94160734c3e53ac961a73c796783be50225e4e9b028e8318654f2d876"].unchecked_into(),
		// 5CddwhVFGKSWquc6CESCq7hNeYZfvdCPsfLNNcjNiYBw8pJR
		hex!["191decd2b0b7e447a2009d0c8f963b118ee7781adada0e217273ac924514b3a8"].unchecked_into(),
	), (
		// 5HYrqX3MRVpuPm465cYU8yd4d2owUg3Gqv8hNvY61PD1qdfz
		hex!["f2a644bf322abbe9d5fc0861056ad5220cdf64b15a3218a116d45f91c709bd79"].unchecked_into(),
		// 5F56DEgumyywb9giK41RjKyAewxRuQgsHuyrTBGJg1AaDy2i
		hex!["84fff3e5ba57d2ea6a64c374f9dc9c4ca7bfebda15bf5b7707ca861a6f3e923c"].unchecked_into(),
		// 5GwBMtHPXmQ55Q7aLQrcE2p3qaRCEU4br34dXbLcasJre5RT
		hex!["d76fca327e6b6c91c220acbe0769d16ede7c96578e7743036a784fe3d528d40d"].unchecked_into(),
		// Initial bonded stake
		9999990000000000000000000,
		// 5H11ZrRNEDEstiZSVWDqKQNyStPkRMmnrNNJTZZLazB1ynez
		hex!["da5bcd957592d12041bb9777605b3e3aeeac7712c2ba2339a80e33acfc5cf07e"].unchecked_into(),
		// 5GwBMtHPXmQ55Q7aLQrcE2p3qaRCEU4br34dXbLcasJre5RT
		hex!["d76fca327e6b6c91c220acbe0769d16ede7c96578e7743036a784fe3d528d40d"].unchecked_into(),
	), (
		// 5Ge2b4tnpHkkwkPmhFJUp9SzeArn7G9ETUg5WUFXrGTw8hZN
		hex!["ca5b161dbda0460526d45b8bca2265b3f17d1d26463219f7c05fa4e8f1bf6f4d"].unchecked_into(),
		// 5FToUwzsQuixuLmiJTiTo5ggAS8j4rmHy2TCvbcxMmPLTNkm
		hex!["96524827b539e1fd6d737f2bd648ed0f2df3cd0b6c55ba14518ec97e51e5ef48"].unchecked_into(),
		// 5Dwsfso7enWY9YkP8DTrHcCJUbQtqbBBHLiQi4MRPxDmUVE9
		hex!["5342c923d5c187f4417862556555ee09475c11141cbc0103272b826e0f8cd0b9"].unchecked_into(),
		// Initial bonded stake
		9999990000000000000000000,
		// 5FrWPF7c3BX7WSHzWKHSoH8kbfc7ZQTNXj7vJp3Akwga8KiB
		hex!["a7a35e31a1b49a5ced7f4f6ef214da56318e7ca0dfad50274dbcb88456f35621"].unchecked_into(),
		// 5Dwsfso7enWY9YkP8DTrHcCJUbQtqbBBHLiQi4MRPxDmUVE9
		hex!["5342c923d5c187f4417862556555ee09475c11141cbc0103272b826e0f8cd0b9"].unchecked_into(),
	)];
}

pub fn get_lockdrop_mainnet_validators() -> Vec<(AccountId, AccountId, AuraId, Balance, GrandpaId, ImOnlineId)> {
	return vec![(
		hex!["0c803696008775f015cb664de6aedc7dcbb50e7189b30607db93b6e76f137410"].unchecked_into(),
		hex!["48976f3463c70556243dc713aa02a4cefa1169ce0792efddd696c9145b048909"].unchecked_into(),
		hex!["f59fbd6c9378029dac7b78418521a4ea3fdc164c949fecd3bdcd8524f2689377"].unchecked_into(),
		18540050068241755271771,
		hex!["f59fbd6c9378029dac7b78418521a4ea3fdc164c949fecd3bdcd8524f2689377"].unchecked_into(),
		hex!["f59fbd6c9378029dac7b78418521a4ea3fdc164c949fecd3bdcd8524f2689377"].unchecked_into(),
	), (
		hex!["285ff3860864a743159963266ed1940acf913c9e5ba286e5a64c1c8accdd5527"].unchecked_into(),
		hex!["00b5279a477ba0dab85bfb956a24f2ed927a5dabb81515c7fe450ec563efe406"].unchecked_into(),
		hex!["938ad1fd363e137db2bddb3cb50f4c8b5b389f91ce1617dccaa841e8b5a47f26"].unchecked_into(),
		18540050068241755271771,
		hex!["938ad1fd363e137db2bddb3cb50f4c8b5b389f91ce1617dccaa841e8b5a47f26"].unchecked_into(),
		hex!["938ad1fd363e137db2bddb3cb50f4c8b5b389f91ce1617dccaa841e8b5a47f26"].unchecked_into(),
	), (
		hex!["ce9f16e6e4a98e2bbb25dcc2adaec078f70dc4c4ed591e10af08be6545dd434a"].unchecked_into(),
		hex!["a411d9c685736fac9626be08efdfb0fbedab74575befbd66f0c5c5e25cab7442"].unchecked_into(),
		hex!["802498a3970ea4e4b06546599672a44da517d9144201ae33ce0daba86159d196"].unchecked_into(),
		18540050068241755271771,
		hex!["802498a3970ea4e4b06546599672a44da517d9144201ae33ce0daba86159d196"].unchecked_into(),
		hex!["802498a3970ea4e4b06546599672a44da517d9144201ae33ce0daba86159d196"].unchecked_into(),
	), (
		hex!["38d1bb01050f4d280810e4d06f0b9ce1ed1be1af2624949cef945abecc5f224a"].unchecked_into(),
		hex!["7a8b58e17e2a3b64782bd58ae99fc00275112ea7f2608c7caa2d2e0c475d0e05"].unchecked_into(),
		hex!["ee1279753c267e3617d63682823b943909fbddfca1d074a1f71e6e60b35f7419"].unchecked_into(),
		18540050068241755271771,
		hex!["ee1279753c267e3617d63682823b943909fbddfca1d074a1f71e6e60b35f7419"].unchecked_into(),
		hex!["ee1279753c267e3617d63682823b943909fbddfca1d074a1f71e6e60b35f7419"].unchecked_into(),
	), (
		hex!["aa063d962737aa0372a07c744540e98eefb1bda952bff49f6020969f31866119"].unchecked_into(),
		hex!["5600bbf93f7e26a9d56b01b29955dc1c1ae8dc359f9d566489edce5c1125e142"].unchecked_into(),
		hex!["31f8f4fd810611c872e1f55cdda292dd52084b2cd4e92b48fa09e914aace6c35"].unchecked_into(),
		18540050068241755271771,
		hex!["31f8f4fd810611c872e1f55cdda292dd52084b2cd4e92b48fa09e914aace6c35"].unchecked_into(),
		hex!["31f8f4fd810611c872e1f55cdda292dd52084b2cd4e92b48fa09e914aace6c35"].unchecked_into(),
	), (
		hex!["447dafb0fc00286cf8dcb110aa56ad22777dc5a5a8a4bc427c70892a3c1c5c71"].unchecked_into(),
		hex!["123c3ae57c291f9d49bc683864a000fb5ddbc4a366c37a7622b75a25e4ea8302"].unchecked_into(),
		hex!["a9fd46c6d824b592d156a34d26c4cb65b41e0674db4224f2e98b774c57f84ab9"].unchecked_into(),
		18540050068241755271771,
		hex!["a9fd46c6d824b592d156a34d26c4cb65b41e0674db4224f2e98b774c57f84ab9"].unchecked_into(),
		hex!["a9fd46c6d824b592d156a34d26c4cb65b41e0674db4224f2e98b774c57f84ab9"].unchecked_into(),
	), (
		hex!["78f1e52cc6fc550ea1b731087a014bb49ec2aad82de8b740c4c52058414e5822"].unchecked_into(),
		hex!["4c34997146bb64c5f365994c97a64c1f57cc76a9208b57b3e0a04b0d4b426070"].unchecked_into(),
		hex!["93a90a4cbd76d83dc0a685026d9ffcd0d0a80be8de64779f0ac156ca094bd817"].unchecked_into(),
		18540050068241755271771,
		hex!["93a90a4cbd76d83dc0a685026d9ffcd0d0a80be8de64779f0ac156ca094bd817"].unchecked_into(),
		hex!["93a90a4cbd76d83dc0a685026d9ffcd0d0a80be8de64779f0ac156ca094bd817"].unchecked_into(),
	), (
		hex!["20e376fa51d6185fe13dfd8bdeb1ba1f09f9d8eeca5bc3dd42934b329f06ec6c"].unchecked_into(),
		hex!["4c49512890e83e96c0f258846a5a75d5358283b710129515362507028a011f27"].unchecked_into(),
		hex!["3bea727ff4cf60406e3ad51a96d90c697c74a27bfecbb5166c2a5ed6bb3b8603"].unchecked_into(),
		18540050068241755271771,
		hex!["3bea727ff4cf60406e3ad51a96d90c697c74a27bfecbb5166c2a5ed6bb3b8603"].unchecked_into(),
		hex!["3bea727ff4cf60406e3ad51a96d90c697c74a27bfecbb5166c2a5ed6bb3b8603"].unchecked_into(),
	), (
		hex!["d2c76aea3dfec96db22d17f52c799898ddcb5abf77d3229f30b3e89235063012"].unchecked_into(),
		hex!["a80e9d3a33f1b369c82301676221d18fc9f1830bdceee955ad05a1d5a339da05"].unchecked_into(),
		hex!["8275113514c743bf3833878ebcee34a4843b45d81513595d20db37b7f2b41c28"].unchecked_into(),
		18540050068241755271771,
		hex!["8275113514c743bf3833878ebcee34a4843b45d81513595d20db37b7f2b41c28"].unchecked_into(),
		hex!["8275113514c743bf3833878ebcee34a4843b45d81513595d20db37b7f2b41c28"].unchecked_into(),
	), (
		hex!["4e6d37f5890f180e2b0097acde94cc376189c14df3dfcf3f1d673629fb875953"].unchecked_into(),
		hex!["145af2aaea8923afe0a017fdedb927faa7116817052fe83c9f925fad59b7be60"].unchecked_into(),
		hex!["56ad4d25418c291be1a24a047ed964d6477dc64e3cba2363f59498db584c2039"].unchecked_into(),
		18540050068241755271771,
		hex!["56ad4d25418c291be1a24a047ed964d6477dc64e3cba2363f59498db584c2039"].unchecked_into(),
		hex!["56ad4d25418c291be1a24a047ed964d6477dc64e3cba2363f59498db584c2039"].unchecked_into(),
	), (
		hex!["0a2a3fb58fe3704fe9628c8bc9018f0d9a0c5b503cb6a9a38d8b2d368feb0906"].unchecked_into(),
		hex!["948c55d3fb714f577d75ab86a935cb396a5ab782ddf4f5ece930d73181dbdb54"].unchecked_into(),
		hex!["c8385e1cb73c32fceb9b228c8cf9cbfde69d687e38b2674d60e29787abae0b88"].unchecked_into(),
		3122802935616628619336663,
		hex!["c8385e1cb73c32fceb9b228c8cf9cbfde69d687e38b2674d60e29787abae0b88"].unchecked_into(),
		hex!["c8385e1cb73c32fceb9b228c8cf9cbfde69d687e38b2674d60e29787abae0b88"].unchecked_into(),
	), (
		hex!["6c646c2cda68e0aa5965416f2fe96152ff4fad2af6506d37f3a67db41aa9025b"].unchecked_into(),
		hex!["9ecf4a718f49ef51a28a42ff9a990492b75a17943dcb14e14b5ab725a4f08e4b"].unchecked_into(),
		hex!["24b2d1ea8ddba89d2899b8b31de1155fc1c155115128484a4ee32ff461ec7b84"].unchecked_into(),
		3122802935616628619336663,
		hex!["24b2d1ea8ddba89d2899b8b31de1155fc1c155115128484a4ee32ff461ec7b84"].unchecked_into(),
		hex!["24b2d1ea8ddba89d2899b8b31de1155fc1c155115128484a4ee32ff461ec7b84"].unchecked_into(),
	), (
		hex!["2a983791380e039f4ba03e988839290e14f632cb5f09059ec8bb62c3728b566a"].unchecked_into(),
		hex!["d4a67bbd63fcb7ddcf29d93b6844d9349c92f3657aba0c591803dd5657c7f32f"].unchecked_into(),
		hex!["ebd40abf2c77bdaa2a7e1988eced156c89dd2b4c3222edc57bbb7e4e311d20aa"].unchecked_into(),
		3122802935616628619336663,
		hex!["ebd40abf2c77bdaa2a7e1988eced156c89dd2b4c3222edc57bbb7e4e311d20aa"].unchecked_into(),
		hex!["ebd40abf2c77bdaa2a7e1988eced156c89dd2b4c3222edc57bbb7e4e311d20aa"].unchecked_into(),
	), (
		hex!["a20c71fe648acea8ea13e7ce0d507cfb00c1788f1eb402b756ba7dbd4dba783a"].unchecked_into(),
		hex!["ec097d18edeb5aafe622cf05288095116656765050143fb63081a74e2d53fc7c"].unchecked_into(),
		hex!["407144e5e32f03ac8ed61300c314002a569d81afbd8a7c67d65851bc600d600b"].unchecked_into(),
		3122802935616628619336663,
		hex!["407144e5e32f03ac8ed61300c314002a569d81afbd8a7c67d65851bc600d600b"].unchecked_into(),
		hex!["407144e5e32f03ac8ed61300c314002a569d81afbd8a7c67d65851bc600d600b"].unchecked_into(),
	), (
		hex!["1aa1c5f4b2f4c0facdd81bda5fc3aea7d868b63db68bc1de1219f7453dfda368"].unchecked_into(),
		hex!["0819394b07f91ff8b0349c88f487745a33a77747f7f34417d78087946ce7e66b"].unchecked_into(),
		hex!["7be56d5f02080a11ad2439ccb95ff464485d825afbd6b12732d07cd06999f8ee"].unchecked_into(),
		3122802935616628619336663,
		hex!["7be56d5f02080a11ad2439ccb95ff464485d825afbd6b12732d07cd06999f8ee"].unchecked_into(),
		hex!["7be56d5f02080a11ad2439ccb95ff464485d825afbd6b12732d07cd06999f8ee"].unchecked_into(),
	), (
		hex!["36ff87f6b0606ee7f22e57b9dcaf5e29a6220bf7d9047801de1afc0a98103c69"].unchecked_into(),
		hex!["6ef7607ca1a5572542e580dcd027f485a5066f2849a93c455a59eb11c2bea417"].unchecked_into(),
		hex!["3028b8ab8c04d4910a76f2b906a33e04af4ecaff06c444da7c82845c6aa7efbe"].unchecked_into(),
		3122802935616628619336663,
		hex!["3028b8ab8c04d4910a76f2b906a33e04af4ecaff06c444da7c82845c6aa7efbe"].unchecked_into(),
		hex!["3028b8ab8c04d4910a76f2b906a33e04af4ecaff06c444da7c82845c6aa7efbe"].unchecked_into(),
	), (
		hex!["14108d040cb6d932fd8a86da32965f863775e20d0cefb94fb2caaddfb1c69f1c"].unchecked_into(),
		hex!["32bdc67ea5634997d31c70d920739d699a5721c407c9a8edca30ec8eb4fd4623"].unchecked_into(),
		hex!["da2acf2f27d65f006a23d6a78dd81aba60f24a51e50295f0e7dac45cc92beadd"].unchecked_into(),
		3122802935616628619336663,
		hex!["da2acf2f27d65f006a23d6a78dd81aba60f24a51e50295f0e7dac45cc92beadd"].unchecked_into(),
		hex!["da2acf2f27d65f006a23d6a78dd81aba60f24a51e50295f0e7dac45cc92beadd"].unchecked_into(),
	), (
		hex!["2afd2211c61d0e9e3aeb6a7138185954e7d4964f3aa286316b14e3596650df0c"].unchecked_into(),
		hex!["b43ee30b2ce1fcf1589d30023c4ddaf2b2a489680e30387370d18b3f602a4468"].unchecked_into(),
		hex!["5797ce08537b95549996a70349baf8f4f63f5b1e10433034b404a9db563a4097"].unchecked_into(),
		3122802935616628619336663,
		hex!["5797ce08537b95549996a70349baf8f4f63f5b1e10433034b404a9db563a4097"].unchecked_into(),
		hex!["5797ce08537b95549996a70349baf8f4f63f5b1e10433034b404a9db563a4097"].unchecked_into(),
	), (
		hex!["78723d907d7a9025c50a02732eca4086b52db5949a321c61b008efa1e4d0c848"].unchecked_into(),
		hex!["b645ae570204613d267e2347482c8d39baf464157d74806ca33c31ffdee2112c"].unchecked_into(),
		hex!["883c7e650b2c8f47cf91dd407ace2e672210d52b67b97ec8b419f3a48ad37a95"].unchecked_into(),
		3122802935616628619336663,
		hex!["883c7e650b2c8f47cf91dd407ace2e672210d52b67b97ec8b419f3a48ad37a95"].unchecked_into(),
		hex!["883c7e650b2c8f47cf91dd407ace2e672210d52b67b97ec8b419f3a48ad37a95"].unchecked_into(),
	), (
		hex!["52a53eccc514b22c4d2a4df1f3f45e5cf73173f9f6b0f68c2bc374af366f8208"].unchecked_into(),
		hex!["7a4dc82bf1d481183621e25018c189585004793591f1a8bdb48c37078b4b0853"].unchecked_into(),
		hex!["75c85340a069c4a82161b96f087f6bfff6248570661fb4e290ea3bae492f85cf"].unchecked_into(),
		3122802935616628619336663,
		hex!["75c85340a069c4a82161b96f087f6bfff6248570661fb4e290ea3bae492f85cf"].unchecked_into(),
		hex!["75c85340a069c4a82161b96f087f6bfff6248570661fb4e290ea3bae492f85cf"].unchecked_into(),
	), (
		hex!["e233a0eceeef1de7d5e844c995a5e15248cc1042c425014e9c8e7beace27a910"].unchecked_into(),
		hex!["d82cafca36ad9f93037ab4bbfcef9d53e1907781611f5b2100d8862e23ff9d31"].unchecked_into(),
		hex!["7429597336d1eb642f024b698f11e64ae560e7343186d8cff22d0370b5650ce4"].unchecked_into(),
		3122802935616628619336663,
		hex!["7429597336d1eb642f024b698f11e64ae560e7343186d8cff22d0370b5650ce4"].unchecked_into(),
		hex!["7429597336d1eb642f024b698f11e64ae560e7343186d8cff22d0370b5650ce4"].unchecked_into(),
	), (
		hex!["040b398943f3dd27bbec2a114e76d86b7f562c57464d81cb18e6385a789a0e44"].unchecked_into(),
		hex!["aa7a9fa240157ac01eb7581a4f0026c32ec3a226ff95ebb22b132146a673ea02"].unchecked_into(),
		hex!["f852c94888521b6ccc7a017ef9326a0b23756c2364025a257fcf1dc89371f5af"].unchecked_into(),
		3122802935616628619336663,
		hex!["f852c94888521b6ccc7a017ef9326a0b23756c2364025a257fcf1dc89371f5af"].unchecked_into(),
		hex!["f852c94888521b6ccc7a017ef9326a0b23756c2364025a257fcf1dc89371f5af"].unchecked_into(),
	), (
		hex!["e03a9bec4af9855a6dd6cffdd932c38d48967b5cba3f8dbfa69cd12b6383cc61"].unchecked_into(),
		hex!["70a2f9d62af61412148df214a074cd44d2d8327cbbc5bd6f35978130bfe5db2c"].unchecked_into(),
		hex!["1db251af2bab85600bc1dd2ab2998f5256022406dc6a7de93ebc046d155d82ed"].unchecked_into(),
		3122802935616628619336663,
		hex!["1db251af2bab85600bc1dd2ab2998f5256022406dc6a7de93ebc046d155d82ed"].unchecked_into(),
		hex!["1db251af2bab85600bc1dd2ab2998f5256022406dc6a7de93ebc046d155d82ed"].unchecked_into(),
	), (
		hex!["668c11291d31427f4505b18fc08eb2996adc3d6012259ab8ccfb3ddfbbae230c"].unchecked_into(),
		hex!["bc07b885cdd2c384e99b6bba101dbab478c4bed94eb760069a226a7ec9afcc22"].unchecked_into(),
		hex!["f8953e49dd01f68a2e0c3c532a5b4a6ac5f797321a0a2b6975a261f770d57ef6"].unchecked_into(),
		3122802935616628619336663,
		hex!["f8953e49dd01f68a2e0c3c532a5b4a6ac5f797321a0a2b6975a261f770d57ef6"].unchecked_into(),
		hex!["f8953e49dd01f68a2e0c3c532a5b4a6ac5f797321a0a2b6975a261f770d57ef6"].unchecked_into(),
	), (
		hex!["028f00d134bf7411d6b8955e497a2dd666f82e2a8953b8da4cf23d3d6a261b61"].unchecked_into(),
		hex!["aa3a264bee978b1687a263a8a379a3a9890c56b46e2bd335ed5c97f7b5852349"].unchecked_into(),
		hex!["a547423144bf37432ca32462e29c59c03a4931d936722369cab5b47b26e5721a"].unchecked_into(),
		2992686141213778738589694,
		hex!["a547423144bf37432ca32462e29c59c03a4931d936722369cab5b47b26e5721a"].unchecked_into(),
		hex!["a547423144bf37432ca32462e29c59c03a4931d936722369cab5b47b26e5721a"].unchecked_into(),
	), (
		hex!["f24a4eab9a3ac6e78bb48d1db3c4ee9d434f80f7bbd2aacabe2fd65f6beeb970"].unchecked_into(),
		hex!["8485ab45bae182236b293b0b9d501d3df97f7ca834661db602caa7c39a37c224"].unchecked_into(),
		hex!["9af3ff71dcda88e17df07b36d5e85d3179986890394ad45a559491faff15bdaa"].unchecked_into(),
		2992686141213778738589694,
		hex!["9af3ff71dcda88e17df07b36d5e85d3179986890394ad45a559491faff15bdaa"].unchecked_into(),
		hex!["9af3ff71dcda88e17df07b36d5e85d3179986890394ad45a559491faff15bdaa"].unchecked_into(),
	), (
		hex!["4e1ab64740da39e819fe6e2e7593824ee1358ea330ca10a7df13fcc900bdbf47"].unchecked_into(),
		hex!["4c8e33f62e7e429362ba4c64f201d177106971edffdd653bf8b890aee2b0b82c"].unchecked_into(),
		hex!["21de37d2dde55dc0bd488731aaf53804adfab98fdbd3e4887b0b5d366edba0ae"].unchecked_into(),
		2992686141213778738589694,
		hex!["21de37d2dde55dc0bd488731aaf53804adfab98fdbd3e4887b0b5d366edba0ae"].unchecked_into(),
		hex!["21de37d2dde55dc0bd488731aaf53804adfab98fdbd3e4887b0b5d366edba0ae"].unchecked_into(),
	), (
		hex!["121fec9ca1278ce28efd279f1bee32de7b3fe820b14708011cc3ef6a9f39f565"].unchecked_into(),
		hex!["5eb62d4616bb188f2756beda38fd37f5e08f7ac58cb6a4e87c9c6d68214b9d6e"].unchecked_into(),
		hex!["92c6d6ffa7ef386e332c0d72304266601d0fb73456d0327bfc5d55d089636e88"].unchecked_into(),
		2992686141213778738589694,
		hex!["92c6d6ffa7ef386e332c0d72304266601d0fb73456d0327bfc5d55d089636e88"].unchecked_into(),
		hex!["92c6d6ffa7ef386e332c0d72304266601d0fb73456d0327bfc5d55d089636e88"].unchecked_into(),
	), (
		hex!["743e2fa7e4d66b03479fa15e6d86e9608754c723228ff24788b8bb7969607f08"].unchecked_into(),
		hex!["9e9b909438bc83e268eb1e7035e0a7d30cc75d74864cd803b7e04bf468492926"].unchecked_into(),
		hex!["bafae333904454bb4e4c7a6568ec2b05c525403a71f1ec188a9cc86e81a33c78"].unchecked_into(),
		2992686141213778738589694,
		hex!["bafae333904454bb4e4c7a6568ec2b05c525403a71f1ec188a9cc86e81a33c78"].unchecked_into(),
		hex!["bafae333904454bb4e4c7a6568ec2b05c525403a71f1ec188a9cc86e81a33c78"].unchecked_into(),
	), (
		hex!["681758bc94876e55a6f38036234c9f8ee679f228b2c4356bea286dc8c7356974"].unchecked_into(),
		hex!["f654838d4ac3c144dd9239d5ee67aac8fb8ba1f3c2b248ebaac8d898eec4563c"].unchecked_into(),
		hex!["dd7c344f06080873c54e514ec40226df12bbad2aab14ebe60dcd362e039d7864"].unchecked_into(),
		2992686141213778738589694,
		hex!["dd7c344f06080873c54e514ec40226df12bbad2aab14ebe60dcd362e039d7864"].unchecked_into(),
		hex!["dd7c344f06080873c54e514ec40226df12bbad2aab14ebe60dcd362e039d7864"].unchecked_into(),
	), (
		hex!["e84b404dc0befdfc0df90abd1fbdc53005a5fc370130947458c6a6a43fe53f33"].unchecked_into(),
		hex!["4647efa51898c17b2c8d6abc8e86f2a484f5612a7138fa8e7af82c9bd85f9f5e"].unchecked_into(),
		hex!["ec191e6743f1ea1a91aad3182b18c1362325bddde9aafb2be7f27674e82241f3"].unchecked_into(),
		88792714904299203628297,
		hex!["ec191e6743f1ea1a91aad3182b18c1362325bddde9aafb2be7f27674e82241f3"].unchecked_into(),
		hex!["ec191e6743f1ea1a91aad3182b18c1362325bddde9aafb2be7f27674e82241f3"].unchecked_into(),
	), (
		hex!["aae52959456c7855784c5bbc9f2ce0354e48e1b316d325cc703a1afa8c14d200"].unchecked_into(),
		hex!["dc43cca12c41c955eb309bb951afc409d2066e67d06cb7ae3afff6831e905c51"].unchecked_into(),
		hex!["67acf5b8bb99f9c559ef5293a2d5b1052706f20a59daca6e9bf3ac43ec2be40a"].unchecked_into(),
		1950646476010340191228,
		hex!["67acf5b8bb99f9c559ef5293a2d5b1052706f20a59daca6e9bf3ac43ec2be40a"].unchecked_into(),
		hex!["67acf5b8bb99f9c559ef5293a2d5b1052706f20a59daca6e9bf3ac43ec2be40a"].unchecked_into(),
	), (
		hex!["f66fa20d487cf983e3b67e44f7fae37f8e2d74f9dfa4a396e6c41c06a3ec1464"].unchecked_into(),
		hex!["0467d9b0978ae5a815bc5682e8f9ef960a84f687c674da9af95893f3d6d8c101"].unchecked_into(),
		hex!["21ca647f9356e2f28d182617c00c15ddf20ffcf74324b21eb8ecf0abd1ddf737"].unchecked_into(),
		71788446170493484580155,
		hex!["21ca647f9356e2f28d182617c00c15ddf20ffcf74324b21eb8ecf0abd1ddf737"].unchecked_into(),
		hex!["21ca647f9356e2f28d182617c00c15ddf20ffcf74324b21eb8ecf0abd1ddf737"].unchecked_into(),
	)];
}

pub fn get_more_endowed() -> Vec<AccountId> {
	return vec![
		// 5GbmRGiV3roZkekSVHigiRonQnauNg5SB3DEnjF1a3VaW2uY
		hex!["c8a18852ebde806e011b4df37861a7b9b3960eea3383d8c5938a2feb6585ca60"].unchecked_into(),
		// 5DaJBK1GzL8ppRvGFGZzCtvXGUzEuqed2dQCp9bQmSWBcovH
		hex!["42cde2ea2ebad078c18be12d567bde59fe019243ddf670c9b46b30cbe0210d44"].unchecked_into(),
		// 5CexFLkSDHFn6jWSaiQs6s3QURig5rPoTCWecdysVxWRD8Jo
		hex!["1a1ec9b100da17f4062f5b23d4c442d6845ac913ff7d6d1ef5357be688b4ef16"].unchecked_into(),
		// 5HNVCZustH928S8mWbQWPcNfEQ5zipccQTiCDbYDrdSDno4f
		hex!["eabcfa4431091ab5742e21164a2e24b0e0d6f3ab96018c002b0188c213272d47"].unchecked_into(),
		// 5CrLFRMME6MbaFXrTUuHZ6VcS3RFHNwtqiP52f8BnEQTARVB
		hex!["22CC8CEE58420B7FF445DC9D6AFAEFC33B658A5F3A26322BA8DAB2D3FB6D2F1F"].unchecked_into(),
	];
}

pub fn get_mainnet_identity_verifiers() -> Vec<AccountId> {
	return vec![
		// 5FP8pEw3pCxTap1GjJh6JVjoLLvnsVyvKtrxu9PSQThMDn1M
		hex!["92c3225d346f99794812069a16cb91d979137d1534fff5795dacbaede7369f1d"].unchecked_into(),
	];
}

pub fn get_mainnet_root_key() -> AccountId {
	// 5C7gaRoByJ99HoiZT9zgJAfx9p3YHLASkdT4Tn3ScgzpX6nX
	return hex!["02456d652d95f78a97ff18e9f1170b3478f3644c4283f58312f2bb98f3ffe74e"].unchecked_into();
}

pub fn get_mainnet_election_members() -> Vec<AccountId> {
	return vec![
		// 5C7gaRoByJ99HoiZT9zgJAfx9p3YHLASkdT4Tn3ScgzpX6nX
		hex!["02456d652d95f78a97ff18e9f1170b3478f3644c4283f58312f2bb98f3ffe74e"].unchecked_into(),
	];
}
