use sp_std::{cmp::min, vec::Vec};
use sp_core::H160;
use evm::{ExitError, ExitSucceed};
use pallet_evm::{Precompile};
use pallet_evm::precompiles::ensure_linear_cost;

/// The identity precompile.
pub struct Ed25519;

impl Precompile for Ed25519 {
	fn execute(
		input: &[u8],
		target_gas: Option<usize>,
	) -> core::result::Result<(ExitSucceed, Vec<u8>, usize), ExitError> {
		let cost = ensure_linear_cost(target_gas, input.len(), 15, 3)?;

		Ok((ExitSucceed::Returned, input.to_vec(), cost))
	}
}
