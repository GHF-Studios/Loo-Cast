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
    use super::field::Field;
    use super::op_kind::VectorMulKind;
    use super::scalar::aliases::UsfOrNormalScalar;
    use super::scalar::usf::UsfScalar;
    use super::vector::aliases::{UsfOrNormalVector, VectorProductOperand};
    use super::vector::usf::UsfVector3d;
    use crate::math::scalar::shared::{SCALAR_FRAC_DIGITS_LEN, SCALAR_INT_DIGITS_LEN, ScalarCoreOps};
    use base_mod_shared::utils::one_of::OneOf2;

    fn seeded_digit_sets(seed: u64, digit_count: usize, digit_set_count: usize) -> Vec<Vec<u8>> {
        use rand::rngs::StdRng;
        use rand::{Rng, SeedableRng};

        let mut rng = StdRng::seed_from_u64(seed);
        let digit_sets: Vec<Vec<u8>> = (0..digit_set_count)
            .map(|_| {
                let digit_set: Vec<u8> = (0..digit_count).map(|_| rng.random_range(0_u8..10_u8)).collect();
                digit_set
            })
            .collect();
        digit_sets
    }

    fn seeded_numbers(seed: u64, digit_count: usize, digit_set_count: usize) -> Vec<UsfScalar> {
        use rand::rngs::StdRng;
        use rand::{Rng, SeedableRng};

        fn would_overflow(negative: bool, int_digits: &[u8], frac_digits: &[u8]) -> bool {
            let mut carry: i16 = 0;
            for d in int_digits.iter().copied().chain(frac_digits.iter().copied()).rev() {
                let v = (if negative { -(d as i16) } else { d as i16 }) + carry;
                let bal = ((v + 5).rem_euclid(10)) - 5;
                carry = (v - bal).div_euclid(10);
            }
            carry != 0
        }

        let primitive_digit_sets = seeded_digit_sets(seed, digit_count, digit_set_count);
        let mut sign_rng = StdRng::seed_from_u64(seed.wrapping_add(0x9E37_79B9_7F4A_7C15));

        primitive_digit_sets
            .into_iter()
            .map(|digits| {
                assert!(digits.len() <= 71, "digit set must be <= 71");

                let split = digits.len().min(SCALAR_INT_DIGITS_LEN);
                let (int_src, frac_src) = digits.split_at(split);

                let mut int_digits = [0_u8; SCALAR_INT_DIGITS_LEN];
                let int_start = SCALAR_INT_DIGITS_LEN - int_src.len();
                for (offset, digit) in int_src.iter().copied().enumerate() {
                    int_digits[int_start + offset] = digit;
                }

                let mut frac_digits = [0_u8; SCALAR_FRAC_DIGITS_LEN];
                let frac_len = frac_src.len().min(SCALAR_FRAC_DIGITS_LEN);
                for (offset, digit) in frac_src.iter().copied().take(frac_len).enumerate() {
                    frac_digits[offset] = digit;
                }
                let mut negative: bool = sign_rng.random(); // ~50% true
                if would_overflow(negative, &int_digits, &frac_digits[..frac_len]) {
                    negative = !negative;
                }
                if would_overflow(negative, &int_digits, &frac_digits[..frac_len]) {
                    // Keep the topmost decimal digit in a range that cannot emit an extra head carry.
                    int_digits[0] %= 4;
                }

                let radix_index = if frac_len == 0 {
                    (SCALAR_INT_DIGITS_LEN as i8) - 1
                } else {
                    i8::try_from((SCALAR_INT_DIGITS_LEN - 1) + frac_len).unwrap()
                };
                <UsfScalar as ScalarCoreOps>::from_decimal_u8_digits(negative, int_digits, frac_digits, radix_index)
            })
            .collect()
    }

    // #[test]
    fn rng_test() {
        let mut numbers = seeded_numbers(1337, 71, 17);
        // let b = numbers.pop().unwrap();
        // let a = numbers.pop().unwrap();

        // println!("a: {}", a);
        // println!("b: {}", b);

        for number in numbers {
            println!("{}", number);
        }
    }

    #[test]
    fn other_test() {
        let number = UsfScalar::from_scientific_str("1.616e-32");
        println!("{}", number);
    }

    #[test]
    fn usf_decimal_roundtrip_stability_test() {
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
            let parsed_digits = <UsfScalar as ScalarCoreOps>::to_decimal_u8_digits(&parsed);

            let mut scalar = <UsfScalar as ScalarCoreOps>::from_decimal_u8_digits(parsed_digits.0, parsed_digits.1, parsed_digits.2, parsed_digits.3);
            let mut as_string = <UsfScalar as ScalarCoreOps>::to_decimal_str(&scalar);
            let mut raw_digits = <UsfScalar as ScalarCoreOps>::to_decimal_u8_digits(&scalar);

            for _ in 0..8 {
                let from_raw = <UsfScalar as ScalarCoreOps>::from_decimal_u8_digits(raw_digits.0, raw_digits.1, raw_digits.2, raw_digits.3);
                assert_eq!(from_raw, scalar, "raw roundtrip changed scalar for seed `{seed}`");
                assert_eq!(
                    <UsfScalar as ScalarCoreOps>::to_decimal_u8_digits(&from_raw),
                    raw_digits,
                    "raw roundtrip changed digit tuple for seed `{seed}`",
                );

                let from_string = <UsfScalar as ScalarCoreOps>::from_decimal_str(as_string.as_str());
                let string_after = <UsfScalar as ScalarCoreOps>::to_decimal_str(&from_string);
                assert_eq!(string_after, as_string, "string roundtrip changed text for seed `{seed}`");

                let digits_after = <UsfScalar as ScalarCoreOps>::to_decimal_u8_digits(&from_string);
                assert_eq!(digits_after, raw_digits, "string roundtrip changed raw digits for seed `{seed}`");

                let scalar_after = <UsfScalar as ScalarCoreOps>::from_decimal_u8_digits(digits_after.0, digits_after.1, digits_after.2, digits_after.3);
                assert_eq!(scalar_after, scalar, "string/raw cycle changed scalar repr for seed `{seed}`");

                scalar = scalar_after;
                raw_digits = digits_after;
                as_string = string_after;
            }
        }
    }

    // #[test]
    fn scalar_core_ops_test() {
        let a = <UsfScalar as ScalarCoreOps>::from_decimal_str("17.3");
        let b = <UsfScalar as ScalarCoreOps>::from_decimal_str("3");
        let b = UsfOrNormalScalar::A(b);
        let sum = a.add(b);
        let expected_sum = <UsfScalar as ScalarCoreOps>::from_decimal_str("20.3");

        assert_eq!(sum, expected_sum);
    }
}
