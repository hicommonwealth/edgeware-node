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


// 2022 rewrite by flipchan @ edgeware

use sc_cli::{KeySubcommand, SignCmd, VanityCmd, VerifyCmd};
//use structopt::StructOpt;
use edgeware_cli_opt::EthApi;
use clap::Parser;

//use ethereum_types::{H160, H256, U256};

#[allow(missing_docs)]
#[derive(Debug, Parser)]
pub struct RunCmd {
	#[allow(missing_docs)]
	#[clap(flatten)]
	pub base: sc_cli::RunCmd,

	#[clap(long = "enable-dev-signer")]
	pub enable_dev_signer: bool,

	/// The dynamic-fee pallet target gas price set by block author
	#[clap(long, default_value = "1")]
	pub target_gas_price: u64,

	/// Enable EVM tracing module on a non-authority node.
	#[clap(
		long,
		conflicts_with = "validator",
		use_value_delimiter = true,
		require_value_delimiter = true,
		multiple_values = true
	)]
	pub ethapi: Vec<EthApi>,

	/// Number of concurrent tracing tasks. Meant to be shared by both "debug"
	/// and "trace" modules.
	#[clap(long, default_value = "10")]
	pub ethapi_max_permits: u32,

	/// Maximum number of trace entries a single request of `trace_filter` is
	/// allowed to return. A request asking for more or an unbounded one going
	/// over this limit will both return an error.
	#[clap(long, default_value = "500")]
	pub ethapi_trace_max_count: u32,

	/// Duration (in seconds) after which the cache of `trace_filter` for a
	/// given block will be discarded.
	#[clap(long, default_value = "300")]
	pub ethapi_trace_cache_duration: u64,

	/// Size in bytes of the LRU cache for block data.
	#[clap(long, default_value = "300000000")]
	pub eth_log_block_cache: usize,

	/// Size in bytes of the LRU cache for transactions statuses data.
	#[clap(long, default_value = "300000000")]
	pub eth_statuses_cache: usize,

	/// Maximum number of logs in a query.
	#[clap(long, default_value = "10000")]
	pub max_past_logs: u32,

	/// Maximum fee history cache size.
	#[clap(long, default_value = "2048")]
	pub fee_history_limit: u64,
}

/// An overarching CLI command definition.
#[derive(Debug, Parser)]
#[clap(
	propagate_version = true,
	args_conflicts_with_subcommands = true,
	subcommand_negates_reqs = true
)]
pub struct Cli {
	/// Possible subcommand with parameters.
	#[clap(subcommand)]
	pub subcommand: Option<Subcommand>,
	#[allow(missing_docs)]
	#[clap(flatten)]
	pub run: RunCmd,
}

/// Possible subcommands of the main binary.
#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {
	/// Key management cli utilities
	#[clap(subcommand)]
	Key(KeySubcommand),

	/// The custom benchmark subcommmand benchmarking runtime pallets.
	// #[clap(name = "benchmark", about = "Benchmark runtime pallets.")]
	#[clap(subcommand)]
	Benchmark(frame_benchmarking_cli::BenchmarkCmd),

	/// Verify a signature for a message, provided on STDIN, with a given
	/// (public or secret) key.
	Verify(VerifyCmd),

	/// Generate a seed that provides a vanity address.
	Vanity(VanityCmd),

	/// Sign a message, with a given (secret) key.
	Sign(SignCmd),

	/// Build a chain specification.
	BuildSpec(sc_cli::BuildSpecCmd),

	/// Validate blocks.
	CheckBlock(sc_cli::CheckBlockCmd),

	/// Export blocks.
	ExportBlocks(sc_cli::ExportBlocksCmd),

	/// Export the state of a given block into a chain spec.
	ExportState(sc_cli::ExportStateCmd),

	/// Import blocks.
	ImportBlocks(sc_cli::ImportBlocksCmd),

	/// Remove the whole chain.
	PurgeChain(sc_cli::PurgeChainCmd),

	/// Revert the chain to a previous state.
	Revert(sc_cli::RevertCmd),
}
