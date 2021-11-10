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

//! A `CodeExecutor` specialization which uses natively compiled runtime when
//! the wasm to be executed is equivalent to the natively compiled code.

use edgeware_runtime_interface;
pub use sc_executor::NativeElseWasmExecutor;

// Declare an instance of the native executor named `ExecutorDispatch`. Include
// the wasm binary as the equivalent wasm code.
pub struct EdgewareExecutor;

impl sc_executor::NativeExecutionDispatch for EdgewareExecutor {
	type ExtendHostFunctions = (
		frame_benchmarking::benchmarking::HostFunctions,
		edgeware_runtime_interface::storage::HostFunctions,
	);

	fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
		edgeware_runtime::api::dispatch(method, data)
	}

	fn native_version() -> sc_executor::NativeVersion {
		edgeware_runtime::native_version()
	}
}
