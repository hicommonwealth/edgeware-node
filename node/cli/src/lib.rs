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

#![warn(missing_docs)]
#![warn(unused_extern_crates)]

use client::ExecutionStrategies;
use edgeware_service as service;
use substrate_service;

#[macro_use]
extern crate log;

mod factory_impl;
mod chain_spec;

use transaction_factory::RuntimeAdapter;
use crate::factory_impl::FactoryState;
use tokio::prelude::Future;
use tokio::runtime::{Builder as RuntimeBuilder, Runtime};
use std::ops::Deref;
use chain_spec::ChainSpec;
use structopt::{StructOpt, clap::App};

use substrate_service::{ServiceFactory, Roles as ServiceRoles};
use substrate_cli as cli;
use exit_future;

pub use service::{Factory};
pub use cli::{AugmentClap, GetLogFilter, parse_and_prepare, ParseAndPrepare};
pub use cli::{VersionInfo, IntoExit, NoCustom, SharedParams, ExecutionStrategyParam};
pub use cli::error;

/// Custom subcommands.
#[derive(Clone, Debug, StructOpt)]
pub enum CustomSubcommands {
	/// The custom factory subcommmand for manufacturing transactions.
	#[structopt(
		name = "factory",
		about = "Manufactures num transactions from Alice to random accounts. \
		Only supported for development or local testnet."
	)]
	Factory(FactoryCmd),
}

impl GetLogFilter for CustomSubcommands {
	fn get_log_filter(&self) -> Option<String> {
		None
	}
}

/// The `factory` command used to generate transactions.
/// Please note: this command currently only works on an empty database!
#[derive(Debug, StructOpt, Clone)]
pub struct FactoryCmd {
	/// How often to repeat. This option only has an effect in mode `MasterToNToM`.
	#[structopt(long="rounds", default_value = "1")]
	pub rounds: u64,

	/// MasterToN: Manufacture `num` transactions from the master account
	///            to `num` randomly created accounts, one each.
	///
	/// MasterTo1: Manufacture `num` transactions from the master account
	///            to exactly one other randomly created account.
	///
	/// MasterToNToM: Manufacture `num` transactions from the master account
	///               to `num` randomly created accounts.
	///               From each of these randomly created accounts manufacture
	///               a transaction to another randomly created account.
	///               Repeat this `rounds` times. If `rounds` = 1 the behavior
	///               is the same as `MasterToN`.{n}
	///               A -> B, A -> C, A -> D, ... x `num`{n}
	///               B -> E, C -> F, D -> G, ...{n}
	///               ... x `rounds`
	///
	/// These three modes control manufacturing.
	#[structopt(long="mode", default_value = "MasterToN")]
	pub mode: transaction_factory::Mode,

	/// Number of transactions to generate. In mode `MasterNToNToM` this is
	/// the number of transactions per round.
	#[structopt(long="num", default_value = "8")]
	pub num: u64,

	#[allow(missing_docs)]
	#[structopt(flatten)]
	pub shared_params: SharedParams,

	/// The means of execution used when calling into the runtime while importing blocks.
	#[structopt(
		long = "execution",
		value_name = "STRATEGY",
		raw(
			possible_values = "&ExecutionStrategyParam::variants()",
			case_insensitive = "true",
			default_value = r#""NativeElseWasm""#
		)
	)]
	pub execution: ExecutionStrategyParam,
}

impl AugmentClap for FactoryCmd {
	fn augment_clap<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
		FactoryCmd::augment_clap(app)
	}
}


fn load_spec(id: &str) -> Result<Option<service::chain_spec::ChainSpec>, String> {
	Ok(match ChainSpec::from(id) {
		Some(spec) => Some(spec.load()?),
		None => None,
	})
}

/// Parse command line arguments into service configuration.
pub fn run<I, T, E>(args: I, exit: E, version: cli::VersionInfo) -> error::Result<()> where
	I: IntoIterator<Item = T>,
	T: Into<std::ffi::OsString> + Clone,
	E: IntoExit,
{
	match parse_and_prepare::<CustomSubcommands, NoCustom, _>(&version, "edgeware-node", args) {
		ParseAndPrepare::Run(cmd) => cmd.run(load_spec, exit, |exit, _cli_args, _custom_args, config| {
			info!("{}", version.name);
			info!("  version {}", config.full_version());
			info!("  by Commonwealth Labs, 2018-2019");
			info!("Chain specification: {}", config.chain_spec.name());
			info!("Node name: {}", config.name);
			info!("Roles: {:?}", config.roles);
			let runtime = RuntimeBuilder::new().name_prefix("main-tokio-").build()
				.map_err(|e| format!("{:?}", e))?;
			match config.roles {
				ServiceRoles::LIGHT => run_until_exit(
					runtime,
					service::Factory::new_light(config).map_err(|e| format!("{:?}", e))?,
					exit
				),
				_ => run_until_exit(
					runtime,
					service::Factory::new_full(config).map_err(|e| format!("{:?}", e))?,
					exit
				),
			}.map_err(|e| format!("{:?}", e))
		}),
		ParseAndPrepare::BuildSpec(cmd) => cmd.run(load_spec),
		ParseAndPrepare::ExportBlocks(cmd) => cmd.run::<service::Factory, _, _>(load_spec, exit),
		ParseAndPrepare::ImportBlocks(cmd) => cmd.run::<service::Factory, _, _>(load_spec, exit),
		ParseAndPrepare::PurgeChain(cmd) => cmd.run(load_spec),
		ParseAndPrepare::RevertChain(cmd) => cmd.run::<service::Factory, _>(load_spec),
		ParseAndPrepare::CustomCommand(CustomSubcommands::Factory(cli_args)) => {
			let mut config = cli::create_config_with_db_path(
				load_spec,
				&cli_args.shared_params,
				&version,
			)?;
			config.execution_strategies = ExecutionStrategies {
				importing: cli_args.execution.into(),
				block_construction: cli_args.execution.into(),
				other: cli_args.execution.into(),
				..Default::default()
			};

			match ChainSpec::from(config.chain_spec.id()) {
				Some(ref c) if c == &ChainSpec::Development || c == &ChainSpec::LocalTestnet => {},
				_ => panic!("Factory is only supported for development and local testnet."),
			}

			let factory_state = FactoryState::new(
				cli_args.mode.clone(),
				cli_args.num,
				cli_args.rounds,
			);
			transaction_factory::factory::<service::Factory, FactoryState<_>>(
				factory_state,
				config,
			).map_err(|e| format!("Error in transaction factory: {}", e))?;

			Ok(())
		}
	}
}

fn run_until_exit<T, C, E>(
	mut runtime: Runtime,
	service: T,
	e: E,
) -> error::Result<()> where
	T: Deref<Target=substrate_service::Service<C>>,
	T: Future<Item = (), Error = substrate_service::error::Error> + Send + 'static,
	C: substrate_service::Components,
	E: IntoExit,
{
	let (exit_send, exit) = exit_future::signal();

	let informant = cli::informant::build(&service);
	runtime.executor().spawn(exit.until(informant).map(|_| ()));

	// we eagerly drop the service so that the internal exit future is fired,
	// but we need to keep holding a reference to the global telemetry guard
	let _telemetry = service.telemetry();

	let service_res = {
		let exit = e.into_exit().map_err(|_| error::Error::Other("Exit future failed.".into()));
		let service = service.map_err(|err| error::Error::Service(err));
		let select = service.select(exit).map(|_| ()).map_err(|(err, _)| err);
		runtime.block_on(select)
	};

	exit_send.fire();

	// TODO [andre]: timeout this future #1318
	let _ = runtime.shutdown_on_idle().wait();

	service_res
}