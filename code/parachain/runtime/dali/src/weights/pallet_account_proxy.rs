
//! Autogenerated weights for `pallet_account_proxy`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-10-26, STEPS: `2`, REPEAT: 2, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `dev`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dali-dev"), DB CACHE: 1024

// Executed Command:
// /nix/store/0apddjbvqpg9m33hfhxzxsja22scw95c-composable/bin/composable
// benchmark
// pallet
// --chain=dali-dev
// --execution=wasm
// --wasm-execution=compiled
// --wasm-instantiation-strategy=legacy-instance-reuse
// --pallet=*
// --extrinsic=*
// --steps=2
// --repeat=2
// --output=code/parachain/runtime/dali/src/weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_account_proxy`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_account_proxy::WeightInfo for WeightInfo<T> {
	// Storage: Proxy Proxies (r:1 w:0)
	// Storage: CallFilter DisabledCalls (r:1 w:0)
	/// The range of component `p` is `[1, 3]`.
	fn proxy(_p: u32, ) -> Weight {
		(28_160_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
	}
	// Storage: Proxy Proxies (r:1 w:0)
	// Storage: Proxy Announcements (r:1 w:1)
	// Storage: CallFilter DisabledCalls (r:1 w:0)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 3]`.
	fn proxy_announced(a: u32, _p: u32, ) -> Weight {
		(40_716_000 as Weight)
			// Standard Error: 198_000
			.saturating_add((193_000 as Weight).saturating_mul(a as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Proxy Announcements (r:1 w:1)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 3]`.
	fn remove_announcement(a: u32, p: u32, ) -> Weight {
		(7_635_000 as Weight)
			// Standard Error: 67_000
			.saturating_add((265_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 1_044_000
			.saturating_add((525_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Proxy Announcements (r:1 w:1)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 3]`.
	fn reject_announcement(a: u32, p: u32, ) -> Weight {
		(9_354_000 as Weight)
			// Standard Error: 91_000
			.saturating_add((259_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 1_413_000
			.saturating_add((133_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Proxy Proxies (r:1 w:0)
	// Storage: Proxy Announcements (r:1 w:1)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 3]`.
	fn announce(a: u32, _p: u32, ) -> Weight {
		(30_483_000 as Weight)
			// Standard Error: 181_000
			.saturating_add((239_000 as Weight).saturating_mul(a as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Proxy Proxies (r:1 w:1)
	/// The range of component `p` is `[1, 3]`.
	fn add_proxy(p: u32, ) -> Weight {
		(22_327_000 as Weight)
			// Standard Error: 1_631_000
			.saturating_add((260_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Proxy Proxies (r:1 w:1)
	/// The range of component `p` is `[1, 3]`.
	fn remove_proxy(p: u32, ) -> Weight {
		(22_716_000 as Weight)
			// Standard Error: 2_948_000
			.saturating_add((458_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Proxy Proxies (r:1 w:1)
	/// The range of component `p` is `[1, 3]`.
	fn remove_proxies(_p: u32, ) -> Weight {
		(11_327_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	// Storage: Proxy Proxies (r:1 w:1)
	/// The range of component `p` is `[1, 3]`.
	fn anonymous(_p: u32, ) -> Weight {
		(26_312_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Proxy Proxies (r:1 w:1)
	/// The range of component `p` is `[0, 2]`.
	fn kill_anonymous(p: u32, ) -> Weight {
		(10_222_000 as Weight)
			// Standard Error: 845_000
			.saturating_add((833_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}
