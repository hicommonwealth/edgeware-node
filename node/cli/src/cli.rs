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

use sc_cli::{KeySubcommand, SignCmd, VanityCmd, VerifyCmd};
use structopt::StructOpt;
use edgeware_cli_opt::EthApi;

#[allow(missing_docs)]
#[derive(Debug, StructOpt)]
pub struct RunCmd {
	#[allow(missing_docs)]
	#[structopt(flatten)]
	pub base: sc_cli::RunCmd,

	#[structopt(long = "enable-dev-signer")]
	pub enable_dev_signer: bool,

	/// The dynamic-fee pallet target gas price set by block author
	#[structopt(long, default_value = "1")]
	pub target_gas_price: u64,

	/// Enable EVM tracing module on a non-authority node.
	#[structopt(
		long,
		conflicts_with = "collator",
		conflicts_with = "validator",
		require_delimiter = true
	)]
	pub ethapi: Vec<EthApi>,

	/// Number of concurrent tracing tasks. Meant to be shared by both "debug"
	/// and "trace" modules.
	#[structopt(long, default_value = "10")]
	pub ethapi_max_permits: u32,

	/// Maximum number of trace entries a single request of `trace_filter` is
	/// allowed to return. A request asking for more or an unbounded one going
	/// over this limit will both return an error.
	#[structopt(long, default_value = "500")]
	pub ethapi_trace_max_count: u32,

	/// Duration (in seconds) after which the cache of `trace_filter` for a
	/// given block will be discarded.
	#[structopt(long, default_value = "300")]
	pub ethapi_trace_cache_duration: u64,

	/// Size of the LRU cache for block data and their transaction statuses.
	#[structopt(long, default_value = "3000")]
	pub eth_log_block_cache: usize,

	/// Maximum number of logs in a query.
	#[structopt(long, default_value = "10000")]
	pub max_past_logs: u32,
}

/// An overarching CLI command definition.
#[derive(Debug, StructOpt)]
pub struct Cli {
	/// Possible subcommand with parameters.
	#[structopt(subcommand)]
	pub subcommand: Option<Subcommand>,
	#[allow(missing_docs)]
	#[structopt(flatten)]
	pub run: RunCmd,
}

/// Possible subcommands of the main binary.
#[derive(Debug, StructOpt)]
pub enum Subcommand {
	/// Key management cli utilities
	Key(KeySubcommand),

	/// The custom benchmark subcommmand benchmarking runtime pallets.
	#[structopt(name = "benchmark", about = "Benchmark runtime pallets.")]
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
