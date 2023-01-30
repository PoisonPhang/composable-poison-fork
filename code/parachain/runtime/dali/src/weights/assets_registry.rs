
//! Autogenerated weights for `assets_registry`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-12-16, STEPS: `50`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `fde3d2d43403`, CPU: `Intel(R) Xeon(R) CPU @ 2.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dali-dev"), DB CACHE: 1024

// Executed Command:
// /nix/store/y1z2mfgy9msqas77hhxszf78hqg6mx5y-composable/bin/composable
// benchmark
// pallet
// --chain=dali-dev
// --execution=wasm
// --wasm-execution=compiled
// --wasm-instantiation-strategy=legacy-instance-reuse
// --pallet=*
// --extrinsic=*
// --steps=50
// --repeat=10
// --output=code/parachain/runtime/dali/src/weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `assets_registry`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> assets_registry::WeightInfo for WeightInfo<T> {
	// Storage: AssetsRegistry ForeignToLocal (r:1 w:1)
	// Storage: CurrencyFactory AssetIdRanges (r:1 w:1)
	// Storage: AssetsRegistry AssetRatio (r:1 w:1)
	// Storage: AssetsRegistry LocalToForeign (r:0 w:1)
	fn register_asset() -> Weight {
		Weight::from_ref_time(63_771_000_u64)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	// Storage: AssetsRegistry MinFeeAmounts (r:1 w:1)
	fn set_min_fee() -> Weight {
		Weight::from_ref_time(43_550_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}

	fn update_asset_location() -> Weight {
		Weight::from_ref_time(49_298_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
    }

	fn update_asset() -> Weight {
		Weight::from_ref_time(49_298_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
    }
}
