#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(trivial_numeric_casts)]
#![allow(clippy::unnecessary_cast)]
use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

const WEIGHT: i32 = 1_000;

pub trait WeightInfo {
	fn create_asset_vault() -> Weight;
	fn create_option() -> Weight;
	fn sell_option() -> Weight;
	fn delete_sell_option() -> Weight;
	fn buy_option() -> Weight;
	fn exercise_option() -> Weight;
	fn withdraw_collateral() -> Weight;
}

/// Weights for pallet_tokenized_options using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: TokenizedOptions AssetToVault (r:1 w:1)
	// Storage: Vault VaultCount (r:1 w:1)
	// Storage: Factory CurrencyCounter (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Vault LpTokensToVaults (r:0 w:1)
	// Storage: Vault Allocations (r:0 w:1)
	// Storage: Vault Vaults (r:0 w:1)
	fn create_asset_vault() -> Weight {
		(144_989_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
	}

	// Storage: TokenizedOptions OptionIdToOption (r:0 w:1)
	// Storage: TokenizedOptions OptionHashToOptionId (r:0 w:1)
	// Storage: Factory CurrencyCounter (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Vault LpTokensToVaults (r:0 w:1)
	// Storage: Vault Allocations (r:0 w:1)
	// Storage: Vault Vaults (r:0 w:1)
	fn create_option() -> Weight {
		(144_989_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
	}

	fn sell_option() -> Weight {
		(144_989_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
	}

	fn delete_sell_option() -> Weight {
		(144_989_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
	}

	fn buy_option() -> Weight {
		(144_989_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
	}

	fn exercise_option() -> Weight {
		(144_989_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
	}

	fn withdraw_collateral() -> Weight {
		(144_989_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn create_asset_vault() -> Weight {
		WEIGHT as Weight
	}

	fn create_option() -> Weight {
		WEIGHT as Weight
	}

	fn sell_option() -> Weight {
		WEIGHT as Weight
	}

	fn delete_sell_option() -> Weight {
		WEIGHT as Weight
	}

	fn buy_option() -> Weight {
		WEIGHT as Weight
	}

	fn exercise_option() -> Weight {
		WEIGHT as Weight
	}

	fn withdraw_collateral() -> Weight {
		WEIGHT as Weight
	}
}