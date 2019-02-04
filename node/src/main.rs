//! Substrate Node Template CLI library.

#![warn(missing_docs)]
#![warn(unused_extern_crates)]

extern crate futures;
#[macro_use]
extern crate error_chain;
extern crate tokio;
#[macro_use]
extern crate log;
extern crate substrate_cli;
extern crate substrate_primitives as primitives;
extern crate substrate_consensus_aura as consensus;
extern crate substrate_client as client;
#[macro_use]
extern crate substrate_network as network;
#[macro_use]
extern crate substrate_executor;
extern crate substrate_transaction_pool as transaction_pool;
#[macro_use]
extern crate substrate_service;
extern crate substrate_inherents;
extern crate substrate_keystore;
extern crate edgeware_runtime;
extern crate node_executor;
extern crate substrate_finality_grandpa as grandpa;
extern crate node_primitives;

use futures::sync::oneshot;
use futures::{future, Future};

use std::cell::RefCell;

mod chain_spec;
mod service;
mod cli;

pub use substrate_cli::{VersionInfo, IntoExit, error};
// handles ctrl-c
struct Exit;
impl cli::IntoExit for Exit {
	type Exit = future::MapErr<oneshot::Receiver<()>, fn(oneshot::Canceled) -> ()>;
	fn into_exit(self) -> Self::Exit {
		// can't use signal directly here because CtrlC takes only `Fn`.
		let (exit_send, exit) = oneshot::channel();

		let exit_send_cell = RefCell::new(Some(exit_send));
		ctrlc::set_handler(move || {
			if let Some(exit_send) = exit_send_cell.try_borrow_mut().expect("signal handler not reentrant; qed").take() {
				exit_send.send(()).expect("Error sending exit notification");
			}
		}).expect("Error setting Ctrl-C handler");

		exit.map_err(drop)
	}
}

quick_main!(run);

fn run() -> cli::error::Result<()> {
	let version = VersionInfo {
		name: "Edgeware Node",
		commit: env!("VERGEN_SHA_SHORT"),
		version: env!("CARGO_PKG_VERSION"),
		executable_name: "edgeware",
		author: "Commonwealth Labs",
		description: "edgeware",
		support_url: "https://github.com/hicommonwealth/edgeware-node/issues/new",
	};
	cli::run(::std::env::args(), Exit, version)
}
