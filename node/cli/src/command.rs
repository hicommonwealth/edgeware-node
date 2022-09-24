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

use crate::{chain_spec, service, service::new_partial, Cli, Subcommand};
use edgeware_cli_opt::RpcConfig;
use sc_cli::{ChainSpec, Result, RuntimeVersion, SubstrateCli};
use sc_service::PartialComponents;

impl SubstrateCli for Cli {
	fn impl_name() -> String {
		"Edgeware Node".into()
	}

	fn impl_version() -> String {
		"4.0.0".into()
	}

	fn description() -> String {
		env!("CARGO_PKG_DESCRIPTION").into()
	}

	fn author() -> String {
		env!("CARGO_PKG_AUTHORS").into()
	}

	fn support_url() -> String {
		"https://github.com/edgeware-network/edgeware-node/issues/new".into()
	}

	fn copyright_start_year() -> i32 {
		2017
	}

	fn executable_name() -> String {
		"edgeware".into()
	}

	fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
		Ok(match id {
			"dev" => Box::new(chain_spec::development_config()),
			"multi-dev" | "multi" => Box::new(chain_spec::multi_development_config()),
			"local" => Box::new(chain_spec::local_testnet_config()),
			"testnet-conf" => Box::new(chain_spec::edgeware_testnet_config(
				"Beresheet".to_string(),
				"beresheet_edgeware_testnet".to_string(),
			)),
			"mainnet-conf" => Box::new(chain_spec::edgeware_mainnet_config()),
			"beresheet-v46-fast" => Box::new(chain_spec::edgeware_beresheet_v46_fast()),
			"beresheet-v46" => Box::new(chain_spec::edgeware_beresheet_v46()),
			"beresheet" => Box::new(chain_spec::edgeware_beresheet_official()),
			"edgeware" => Box::new(chain_spec::edgeware_mainnet_official()),
			path => Box::new(chain_spec::ChainSpec::from_json_file(std::path::PathBuf::from(path))?),
		})
	}

	fn native_runtime_version(_: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
		&edgeware_runtime::VERSION
	}
}

/// Parse command line arguments into service configuration.
pub fn run() -> Result<()> {
	let cli = Cli::from_args();

	match &cli.subcommand {
		Some(Subcommand::Benchmark(_cmd)) => {
			Err("Benchmarking wasn't enabled when building the node. \
			You can enable it with `--features runtime-benchmarks`."
				.into())
		}
		Some(Subcommand::Key(cmd)) => cmd.run(&cli),
		Some(Subcommand::Sign(cmd)) => cmd.run(),
		Some(Subcommand::Verify(cmd)) => cmd.run(),
		Some(Subcommand::Vanity(cmd)) => cmd.run(),
		Some(Subcommand::BuildSpec(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
		}
		Some(Subcommand::CheckBlock(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let PartialComponents {
					client,
					task_manager,
					import_queue,
					..
				} = new_partial(&config, &cli)?;
				Ok((cmd.run(client, import_queue), task_manager))
			})
		}
		Some(Subcommand::ExportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let PartialComponents {
					client, task_manager, ..
				} = new_partial(&config, &cli)?;
				Ok((cmd.run(client, config.database), task_manager))
			})
		}
		Some(Subcommand::ExportState(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let PartialComponents {
					client, task_manager, ..
				} = new_partial(&config, &cli)?;
				Ok((cmd.run(client, config.chain_spec), task_manager))
			})
		}
		Some(Subcommand::ImportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let PartialComponents {
					client,
					task_manager,
					import_queue,
					..
				} = new_partial(&config, &cli)?;
				Ok((cmd.run(client, import_queue), task_manager))
			})
		}
		Some(Subcommand::PurgeChain(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run(config.database))
		}
		Some(Subcommand::Revert(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let PartialComponents { client, task_manager, backend, .. } = new_partial(&config,&cli)?;
				let aux_revert = Box::new(move |client,_, blocks| {
					sc_finality_grandpa::revert(client, blocks)?;
					Ok(())
				});
				Ok((cmd.run(client, backend, Some(aux_revert)), task_manager))
			})
		}
		None => {
			let runner = cli.create_runner(&cli.run.base)?;

			let rpc_config = RpcConfig {
				ethapi: cli.run.ethapi.clone(),
				ethapi_max_permits: cli.run.ethapi_max_permits,
				ethapi_trace_max_count: cli.run.ethapi_trace_max_count,
				ethapi_trace_cache_duration: cli.run.ethapi_trace_cache_duration,
				eth_log_block_cache: cli.run.eth_log_block_cache,
				eth_statuses_cache: cli.run.eth_statuses_cache,
				fee_history_limit: cli.run.fee_history_limit,
				max_past_logs: cli.run.max_past_logs,
				relay_chain_rpc_url: None,
			};

			runner.run_node_until_exit(|config| async move {
				match config.role {
					_ => service::new_full(config, &cli, rpc_config),
				}
				.map_err(sc_cli::Error::Service)
			})
		}
	}
}
