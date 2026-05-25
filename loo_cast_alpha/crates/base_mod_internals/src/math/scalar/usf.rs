pub use super::aliases::{UsfOrNormalFractionalScalar, UsfOrNormalScalar};
use super::shared::{
    FloatType, IntegerType, SCALAR_FRAC_DIGITS_LEN, SCALAR_INT_DIGITS_LEN, ScalarCoreOps, ScalarDecimalU8Parts, ScalarFracDigitBuffer, ScalarIntDigitBuffer,
    ScalarType, SignedIntegerType, UnsignedIntegerType,
};
use crate::math::scalar::digits::ScalarDecimalDigits;
pub use crate::math::scalar::digits::{DecimalParseError, ScalarParseError, ScientificParseError};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct UsfScalar {
    /// Canonical balanced-digit carrier for this scalar value.
    pub digits: ScalarDecimalDigits,
}

impl UsfScalar {
    fn from_raw_parts(negative: bool, int_digits: ScalarIntDigitBuffer, frac_digits: ScalarFracDigitBuffer, radix_index: i8) -> Self {
        <Self as ScalarCoreOps>::from_digits(negative, int_digits, frac_digits, radix_index)
    }

    /// Canonical additive identity (`0`) built from raw digit parts.
    pub fn zero() -> Self {
        Self::from_raw_parts(
            false,
            [0; SCALAR_INT_DIGITS_LEN],
            [0; SCALAR_FRAC_DIGITS_LEN],
            ScalarDecimalU8Parts::RADIX_INDEX_MIN,
        )
    }

    /// Canonical multiplicative identity (`1`) built from raw digit parts.
    pub fn one() -> Self {
        let mut int_digits = [0_u8; SCALAR_INT_DIGITS_LEN];
        int_digits[SCALAR_INT_DIGITS_LEN - 1] = 1;
        Self::from_raw_parts(false, int_digits, [0; SCALAR_FRAC_DIGITS_LEN], ScalarDecimalU8Parts::RADIX_INDEX_MIN)
    }

    /// Smallest positive public step
    /// (`0.00000000000000000000000000000000001`, i.e. `10^-35`) built from raw digit parts.
    pub fn epsilon() -> Self {
        let mut frac_digits = [0_u8; SCALAR_FRAC_DIGITS_LEN];
        frac_digits[SCALAR_FRAC_DIGITS_LEN - 1] = 1;
        Self::from_raw_parts(false, [0; SCALAR_INT_DIGITS_LEN], frac_digits, ScalarDecimalU8Parts::RADIX_INDEX_MAX)
    }

    /// Largest currently constructible decimal constant used as practical upper bound in tests:
    /// `399999999999999999999999999999999999.99999999999999999999999999999999999`.
    pub fn max() -> Self {
        let mut int_digits = [9_u8; SCALAR_INT_DIGITS_LEN];
        int_digits[0] = 3;
        Self::from_raw_parts(false, int_digits, [9; SCALAR_FRAC_DIGITS_LEN], ScalarDecimalU8Parts::RADIX_INDEX_MAX)
    }

    /// Smallest currently constructible decimal constant used as practical lower bound in tests:
    /// `-499999999999999999999999999999999999.99999999999999999999999999999999999`.
    pub fn min() -> Self {
        let mut int_digits = [9_u8; SCALAR_INT_DIGITS_LEN];
        int_digits[0] = 4;
        Self::from_raw_parts(true, int_digits, [9; SCALAR_FRAC_DIGITS_LEN], ScalarDecimalU8Parts::RADIX_INDEX_MAX)
    }

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

    /// Raises `self` to `rhs`.
    ///
    /// # Panics
    /// - See [`ScalarCoreOps::pow`] for more information.
    pub fn pow(&self, rhs: UsfOrNormalScalar) -> Self {
        <Self as ScalarCoreOps>::pow(self, rhs)
    }

    /// Computes `e^self`.
    ///
    /// # Panics
    /// - See [`ScalarCoreOps::exp`] for more information.
    pub fn exp(&self) -> Self {
        <Self as ScalarCoreOps>::exp(self)
    }

    /// Computes `2^self`.
    ///
    /// # Panics
    /// - See [`ScalarCoreOps::exp2`] for more information.
    pub fn exp2(&self) -> Self {
        <Self as ScalarCoreOps>::exp2(self)
    }

    /// Computes `10^self`.
    ///
    /// # Panics
    /// - See [`ScalarCoreOps::exp10`] for more information.
    pub fn exp10(&self) -> Self {
        <Self as ScalarCoreOps>::exp10(self)
    }

    /// Computes natural logarithm.
    ///
    /// # Panics
    /// - See [`ScalarCoreOps::ln`] for more information.
    pub fn ln(&self) -> Self {
        <Self as ScalarCoreOps>::ln(self)
    }

    /// Computes base-2 logarithm.
    ///
    /// # Panics
    /// - See [`ScalarCoreOps::log2`] for more information.
    pub fn log2(&self) -> Self {
        <Self as ScalarCoreOps>::log2(self)
    }

    /// Computes base-10 logarithm.
    ///
    /// # Panics
    /// - See [`ScalarCoreOps::log10`] for more information.
    pub fn log10(&self) -> Self {
        <Self as ScalarCoreOps>::log10(self)
    }

    /// Computes logarithm in arbitrary base.
    ///
    /// # Panics
    /// - See [`ScalarCoreOps::log`] for more information.
    pub fn log(&self, base: UsfOrNormalScalar) -> Self {
        <Self as ScalarCoreOps>::log(self, base)
    }

    /// Alias for base-10 logarithm.
    pub fn log_10(&self) -> Self {
        self.log10()
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

    fn zero() -> Self {
        UsfScalar::zero()
    }

    fn one() -> Self {
        UsfScalar::one()
    }

    fn two() -> Self {
        let mut int_digits = [0_u8; SCALAR_INT_DIGITS_LEN];
        int_digits[SCALAR_INT_DIGITS_LEN - 1] = 2;
        Self::from_raw_parts(false, int_digits, [0; SCALAR_FRAC_DIGITS_LEN], ScalarDecimalU8Parts::RADIX_INDEX_MIN)
    }

    fn ten() -> Self {
        let mut int_digits = [0_u8; SCALAR_INT_DIGITS_LEN];
        int_digits[SCALAR_INT_DIGITS_LEN - 2] = 1;
        Self::from_raw_parts(false, int_digits, [0; SCALAR_FRAC_DIGITS_LEN], ScalarDecimalU8Parts::RADIX_INDEX_MIN)
    }

    fn neg_one() -> Self {
        let mut int_digits = [0_u8; SCALAR_INT_DIGITS_LEN];
        int_digits[SCALAR_INT_DIGITS_LEN - 1] = 1;
        Self::from_raw_parts(true, int_digits, [0; SCALAR_FRAC_DIGITS_LEN], ScalarDecimalU8Parts::RADIX_INDEX_MIN)
    }

    fn from_digits(negative: bool, int_digits: ScalarIntDigitBuffer, frac_digits: ScalarFracDigitBuffer, radix_index: i8) -> Self {
        let parts = ScalarDecimalU8Parts::new_checked(negative, int_digits, frac_digits, radix_index);
        let digits = ScalarDecimalDigits::from_decimal_u8_parts_checked(parts.negative(), *parts.int_digits(), *parts.frac_digits());
        Self { digits }
    }

    fn to_digits(&self) -> (bool, ScalarIntDigitBuffer, ScalarFracDigitBuffer, i8) {
        self.digits.assert_invariants();
        let parts = self.digits.to_decimal_u8_parts();
        (parts.negative(), *parts.int_digits(), *parts.frac_digits(), parts.radix_index())
    }
}
impl super::shared::ScalarFieldOps for UsfScalar {}
impl super::shared::ScalarBridgeOps for UsfScalar {}
impl super::shared::FractionalScalarContract for UsfScalar {}
