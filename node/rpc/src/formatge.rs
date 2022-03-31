// easier to have it here than to have it in frontier //@flipchan
use pallet_ethereum::TransactionValidationError as VError;
use sc_transaction_pool_api::error::{Error as PError, IntoPoolError};
use sp_runtime::transaction_validity::InvalidTransaction;
//use sc_service::IntoPoolError;
//use sc_transaction_pool::error::{Error as PError, IntoPoolError};

pub trait Formatter: Send + Sync + 'static {
	fn pool_error(err: impl IntoPoolError) -> String;
}

// Formatter keeping the same output as before the introduction of this
// formatter design.
pub struct Legacy;

impl Formatter for Legacy {
	fn pool_error(err: impl IntoPoolError) -> String {
		format!("submit transaction to pool failed: {:?}", err)
	}
}

// Formats the same way Geth node formats responses.
pub struct Geth;

impl Formatter for Geth {
	fn pool_error(err: impl IntoPoolError) -> String {
		// Error strings from :
		// https://github.com/ethereum/go-ethereum/blob/794c6133efa2c7e8376d9d141c900ea541790bce/core/error.go
		match err.into_pool_error() {
			Ok(PError::AlreadyImported(_)) => "already known".to_string(),
			// In Frontier the only case there is a `TemporarilyBanned` is because
			// the same transaction was received before and returned
			// `InvalidTransaction::Stale`. Thus we return the same error.
			Ok(PError::TemporarilyBanned) => "nonce too low".into(),
			Ok(PError::TooLowPriority { .. }) => "replacement transaction underpriced".into(),
			Ok(PError::InvalidTransaction(inner)) => match inner {
				InvalidTransaction::Stale => "nonce too low".into(),
				InvalidTransaction::Payment => "insufficient funds for gas * price + value".into(),
				InvalidTransaction::ExhaustsResources => "gas limit reached".into(),
				InvalidTransaction::Custom(inner) => match inner.into() {
					VError::UnknownError => "unknown error".into(),
					VError::InvalidChainId => "invalid chain id".into(),
					VError::InvalidSignature => "invalid sender".into(),
					VError::GasLimitTooLow => "intrinsic gas too low".into(),
					VError::GasLimitTooHigh => "exceeds block gas limit".into(),
					VError::InsufficientFundsForTransfer => {
						"insufficient funds for transfer".into()
					}
				},
				_ => "unknown error".into(),
			},
			err @ _ => format!("submit transaction to pool failed: {:?}", err),
		}
	}
}
