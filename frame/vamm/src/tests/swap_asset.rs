use crate::{
	mock::{Balance, ExtBuilder, MockRuntime, System, TestPallet, VammId},
	pallet::{Error, Event, VammMap},
	tests::{
		helpers::{
			create_vamm, default_swap_config, default_vamm_config, run_for_seconds, run_to_block,
			swap_config,
		},
		helpers_propcompose::{
			any_vamm_state, balance_range_lower_half, balance_range_upper_half, get_swap_config,
			get_vamm_state, multiple_swaps, then_and_now,
		},
		TestSwapConfig, RUN_CASES,
	},
};
use composable_traits::vamm::{AssetType, Direction, SwapOutput, Vamm as VammTrait};
use frame_support::{assert_noop, assert_ok};
use proptest::prelude::*;
use sp_core::U256;
use sp_runtime::traits::Zero;

// -------------------------------------------------------------------------------------------------
//                                            Unit Tests
// -------------------------------------------------------------------------------------------------

#[test]
fn should_succeed_returning_correct_values_and_emitting_events_add_base() {
	ExtBuilder::default().build().execute_with(|| {
		let vamm_config = default_vamm_config();
		let swap_config = default_swap_config(AssetType::Base, Direction::Add);
		create_vamm(&vamm_config);

		// For event emission
		run_to_block(1);

		let swap = TestPallet::swap(&swap_config);
		let vamm_after_swap = VammMap::<MockRuntime>::get(swap_config.vamm_id).unwrap();
		assert_ok!(swap, SwapOutput { output: 16666666666666666667, negative: false });
		assert_eq!(vamm_after_swap.base_asset_reserves, 3000000000000000000);
		assert_eq!(vamm_after_swap.quote_asset_reserves, 33333333333333333333);
		assert_eq!(
			vamm_after_swap.base_asset_reserves,
			vamm_config.base_asset_reserves + swap_config.input_amount
		);
		assert_eq!(
			vamm_after_swap.quote_asset_reserves + swap.unwrap().output,
			vamm_config.quote_asset_reserves
		);
		System::assert_last_event(
			Event::Swapped {
				vamm_id: swap_config.vamm_id,
				input_amount: swap_config.input_amount,
				output_amount: swap.unwrap(),
				input_asset_type: swap_config.asset,
				direction: swap_config.direction,
			}
			.into(),
		);
	})
}

#[test]
fn should_succeed_returning_correct_values_and_emitting_events_add_quote() {
	ExtBuilder::default().build().execute_with(|| {
		let vamm_config = default_vamm_config();
		let swap_config = default_swap_config(AssetType::Quote, Direction::Add);
		create_vamm(&vamm_config);

		// For event emission
		run_to_block(1);

		let swap = TestPallet::swap(&swap_config);
		let vamm_after_swap = VammMap::<MockRuntime>::get(swap_config.vamm_id).unwrap();
		assert_ok!(swap, SwapOutput { output: 39215686274509804, negative: false });
		assert_eq!(vamm_after_swap.base_asset_reserves, 1960784313725490196);
		assert_eq!(vamm_after_swap.quote_asset_reserves, 51000000000000000000);
		assert_eq!(
			vamm_after_swap.quote_asset_reserves,
			vamm_config.quote_asset_reserves + swap_config.input_amount
		);
		assert_eq!(
			vamm_after_swap.base_asset_reserves + swap.unwrap().output,
			vamm_config.base_asset_reserves
		);
		System::assert_last_event(
			Event::Swapped {
				vamm_id: swap_config.vamm_id,
				input_amount: swap_config.input_amount,
				output_amount: swap.unwrap(),
				input_asset_type: swap_config.asset,
				direction: swap_config.direction,
			}
			.into(),
		);
	})
}

#[test]
fn should_succeed_returning_correct_values_and_emitting_events_remove_base() {
	ExtBuilder::default().build().execute_with(|| {
		let vamm_config = default_vamm_config();
		let swap_config = default_swap_config(AssetType::Base, Direction::Remove);
		create_vamm(&vamm_config);

		// For event emission
		run_to_block(1);

		let swap = TestPallet::swap(&swap_config);
		let vamm_after_swap = VammMap::<MockRuntime>::get(swap_config.vamm_id).unwrap();
		assert_ok!(swap, SwapOutput { output: 50000000000000000000, negative: false });
		assert_eq!(vamm_after_swap.base_asset_reserves, 1000000000000000000);
		assert_eq!(vamm_after_swap.quote_asset_reserves, 100000000000000000000);
		assert_eq!(
			vamm_after_swap.quote_asset_reserves,
			vamm_config.quote_asset_reserves + swap.unwrap().output
		);
		assert_eq!(
			vamm_after_swap.base_asset_reserves,
			vamm_config.base_asset_reserves - swap_config.input_amount
		);
		System::assert_last_event(
			Event::Swapped {
				vamm_id: swap_config.vamm_id,
				input_amount: swap_config.input_amount,
				output_amount: swap.unwrap(),
				input_asset_type: swap_config.asset,
				direction: swap_config.direction,
			}
			.into(),
		);
	})
}

#[test]
fn should_succeed_returning_correct_values_and_emitting_events_remove_quote() {
	ExtBuilder::default().build().execute_with(|| {
		let vamm_config = default_vamm_config();
		let swap_config = default_swap_config(AssetType::Quote, Direction::Remove);
		create_vamm(&vamm_config);

		// For event emission
		run_to_block(1);

		let swap = TestPallet::swap(&swap_config);
		let vamm_after_swap = VammMap::<MockRuntime>::get(swap_config.vamm_id).unwrap();
		assert_ok!(swap, SwapOutput { output: 40816326530612244, negative: true });
		assert_eq!(vamm_after_swap.base_asset_reserves, 2040816326530612244);
		assert_eq!(vamm_after_swap.quote_asset_reserves, 49000000000000000000);
		assert_eq!(
			vamm_after_swap.base_asset_reserves,
			vamm_config.base_asset_reserves + swap.unwrap().output
		);
		assert_eq!(
			vamm_after_swap.quote_asset_reserves,
			vamm_config.quote_asset_reserves - swap_config.input_amount
		);
		System::assert_last_event(
			Event::Swapped {
				vamm_id: swap_config.vamm_id,
				input_amount: swap_config.input_amount,
				output_amount: swap.unwrap(),
				input_asset_type: swap_config.asset,
				direction: swap_config.direction,
			}
			.into(),
		);
	})
}

// -------------------------------------------------------------------------------------------------
//                                             Proptests
// -------------------------------------------------------------------------------------------------

proptest! {
	#![proptest_config(ProptestConfig::with_cases(RUN_CASES))]
	#[test]
	fn should_fail_if_vamm_does_not_exist(
		vamm_state in get_vamm_state(Default::default()),
		swap_config in get_swap_config(Default::default()),
	) {
		prop_assume!(swap_config.vamm_id != 0);

		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			assert_noop!(
				TestPallet::swap(&swap_config),
				Error::<MockRuntime>::VammDoesNotExist
			);
		})
	}

	#[test]
	fn should_fail_if_vamm_is_closed(
		mut vamm_state in get_vamm_state(Default::default()),
		mut swap_config in get_swap_config(Default::default()),
		(close, now) in then_and_now()
	) {
		// Make the current time be greater than the time when the vamm is
		// set to close, doing this we ensure we can't make swaps due to the
		// vamm be closed.
		vamm_state.closed = Some(close);
		swap_config.vamm_id = VammId::zero();

		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			run_to_block(now);

			assert_noop!(
				TestPallet::swap(&swap_config),
				Error::<MockRuntime>::VammIsClosed
			);
		})
	}

	#[test]
	fn should_fail_if_output_is_less_than_minimum_limit(
		mut vamm_state in any_vamm_state(),
		mut swap_config in get_swap_config(Default::default()),
		limit in balance_range_upper_half(),
	) {
		// Ensure vamm is open before start operation to swap assets.
		vamm_state.closed = None;

		// Ensure input amount will not cause `InsufficientFundsForTrade`,
		// `Overflow`, `Underflow`, etc.
		swap_config.input_amount = 0;

		swap_config.output_amount_limit = limit;
		swap_config.vamm_id = VammId::zero();

		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			assert_noop!(
				TestPallet::swap(&swap_config),
				Error::<MockRuntime>::SwappedAmountLessThanMinimumLimit
			);
		})
	}

	#[test]
	fn should_succeed_emitting_event(
		mut vamm_state in any_vamm_state(),
		mut swap_config in swap_config(),
	) {
		// Ensure vamm is open before start operation to swap assets.
		vamm_state.closed = None;

		// Disable output limit check
		swap_config.output_amount_limit = Balance::zero();

		swap_config.vamm_id = VammId::zero();

		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			// For event emission
			run_for_seconds(1);
			let swap = TestPallet::swap(&swap_config);
			assert_ok!(swap);
			System::assert_last_event(
				Event::Swapped {
					vamm_id: swap_config.vamm_id,
					input_amount: swap_config.input_amount,
					output_amount: swap.unwrap(),
					input_asset_type: swap_config.asset,
					direction: swap_config.direction,
				}.into()
			);
		})
	}

	#[test]
	fn should_fail_if_insufficient_funds_base(
		mut vamm_state in get_vamm_state(Default::default()),
		mut swap_config in get_swap_config(
			TestSwapConfig {
				direction: Some(Direction::Remove),
				vamm_id: Some(0),
				asset: Some(AssetType::Base),
				..Default::default()}),
		input_amount in balance_range_upper_half(),
		base_asset_reserves in balance_range_lower_half(),
	) {
		prop_assume!(input_amount > base_asset_reserves);
		prop_assume!(swap_config.direction == Direction::Remove);

		// Ensure vamm is open before starting operation to swap assets.
		vamm_state.closed = None;

		swap_config.input_amount = input_amount;
		vamm_state.base_asset_reserves = base_asset_reserves;

		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			assert_noop!(
				TestPallet::swap(&swap_config),
				Error::<MockRuntime>::InsufficientFundsForTrade
			);
		})
	}

	#[test]
	fn should_fail_if_insufficient_funds_quote(
		mut vamm_state in get_vamm_state(Default::default()),
		mut swap_config in get_swap_config(
			TestSwapConfig {
				direction: Some(Direction::Remove),
				vamm_id: Some(0),
				asset: Some(AssetType::Quote),
				..Default::default()}),
		input_amount in balance_range_upper_half(),
		quote_asset_reserves in balance_range_lower_half(),
	) {
		prop_assume!(input_amount > quote_asset_reserves);
		prop_assume!(swap_config.direction == Direction::Remove);

		// Ensure vamm is open before starting operation to swap assets.
		vamm_state.closed = None;

		swap_config.input_amount = input_amount;
		vamm_state.quote_asset_reserves = quote_asset_reserves;

		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			assert_noop!(
				TestPallet::swap(&swap_config),
				Error::<MockRuntime>::InsufficientFundsForTrade
			);
		})
	}

	#[test]
	fn should_succeed_removing_base(
		mut vamm_state in any_vamm_state(),
		mut swap_config in get_swap_config(
			TestSwapConfig {
				direction: Some(Direction::Remove),
				vamm_id: Some(0),
				asset: Some(AssetType::Base),
				..Default::default()}),
	) {
		// Ensure vamm is open before starting operation to swap assets.
		vamm_state.closed = None;

		// Ensure input amount will not cause `InsufficientFundsForTrade`,
		// `Overflow`, `Underflow`, etc.
		swap_config.input_amount = 0;

		// Disable output limit check
		swap_config.output_amount_limit = 0;

		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			assert_ok!(TestPallet::swap(&swap_config));
		})
	}

	#[test]
	fn should_succeed_removing_quote(
		mut vamm_state in any_vamm_state(),
		mut swap_config in get_swap_config(
			TestSwapConfig {
				direction: Some(Direction::Remove),
				vamm_id: Some(0),
				asset: Some(AssetType::Quote),
				..Default::default()}),
	) {
		// Ensure vamm is open before starting operation to swap assets.
		vamm_state.closed = None;

		// Ensure input amount will not cause `InsufficientFundsForTrade`,
		// `Overflow`, `Underflow`, etc.
		swap_config.input_amount = 0;

		// Disable output limit check
		swap_config.output_amount_limit = 0;

		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			assert_ok!(TestPallet::swap(&swap_config));
		})
	}

	#[test]
	fn should_fail_if_trade_extrapolates_maximum_supported_amount_base(
		mut vamm_state in get_vamm_state(Default::default()),
		mut swap_config in get_swap_config(
			TestSwapConfig {
				direction: Some(Direction::Add),
				vamm_id: Some(0),
				asset: Some(AssetType::Base),
				..Default::default()}),
		input_amount in balance_range_upper_half(),
		base_asset_reserves in balance_range_upper_half(),
	) {
		prop_assume!(swap_config.direction == Direction::Add);

		// Ensure vamm is open before starting operation to swap assets.
		vamm_state.closed = None;

		swap_config.input_amount = input_amount;
		vamm_state.base_asset_reserves = base_asset_reserves;

		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			assert_noop!(
				TestPallet::swap(&swap_config),
				Error::<MockRuntime>::TradeExtrapolatesMaximumSupportedAmount
			);
		})
	}

	#[test]
	fn should_fail_if_trade_extrapolates_maximum_supported_amount_quote(
		mut vamm_state in get_vamm_state(Default::default()),
		mut swap_config in get_swap_config(
			TestSwapConfig {
				direction: Some(Direction::Add),
				vamm_id: Some(0),
				asset: Some(AssetType::Quote),
				..Default::default()}),
		input_amount in balance_range_upper_half(),
		quote_asset_reserves in balance_range_upper_half(),
	) {
		prop_assume!(swap_config.direction == Direction::Add);

		// Ensure vamm is open before starting operation to swap assets.
		vamm_state.closed = None;

		swap_config.input_amount = input_amount;
		vamm_state.quote_asset_reserves = quote_asset_reserves;

		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			assert_noop!(
				TestPallet::swap(&swap_config),
				Error::<MockRuntime>::TradeExtrapolatesMaximumSupportedAmount
			);
		})
	}
}

proptest! {
	#![proptest_config(ProptestConfig::with_cases(1))]
	#[test]
	fn multiple_swaps_dont_diverge_from_original_invariant(
		mut vamm_state in any_vamm_state(),
		swap_config in multiple_swaps()
	) {
		// Ensure vamm is always open
		vamm_state.closed = None;

		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			let vamm_before_swap = VammMap::<MockRuntime>::get(0);
			for x in swap_config.iter() {
				assert_ok!(TestPallet::swap(x));
			}
			let vamm_after_swap = VammMap::<MockRuntime>::get(0);

			let invariant_before = TestPallet::compute_invariant(
				vamm_before_swap.unwrap().base_asset_reserves,
				vamm_before_swap.unwrap().quote_asset_reserves,
			).unwrap();

			let invariant_after = TestPallet::compute_invariant(
				vamm_after_swap.unwrap().base_asset_reserves,
				vamm_after_swap.unwrap().quote_asset_reserves,
			).unwrap();

			let invariant_epsilon = invariant_before / U256::exp10(8);
			let invariant_delta = invariant_before.max(invariant_after)
				- invariant_before.min(invariant_after);

			assert!(invariant_delta <= invariant_epsilon);
		});
	}

	#[test]
	fn multiple_swaps_dont_diverge_from_original_invariant_only_base(
		mut vamm_state in any_vamm_state(),
		mut swap_config in multiple_swaps()
	) {
		// Ensure vamm is always open
		vamm_state.closed = None;

		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			let vamm_before_swap = VammMap::<MockRuntime>::get(0);
			for mut x in swap_config.iter_mut() {
				// Make swaps only for base asset
				x.asset = AssetType::Base;
				assert_ok!(TestPallet::swap(x));
			}
			let vamm_after_swap = VammMap::<MockRuntime>::get(0);

			let invariant_before = TestPallet::compute_invariant(
				vamm_before_swap.unwrap().base_asset_reserves,
				vamm_before_swap.unwrap().quote_asset_reserves,
			).unwrap();

			let invariant_after = TestPallet::compute_invariant(
				vamm_after_swap.unwrap().base_asset_reserves,
				vamm_after_swap.unwrap().quote_asset_reserves,
			).unwrap();

			let invariant_epsilon = invariant_before / U256::exp10(8);
			let invariant_delta = invariant_before.max(invariant_after)
				- invariant_before.min(invariant_after);

			assert!(invariant_delta <= invariant_epsilon);
		});

	}

	#[test]
	fn multiple_swaps_dont_diverge_from_original_invariant_only_quote(
		mut vamm_state in any_vamm_state(),
		mut swap_config in multiple_swaps()
	) {
		// Ensure vamm is always open
		vamm_state.closed = None;

		ExtBuilder {
			vamm_count: 1,
			vamms: vec![(0, vamm_state)]
		}.build().execute_with(|| {
			let vamm_before_swap = VammMap::<MockRuntime>::get(0);
			for mut x in swap_config.iter_mut() {
				// Make swaps only for quote asset
				x.asset = AssetType::Quote;
				assert_ok!(TestPallet::swap(x));
			}
			let vamm_after_swap = VammMap::<MockRuntime>::get(0);

			let invariant_before = TestPallet::compute_invariant(
				vamm_before_swap.unwrap().base_asset_reserves,
				vamm_before_swap.unwrap().quote_asset_reserves,
			).unwrap();

			let invariant_after = TestPallet::compute_invariant(
				vamm_after_swap.unwrap().base_asset_reserves,
				vamm_after_swap.unwrap().quote_asset_reserves,
			).unwrap();

			let invariant_epsilon = invariant_before / U256::exp10(8);
			let invariant_delta = invariant_before.max(invariant_after)
				- invariant_before.min(invariant_after);

			assert!(invariant_delta <= invariant_epsilon);
		});
	}
}
