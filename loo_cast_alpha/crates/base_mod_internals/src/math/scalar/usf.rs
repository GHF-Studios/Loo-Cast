pub use super::aliases::{UsfOrNormalFractionalScalar, UsfOrNormalScalar};
use super::shared::{
    FloatType, IntegerType, ScalarCoreOps, ScalarDecimalU8Parts, ScalarFracDigitBuffer, ScalarIntDigitBuffer, ScalarType, SignedIntegerType,
    UnsignedIntegerType,
};
use crate::math::scalar::digits::ScalarDecimalDigits;
pub use crate::math::scalar::digits::{DecimalParseError, ScalarParseError, ScientificParseError};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct UsfScalar {
    pub digits: ScalarDecimalDigits,
}

impl UsfScalar {
    /// Fallible decimal parser with operation-specific error type.
    pub fn try_from_decimal_str(s: &str) -> Result<Self, DecimalParseError> {
        ScalarDecimalDigits::try_from_decimal_str(s).map(|digits| Self { digits })
    }

    /// Fallible scientific parser with operation-specific error type.
    pub fn try_from_scientific_str(s: &str) -> Result<Self, ScientificParseError> {
        ScalarDecimalDigits::try_from_scientific_str(s).map(|digits| Self { digits })
    }

    /// Fallible decimal parser returning a unified parse-error envelope.
    pub fn try_from_decimal_str_scalar_error(s: &str) -> Result<Self, ScalarParseError> {
        Self::try_from_decimal_str(s).map_err(ScalarParseError::from)
    }

    /// Fallible scientific parser returning a unified parse-error envelope.
    pub fn try_from_scientific_str_scalar_error(s: &str) -> Result<Self, ScalarParseError> {
        Self::try_from_scientific_str(s).map_err(ScalarParseError::from)
    }

    /// Formats with internal precision (public fractional digits + shadow layers).
    pub fn to_decimal_str_internal(&self) -> String {
        self.digits.to_decimal_string_internal()
    }

    /// Quantizes in-place to the given layer index.
    pub fn quantize_to_layer_checked(&mut self, radix_position: i8) {
        self.digits.quantize_to_layer_checked(radix_position);
    }

    /// Returns a quantized copy at the given layer index.
    pub fn quantized_to_layer_checked(mut self, radix_position: i8) -> Self {
        self.quantize_to_layer_checked(radix_position);
        self
    }

    /// Coarsest allowed layer index.
    pub const fn radix_position_min() -> i8 {
        ScalarDecimalDigits::radix_position_min()
    }

    /// Finest allowed layer index.
    pub const fn radix_position_max() -> i8 {
        ScalarDecimalDigits::radix_position_max()
    }

    /// Number of internal shadow fractional layers.
    pub const fn shadow_frac_digits_len() -> usize {
        ScalarDecimalDigits::shadow_frac_digits_len()
    }

    /// Total internal fractional precision (`public + shadow`).
    pub const fn internal_frac_digits_len() -> usize {
        ScalarDecimalDigits::internal_frac_digits_len()
    }
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
    fn from_decimal_str(s: &str) -> Self {
        match Self::try_from_decimal_str(s) {
            Ok(value) => value,
            Err(err) => panic!("{err}"),
        }
    }

    fn to_decimal_str(&self) -> String {
        self.digits.to_decimal_string()
    }

    fn from_scientific_str(s: &str) -> Self {
        match Self::try_from_scientific_str(s) {
            Ok(value) => value,
            Err(err) => panic!("{err}"),
        }
    }

    fn to_scientific_str(&self) -> String {
        self.digits.to_scientific_string()
    }

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
