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
use hex_literal::hex;
use pallet_im_online::ed25519::AuthorityId as ImOnlineId;
use sc_network::config::MultiaddrWithPeerId;
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_aura::ed25519::AuthorityId as AuraId;
use sp_core::crypto::UncheckedInto;
use sp_finality_grandpa::AuthorityId as GrandpaId;

/// Testnet root key
pub fn get_testnet_root_key() -> AccountId {
	// Beresheet sudo key: 5HVniu9naSbxjFLYBUn6aUTpExoxBzweJCQGRT2GZTMsr5a7
	return hex!["f04eaed79cba531626964ba59d727b670524247c92cdd0b5f5da04c8eccb796b"].into();
}

/// Beresheet bootnodes
pub fn get_beresheet_bootnodes() -> Vec<MultiaddrWithPeerId> {
	return vec![
		"/ip4/45.77.148.197/tcp/30333/p2p/12D3KooWGq9cJfDdY3Mg7TZGZpLFTyMkdNB8G2gKCr2HQQgdcxwX"
			.parse()
			.unwrap(),
		"/ip4/45.77.106.16/tcp/30333/p2p/12D3KooWDwSqpLiVERs9rhM6MQdfZun46WyDNJ5ufXDLxvPH65by"
			.parse()
			.unwrap(),
		"/ip4/207.148.19.178/tcp/30333/p2p/12D3KooWQnQPmHNJCDXcNUSKdAe2PE3Y573tYr6wF8pBGXhbiEun"
			.parse()
			.unwrap(),
		"/ip4/45.63.20.50/tcp/30333/p2p/12D3KooWBLzbi2Ur7nxXQfnKoDQAFvgc3jthaDRkGkWBFatZp8KW"
			.parse()
			.unwrap(),
		"/ip4/108.61.132.86/tcp/30333/p2p/12D3KooWAn2uPuZCABHSm4JyhYJt7mSQa1GRrpSWcEkSg4oqY4gm"
			.parse()
			.unwrap(),
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
	return vec![
		(
			hex!["d0403d32c41576b2f58c91792913e32ef36549ea8668638f2803ba9021a5c540"].into(),
			hex!["04fc990505c36a1725eba235594c852b8591553e2f0ff00ffc31fc47a000a564"].into(),
			hex!["80a875dda00106ee48795b3f58fea60e297dce90ae8de099a767e83e37a24867"].unchecked_into(),
			hex!["929ff8381a23b32cbc97c789fce25b4023c521e3ef1d440d787ef1fa0924fc4f"].unchecked_into(),
			hex!["6d5dd00530489bddd02540f95c51b7e755442dd9ec44bb8c0abbcf4fe9efb99a"].unchecked_into(),
			hex!["ae5725899c7bf38ee0a7676af1f9d68bd4f24c92b1311a646fa821cdd51ab92e"].unchecked_into(),
		),
		(
			hex!["be75eba0978208a501c32f13ac9533c623bccaad0e4357c76fc02f872559762e"].into(),
			hex!["7655af5c8313bc9e53c4100be0ac5fe00b028f60bb690cae9b5b6c1a1d489043"].into(),
			hex!["43970f5535a774e1eaac7a92cf58a0038f424422d9d8fb9cb0ee73497a706cec"].unchecked_into(),
			hex!["e6cd805c1380cd03598b32e45537148190931913ec37c303196e2fd65fabe7f1"].unchecked_into(),
			hex!["aa8971133ee02484eabe74996452dbdca2b933431dfe19d51709c3c1d887648b"].unchecked_into(),
			hex!["90fea7ba6bb163d884dbdd8b2ba5b22113189c1d4944b939294b28e662ab4f17"].unchecked_into(),
		),
		(
			hex!["9cb2224f01ded140fd6ff494dc106c82715697bc83c4c6f33d58f0b3274fc214"].into(),
			hex!["a2f39001e9c1dec6824d7dc7f9f4ff05e967b1dea9c884e19261c487eaeda819"].into(),
			hex!["7415b2ea8dd54a86dc035bfca42e844920192614e8db17a4118f7eb3606322db"].unchecked_into(),
			hex!["e37cefbd9712a8848b355cd57ca23ac129a4d66de15bc476ce33fe65e6b11777"].unchecked_into(),
			hex!["0883d53c64d360d43b29619f11046a51ae0ab10e1a057851e00e77f6f9043b71"].unchecked_into(),
			hex!["9e5f5ce86bddce022c7fa7d11052117526f39fa8b9306b46a6a87e725e5f3110"].unchecked_into(),
		),
		(
			hex!["d606367a017eed18e1c179dda9eecad1bb6bfbd901ab8bb809e4a7701394c54d"].into(),
			hex!["e2b32900704016d9d9375e5b673a22afa481e865e0fbc1129d3e409f7dbc8e30"].into(),
			hex!["6d12ec818ed65ac5fc3e46c1e4421b7af8f61098dbfc35fed2e37a7d2946d5a3"].unchecked_into(),
			hex!["ae250307973a96b368f1d4fef704dd8a0beb95e16b5363af5bb65e8d9de401cc"].unchecked_into(),
			hex!["afb6f2137d72a5c2511858bc06309918dc3fd3ab33503de10067681973755f9e"].unchecked_into(),
			hex!["94fec49b0b244e2bef6f0c9c4cb4c02c3b055362739bc5d4e54e7fd0525b4d04"].unchecked_into(),
		),
		(
			hex!["56a031ff9c856c605d5ef165145c7055fa5d4a236b0de3367c51ab3b6aa93a71"].into(),
			hex!["d6d19faba8eb8c42c21287da2805d820b74efa60bc2703ea8e5246c84766c54d"].into(),
			hex!["3b994d10cd1e052546097ae8a41cad1b2441b471f2de7917425dbe84e34160ea"].unchecked_into(),
			hex!["d33194e2bd13b3361ebeeb5a385a9471ec9212ae7bd5220b5b68b98535cafc09"].unchecked_into(),
			hex!["632a979cc1a2608bced771160eed35129ed9372e3bfa04632f8189dc32aae57d"].unchecked_into(),
			hex!["2ed5510d149e2de79bd6fdb9fe8688261b0931a173addd96f40e9c0877cae306"].unchecked_into(),
		),
	];
}
