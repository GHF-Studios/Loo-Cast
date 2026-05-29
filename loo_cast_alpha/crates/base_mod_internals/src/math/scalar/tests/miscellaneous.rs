use crate::math::scalar::usf::UsfScalar;
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

// This test is weak af
#[test]
fn usf_scalar_debug_fmt_test() {
    let number = UsfScalar::ONE;
    let result = format!("{:?}", number);
    let expected = "UsfScalar { digits: ScalarDecimalDigits[ 000000000000000000000000000000000001.00000000000000000000000000000000000 ] }";
    assert_eq!(result, expected);
}

// This test is weak af
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