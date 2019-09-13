// Copyright 2018 Commonwealth Labs, Inc.
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
// along with Edgeware.  If not, see <http://www.gnu.org/licenses/>

use edgeware_service as service;

/// The chain specification option.
#[derive(Clone, Debug, PartialEq)]
pub enum ChainSpec {
	/// Whatever the current runtime is, with just Alice as an auth.
	Development,
	/// Whatever the current runtime is, with simple Alice/Bob auths.
	LocalTestnet,
	/// Edgeware testnet V9.
	EdgewareTestnetV9,
	/// Edgeware testnet V8.
	EdgewareTestnetV8,
	/// Edgeware testnet configuration (intermediate build process)
	EdgewareTestnetConfiguration,
	/// Edgeware mainnet configuration (intermediate build process)
	EdgewareMainnetConfiguration,
	/// Edgeware Mainnet
	EdgewareMainnet,
}

impl Default for ChainSpec {
	fn default() -> Self {
		ChainSpec::Development
	}
}

/// Get a chain config from a spec setting.
impl ChainSpec {
	pub(crate) fn load(self) -> Result<service::chain_spec::ChainSpec, String> {
		Ok(match self {
			ChainSpec::EdgewareMainnet => service::chain_spec::edgeware_mainnet(),
			ChainSpec::EdgewareMainnetConfiguration => service::chain_spec::edgeware_mainnet_config()?,
			ChainSpec::EdgewareTestnetConfiguration => service::chain_spec::edgeware_testnet_config()?,
			ChainSpec::EdgewareTestnetV8 => service::chain_spec::edgeware_testnet_v8_config(),
			ChainSpec::EdgewareTestnetV9 => service::chain_spec::edgeware_testnet_v9_config(),
			ChainSpec::Development => service::chain_spec::development_config(),
			ChainSpec::LocalTestnet => service::chain_spec::local_testnet_config(),
		})
	}

	pub(crate) fn from(s: &str) -> Option<Self> {
		match s {
			"dev" => Some(ChainSpec::Development),
			"local" => Some(ChainSpec::LocalTestnet),
			"edgeware-mainnet-conf" => Some(ChainSpec::EdgewareMainnetConfiguration),
			"edgeware" => Some(ChainSpec::EdgewareMainnet),
			"edgeware-testnet-conf" => Some(ChainSpec::EdgewareTestnetConfiguration),
			"edgeware-testnet-v9" => Some(ChainSpec::EdgewareTestnetV9),
			"edgeware-testnet-v8" => Some(ChainSpec::EdgewareTestnetV8),
			"" => Some(ChainSpec::default()),
			_ => None,
		}
	}
}