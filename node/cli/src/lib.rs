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

//! Substrate CLI library.
//!
//! This package has two Cargo features:
//!
//! - `cli` (default): exposes functions that parse command-line options, then start and run the
//! node as a CLI application.
//!
//! - `browser`: exposes the content of the `browser` module, which consists of exported symbols
//! that are meant to be passed through the `wasm-bindgen` utility and called from JavaScript.
//! Despite its name the produced WASM can theoretically also be used from NodeJS, although this
//! hasn't been tested.

#![warn(missing_docs)]

pub mod chain_spec;

#[macro_use]
mod service;
#[cfg(feature = "browser")]
mod browser;
#[cfg(feature = "cli")]
mod cli;
#[cfg(feature = "cli")]
mod factory_impl;
#[cfg(feature = "cli")]
mod command;

#[cfg(feature = "browser")]
pub use browser::*;
#[cfg(feature = "cli")]
pub use cli::*;
#[cfg(feature = "cli")]
pub use command::*;

pub mod mainnet_fixtures;
pub mod testnet_fixtures;

/// The chain specification option.
#[derive(Clone, Debug, PartialEq)]
pub enum ChainSpec {
	/// Whatever the current runtime is, with just Alice as an auth.
	Development,
	/// Whatever the current runtime is, with just Alice as an auth.
	MultiNodeDevelopment,
	/// Whatever the current runtime is, with simple Alice/Bob auths.
	LocalTestnet,
	/// Edgeware testnet configuration
	EdgewareTestnetConfig,
	/// 0.9.9 Testnet
	EDGTestnet099,
	/// 1.0.0 Testnet,
	BerlinTestnet,
	/// Edgeware mainnet configuration (should be used to generate chainspec)
	EdgewareMainnetConfig,
	/// Edgeware mainnet (should be used to connect to Edgeware)
	EdgewareMainnet
}

/// Get a chain config from a spec setting.
impl ChainSpec {
	pub(crate) fn load(self) -> Result<chain_spec::ChainSpec, String> {
		Ok(match self {
			ChainSpec::Development => chain_spec::development_config(),
			ChainSpec::MultiNodeDevelopment => chain_spec::multi_development_config(),
			ChainSpec::LocalTestnet => chain_spec::local_testnet_config(),
			ChainSpec::EdgewareTestnetConfig => chain_spec::edgeware_testnet_config(
				"Berlin".to_string(),
				"berlin_edgeware_testnet".to_string(),
			),
			ChainSpec::EDGTestnet099 => chain_spec::edgeware_testnet_v099_config(),
			ChainSpec::BerlinTestnet => chain_spec::edgeware_berlin_testnet_config(),
			ChainSpec::EdgewareMainnetConfig => chain_spec::edgeware_mainnet_config(),
			ChainSpec::EdgewareMainnet => chain_spec::edgeware_mainnet_official(),
		})
	}

	pub(crate) fn from(s: &str) -> Option<Self> {
		match s {
			"dev" => Some(ChainSpec::Development),
			"multi-dev" => Some(ChainSpec::MultiNodeDevelopment),
			"local" => Some(ChainSpec::LocalTestnet),
			"edgeware-testnet" => Some(ChainSpec::EdgewareTestnetConfig),
			"edg-0.9.9" => Some(ChainSpec::EDGTestnet099),
			"berlin" => Some(ChainSpec::BerlinTestnet),
			"edgeware-mainnet" => Some(ChainSpec::EdgewareMainnetConfig),
			"edgeware" => Some(ChainSpec::EdgewareMainnet),
			_ => None,
		}
	}
}

fn load_spec(id: &str) -> Result<Option<chain_spec::ChainSpec>, String> {
	Ok(match ChainSpec::from(id) {
		Some(spec) => Some(spec.load()?),
		None => None,
	})
}
