use sp_std::{fmt::Debug};
use codec::{FullCodec, HasCompact};
use sp_runtime::{
	DispatchResult,
	traits::{
		MaybeSerializeDeserialize, AtLeast32BitUnsigned, Member,
	},
};
use frame_support::Parameter;


/// Abstraction over a fungible asset system.
pub trait FungibleAsset<AccountId> {
	/// The balance of an account.
	type Balance: AtLeast32BitUnsigned + FullCodec + Copy + MaybeSerializeDeserialize + Debug +
		Default + HasCompact;

	type AssetId: Member + Parameter + Default + Copy + HasCompact;

	// PUBLIC IMMUTABLES
	/// The total amount of issuance in the system for a specific asset.
	fn total_issuance(id: Self::AssetId) -> Self::Balance;

	/// The balance of a given account.
	///
	/// This is the only balance that matters in terms of most operations on tokens. It alone
	/// is used to determine the balance when in the contract execution environment. When this
	/// balance falls below the value of `ExistentialDeposit`, then the 'current account' is
	/// deleted: specifically `FreeBalance`.
	///
	/// `system::AccountNonce` is also deleted if `ReservedBalance` is also zero (it also gets
	/// collapsed to zero if it ever becomes less than `ExistentialDeposit`.
	fn balance_of(id: Self::AssetId, who: AccountId) -> Self::Balance;

	// PUBLIC MUTABLES (DANGEROUS)

	/// Transfer some liquid free balance of an asset to another account.
	fn transfer(id: Self::AssetId, from: AccountId, to: AccountId, amount: Self::Balance) -> DispatchResult;
}

pub trait MintableAsset<AccountId>: FungibleAsset<AccountId> {
	/// Increase the total issuance of of a specific asset by `amount` for a specific account.
	///
	/// Returns `Ok` iff the mint was successful.
	/// `Err` with the reason why otherwise.
	fn mint(id: Self::AssetId, beneficiary: AccountId, amount: Self::Balance) -> DispatchResult;
}

pub trait BurnableAsset<AccountId>: FungibleAsset<AccountId> {
	/// Reduce the total number of assets a specific account owns for a specific asset.
	///
	/// Returns `Ok` iff the burn was successful.
	/// `Err` with the reason why otherwise.
	fn burn(id: Self::AssetId, who: AccountId, amount: Self::Balance) -> DispatchResult;
}

pub trait FreezableAsset<AccountId>: FungibleAsset<AccountId> {
	/// Freeze an amount of tokens for a specific account.
	///
	/// Returns `Ok` iff the mint was successful.
	/// `Err` with the reason why otherwise.
	fn freeze(id: Self::AssetId, who: AccountId) -> DispatchResult;

	/// Unfreeze an amount of tokens for a specific account.
	///
	/// Returns `Ok` iff the mint was successful.
	/// `Err` with the reason why otherwise.
	fn thaw(id: Self::AssetId, who: AccountId) -> DispatchResult;
}

pub trait ManageableAsset<AccountId>: FungibleAsset<AccountId> {
	/// Creates the asset without requiring a deposit, but enforcing (min_balance != 0)
	///
	/// Returns `Ok` iff the ownership transfer was successful.
	/// `Err` with the reason why otherwise.
	fn force_create_asset(id: Self::AssetId, owner: AccountId, max_zombies: u32, min_balance: Self::Balance)
		-> DispatchResult;

	/// Destroys the asset if no non-zombie, reference-bearing accounts exist
	/// Requires (zombies_witness >= the_current_number_of_zombie_accounts) as input
	/// This is NOT an O(1) operation!! Linear in zombies_witness.
	///
	/// Returns `Ok` iff the ownership transfer was successful.
	/// `Err` with the reason why otherwise.
	fn force_destroy_asset(id: Self::AssetId, zombies_witness: u32)
		-> DispatchResult;

	/// Set ownership of administrative functions for a specific token.
	///
	/// Returns `Ok` iff the ownership transfer was successful.
	/// `Err` with the reason why otherwise.
	fn set_team(id: Self::AssetId, issuer: AccountId, admin: AccountId, freezer: AccountId) -> DispatchResult;

	/// Set ownership of administrative functions for a specific token.
	///
	/// Returns `Ok` iff the ownership transfer was successful.
	/// `Err` with the reason why otherwise.
	fn set_owner(id: Self::AssetId, owner: AccountId) -> DispatchResult;
}
