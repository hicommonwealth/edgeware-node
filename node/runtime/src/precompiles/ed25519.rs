use sp_std::{cmp::min, vec::Vec};
use evm::{ExitError, ExitSucceed};
use pallet_evm::{Precompile};
use pallet_evm::precompiles::ensure_linear_cost;
use sp_application_crypto::TryFrom;
use ed25519_dalek::{PublicKey, Verifier, Signature};

/// The identity precompile.
pub struct Ed25519;

impl Precompile for Ed25519 {
	fn execute(
		input: &[u8],
		target_gas: Option<usize>,
	) -> core::result::Result<(ExitSucceed, Vec<u8>, usize), ExitError> {
		let cost = ensure_linear_cost(target_gas, input.len(), 15, 3)?;

		let len = min(input.len(), 128);

		let mut i = [0u8; 128];
		i[..len].copy_from_slice(&input[..len]);

		let mut buf = [0u8; 4];

		let msg = &i[0..32];
		let pk = PublicKey::from_bytes(&i[32..64])
			.map_err(|_| ExitError::Other("Public key recover failed"))?;
		let sig = Signature::try_from(&i[64..128])
			.map_err(|_| ExitError::Other("Signature recover failed"))?;

		// https://docs.rs/rust-crypto/0.2.36/crypto/ed25519/fn.verify.html
		if pk.verify(msg, &sig).is_ok() {
			buf[3] = 0u8;
		} else {
			buf[3] = 1u8;
		};

		Ok((ExitSucceed::Returned, buf.to_vec(), cost))
	}
}
