use crate::dex::constant_product::compute_out_given_in_new;
use proptest::prelude::*;
use rust_decimal::prelude::*;
use sp_runtime::Permill;

/// Tests related to constant product math functions
mod constant_product {
	use super::*;
	/// Tests related to the function `compute_out_given_in_new`
	mod compute_out_given_in_new {
		use core::ops::Range;

		use super::*;

		const CHECKED_I_AND_O_LIST: [InputsAndOutputs; 5] = [
			InputsAndOutputs {
				w_i: Permill::from_percent(50),
				w_o: Permill::from_percent(50),
				b_i: 2048,
				b_o: 2048,
				a_sent: 100,
				f: Permill::from_percent(10),
				a_out: 86,
				fee: 10,
			},
			InputsAndOutputs {
				w_i: Permill::from_percent(34),
				w_o: Permill::from_percent(66),
				b_i: 1024,
				b_o: 2048,
				a_sent: 100,
				f: Permill::from_percent(10),
				a_out: 86,
				fee: 10,
			},
			InputsAndOutputs {
				w_i: Permill::from_percent(25),
				w_o: Permill::from_percent(75),
				b_i: 10_000,
				b_o: 30_000,
				a_sent: 100,
				f: Permill::from_percent(10),
				a_out: 89,
				fee: 10,
			},
			InputsAndOutputs {
				w_i: Permill::from_percent(40),
				w_o: Permill::from_percent(60),
				b_i: 20_000_000,
				b_o: 30_000_000,
				a_sent: 100_000,
				f: Permill::from_percent(10),
				a_out: 89_663,
				fee: 10_000,
			},
			InputsAndOutputs {
				w_i: Permill::from_percent(40),
				w_o: Permill::from_percent(20),
				b_i: 20_000_000,
				b_o: 10_000_000,
				a_sent: 100_000,
				f: Permill::from_percent(10),
				a_out: 89_396,
				fee: 10_000,
			},
		];

		#[derive(Debug, Eq, PartialEq, Clone, Copy)]
		struct InputsAndOutputs {
			w_i: Permill,
			w_o: Permill,
			b_i: u128,
			b_o: u128,
			a_sent: u128,
			f: Permill,
			a_out: u128,
			fee: u128,
		}

		prop_compose! {
			/// Returns (w_i, w_o, b_i, b_o, a_sent, fee, a_out, fee_out)
			fn checked_inputs_and_outputs()
			(x in 0..CHECKED_I_AND_O_LIST.len()) -> InputsAndOutputs {

				CHECKED_I_AND_O_LIST[x]
			}
		}

		prop_compose! {
			#[allow(clippy::useless_conversion)]
			fn range_inputs()
			(
				w_i in Range::<u32>::from(1..100),
				w_o in Range::<u32>::from(1..100),
				b_i in Range::<u128>::from(257_000_000_000_000..Decimal::MAX
					.to_u128()
					.expect("Decimal::MAX is safe for into ops; QED")),
				b_o in Range::<u128>::from(257_000_000_000_000..Decimal::MAX
					.to_u128()
					.expect("Decimal::MAX is safe for into ops; QED")),
				a_sent in Range::<u128>::from(1_000_000_000_000..256_000_000_000_000),
				f in Range::<u32>::from(0..10_000),
			)
			-> InputsAndOutputs {
				InputsAndOutputs {
					w_i: Permill::from_percent(w_i),
					w_o: Permill::from_percent(w_o),
					b_i,
					b_o,
					a_sent,
					f: Permill::from_parts(f),
					a_out: u128::default(),
					fee: u128::default(),
				}
			}
		}

		#[test]
		fn should_return_zero_fee_when_fee_is_zero() {
			let w_i = Permill::from_rational::<u32>(1, 2);
			let w_o = Permill::from_rational::<u32>(1, 2);
			let b_i = 12;
			let b_o = 12;
			let a_sent = 2;
			let fee = Permill::zero();

			let res = compute_out_given_in_new(w_i, w_o, b_i, b_o, a_sent, fee)
				.expect("Valid input; QED");

			assert_eq!(res.1, 0);
		}

		#[test]
		fn should_return_error_if_w_o_is_zero() {
			let w_i = Permill::from_rational::<u32>(1, 2);
			let w_o = Permill::zero();
			let b_i = 12;
			let b_o = 12;
			let a_sent = 2;
			let fee = Permill::zero();

			let res = compute_out_given_in_new(w_i, w_o, b_i, b_o, a_sent, fee);

			assert_eq!(res, Err(sp_runtime::ArithmeticError::DivisionByZero));
		}

		#[test]
		fn should_return_error_if_b_i_and_a_sent_are_zero() {
			let w_i = Permill::from_rational::<u32>(1, 2);
			let w_o = Permill::from_rational::<u32>(1, 2);
			let b_i = 0;
			let b_o = 12;
			let a_sent = 0;
			let fee = Permill::zero();

			let res = compute_out_given_in_new(w_i, w_o, b_i, b_o, a_sent, fee);

			assert_eq!(res, Err(sp_runtime::ArithmeticError::DivisionByZero));
		}

		proptest! {
			#![proptest_config(ProptestConfig::with_cases(CHECKED_I_AND_O_LIST.len() as u32))]

			#[test]
			fn should_pass_with_expected_values(i_and_o in checked_inputs_and_outputs()) {
				let res = compute_out_given_in_new(
					i_and_o.w_i,
					i_and_o.w_o,
					i_and_o.b_i,
					i_and_o.b_o,
					i_and_o.a_sent,
					i_and_o.f
				)
				.expect("Valid input; QED");

				prop_assert_eq!(res.0, i_and_o.a_out);
				prop_assert_eq!(res.1, i_and_o.fee);
			}
		}

		proptest! {
			#![proptest_config(ProptestConfig::with_cases(10_000))]

			#[test]
			fn no_unexpected_errors_in_range(i_and_o in range_inputs()) {
				let res = compute_out_given_in_new(
					i_and_o.w_i,
					i_and_o.w_o,
					i_and_o.b_i,
					i_and_o.b_o,
					i_and_o.a_out,
					i_and_o.f
				);

				prop_assert!(res.is_ok());
			}
		}
	}
}
