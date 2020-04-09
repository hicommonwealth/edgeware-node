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

use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_aura::ed25519::AuthorityId as AuraId;
use edgeware_primitives::{AccountId};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use pallet_im_online::ed25519::{AuthorityId as ImOnlineId};
use sp_core::crypto::UncheckedInto;
use hex_literal::hex;
use sc_network::config::MultiaddrWithPeerId;

/// Testnet bootnodes
pub fn get_testnet_bootnodes() -> Vec<MultiaddrWithPeerId> {
	return vec![
		"/ip4/45.77.78.68/tcp/30333/p2p/QmVxmq3EtJa7NWae6hQv3PqBNud2UhubUWVXjVewtGeciK".parse().unwrap(),
		"/ip4/45.76.17.97/tcp/30333/p2p/QmSpWQuAmrFgiygF2DZd3bzyHCWuzYcFXmLpzcph5mnTu9".parse().unwrap(),
		"/ip4/45.77.93.189/tcp/30333/p2p/QmVJfKRBVPcZHnaqfGyRym3aTXZMtrmNn8XYGsowoxUhr9".parse().unwrap(),
		"/ip4/108.61.209.73/tcp/30333/p2p/QmQFddsCpMh3rF7xTWuqWAcWy1pqS4bvShTRVe9zbYJQRh".parse().unwrap(),
		"/ip4/45.76.248.131/tcp/30333/p2p/QmeDZ3uK2aSv738r7trsQrpGt7HGgDLL3JqTY2SGHxN7qC".parse().unwrap(),
		"/ip4/96.30.192.236/tcp/30333/p2p/QmNkioYhmd26XqzHRmRWCnvd5NGoMNKYh9fgxvAy9Zf7mS".parse().unwrap(),
	];
}



/// Testnet initial authorities
pub fn get_testnet_initial_authorities() -> Vec<(AccountId, AccountId, GrandpaId, AuraId, ImOnlineId, AuthorityDiscoveryId)> {
	return vec![(
		// 5DRQpsFg1BgziDA5oMfwVyWzF8CkwwnxsguSu1utbgRNQFrK
		hex!["3c070e2721a02249bd35a0677e1a3b1a8b9a07c25a902e7e9373b4e4d0378a54"].into(),
		// 5GNyVHKVDEh4FfAQNhdkTWKuPT42j9ExsFJHfP7RjDo4s8qB
		hex!["bedff87aaf3154ee73dae8754f9af11e033a0cbba09a8e91f4dde82d3d6bed20"].into(),
		// 5HfcYNfpoch9w88CzAqH9PuWHUzwbSJHBA1v3BF9WsRoLht7
		hex!["f7ccdcf57cd3ecd9e56c3324ad95a1484e6f21b0b6f3540a09ada389499cab9d"].unchecked_into(),
		// 5ETqvphCpj22HfKcKRc4zS1VCdZXKtaF1ArhHAyQU73ceVm5
		hex!["6a1e4860a31716685e0e0f49f3333c5799bbdab5dd3bb1e674134f6f9b2be689"].unchecked_into(),
		// 5ETqvphCpj22HfKcKRc4zS1VCdZXKtaF1ArhHAyQU73ceVm5
		hex!["6a1e4860a31716685e0e0f49f3333c5799bbdab5dd3bb1e674134f6f9b2be689"].unchecked_into(),
		// 5ETqvphCpj22HfKcKRc4zS1VCdZXKtaF1ArhHAyQU73ceVm5
		hex!["6a1e4860a31716685e0e0f49f3333c5799bbdab5dd3bb1e674134f6f9b2be689"].unchecked_into(),
	),(
		// 5EHvssibZkZHkZwoABzBqbb21rsaJjQg8KfW75TjVZmzbEh9
		hex!["628e7d34e61faa51f4aac5c400406646876c7189575d84eb6d4e4f5ecec8e672"].into(),
		// 5DscYaYpUohKFeRJRvKYGU1XuDvLNo4XKuN6gDzeKSxF95eB
		hex!["5002e2d414c9c0dc6753b54499077da71b8abe348ab0e745a78d1ca5e70cd752"].into(),
		// 5DU7imzCeBoaWPkw6dqVpMUj8zzkgKom3uG3RJtPLNQpVhzk
		hex!["3e1735fcc35cf289761f00cddabc74e91a9b565b9838a205f0027e23d06e76b1"].unchecked_into(),
		// 5GnSM7vxsa5weU2EFTFi3qBRtxB66g4MtbaRpgCBRfEzA1G9
		hex!["d0c50804164d9e79b3899df678d6de83a226b581fc972f8b8bdc74070ae7e8af"].unchecked_into(),
		// 5GnSM7vxsa5weU2EFTFi3qBRtxB66g4MtbaRpgCBRfEzA1G9
		hex!["d0c50804164d9e79b3899df678d6de83a226b581fc972f8b8bdc74070ae7e8af"].unchecked_into(),
		// 5GnSM7vxsa5weU2EFTFi3qBRtxB66g4MtbaRpgCBRfEzA1G9
		hex!["d0c50804164d9e79b3899df678d6de83a226b581fc972f8b8bdc74070ae7e8af"].unchecked_into(),
	), (
		// 5CwKvDp9JTo3fLW9Q6NrEZ7PaCCjeLCmGbTnN2hEfs9WfRM7
		hex!["269ba9c9b8a209acdb1858501a649ac20ea2331a519c9104dbdda40356e3723f"].into(),
		// 5E6xrSDyaARfbwkYQfDqsP2xA1wzLMoFRiXrpmWiuuV8GuZm
		hex!["5a31704dfdb8e263a15b4af4ddd1c0b14e675377126c3bcddcb9cba0570c040f"].into(),
		// 5CzV3FMTHzQxtF3TSkVcp2CNJHnuwUCJjhTsuYEUGxizRAUq
		hex!["29041a9d8ca43fd99a9c0e2447c6d137e7ba991d8475c790cbf78744636f9915"].unchecked_into(),
		// 5DDtisexsMoEG94f4tr5qRaSKJ42f1H1kBxEHKgq5Kocvsdq
		hex!["333e04dd11f60ebf3037e2615be6d63b01f310b920f8022fb1d6737a2c73dfa5"].unchecked_into(),
		// 5DDtisexsMoEG94f4tr5qRaSKJ42f1H1kBxEHKgq5Kocvsdq
		hex!["333e04dd11f60ebf3037e2615be6d63b01f310b920f8022fb1d6737a2c73dfa5"].unchecked_into(),
		// 5DDtisexsMoEG94f4tr5qRaSKJ42f1H1kBxEHKgq5Kocvsdq
		hex!["333e04dd11f60ebf3037e2615be6d63b01f310b920f8022fb1d6737a2c73dfa5"].unchecked_into(),
	), (
		// 5DvWxEcMP66DgHigGm2eTTg4pPueDMMDS5F67ixK2WpCTKMU
		hex!["5239cc265b2d7ac6dad6b640a28a64ce5e09b7de22fd0549c2d282d461da260e"].into(),
		// 5EkGSct2SPojcFF6fX6EvF3xbaW5aq3oEW2ujJLjTKk8pexP
		hex!["76a4bad1d5fe37ba60dcc9160f3b0fb1822c64f0e92f2171c358a0b7e59aef37"].into(),
		// 5GywTGqF81sdGsynZG7hr8DgibrCDsdvNN9mGCQhf7CNqHpv
		hex!["d98ab5ea66c0ee4d443b3b6f896daf9c7fefb4d1d2eeca7653ffae84557cf5f3"].unchecked_into(),
		// 5D8sbkeQpqoAXt7E4WcNTBEK3sn4CGp67HRBJRQsqXFfsXB5
		hex!["2f6a032ba0dbdcac7fa68607533971ba399a9a06002978b4c071f87334d153b0"].unchecked_into(),
		// 5D8sbkeQpqoAXt7E4WcNTBEK3sn4CGp67HRBJRQsqXFfsXB5
		hex!["2f6a032ba0dbdcac7fa68607533971ba399a9a06002978b4c071f87334d153b0"].unchecked_into(),
		// 5D8sbkeQpqoAXt7E4WcNTBEK3sn4CGp67HRBJRQsqXFfsXB5
		hex!["2f6a032ba0dbdcac7fa68607533971ba399a9a06002978b4c071f87334d153b0"].unchecked_into(),
	), (
		// 5ECSwHL89ShGsfBt34HyjCHK7gkd6vGT5A4gTa5yd4mKPhYe
		hex!["5e603412d1c84d56f590423a78050aebd3ec34e6d3bc775ca87d19216eb91911"].into(),
		// 5C7rVE4qA7GvruqzHjc9RYnoNBP5hbCCKqpEjCm5KEmfvHir
		hex!["0266c9d3e063215ef8f35fc87ccd50489b3c6a2356aac39f89d0667145fab16b"].into(),
		// 5GCJ3HKN5MaseCwqwNJ4pUbpJqRbfAmZXWB8SCMJM6FMyM9B
		hex!["b6bab8caa7be249400b5062d17908c59c0e40dcbe2bd1c818098a5dae916a869"].unchecked_into(),
		// 5HPtGdWoRmiReRYE16AQitm4MG8s47ngfHLGUeKHZuo1Cdry
		hex!["ebcde238597379c874dd51fcca5e0f651972b218c46aa21c471167474e089c86"].unchecked_into(),
		// 5HPtGdWoRmiReRYE16AQitm4MG8s47ngfHLGUeKHZuo1Cdry
		hex!["ebcde238597379c874dd51fcca5e0f651972b218c46aa21c471167474e089c86"].unchecked_into(),
		// 5HPtGdWoRmiReRYE16AQitm4MG8s47ngfHLGUeKHZuo1Cdry
		hex!["ebcde238597379c874dd51fcca5e0f651972b218c46aa21c471167474e089c86"].unchecked_into(),
	), (
		// 5GH23iTUJtvz9KGDK36nXWHtrkm6E83ZZjVPFhb8DKQk3cv3
		hex!["ba551cfbf9e91da337f21658276dfbd56ba43be852395db10a89a64e07978f31"].into(),
		// 5HQwfPfmbuWt3fKyEK3SSuDneVtF4MwHbK1afsXPHxAfogyj
		hex!["ec9c8c8d80eab0b1fc4733e25af31137fb656390c595bb1c7536f73b201ede57"].into(),
		// 5GokhX8Ce1VrMaWFt5RMdAq2EkzoBxdUerFoMzRLDYNPyS2M
		hex!["d1c60ddadc9a3f65da208c5c50e7fc9ed0ab069e79553d08dcc36a401948fa1a"].unchecked_into(),
		// 5Ec2hh96mXEavdu2C866hgUC4joBYpBVujXQJgQsDWfUMmM1
		hex!["705c8360296c7b6af2f842e7a0804492c86a855aaa605fdf419f577f1f4fecbe"].unchecked_into(),
		// 5Ec2hh96mXEavdu2C866hgUC4joBYpBVujXQJgQsDWfUMmM1
		hex!["705c8360296c7b6af2f842e7a0804492c86a855aaa605fdf419f577f1f4fecbe"].unchecked_into(),
		// 5Ec2hh96mXEavdu2C866hgUC4joBYpBVujXQJgQsDWfUMmM1
		hex!["705c8360296c7b6af2f842e7a0804492c86a855aaa605fdf419f577f1f4fecbe"].unchecked_into(),
	)];
}

/// Testnet root key
pub fn get_testnet_root_key() -> AccountId {
	// 5G8jA2TLTQqnofx2jCE1MAtaZNqnJf1ujv7LdZBv2LGznJE2
	return hex!["f04eaed79cba531626964ba59d727b670524247c92cdd0b5f5da04c8eccb796b"].into();
}

/// Testnet election members
pub fn get_testnet_election_members() -> Vec<AccountId> {
	return vec![
		// 5EeJqpx6RCQxg13WW2WJt4CPE6w6vSmFSWNBdzYCh2YX7bFU
		hex!["72195640f79f8254ce35db3b5d0b17c0243b0fb4489fa4b04688ed121ba22603"].into(),
		hex!["80d5673f528ec827a9a1ed4bb5b47b737e9dffae3e62e95e104b4f1afc52ec68"].into(),
		hex!["9878e771c7637945322a46ec58ab59ee62d0a308fc38335cbdd98c50fd0fdc41"].into(),

	];
}

/// MTestnet bootnodes (network compatible from Edgeware mainnet launch)
pub fn get_mtestnet_bootnodes() -> Vec<MultiaddrWithPeerId> {
	return vec![
		"/ip4/45.77.148.197/tcp/30333/p2p/QmUeUUGWMthMvmqKVRPKdXsuDMynCQfmZ7sjVMvce9VU7V".parse().unwrap(),
		"/ip4/45.77.106.16/tcp/30333/p2p/QmcfVBingGpGxZX34KW5JNmknTvPXPP5n6yV6VC9sLot4o".parse().unwrap(),
		"/ip4/207.148.19.178/tcp/30333/p2p/QmZFSZSkT5EjPFtocrFmm4qxfnzuz5gen21mNLM4gZ3ETs".parse().unwrap(),
		"/ip4/45.63.20.50/tcp/30333/p2p/QmWuqHRV2FRaGpDW4TYYhwEeUut8Tqurwh5Nx4xWCMhPZq".parse().unwrap(),
		"/ip4/108.61.132.86/tcp/30333/p2p/Qma1a5Wqn8XrVabVkvg5RW7fUQqJrjZoajTMb4wXFBY8ko".parse().unwrap(),
		"/ip4/8.9.37.163/tcp/30333/p2p/QmbZspwqedBb9khWRM7vEeub7CoYcqzGLUg3ZGt2EXVexf".parse().unwrap(),
		"/ip4/149.28.224.192/tcp/30333/p2p/QmTTmfPicKr3qRSbfKCA8UJShD6HMVCt6KVwGhBwJ4X9Bx".parse().unwrap(),
		"/ip4/45.76.7.184/tcp/30333/p2p/Qmb94o15MrsVSXoL2914duanHzWN4kSyg3cUaRptuKeCQz".parse().unwrap(),
		"/ip4/45.63.23.125/tcp/30333/p2p/QmNvjhhpJdJfcaPRk3EyWN9jxuufNWBEPDCJSzF7ViQdYK".parse().unwrap(),
		"/ip4/96.30.192.15/tcp/30333/p2p/QmS3Qi9QbVVdvFp6wtFoMqyKYRSoAWswheyLqWp6vAFDph".parse().unwrap(),
	];
}

/// Testnet initial authorities
pub fn get_mtestnet_initial_authorities() -> Vec<(AccountId, AccountId, GrandpaId, AuraId, ImOnlineId, AuthorityDiscoveryId)> {
	return vec![(
		hex!["d0403d32c41576b2f58c91792913e32ef36549ea8668638f2803ba9021a5c540"].into(),
		hex!["04fc990505c36a1725eba235594c852b8591553e2f0ff00ffc31fc47a000a564"].into(),
		hex!["80a875dda00106ee48795b3f58fea60e297dce90ae8de099a767e83e37a24867"].unchecked_into(),
		hex!["929ff8381a23b32cbc97c789fce25b4023c521e3ef1d440d787ef1fa0924fc4f"].unchecked_into(),
		hex!["6d5dd00530489bddd02540f95c51b7e755442dd9ec44bb8c0abbcf4fe9efb99a"].unchecked_into(),
		hex!["ae5725899c7bf38ee0a7676af1f9d68bd4f24c92b1311a646fa821cdd51ab92e"].unchecked_into(),
	),(
		hex!["be75eba0978208a501c32f13ac9533c623bccaad0e4357c76fc02f872559762e"].into(),
		hex!["7655af5c8313bc9e53c4100be0ac5fe00b028f60bb690cae9b5b6c1a1d489043"].into(),
		hex!["43970f5535a774e1eaac7a92cf58a0038f424422d9d8fb9cb0ee73497a706cec"].unchecked_into(),
		hex!["e6cd805c1380cd03598b32e45537148190931913ec37c303196e2fd65fabe7f1"].unchecked_into(),
		hex!["aa8971133ee02484eabe74996452dbdca2b933431dfe19d51709c3c1d887648b"].unchecked_into(),
		hex!["90fea7ba6bb163d884dbdd8b2ba5b22113189c1d4944b939294b28e662ab4f17"].unchecked_into(),
	), (
		hex!["9cb2224f01ded140fd6ff494dc106c82715697bc83c4c6f33d58f0b3274fc214"].into(),
		hex!["a2f39001e9c1dec6824d7dc7f9f4ff05e967b1dea9c884e19261c487eaeda819"].into(),
		hex!["7415b2ea8dd54a86dc035bfca42e844920192614e8db17a4118f7eb3606322db"].unchecked_into(),
		hex!["e37cefbd9712a8848b355cd57ca23ac129a4d66de15bc476ce33fe65e6b11777"].unchecked_into(),
		hex!["0883d53c64d360d43b29619f11046a51ae0ab10e1a057851e00e77f6f9043b71"].unchecked_into(),
		hex!["9e5f5ce86bddce022c7fa7d11052117526f39fa8b9306b46a6a87e725e5f3110"].unchecked_into(),
	), (
		hex!["d606367a017eed18e1c179dda9eecad1bb6bfbd901ab8bb809e4a7701394c54d"].into(),
		hex!["e2b32900704016d9d9375e5b673a22afa481e865e0fbc1129d3e409f7dbc8e30"].into(),
		hex!["6d12ec818ed65ac5fc3e46c1e4421b7af8f61098dbfc35fed2e37a7d2946d5a3"].unchecked_into(),
		hex!["ae250307973a96b368f1d4fef704dd8a0beb95e16b5363af5bb65e8d9de401cc"].unchecked_into(),
		hex!["afb6f2137d72a5c2511858bc06309918dc3fd3ab33503de10067681973755f9e"].unchecked_into(),
		hex!["94fec49b0b244e2bef6f0c9c4cb4c02c3b055362739bc5d4e54e7fd0525b4d04"].unchecked_into(),
	), (
		hex!["56a031ff9c856c605d5ef165145c7055fa5d4a236b0de3367c51ab3b6aa93a71"].into(),
		hex!["d6d19faba8eb8c42c21287da2805d820b74efa60bc2703ea8e5246c84766c54d"].into(),
		hex!["3b994d10cd1e052546097ae8a41cad1b2441b471f2de7917425dbe84e34160ea"].unchecked_into(),
		hex!["d33194e2bd13b3361ebeeb5a385a9471ec9212ae7bd5220b5b68b98535cafc09"].unchecked_into(),
		hex!["632a979cc1a2608bced771160eed35129ed9372e3bfa04632f8189dc32aae57d"].unchecked_into(),
		hex!["2ed5510d149e2de79bd6fdb9fe8688261b0931a173addd96f40e9c0877cae306"].unchecked_into(),
	), (
		hex!["dcfa10872cbdc30efa5a5cc14612084044c35f367ddd1ed8af5d80958ec84910"].into(),
		hex!["3ade1880c7a80eadf40fe81ced4ab4d1b1cf0560d691f896352e36a756846942"].into(),
		hex!["4205907674a7341551348a16cab18383fa7207c033637df2466317c0aca4876b"].unchecked_into(),
		hex!["6291c8f876525a3eee31d387bd55b1b4fd82a722e4006a54de404f956073c591"].unchecked_into(),
		hex!["3ad6d8c9a8626eff2df6f455976db799b0f1d4524eb00b11768a8513422dc864"].unchecked_into(),
		hex!["12e5b5d715ac0b6dc3f0b2e6f1fb2dee57e8b595bbbd621020ea63ce038aa94c"].unchecked_into(),
	), (
		hex!["e48f98d4bbcafbdd0c4684c4606a2c584abc35d0e186ab8d045aeceabe1c6c4b"].into(),
		hex!["5247cdd57825d2bd7922c582c7c073e97387083a4a5cd0d1ea095fa20ac8393a"].into(),
		hex!["b750fafb7f16e02617b58c2c4a8fc94fff3934bbd5361a78c4f0c9d4e688b3e0"].unchecked_into(),
		hex!["7155fcc6466f89e331c174c656258ac9701ca4290e0fd06383e61336bbc18b1a"].unchecked_into(),
		hex!["2727f5f65caab717de55fcff8871e51951391bd15f46f74404479ffbd155fb51"].unchecked_into(),
		hex!["705277403dcdb765fd68b6a0db71ade446189f5b1e047be6228af51a6fa39c34"].unchecked_into(),
	), (
		hex!["186a149dbd823f37a1d98cc55f61ad06832e2154b48457cc5cb80452014aff5c"].into(),
		hex!["24f65879dee07a7a91ada9e8a024824c5b32af27687d796e40eeb0ca666eef6a"].into(),
		hex!["1da90a0ef8a500510e68ab4713837ac0da1b05b38538fd20a157e3bb5b1d6603"].unchecked_into(),
		hex!["9fd89819442f21949c3f5915dbe961219de42e32947c47669d89fb8b26e5360c"].unchecked_into(),
		hex!["614f8583a6006053ea5529331c3d92603e716398311e899d9137b4c26b70a826"].unchecked_into(),
		hex!["3aec8b7baf0e520fc91b73e8e78d69ce4232bb23ed08def718dd040a396b7712"].unchecked_into(),
	), (
		hex!["9834b2fa930a67ccdab7dbbfb4fcf74416f0fc8c5658f67fe2e3feccf553566d"].into(),
		hex!["86347d327bd5991a471c0eee25a7dab54bf4f441e4aed1ca59f41efae5eaa128"].into(),
		hex!["ba7b5b18baa2fcf8b0aa3f1923a417ffefd3c909005ce530a37fb42abeb3bbb4"].unchecked_into(),
		hex!["4f119bac5db308398feaaba7f16c25cbe26f6bb99b1a72b04d4ebb649d6151dc"].unchecked_into(),
		hex!["f620a289821496f9ffdca490a5a457da493ebadde242fea7a38b4fd1340040a2"].unchecked_into(),
		hex!["a4a20f07d64813595efe5fe6f31627d7386e3673f380b9a1f319b3c168573717"].unchecked_into(),
	), (
		hex!["92c8abefa9fe6c8ee91acea87143436e44095f8252630c175feefdb18e3fb73c"].into(),
		hex!["3279738232083ef63a8f7005916b677aac0efca0d71a9d26cce613b2f58f6d23"].into(),
		hex!["66317ac47aae3c8e8fcdecee852a968be931a89bd9a146d9bb16b345d98b6d1d"].unchecked_into(),
		hex!["64838c72edccba1a0d3ec703a123257e442a82380e8403a316cbdb90123138a4"].unchecked_into(),
		hex!["f1a07551ad066eebd0193cf55e619836dce1dbb9dfda2d9405072eac9031bf9f"].unchecked_into(),
		hex!["a245ab29c25d4e12af0c91bc65f6c72d6ab9b834acd4364576362575077bbd37"].unchecked_into(),
	)];
}