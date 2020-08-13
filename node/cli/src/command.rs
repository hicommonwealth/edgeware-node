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

use crate::{chain_spec, service};
use crate::cli::Cli;
use sc_cli::{SubstrateCli, RuntimeVersion, ChainSpec, Role};
use sc_service::ServiceParams;
use crate::service::new_full_params;

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
		2019
	}

	fn executable_name() -> String {
		"edgeware".into()
	}

	fn native_runtime_version(_: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
		&edgeware_runtime::VERSION
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
			"beresheet" => Box::new(chain_spec::edgeware_beresheet_official()),
			"edgeware" => Box::new(chain_spec::edgeware_mainnet_official()),
			path => Box::new(chain_spec::ChainSpec::from_json_file(
				std::path::PathBuf::from(path),
			)?),
		})
	}
}

/// Parse and run command line arguments
pub fn run() -> sc_cli::Result<()> {
	let cli = Cli::from_args();

	match &cli.subcommand {
		Some(subcommand) => {
			let runner = cli.create_runner(subcommand)?;
			runner.run_subcommand(subcommand, |config| {
				let (ServiceParams { client, backend, task_manager, import_queue, .. }, ..)
					= new_full_params(config, cli.run.manual_seal)?;
				Ok((client, backend, import_queue, task_manager))
			})
		}
		None => {
			let runner = cli.create_runner(&cli.run.base)?;
			runner.run_node_until_exit(|config| match config.role {
				Role::Light => service::new_light(config),
				_ => service::new_full(config, cli.run.manual_seal),
			})
		}
	}
}
