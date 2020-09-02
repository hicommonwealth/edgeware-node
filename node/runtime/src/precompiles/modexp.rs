use sp_std::{cmp::max, vec::Vec};
use evm::{ExitError, ExitSucceed};
use pallet_evm::{Precompile};
use pallet_evm::precompiles::ensure_linear_cost;
use num::{BigUint, Zero, One};

pub struct Modexp;

impl Precompile for Modexp {
	fn execute(
		input: &[u8],
		target_gas: Option<usize>,
	) -> core::result::Result<(ExitSucceed, Vec<u8>, usize), ExitError> {
		let cost = ensure_linear_cost(target_gas, input.len(), 15, 3)?;
		let mut buf = [0; 32];
		buf.copy_from_slice(&input[0..32]);
		let mut len_bytes = [0u8; 8];
		len_bytes.copy_from_slice(&buf[24..]);
		let base_len = u64::from_be_bytes(len_bytes) as usize;

		buf = [0; 32];
		buf.copy_from_slice(&input[32..64]);
		len_bytes = [0u8; 8];
		len_bytes.copy_from_slice(&buf[24..]);
		let exp_len = u64::from_be_bytes(len_bytes) as usize;

		buf = [0; 32];
		buf.copy_from_slice(&input[64..96]);
		len_bytes = [0u8; 8];
		len_bytes.copy_from_slice(&buf[24..]);
		let mod_len = u64::from_be_bytes(len_bytes) as usize;

		// Gas formula allows arbitrary large exp_len when base and modulus are empty, so we need to handle empty base first.
		let r = if base_len == 0 && mod_len == 0 {
			BigUint::zero()
		} else {
			// read the numbers themselves.
			let mut buf = Vec::with_capacity(max(mod_len, max(base_len, exp_len)));
			buf.copy_from_slice(&input[0..base_len]);
			let base = BigUint::from_bytes_be(&buf[..base_len]);

			buf = Vec::with_capacity(max(mod_len, max(base_len, exp_len)));
			buf.copy_from_slice(&input[base_len..base_len + exp_len]);
			let exponent = BigUint::from_bytes_be(&buf[..exp_len]);

			buf = Vec::with_capacity(max(mod_len, max(base_len, exp_len)));
			buf.copy_from_slice(&input[(base_len + exp_len)..(base_len + exp_len + mod_len)]);
			let modulus = BigUint::from_bytes_be(&buf[..mod_len]);

			if modulus.is_zero() || modulus.is_one() {
				BigUint::zero()
			} else {
				base.modpow(&exponent, &modulus)
			}
		};

		// write output to given memory, left padded and same length as the modulus.
		let bytes = r.to_bytes_be();

		// always true except in the case of zero-length modulus, which leads to
		// output of length and value 1.
		if bytes.len() <= mod_len {
			let res_start = mod_len - bytes.len();
			let mut ret = Vec::with_capacity(bytes.len() - mod_len);
			ret.copy_from_slice(&bytes[res_start..bytes.len()]);
			Ok((ExitSucceed::Returned, ret.to_vec(), cost))
		} else {
			Err(ExitError::Other("failed"))
		}
	}
}
