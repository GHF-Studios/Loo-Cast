use super::super::field::Field;
pub use super::aliases::{UsfOrNormalFractionalScalar, UsfOrNormalScalar};
use super::shared::{FloatType, IntegerType, ScalarCoreOps, ScalarType, SignedIntegerType, UnsignedIntegerType};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct UsfDigit {
    digit: i8,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct UsfScalar {
    pub digits: Field<Vec<UsfDigit>>,
    pub radix_position: Field<i64>,
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

impl super::shared::ScalarCoreOps for UsfScalar {
    fn from_decimal_u8_digits(negative: bool, int_digits: Vec<u8>, frac_digits: Vec<u8>) -> Self {
        assert_eq!(int_digits.len(), 36, "int_digits must be len 36");
        assert!(frac_digits.len() <= 35, "frac_digits must be <= 35");
        assert!(
            int_digits.iter().chain(frac_digits.iter()).all(|d| *d <= 9),
            "all decimal digits must be in 0..=9"
        );

        let mut out_rev: Vec<UsfDigit> = Vec::with_capacity(int_digits.len() + frac_digits.len() + 1);
        let mut carry: i16 = 0;

        for d in int_digits.into_iter().chain(frac_digits).rev() {
            let v = (if negative { -(d as i16) } else { d as i16 }) + carry;
            let bal = ((v + 5).rem_euclid(10)) - 5; // range: -5..5
            carry = (v - bal).div_euclid(10);
            out_rev.push(UsfDigit { digit: bal as i8 });
        }

        while carry != 0 {
            let bal = ((carry + 5).rem_euclid(10)) - 5; // range: -5..5
            carry = (carry - bal).div_euclid(10);
            out_rev.push(UsfDigit { digit: bal as i8 });
        }

        out_rev.reverse();
        while out_rev.len() > 1 && out_rev.last().is_some_and(|d| d.digit == 0) {
            out_rev.pop();
        }
        let radix_position = out_rev.len().checked_sub(1).expect("at least one digit") as i64;
        assert!((0..=70).contains(&radix_position), "radix_position out of range");

        Self { digits: Field::new(out_rev), radix_position: Field::new(radix_position) }
    }

    fn to_decimal_u8_digits(&self) -> (bool, Vec<u8>, Vec<u8>) {
        const INT_DIGITS: usize = 36;
        const FRAC_DIGITS: usize = 35;
        const TOTAL_DIGITS: usize = INT_DIGITS + FRAC_DIGITS;

        let digits = self.digits.get();
        let radix_position = self.radix_position.get();

        assert!((0..=70).contains(&radix_position), "radix_position out of range");
        assert_eq!(digits.len(), (radix_position as usize) + 1, "digits/radix_position mismatch");

        let mut balanced: Vec<i16> = vec![0; TOTAL_DIGITS];
        for (idx, d) in digits.iter().enumerate() {
            assert!((-5..=5).contains(&d.digit), "usf digit out of balanced range");
            balanced[idx] = d.digit as i16;
        }

        let first_nonzero = balanced.iter().position(|d| *d != 0);
        if first_nonzero.is_none() {
            return (false, vec![0; INT_DIGITS], vec![0; FRAC_DIGITS]);
        }
        let negative = balanced[first_nonzero.unwrap()] < 0;

        if negative {
            for d in &mut balanced {
                *d = -*d;
            }
        }

        let mut carry: i16 = 0;
        let mut decimal_rev: Vec<u8> = Vec::with_capacity(TOTAL_DIGITS);
        for d in balanced.into_iter().rev() {
            let v = d + carry;
            let digit = v.rem_euclid(10) as u8;
            carry = v.div_euclid(10);
            decimal_rev.push(digit);
        }
        assert_eq!(carry, 0, "failed to convert balanced digits into canonical decimal digits");

        decimal_rev.reverse();
        let int_digits = decimal_rev[..INT_DIGITS].to_vec();
        let frac_digits = decimal_rev[INT_DIGITS..].to_vec();
        (negative, int_digits, frac_digits)
    }
}
impl super::shared::ScalarFieldOps for UsfScalar {}
impl super::shared::ScalarBridgeOps for UsfScalar {}
impl super::shared::FractionalScalarContract for UsfScalar {}
