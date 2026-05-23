pub use super::aliases::{UsfOrNormalFractionalScalar, UsfOrNormalScalar};
use super::shared::{
    FloatType, IntegerType, ScalarCoreOps, ScalarDecimalU8Parts, ScalarFracDigitBuffer, ScalarIntDigitBuffer, ScalarType, SignedIntegerType,
    UnsignedIntegerType,
};
use crate::math::scalar::digits::ScalarDecimalDigits;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct UsfScalar {
    pub digits: ScalarDecimalDigits,
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
    fn from_decimal_u8_digits(negative: bool, int_digits: ScalarIntDigitBuffer, frac_digits: ScalarFracDigitBuffer, radix_index: i8) -> Self {
        let parts = ScalarDecimalU8Parts::new_checked(negative, int_digits, frac_digits, radix_index);
        let digits = ScalarDecimalDigits::from_decimal_u8_parts_checked(parts.negative(), *parts.int_digits(), *parts.frac_digits());
        Self { digits }
    }

    fn to_decimal_u8_digits(&self) -> (bool, ScalarIntDigitBuffer, ScalarFracDigitBuffer, i8) {
        self.digits.assert_invariants();
        let parts = self.digits.to_decimal_u8_parts();
        (parts.negative(), *parts.int_digits(), *parts.frac_digits(), parts.radix_index())
    }
}
impl super::shared::ScalarFieldOps for UsfScalar {}
impl super::shared::ScalarBridgeOps for UsfScalar {}
impl super::shared::FractionalScalarContract for UsfScalar {}
