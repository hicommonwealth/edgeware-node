use sp_std::{cmp::min, vec::Vec};
use sp_core::H160;
use evm::{ExitError, ExitSucceed};
use pallet_evm::{Precompile};
use pallet_evm::precompiles::ensure_linear_cost;
use blake2_rfc::blake2b::Blake2b;

/// The identity precompile.
pub struct Blake2F;

impl Precompile for Blake2F {
	fn execute(
		input: &[u8],
		target_gas: Option<usize>,
	) -> core::result::Result<(ExitSucceed, Vec<u8>, usize), ExitError> {
		let cost = ensure_linear_cost(target_gas, input.len(), 15, 3)?;
        let mut state = Blake2b::new(64);
        state.update(&input);
        let ret = state.finalize().as_bytes().to_vec();
		Ok((ExitSucceed::Returned, ret, cost))
	}
}