use super::digit_stack::{normalize_balanced_digits_wrap, wrap_balanced_10};
use super::scalar::USFScalar;
use crate::usf::scale::Scale;

#[test]
fn wrap_balanced_10_handles_carry_and_borrow() {
    assert_eq!(wrap_balanced_10(5), (-5, 1));
    assert_eq!(wrap_balanced_10(-6), (4, -1));
    assert_eq!(wrap_balanced_10(14), (4, 1));
}

#[test]
fn normalize_balanced_digits_wrap_carries_across_stack() {
    let mut digits = [0_i32, 4, 5];
    let carry = normalize_balanced_digits_wrap(&mut digits);
    assert_eq!(carry, 0);
    assert_eq!(digits, [1, -5, -5]);
}

#[test]
fn normalize_balanced_digits_wrap_borrows_across_stack() {
    let mut digits = [0_i32, -5, -6];
    let carry = normalize_balanced_digits_wrap(&mut digits);
    assert_eq!(carry, 0);
    assert_eq!(digits, [-1, 4, 4]);
}

#[test]
fn scalar_normalize_is_idempotent() {
    let raw = {
        let mut data = [0_i32; USFScalar::DIGIT_COUNT];
        data[35] = 15;
        data[36] = -22;
        data[70] = 49;
        data
    };

    let mut scalar = USFScalar::from_digits_wrap(raw);
    let once = scalar;
    let _ = scalar.normalize_wrap();
    assert_eq!(scalar, once);
}

#[test]
fn scalar_wrap_overflow_discards_top_carry() {
    let max = {
        let mut digits = [0_i32; USFScalar::DIGIT_COUNT];
        digits.fill(4);
        USFScalar::from_digits_wrap(digits)
    };
    let one_at_min = {
        let mut digits = [0_i32; USFScalar::DIGIT_COUNT];
        digits[Scale::MIN.index_from_top() as usize] = 1;
        USFScalar::from_digits_wrap(digits)
    };

    let wrapped = max + one_at_min;
    let wrapped_digits = wrapped.digits();
    assert!(wrapped_digits.iter().all(|digit| *digit == -5));
}
