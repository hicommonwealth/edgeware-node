use sp_std::{cmp::min, vec::Vec};
use sp_core::H160;
use evm::{ExitError, ExitSucceed};
use pallet_evm::{Precompile};
use pallet_evm::precompiles::ensure_linear_cost;
use ethereum_types::{U256};

fn read_fr(input: &[u8], start_inx: usize) -> Result<bn::Fr, ExitError> {
	bn::Fr::from_slice(&input[start_inx..(start_inx + 32)]).map_err(|_| ExitError::Other("Invalid field element"))
}

fn read_point(input: &[u8], start_inx: usize) -> Result<bn::G1, ExitError> {
	use bn::{Fq, AffineG1, G1, Group};

	let px = Fq::from_slice(&input[start_inx..(start_inx + 32)]).map_err(|_| ExitError::Other("Invalid point x coordinate"))?;
	let py = Fq::from_slice(&input[(start_inx + 32)..(start_inx + 64)]).map_err(|_| ExitError::Other("Invalid point y coordinate"))?;
	Ok(
		if px == Fq::zero() && py == Fq::zero() {
			G1::zero()
		} else {
			AffineG1::new(px, py).map_err(|_| ExitError::Other("Invalid curve point"))?.into()
		}
	)
}

/// The Bn128Add builtin
pub struct Bn128Add;

/// The Bn128Mul builtin
pub struct Bn128Mul;

/// The Bn128Pairing builtin
pub struct Bn128Pairing;

impl Precompile for Bn128Add {
	fn execute(
		input: &[u8],
		target_gas: Option<usize>,
	) -> core::result::Result<(ExitSucceed, Vec<u8>, usize), ExitError> {
		let cost = ensure_linear_cost(target_gas, input.len(), 15, 3)?;

		use bn::AffineG1;

		let p1 = read_point(input, 0)?;
		let p2 = read_point(input, 64)?;

		let mut buf = [0u8; 64];
		if let Some(sum) = AffineG1::from_jacobian(p1 + p2) {
			// point not at infinity
			sum.x().to_big_endian(&mut buf[0..32]).map_err(|_| ExitError::Other("Cannot fail since 0..32 is 32-byte length"))?;
			sum.y().to_big_endian(&mut buf[32..64]).map_err(|_| ExitError::Other("Cannot fail since 32..64 is 32-byte length"))?;
		}

		Ok((ExitSucceed::Returned, buf.to_vec(), cost))
	}
}

impl Precompile for Bn128Mul {
	fn execute(
		input: &[u8],
		target_gas: Option<usize>,
	) -> core::result::Result<(ExitSucceed, Vec<u8>, usize), ExitError> {
		use bn::AffineG1;

		let cost = ensure_linear_cost(target_gas, input.len(), 15, 3)?;

		let p = read_point(input, 0)?;
		let fr = read_fr(input, 64)?;

		let mut buf = [0u8; 64];
		if let Some(sum) = AffineG1::from_jacobian(p * fr) {
			// point not at infinity
			sum.x().to_big_endian(&mut buf[0..32]).map_err(|_| ExitError::Other("Cannot fail since 0..32 is 32-byte length"))?;
			sum.y().to_big_endian(&mut buf[32..64]).map_err(|_| ExitError::Other("Cannot fail since 32..64 is 32-byte length"))?;
		}

		Ok((ExitSucceed::Returned, buf.to_vec(), cost))
	}
}

impl Precompile for Bn128Pairing {
	fn execute(
		input: &[u8],
		target_gas: Option<usize>,
	) -> core::result::Result<(ExitSucceed, Vec<u8>, usize), ExitError> {
		use bn::{AffineG1, AffineG2, Fq, Fq2, pairing_batch, G1, G2, Gt, Group};

		let cost = ensure_linear_cost(target_gas, input.len(), 15, 3)?;

		let ret_val = if input.is_empty() {
			U256::one()
		} else {
			// (a, b_a, b_b - each 64-byte affine coordinates)
			let elements = input.len() / 192;
			let mut vals = Vec::new();
			for idx in 0..elements {
				let a_x = Fq::from_slice(&input[idx*192..idx*192+32])
					.map_err(|_| ExitError::Other("Invalid a argument x coordinate"))?;

				let a_y = Fq::from_slice(&input[idx*192+32..idx*192+64])
					.map_err(|_| ExitError::Other("Invalid a argument y coordinate"))?;

				let b_a_y = Fq::from_slice(&input[idx*192+64..idx*192+96])
					.map_err(|_| ExitError::Other("Invalid b argument imaginary coeff x coordinate"))?;

				let b_a_x = Fq::from_slice(&input[idx*192+96..idx*192+128])
					.map_err(|_| ExitError::Other("Invalid b argument imaginary coeff y coordinate"))?;

				let b_b_y = Fq::from_slice(&input[idx*192+128..idx*192+160])
					.map_err(|_| ExitError::Other("Invalid b argument real coeff x coordinate"))?;

				let b_b_x = Fq::from_slice(&input[idx*192+160..idx*192+192])
					.map_err(|_| ExitError::Other("Invalid b argument real coeff y coordinate"))?;

				let b_a = Fq2::new(b_a_x, b_a_y);
				let b_b = Fq2::new(b_b_x, b_b_y);
				let b = if b_a.is_zero() && b_b.is_zero() {
					G2::zero()
				} else {
					G2::from(AffineG2::new(b_a, b_b).map_err(|_| ExitError::Other("Invalid b argument - not on curve"))?)
				};
				let a = if a_x.is_zero() && a_y.is_zero() {
					G1::zero()
				} else {
					G1::from(AffineG1::new(a_x, a_y).map_err(|_| ExitError::Other("Invalid a argument - not on curve"))?)
				};
				vals.push((a, b));
			};

			let mul = pairing_batch(&vals);

			if mul == Gt::one() {
				U256::one()
			} else {
				U256::zero()
			}
		};

		let mut buf = [0u8; 32];
		ret_val.to_big_endian(&mut buf);
		
		Ok((ExitSucceed::Returned, buf.to_vec(), cost))
	}
}
