
//! Autogenerated weights for `session`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-08, STEPS: `50`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `5a3b4d6e5166`, CPU: `Intel(R) Xeon(R) CPU @ 3.10GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("picasso-dev"), DB CACHE: 1024

// Executed Command:
// /nix/store/9gdd70pyc12n9i1v6gx99rhz8q2n67z0-composable/bin/composable
// benchmark
// pallet
// --chain=picasso-dev
// --execution=wasm
// --wasm-execution=compiled
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

/// Weight functions for `session`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> session::WeightInfo for WeightInfo<T> {
	// Storage: Session NextKeys (r:1 w:1)
	// Storage: Session KeyOwner (r:1 w:1)
	fn set_keys() -> Weight {
		// Minimum execution time: 43_618 nanoseconds.
		Weight::from_ref_time(43_956_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Session NextKeys (r:1 w:1)
	// Storage: Session KeyOwner (r:0 w:1)
	fn purge_keys() -> Weight {
		// Minimum execution time: 36_377 nanoseconds.
		Weight::from_ref_time(37_121_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
}
