
//! Autogenerated weights for `pallet_staking_rewards`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-08, STEPS: `50`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `05551ac21fb8`, CPU: `Intel(R) Xeon(R) CPU @ 3.10GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dali-dev"), DB CACHE: 1024

// Executed Command:
// /nix/store/9gdd70pyc12n9i1v6gx99rhz8q2n67z0-composable/bin/composable
// benchmark
// pallet
// --chain=dali-dev
// --execution=wasm
// --wasm-execution=compiled
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

/// Weight functions for `pallet_staking_rewards`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_staking_rewards::WeightInfo for WeightInfo<T> {
	// Storage: StakingRewards RewardPools (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Fnft Collection (r:1 w:1)
	/// The range of component `r` is `[1, 10]`.
	fn create_reward_pool(r: u32, ) -> Weight {
		// Minimum execution time: 63_152 nanoseconds.
		Weight::from_ref_time(64_993_910 as u64)
			// Standard Error: 28_049
			.saturating_add(Weight::from_ref_time(631_945 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: StakingRewards RewardPools (r:1 w:1)
	// Storage: Tokens Accounts (r:3 w:3)
	// Storage: Tokens TotalIssuance (r:2 w:1)
	// Storage: Fnft FinancialNftId (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Tokens Locks (r:2 w:2)
	// Storage: Fnft Instance (r:1 w:1)
	// Storage: Fnft Collection (r:1 w:0)
	// Storage: Fnft OwnerInstances (r:1 w:1)
	// Storage: Proxy Proxies (r:1 w:1)
	// Storage: StakingRewards Stakes (r:0 w:1)
	/// The range of component `r` is `[1, 10]`.
	fn stake(r: u32, ) -> Weight {
		// Minimum execution time: 237_642 nanoseconds.
		Weight::from_ref_time(241_955_678 as u64)
			// Standard Error: 54_411
			.saturating_add(Weight::from_ref_time(1_073_684 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(15 as u64))
			.saturating_add(T::DbWeight::get().writes(13 as u64))
	}
	// Storage: Fnft Instance (r:1 w:0)
	// Storage: StakingRewards Stakes (r:1 w:1)
	// Storage: StakingRewards RewardPools (r:1 w:1)
	// Storage: Tokens Accounts (r:3 w:3)
	// Storage: Tokens TotalIssuance (r:2 w:1)
	// Storage: Tokens Locks (r:2 w:2)
	// Storage: Timestamp Now (r:1 w:0)
	/// The range of component `r` is `[1, 10]`.
	fn extend(r: u32, ) -> Weight {
		// Minimum execution time: 174_722 nanoseconds.
		Weight::from_ref_time(178_381_045 as u64)
			// Standard Error: 50_910
			.saturating_add(Weight::from_ref_time(963_133 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(11 as u64))
			.saturating_add(T::DbWeight::get().writes(8 as u64))
	}
	// Storage: Fnft Instance (r:1 w:1)
	// Storage: StakingRewards Stakes (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: StakingRewards RewardPools (r:1 w:1)
	// Storage: Tokens TotalIssuance (r:1 w:0)
	// Storage: Tokens Locks (r:2 w:2)
	// Storage: Tokens Accounts (r:5 w:5)
	// Storage: System Account (r:3 w:3)
	// Storage: Fnft OwnerInstances (r:1 w:1)
	/// The range of component `r` is `[1, 10]`.
	fn unstake(r: u32, ) -> Weight {
		// Minimum execution time: 254_570 nanoseconds.
		Weight::from_ref_time(256_732_440 as u64)
			// Standard Error: 51_582
			.saturating_add(Weight::from_ref_time(3_159_922 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(16 as u64))
			.saturating_add(T::DbWeight::get().writes(14 as u64))
	}
	// Storage: Fnft Instance (r:2 w:1)
	// Storage: StakingRewards Stakes (r:1 w:2)
	// Storage: StakingRewards RewardPools (r:1 w:0)
	// Storage: Fnft FinancialNftId (r:1 w:1)
	// Storage: Fnft Collection (r:1 w:0)
	// Storage: Fnft OwnerInstances (r:1 w:1)
	// Storage: Proxy Proxies (r:1 w:1)
	// Storage: Tokens Locks (r:4 w:4)
	// Storage: Tokens Accounts (r:4 w:4)
	// Storage: System Account (r:2 w:1)
	/// The range of component `r` is `[1, 10]`.
	fn split(r: u32, ) -> Weight {
		// Minimum execution time: 296_777 nanoseconds.
		Weight::from_ref_time(301_082_677 as u64)
			// Standard Error: 60_192
			.saturating_add(Weight::from_ref_time(1_217_682 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(18 as u64))
			.saturating_add(T::DbWeight::get().writes(15 as u64))
	}
	// Storage: StakingRewards RewardsPotIsEmpty (r:1 w:1)
	// Storage: System Account (r:1 w:0)
	fn reward_accumulation_hook_reward_update_calculation() -> Weight {
		// Minimum execution time: 46_459 nanoseconds.
		Weight::from_ref_time(49_257_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Timestamp Now (r:1 w:0)
	fn unix_time_now() -> Weight {
		// Minimum execution time: 4_598 nanoseconds.
		Weight::from_ref_time(4_924_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	// Storage: StakingRewards RewardPools (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: StakingRewards RewardsPotIsEmpty (r:1 w:0)
	/// The range of component `r` is `[1, 10]`.
	fn update_rewards_pool(r: u32, ) -> Weight {
		// Minimum execution time: 60_524 nanoseconds.
		Weight::from_ref_time(35_462_547 as u64)
			// Standard Error: 495_255
			.saturating_add(Weight::from_ref_time(23_702_153 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(r as u64)))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Fnft Instance (r:1 w:0)
	// Storage: StakingRewards Stakes (r:1 w:1)
	// Storage: StakingRewards RewardPools (r:1 w:1)
	// Storage: Tokens TotalIssuance (r:1 w:0)
	/// The range of component `r` is `[1, 10]`.
	fn claim(_r: u32, ) -> Weight {
		// Minimum execution time: 78_332 nanoseconds.
		Weight::from_ref_time(99_398_380 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: StakingRewards RewardPools (r:1 w:1)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	// Storage: StakingRewards RewardsPotIsEmpty (r:1 w:0)
	fn add_to_rewards_pot() -> Weight {
		// Minimum execution time: 104_173 nanoseconds.
		Weight::from_ref_time(105_838_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
}
