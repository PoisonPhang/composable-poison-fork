//! Autogenerated weights for membership
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-08-23, STEPS: `[50, ]`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("picasso-dev"), DB CACHE: 128

// Executed Command:
// ./target/release/composable
// benchmark
// --chain=picasso-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=membership
// --extrinsic=*
// --steps=50
// --repeat=20
// --raw
// --output=./runtime/picasso/src/weights

#![allow(unused_parens)]
#![allow(unused_imports)]

use sp_std::marker::PhantomData;
use support::{traits::Get, weights::Weight};

/// Weight functions for membership.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> membership::WeightInfo for WeightInfo<T> {
	fn add_member(m: u32) -> Weight {
		(78_864_000 as Weight)
			// Standard Error: 57_000
			.saturating_add((925_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn remove_member(m: u32) -> Weight {
		(81_076_000 as Weight)
			// Standard Error: 10_000
			.saturating_add((494_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn swap_member(m: u32) -> Weight {
		(84_930_000 as Weight)
			// Standard Error: 16_000
			.saturating_add((612_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn reset_member(m: u32) -> Weight {
		(90_093_000 as Weight)
			// Standard Error: 9_000
			.saturating_add((1_065_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn change_key(m: u32) -> Weight {
		(76_029_000 as Weight)
			// Standard Error: 18_000
			.saturating_add((903_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn set_prime(_m: u32) -> Weight {
		(57_245_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn clear_prime(_m: u32) -> Weight {
		(12_220_000 as Weight).saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}
