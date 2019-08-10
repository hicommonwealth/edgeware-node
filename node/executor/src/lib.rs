// Copyright 2018-2019 Commonwealth Labs, Inc.
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

//! A `CodeExecutor` specialization which uses natively compiled runtime when the wasm to be
//! executed is equivalent to the natively compiled code.

#![cfg_attr(feature = "benchmarks", feature(test))]

#[cfg(feature = "benchmarks")] extern crate test;

pub use substrate_executor::NativeExecutor;
use substrate_executor::native_executor_instance;

// Declare an instance of the native executor named `Executor`. Include the wasm binary as the
// equivalent wasm code.
native_executor_instance!(
	pub Executor,
	edgeware_runtime::api::dispatch,
	edgeware_runtime::native_version,
	edgeware_runtime::WASM_BINARY
);
