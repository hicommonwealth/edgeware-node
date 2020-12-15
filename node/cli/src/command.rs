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

use crate::{chain_spec, service, Cli, Subcommand};
use edgeware_executor::Executor;
use edgeware_runtime::{Block, RuntimeApi};
use sc_cli::{Result, SubstrateCli, RuntimeVersion, Role, ChainSpec};
use sc_service::PartialComponents;
use crate::service::{new_partial};

fn get_exec_name() -> Option<String> {
	std::env::current_exe()
		.ok()
		.and_then(|pb| pb.file_name().map(|s| s.to_os_string()))
		.and_then(|s| s.into_string().ok())
}

impl SubstrateCli for Cli {
	fn impl_name() -> String {
		"Edgeware Node".into()
	}

	fn impl_version() -> String {
		env!("SUBSTRATE_CLI_IMPL_VERSION").into()
	}

	fn description() -> String {
		env!("CARGO_PKG_DESCRIPTION").into()
	}

	fn author() -> String {
		env!("CARGO_PKG_AUTHORS").into()
	}

	fn support_url() -> String {
		"https://github.com/hicommonwealth/edgeware-node/issues/new".into()
	}

	fn copyright_start_year() -> i32 {
		2017
	}

	fn executable_name() -> String {
		"edgeware".into()
	}

	fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
		let id = if id.is_empty() {
			let n = get_exec_name().unwrap_or_default();
			["edgeware", "beresheet", "development"]
				.iter()
				.cloned()
				.find(|&chain| n.starts_with(chain))
				.unwrap_or("edgeware")
		} else {
			id
		};

		Ok(match id {
			#[cfg(feature = "with-development-runtime")]
			"dev" => Box::new(chain_spec::development::development_config()),
			#[cfg(feature = "with-development-runtime")]
			"multi-dev" | "multi" => Box::new(chain_spec::development::multi_development_config()),
			#[cfg(feature = "with-development-runtime")]
			"local" => Box::new(chain_spec::development::local_testnet_config()),
			#[cfg(feature = "with-beresheet-runtime")]
			"testnet-conf" => Box::new(chain_spec::beresheet::edgeware_testnet_config(
				"Beresheet".to_string(),
				"beresheet_edgeware_testnet".to_string(),
			)),
			#[cfg(feature = "with-beresheet-runtime")]
			"beresheet" => Box::new(chain_spec::beresheet::edgeware_beresheet_official()),
			#[cfg(feature = "with-mainnet-runtime")]
			"mainnet-conf" => Box::new(chain_spec::mainnet::edgeware_mainnet_config()),
			#[cfg(feature = "with-mainnet-runtime")]
			"edgeware" => Box::new(chain_spec::mainnet::edgeware_mainnet_official()),
			path => {
				let path = std::path::PathBuf::from(path);

				let starts_with = |prefix: &str| {
					path.file_name()
						.map(|f| f.to_str().map(|s| s.starts_with(&prefix)))
						.flatten()
						.unwrap_or(false)
				};

				if starts_with("mainnet") || starts_with("edgeware") {
					#[cfg(feature = "with-mainnet-runtime")]
					{
						Box::new(chain_spec::mainnet::ChainSpec::from_json_file(path)?)
					}

					#[cfg(not(feature = "with-mainnet-runtime"))]
					return Err("Mainnet runtime is not available. Please compile the node with `--features with-mainnet-runtime` to enable it.".into());
				} else if starts_with("beresheet") {
					#[cfg(feature = "with-beresheet-runtime")]
					{
						Box::new(chain_spec::beresheet::ChainSpec::from_json_file(path)?)
					}
					#[cfg(not(feature = "with-beresheet-runtime"))]
					return Err("Beresheet runtime is not available. Please compile the node with `--features with-beresheet-runtime` to enable it.".into());
				} else {
					#[cfg(feature = "with-development-runtime")]
					{
						Box::new(chain_spec::development::ChainSpec::from_json_file(path)?)
					}
					#[cfg(not(feature = "with-development-runtime"))]
					return Err("Development runtime is not available. Please compile the node with `--features with-development-runtime` to enable it.".into());
				}
			}
		})
	}

	fn native_runtime_version(_: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
		if spec.is_mainnet() {
			#[cfg(feature = "with-mainnet-runtime")]
			return &service::acala_runtime::VERSION;
			#[cfg(not(feature = "with-mainnet-runtime"))]
			panic!("Mainnet runtime is not available. Please compile the node with `--features with-mainnet-runtime` to enable it.");
		} else if spec.is_beresheet() {
			#[cfg(feature = "with-beresheet-runtime")]
			return &service::beresheet_runtime::VERSION;
			#[cfg(not(feature = "with-beresheet-runtime"))]
			panic!("Beresheet runtime is not available. Please compile the node with `--features with-beresheet-runtime` to enable it.");
		} else {
			#[cfg(feature = "with-development-runtime")]
			return &service::development_runtime::VERSION;
			#[cfg(not(feature = "with-development-runtime"))]
			panic!("Development runtime is not available. Please compile the node with `--features with-development-runtime` to enable it.");
		}
	}
}

/// Parse command line arguments into service configuration.
pub fn run() -> Result<()> {
	let cli = Cli::from_args();

	match &cli.subcommand {
		None => {
			let runner = cli.create_runner(&cli.run.base)?;
			runner.run_node_until_exit(|config| async move {
				match config.role {
					Role::Light => service::new_light(config),
					_ => service::new_full(config, cli.run.enable_dev_signer),
				}
			})
		}
		Some(Subcommand::Inspect(cmd)) => {
			let runner = cli.create_runner(cmd)?;

			runner.sync_run(|config| cmd.run::<Block, RuntimeApi, Executor>(config))
		}
		Some(Subcommand::Benchmark(cmd)) => {
			if cfg!(feature = "runtime-benchmarks") {
				let runner = cli.create_runner(cmd)?;

				runner.sync_run(|config| cmd.run::<Block, Executor>(config))
			} else {
				Err("Benchmarking wasn't enabled when building the node. \
				You can enable it with `--features runtime-benchmarks`.".into())
			}
		}
		Some(Subcommand::Key(cmd)) => cmd.run(),
		Some(Subcommand::Sign(cmd)) => cmd.run(),
		Some(Subcommand::Verify(cmd)) => cmd.run(),
		Some(Subcommand::Vanity(cmd)) => cmd.run(),
		Some(Subcommand::BuildSpec(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
		},
		Some(Subcommand::CheckBlock(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let PartialComponents { client, task_manager, import_queue, ..}
					= new_partial(&config)?;
				Ok((cmd.run(client, import_queue), task_manager))
			})
		},
		Some(Subcommand::ExportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let PartialComponents { client, task_manager, ..}
					= new_partial(&config)?;
				Ok((cmd.run(client, config.database), task_manager))
			})
		},
		Some(Subcommand::ExportState(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let PartialComponents { client, task_manager, ..}
					= new_partial(&config)?;
				Ok((cmd.run(client, config.chain_spec), task_manager))
			})
		},
		Some(Subcommand::ImportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let PartialComponents { client, task_manager, import_queue, ..}
					= new_partial(&config)?;
				Ok((cmd.run(client, import_queue), task_manager))
			})
		},
		Some(Subcommand::PurgeChain(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run(config.database))
		},
		Some(Subcommand::Revert(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let PartialComponents { client, task_manager, backend, ..}
					= new_partial(&config)?;
				Ok((cmd.run(client, backend), task_manager))
			})
		},
	}
}
