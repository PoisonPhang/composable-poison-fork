
//! Autogenerated weights for `vesting`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-08-10, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("picasso-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/composable
// benchmark
// pallet
// --chain=picasso-dev
// --execution=wasm
// --wasm-execution=compiled
// --wasm-instantiation-strategy=legacy-instance-reuse
// --pallet=*
// --extrinsic=*
// --steps=50
// --repeat=20
// --output=runtime/picasso/src/weights
// --log
// error

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `vesting`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> vesting::WeightInfo for WeightInfo<T> {
	// Storage: Vesting VestingSchedules (r:1 w:1)
	// Storage: Tokens Locks (r:1 w:1)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	fn claim(s: u32, ) -> Weight {
		(77_307_000 as Weight)
			// Standard Error: 31_000
			.saturating_add((2_680_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Vesting VestingSchedules (r:1 w:1)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: System Account (r:2 w:1)
	// Storage: Tokens Locks (r:1 w:1)
	fn vested_transfer() -> Weight {
		(138_156_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: Tokens Locks (r:1 w:1)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Vesting VestingSchedules (r:0 w:1)
	fn update_vesting_schedules(s: u32, ) -> Weight {
		(72_420_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((142_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Vesting VestingSchedules (r:1 w:1)
	// Storage: Tokens Locks (r:1 w:1)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	fn claim_for(s: u32, ) -> Weight {
		(70_131_000 as Weight)
			// Standard Error: 5_000
			.saturating_add((2_481_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
}
