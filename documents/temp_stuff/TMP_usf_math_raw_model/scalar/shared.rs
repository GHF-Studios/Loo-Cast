#![allow(dead_code)]

use super::aliases::{UsfOrNormalDecimalScalar, UsfOrNormalScalar};
use crate::utils::one_of::OneOf2;

pub trait ScalarType: Clone + 'static {}
pub trait IntegerType: ScalarType {}
pub trait SignedIntegerType: IntegerType {}
pub trait UnsignedIntegerType: IntegerType {}
pub trait FloatType: ScalarType {}

impl ScalarType for i8 {}
impl IntegerType for i8 {}
impl SignedIntegerType for i8 {}

impl ScalarType for i16 {}
impl IntegerType for i16 {}
impl SignedIntegerType for i16 {}

impl ScalarType for i32 {}
impl IntegerType for i32 {}
impl SignedIntegerType for i32 {}

impl ScalarType for i64 {}
impl IntegerType for i64 {}
impl SignedIntegerType for i64 {}

impl ScalarType for i128 {}
impl IntegerType for i128 {}
impl SignedIntegerType for i128 {}

impl ScalarType for isize {}
impl IntegerType for isize {}
impl SignedIntegerType for isize {}

impl ScalarType for u8 {}
impl IntegerType for u8 {}
impl UnsignedIntegerType for u8 {}

impl ScalarType for u16 {}
impl IntegerType for u16 {}
impl UnsignedIntegerType for u16 {}

impl ScalarType for u32 {}
impl IntegerType for u32 {}
impl UnsignedIntegerType for u32 {}

impl ScalarType for u64 {}
impl IntegerType for u64 {}
impl UnsignedIntegerType for u64 {}

impl ScalarType for u128 {}
impl IntegerType for u128 {}
impl UnsignedIntegerType for u128 {}

impl ScalarType for usize {}
impl IntegerType for usize {}
impl UnsignedIntegerType for usize {}

impl ScalarType for f32 {}
impl FloatType for f32 {}

impl ScalarType for f64 {}
impl FloatType for f64 {}

pub trait ScalarCoreOps: Clone + Sized {
    /// Returns additive identity.
    fn zero() -> Self {
        todo!()
    }
    /// Returns multiplicative identity.
    fn one() -> Self {
        todo!()
    }
    /// Returns scalar `2`.
    fn two() -> Self {
        todo!()
    }
    /// Returns scalar `10`.
    fn ten() -> Self {
        todo!()
    }
    /// Returns scalar `-1`.
    fn neg_one() -> Self {
        todo!()
    }
    /// Returns constant π.
    fn pi() -> Self {
        todo!()
    }
    /// Returns constant τ.
    fn tau() -> Self {
        todo!()
    }
    /// Returns constant e.
    fn e() -> Self {
        todo!()
    }
    /// Returns NaN sentinel value.
    fn nan() -> Self {
        todo!()
    }
    /// Returns positive infinity.
    fn infinity() -> Self {
        todo!()
    }
    /// Returns negative infinity.
    fn neg_infinity() -> Self {
        todo!()
    }
    /// Parses decimal literal.
    fn parse_decimal(_text: &str) -> Self {
        todo!()
    }
    /// Formats decimal string.
    fn to_decimal_string(&self) -> String {
        todo!()
    }
    /// Formats scientific notation string.
    fn to_scientific_string(&self) -> String {
        todo!()
    }
    /// Canonicalizes representation.
    fn normalize(&self) -> Self {
        todo!()
    }
    /// Returns true when value is zero.
    fn is_zero(&self) -> bool {
        todo!()
    }
    /// Returns true when value is one.
    fn is_one(&self) -> bool {
        todo!()
    }
    /// Returns true when value is NaN.
    fn is_nan(&self) -> bool {
        todo!()
    }
    /// Returns true when value is +/- infinity.
    fn is_infinite(&self) -> bool {
        todo!()
    }
    /// Returns true when value is finite.
    fn is_finite(&self) -> bool {
        todo!()
    }
    /// Returns true when value is positive.
    fn is_positive(&self) -> bool {
        todo!()
    }
    /// Returns true when value is negative.
    fn is_negative(&self) -> bool {
        todo!()
    }
    /// Returns sign of this value.
    fn signum(&self) -> Self {
        todo!()
    }
    /// Returns reciprocal.
    fn recip(&self) -> Self {
        todo!()
    }
    /// Returns squared value.
    fn square(&self) -> Self {
        todo!()
    }
    /// Returns cubed value.
    fn cube(&self) -> Self {
        todo!()
    }
    /// Returns square root.
    fn sqrt(&self) -> Self {
        todo!()
    }
    /// Returns cubic root.
    fn cbrt(&self) -> Self {
        todo!()
    }
    /// Returns e^x.
    fn exp(&self) -> Self {
        todo!()
    }
    /// Returns 2^x.
    fn exp2(&self) -> Self {
        todo!()
    }
    /// Returns 10^x.
    fn exp10(&self) -> Self {
        todo!()
    }
    /// Returns natural logarithm.
    fn ln(&self) -> Self {
        todo!()
    }
    /// Returns base-2 logarithm.
    fn log2(&self) -> Self {
        todo!()
    }
    /// Returns base-10 logarithm.
    fn log10(&self) -> Self {
        todo!()
    }
    /// Returns sine.
    fn sin(&self) -> Self {
        todo!()
    }
    /// Returns cosine.
    fn cos(&self) -> Self {
        todo!()
    }
    /// Returns tangent.
    fn tan(&self) -> Self {
        todo!()
    }
    /// Returns arcsine.
    fn asin(&self) -> Self {
        todo!()
    }
    /// Returns arccosine.
    fn acos(&self) -> Self {
        todo!()
    }
    /// Returns arctangent.
    fn atan(&self) -> Self {
        todo!()
    }
    /// Returns hyperbolic sine.
    fn sinh(&self) -> Self {
        todo!()
    }
    /// Returns hyperbolic cosine.
    fn cosh(&self) -> Self {
        todo!()
    }
    /// Returns hyperbolic tangent.
    fn tanh(&self) -> Self {
        todo!()
    }
    /// Applies floor.
    fn floor(&self) -> Self {
        todo!()
    }
    /// Applies ceil.
    fn ceil(&self) -> Self {
        todo!()
    }
    /// Applies round.
    fn round(&self) -> Self {
        todo!()
    }
    /// Applies trunc.
    fn trunc(&self) -> Self {
        todo!()
    }
    /// Returns fractional component.
    fn fract(&self) -> Self {
        todo!()
    }
    /// Returns negated value.
    fn neg(&self) -> Self {
        todo!()
    }
    /// Returns absolute value.
    fn abs(&self) -> Self {
        todo!()
    }
    /// Adds scalar operand.
    fn add(&self, _rhs: UsfOrNormalScalar) -> Self {
        todo!()
    }
    /// Subtracts scalar operand.
    fn sub(&self, _rhs: UsfOrNormalScalar) -> Self {
        todo!()
    }
    /// Multiplies scalar operand.
    fn mul(&self, _rhs: UsfOrNormalScalar) -> Self {
        todo!()
    }
    /// Divides scalar operand.
    fn div(&self, _rhs: UsfOrNormalScalar) -> Self {
        todo!()
    }
    /// Computes remainder against scalar operand.
    fn rem(&self, _rhs: UsfOrNormalScalar) -> Self {
        todo!()
    }
    /// Returns minimum of operands.
    fn min(&self, _rhs: UsfOrNormalScalar) -> Self {
        todo!()
    }
    /// Returns maximum of operands.
    fn max(&self, _rhs: UsfOrNormalScalar) -> Self {
        todo!()
    }
    /// Clamps this value to `[lo, hi]`.
    fn clamp(&self, _lo: UsfOrNormalScalar, _hi: UsfOrNormalScalar) -> Self {
        todo!()
    }
    /// Raises to power.
    fn pow(&self, _rhs: UsfOrNormalScalar) -> Self {
        todo!()
    }
    /// Computes `atan2(self, rhs)`.
    fn atan2(&self, _rhs: UsfOrNormalScalar) -> Self {
        todo!()
    }
    /// Computes hypotenuse with `rhs`.
    fn hypot(&self, _rhs: UsfOrNormalScalar) -> Self {
        todo!()
    }
    /// Computes Euclidean modulo.
    fn mod_euclid(&self, _rhs: UsfOrNormalScalar) -> Self {
        todo!()
    }
    /// Computes `self * b + c`.
    fn fma(&self, _b: UsfOrNormalScalar, _c: UsfOrNormalScalar) -> Self {
        todo!()
    }
    /// Performs linear interpolation.
    fn lerp(&self, _rhs: UsfOrNormalScalar, _t: UsfOrNormalDecimalScalar) -> Self {
        todo!()
    }
    /// Performs smoothstep interpolation.
    fn smoothstep(&self, _edge0: UsfOrNormalScalar, _edge1: UsfOrNormalScalar, _t: UsfOrNormalDecimalScalar) -> Self {
        todo!()
    }
    /// Compares equality.
    fn cmp_eq(&self, _rhs: UsfOrNormalScalar) -> bool {
        todo!()
    }
    /// Compares inequality.
    fn cmp_ne(&self, _rhs: UsfOrNormalScalar) -> bool {
        todo!()
    }
    /// Compares less-than.
    fn cmp_lt(&self, _rhs: UsfOrNormalScalar) -> bool {
        todo!()
    }
    /// Compares less-or-equal.
    fn cmp_le(&self, _rhs: UsfOrNormalScalar) -> bool {
        todo!()
    }
    /// Compares greater-than.
    fn cmp_gt(&self, _rhs: UsfOrNormalScalar) -> bool {
        todo!()
    }
    /// Compares greater-or-equal.
    fn cmp_ge(&self, _rhs: UsfOrNormalScalar) -> bool {
        todo!()
    }
}

pub trait ScalarFieldOps: ScalarCoreOps {
    /// Returns wrapped scalar value.
    fn get_value(&self) -> Self {
        todo!()
    }
    /// Sets wrapped scalar value.
    fn set_value(&mut self, _value: Self) {
        todo!()
    }
}

pub trait ScalarBridgeOps: ScalarCoreOps {
    /// Converts from scalar union.
    fn from_scalar<ScalarB: ScalarContract>(_value: OneOf2<Self, ScalarB>) -> Self {
        todo!()
    }
    /// Converts to scalar union.
    fn to_scalar<ScalarB: ScalarContract>(&self) -> OneOf2<Self, ScalarB> {
        todo!()
    }
}

pub trait ScalarContract: ScalarCoreOps + ScalarFieldOps + ScalarBridgeOps {}

impl<T> ScalarContract for T where T: ScalarCoreOps + ScalarFieldOps + ScalarBridgeOps {}
