//! Shared scalar contracts for both USF and normal numeric representations.
//!
//! Facade-first rule:
//! - These traits define semantics and panic contracts.
//! - Script-facing APIs should be exposed through facade/binding layers, not direct generic trait calls.
//!
//! Kind/repr mechanism:
//! - Mixed-repr scalar inputs use `UsfOrNormalScalar`.
//! - Mixed-repr interpolation factors use `UsfOrNormalFractionalScalar`.
//! - Output projection policy for mixed-repr read paths is handled by facade-level `Mode: OpMode` parameterization where needed.
//! - Operation-intrinsic mode variance should be expressed with `op_policy::OpPolicy`, and policy compatibility must be validated at runtime by each concrete algorithm implementation.
//!
//! Method doc schema:
//! - Summary line: describe intent and core working principle.
//! - `# Parameters`: document each argument and expected role.
//! - `# Returns`: document the returned value and shape/branch semantics.
//! - Optional `# Repr` section for mixed-repr semantics.
//! - Optional `# Panics` section for runtime guard clauses and undefined math states.

use super::aliases::{UsfOrNormalFractionalScalar, UsfOrNormalScalar};
use base_mod_shared::utils::one_of::OneOf2;

/// Base trait for scalar carrier types used by the math sketch.
pub trait ScalarType: Clone + 'static {}
/// Marker trait for integer scalar types.
pub trait IntegerType: ScalarType {}
/// Marker trait for signed integer scalar types.
pub trait SignedIntegerType: IntegerType {}
/// Marker trait for unsigned integer scalar types.
pub trait UnsignedIntegerType: IntegerType {}
/// Marker trait for floating-point scalar types.
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

/// Repr-agnostic scalar operations.
/// This trait encodes arithmetic and transcendental behavior independent of the concrete
/// representation family (`Usf` vs `Normal`).
/// # Working Principle
/// - Implementers define scalar semantics for their concrete representation.
/// - Default method bodies are contract stubs and should be replaced by backend logic.
/// - Mixed-repr operands are accepted through `UsfOrNormal*` aliases and resolved by backend policy.
/// # Precision & Range
/// - Implementations are responsible for enforcing range and precision constraints.
/// - Repr projection rules are coordinated through facade-level output policies.
/// # Usage
/// - Use `ScalarContract` bounds when consumers need core, field, and bridge operations together.
/// - Use `UsfOrNormalScalar` / `UsfOrNormalFractionalScalar` parameters for mixed-repr inputs.
///
/// # Examples
/// ```ignore
/// use crate::usf::math::scalar::shared::ScalarContract;
/// use crate::usf::math::scalar::aliases::UsfOrNormalScalar;
/// use crate::usf::math::scalar::aliases::UsfOrNormalFractionalScalar;
///
/// fn blend<S: ScalarContract>(lhs: &S, rhs: UsfOrNormalScalar, factor: UsfOrNormalFractionalScalar) -> S {
///     lhs.lerp(rhs, factor)
/// }
/// ```
pub trait ScalarCoreOps: Clone + Sized {
    // Naming contract: scalars are single-component values, so canonical arithmetic names
    // stay `add/sub/mul/div/rem` instead of `component_*`.
    /// Returns the additive identity value.
    ///
    /// # Parameters
    /// - None.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn zero() -> Self {
        todo!()
    }

    /// Returns the multiplicative identity value.
    ///
    /// # Parameters
    /// - None.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn one() -> Self {
        todo!()
    }

    /// Returns constant two.
    ///
    /// # Parameters
    /// - None.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn two() -> Self {
        todo!()
    }

    /// Returns constant ten.
    ///
    /// # Parameters
    /// - None.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn ten() -> Self {
        todo!()
    }

    /// Returns constant negative one.
    ///
    /// # Parameters
    /// - None.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn neg_one() -> Self {
        todo!()
    }

    /// Returns constant pi.
    ///
    /// # Parameters
    /// - None.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn pi() -> Self {
        todo!()
    }

    /// Returns constant tau.
    ///
    /// # Parameters
    /// - None.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn tau() -> Self {
        todo!()
    }

    /// Returns constant e.
    ///
    /// # Parameters
    /// - None.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn e() -> Self {
        todo!()
    }

    /// Returns a NaN value.
    ///
    /// # Parameters
    /// - None.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn nan() -> Self {
        todo!()
    }

    /// Returns positive infinity.
    ///
    /// # Parameters
    /// - None.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn infinity() -> Self {
        todo!()
    }

    /// Returns negative infinity.
    ///
    /// # Parameters
    /// - None.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn neg_infinity() -> Self {
        todo!()
    }

    /// Parses a decimal literal into this scalar type.
    ///
    /// # Parameters
    /// - `text` (&str): Source text to parse or interpret.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Panics
    /// - Panics if `text` is not a valid finite decimal literal for this backend.
    /// - Panics if parsed range/precision cannot be represented by this backend's numeric model.
    fn parse_decimal(_text: &str) -> Self {
        todo!()
    }

    /// Formats this value as a decimal string.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - String representation of the value.
    fn to_decimal_string(&self) -> String {
        todo!()
    }

    /// Formats this value as a scientific-notation string.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - String representation of the value.
    fn to_scientific_string(&self) -> String {
        todo!()
    }

    /// Canonicalizes internal digit/radix representation.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn normalize(&self) -> Self {
        todo!()
    }

    /// Returns true when value equals zero.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - Boolean result of the requested predicate or comparison.
    fn is_zero(&self) -> bool {
        todo!()
    }

    /// Returns true when value equals one.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - Boolean result of the requested predicate or comparison.
    fn is_one(&self) -> bool {
        todo!()
    }

    /// Returns true when value is NaN.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - Boolean result of the requested predicate or comparison.
    fn is_nan(&self) -> bool {
        todo!()
    }

    /// Returns true when value is +/- infinity.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - Boolean result of the requested predicate or comparison.
    fn is_infinite(&self) -> bool {
        todo!()
    }

    /// Returns true when value is finite.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - Boolean result of the requested predicate or comparison.
    fn is_finite(&self) -> bool {
        todo!()
    }

    /// Returns true when value is strictly positive.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - Boolean result of the requested predicate or comparison.
    fn is_positive(&self) -> bool {
        todo!()
    }

    /// Returns true when value is strictly negative.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - Boolean result of the requested predicate or comparison.
    fn is_negative(&self) -> bool {
        todo!()
    }

    /// Returns the sign of this value.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn signum(&self) -> Self {
        todo!()
    }

    /// Computes the multiplicative reciprocal.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Panics
    /// - Panics if `self` is zero.
    fn recip(&self) -> Self {
        todo!()
    }

    /// Squares this value.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn square(&self) -> Self {
        todo!()
    }

    /// Cubes this value.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn cube(&self) -> Self {
        todo!()
    }

    /// Computes the square root.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Panics
    /// - Panics if `self` is negative and real-only sqrt semantics are enforced.
    fn sqrt(&self) -> Self {
        todo!()
    }

    /// Computes cubic root.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn cbrt(&self) -> Self {
        todo!()
    }

    /// Computes e^x.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn exp(&self) -> Self {
        todo!()
    }

    /// Computes 2^x.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn exp2(&self) -> Self {
        todo!()
    }

    /// Computes 10^x.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn exp10(&self) -> Self {
        todo!()
    }

    /// Computes the natural logarithm.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Panics
    /// - Panics if `self` is non-positive.
    fn ln(&self) -> Self {
        todo!()
    }

    /// Computes the base-2 logarithm.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Panics
    /// - Panics if `self` is non-positive.
    fn log2(&self) -> Self {
        todo!()
    }

    /// Computes the base-10 logarithm.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Panics
    /// - Panics if `self` is non-positive.
    fn log10(&self) -> Self {
        todo!()
    }

    /// Computes sine in radians.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn sin(&self) -> Self {
        todo!()
    }

    /// Computes cosine in radians.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn cos(&self) -> Self {
        todo!()
    }

    /// Computes tangent in radians.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Panics
    /// - Panics if `self` is at a tangent singularity and strict singularity handling is used.
    fn tan(&self) -> Self {
        todo!()
    }

    /// Computes arcsine in radians.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Panics
    /// - Panics if `self` is outside `[-1, 1]` under real-only semantics.
    fn asin(&self) -> Self {
        todo!()
    }

    /// Computes arccosine in radians.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Panics
    /// - Panics if `self` is outside `[-1, 1]` under real-only semantics.
    fn acos(&self) -> Self {
        todo!()
    }

    /// Computes arctangent in radians.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn atan(&self) -> Self {
        todo!()
    }

    /// Computes hyperbolic sine.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn sinh(&self) -> Self {
        todo!()
    }

    /// Computes hyperbolic cosine.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn cosh(&self) -> Self {
        todo!()
    }

    /// Computes hyperbolic tangent.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn tanh(&self) -> Self {
        todo!()
    }

    /// Rounds down.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn floor(&self) -> Self {
        todo!()
    }

    /// Rounds up.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn ceil(&self) -> Self {
        todo!()
    }

    /// Rounds to nearest integer.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn round(&self) -> Self {
        todo!()
    }

    /// Truncates fractional component.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn trunc(&self) -> Self {
        todo!()
    }

    /// Returns fractional component.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn fract(&self) -> Self {
        todo!()
    }

    /// Negates this value.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn neg(&self) -> Self {
        todo!()
    }

    /// Returns absolute value.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn abs(&self) -> Self {
        todo!()
    }

    /// Adds another scalar.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (UsfOrNormalScalar): Right-hand-side operand.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Repr
    /// - Allowed: `{rhs: Usf}` and `{rhs: Normal}`.
    /// - Disallowed combinations: none; `UsfScalar` accepts both rhs repr branches.
    /// # Panics
    /// - Panics if repr selection is invalid for this backend.
    fn add(&self, _rhs: UsfOrNormalScalar) -> Self {
        todo!()
    }

    /// Subtracts another scalar.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (UsfOrNormalScalar): Right-hand-side operand.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Repr
    /// - Allowed: `{rhs: Usf}` and `{rhs: Normal}`.
    /// - Disallowed combinations: none; `UsfScalar` accepts both rhs repr branches.
    /// # Panics
    /// - Panics if repr selection is invalid for this backend.
    fn sub(&self, _rhs: UsfOrNormalScalar) -> Self {
        todo!()
    }

    /// Multiplies by another scalar.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (UsfOrNormalScalar): Right-hand-side operand.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Repr
    /// - Allowed: `{rhs: Usf}` and `{rhs: Normal}`.
    /// - Disallowed combinations: none; `UsfScalar` accepts both rhs repr branches.
    /// # Panics
    /// - Panics if repr selection is invalid for this backend.
    fn mul(&self, _rhs: UsfOrNormalScalar) -> Self {
        todo!()
    }

    /// Divides by the right-hand-side operand.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (UsfOrNormalScalar): Right-hand-side operand.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Repr
    /// - Allowed: `{rhs: Usf}` and `{rhs: Normal}`.
    /// - Disallowed combinations: none; `UsfScalar` accepts both rhs repr branches.
    /// # Panics
    /// - Panics if repr selection is invalid for this backend.
    /// - Panics if `rhs` is zero.
    fn div(&self, _rhs: UsfOrNormalScalar) -> Self {
        todo!()
    }

    /// Computes remainder against the right-hand-side operand.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (UsfOrNormalScalar): Right-hand-side operand.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Repr
    /// - Allowed: `{rhs: Usf}` and `{rhs: Normal}`.
    /// - Disallowed combinations: none; `UsfScalar` accepts both rhs repr branches.
    /// # Panics
    /// - Panics if repr selection is invalid for this backend.
    /// - Panics if `rhs` is zero.
    fn rem(&self, _rhs: UsfOrNormalScalar) -> Self {
        todo!()
    }

    /// Returns minimum with `rhs`.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (UsfOrNormalScalar): Right-hand-side operand.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Repr
    /// - Allowed: `{rhs: Usf}` and `{rhs: Normal}`.
    /// - Disallowed combinations: none; `UsfScalar` accepts both rhs repr branches.
    /// # Panics
    /// - Panics if repr selection is invalid for this backend.
    fn min(&self, _rhs: UsfOrNormalScalar) -> Self {
        todo!()
    }

    /// Returns maximum with `rhs`.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (UsfOrNormalScalar): Right-hand-side operand.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Repr
    /// - Allowed: `{rhs: Usf}` and `{rhs: Normal}`.
    /// - Disallowed combinations: none; `UsfScalar` accepts both rhs repr branches.
    /// # Panics
    /// - Panics if repr selection is invalid for this backend.
    fn max(&self, _rhs: UsfOrNormalScalar) -> Self {
        todo!()
    }

    /// Clamps the value to the provided bounds.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `lo` (UsfOrNormalScalar): Lower bound.
    /// - `hi` (UsfOrNormalScalar): Upper bound.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Repr
    /// - Allowed: all `{lo, hi}` pairings in `{Usf, Normal} × {Usf, Normal}`.
    /// - Disallowed combinations: none; all repr pairs are accepted.
    /// # Panics
    /// - Panics if repr selection is invalid for this backend.
    /// - Panics if `lo > hi`.
    fn clamp(&self, _lo: UsfOrNormalScalar, _hi: UsfOrNormalScalar) -> Self {
        todo!()
    }

    /// Raises this value to the given power.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (UsfOrNormalScalar): Right-hand-side operand.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Repr
    /// - Allowed: `{rhs: Usf}` and `{rhs: Normal}`.
    /// - Disallowed combinations: none; `UsfScalar` accepts both rhs repr branches.
    /// # Panics
    /// - Panics if repr selection is invalid for this backend.
    /// - Panics for undefined exponent/base combinations under real-only semantics.
    fn pow(&self, _rhs: UsfOrNormalScalar) -> Self {
        todo!()
    }

    /// Computes two-argument arctangent.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (UsfOrNormalScalar): Right-hand-side operand.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Repr
    /// - Allowed: `{rhs: Usf}` and `{rhs: Normal}`.
    /// - Disallowed combinations: none; `UsfScalar` accepts both rhs repr branches.
    /// # Panics
    /// - Panics if repr selection is invalid for this backend.
    fn atan2(&self, _rhs: UsfOrNormalScalar) -> Self {
        todo!()
    }

    /// Computes hypotenuse with another scalar.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (UsfOrNormalScalar): Right-hand-side operand.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Repr
    /// - Allowed: `{rhs: Usf}` and `{rhs: Normal}`.
    /// - Disallowed combinations: none; `UsfScalar` accepts both rhs repr branches.
    /// # Panics
    /// - Panics if repr selection is invalid for this backend.
    fn hypot(&self, _rhs: UsfOrNormalScalar) -> Self {
        todo!()
    }

    /// Computes Euclidean modulo with the right-hand-side operand.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (UsfOrNormalScalar): Right-hand-side operand.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Repr
    /// - Allowed: `{rhs: Usf}` and `{rhs: Normal}`.
    /// - Disallowed combinations: none; `UsfScalar` accepts both rhs repr branches.
    /// # Panics
    /// - Panics if repr selection is invalid for this backend.
    /// - Panics if `rhs` is zero.
    fn mod_euclid(&self, _rhs: UsfOrNormalScalar) -> Self {
        todo!()
    }

    /// Computes fused multiply-add: `self * b + c`.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `b` (UsfOrNormalScalar): Secondary operand used by the operation.
    /// - `c` (UsfOrNormalScalar): Tertiary operand used by the operation.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Repr
    /// - Allowed: all `{b, c}` pairings in `{Usf, Normal} × {Usf, Normal}`.
    /// - Disallowed combinations: none; all repr pairs are accepted.
    /// # Panics
    /// - Panics if repr selection is invalid for this backend.
    fn fma(&self, _b: UsfOrNormalScalar, _c: UsfOrNormalScalar) -> Self {
        todo!()
    }

    /// Performs linear interpolation.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (UsfOrNormalScalar): Right-hand-side operand.
    /// - `t` (UsfOrNormalFractionalScalar): Interpolation factor.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Repr
    /// - Allowed: `{rhs: Usf}` and `{rhs: Normal}`.
    /// - Allowed: `{t: Usf}` and `{t: Normal}`.
    /// - Disallowed combinations: none; all repr pairs are accepted.
    /// # Panics
    /// - Panics if repr selection is invalid for this backend.
    fn lerp(&self, _rhs: UsfOrNormalScalar, _t: UsfOrNormalFractionalScalar) -> Self {
        todo!()
    }

    /// Performs smoothstep interpolation.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `edge0` (UsfOrNormalScalar): Lower interpolation edge.
    /// - `edge1` (UsfOrNormalScalar): Upper interpolation edge.
    /// - `t` (UsfOrNormalFractionalScalar): Interpolation factor.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Repr
    /// - Allowed: all `{edge0, edge1}` pairings in `{Usf, Normal} × {Usf, Normal}`.
    /// - Allowed: `{t: Usf}` and `{t: Normal}`.
    /// - Disallowed combinations: none; all repr pairs are accepted.
    /// # Panics
    /// - Panics if repr selection is invalid for this backend.
    /// - Panics if edge ordering is invalid (`edge0 > edge1`) under strict smoothstep semantics.
    fn smoothstep(&self, _edge0: UsfOrNormalScalar, _edge1: UsfOrNormalScalar, _t: UsfOrNormalFractionalScalar) -> Self {
        todo!()
    }

    /// Compares equality.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (UsfOrNormalScalar): Right-hand-side operand.
    ///
    /// # Returns
    /// - Boolean result of the requested predicate or comparison.
    ///
    /// # Repr
    /// - Allowed: `{rhs: Usf}` and `{rhs: Normal}`.
    /// - Disallowed combinations: none; `UsfScalar` accepts both rhs repr branches.
    /// # Panics
    /// - Panics if repr selection is invalid for this backend.
    fn cmp_eq(&self, _rhs: UsfOrNormalScalar) -> bool {
        todo!()
    }

    /// Compares inequality.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (UsfOrNormalScalar): Right-hand-side operand.
    ///
    /// # Returns
    /// - Boolean result of the requested predicate or comparison.
    ///
    /// # Repr
    /// - Allowed: `{rhs: Usf}` and `{rhs: Normal}`.
    /// - Disallowed combinations: none; `UsfScalar` accepts both rhs repr branches.
    /// # Panics
    /// - Panics if repr selection is invalid for this backend.
    fn cmp_ne(&self, _rhs: UsfOrNormalScalar) -> bool {
        todo!()
    }

    /// Compares `<`.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (UsfOrNormalScalar): Right-hand-side operand.
    ///
    /// # Returns
    /// - Boolean result of the requested predicate or comparison.
    ///
    /// # Repr
    /// - Allowed: `{rhs: Usf}` and `{rhs: Normal}`.
    /// - Disallowed combinations: none; `UsfScalar` accepts both rhs repr branches.
    /// # Panics
    /// - Panics if repr selection is invalid for this backend.
    fn cmp_lt(&self, _rhs: UsfOrNormalScalar) -> bool {
        todo!()
    }

    /// Compares `<=`.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (UsfOrNormalScalar): Right-hand-side operand.
    ///
    /// # Returns
    /// - Boolean result of the requested predicate or comparison.
    ///
    /// # Repr
    /// - Allowed: `{rhs: Usf}` and `{rhs: Normal}`.
    /// - Disallowed combinations: none; `UsfScalar` accepts both rhs repr branches.
    /// # Panics
    /// - Panics if repr selection is invalid for this backend.
    fn cmp_le(&self, _rhs: UsfOrNormalScalar) -> bool {
        todo!()
    }

    /// Compares `>`.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (UsfOrNormalScalar): Right-hand-side operand.
    ///
    /// # Returns
    /// - Boolean result of the requested predicate or comparison.
    ///
    /// # Repr
    /// - Allowed: `{rhs: Usf}` and `{rhs: Normal}`.
    /// - Disallowed combinations: none; `UsfScalar` accepts both rhs repr branches.
    /// # Panics
    /// - Panics if repr selection is invalid for this backend.
    fn cmp_gt(&self, _rhs: UsfOrNormalScalar) -> bool {
        todo!()
    }

    /// Compares `>=`.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (UsfOrNormalScalar): Right-hand-side operand.
    ///
    /// # Returns
    /// - Boolean result of the requested predicate or comparison.
    ///
    /// # Repr
    /// - Allowed: `{rhs: Usf}` and `{rhs: Normal}`.
    /// - Disallowed combinations: none; `UsfScalar` accepts both rhs repr branches.
    /// # Panics
    /// - Panics if repr selection is invalid for this backend.
    fn cmp_ge(&self, _rhs: UsfOrNormalScalar) -> bool {
        todo!()
    }
}

/// Scalar field accessor contract.
/// These methods model property-like get/set behavior that facade/binding layers can map to
/// script-facing field semantics.
pub trait ScalarFieldOps: ScalarCoreOps {
    /// Gets stored scalar value.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn get_value(&self) -> Self {
        todo!()
    }

    /// Sets stored scalar value.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `value` (Self): Input value for this operation.
    ///
    /// # Returns
    /// - No value; mutates receiver state where applicable.
    fn set_value(&mut self, _value: Self) {
        todo!()
    }
}

/// Scalar conversion/bridge contract.
/// This trait models repr-bridge hooks used by facade layers when converting between
/// concrete scalar carriers.
pub trait ScalarBridgeOps: ScalarCoreOps {
    /// Converts a primitive scalar into this scalar carrier.
    ///
    /// # Parameters
    /// - `value` (T): Input value for this operation.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn from_primitive<T: ScalarType>(_value: T) -> Self {
        todo!()
    }

    /// Converts this scalar carrier into primitive scalar `T`.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - Computed result of type `T`.
    ///
    /// # Panics
    /// - Panics if conversion to `T` would overflow, underflow, or lose required repr semantics.
    /// - Panics if `T` is unsupported by the concrete conversion backend.
    fn to_primitive<T: ScalarType>(&self) -> T {
        todo!()
    }

    /// Converts a `NormalScalar` wrapper into this scalar carrier.
    ///
    /// # Parameters
    /// - `value` (super::normal::NormalScalar): Input value for this operation.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn from_scalar_wrapper(_value: super::normal::NormalScalar) -> Self {
        todo!()
    }

    /// Converts this scalar carrier into `NormalScalar` wrapper.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - Computed result of type `super::normal::NormalScalar`.
    fn to_scalar_wrapper(&self) -> super::normal::NormalScalar {
        todo!()
    }

    /// Converts from scalar union.
    ///
    /// # Parameters
    /// - `value` (OneOf2<Self, ScalarB>): Input value for this operation.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn from_scalar<ScalarB: ScalarContract>(_value: OneOf2<Self, ScalarB>) -> Self {
        todo!()
    }

    /// Converts to scalar union.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - Branch-union result of type `OneOf2<Self, ScalarB>`.
    fn to_scalar<ScalarB: ScalarContract>(&self) -> OneOf2<Self, ScalarB> {
        todo!()
    }
}

/// Full scalar contract used by higher-level math traits and facade generators.
pub trait ScalarContract: ScalarCoreOps + ScalarFieldOps + ScalarBridgeOps {}

impl<T> ScalarContract for T where T: ScalarCoreOps + ScalarFieldOps + ScalarBridgeOps {}

/// Erased scalar contract for generic facade plumbing.
pub trait ScalarAnyContract: Clone + Sized {}
impl<T> ScalarAnyContract for T where T: ScalarContract {}

/// Marker contract for scalar carriers that can represent fractional values.
/// This is a capability marker, not a value-class marker:
/// - Types implementing this trait may still hold integer-valued runtime states.
/// - The contract only requires that fractional representation is supported when needed.
///
/// This preserves semantics for operations that must not project into integer-only scalar
/// repr branches (for example norms, distances, and angles).
///
/// # Examples
/// ```ignore
/// use crate::usf::math::scalar::shared::{FractionalScalarContract, ScalarContract};
///
/// fn keep_fractional<S: FractionalScalarContract>(value: S) -> S {
///     value
/// }
///
/// fn generic_scalar<S: ScalarContract>(value: S) -> S {
///     value
/// }
/// ```
pub trait FractionalScalarContract: ScalarContract {}
