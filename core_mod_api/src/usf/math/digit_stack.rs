pub const BALANCED_BASE: i32 = 10;
pub const BALANCED_MIN_DIGIT: i8 = -5;
pub const BALANCED_MAX_DIGIT: i8 = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DigitStackOverflow {
    pub carry_out: i32,
}

#[inline]
pub fn wrap_balanced_10(raw: i32) -> (i8, i32) {
    let wrapped = ((raw + 5).rem_euclid(BALANCED_BASE)) - 5;
    let carry = (raw - wrapped).div_euclid(BALANCED_BASE);
    (wrapped as i8, carry)
}

#[inline]
fn normalize_balanced_digits(digits: &mut [i32]) -> i32 {
    let mut carry = 0_i32;
    for digit in digits.iter_mut().rev() {
        let raw = *digit + carry;
        let (wrapped, next_carry) = wrap_balanced_10(raw);
        *digit = wrapped as i32;
        carry = next_carry;
    }
    carry
}

pub fn normalize_balanced_digits_wrap(digits: &mut [i32]) -> i32 {
    normalize_balanced_digits(digits)
}

pub fn normalize_balanced_digits_checked(digits: &mut [i32]) -> Result<(), DigitStackOverflow> {
    let carry = normalize_balanced_digits(digits);
    if carry != 0 {
        return Err(DigitStackOverflow { carry_out: carry });
    }
    Ok(())
}

pub fn normalize_balanced_digits_strict(digits: &mut [i32]) {
    let carry = normalize_balanced_digits(digits);
    assert!(carry == 0, "Digit stack overflow in strict mode: carry_out={carry}");
}

fn combine_digit_slices_in_place(lhs: &mut [i32], rhs: &[i32], sign: i32) {
    assert!(lhs.len() == rhs.len(), "Digit slice length mismatch: lhs={}, rhs={}", lhs.len(), rhs.len());
    for (left, right) in lhs.iter_mut().zip(rhs.iter()) {
        *left += *right * sign;
    }
}

pub fn add_digit_slices_wrap(lhs: &mut [i32], rhs: &[i32]) -> i32 {
    combine_digit_slices_in_place(lhs, rhs, 1);
    normalize_balanced_digits_wrap(lhs)
}

pub fn add_digit_slices_checked(lhs: &mut [i32], rhs: &[i32]) -> Result<(), DigitStackOverflow> {
    combine_digit_slices_in_place(lhs, rhs, 1);
    normalize_balanced_digits_checked(lhs)
}

pub fn add_digit_slices_strict(lhs: &mut [i32], rhs: &[i32]) {
    combine_digit_slices_in_place(lhs, rhs, 1);
    normalize_balanced_digits_strict(lhs);
}

pub fn sub_digit_slices_wrap(lhs: &mut [i32], rhs: &[i32]) -> i32 {
    combine_digit_slices_in_place(lhs, rhs, -1);
    normalize_balanced_digits_wrap(lhs)
}

pub fn sub_digit_slices_checked(lhs: &mut [i32], rhs: &[i32]) -> Result<(), DigitStackOverflow> {
    combine_digit_slices_in_place(lhs, rhs, -1);
    normalize_balanced_digits_checked(lhs)
}

pub fn sub_digit_slices_strict(lhs: &mut [i32], rhs: &[i32]) {
    combine_digit_slices_in_place(lhs, rhs, -1);
    normalize_balanced_digits_strict(lhs);
}
