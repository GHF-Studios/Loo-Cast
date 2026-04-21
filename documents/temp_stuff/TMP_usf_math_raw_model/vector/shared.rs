#![allow(dead_code)]

//! Shared vector contracts for both USF and normal vector surfaces.
//!
//! Facade-first rule:
//! - These traits are semantic contracts, not final end-user APIs.
//! - Rhai exposure should happen through monomorphized facades/bindings.
//!
//! Domain/quality mechanism:
//! - Mixed-domain vector/scalar operands are represented with `UsfOrNormal*` aliases.
//! - Output projection requests are represented with `OutputMode`.
//! - Invalid domain-quality combinations are guarded by panic-fast checks.
//!
//! Method doc schema:
//! - Summary line: describe intent and core working principle.
//! - `# Parameters`: document each argument and expected role.
//! - `# Returns`: document the returned value and shape/branch semantics.
//! - Optional `# Domain` section for mixed-domain semantics.
//! - Optional `# Panics` section for runtime guard clauses and undefined math states.

use super::super::aliases::OutputMode;
use super::super::scalar::aliases::{UsfOrNormalDecimalScalar, UsfOrNormalScalar};
use super::aliases::UsfOrNormalVector;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HomogeneousWState {
    /// `w` is finite and non-zero, so strict point dehomogenization is valid.
    Finite,
    /// `w` is exactly zero, so strict point dehomogenization is undefined.
    Zero,
    /// `w` is NaN/Inf or otherwise non-finite under strict normalization policy.
    NonFinite,
}

#[derive(Clone, Debug, PartialEq)]
pub enum HomogeneousPointOrDirection<Vector3d> {
    Point(Vector3d),
    Direction(Vector3d),
}

/// Dimension-generic vector core operations.
/// # Working Principle
/// - `D` is the compile-time vector dimension and governs component-oriented behavior.
/// - Methods use mixed-domain aliases for inputs while returning backend-owned `Self`.
/// # Usage
/// - Implement this trait on concrete vector carriers (`Usf`, `Normal`, or other backends).
/// - Use `VectorContract<D>` bounds when generic call sites need vector core+field+bridge behavior.
pub trait VectorCoreOps<const D: usize>: Clone + Sized {
    /// Zero vector.
    ///
    /// # Parameters
    /// - None.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn zero() -> Self {
        todo!()
    }

    /// Returns all-ones vector.
    ///
    /// # Parameters
    /// - None.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn one() -> Self {
        todo!()
    }

    /// Returns vector with all vector components set to `value`.
    ///
    /// # Parameters
    /// - `value` (UsfOrNormalScalar): Input value for this operation.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Domain
    /// - Allowed: `{value: Usf}`.
    /// - Disallowed combinations: `{value: Normal}` in this concrete `UsfVector` API.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    fn splat(_value: UsfOrNormalScalar) -> Self {
        todo!()
    }

    /// Builds vector from USF vector component array.
    ///
    /// # Parameters
    /// - `vector_components` ([UsfOrNormalScalar; D]): Vector component payload.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Domain
    /// - Allowed: `{vector_components: Usf}`.
    /// - Disallowed combinations: `{vector_components: Normal}` in this concrete `UsfVector` API.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    /// - Panics if `D < 2` is rejected by runtime validation.
    fn from_vector_components(_vector_components: [UsfOrNormalScalar; D]) -> Self {
        todo!()
    }

    /// Returns vector component array representation.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `output_mode` (OutputMode): Output domain/quality projection policy.
    ///
    /// # Returns
    /// - Fixed-size array result of type `[UsfOrNormalScalar; D]`.
    ///
    /// # Domain
    /// - Allowed output: `{vector_components: Usf}`.
    /// - Disallowed combinations: `{vector_components: Normal}` in this concrete `UsfVector` API.
    fn to_vector_components(&self, _output_mode: OutputMode) -> [UsfOrNormalScalar; D] {
        todo!()
    }

    /// Returns normalized direction.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Domain
    /// - Allowed: `{self: Usf}`.
    /// - Disallowed combinations: not applicable in this unary concrete `UsfVector` API.
    /// # Panics
    /// - Panics if the vector has zero length.
    fn normalize(&self) -> Self {
        todo!()
    }

    /// Rounds each vector component down.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn floor(&self) -> Self {
        todo!()
    }

    /// Rounds each vector component up.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn ceil(&self) -> Self {
        todo!()
    }

    /// Rounds each vector component to nearest integer.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn round(&self) -> Self {
        todo!()
    }

    /// Keeps fractional part per vector component.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn fract(&self) -> Self {
        todo!()
    }

    /// Negates each vector component.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn neg(&self) -> Self {
        todo!()
    }

    /// Takes absolute value per vector component.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn abs(&self) -> Self {
        todo!()
    }

    /// Adds a vector in either domain.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (V): Right-hand-side operand.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Domain
    /// - Accepts `{self: Usf, rhs: Usf}`.
    /// - Accepts `{self: Usf, rhs: Normal}`.
    /// - Disallowed combinations: none; all domain pairs are accepted.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    fn add<V: VectorContract<D>>(&self, _rhs: V) -> Self {
        todo!()
    }

    /// Subtracts a vector in either domain.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (V): Right-hand-side operand.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Domain
    /// - Accepts `{self: Usf, rhs: Usf}`.
    /// - Accepts `{self: Usf, rhs: Normal}`.
    /// - Disallowed combinations: none; all domain pairs are accepted.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    fn sub<V: VectorContract<D>>(&self, _rhs: V) -> Self {
        todo!()
    }

    /// Multiplies component-wise by a vector in either domain.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (V): Right-hand-side operand.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Domain
    /// - Accepts `{self: Usf, rhs: Usf}`.
    /// - Accepts `{self: Usf, rhs: Normal}`.
    /// - Disallowed combinations: none; all domain pairs are accepted.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    fn component_mul<V: VectorContract<D>>(&self, _rhs: V) -> Self {
        todo!()
    }

    /// Divides component-wise by a vector in either domain.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (V): Right-hand-side operand.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Domain
    /// - Accepts `{self: Usf, rhs: Usf}`.
    /// - Accepts `{self: Usf, rhs: Normal}`.
    /// - Disallowed combinations: none; all domain pairs are accepted.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    /// - Panics if any corresponding vector component in `rhs` is zero.
    fn component_div<V: VectorContract<D>>(&self, _rhs: V) -> Self {
        todo!()
    }

    /// Computes component-wise remainder with a vector in either domain.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (V): Right-hand-side operand.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Domain
    /// - Accepts `{self: Usf, rhs: Usf}`.
    /// - Accepts `{self: Usf, rhs: Normal}`.
    /// - Disallowed combinations: none; all domain pairs are accepted.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    /// - Panics if any corresponding vector component in `rhs` is zero.
    fn component_rem<V: VectorContract<D>>(&self, _rhs: V) -> Self {
        todo!()
    }

    /// Returns component-wise minimum.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (V): Right-hand-side operand.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Domain
    /// - Accepts `{self: Usf, rhs: Usf}`.
    /// - Accepts `{self: Usf, rhs: Normal}`.
    /// - Disallowed combinations: none; all domain pairs are accepted.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    fn min<V: VectorContract<D>>(&self, _rhs: V) -> Self {
        todo!()
    }

    /// Returns component-wise maximum.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (V): Right-hand-side operand.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Domain
    /// - Accepts `{self: Usf, rhs: Usf}`.
    /// - Accepts `{self: Usf, rhs: Normal}`.
    /// - Disallowed combinations: none; all domain pairs are accepted.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    fn max<V: VectorContract<D>>(&self, _rhs: V) -> Self {
        todo!()
    }

    /// Clamps the value to the provided bounds.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `lo` (V): Lower bound.
    /// - `hi` (V): Upper bound.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Domain
    /// - Accepts all `{lo, hi}` pairings in `{Usf, Normal} × {Usf, Normal}`.
    /// - Disallowed combinations: none; all domain pairs are accepted.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    /// - Panics if any vector component has `lo > hi`.
    fn clamp<V: VectorContract<D>>(&self, _lo: V, _hi: V) -> Self {
        todo!()
    }

    /// Performs linear interpolation.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (V): Right-hand-side operand.
    /// - `t` (UsfOrNormalDecimalScalar): Interpolation factor.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Domain
    /// - Accepts `rhs` in `{Usf, Normal}`.
    /// - Accepts `t` in `{Usf, Normal}`.
    /// - Disallowed combinations: none; all domain pairs are accepted.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    fn lerp<V: VectorContract<D>>(&self, _rhs: V, _t: UsfOrNormalDecimalScalar) -> Self {
        todo!()
    }

    /// Performs smoothstep interpolation.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (V): Right-hand-side operand.
    /// - `t` (UsfOrNormalDecimalScalar): Interpolation factor.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Domain
    /// - Accepts `rhs` in `{Usf, Normal}`.
    /// - Accepts `t` in `{Usf, Normal}`.
    /// - Disallowed combinations: none; all domain pairs are accepted.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    fn smoothstep<V: VectorContract<D>>(&self, _rhs: V, _t: UsfOrNormalDecimalScalar) -> Self {
        todo!()
    }

    /// Computes dot product in requested output mode.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (V): Right-hand-side operand.
    /// - `output_mode` (OutputMode): Output domain/quality projection policy.
    ///
    /// # Returns
    /// - Computed result of type `UsfOrNormalDecimalScalar`.
    ///
    /// # Domain
    /// - Accepts `{self: Usf, rhs: Usf}`.
    /// - Accepts `{self: Usf, rhs: Normal}`.
    /// - Disallowed combinations: none; all domain pairs are accepted.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    /// - Panics when `output_mode.domain == OutputDomain::Usf` and `output_mode.quality_constraint == OutputQualityConstraint::AllowLossy`, because USF output never uses lossy projection.
    /// - Panics when `output_mode.domain == OutputDomain::Normal` and `output_mode.quality_constraint == OutputQualityConstraint::RequireLossless` but the projection loses precision or range.
    fn dot<V: VectorContract<D>>(&self, _rhs: V, _output_mode: OutputMode) -> UsfOrNormalDecimalScalar {
        todo!()
    }

    /// Computes Euclidean distance in requested output mode.
    /// Output behavior:
    /// - Computes using canonical USF working precision.
    /// - Projects the result into `output_mode.domain`.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (V): Right-hand-side operand.
    /// - `output_mode` (OutputMode): Output domain/quality projection policy.
    ///
    /// # Returns
    /// - Computed result of type `UsfOrNormalDecimalScalar`.
    ///
    /// # Domain
    /// - Accepts `{self: Usf, rhs: Usf}`.
    /// - Accepts `{self: Usf, rhs: Normal}`.
    /// - Disallowed combinations: none; all domain pairs are accepted.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    /// - Panics when `output_mode.domain == OutputDomain::Usf` and `output_mode.quality_constraint == OutputQualityConstraint::AllowLossy`, because USF output never uses lossy projection.
    /// - Panics when `output_mode.domain == OutputDomain::Normal` and `output_mode.quality_constraint == OutputQualityConstraint::RequireLossless` but the projection loses precision or range.
    fn distance<V: VectorContract<D>>(&self, _rhs: V, _output_mode: OutputMode) -> UsfOrNormalDecimalScalar {
        todo!()
    }

    /// Computes angle between vectors in requested output mode.
    /// Output behavior:
    /// - Computes using canonical USF working precision.
    /// - Projects the result into `output_mode.domain`.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (V): Right-hand-side operand.
    /// - `output_mode` (OutputMode): Output domain/quality projection policy.
    ///
    /// # Returns
    /// - Computed result of type `UsfOrNormalDecimalScalar`.
    ///
    /// # Domain
    /// - Accepts `{self: Usf, rhs: Usf}`.
    /// - Accepts `{self: Usf, rhs: Normal}`.
    /// - Disallowed combinations: none; all domain pairs are accepted.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    /// - Panics if either vector has zero length.
    /// - Panics when `output_mode.domain == OutputDomain::Usf` and `output_mode.quality_constraint == OutputQualityConstraint::AllowLossy`, because USF output never uses lossy projection.
    /// - Panics when `output_mode.domain == OutputDomain::Normal` and `output_mode.quality_constraint == OutputQualityConstraint::RequireLossless` but the projection loses precision or range.
    fn angle_between<V: VectorContract<D>>(&self, _rhs: V, _output_mode: OutputMode) -> UsfOrNormalDecimalScalar {
        todo!()
    }

    /// Projects this value onto the provided operand.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `onto` (V): Projection target.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Domain
    /// - Accepts `{self: Usf, onto: Usf}`.
    /// - Accepts `{self: Usf, onto: Normal}`.
    /// - Disallowed combinations: none; all domain pairs are accepted.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    /// - Panics if `onto` is the zero vector.
    fn project<V: VectorContract<D>>(&self, _onto: V) -> Self {
        todo!()
    }

    /// Computes the rejection of this value from the provided operand.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `onto` (V): Projection target.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Domain
    /// - Accepts `{self: Usf, onto: Usf}`.
    /// - Accepts `{self: Usf, onto: Normal}`.
    /// - Disallowed combinations: none; all domain pairs are accepted.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    /// - Panics if `onto` is the zero vector.
    fn reject<V: VectorContract<D>>(&self, _onto: V) -> Self {
        todo!()
    }

    /// Reflects this value around the provided normal.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `normal` (V): Normal vector.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Domain
    /// - Accepts `{self: Usf, normal: Usf}`.
    /// - Accepts `{self: Usf, normal: Normal}`.
    /// - Disallowed combinations: none; all domain pairs are accepted.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    /// - Panics if `normal` is the zero vector.
    fn reflect<V: VectorContract<D>>(&self, _normal: V) -> Self {
        todo!()
    }

    /// Computes fused multiply-add per vector component.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `b` (V): Secondary operand used by the operation.
    /// - `c` (V): Tertiary operand used by the operation.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Domain
    /// - Allowed: `{self: Usf, b: Usf, c: Usf}`.
    /// - Disallowed combinations: mixed-domain `b`/`c` values in this concrete `UsfVector` API.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    fn fma<V: VectorContract<D>>(&self, _b: V, _c: V) -> Self {
        todo!()
    }

    /// Adds scalar to each vector component.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (UsfOrNormalScalar): Right-hand-side operand.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Domain
    /// - Accepts `{self: Usf, scalar: Usf}`.
    /// - Accepts `{self: Usf, scalar: Normal}`.
    /// - Disallowed combinations: none; all domain pairs are accepted.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    fn add_scalar(&self, _rhs: UsfOrNormalScalar) -> Self {
        todo!()
    }

    /// Subtracts scalar from each vector component.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (UsfOrNormalScalar): Right-hand-side operand.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Domain
    /// - Accepts `{self: Usf, scalar: Usf}`.
    /// - Accepts `{self: Usf, scalar: Normal}`.
    /// - Disallowed combinations: none; all domain pairs are accepted.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    fn sub_scalar(&self, _rhs: UsfOrNormalScalar) -> Self {
        todo!()
    }

    /// Multiplies each vector component by scalar.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (UsfOrNormalScalar): Right-hand-side operand.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Domain
    /// - Accepts `{self: Usf, scalar: Usf}`.
    /// - Accepts `{self: Usf, scalar: Normal}`.
    /// - Disallowed combinations: none; all domain pairs are accepted.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    fn mul_scalar(&self, _rhs: UsfOrNormalScalar) -> Self {
        todo!()
    }

    /// Divides each vector component by scalar.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (UsfOrNormalScalar): Right-hand-side operand.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Domain
    /// - Accepts `{self: Usf, scalar: Usf}`.
    /// - Accepts `{self: Usf, scalar: Normal}`.
    /// - Disallowed combinations: none; all domain pairs are accepted.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    /// - Panics if `rhs` is zero.
    fn div_scalar(&self, _rhs: UsfOrNormalScalar) -> Self {
        todo!()
    }

    /// Scales this vector by scalar factor.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (UsfOrNormalScalar): Right-hand-side operand.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Domain
    /// - Accepts `{self: Usf, scalar: Usf}`.
    /// - Accepts `{self: Usf, scalar: Normal}`.
    /// - Disallowed combinations: none; all domain pairs are accepted.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    fn scale(&self, _rhs: UsfOrNormalScalar) -> Self {
        todo!()
    }

    /// Returns compile-time dimension value.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - Numeric metadata value (usize).
    fn get_dimension(&self) -> usize {
        todo!()
    }

    /// Returns vector length in requested output mode.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `output_mode` (OutputMode): Output domain/quality projection policy.
    ///
    /// # Returns
    /// - Computed result of type `UsfOrNormalDecimalScalar`.
    ///
    /// # Domain
    /// - Output projection is selected via `output_mode`.
    /// # Panics
    /// - Panics when `output_mode.domain == OutputDomain::Usf` and `output_mode.quality_constraint == OutputQualityConstraint::AllowLossy`, because USF output never uses lossy projection.
    /// - Panics when `output_mode.domain == OutputDomain::Normal` and `output_mode.quality_constraint == OutputQualityConstraint::RequireLossless` but the projection loses precision or range.
    fn get_length(&self, _output_mode: OutputMode) -> UsfOrNormalDecimalScalar {
        todo!()
    }

    /// Returns squared vector length in requested output mode.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `output_mode` (OutputMode): Output domain/quality projection policy.
    ///
    /// # Returns
    /// - Computed result of type `UsfOrNormalDecimalScalar`.
    ///
    /// # Domain
    /// - Output projection is selected via `output_mode`.
    /// # Panics
    /// - Panics when `output_mode.domain == OutputDomain::Usf` and `output_mode.quality_constraint == OutputQualityConstraint::AllowLossy`, because USF output never uses lossy projection.
    /// - Panics when `output_mode.domain == OutputDomain::Normal` and `output_mode.quality_constraint == OutputQualityConstraint::RequireLossless` but the projection loses precision or range.
    fn get_length_squared(&self, _output_mode: OutputMode) -> UsfOrNormalDecimalScalar {
        todo!()
    }

    /// Returns vector component at index in requested output mode.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `index` (usize): Zero-based index.
    /// - `output_mode` (OutputMode): Output domain/quality projection policy.
    ///
    /// # Returns
    /// - Computed result of type `UsfOrNormalScalar`.
    ///
    /// # Domain
    /// - Output projection is selected via `output_mode`.
    /// # Panics
    /// - Panics if `index` is out of bounds.
    /// - Panics when `output_mode.domain == OutputDomain::Usf` and `output_mode.quality_constraint == OutputQualityConstraint::AllowLossy`, because USF output never uses lossy projection.
    /// - Panics when `output_mode.domain == OutputDomain::Normal` and `output_mode.quality_constraint == OutputQualityConstraint::RequireLossless` but component projection loses precision or range.
    fn get_vector_component(&self, _index: usize, _output_mode: OutputMode) -> UsfOrNormalScalar {
        todo!()
    }

    /// Sets vector component at index.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `index` (usize): Zero-based index.
    /// - `value` (UsfOrNormalScalar): Input value for this operation.
    ///
    /// # Returns
    /// - No value; mutates receiver state where applicable.
    ///
    /// # Domain
    /// - Accepts `{value: Usf}` and `{value: Normal}`.
    /// - Disallowed combinations: none; all domain values are accepted.
    /// # Panics
    /// - Panics if `index` is out of bounds.
    /// - Panics if domain selection is invalid for this backend.
    /// - Panics if the vector component is immutable under runtime field mutability policy.
    fn set_vector_component(&mut self, _index: usize, _value: UsfOrNormalScalar) {
        todo!()
    }
}

/// Bridge-only extension point for vector surfaces.
pub trait VectorBridgeOps<const D: usize>: VectorCoreOps<D> {}

/// Field-like vector access contract.
pub trait VectorFieldOps<const D: usize>: VectorCoreOps<D> {}
impl<T, const D: usize> VectorFieldOps<D> for T where T: VectorCoreOps<D> {}

/// 2D-specific component accessors layered on top of generic vector access.
pub trait Vector2dFieldOps: Clone + Sized {
    /// Returns `x` vector component.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `output_mode` (OutputMode): Output domain/quality projection policy.
    ///
    /// # Returns
    /// - Computed result of type `UsfOrNormalScalar`.
    fn get_x(&self, _output_mode: OutputMode) -> UsfOrNormalScalar {
        todo!()
    }

    /// Returns `y` vector component.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `output_mode` (OutputMode): Output domain/quality projection policy.
    ///
    /// # Returns
    /// - Computed result of type `UsfOrNormalScalar`.
    fn get_y(&self, _output_mode: OutputMode) -> UsfOrNormalScalar {
        todo!()
    }

    /// Sets `x` vector component.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `value` (UsfOrNormalScalar): Input value for this operation.
    ///
    /// # Returns
    /// - No value; mutates receiver state where applicable.
    fn set_x(&mut self, _value: UsfOrNormalScalar) {
        todo!()
    }

    /// Sets `y` vector component.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `value` (UsfOrNormalScalar): Input value for this operation.
    ///
    /// # Returns
    /// - No value; mutates receiver state where applicable.
    fn set_y(&mut self, _value: UsfOrNormalScalar) {
        todo!()
    }
}

/// 3D-specific component accessors layered on top of 2D accessors.
pub trait Vector3dFieldOps: Vector2dFieldOps {
    /// Returns `z` vector component.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `output_mode` (OutputMode): Output domain/quality projection policy.
    ///
    /// # Returns
    /// - Computed result of type `UsfOrNormalScalar`.
    fn get_z(&self, _output_mode: OutputMode) -> UsfOrNormalScalar {
        todo!()
    }

    /// Sets `z` vector component.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `value` (UsfOrNormalScalar): Input value for this operation.
    ///
    /// # Returns
    /// - No value; mutates receiver state where applicable.
    fn set_z(&mut self, _value: UsfOrNormalScalar) {
        todo!()
    }
}

/// 4D-specific component accessors layered on top of 3D accessors.
pub trait Vector4dFieldOps: Vector3dFieldOps {
    /// Returns `w` vector component.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `output_mode` (OutputMode): Output domain/quality projection policy.
    ///
    /// # Returns
    /// - Computed result of type `UsfOrNormalScalar`.
    fn get_w(&self, _output_mode: OutputMode) -> UsfOrNormalScalar {
        todo!()
    }

    /// Sets `w` vector component.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `value` (UsfOrNormalScalar): Input value for this operation.
    ///
    /// # Returns
    /// - No value; mutates receiver state where applicable.
    fn set_w(&mut self, _value: UsfOrNormalScalar) {
        todo!()
    }
}

/// 2D vector operations that are not dimension-agnostic (perp, polar).
pub trait Vector2dCoreOps: Vector2dFieldOps + VectorCoreOps<2> {
    /// Returns 90° CCW perpendicular.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn perp_ccw(&self) -> Self {
        todo!()
    }

    /// Returns 90° CW perpendicular.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn perp_cw(&self) -> Self {
        todo!()
    }

    /// Returns 2D perpendicular dot product.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (Rhs): Right-hand-side operand.
    /// - `output_mode` (OutputMode): Output domain/quality projection policy.
    ///
    /// # Returns
    /// - Computed result of type `UsfOrNormalDecimalScalar`.
    ///
    /// # Panics
    /// - Panics when `output_mode` requests an unsupported projection policy.
    fn perp_dot<Rhs: VectorContract<2>>(&self, _rhs: Rhs, _output_mode: OutputMode) -> UsfOrNormalDecimalScalar {
        todo!()
    }

    /// Builds unit direction from angle.
    ///
    /// # Parameters
    /// - `angle_rad` (UsfOrNormalDecimalScalar): Angle in radians.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn from_angle(_angle_rad: UsfOrNormalDecimalScalar) -> Self {
        todo!()
    }

    /// Returns polar angle.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `output_mode` (OutputMode): Output domain/quality projection policy.
    ///
    /// # Returns
    /// - Computed result of type `UsfOrNormalDecimalScalar`.
    ///
    /// # Panics
    /// - Panics when `output_mode` requests an unsupported projection policy.
    fn angle(&self, _output_mode: OutputMode) -> UsfOrNormalDecimalScalar {
        todo!()
    }

    /// Rotates by angle.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `angle_rad` (UsfOrNormalDecimalScalar): Angle in radians.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn rotate(&self, _angle_rad: UsfOrNormalDecimalScalar) -> Self {
        todo!()
    }

    /// Converts to `(radius, angle)`.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `output_mode` (OutputMode): Output domain/quality projection policy.
    ///
    /// # Returns
    /// - Tuple result of type `(UsfOrNormalDecimalScalar, UsfOrNormalDecimalScalar)`.
    ///
    /// # Panics
    /// - Panics when `output_mode` requests an unsupported projection policy.
    fn to_polar(&self, _output_mode: OutputMode) -> (UsfOrNormalDecimalScalar, UsfOrNormalDecimalScalar) {
        todo!()
    }

    /// Builds from `(radius, angle)`.
    ///
    /// # Parameters
    /// - `radius` (UsfOrNormalDecimalScalar): Radius value.
    /// - `angle_rad` (UsfOrNormalDecimalScalar): Angle in radians.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn from_polar(_radius: UsfOrNormalDecimalScalar, _angle_rad: UsfOrNormalDecimalScalar) -> Self {
        todo!()
    }
}

/// 3D vector operations that are not dimension-agnostic (cross, spherical, axis-angle helpers).
pub trait Vector3dCoreOps: Vector3dFieldOps + VectorCoreOps<3> {
    /// Computes 3D cross product.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (Rhs): Right-hand-side operand.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Domain
    /// - Allowed: `{self: Usf, rhs: Usf}`.
    /// - Disallowed combinations: `{rhs: Normal}` in this concrete `UsfVector<3>` API.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    fn cross<Rhs: VectorContract<3>>(&self, _rhs: Rhs) -> Self {
        todo!()
    }

    /// Computes normalized cross product.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (Rhs): Right-hand-side operand.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn cross_normalized<Rhs: VectorContract<3>>(&self, _rhs: Rhs) -> Self {
        todo!()
    }

    /// Computes scalar triple product.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `b` (B): Secondary operand used by the operation.
    /// - `c` (C): Tertiary operand used by the operation.
    /// - `output_mode` (OutputMode): Output domain/quality projection policy.
    ///
    /// # Returns
    /// - Computed result of type `UsfOrNormalDecimalScalar`.
    ///
    /// # Panics
    /// - Panics when `output_mode` requests an unsupported projection policy.
    fn triple_product<B: VectorContract<3>, C: VectorContract<3>>(&self, _b: B, _c: C, _output_mode: OutputMode) -> UsfOrNormalDecimalScalar {
        todo!()
    }

    /// Projects onto plane.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `plane_normal` (PlaneNormal): Plane normal vector.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn project_on_plane<PlaneNormal: VectorContract<3>>(&self, _plane_normal: PlaneNormal) -> Self {
        todo!()
    }

    /// Reflects on plane.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `plane_normal` (PlaneNormal): Plane normal vector.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn reflect_on_plane<PlaneNormal: VectorContract<3>>(&self, _plane_normal: PlaneNormal) -> Self {
        todo!()
    }

    /// Rotates around axis.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `axis` (Axis): Axis vector.
    /// - `angle_rad` (UsfOrNormalDecimalScalar): Angle in radians.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn rotate_around_axis<Axis: VectorContract<3>>(&self, _axis: Axis, _angle_rad: UsfOrNormalDecimalScalar) -> Self {
        todo!()
    }

    /// Computes signed angle around axis.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (Rhs): Right-hand-side operand.
    /// - `axis` (Axis): Axis vector.
    /// - `output_mode` (OutputMode): Output domain/quality projection policy.
    ///
    /// # Returns
    /// - Computed result of type `UsfOrNormalDecimalScalar`.
    ///
    /// # Panics
    /// - Panics when `_axis` has zero length.
    /// - Panics when `output_mode` requests an unsupported projection policy.
    fn signed_angle<Rhs: VectorContract<3>, Axis: VectorContract<3>>(&self, _rhs: Rhs, _axis: Axis, _output_mode: OutputMode) -> UsfOrNormalDecimalScalar {
        todo!()
    }

    /// Converts to spherical coordinates.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `output_mode` (OutputMode): Output domain/quality projection policy.
    ///
    /// # Returns
    /// - Tuple result of type `(UsfOrNormalDecimalScalar, UsfOrNormalDecimalScalar, UsfOrNormalDecimalScalar)`.
    ///
    /// # Panics
    /// - Panics when `output_mode` requests an unsupported projection policy.
    fn to_spherical(&self, _output_mode: OutputMode) -> (UsfOrNormalDecimalScalar, UsfOrNormalDecimalScalar, UsfOrNormalDecimalScalar) {
        todo!()
    }

    /// Builds from spherical coordinates.
    ///
    /// # Parameters
    /// - `radius` (UsfOrNormalDecimalScalar): Radius value.
    /// - `azimuth` (UsfOrNormalDecimalScalar): Azimuth angle in radians.
    /// - `inclination` (UsfOrNormalDecimalScalar): Inclination angle in radians.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn from_spherical(_radius: UsfOrNormalDecimalScalar, _azimuth: UsfOrNormalDecimalScalar, _inclination: UsfOrNormalDecimalScalar) -> Self {
        todo!()
    }
}

/// 4D vector operations including homogeneous-coordinate helpers.
pub trait Vector4dCoreOps: Vector4dFieldOps + VectorCoreOps<4> {
    /// Builds from `(xyz, w)`.
    ///
    /// # Parameters
    /// - `xyz` (UsfOrNormalVector<3>): XYZ vector component.
    /// - `w` (UsfOrNormalScalar): W component value.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn from_vec3_w(_xyz: UsfOrNormalVector<3>, _w: UsfOrNormalScalar) -> Self {
        todo!()
    }

    /// Returns xyz projection.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `output_mode` (OutputMode): Output domain/quality projection policy.
    ///
    /// # Returns
    /// - Computed result of type `UsfOrNormalVector<3>`.
    fn xyz(&self, _output_mode: OutputMode) -> UsfOrNormalVector<3> {
        todo!()
    }

    /// Returns copy with replaced `w`.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `w` (UsfOrNormalScalar): W component value.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn with_w(&self, _w: UsfOrNormalScalar) -> Self {
        todo!()
    }

    /// Computes 3D-style dot product over xyz vector components.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (Rhs): Right-hand-side operand.
    /// - `output_mode` (OutputMode): Output domain/quality projection policy.
    ///
    /// # Returns
    /// - Computed result of type `UsfOrNormalDecimalScalar`.
    ///
    /// # Panics
    /// - Panics when `output_mode` requests an unsupported projection policy.
    fn dot3<Rhs: VectorContract<4>>(&self, _rhs: Rhs, _output_mode: OutputMode) -> UsfOrNormalDecimalScalar {
        todo!()
    }
}

/// 4D bridge operations for homogeneous point/direction conversion policies.
pub trait Vector4dBridgeOps: Vector4dCoreOps + VectorBridgeOps<4> {
    /// Classifies homogeneous `w` vector component.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - Computed result of type `HomogeneousWState`.
    fn classify_homogeneous_w(&self) -> HomogeneousWState {
        todo!()
    }

    /// Dehomogenizes into a strict 3D point representation.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - Computed result of type `HomogeneousPointOrDirection<UsfOrNormalVector<3>>`.
    ///
    /// # Panics
    /// - Panics when `w == 0` (direction/point-at-infinity) under strict point dehomogenization.
    /// - Panics when `w` is non-finite under strict point dehomogenization mode.
    fn homogenized_to_vec3_strict(&self) -> HomogeneousPointOrDirection<UsfOrNormalVector<3>> {
        todo!()
    }

    /// Non-panicking dehomogenization policy:
    /// - if `w == 0` or non-finite, treat value as direction branch.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - Computed result of type `HomogeneousPointOrDirection<UsfOrNormalVector<3>>`.
    fn homogenized_to_vec3_or_direction(&self) -> HomogeneousPointOrDirection<UsfOrNormalVector<3>> {
        todo!()
    }

    /// Returns `(xyz, is_direction)` where `is_direction == true` means `w == 0` or non-finite
    /// under the configured classification mode.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - Tuple result of type `(UsfOrNormalVector<3>, bool)`.
    fn dehomogenize_point_vs_direction(&self) -> (UsfOrNormalVector<3>, bool) {
        todo!()
    }
}

/// Full generic vector contract (dimension `D`).
pub trait VectorContract<const D: usize>: VectorCoreOps<D> + VectorFieldOps<D> + VectorBridgeOps<D> {}
impl<T, const D: usize> VectorContract<D> for T where T: VectorCoreOps<D> + VectorFieldOps<D> + VectorBridgeOps<D> {}

/// Full 2D vector contract.
pub trait Vector2dContract: Vector2dCoreOps + Vector2dFieldOps {}
impl<T> Vector2dContract for T where T: Vector2dCoreOps + Vector2dFieldOps {}

/// Full 3D vector contract.
pub trait Vector3dContract: Vector3dCoreOps + Vector3dFieldOps {}
impl<T> Vector3dContract for T where T: Vector3dCoreOps + Vector3dFieldOps {}

/// Full 4D vector contract.
pub trait Vector4dContract: Vector4dCoreOps + Vector4dFieldOps + Vector4dBridgeOps {}
impl<T> Vector4dContract for T where T: Vector4dCoreOps + Vector4dFieldOps + Vector4dBridgeOps {}
