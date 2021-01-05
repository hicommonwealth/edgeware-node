#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Encode, Decode};
use frame_support::{decl_error, decl_event, decl_module, decl_storage, ensure, traits::{Get, EnsureOrigin}};
use frame_system::{ensure_none, ensure_signed};
use sp_core::ecdsa;
use sp_io::{crypto::secp256k1_ecdsa_recover, hashing::keccak_256};
use sp_runtime::{
	Permill,
	ModuleId,
	traits::{AccountIdConversion},
	transaction_validity::{
		InvalidTransaction, TransactionPriority, TransactionSource, TransactionValidity, ValidTransaction,
	},
	DispatchResult,
};
use sp_std::vec::Vec;
// use coinaddress as btc_address;
use edge_assets::traits::{FungibleAsset, MintableAsset, BurnableAsset, ManageableAsset};

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
#[cfg(feature = "runtime-benchmarks")]
pub mod benchmarking;

pub mod weights;
pub use weights::WeightInfo;

type EcdsaSignature = ecdsa::Signature;
type DestAddress = Vec<u8>;



pub type TokenIdOf<T> = <<T as Config>::Assets as FungibleAsset<<T as frame_system::Config>::AccountId>>::AssetId;
pub type BalanceOf<T> = <<T as Config>::Assets as FungibleAsset<<T as frame_system::Config>::AccountId>>::Balance;


const NAME_MAX_LENGTH : u8 = 32;

pub trait Config: frame_system::Config {

	/// Ubiquitous Event type
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;

	/// Priority level for  unsigned extrinsics of this pallet
	type RenvmBridgeUnsignedPriority: Get<TransactionPriority>;

	/// The privileged origin for this pallet for token crud and spending
	type ControllerOrigin: EnsureOrigin<Self::Origin>;

	/// The module id for this pallet
	type ModuleId: Get<ModuleId>;

	/// The pallet used for multiple fungible assets
	type Assets: FungibleAsset<Self::AccountId> + MintableAsset<Self::AccountId> + BurnableAsset<Self::AccountId> + ManageableAsset<Self::AccountId>;

	/// Weight information for extrinsics in this pallet.
	type WeightInfo: WeightInfo;
}



/// struct	RenTokenInfo
#[derive(Encode,Decode, Clone, PartialEq, Eq, Debug, Default)]
pub struct RenTokenInfo<TokenIdOf>
	{
	/// ren_token_id		What the Assets pallet uses to identify tokens ( unique for a given token/asset on this pallet for a given chain )
	pub ren_token_id: TokenIdOf,
	/// ren_token_name 		Name of the token; used to determine the validation process if any
	pub ren_token_name: Vec<u8>,
	/// ren_token_renvm_id	What RenVM uses to uniquely identify this token across different chains
	pub ren_token_renvm_id: [u8; 32],
	/// ren_token_pub_key 	The PublicKey used to check the RenVM signature against
	pub ren_token_pub_key: [u8; 20],
	/// ren_token_mint_enabled To enable/disable mint temporarily
	pub ren_token_mint_enabled: bool,
	/// ren_token_burn_enabled To enable/disable burn temporarily
	pub ren_token_burn_enabled: bool,
	/// ren_token_mint_fee	Parts-per-million fee on mint sent to the pallet account
	pub ren_token_mint_fee: u32,
	/// ren_token_burn_fee	Parts-per-million fee on burn sent to the pallet account
	pub ren_token_burn_fee: u32,
}

type RenTokenInfoType<T> = RenTokenInfo<TokenIdOf<T>>;

decl_storage! {
	trait Store for Module<T: Config> as Template {
		/// Signature blacklist. This is required to prevent double claim. Bounded only by the range of EcdsaSignature(s)
		Signatures get(fn signatures): map hasher(opaque_twox_256) EcdsaSignature => Option<()>;
		/// Record burn event details
		BurnEvents get(fn burn_events): map hasher(twox_64_concat) u32 => Option<(TokenIdOf<T>, T::BlockNumber, DestAddress, BalanceOf<T>)>;
		/// Next burn event ID
		NextBurnEventId get(fn next_burn_event_id): u32;
		/// Registry of all active tokens
		RenTokenRegistry get(fn ren_token_registry): map hasher(blake2_128_concat) TokenIdOf<T> => Option<RenTokenInfoType<T>>;
	}
}

decl_event!(
	pub enum Event<T> where
		<T as frame_system::Config>::AccountId,
		TokenId = TokenIdOf<T>,
		Balance = BalanceOf<T>
	{
		/// Event trigger when token is added to the Registry
		RenTokenAdded(TokenId),
		/// Event trigger when token is updates in the Registry
		RenTokenUpdated(TokenId),
		/// Event trigger when token is deleted from the Registry
		RenTokenDeleted(TokenId),
		/// Event trigger when token is spent from the pallet account
		RenTokenSpent(TokenId, AccountId, Balance),
		/// Event trigger when token is minted to a user account
		RenTokenMinted(TokenId, AccountId, Balance),
		/// Event trigger when token is burnt from a user account
		RenTokenBurnt(TokenId, AccountId, DestAddress, Balance),

	}
);

decl_error! {
	pub enum Error for Module<T: Config> {
		/// The mint signature is invalid.
		InvalidMintSignature,
		/// Burn ID overflow.
		BurnIdOverflow,
		/// RenTokenAlready Exists
		RenTokenAlreadyExists,
		/// No token with this ren_token_id found
		RenTokenNotFound,
		/// Asset for the token does not exist
		RenTokenAssetNotFound,
		/// Token name exceeds length limit
		RenTokenNameLengthLimitExceeded,
		/// The to address provided to the burn function failed validation corresponding to the token's name
		InvalidBurnToAddress,
		/// An operation that was not expected to fail failed
		UnexpectedError,
		/// Minting this token has been disabled
		RenTokenMintDisabled,
		/// Burning this token has been disabled
		RenTokenBurnDisabled

	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		type Error = Error<T>;

		fn deposit_event() = default;

		/// Instantiates the token in the Registry AFTER instantiating the asset in Assets
		/// Requries max_zombies and min_balance as inputs to Assets
		/// Max length of the token name implemented
		#[weight = T::WeightInfo::add_ren_token()]
		pub fn add_ren_token(
			origin,
			#[compact] _ren_token_id: TokenIdOf<T>,
			_ren_token_name: Vec<u8>,
			_ren_token_renvm_id: [u8; 32],
			_ren_token_pub_key: [u8; 20],
			_ren_token_mint_enabled: bool,
			_ren_token_burn_enabled: bool,
			_ren_token_mint_fee: u32,
			_ren_token_burn_fee: u32,
			_ren_token_max_zombies: u32,
			_ren_token_min_balance: BalanceOf<T>,
		) -> DispatchResult
		{
			T::ControllerOrigin::ensure_origin(origin)?;

			ensure!(_ren_token_name.len()<=NAME_MAX_LENGTH.into(), Error::<T>::RenTokenNameLengthLimitExceeded);
			ensure!(!<RenTokenRegistry<T>>::contains_key(&_ren_token_id), Error::<T>::RenTokenAlreadyExists);

			T::Assets::force_create_asset(_ren_token_id, Self::account_id(), _ren_token_max_zombies, _ren_token_min_balance)?;

			let _ren_token_info = RenTokenInfo{
				ren_token_id: _ren_token_id,
				ren_token_name: _ren_token_name,
				ren_token_renvm_id: _ren_token_renvm_id,
				ren_token_pub_key: _ren_token_pub_key,
				ren_token_mint_enabled: _ren_token_mint_enabled,
				ren_token_burn_enabled: _ren_token_burn_enabled,
				ren_token_mint_fee: _ren_token_mint_fee,
				ren_token_burn_fee: _ren_token_burn_fee,
			};


			RenTokenRegistry::<T>::insert(&_ren_token_id,_ren_token_info);

			Self::deposit_event(RawEvent::RenTokenAdded(_ren_token_id));
			Ok(())
		}


		/// Method to update a tokens info using its ren_token_id, all other fields are optional
		/// Max length of the token name implemented
		#[weight = T::WeightInfo::update_ren_token()]
		pub fn update_ren_token(
			origin,
			#[compact] _ren_token_id: TokenIdOf<T>,
			_ren_token_name_option: Option<Vec<u8>>,
			_ren_token_renvm_id_option: Option<[u8; 32]>,
			_ren_token_pub_key_option: Option<[u8; 20]>,
			_ren_token_mint_enabled_option: Option<bool>,
			_ren_token_burn_enabled_option: Option<bool>,
			_ren_token_mint_fee_option: Option<u32>,
			_ren_token_burn_fee_option: Option<u32>,
		) -> DispatchResult
		{
			T::ControllerOrigin::ensure_origin(origin)?;

			if let Some(ref x) = _ren_token_name_option
			{ensure!(x.len()<=NAME_MAX_LENGTH.into(), Error::<T>::RenTokenNameLengthLimitExceeded);}

			RenTokenRegistry::<T>::try_mutate_exists(
				&_ren_token_id,
				|maybe_token_info| -> DispatchResult {
					let mut token_info = maybe_token_info.as_mut().ok_or(Error::<T>::RenTokenNotFound)?;

					if let Some(x) = _ren_token_name_option { token_info.ren_token_name = x; }
					if let Some(x) = _ren_token_renvm_id_option { token_info.ren_token_renvm_id = x; }
					if let Some(x) = _ren_token_pub_key_option { token_info.ren_token_pub_key = x; }
					if let Some(x) = _ren_token_mint_enabled_option { token_info.ren_token_mint_enabled = x; }
					if let Some(x) = _ren_token_burn_enabled_option { token_info.ren_token_burn_enabled = x; }
					if let Some(x) = _ren_token_mint_fee_option { token_info.ren_token_mint_fee = x; }
					if let Some(x) = _ren_token_burn_fee_option { token_info.ren_token_burn_fee = x; }

					Ok(())
				}

			)?;

			Self::deposit_event(RawEvent::RenTokenUpdated(_ren_token_id));
			Ok(())
		}

		/// Deletes the token from the Registry AFTER deleting the asset from Assets
		/// Requires zombies_witness as input to Assets
		#[weight = T::WeightInfo::delete_ren_token(*_ren_token_zombies_witness)]
		pub fn delete_ren_token(
			origin,
			#[compact] _ren_token_id: TokenIdOf<T>,
			_ren_token_zombies_witness: u32
		) -> DispatchResult
		{
			T::ControllerOrigin::ensure_origin(origin)?;

			ensure!(<RenTokenRegistry<T>>::contains_key(&_ren_token_id), Error::<T>::RenTokenNotFound);

			T::Assets::force_destroy_asset(_ren_token_id, _ren_token_zombies_witness)?;

			RenTokenRegistry::<T>::remove(&_ren_token_id);

			Self::deposit_event(RawEvent::RenTokenDeleted(_ren_token_id));
			Ok(())
		}


		/// Transfers tokens from the pallet account to a specified account
		#[weight = T::WeightInfo::spend_tokens()]
		pub fn spend_tokens(
			origin,
			#[compact] _ren_token_id: TokenIdOf<T>,
			who: T::AccountId,
			amount: BalanceOf<T>,
		) -> DispatchResult
		{
			T::ControllerOrigin::ensure_origin(origin)?;

			ensure!(<RenTokenRegistry<T>>::contains_key(&_ren_token_id), Error::<T>::RenTokenNotFound);

			T::Assets::transfer(_ren_token_id, Self::account_id().into(), who.clone(), amount)?;

			Self::deposit_event(RawEvent::RenTokenSpent(_ren_token_id, who, amount));
			Ok(())
		}

		/// Mints the token, after fee deduction, if the signature provided is valid over the provided arguments
		#[weight = T::WeightInfo::validate_and_mint()]
		pub fn mint(
			origin,
			#[compact] _ren_token_id: TokenIdOf<T>,
			who: T::AccountId,
			p_hash: [u8; 32],
			amount: BalanceOf<T>,
			n_hash: [u8; 32],
			sig: EcdsaSignature,
		) -> DispatchResult
		{
			ensure_none(origin)?;

			let ren_token = RenTokenRegistry::<T>::get(&_ren_token_id).ok_or_else(|| Error::<T>::RenTokenNotFound)?;

			ensure!(ren_token.ren_token_mint_enabled, Error::<T>::RenTokenMintDisabled);

			let mint_fee_value = Permill::from_parts(ren_token.ren_token_mint_fee).mul_floor(amount);

			// Skipped checking if the mint call for fee might overflow or cause min_balance error
			// as that should not prevent a user from his operation

			// MINT CALL for user
			T::Assets::mint(_ren_token_id, who.clone(), (amount - mint_fee_value).into())?;
			// Attempt mint call for fees
			let _ = T::Assets::mint(_ren_token_id, Self::account_id().into(), mint_fee_value.into());

			Signatures::insert(&sig, ());
			Self::deposit_event(RawEvent::RenTokenMinted(_ren_token_id, who, amount));
			Ok(())
		}


		/// Burns the token so that the corresponding pegged currency can be released to the "to" address by RenVM
		/// Checks if the token is bitcoin by name, "renBTC" (or "renTestBTC" for testnet) and validates the "to" address accordingly
		/// The actual amount burnt is calculated based on the burn fee, and is the value of the corresponding pegged currency that the user will be granted from RenVM
		/// The burn fee is transfered from the users account to the pallets account
		#[weight = T::WeightInfo::burn()]
		pub fn burn(
			origin,
			#[compact] _ren_token_id: TokenIdOf<T>,
			to: DestAddress,
			amount: BalanceOf<T>,
		) -> DispatchResult
		{
			let sender = ensure_signed(origin)?;

			let ren_token = RenTokenRegistry::<T>::get(&_ren_token_id).ok_or_else(|| Error::<T>::RenTokenNotFound)?;

			ensure!(ren_token.ren_token_burn_enabled, Error::<T>::RenTokenBurnDisabled);

			// match sp_std::str::from_utf8(ren_token.ren_token_name.as_slice()).map_err(|_| Error::<T>::UnexpectedError) {
			// 	Ok("renBTC") 		=> btc_address::validate_btc_address(sp_std::str::from_utf8(to.as_slice()).unwrap_or_else(|_| ""))
			// 							.map_err(|_| Error::<T>::InvalidBurnToAddress)
			// 							.and_then(|x| { if [0,5].contains(&x) {Ok(())} else {Err(Error::<T>::InvalidBurnToAddress)}}),
			// 	Ok("renTestBTC") 	=> btc_address::validate_btc_address(sp_std::str::from_utf8(to.as_slice()).unwrap_or_else(|_| ""))
			// 							.map_err(|_| Error::<T>::InvalidBurnToAddress)
			// 							.and_then(|x| { if [5,111].contains(&x) {Ok(())} else {Err(Error::<T>::InvalidBurnToAddress)}}),
			// 	Err(x)				=> Err(x),
			// 	_					=> Ok(()),
			// }?;


			NextBurnEventId::try_mutate(|id| -> DispatchResult {
				let this_id = *id;
				*id = id.checked_add(1).ok_or(Error::<T>::BurnIdOverflow)?;

				let burn_fee_value = Permill::from_parts(ren_token.ren_token_burn_fee).mul_floor(amount);

				let actual_burn = amount - burn_fee_value;

				// Skipped checking if the mint call for fee might overflow or cause min_balance error
				// as that should not prevent a user from his operation

				// BURN CALL for user
				T::Assets::burn(_ren_token_id, sender.clone(), actual_burn)?;
				// Attempt transfer call for fees
				let _ = T::Assets::transfer(_ren_token_id, sender.clone(), Self::account_id().into(), burn_fee_value);

				BurnEvents::<T>::insert(this_id, (_ren_token_id, frame_system::Module::<T>::block_number(), &to, actual_burn));
				Self::deposit_event(RawEvent::RenTokenBurnt(_ren_token_id, sender, to, actual_burn));

				Ok(())
			})?;
			Ok(())
		}

	}
}


impl<T: Config> Module<T> {

	/// The account ID that holds the pallet's accumulated funds on pallet-assets; mostly fees for now, maybe for loss of exsistential deposit in future.
    pub fn account_id() -> T::AccountId {
        T::ModuleId::get().into_account()
    }

	/// Encode the values for creating the signature hash.
	fn signable_message(p_hash: &[u8; 32], amount: BalanceOf<T>, to: &[u8], n_hash: &[u8; 32], token: &[u8; 32]) -> Vec<u8> {

		let mut amount_slice = Encode::encode(&amount);
		amount_slice.reverse();

		// p_hash ++ amount ++ token ++ to ++ n_hash
		let length = 32 + 32 + 32 + to.len() + 32;
		let mut v = Vec::with_capacity(length);
		v.extend_from_slice(&p_hash[..]);
		v.extend_from_slice(&[0u8; 16][..]);
		v.extend_from_slice(&amount_slice);
		v.extend_from_slice(&token[..]);
		v.extend_from_slice(to);
		v.extend_from_slice(&n_hash[..]);
		v
	}

	/// Verify that the signature has been signed by RenVM using the PublicKey of the token in token info
	fn verify_signature(
		_ren_token_id: TokenIdOf<T>,
		p_hash: &[u8; 32],
		amount: BalanceOf<T>,
		to: &[u8],
		n_hash: &[u8; 32],
		sig: &[u8; 65],
	) -> DispatchResult {

		let ren_token = RenTokenRegistry::<T>::get(&_ren_token_id).ok_or_else(|| Error::<T>::RenTokenNotFound)?;
		let identifier = ren_token.ren_token_renvm_id;

		let signed_message_hash = keccak_256(&Self::signable_message(p_hash, amount, to, n_hash, &identifier));
		let recoverd =
			secp256k1_ecdsa_recover(&sig, &signed_message_hash).map_err(|_| Error::<T>::InvalidMintSignature)?;
		let addr = &keccak_256(&recoverd)[12..];

		let pub_key = ren_token.ren_token_pub_key;

		ensure!(addr == pub_key, Error::<T>::InvalidMintSignature);

		Ok(())
	}

}


#[allow(deprecated)]
impl<T: Config> frame_support::unsigned::ValidateUnsigned for Module<T> {
	type Call = Call<T>;

	/// The entry point for the validation process of unsigned transactions
	fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
		if let Call::mint(_ren_token_id, who, p_hash, amount, n_hash, sig) = call {
			// check if already exists
			if Signatures::contains_key(&sig) {
				return InvalidTransaction::Stale.into();
			}

			let verify_result = Encode::using_encoded(&who, |encoded| -> DispatchResult {
				Self::verify_signature(*_ren_token_id, &p_hash, *amount, encoded, &n_hash, &sig.0)
			});

			// verify signature
			if verify_result.is_err() {
				return InvalidTransaction::BadProof.into();
			}

			ValidTransaction::with_tag_prefix("edge-ren")
				.priority(T::RenvmBridgeUnsignedPriority::get())
				.and_provides(sig)
				.longevity(64_u64)
				.propagate(true)
				.build()
		} else {
			InvalidTransaction::Call.into()
		}
	}
}


/// Simple ensure origin for the RenVM account
pub struct EnsureRenVM<T>(sp_std::marker::PhantomData<T>);
impl<T: Config> EnsureOrigin<T::Origin> for EnsureRenVM<T> {
	type Success = T::AccountId;
	fn try_origin(o: T::Origin) -> Result<Self::Success, T::Origin> {
		let renvm_id = Module::<T>::account_id();
		o.into().and_then(|o| match o {
			frame_system::RawOrigin::Signed(who) if who == renvm_id => Ok(renvm_id),
			r => Err(T::Origin::from(r)),
		})
	}

	// #[cfg(not(test))]
	#[cfg(feature = "runtime-benchmarks")]
	fn successful_origin() -> T::Origin {
		T::Origin::from(frame_system::RawOrigin::Signed(Module::<T>::account_id()))
	}

}
