//! Autogenerated weights for timestamp
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-08-22, STEPS: `[5, ]`, REPEAT: 2, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("picasso-dev"), DB CACHE: 128

// Executed Command:
// ./target/release/composable
// benchmark
// --chain=picasso-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=timestamp
// --extrinsic=*
// --steps=5
// --repeat=2
// --raw
// --output=./runtime/picasso/src/weights

#![allow(unused_parens)]
#![allow(unused_imports)]

use sp_std::marker::PhantomData;
use support::{traits::Get, weights::Weight};

/// Weight functions for timestamp.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> timestamp::WeightInfo for WeightInfo<T> {
	fn set() -> Weight {
		(18_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn on_finalize() -> Weight {
		(7_000_000 as Weight)
	}
}
