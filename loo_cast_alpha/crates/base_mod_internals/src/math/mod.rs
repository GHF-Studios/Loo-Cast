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
    use super::scalar::usf::UsfScalar;
    use crate::math::scalar::shared::ScalarCoreOps;

    fn scalar_from_decimal(text: &str) -> UsfScalar {
        <UsfScalar as ScalarCoreOps>::from_decimal_str(text)
    }

    fn additive_law_samples() -> (UsfScalar, UsfScalar, UsfScalar) {
        (
            scalar_from_decimal("17.3"),
            scalar_from_decimal("-3.125"),
            scalar_from_decimal("0.00000000000000000000000000000000001"),
        )
    }

    fn multiplicative_law_samples() -> (UsfScalar, UsfScalar) {
        (scalar_from_decimal("17.3"), scalar_from_decimal("-3.125"))
    }

    mod test_utils {
        use super::{UsfScalar, scalar_from_decimal};
        use crate::math::scalar::rand::RandomDistributionBuilder;
        use rand::rngs::StdRng;
        use rand::{Rng, SeedableRng};

        #[derive(Clone, Copy)]
        pub struct ScalarRandomSpec {
            pub max_int_digits: usize,
            pub max_frac_digits: usize,
            pub allow_negative: bool,
            pub force_positive: bool,
            pub require_non_zero: bool,
        }

        impl ScalarRandomSpec {
            pub const fn mul_div() -> Self {
                Self {
                    max_int_digits: 4,
                    max_frac_digits: 8,
                    allow_negative: true,
                    force_positive: false,
                    require_non_zero: false,
                }
            }

            pub const fn non_zero_divisor() -> Self {
                Self {
                    require_non_zero: true,
                    ..Self::mul_div()
                }
            }

            pub const fn exp_ln_domain() -> Self {
                Self {
                    max_int_digits: 2,
                    max_frac_digits: 12,
                    allow_negative: true,
                    force_positive: false,
                    require_non_zero: false,
                }
            }

            pub const fn positive_log_domain() -> Self {
                Self {
                    max_int_digits: 2,
                    max_frac_digits: 10,
                    allow_negative: false,
                    force_positive: true,
                    require_non_zero: true,
                }
            }

            pub const fn positive_log_base_gt_one() -> Self {
                Self {
                    max_int_digits: 1,
                    max_frac_digits: 10,
                    allow_negative: false,
                    force_positive: true,
                    require_non_zero: true,
                }
            }
        }

        pub struct ScalarRandomGen {
            rng: StdRng,
            spec: ScalarRandomSpec,
        }

        impl ScalarRandomGen {
            pub fn new(seed: u64, spec: ScalarRandomSpec) -> Self {
                Self {
                    rng: StdRng::seed_from_u64(seed),
                    spec,
                }
            }

            fn random_decimal_text(&mut self) -> String {
                let int_len = self.rng.random_range(1_usize..=self.spec.max_int_digits.max(1));
                let frac_len = self.rng.random_range(0_usize..=self.spec.max_frac_digits);

                let mut int_digits = String::with_capacity(int_len);
                for idx in 0..int_len {
                    let digit = if idx == 0 && int_len > 1 {
                        self.rng.random_range(1_u8..10_u8)
                    } else {
                        self.rng.random_range(0_u8..10_u8)
                    };
                    int_digits.push(char::from(b'0' + digit));
                }

                let mut frac_digits = String::with_capacity(frac_len);
                for _ in 0..frac_len {
                    let digit = self.rng.random_range(0_u8..10_u8);
                    frac_digits.push(char::from(b'0' + digit));
                }

                let all_zero = int_digits.bytes().all(|b| b == b'0') && frac_digits.bytes().all(|b| b == b'0');
                if self.spec.require_non_zero && all_zero {
                    if frac_len > 0 {
                        frac_digits.replace_range(frac_len - 1..frac_len, "1");
                    } else {
                        int_digits.replace_range(int_len - 1..int_len, "1");
                    }
                }

                let mut out = String::new();
                let negative = self.spec.allow_negative && !self.spec.force_positive && self.rng.random::<bool>();
                if negative {
                    out.push('-');
                }
                out.push_str(&int_digits);
                if frac_len > 0 {
                    out.push('.');
                    out.push_str(&frac_digits);
                }
                out
            }

            pub fn next_scalar(&mut self) -> UsfScalar {
                loop {
                    let text = self.random_decimal_text();
                    let value = scalar_from_decimal(text.as_str());

                    if self.spec.force_positive && value <= UsfScalar::ZERO {
                        continue;
                    }
                    if self.spec.require_non_zero && value == UsfScalar::ZERO {
                        continue;
                    }
                    return value;
                }
            }
        }

        pub fn random_scalar_pool(seed: u64, count: usize, spec: ScalarRandomSpec) -> Vec<UsfScalar> {
            let mut gen_ = ScalarRandomGen::new(seed, spec);
            (0..count).map(|_| gen_.next_scalar()).collect()
        }

        pub fn random_add_sub_values(seed: u64, count: usize) -> Vec<UsfScalar> {
            let one = scalar_from_decimal("1");
            let neg_one = scalar_from_decimal("-1");

            RandomDistributionBuilder::new()
                .default_max_int_digits(8)
                .default_max_frac_digits(12)
                .component(3, |c| c.positive().integer().greater_than(one.clone()))
                .component(3, |c| c.negative().integer().less_than(neg_one.clone()))
                .component(2, |c| c.positive().fractional().greater_than(UsfScalar::ZERO).less_than(one))
                .component(2, |c| c.negative().fractional().greater_than(neg_one).less_than(UsfScalar::ZERO))
                .build(seed, count)
        }

        pub fn random_mul_values(seed: u64, count: usize) -> Vec<UsfScalar> {
            random_scalar_pool(seed, count, ScalarRandomSpec::mul_div())
        }

        pub fn random_divisors_non_zero(seed: u64, count: usize) -> Vec<UsfScalar> {
            random_scalar_pool(seed, count, ScalarRandomSpec::non_zero_divisor())
        }

        pub fn random_exp_ln_values(seed: u64, count: usize) -> Vec<UsfScalar> {
            random_scalar_pool(seed, count, ScalarRandomSpec::exp_ln_domain())
        }

        pub fn random_positive_log_values(seed: u64, count: usize) -> Vec<UsfScalar> {
            random_scalar_pool(seed, count, ScalarRandomSpec::positive_log_domain())
        }

        pub fn random_positive_log_bases_gt_one(seed: u64, count: usize) -> Vec<UsfScalar> {
            let mut values = random_scalar_pool(seed, count, ScalarRandomSpec::positive_log_base_gt_one());
            let one = scalar_from_decimal("1");
            for value in &mut values {
                if *value <= one {
                    *value = scalar_from_decimal("2");
                }
            }
            values
        }
    }

    mod add_sub_laws {
        use super::{UsfOrNormalScalar, UsfScalar, additive_law_samples};
        use crate::math::scalar::shared::ScalarCoreOps;

        #[test]
        fn usf_scalar_add_identity_test() {
            let (a, _, _) = additive_law_samples();
            let zero = UsfScalar::ZERO;

            let a_plus_zero = a.add(UsfOrNormalScalar::A(zero.clone()));
            let zero_plus_a = zero.add(UsfOrNormalScalar::A(a.clone()));

            assert_eq!(a_plus_zero, a);
            assert_eq!(zero_plus_a, a);
        }

        #[test]
        fn usf_scalar_add_inverse_test() {
            let a = <UsfScalar as ScalarCoreOps>::from_decimal_str("17.3");
            let neg_a = <UsfScalar as ScalarCoreOps>::from_decimal_str("-17.3");
            let zero = UsfScalar::ZERO;

            let a_plus_neg_a = a.add(UsfOrNormalScalar::A(neg_a.clone()));
            let neg_a_plus_a = neg_a.add(UsfOrNormalScalar::A(a.clone()));

            assert_eq!(a_plus_neg_a, zero);
            assert_eq!(neg_a_plus_a, zero);
        }

        #[test]
        fn usf_scalar_add_associative_test() {
            let (a, b, c) = additive_law_samples();

            let lhs = a.clone().add(UsfOrNormalScalar::A(b.clone())).add(UsfOrNormalScalar::A(c.clone()));
            let rhs = a.add(UsfOrNormalScalar::A(b.add(UsfOrNormalScalar::A(c))));

            assert_eq!(lhs, rhs);
        }

        #[test]
        fn usf_scalar_add_commutative_test() {
            let (a, b, _) = additive_law_samples();

            let a_plus_b = a.add(UsfOrNormalScalar::A(b.clone()));
            let b_plus_a = b.add(UsfOrNormalScalar::A(a.clone()));

            assert_eq!(a_plus_b, b_plus_a);
        }

        #[test]
        fn usf_scalar_sub_identity_test() {
            let (a, _, _) = additive_law_samples();
            let zero = UsfScalar::ZERO;

            let a_minus_zero = a.sub(UsfOrNormalScalar::A(zero));
            assert_eq!(a_minus_zero, a);
        }

        #[test]
        fn usf_scalar_sub_self_zero_test() {
            let (a, _, _) = additive_law_samples();
            let zero = UsfScalar::ZERO;

            let a_minus_a = a.sub(UsfOrNormalScalar::A(a.clone()));
            assert_eq!(a_minus_a, zero);
        }
    }

    mod mul_div_laws {
        use super::{UsfOrNormalScalar, UsfScalar, multiplicative_law_samples};
        use crate::math::scalar::shared::ScalarCoreOps;

        #[test]
        #[ignore = "ScalarCoreOps::mul is still todo!()"]
        fn usf_scalar_mul_commutative_test() {
            let (a, b) = multiplicative_law_samples();

            let a_mul_b = a.mul(UsfOrNormalScalar::A(b.clone()));
            let b_mul_a = b.mul(UsfOrNormalScalar::A(a.clone()));

            assert_eq!(a_mul_b, b_mul_a);
        }

        #[test]
        #[ignore = "ScalarCoreOps::div is still todo!()"]
        fn usf_scalar_div_identity_test() {
            let (a, _) = multiplicative_law_samples();
            let one = UsfScalar::ONE;

            let a_div_one = a.div(UsfOrNormalScalar::A(one));
            assert_eq!(a_div_one, a);
        }

        #[test]
        #[ignore = "ScalarCoreOps::div is still todo!()"]
        fn usf_scalar_div_self_one_test() {
            let (a, _) = multiplicative_law_samples();
            let one = UsfScalar::ONE;

            let a_div_a = a.div(UsfOrNormalScalar::A(a.clone()));
            assert_eq!(a_div_a, one);
        }
    }

    mod exp_log_laws {
        use super::{UsfOrNormalScalar, scalar_from_decimal};

        #[test]
        #[ignore = "exp/log ops are not implemented yet"]
        fn usf_scalar_exp_ln_inverse_test() {
            let x = scalar_from_decimal("2.5");
            let result = x.exp().ln();
            assert_eq!(result, x);
        }

        #[test]
        #[ignore = "exp/log ops are not implemented yet"]
        fn usf_scalar_pow_log_inverse_test() {
            let base = scalar_from_decimal("2");
            let exponent = scalar_from_decimal("3");
            let value = base.pow(UsfOrNormalScalar::A(exponent.clone()));
            let recovered = value.log(UsfOrNormalScalar::A(base));
            assert_eq!(recovered, exponent);
        }

        #[test]
        #[ignore = "exp/log ops are not implemented yet"]
        fn usf_scalar_log10_alias_test() {
            let x = scalar_from_decimal("1000");
            assert_eq!(x.log10(), x.log_10());
        }
    }

    mod stress_tests {
        use super::UsfOrNormalScalar;
        use super::test_utils::{
            random_add_sub_values, random_divisors_non_zero, random_exp_ln_values, random_mul_values, random_positive_log_bases_gt_one,
            random_positive_log_values,
        };
        use crate::math::scalar::shared::ScalarCoreOps;

        #[test]
        fn usf_scalar_add_randomized_stress_test() {
            let values = random_add_sub_values(0xADDD_0001, 24);
            for a in &values {
                for b in &values {
                    let lhs = a.clone().add(UsfOrNormalScalar::A(b.clone()));
                    let rhs = b.clone().add(UsfOrNormalScalar::A(a.clone()));
                    assert_eq!(lhs, rhs, "add commutativity failed for a={a}, b={b}");
                }
            }
        }

        #[test]
        fn usf_scalar_sub_randomized_stress_test() {
            let values = random_add_sub_values(0xADDD_0002, 24);
            for a in &values {
                for b in &values {
                    let recovered = a.clone().sub(UsfOrNormalScalar::A(b.clone())).add(UsfOrNormalScalar::A(b.clone()));
                    assert_eq!(recovered, a.clone(), "sub cancellation failed for a={a}, b={b}");
                }
            }
        }

        #[test]
        #[ignore = "ScalarCoreOps::mul is still todo!()"]
        fn usf_scalar_mul_randomized_stress_test() {
            let values = random_mul_values(0xBEEF_0001, 24);
            for a in &values {
                for b in &values {
                    let lhs = a.clone().mul(UsfOrNormalScalar::A(b.clone()));
                    let rhs = b.clone().mul(UsfOrNormalScalar::A(a.clone()));
                    assert_eq!(lhs, rhs, "mul commutativity failed for a={a}, b={b}");
                }
            }
        }

        #[test]
        #[ignore = "ScalarCoreOps::div is still todo!()"]
        fn usf_scalar_div_randomized_stress_test() {
            let numerators = random_mul_values(0xBEEF_0002, 24);
            let divisors = random_divisors_non_zero(0xBEEF_0003, 24);
            for a in &numerators {
                for b in &divisors {
                    let recovered = a.clone().div(UsfOrNormalScalar::A(b.clone())).mul(UsfOrNormalScalar::A(b.clone()));
                    assert_eq!(recovered, a.clone(), "div/mul recovery failed for a={a}, b={b}");
                }
            }
        }

        #[test]
        #[ignore = "exp/log ops are not implemented yet"]
        fn usf_scalar_exp_ln_randomized_stress_test() {
            let values = random_exp_ln_values(0xE001, 48);
            for x in values {
                let recovered = x.exp().ln();
                assert_eq!(recovered, x, "exp/ln roundtrip failed for x={x}");
            }
        }

        #[test]
        #[ignore = "exp/log/pow ops are not implemented yet"]
        fn usf_scalar_pow_log_randomized_stress_test() {
            let exponents = random_exp_ln_values(0xE002, 48);
            let bases = random_positive_log_bases_gt_one(0xE003, 48);
            for (base, exponent) in bases.into_iter().zip(exponents.into_iter()) {
                let value = base.clone().pow(UsfOrNormalScalar::A(exponent.clone()));
                let recovered = value.log(UsfOrNormalScalar::A(base.clone()));
                assert_eq!(recovered, exponent, "pow/log roundtrip failed for base={base}, exponent={exponent}");
            }
        }

        #[test]
        #[ignore = "log ops are not implemented yet"]
        fn usf_scalar_log_base_consistency_randomized_stress_test() {
            let values = random_positive_log_values(0xE004, 48);
            let two = super::scalar_from_decimal("2");
            let ten = super::scalar_from_decimal("10");

            for x in values {
                assert_eq!(x.log2(), x.log(UsfOrNormalScalar::A(two.clone())), "log2 mismatch for x={x}");
                assert_eq!(x.log10(), x.log(UsfOrNormalScalar::A(ten.clone())), "log10 mismatch for x={x}");
                assert_eq!(x.log10(), x.log_10(), "log10 alias mismatch for x={x}");
            }
        }
    }

    mod misc_invariants {
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
