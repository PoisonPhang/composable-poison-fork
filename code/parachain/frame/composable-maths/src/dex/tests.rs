use crate::dex::constant_product::compute_in_given_out_new;
use proptest::prelude::*;
use sp_runtime::{ArithmeticError, Permill};

/// Tests related to constant product math functions
mod constant_product {
	use super::*;
	/// Tests related to the function `compute_in_given_out_new`
	mod compute_in_given_out_new {
		use crate::dex::constant_product::ConstantProductAmmError;

		use super::*;

		#[derive(Debug, Eq, PartialEq, Clone, Copy)]
		struct InputsAndOutputs {
			w_i: Permill,
			w_o: Permill,
			b_i: u128,
			b_o: u128,
			a_out: u128,
			f: Permill,
			a_sent: u128,
			fee: u128,
		}

		const CHECKED_I_AND_O_LIST: [InputsAndOutputs; 5] = [
			InputsAndOutputs {
				w_i: Permill::from_percent(50),
				w_o: Permill::from_percent(50),
				b_i: 2048,
				b_o: 2048,
				a_out: 100,
				f: Permill::from_percent(10),
				a_sent: 117,
				fee: 12,
			},
			InputsAndOutputs {
				w_i: Permill::from_percent(34),
				w_o: Permill::from_percent(66),
				b_i: 1024,
				b_o: 2048,
				a_out: 100,
				f: Permill::from_percent(10),
				a_sent: 117,
				fee: 12,
			},
			InputsAndOutputs {
				w_i: Permill::from_percent(25),
				w_o: Permill::from_percent(75),
				b_i: 10_000,
				b_o: 30_000,
				a_out: 100,
				f: Permill::from_percent(10),
				a_sent: 112,
				fee: 12,
			},
			InputsAndOutputs {
				w_i: Permill::from_percent(40),
				w_o: Permill::from_percent(60),
				b_i: 20_000_000,
				b_o: 30_000_000,
				a_out: 100_000,
				f: Permill::from_percent(10),
				a_sent: 111_576,
				fee: 11_158,
			},
			InputsAndOutputs {
				w_i: Permill::from_percent(40),
				w_o: Permill::from_percent(20),
				b_i: 20_000_000,
				b_o: 10_000_000,
				a_out: 100_000,
				f: Permill::from_percent(10),
				a_sent: 111_952,
				fee: 11_196,
			},
		];

		prop_compose! {
			fn checked_inputs_and_outputs()
			(x in 0..CHECKED_I_AND_O_LIST.len()) -> InputsAndOutputs {
				CHECKED_I_AND_O_LIST[x]
			}
		}

		#[test]
		fn should_return_zero_fee_when_f_is_zero() {
			let w_i = Permill::from_rational::<u32>(1, 2);
			let w_o = Permill::from_rational::<u32>(1, 2);
			let b_i = 12;
			let b_o = 12;
			let a_out = 2;
			let f = Permill::zero();

			let res = compute_in_given_out_new(w_i, w_o, b_i, b_o, a_out, f)
				.expect("Input is valid; QED");

			assert_eq!(res.1, 0);
		}

		#[test]
		fn should_return_error_when_w_i_is_zero() {
			let w_i = Permill::zero();
			let w_o = Permill::from_rational::<u32>(1, 2);
			let b_i = 12;
			let b_o = 12;
			let a_out = 2;
			let f = Permill::from_percent(10);

			let res = compute_in_given_out_new(w_i, w_o, b_i, b_o, a_out, f);

			assert_eq!(res, Err(ConstantProductAmmError::from(ArithmeticError::DivisionByZero)));
		}

		#[test]
		fn should_return_error_when_b_o_and_a_out_are_zero() {
			let w_i = Permill::from_rational::<u32>(1, 2);
			let w_o = Permill::from_rational::<u32>(1, 2);
			let b_i = 12;
			let b_o = 0;
			let a_out = 0;
			let f = Permill::from_percent(10);

			let res = compute_in_given_out_new(w_i, w_o, b_i, b_o, a_out, f);

			assert_eq!(res, Err(ConstantProductAmmError::from(ArithmeticError::DivisionByZero)));
		}

		#[test]
		fn should_return_error_when_a_out_is_greater_than_b_o() {
			let w_i = Permill::from_rational::<u32>(1, 2);
			let w_o = Permill::from_rational::<u32>(1, 2);
			let b_i = 512;
			let b_o = 128;
			let a_out = 256;
			let f = Permill::from_percent(10);

			let res = compute_in_given_out_new(w_i, w_o, b_i, b_o, a_out, f);

			assert_eq!(res, Err(ConstantProductAmmError::CannotTakeMoreThanAvailable))
		}

		proptest! {
			#![proptest_config(ProptestConfig::with_cases(CHECKED_I_AND_O_LIST.len() as u32))]

			#[test]
			fn should_pass_with_expected_values(i_and_o in checked_inputs_and_outputs()) {
				let res = compute_in_given_out_new(
					i_and_o.w_i,
					i_and_o.w_o,
					i_and_o.b_i,
					i_and_o.b_o,
					i_and_o.a_out,
					i_and_o.f)
				.expect("Input is valid; QED");

				prop_assert_eq!(res.0, i_and_o.a_sent);
				prop_assert_eq!(res.1, i_and_o.fee);
			}
		}
	}
}
