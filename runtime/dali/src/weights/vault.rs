
//! Autogenerated weights for `vault`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-05-21, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dali-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/composable
// benchmark
// pallet
// --chain=dali-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=*
// --extrinsic=*
// --steps=50
// --repeat=20
// --output=runtime/dali/src/weights
// --log
// error

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `vault`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> vault::WeightInfo for WeightInfo<T> {
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Vault VaultCount (r:1 w:1)
	// Storage: CurrencyFactory AssetIdRanges (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: CurrencyFactory AssetEd (r:0 w:1)
	// Storage: Vault LpTokensToVaults (r:0 w:1)
	// Storage: Vault Vaults (r:0 w:1)
	fn create() -> Weight {
		(154_541_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
	}
	// Storage: Vault Vaults (r:1 w:0)
	// Storage: Tokens Accounts (r:3 w:3)
	// Storage: Tokens TotalIssuance (r:2 w:1)
	// Storage: Vault CapitalStructure (r:2 w:0)
	// Storage: System Account (r:1 w:1)
	fn deposit() -> Weight {
		(149_758_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: Vault Vaults (r:1 w:0)
	// Storage: Tokens Accounts (r:3 w:3)
	// Storage: Vault CapitalStructure (r:2 w:0)
	// Storage: Tokens TotalIssuance (r:2 w:1)
	fn withdraw() -> Weight {
		(124_070_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Vault Vaults (r:1 w:1)
	fn emergency_shutdown() -> Weight {
		(30_259_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Vault Vaults (r:1 w:1)
	fn start_() -> Weight {
		(29_720_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Vault Vaults (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn add_surcharge() -> Weight {
		(79_734_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Vault Vaults (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn claim_surcharge() -> Weight {
		(75_491_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Vault Vaults (r:1 w:1)
	// Storage: System Account (r:1 w:0)
	// Storage: Vault LpTokensToVaults (r:0 w:1)
	fn delete_tombstoned() -> Weight {
		(26_717_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}
