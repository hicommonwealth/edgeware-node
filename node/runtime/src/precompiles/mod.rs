// Copyright (C) 2020 Commonwealth Labs Inc.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! A list of the different weight modules for our runtime.

use evm::{ExitError, ExitSucceed};
pub use pallet_evm::precompiles::{ECRecover, Ripemd160, Sha256};

pub mod ed25519;
pub mod bls;
pub mod blake2;
pub mod modexp;

pub use ed25519::Ed25519;
pub use bls::{
	Bls12G1Add, Bls12G1Mul, Bls12G1MultiExp, Bls12G2Add, Bls12G2Mul,
	Bls12G2MultiExp, Bls12Pairing, Bls12MapFpToG1, Bls12MapFp2ToG2,
};
pub use blake2::Blake2F;
pub use modexp::ModExp;
