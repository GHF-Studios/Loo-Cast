use std::ops::{Add, AddAssign, Sub, SubAssign};

use crate::usf::math::digit_stack::{
    DigitStackOverflow, add_digit_slices_checked, add_digit_slices_strict, add_digit_slices_wrap, normalize_balanced_digits_checked,
    normalize_balanced_digits_strict, normalize_balanced_digits_wrap, sub_digit_slices_checked, sub_digit_slices_strict, sub_digit_slices_wrap,
};
use crate::usf::scale::{DynScale, Scale};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum USFScalarError {
    Overflow(DigitStackOverflow),
    NonFiniteInput,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct USFScalar {
    digits: [i8; Self::DIGIT_COUNT],
}
impl USFScalar {
    pub const DIGIT_COUNT: usize = Scale::SCALE_LEVEL_COUNT as usize;
    pub const ZERO: Self = Self {
        digits: [0; Self::DIGIT_COUNT],
    };

    pub fn zero() -> Self {
        Self::ZERO
    }

    pub fn is_zero(&self) -> bool {
        self.digits.iter().all(|digit| *digit == 0)
    }

    pub fn digit_at(&self, scale: Scale) -> i8 {
        self.digits[scale.index_from_top() as usize]
    }

    pub fn digits(&self) -> [i8; Self::DIGIT_COUNT] {
        self.digits
    }

    pub fn from_digits_wrap(mut digits: [i32; Self::DIGIT_COUNT]) -> Self {
        let _ = normalize_balanced_digits_wrap(&mut digits);
        Self::from_i32_digits(digits)
    }

    pub fn from_digits_checked(mut digits: [i32; Self::DIGIT_COUNT]) -> Result<Self, USFScalarError> {
        normalize_balanced_digits_checked(&mut digits).map_err(USFScalarError::Overflow)?;
        Ok(Self::from_i32_digits(digits))
    }

    pub fn from_digits_strict(mut digits: [i32; Self::DIGIT_COUNT]) -> Self {
        normalize_balanced_digits_strict(&mut digits);
        Self::from_i32_digits(digits)
    }

    pub fn from_i32(value: i32) -> Self {
        Self::from_i64(value as i64)
    }

    pub fn from_i64(value: i64) -> Self {
        Self::from_i128_at_scale_wrap(value as i128, Scale::MID)
    }

    pub fn from_window_f64_wrap(origin: Self, value: f64, active_scale: Scale, precision_scale: Scale) -> Result<Self, USFScalarError> {
        if !value.is_finite() {
            return Err(USFScalarError::NonFiniteInput);
        }

        let active_exp = active_scale.scale_factor_exponent() as i32;
        let precision_exp = precision_scale.scale_factor_exponent() as i32;
        let shifted = value * 10.0_f64.powi(active_exp - precision_exp);
        if !shifted.is_finite() {
            return Err(USFScalarError::NonFiniteInput);
        }

        let steps = shifted.round();
        if steps > i128::MAX as f64 || steps < i128::MIN as f64 {
            return Err(USFScalarError::Overflow(DigitStackOverflow { carry_out: 1 }));
        }

        let delta = Self::from_i128_at_scale_wrap(steps as i128, precision_scale);
        Ok(origin + delta)
    }

    pub fn from_window_f64_checked(origin: Self, value: f64, active_scale: Scale, precision_scale: Scale) -> Result<Self, USFScalarError> {
        if !value.is_finite() {
            return Err(USFScalarError::NonFiniteInput);
        }

        let active_exp = active_scale.scale_factor_exponent() as i32;
        let precision_exp = precision_scale.scale_factor_exponent() as i32;
        let shifted = value * 10.0_f64.powi(active_exp - precision_exp);
        if !shifted.is_finite() {
            return Err(USFScalarError::NonFiniteInput);
        }

        let steps = shifted.round();
        if steps > i128::MAX as f64 || steps < i128::MIN as f64 {
            return Err(USFScalarError::Overflow(DigitStackOverflow { carry_out: 1 }));
        }

        let delta = Self::from_i128_at_scale_checked(steps as i128, precision_scale)?;
        origin.add_checked(delta)
    }

    pub fn from_window_f64_strict(origin: Self, value: f64, active_scale: Scale, precision_scale: Scale) -> Self {
        Self::from_window_f64_checked(origin, value, active_scale, precision_scale).expect("USFScalar strict window conversion failed")
    }

    pub fn to_window_f64(self, origin: Self, active_scale: Scale) -> f64 {
        let delta = self - origin;
        let active_exp = active_scale.scale_factor_exponent() as i32;

        delta
            .digits
            .iter()
            .enumerate()
            .filter(|(_, digit)| **digit != 0)
            .fold(0.0_f64, |acc, (idx, digit)| {
                let exp = 35_i32 - idx as i32;
                acc + (*digit as f64 * 10.0_f64.powi(exp - active_exp))
            })
    }

    pub fn normalize_wrap(&mut self) -> i32 {
        let mut raw = self.to_i32_digits();
        let carry = normalize_balanced_digits_wrap(&mut raw);
        *self = Self::from_i32_digits(raw);
        carry
    }

    pub fn normalize_checked(&mut self) -> Result<(), USFScalarError> {
        let mut raw = self.to_i32_digits();
        normalize_balanced_digits_checked(&mut raw).map_err(USFScalarError::Overflow)?;
        *self = Self::from_i32_digits(raw);
        Ok(())
    }

    pub fn normalize_strict(&mut self) {
        let mut raw = self.to_i32_digits();
        normalize_balanced_digits_strict(&mut raw);
        *self = Self::from_i32_digits(raw);
    }

    pub fn add_wrap(self, rhs: Self) -> Self {
        let mut raw = self.to_i32_digits();
        let rhs_raw = rhs.to_i32_digits();
        let _ = add_digit_slices_wrap(&mut raw, &rhs_raw);
        Self::from_i32_digits(raw)
    }

    pub fn add_checked(self, rhs: Self) -> Result<Self, USFScalarError> {
        let mut raw = self.to_i32_digits();
        let rhs_raw = rhs.to_i32_digits();
        add_digit_slices_checked(&mut raw, &rhs_raw).map_err(USFScalarError::Overflow)?;
        Ok(Self::from_i32_digits(raw))
    }

    pub fn add_strict(self, rhs: Self) -> Self {
        let mut raw = self.to_i32_digits();
        let rhs_raw = rhs.to_i32_digits();
        add_digit_slices_strict(&mut raw, &rhs_raw);
        Self::from_i32_digits(raw)
    }

    pub fn sub_wrap(self, rhs: Self) -> Self {
        let mut raw = self.to_i32_digits();
        let rhs_raw = rhs.to_i32_digits();
        let _ = sub_digit_slices_wrap(&mut raw, &rhs_raw);
        Self::from_i32_digits(raw)
    }

    pub fn sub_checked(self, rhs: Self) -> Result<Self, USFScalarError> {
        let mut raw = self.to_i32_digits();
        let rhs_raw = rhs.to_i32_digits();
        sub_digit_slices_checked(&mut raw, &rhs_raw).map_err(USFScalarError::Overflow)?;
        Ok(Self::from_i32_digits(raw))
    }

    pub fn sub_strict(self, rhs: Self) -> Self {
        let mut raw = self.to_i32_digits();
        let rhs_raw = rhs.to_i32_digits();
        sub_digit_slices_strict(&mut raw, &rhs_raw);
        Self::from_i32_digits(raw)
    }

    pub fn mul_i32_wrap(self, rhs: i32) -> Self {
        let mut raw = self.to_i32_digits();
        for digit in &mut raw {
            *digit *= rhs;
        }
        let _ = normalize_balanced_digits_wrap(&mut raw);
        Self::from_i32_digits(raw)
    }

    pub fn mul_i32_checked(self, rhs: i32) -> Result<Self, USFScalarError> {
        let mut raw = self.to_i32_digits();
        for digit in &mut raw {
            *digit *= rhs;
        }
        normalize_balanced_digits_checked(&mut raw).map_err(USFScalarError::Overflow)?;
        Ok(Self::from_i32_digits(raw))
    }

    pub fn mul_i32_strict(self, rhs: i32) -> Self {
        let mut raw = self.to_i32_digits();
        for digit in &mut raw {
            *digit *= rhs;
        }
        normalize_balanced_digits_strict(&mut raw);
        Self::from_i32_digits(raw)
    }

    fn from_i128_at_scale_wrap(mut value: i128, scale: Scale) -> Self {
        let mut digits = [0_i8; Self::DIGIT_COUNT];
        let mut idx = scale.index_from_top() as isize;

        while value != 0 && idx >= 0 {
            let wrapped = ((value + 5).rem_euclid(10)) - 5;
            value = (value - wrapped).div_euclid(10);
            digits[idx as usize] = wrapped as i8;
            idx -= 1;
        }

        Self { digits }
    }

    fn from_i128_at_scale_checked(mut value: i128, scale: Scale) -> Result<Self, USFScalarError> {
        let mut digits = [0_i8; Self::DIGIT_COUNT];
        let mut idx = scale.index_from_top() as isize;

        while value != 0 && idx >= 0 {
            let wrapped = ((value + 5).rem_euclid(10)) - 5;
            value = (value - wrapped).div_euclid(10);
            digits[idx as usize] = wrapped as i8;
            idx -= 1;
        }

        if value != 0 {
            return Err(USFScalarError::Overflow(DigitStackOverflow {
                carry_out: value.signum() as i32,
            }));
        }

        Ok(Self { digits })
    }

    fn to_i32_digits(self) -> [i32; Self::DIGIT_COUNT] {
        self.digits.map(|digit| digit as i32)
    }

    fn from_i32_digits(digits: [i32; Self::DIGIT_COUNT]) -> Self {
        let mut out = [0_i8; Self::DIGIT_COUNT];
        for (idx, digit) in digits.into_iter().enumerate() {
            debug_assert!((-5..=4).contains(&digit), "Digit at index {idx} is out of balanced range: {digit}");
            out[idx] = digit as i8;
        }
        Self { digits: out }
    }
}

impl Default for USFScalar {
    fn default() -> Self {
        Self::ZERO
    }
}

impl Add for USFScalar {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.add_wrap(rhs)
    }
}

impl AddAssign for USFScalar {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.add_wrap(rhs);
    }
}

impl Sub for USFScalar {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.sub_wrap(rhs)
    }
}

impl SubAssign for USFScalar {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.sub_wrap(rhs);
    }
}
