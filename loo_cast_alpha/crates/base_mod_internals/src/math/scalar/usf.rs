pub use super::aliases::{UsfOrNormalFractionalScalar, UsfOrNormalScalar};
use super::shared::{
    FloatType, IntegerType, SCALAR_FRAC_DIGITS_MAX_LEN, SCALAR_INT_DIGITS_LEN, ScalarCoreOps, ScalarType, SignedIntegerType, UnsignedIntegerType,
};
use crate::math::scalar::digits::{ScalarDecimalDigit, ScalarDecimalDigits};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct UsfScalar {
    pub digits: ScalarDecimalDigits,
    pub radix_position: i64,
}

impl std::fmt::Display for UsfScalar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_decimal_str())
    }
}

impl ScalarType for UsfScalar {}
impl IntegerType for UsfScalar {}
impl SignedIntegerType for UsfScalar {}
impl UnsignedIntegerType for UsfScalar {}
impl FloatType for UsfScalar {}

impl UsfScalar {
    fn validate_decimal_input(int_digits: &[u8], frac_digits: &[u8]) {
        assert_eq!(int_digits.len(), SCALAR_INT_DIGITS_LEN, "int_digits must be len {SCALAR_INT_DIGITS_LEN}", );
        assert!(
            frac_digits.len() <= SCALAR_FRAC_DIGITS_MAX_LEN,
            "frac_digits must be <= {SCALAR_FRAC_DIGITS_MAX_LEN}",
        );
        assert!(
            int_digits.iter().chain(frac_digits.iter()).all(|d| *d <= 9),
            "all digits must be in range 0..=9"
        );
    }

    fn balanced_digit_and_carry(value: i16) -> (ScalarDecimalDigit, i16) {
        let balanced = ((value + 5).rem_euclid(10)) - 5; // range: -5..5 (-5..=4)
        let carry = (value - balanced).div_euclid(10);
        (ScalarDecimalDigit::new_checked(balanced as i8), carry)
    }

    fn encode_balanced_digits(negative: bool, int_digits: Vec<u8>, frac_digits: Vec<u8>) -> Vec<ScalarDecimalDigit> {
        let mut out_rev: Vec<ScalarDecimalDigit> = Vec::with_capacity(int_digits.len() + frac_digits.len() + 1);
        let mut carry: i16 = 0;

        for d in int_digits.into_iter().chain(frac_digits).rev() {
            let signed = if negative { -(d as i16) } else { d as i16 };
            let (balanced_digit, next_carry) = Self::balanced_digit_and_carry(signed + carry);
            carry = next_carry;
            out_rev.push(balanced_digit);
        }

        while carry != 0 {
            let (balanced_digit, next_carry) = Self::balanced_digit_and_carry(carry);
            carry = next_carry;
            out_rev.push(balanced_digit);
        }

        out_rev.reverse();
        Self::trim_balanced_tail_zeros(&mut out_rev);
        out_rev
    }

    fn trim_balanced_tail_zeros(digits: &mut Vec<ScalarDecimalDigit>) {
        while digits.len() > 1 && digits.last().is_some_and(|d| d.get() == 0) {
            digits.pop();
        }
    }

    fn build_scalar_decimal_digits(negative: bool, balanced_digits: Vec<ScalarDecimalDigit>) -> ScalarDecimalDigits {
        let split = balanced_digits.len().min(SCALAR_INT_DIGITS_LEN);
        let (int_digits, frac_digits) = balanced_digits.split_at(split);
        ScalarDecimalDigits::from_variable_parts_checked(negative, int_digits.to_vec(), frac_digits.to_vec())
    }

    fn radix_position_from_digits(digits: &ScalarDecimalDigits) -> i64 {
        let radix_position = digits.len().checked_sub(1).expect("at least one digit") as i64;
        assert!((0..=70).contains(&radix_position), "radix_position out of range");
        radix_position
    }
}

impl super::shared::ScalarCoreOps for UsfScalar {
    fn from_decimal_u8_digits(negative: bool, int_digits: Vec<u8>, frac_digits: Vec<u8>) -> Self {
        Self::validate_decimal_input(&int_digits, &frac_digits);
        let balanced_digits = Self::encode_balanced_digits(negative, int_digits, frac_digits);
        let digits = Self::build_scalar_decimal_digits(negative, balanced_digits);
        let radix_position = Self::radix_position_from_digits(&digits);

        Self { digits, radix_position }
    }

    fn to_decimal_u8_digits(&self) -> (bool, Vec<u8>, Vec<u8>) {
        let radix_position = self.radix_position;
        assert!((0..=70).contains(&radix_position), "radix_position out of range");
        self.digits.clone().into_tuple()
    }
}
impl super::shared::ScalarFieldOps for UsfScalar {}
impl super::shared::ScalarBridgeOps for UsfScalar {}
impl super::shared::FractionalScalarContract for UsfScalar {}
