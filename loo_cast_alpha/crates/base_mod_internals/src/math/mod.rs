//! # Scope
//! - Provides math types and functionality in the form of horribly-unergonomic but highly-generalized backends.
//!
//! Currently supports the following mathematical objects:\
//! [`scalar`]\
//! [`vector`]\
//! [`matrix`]\
//! [`tensor`]\
//! [`tensor4`]\
//! [`quaternion`]\
//! [`transform`]
//!
//! # Architecture
//! Each submodule is represented by the following sub-submodules:\
//! - [`aliases`]: Provides type aliases.
//! - [`shared`]: Provides the respective `*Contract` trait, and the "sub traits" it composes.
//! - [`normal`]: Implements the respective `*Contract` trait generically, and monomorphizes it into `normal` type aliases.
//! - [`usf`]: Implements the respective `*Contract` trait generically, and monomorphizes it into `usf` type aliases.
//!
//! ## More on `shared`:
//! - Provides the `*Contract` trait(s) for the respective mathematical object.
//! - This trait is NOT to be implemented manually; it is an implementation detail.
//!
//! ## More on `normal`:
//! - Provides the `Normal*` type for the respective mathematical object.
//! - This type is generic, and is locally "monomorphized" into concrete type aliases.
//! - These type aliases and the respective shared `*Contract` trait(s) compose the `normal` facade-facing API for working with these math internals.
//!
//! ## More on `usf`:
//! - Provides the `Usf*` type for the respective mathematical object.
//! - This type is generic, and is locally "monomorphized" into concrete type aliases.
//! - These type aliases and the respective shared `*Contract` trait(s) are the `usf` backbone for working with this module.
//!
//! The (`normal` and `usf`) type aliases and the `*Contract` traits together make up the "public" API for facades to consume.
//!
//! # Use Cases and Examples
//!

pub mod aliases;
pub mod conversion;
pub mod field;
pub mod matrix;
pub mod op_kind;
pub mod op_mode;
pub mod op_policy;
pub mod quaternion;
pub mod scalar;
pub mod tensor;
pub mod tensor4;
pub mod transform;
pub mod vector;

#[cfg(test)]
mod tests {
    use super::scalar::aliases::UsfOrNormalScalar;
    use super::scalar::rand::RandomDistributionBuilder;
    use super::scalar::usf::{UsfScalar, UsfScalarConstants};

    /// Builds the randomized corpus for the add-commutativity sweep.
    ///
    /// Scope policy:
    /// - Keep this as the only sweep corpus helper until prototype approval.
    /// - Expand into additional op/invariant corpus helpers only after review.
    fn add_commutativity_sweep_values(seed: u64, count: usize) -> Vec<UsfScalar> {
        let pos_one_trillion = <UsfScalar as UsfScalarConstants>::POSITIVE_ONE_TRILLION;
        let neg_one_trillion = <UsfScalar as UsfScalarConstants>::NEGATIVE_ONE_TRILLION;

        RandomDistributionBuilder::new()
            .component(1, |c| c.positive().integer().less_than(pos_one_trillion))
            .component(1, |c| c.negative().integer().greater_than(neg_one_trillion))
            .build(seed, count)
    }

    /// Single-invariant sweep prototype.
    ///
    /// Intent:
    /// - Keep one clear property test shape for design review.
    /// - Delay broader expansion until this layout is approved.
    mod add_invariant_sweep_prototype {
        use super::{UsfOrNormalScalar, add_commutativity_sweep_values};
        use crate::math::scalar::shared::ScalarCoreOps;

        /// Sweep invariant for `add`: `a + b == b + a`.
        #[test]
        fn add_commutative_sweep_prototype() {
            let values = add_commutativity_sweep_values(0xADDD_0001, 155); // 155 and above crashes? It seems that maybe the amount of generated values changes the effective rng seed and thus produces a different set of numbers at len=155, hence why the error is NOT happening with number 155, but wy before that, suggesting new number generations may retroactively influence past generations? What the fuck? That cannot be right!
            for (idx_a, a) in values.iter().enumerate() {
                println!("({}: {})", idx_a, a);
                for (idx_b, b) in values.iter().enumerate() {
                    let lhs = a.clone().add(UsfOrNormalScalar::A(b.clone()));
                    let rhs = b.clone().add(UsfOrNormalScalar::A(a.clone()));
                    assert_eq!(lhs, rhs, "add commutativity failed for pair index ({idx_a}, {idx_b})");
                }
            }
        }
    }

    mod scalar_representation_invariants {
        use super::UsfScalar;
        use crate::math::scalar::shared::ScalarCoreOps;

        #[test]
        fn usf_scalar_core_test() {
            let zero = UsfScalar::ZERO;
            let one = UsfScalar::ONE;
            let epsilon = UsfScalar::EPSILON;
            let max = UsfScalar::MAX;
            let min = UsfScalar::MIN;

            assert_eq!(format!("{zero}"), "0");
            assert_eq!(format!("{one}"), "1");
            assert_eq!(format!("{epsilon}"), "0.00000000000000000000000000000000001");
            assert_eq!(format!("{max}"), "399999999999999999999999999999999999.99999999999999999999999999999999999");
            assert_eq!(format!("{min}"), "-499999999999999999999999999999999999.99999999999999999999999999999999999");

            assert_eq!(one, one);
            assert!(one > zero);
            assert!(zero < one);
            assert!(max > one);
            assert!(min < zero);
        }

        #[test]
        fn usf_scalar_debug_fmt_test() {
            let number = UsfScalar::ONE;
            let result = format!("{:?}", number);
            let expected = "UsfScalar { digits: ScalarDecimalDigits[ 000000000000000000000000000000000001.00000000000000000000000000000000000 ] }";
            assert_eq!(result, expected);
        }

        #[test]
        fn usf_scalar_display_fmt_test() {
            let number = UsfScalar::ONE;
            let result = format!("{}", number);
            let expected = "1";
            assert_eq!(result, expected);
        }

        #[test]
        fn usf_scalar_from_scientific_str_core_test() {
            let number_str_scientific = "1.61625512345678987654321e-35";
            let number_str_decimal_public = "0.00000000000000000000000000000000002";
            let number_str_decimal_internal = "0.00000000000000000000000000000000001616255123";
            let number = UsfScalar::from_scientific_str(number_str_scientific);
            let number_str_recovered_public = format!("{}", number);
            let number_str_recovered_internal = number.to_decimal_str_internal();

            assert_eq!(number_str_recovered_public, number_str_decimal_public);
            assert_eq!(number_str_recovered_internal, number_str_decimal_internal);
        }

        #[test]
        fn usf_scalar_from_decimal_str_rounding_test() {
            let number_str_decimal_up = "0.00000000000000000000000000000000001616255123";
            let number_str_decimal_up_public = "0.00000000000000000000000000000000002";

            let number_up = UsfScalar::from_decimal_str(number_str_decimal_up);
            let number_up_public = format!("{}", number_up);
            let number_up_internal = number_up.to_decimal_str_internal();

            assert_eq!(number_up_public, number_str_decimal_up_public);
            assert_eq!(number_up_internal, number_str_decimal_up);

            let number_str_decimal_down = "0.00000000000000000000000000000000001416255123";
            let number_str_decimal_down_public = "0.00000000000000000000000000000000001";

            let number_down = UsfScalar::from_decimal_str(number_str_decimal_down);
            let number_down_public = format!("{}", number_down);
            let number_down_internal = number_down.to_decimal_str_internal();

            assert_eq!(number_down_public, number_str_decimal_down_public);
            assert_eq!(number_down_internal, number_str_decimal_down);
        }

        #[test]
        fn usf_scalar_from_decimal_str_excess_frac_err_test() {
            let number_str_decimal_excess = "0.0000000000000000000000000000000000161625512345678987654321";
            let err = UsfScalar::try_from_decimal_str(number_str_decimal_excess).unwrap_err();
            assert!(err.message().contains("fractional part exceeds"));
        }

        #[test]
        fn usf_scalar_roundtrip_stability_test() {
            let seeds = [
                "0",
                "-0.0000",
                "42",
                "-42",
                "000000000000000000000000000457827552.09973578589733825723454287935874215",
                "-000000000000000000000000000457827552.09973578589733825723454287935874215",
                "0.1",
                "-0.1",
            ];

            for seed in seeds {
                let parsed = <UsfScalar as ScalarCoreOps>::from_decimal_str(seed);
                let parsed_digits = <UsfScalar as ScalarCoreOps>::to_digits(&parsed);

                let mut scalar = <UsfScalar as ScalarCoreOps>::from_digits(parsed_digits.0, parsed_digits.1, parsed_digits.2, parsed_digits.3);
                let mut as_string = <UsfScalar as ScalarCoreOps>::to_decimal_str(&scalar);
                let mut raw_digits = <UsfScalar as ScalarCoreOps>::to_digits(&scalar);

                for _ in 0..8 {
                    let from_raw = <UsfScalar as ScalarCoreOps>::from_digits(raw_digits.0, raw_digits.1, raw_digits.2, raw_digits.3);
                    assert_eq!(from_raw, scalar, "raw roundtrip changed scalar for seed `{seed}`");
                    assert_eq!(
                        <UsfScalar as ScalarCoreOps>::to_digits(&from_raw),
                        raw_digits,
                        "raw roundtrip changed digit tuple for seed `{seed}`",
                    );

                    let from_string = <UsfScalar as ScalarCoreOps>::from_decimal_str(as_string.as_str());
                    let string_after = <UsfScalar as ScalarCoreOps>::to_decimal_str(&from_string);
                    assert_eq!(string_after, as_string, "string roundtrip changed text for seed `{seed}`");

                    let digits_after = <UsfScalar as ScalarCoreOps>::to_digits(&from_string);
                    assert_eq!(digits_after, raw_digits, "string roundtrip changed raw digits for seed `{seed}`");

                    let scalar_after = <UsfScalar as ScalarCoreOps>::from_digits(digits_after.0, digits_after.1, digits_after.2, digits_after.3);
                    assert_eq!(scalar_after, scalar, "string/raw cycle changed scalar repr for seed `{seed}`");

                    scalar = scalar_after;
                    raw_digits = digits_after;
                    as_string = string_after;
                }
            }
        }
    }
}
