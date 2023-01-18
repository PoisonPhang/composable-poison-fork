//! # Assets Pallet
//!
//! The Assets pallet provides implementation of common currency traits
//! (e.g. from [`orml`](https://docs.rs/orml-traits) or `frame_support`)
//! and functionality for handling transfers and minting.
//!
//! - [`Config`]
//! - [`Call`]
//! - [`Pallet`]
//!
//! ## Overview
//!
//! The Assets pallet provides functions for:
//!
//! - Transferring balances of native and other assets between accounts.
//! - Minting and burn new assets by per asset governance.
//! - Crediting and debiting of created asset balances.
//! - By design similar to [orml_currencies](https://docs.rs/orml-currencies/latest/orml_currencies/)
//!   and [substrate_assets](https://github.com/paritytech/substrate/tree/master/frame/assets)
//! Functions requiring authorization are checked via asset's governance registry origin. Example,
//! minting.
//!
//! ### Implementations
//!
//! The Assets pallet provides implementations for the following traits:
//!
//! - [`Currency`](frame_support::traits::Currency):
//!  Functions for dealing with a fungible assets system.
//! - [`ReservableCurrency`](frame_support::traits::ReservableCurrency):
//!  Functions for dealing with assets that can be reserved from an account.
//! - [`MultiCurrency`](orml_traits::MultiCurrency):
//!  Abstraction over a fungible multi-currency system.
//! - [`MultiLockableCurrency`](orml_traits::MultiLockableCurrency):
//!  A fungible multi-currency system whose accounts can have liquidity restrictions.
//! - [`MultiReservableCurrency`](orml_traits::MultiReservableCurrency):
//!  A fungible multi-currency system where funds can be reserved from the user.
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! - `transfer`
//! - `transfer_native`
//! - `force_transfer`
//! - `force_transfer_native`
//! - `transfer_all`
//! - `transfer_all_native`
//! - `mint_initialize`
//! - `mint_initialize_with_governance`
//! - `mint_into`
//! - `burn_from`
//
// we start lag behind useful traits:
// TODO: implement fungibles::Balanced like orml Tokens do
// TODO: implement tokens::NamedReservableCurrency like orml Tokens do

#![cfg_attr(
	not(test),
	warn(
		clippy::disallowed_methods,
		clippy::disallowed_types,
		clippy::indexing_slicing,
		clippy::todo,
		clippy::unwrap_used,
		clippy::panic
	)
)] // allow in tests
#![warn(clippy::unseparated_literal_suffix)]
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

mod orml;

#[cfg(test)]
mod mocks;

#[cfg(test)]
mod tests;

// #[cfg(any(feature = "runtime-benchmarks", test))]
// mod benchmarking;
pub mod weights;

macro_rules! route {
	(
		fn $fn:ident($asset:ident: $asset_ty:ty $(, $arg:ident: $ty:ty)* $(,)?) $(-> $ret:ty)?;
	) => {
		fn $fn($asset: $asset_ty, $($arg:$ty),*) $(-> $ret)? {
			if T::AssetId::from($asset.into()) == <T::NativeAssetId as frame_support::traits::Get<_>>::get() {
				<<T as Config>::NativeCurrency>::$fn($($arg),*)
			} else {
				crate::route_asset_type! { $fn($asset, $($arg),*) }
			}
		}
	};
}

macro_rules! route_asset_type {
	(
		$fn:ident($asset:ident $(, $arg:ident)* $(,)?)
	) => {
		match <T::AssetLookup as composable_traits::assets::AssetTypeInspect>::inspect(&$asset) {
			composable_traits::assets::AssetType::Foreign => {
				<<T as Config>::ForeignTransactor>::$fn($asset, $($arg),*)
			}
			composable_traits::assets::AssetType::Local => {
				<<T as Config>::LocalTransactor>::$fn($asset, $($arg),*)
			}
		}
	};
}

pub(crate) use route;
pub(crate) use route_asset_type;

#[frame_support::pallet]
pub mod pallet {
	use crate::weights::WeightInfo;
	use codec::FullCodec;
	use composable_traits::{
		assets::{AssetTypeInspect, CreateAsset, InspectRegistryMetadata, MutateRegistryMetadata},
		currency::{AssetIdLike, BalanceLike, Rational64},
		governance::{GovernanceRegistry, SignedRawOrigin},
		xcm::assets::RemoteAssetRegistryMutate,
	};
	use frame_support::{
		dispatch::DispatchResult,
		pallet_prelude::*,
		sp_runtime::traits::StaticLookup,
		traits::{
			fungible, fungibles, Currency, EnsureOrigin, LockableCurrency, ReservableCurrency,
		},
	};
	use frame_system::{ensure_root, ensure_signed, pallet_prelude::OriginFor};
	use orml_traits::{GetByKey, MultiCurrency, MultiLockableCurrency, MultiReservableCurrency};
	use sp_runtime::DispatchError;
	use sp_std::{fmt::Debug, str, vec::Vec};

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// currency id
		type AssetId: AssetIdLike + From<u128> + Into<u128> + MaybeSerializeDeserialize;
		type AssetLocation: FullCodec
			+ Eq
			+ PartialEq
			+ MaybeSerializeDeserialize
			+ Debug
			+ Clone
			+ Default
			+ TypeInfo
			+ MaxEncodedLen;
		type Balance: BalanceLike;

		#[pallet::constant]
		type NativeAssetId: Get<Self::AssetId>;

		type ForeignTransactor: fungibles::Inspect<Self::AccountId, Balance = Self::Balance, AssetId = Self::AssetId>
			+ fungibles::Transfer<Self::AccountId>
			+ fungibles::Mutate<Self::AccountId>
			+ fungibles::Unbalanced<Self::AccountId>
			+ fungibles::InspectHold<Self::AccountId>
			+ fungibles::MutateHold<Self::AccountId>
			+ MultiCurrency<Self::AccountId, Balance = Self::Balance, CurrencyId = Self::AssetId>
			+ MultiLockableCurrency<
				Self::AccountId,
				Balance = Self::Balance,
				CurrencyId = Self::AssetId,
			> + MultiReservableCurrency<
				Self::AccountId,
				Balance = Self::Balance,
				CurrencyId = Self::AssetId,
			>;

		type LocalTransactor: fungibles::Inspect<Self::AccountId, Balance = Self::Balance, AssetId = Self::AssetId>
			+ fungibles::Transfer<Self::AccountId>
			+ fungibles::Mutate<Self::AccountId>
			+ fungibles::Unbalanced<Self::AccountId>
			+ fungibles::InspectHold<Self::AccountId>
			+ fungibles::MutateHold<Self::AccountId>
			+ MultiCurrency<Self::AccountId, Balance = Self::Balance, CurrencyId = Self::AssetId>
			+ MultiLockableCurrency<
				Self::AccountId,
				Balance = Self::Balance,
				CurrencyId = Self::AssetId,
			> + MultiReservableCurrency<
				Self::AccountId,
				Balance = Self::Balance,
				CurrencyId = Self::AssetId,
			>;

		// TODO(benluelo): Move trait bounds here, rename to NativeTransactor
		type NativeCurrency: fungible::Inspect<Self::AccountId, Balance = Self::Balance>
			+ fungible::Transfer<Self::AccountId>
			+ fungible::Mutate<Self::AccountId>
			+ fungible::Unbalanced<Self::AccountId>
			+ fungible::InspectHold<Self::AccountId>
			+ fungible::Transfer<Self::AccountId>
			+ fungible::MutateHold<Self::AccountId>
			+ Currency<Self::AccountId, Balance = Self::Balance>
			+ LockableCurrency<Self::AccountId, Balance = Self::Balance>
			+ ReservableCurrency<Self::AccountId, Balance = Self::Balance>;

		type GovernanceRegistry: GetByKey<Self::AssetId, Result<SignedRawOrigin<Self::AccountId>, DispatchError>>
			+ GovernanceRegistry<Self::AssetId, Self::AccountId>;

		/// origin of admin of this pallet
		type AdminOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		// will be assets-registry
		type AssetLookup: AssetTypeInspect<AssetId = Self::AssetId>
			+ RemoteAssetRegistryMutate<
				AssetId = Self::AssetId,
				AssetNativeLocation = Self::AssetLocation,
			> + InspectRegistryMetadata<AssetId = Self::AssetId>
			+ MutateRegistryMetadata<AssetId = Self::AssetId>;

		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::error]
	pub enum Error<T> {
		CannotSetNewCurrencyToRegistry,
		InvalidCurrency,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Transfer `amount` of `asset` from `origin` to `dest`.
		///
		/// # Errors
		///  - When `origin` is not signed.
		///  - If the account has insufficient free balance to make the transfer, or if `keep_alive`
		///    cannot be respected.
		///  - If the `dest` cannot be looked up.
		#[pallet::weight(T::WeightInfo::transfer())]
		pub fn transfer(
			origin: OriginFor<T>,
			asset: T::AssetId,
			dest: <T::Lookup as StaticLookup>::Source,
			#[pallet::compact] amount: T::Balance,
			keep_alive: bool,
		) -> DispatchResult {
			let source = ensure_signed(origin)?;
			let dest = T::Lookup::lookup(dest)?;

			<Pallet<T> as fungibles::Transfer<_>>::transfer(
				asset, &source, &dest, amount, keep_alive,
			)?;
			Ok(())
		}

		/// Transfer `amount` of the native asset from `origin` to `dest`. This is slightly
		/// cheaper to call, as it avoids an asset lookup.
		///
		/// # Errors
		///  - When `origin` is not signed.
		///  - If the account has insufficient free balance to make the transfer, or if `keep_alive`
		///    cannot be respected.
		///  - If the `dest` cannot be looked up.
		#[pallet::weight(T::WeightInfo::transfer_native())]
		pub fn transfer_native(
			origin: OriginFor<T>,
			dest: <T::Lookup as StaticLookup>::Source,
			#[pallet::compact] value: T::Balance,
			keep_alive: bool,
		) -> DispatchResult {
			let source = ensure_signed(origin)?;
			let dest = T::Lookup::lookup(dest)?;
			<Self as fungible::Transfer<_>>::transfer(&source, &dest, value, keep_alive)?;
			Ok(())
		}

		/// Transfer `amount` of the `asset` from `origin` to `dest`. This requires root.
		///
		/// # Errors
		///  - When `origin` is not root.
		///  - If the account has insufficient free balance to make the transfer, or if `keep_alive`
		///    cannot be respected.
		///  - If the `dest` cannot be looked up.
		#[pallet::weight(T::WeightInfo::force_transfer())]
		pub fn force_transfer(
			origin: OriginFor<T>,
			asset: T::AssetId,
			source: <T::Lookup as StaticLookup>::Source,
			dest: <T::Lookup as StaticLookup>::Source,
			#[pallet::compact] value: T::Balance,
			keep_alive: bool,
		) -> DispatchResult {
			ensure_root(origin)?;
			let source = T::Lookup::lookup(source)?;
			let dest = T::Lookup::lookup(dest)?;
			<Self as fungibles::Transfer<_>>::transfer(asset, &source, &dest, value, keep_alive)?;
			Ok(())
		}

		/// Transfer `amount` of the the native asset from `origin` to `dest`. This requires root.
		///
		/// # Errors
		///  - When `origin` is not root.
		///  - If the account has insufficient free balance to make the transfer, or if `keep_alive`
		///    cannot be respected.
		///  - If the `dest` cannot be looked up.
		#[pallet::weight(T::WeightInfo::force_transfer_native())]
		pub fn force_transfer_native(
			origin: OriginFor<T>,
			source: <T::Lookup as StaticLookup>::Source,
			dest: <T::Lookup as StaticLookup>::Source,
			#[pallet::compact] value: T::Balance,
			keep_alive: bool,
		) -> DispatchResult {
			ensure_root(origin)?;
			let source = T::Lookup::lookup(source)?;
			let dest = T::Lookup::lookup(dest)?;
			<Self as fungible::Transfer<_>>::transfer(&source, &dest, value, keep_alive)?;
			Ok(())
		}

		/// Transfer all free balance of the `asset` from `origin` to `dest`.
		///
		/// # Errors
		///  - When `origin` is not signed.
		///  - If the `dest` cannot be looked up.
		#[pallet::weight(T::WeightInfo::transfer_all())]
		pub fn transfer_all(
			origin: OriginFor<T>,
			asset: T::AssetId,
			dest: <T::Lookup as StaticLookup>::Source,
			keep_alive: bool,
		) -> DispatchResult {
			let transactor = ensure_signed(origin)?;
			let reducible_balance = <Self as fungibles::Inspect<T::AccountId>>::reducible_balance(
				asset,
				&transactor,
				keep_alive,
			);
			let dest = T::Lookup::lookup(dest)?;
			<Self as fungibles::Transfer<T::AccountId>>::transfer(
				asset,
				&transactor,
				&dest,
				reducible_balance,
				keep_alive,
			)?;
			Ok(())
		}

		/// Transfer all free balance of the native asset from `origin` to `dest`.
		///
		/// # Errors
		///  - When `origin` is not signed.
		///  - If the `dest` cannot be looked up.
		#[pallet::weight(T::WeightInfo::transfer_all_native())]
		pub fn transfer_all_native(
			origin: OriginFor<T>,
			dest: <T::Lookup as StaticLookup>::Source,
			keep_alive: bool,
		) -> DispatchResult {
			let transactor = ensure_signed(origin)?;
			let reducible_balance =
				<Self as fungible::Inspect<_>>::reducible_balance(&transactor, keep_alive);
			let dest = T::Lookup::lookup(dest)?;
			<Self as fungible::Transfer<_>>::transfer(
				&transactor,
				&dest,
				reducible_balance,
				keep_alive,
			)?;
			Ok(())
		}

		/// Creates a new asset, minting `amount` of funds into the `dest` account. Intended to be
		/// used for creating wrapped assets, not associated with any project.
		#[pallet::weight(T::WeightInfo::mint_initialize())]
		pub fn mint_initialize(
			origin: OriginFor<T>,
			protocol_id: [u8; 8],
			nonce: u64,
			name: Vec<u8>,
			symbol: Vec<u8>,
			decimals: u8,
			ratio: Option<Rational64>,
			#[pallet::compact] amount: T::Balance,
			dest: <T::Lookup as StaticLookup>::Source,
		) -> DispatchResult {
			ensure_root(origin)?;
			let id = <Self as CreateAsset>::create_local_asset(
				protocol_id.try_into().expect(""),
				nonce,
				name,
				symbol,
				decimals,
				ratio,
			)?;
			let dest = T::Lookup::lookup(dest)?;
			<Self as fungibles::Mutate<T::AccountId>>::mint_into(id, &dest, amount)?;
			Ok(())
		}

		/// Creates a new asset, minting `amount` of funds into the `dest` account. The `dest`
		/// account can use the democracy pallet to mint further assets, or if the governance_origin
		/// is set to an owned account, using signed transactions. In general the
		/// `governance_origin` should be generated from the pallet id.
		#[pallet::weight(T::WeightInfo::mint_initialize())]
		pub fn mint_initialize_with_governance(
			origin: OriginFor<T>,
			protocol_id: [u8; 8],
			nonce: u64,
			name: Vec<u8>,
			symbol: Vec<u8>,
			decimals: u8,
			ratio: Option<Rational64>,
			#[pallet::compact] amount: T::Balance,
			governance_origin: <T::Lookup as StaticLookup>::Source,
			dest: <T::Lookup as StaticLookup>::Source,
		) -> DispatchResult {
			ensure_root(origin)?;
			let id = <Self as CreateAsset>::create_local_asset(
				protocol_id,
				nonce,
				name,
				symbol,
				decimals,
				ratio,
			)?;
			let governance_origin = T::Lookup::lookup(governance_origin)?;
			T::GovernanceRegistry::set(id, SignedRawOrigin::Signed(governance_origin));
			let dest = T::Lookup::lookup(dest)?;
			<Self as fungibles::Mutate<T::AccountId>>::mint_into(id, &dest, amount)?;
			Ok(())
		}

		/// Mints `amount` of `asset_id` into the `dest` account.
		#[pallet::weight(T::WeightInfo::mint_into())]
		pub fn mint_into(
			origin: OriginFor<T>,
			asset_id: T::AssetId,
			dest: <T::Lookup as StaticLookup>::Source,
			#[pallet::compact] amount: T::Balance,
		) -> DispatchResult {
			Pallet::<T>::ensure_admin_or_governance(origin, &asset_id)?;
			let dest = T::Lookup::lookup(dest)?;
			<Self as fungibles::Mutate<T::AccountId>>::mint_into(asset_id, &dest, amount)?;
			Ok(())
		}

		/// Burns `amount` of `asset_id` into the `dest` account.
		#[pallet::weight(T::WeightInfo::burn_from())]
		pub fn burn_from(
			origin: OriginFor<T>,
			asset_id: T::AssetId,
			dest: <T::Lookup as StaticLookup>::Source,
			#[pallet::compact] amount: T::Balance,
		) -> DispatchResult {
			Pallet::<T>::ensure_admin_or_governance(origin, &asset_id)?;
			let dest = T::Lookup::lookup(dest)?;
			<Self as fungibles::Mutate<T::AccountId>>::burn_from(asset_id, &dest, amount)?;
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		/// Returns `Ok(())` if origin is root or asset is signed by root or by origin
		pub(crate) fn ensure_admin_or_governance(
			origin: OriginFor<T>,
			asset_id: &T::AssetId,
		) -> Result<(), DispatchError> {
			// TODO: that must be ensure_asset_origin(origin, asset_id))
			if T::AdminOrigin::ensure_origin(origin.clone()).is_ok() {
				return Ok(())
			}

			match origin.into() {
				Ok(frame_system::RawOrigin::Signed(account)) => {
					match T::GovernanceRegistry::get(asset_id) {
						Ok(SignedRawOrigin::Root) => Ok(()), /* ISSUE: it says if */
						// (call_origin.is_signed &&
						// asst_owner.is_root) then allow
						// mint/burn -> anybody can mint and
						// burn PICA?
						// TODO: https://app.clickup.com/t/37h4edu
						Ok(SignedRawOrigin::Signed(acc)) if acc == account => Ok(()),
						_ => Err(DispatchError::BadOrigin),
					}
				},
				Ok(frame_system::RawOrigin::Root) => Ok(()),
				_ => Err(DispatchError::BadOrigin), /* ISSUE: likely will not support collective
													* origin which is reasonable to have for
													* governance
													https://app.clickup.com/t/37h4edu
													*/
			}
		}
	}

	impl<T: Config> CreateAsset for Pallet<T> {
		type LocalAssetId = T::AssetId;
		type ForeignAssetId = T::AssetLocation;

		fn create_local_asset(
			protocol_id: [u8; 8],
			nonce: u64,
			name: Vec<u8>,
			symbol: Vec<u8>,
			decimals: u8,
			ratio: Option<Rational64>,
		) -> Result<Self::LocalAssetId, DispatchError> {
			let bytes = protocol_id
				.into_iter()
				.chain(nonce.to_le_bytes())
				.collect::<Vec<u8>>()
				.try_into()
				.expect("[u8; 8] + bytes(u64) = [u8; 16]");
			let asset_id = Self::LocalAssetId::from(u128::from_le_bytes(bytes));

			T::AssetLookup::register_asset(asset_id, None, ratio, name, symbol, decimals)?;

			Ok(asset_id)
		}

		fn create_foreign_asset(
			foreign_asset_id: Self::ForeignAssetId,
			name: Vec<u8>,
			symbol: Vec<u8>,
			decimals: u8,
			ratio: Option<Rational64>,
		) -> Result<Self::LocalAssetId, DispatchError> {
			let asset_id = Self::LocalAssetId::from(u128::from_be_bytes(sp_core::blake2_128(
				&foreign_asset_id.encode(),
			)));

			T::AssetLookup::register_asset(
				asset_id,
				Some(foreign_asset_id),
				ratio,
				name,
				symbol,
				decimals,
			)?;

			Ok(asset_id)
		}
	}
}

/// Implementations of the various `fungible::*` traits for the pallet.
///
/// All of these implementations route to the NativeTransactor.
mod fungible_impls {
	use frame_support::{
		pallet_prelude::*,
		traits::tokens::{
			fungible::{Inspect, InspectHold, Mutate, MutateHold, Transfer, Unbalanced},
			DepositConsequence, WithdrawConsequence,
		},
	};

	use crate::{Config, Pallet};

	impl<T: Config> MutateHold<T::AccountId> for Pallet<T> {
		fn hold(who: &T::AccountId, amount: Self::Balance) -> DispatchResult {
			<<T as Config>::NativeCurrency>::hold(who, amount)
		}

		fn release(
			who: &T::AccountId,
			amount: Self::Balance,
			best_effort: bool,
		) -> Result<Self::Balance, DispatchError> {
			<<T as Config>::NativeCurrency>::release(who, amount, best_effort)
		}

		fn transfer_held(
			source: &T::AccountId,
			dest: &T::AccountId,
			amount: Self::Balance,
			best_effort: bool,
			on_held: bool,
		) -> Result<Self::Balance, DispatchError> {
			<<T as Config>::NativeCurrency>::transfer_held(
				source,
				dest,
				amount,
				best_effort,
				on_held,
			)
		}
	}

	impl<T: Config> Mutate<T::AccountId> for Pallet<T> {
		fn mint_into(who: &T::AccountId, amount: Self::Balance) -> DispatchResult {
			<<T as Config>::NativeCurrency>::mint_into(who, amount)
		}
		fn burn_from(
			who: &T::AccountId,
			amount: Self::Balance,
		) -> Result<Self::Balance, DispatchError> {
			<<T as Config>::NativeCurrency>::burn_from(who, amount)
		}

		fn slash(
			who: &T::AccountId,
			amount: Self::Balance,
		) -> Result<Self::Balance, DispatchError> {
			<<T as Config>::NativeCurrency>::slash(who, amount)
		}
		fn teleport(
			source: &T::AccountId,
			dest: &T::AccountId,
			amount: Self::Balance,
		) -> Result<Self::Balance, DispatchError> {
			<<T as Config>::NativeCurrency>::teleport(source, dest, amount)
		}
	}

	impl<T: Config> Unbalanced<T::AccountId> for Pallet<T> {
		fn set_balance(who: &T::AccountId, amount: Self::Balance) -> DispatchResult {
			<<T as Config>::NativeCurrency>::set_balance(who, amount)
		}

		fn set_total_issuance(amount: Self::Balance) {
			<<T as Config>::NativeCurrency>::set_total_issuance(amount)
		}

		fn decrease_balance(
			who: &T::AccountId,
			amount: Self::Balance,
		) -> Result<Self::Balance, DispatchError> {
			<<T as Config>::NativeCurrency>::decrease_balance(who, amount)
		}

		fn decrease_balance_at_most(who: &T::AccountId, amount: Self::Balance) -> Self::Balance {
			<<T as Config>::NativeCurrency>::decrease_balance_at_most(who, amount)
		}

		fn increase_balance(
			who: &T::AccountId,
			amount: Self::Balance,
		) -> Result<Self::Balance, DispatchError> {
			<<T as Config>::NativeCurrency>::increase_balance(who, amount)
		}

		fn increase_balance_at_most(who: &T::AccountId, amount: Self::Balance) -> Self::Balance {
			<<T as Config>::NativeCurrency>::increase_balance_at_most(who, amount)
		}
	}

	impl<T: Config> Transfer<T::AccountId> for Pallet<T> {
		fn transfer(
			source: &T::AccountId,
			dest: &T::AccountId,
			amount: Self::Balance,
			keep_alive: bool,
		) -> Result<Self::Balance, DispatchError> {
			<<T as Config>::NativeCurrency>::transfer(source, dest, amount, keep_alive)
		}
	}

	impl<T: Config> Inspect<T::AccountId> for Pallet<T> {
		type Balance = T::Balance;

		fn total_issuance() -> Self::Balance {
			<<T as Config>::NativeCurrency>::total_issuance()
		}

		fn minimum_balance() -> Self::Balance {
			<<T as Config>::NativeCurrency>::minimum_balance()
		}

		fn balance(who: &T::AccountId) -> Self::Balance {
			<<T as Config>::NativeCurrency>::balance(who)
		}

		fn reducible_balance(who: &T::AccountId, keep_alive: bool) -> Self::Balance {
			<<T as Config>::NativeCurrency>::reducible_balance(who, keep_alive)
		}

		fn can_deposit(
			who: &T::AccountId,
			amount: Self::Balance,
			mint: bool,
		) -> DepositConsequence {
			<<T as Config>::NativeCurrency>::can_deposit(who, amount, mint)
		}

		fn can_withdraw(
			who: &T::AccountId,
			amount: Self::Balance,
		) -> WithdrawConsequence<Self::Balance> {
			<<T as Config>::NativeCurrency>::can_withdraw(who, amount)
		}
	}

	impl<T: Config> InspectHold<T::AccountId> for Pallet<T>
	where
		<T as Config>::NativeCurrency:
			Inspect<T::AccountId, Balance = T::Balance> + InspectHold<T::AccountId>,
	{
		fn balance_on_hold(who: &T::AccountId) -> Self::Balance {
			<<T as Config>::NativeCurrency>::balance_on_hold(who)
		}

		fn can_hold(who: &T::AccountId, amount: Self::Balance) -> bool {
			<<T as Config>::NativeCurrency>::can_hold(who, amount)
		}
	}
}

mod fungibles_impls {
	use composable_traits::assets::{
		AssetType, AssetTypeInspect, InspectRegistryMetadata, MutateRegistryMetadata,
	};
	use frame_support::{
		pallet_prelude::*,
		traits::tokens::{
			fungible::{
				Inspect as NativeInspect, InspectHold as NativeInspectHold, Mutate as NativeMutate,
				MutateHold as NativeMutateHold, Transfer as NativeTransfer,
				Unbalanced as NativeUnbalanced,
			},
			fungibles::{self, Inspect, InspectHold, Mutate, MutateHold, Transfer, Unbalanced},
			DepositConsequence, WithdrawConsequence,
		},
	};

	use crate::{Config, Pallet};

	impl<T: Config> fungibles::metadata::Inspect<T::AccountId> for Pallet<T> {
		fn name(asset: Self::AssetId) -> Vec<u8> {
			<T::AssetLookup as InspectRegistryMetadata>::asset_name(&asset).unwrap_or_default()
		}

		fn symbol(asset: Self::AssetId) -> Vec<u8> {
			<T::AssetLookup as InspectRegistryMetadata>::symbol(&asset).unwrap_or_default()
		}

		fn decimals(asset: Self::AssetId) -> u8 {
			<T::AssetLookup as InspectRegistryMetadata>::decimals(&asset).unwrap_or_default()
		}
	}

	impl<T: Config> fungibles::metadata::Mutate<T::AccountId> for Pallet<T> {
		fn set(
			asset: Self::AssetId,
			_from: &T::AccountId,
			name: Vec<u8>,
			symbol: Vec<u8>,
			decimals: u8,
		) -> DispatchResult {
			<T::AssetLookup as MutateRegistryMetadata>::set_metadata(&asset, name, symbol, decimals)
		}
	}

	impl<T: Config> fungibles::InspectMetadata<T::AccountId> for Pallet<T> {
		fn name(asset: &Self::AssetId) -> Vec<u8> {
			<Self as fungibles::metadata::Inspect<T::AccountId>>::name(*asset)
		}

		fn symbol(asset: &Self::AssetId) -> Vec<u8> {
			<Self as fungibles::metadata::Inspect<T::AccountId>>::symbol(*asset)
		}

		fn decimals(asset: &Self::AssetId) -> u8 {
			<Self as fungibles::metadata::Inspect<T::AccountId>>::decimals(*asset)
		}
	}

	impl<T: Config> Unbalanced<T::AccountId> for Pallet<T> {
		route! {
			fn set_balance(
				asset: Self::AssetId,
				who: &T::AccountId,
				amount: Self::Balance
			) -> DispatchResult;
		}

		route! {
			fn set_total_issuance(asset: Self::AssetId, amount: Self::Balance);
		}

		route! {
			fn decrease_balance(
				asset: Self::AssetId,
				who: &T::AccountId,
				amount: Self::Balance
			) -> Result<Self::Balance, DispatchError>;
		}

		route! {
			fn decrease_balance_at_most(
				asset: Self::AssetId,
				who: &T::AccountId,
				amount: Self::Balance
			) -> Self::Balance;
		}

		route! {
			fn increase_balance(
				asset: Self::AssetId,
				who: &T::AccountId,
				amount: Self::Balance
			) -> Result<Self::Balance, DispatchError>;
		}

		route! {
			fn increase_balance_at_most(
				asset: Self::AssetId,
				who: &T::AccountId,
				amount: Self::Balance
			) -> Self::Balance;
		}
	}

	impl<T: Config> Transfer<T::AccountId> for Pallet<T> {
		route! {
			fn transfer(
				asset: Self::AssetId,
				source: &T::AccountId,
				dest: &T::AccountId,
				amount: Self::Balance,
				keep_alive: bool
			) -> Result<Self::Balance, DispatchError>;
		}
	}

	impl<T: Config> MutateHold<T::AccountId> for Pallet<T> {
		route! {
			fn hold(asset: Self::AssetId, who: &T::AccountId, amount: Self::Balance) -> DispatchResult;
		}

		route! {
			fn release(
				asset: Self::AssetId,
				who: &T::AccountId,
				amount: Self::Balance,
				best_effort: bool
			) -> Result<Self::Balance, DispatchError>;
		}

		route! {
			fn transfer_held(
				asset: Self::AssetId,
				source: &T::AccountId,
				dest: &T::AccountId,
				amount: Self::Balance,
				best_effort: bool,
				on_hold: bool
			) -> Result<Self::Balance, DispatchError>;
		}
	}

	impl<T: Config> Mutate<T::AccountId> for Pallet<T> {
		route! {
			fn mint_into(
				asset: Self::AssetId,
				who: &T::AccountId,
				amount: Self::Balance
			) -> DispatchResult;
		}

		route! {
			fn burn_from(
				asset: Self::AssetId,
				who: &T::AccountId,
				amount: Self::Balance
			) -> Result<Self::Balance, DispatchError>;
		}

		route! {
			fn slash(
				asset: Self::AssetId,
				who: &T::AccountId,
				amount: Self::Balance
			) -> Result<Self::Balance, DispatchError>;
		}

		route! {
			fn teleport(
				asset: Self::AssetId,
				source: &T::AccountId,
				dest: &T::AccountId,
				amount: Self::Balance
			) -> Result<Self::Balance, DispatchError>;
		}
	}

	impl<T: Config> Inspect<T::AccountId> for Pallet<T> {
		type AssetId = T::AssetId;
		type Balance = T::Balance;

		route! {
			fn total_issuance(asset: Self::AssetId) -> Self::Balance;
		}

		route! {
			fn minimum_balance(asset: Self::AssetId) -> Self::Balance;
		}

		route! {
			fn balance(asset: Self::AssetId, who: &T::AccountId) -> Self::Balance;
		}

		route! {
			fn reducible_balance(
				asset: Self::AssetId,
				who: &T::AccountId,
				keep_alive: bool
			) -> Self::Balance;
		}

		route! {
			fn can_deposit(
				asset: Self::AssetId,
				who: &T::AccountId,
				amount: Self::Balance,
				mint: bool
			) -> DepositConsequence;
		}

		route! {
			fn can_withdraw(
				asset: Self::AssetId,
				who: &T::AccountId,
				amount: Self::Balance
			) -> WithdrawConsequence<Self::Balance>;
		}
	}

	impl<T: Config> InspectHold<T::AccountId> for Pallet<T> {
		route! {
			fn balance_on_hold(asset: Self::AssetId, who: &T::AccountId) -> Self::Balance;
		}

		route! {
			fn can_hold(asset: Self::AssetId, who: &T::AccountId, amount: Self::Balance) -> bool;
		}
	}
}
