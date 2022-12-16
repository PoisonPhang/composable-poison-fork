
//! Autogenerated weights for `assets_registry`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-12-16, STEPS: `50`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `c93baf6406af`, CPU: `Intel(R) Xeon(R) CPU @ 2.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("picasso-dev"), DB CACHE: 1024

// Executed Command:
// /nix/store/y1z2mfgy9msqas77hhxszf78hqg6mx5y-composable/bin/composable
// benchmark
// pallet
// --chain=picasso-dev
// --execution=wasm
// --wasm-execution=compiled
// --wasm-instantiation-strategy=legacy-instance-reuse
// --pallet=*
// --extrinsic=*
// --steps=50
// --repeat=10
// --output=code/parachain/runtime/picasso/src/weights

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
		(63_995_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: AssetsRegistry AssetRatio (r:1 w:1)
	// Storage: AssetsRegistry LocalToForeign (r:0 w:1)
	// Storage: AssetsRegistry ForeignToLocal (r:0 w:1)
	fn update_asset() -> Weight {
		(48_869_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: AssetsRegistry MinFeeAmounts (r:1 w:1)
	fn set_min_fee() -> Weight {
		(42_380_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}
