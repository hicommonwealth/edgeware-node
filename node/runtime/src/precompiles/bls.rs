use sp_std::{cmp::min, vec::Vec};
use sp_core::H160;
use evm::{ExitError, ExitSucceed};
use pallet_evm::{Precompile};
use pallet_evm::precompiles::ensure_linear_cost;
// use eth_pairings::public_interface::eip2537::{EIP2537Executor};

/// The Bls12G1Add builtin.
pub struct Bls12G1Add;

/// The Bls12G1Mul builtin.
pub struct Bls12G1Mul;

/// The Bls12G1MultiExp builtin.
pub struct Bls12G1MultiExp;

/// The Bls12G2Add builtin.
pub struct Bls12G2Add;

/// The Bls12G2Mul builtin.
pub struct Bls12G2Mul;

/// The Bls12G2MultiExp builtin.
pub struct Bls12G2MultiExp;

/// The Bls12Pairing builtin.
pub struct Bls12Pairing;

/// The Bls12MapFpToG1 builtin.
pub struct Bls12MapFpToG1;

/// The Bls12MapFp2ToG2 builtin.
pub struct Bls12MapFp2ToG2;

impl Precompile for Bls12G1Add {
	fn execute(
		input: &[u8],
		target_gas: Option<usize>,
	) -> core::result::Result<(ExitSucceed, Vec<u8>, usize), ExitError> {
		let cost = ensure_linear_cost(target_gas, input.len(), 15, 3)?;

		// let result = EIP2537Executor::g1_add(input);

		// match result {
		// 	Ok(result_bytes) => {
		// 		Ok((ExitSucceed::Returned, result_bytes.to_vec(), cost))
		// 	},
		// 	Err(e) => {
		// 		Err(ExitError::Other("Bls12G1Add error"))
		// 	}
		
		Ok((ExitSucceed::Returned, input.to_vec(), cost))// }
	}
}

impl Precompile for Bls12G1Mul {
	fn execute(
		input: &[u8],
		target_gas: Option<usize>,
	) -> core::result::Result<(ExitSucceed, Vec<u8>, usize), ExitError> {
		let cost = ensure_linear_cost(target_gas, input.len(), 15, 3)?;

		// let result = EIP2537Executor::g1_mul(input);

		// match result {
		// 	Ok(result_bytes) => {
		// 		Ok((ExitSucceed::Returned, result_bytes.to_vec(), cost))
		// 	},
		// 	Err(e) => {
		// 		Err(ExitError::Other("Bls12G1Mul error"))
		// 	}
		// }
		Ok((ExitSucceed::Returned, input.to_vec(), cost))
	}
}

impl Precompile for Bls12G1MultiExp {
	fn execute(
		input: &[u8],
		target_gas: Option<usize>,
	) -> core::result::Result<(ExitSucceed, Vec<u8>, usize), ExitError> {
		let cost = ensure_linear_cost(target_gas, input.len(), 15, 3)?;

		// let result = EIP2537Executor::g1_multiexp(input);

		// match result {
		// 	Ok(result_bytes) => {
		// 		Ok((ExitSucceed::Returned, result_bytes.to_vec(), cost))
		// 	},
		// 	Err(e) => {
		// 		Err(ExitError::Other("Bls12G1MultiExp error"))
		// 	}
		// }
		Ok((ExitSucceed::Returned, input.to_vec(), cost))
	}
}

impl Precompile for Bls12G2Add {
	fn execute(
		input: &[u8],
		target_gas: Option<usize>,
	) -> core::result::Result<(ExitSucceed, Vec<u8>, usize), ExitError> {
		let cost = ensure_linear_cost(target_gas, input.len(), 15, 3)?;

		// let result = EIP2537Executor::g2_add(input);

		// match result {
		// 	Ok(result_bytes) => {
		// 		Ok((ExitSucceed::Returned, result_bytes.to_vec(), cost))
		// 	},
		// 	Err(e) => {
		// 		Err(ExitError::Other("Bls12G2Add error"))
		// 	}
		// }
		Ok((ExitSucceed::Returned, input.to_vec(), cost))
	}
}

impl Precompile for Bls12G2Mul {
	fn execute(
		input: &[u8],
		target_gas: Option<usize>,
	) -> core::result::Result<(ExitSucceed, Vec<u8>, usize), ExitError> {
		let cost = ensure_linear_cost(target_gas, input.len(), 15, 3)?;

		// let result = EIP2537Executor::g2_mul(input);

		// match result {
		// 	Ok(result_bytes) => {
		// 		Ok((ExitSucceed::Returned, result_bytes.to_vec(), cost))
		// 	},
		// 	Err(e) => {
		// 		Err(ExitError::Other("Bls12G2Mul error"))
		// 	}
		// }
		Ok((ExitSucceed::Returned, input.to_vec(), cost))
	}
}

impl Precompile for Bls12G2MultiExp {
	fn execute(
		input: &[u8],
		target_gas: Option<usize>,
	) -> core::result::Result<(ExitSucceed, Vec<u8>, usize), ExitError> {
		let cost = ensure_linear_cost(target_gas, input.len(), 15, 3)?;

		// let result = EIP2537Executor::g2_multiexp(input);

		// match result {
		// 	Ok(result_bytes) => {
		// 		Ok((ExitSucceed::Returned, result_bytes.to_vec(), cost))
		// 	},
		// 	Err(e) => {
		// 		Err(ExitError::Other("Bls12G2MultiExp error"))
		// 	}
		// }
		Ok((ExitSucceed::Returned, input.to_vec(), cost))
	}
}

impl Precompile for Bls12Pairing {
	fn execute(
		input: &[u8],
		target_gas: Option<usize>,
	) -> core::result::Result<(ExitSucceed, Vec<u8>, usize), ExitError> {
		let cost = ensure_linear_cost(target_gas, input.len(), 15, 3)?;

		// let result = EIP2537Executor::pair(input);

		// match result {
		// 	Ok(result_bytes) => {
		// 		Ok((ExitSucceed::Returned, result_bytes.to_vec(), cost))
		// 	},
		// 	Err(e) => {
		// 		Err(ExitError::Other("Bls12Pairing error"))
		// 	}
		// }
		Ok((ExitSucceed::Returned, input.to_vec(), cost))
	}
}

impl Precompile for Bls12MapFpToG1 {
	fn execute(
		input: &[u8],
		target_gas: Option<usize>,
	) -> core::result::Result<(ExitSucceed, Vec<u8>, usize), ExitError> {
		let cost = ensure_linear_cost(target_gas, input.len(), 15, 3)?;

		// let result = EIP2537Executor::map_fp_to_g1(input);

		// match result {
		// 	Ok(result_bytes) => {
		// 		Ok((ExitSucceed::Returned, result_bytes.to_vec(), cost))
		// 	},
		// 	Err(e) => {
		// 		Err(ExitError::Other("Bls12MapFpToG1 error"))
		// 	}
		// }
		Ok((ExitSucceed::Returned, input.to_vec(), cost))
	}
}

impl Precompile for Bls12MapFp2ToG2 {
	fn execute(
		input: &[u8],
		target_gas: Option<usize>,
	) -> core::result::Result<(ExitSucceed, Vec<u8>, usize), ExitError> {
		let cost = ensure_linear_cost(target_gas, input.len(), 15, 3)?;

		// let result = EIP2537Executor::map_fp_to_g2(input);

		// match result {
		// 	Ok(result_bytes) => {
		// 		Ok((ExitSucceed::Returned, result_bytes.to_vec(), cost))
		// 	},
		// 	Err(e) => {
		// 		Err(ExitError::Other("Bls12MapFp2ToG2 error"))
		// 	}
		// }
		Ok((ExitSucceed::Returned, input.to_vec(), cost))
	}
}