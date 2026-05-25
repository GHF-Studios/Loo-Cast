pub use super::aliases::{UsfOrNormalFractionalScalar, UsfOrNormalScalar};
use super::shared::{
    FloatType, IntegerType, PublicSignedMagnitude, ScalarConstOps, ScalarCoreOps, ScalarDecimalU8Parts, ScalarFracDigitBuffer, ScalarIntDigitBuffer,
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
    /// Canonical additive identity (`0`) built from fixed-width balanced digits.
    pub const ZERO: Self = Self {
        digits: ScalarDecimalDigits::ZERO,
    };

    /// Canonical multiplicative identity (`1`) built from fixed-width balanced digits.
    pub const ONE: Self = Self {
        digits: ScalarDecimalDigits::ONE,
    };

    /// Canonical negative-one constant (`-1`) built from fixed-width balanced digits.
    pub const NEG_ONE: Self = Self {
        digits: ScalarDecimalDigits::NEG_ONE,
    };

    /// Canonical two constant (`2`) built from fixed-width balanced digits.
    pub const TWO: Self = Self {
        digits: ScalarDecimalDigits::TWO,
    };

    /// Canonical ten constant (`10`) built from fixed-width balanced digits.
    pub const TEN: Self = Self {
        digits: ScalarDecimalDigits::TEN,
    };

    /// Canonical smallest positive finite step (`10^-35`).
    ///
    /// Decimal text: `0.00000000000000000000000000000000001`.
    pub const EPSILON: Self = Self {
        digits: ScalarDecimalDigits::EPSILON,
    };

    /// Canonical pi constant.
    ///
    /// Decimal text: `3.14159265358979323846264338327950288`.
    pub const PI: Self = Self {
        digits: ScalarDecimalDigits::PI,
    };

    /// Canonical tau constant.
    ///
    /// Decimal text: `6.28318530717958647692528676655900577`.
    pub const TAU: Self = Self {
        digits: ScalarDecimalDigits::TAU,
    };

    /// Canonical Euler's number constant.
    ///
    /// Decimal text: `2.71828182845904523536028747135266250`.
    pub const E: Self = Self {
        digits: ScalarDecimalDigits::E,
    };

    /// Canonical maximum finite scalar constant for this backend.
    ///
    /// Decimal text: `399999999999999999999999999999999999.99999999999999999999999999999999999`.
    pub const MAX: Self = Self {
        digits: ScalarDecimalDigits::MAX,
    };

    /// Canonical minimum finite scalar constant for this backend.
    ///
    /// Decimal text: `-499999999999999999999999999999999999.99999999999999999999999999999999999`.
    pub const MIN: Self = Self {
        digits: ScalarDecimalDigits::MIN,
    };

    /// Converts mixed-repr scalar operand into signed-magnitude view.
    fn rhs_to_signed_magnitude(rhs: UsfOrNormalScalar) -> PublicSignedMagnitude {
        let (negative, int_digits, frac_digits, _radix) = match rhs {
            UsfOrNormalScalar::A(value) => value.to_digits(),
            UsfOrNormalScalar::B(value) => value.to_digits(),
        };
        ScalarDecimalDigits::public_signed_magnitude_from_u8_parts(negative, int_digits, frac_digits)
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

impl ScalarConstOps for UsfScalar {
    fn zero() -> Self {
        UsfScalar::ZERO
    }

    fn one() -> Self {
        UsfScalar::ONE
    }

    fn two() -> Self {
        UsfScalar::TWO
    }

    fn ten() -> Self {
        UsfScalar::TEN
    }

    fn max() -> Self {
        UsfScalar::MAX
    }

    fn min() -> Self {
        UsfScalar::MIN
    }

    fn neg_one() -> Self {
        UsfScalar::NEG_ONE
    }

    fn epsilon() -> Self {
        UsfScalar::EPSILON
    }

    fn pi() -> Self {
        UsfScalar::PI
    }

    fn tau() -> Self {
        UsfScalar::TAU
    }

    fn e() -> Self {
        UsfScalar::E
    }
}

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

    fn exp(&self) -> Self {
        todo!()
    }

    fn exp2(&self) -> Self {
        todo!()
    }

    fn exp10(&self) -> Self {
        todo!()
    }

    fn ln(&self) -> Self {
        todo!()
    }

    fn log2(&self) -> Self {
        todo!()
    }

    fn log10(&self) -> Self {
        todo!()
    }

    fn log(&self, _base: UsfOrNormalScalar) -> Self {
        todo!()
    }

    fn add(&self, rhs: UsfOrNormalScalar) -> Self {
        let lhs = self.digits.to_public_signed_magnitude();
        let rhs = Self::rhs_to_signed_magnitude(rhs);
        Self {
            digits: ScalarDecimalDigits::from_public_signed_magnitude(PublicSignedMagnitude::combine_add(lhs, rhs)),
        }
    }

    fn sub(&self, rhs: UsfOrNormalScalar) -> Self {
        let lhs = self.digits.to_public_signed_magnitude();
        let rhs = Self::rhs_to_signed_magnitude(rhs);
        Self {
            digits: ScalarDecimalDigits::from_public_signed_magnitude(PublicSignedMagnitude::combine_sub(lhs, rhs)),
        }
    }

    fn mul(&self, rhs: UsfOrNormalScalar) -> Self {
        let lhs = self.digits.to_public_signed_magnitude();
        let rhs = Self::rhs_to_signed_magnitude(rhs);
        Self {
            digits: ScalarDecimalDigits::from_public_signed_magnitude(PublicSignedMagnitude::combine_mul(lhs, rhs)),
        }
    }

    fn div(&self, rhs: UsfOrNormalScalar) -> Self {
        let lhs = self.digits.to_public_signed_magnitude();
        let rhs = Self::rhs_to_signed_magnitude(rhs);
        Self {
            digits: ScalarDecimalDigits::from_public_signed_magnitude(PublicSignedMagnitude::combine_div(lhs, rhs)),
        }
    }

    fn rem(&self, rhs: UsfOrNormalScalar) -> Self {
        let lhs = self.digits.to_public_signed_magnitude();
        let rhs = Self::rhs_to_signed_magnitude(rhs);
        Self {
            digits: ScalarDecimalDigits::from_public_signed_magnitude(PublicSignedMagnitude::combine_rem(lhs, rhs)),
        }
    }

    fn pow(&self, _rhs: UsfOrNormalScalar) -> Self {
        todo!()
    }
}
impl super::shared::ScalarFieldOps for UsfScalar {}
impl super::shared::ScalarBridgeOps for UsfScalar {}
impl super::shared::FractionalScalarContract for UsfScalar {}
