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
use super::super::scalar::aliases::{UsfOrNormalFractionalScalar, UsfOrNormalScalar};
use super::super::scalar::shared::{FractionalScalarContract, ScalarContract};
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
/// - Methods use mixed-domain aliases for inputs and allow generic output codomains where needed.
/// - Fractional-return operations constrain output with `FractionalScalarContract` to preserve
///   fractional-capable semantics without forcing a value to be non-integer at runtime.
/// # Usage
/// - Implement this trait on concrete vector carriers (`Usf`, `Normal`, or other backends).
/// - Use `VectorContract<D>` bounds when generic call sites need vector core+field+bridge behavior.
/// - Facade layers monomorphize valid permutations and hide unsupported ones.
///
/// # Examples
/// ```ignore
/// use crate::usf::math::vector::shared::{Vector3dCoreOps, VectorContract};
/// use crate::usf::math::scalar::shared::FractionalScalarContract;
/// use crate::usf::math::aliases::OutputMode;
///
/// fn cross_into_out<V, Rhs, Out>(lhs: &V, rhs: Rhs) -> Out
/// where
///     V: Vector3dCoreOps,
///     Rhs: VectorContract<3>,
///     Out: VectorContract<3>,
/// {
///     lhs.cross::<Rhs, Out>(rhs)
/// }
///
/// fn length_into_fractional<V, OutFractional>(v: &V, mode: OutputMode) -> OutFractional
/// where
///     V: VectorContract<3>,
///     OutFractional: FractionalScalarContract,
/// {
///     v.get_length::<OutFractional>(mode)
/// }
/// ```
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

    /// Returns vector with all components set to `value`.
    ///
    /// # Parameters
    /// - `value` (UsfOrNormalScalar): Input value for this operation.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Domain
    /// - `value` can come from either branch of `UsfOrNormalScalar`.
    /// # Panics
    /// - Panics if this backend does not support the selected domain branch.
    fn splat(_value: UsfOrNormalScalar) -> Self {
        todo!()
    }

    /// Builds vector from a component array.
    ///
    /// # Parameters
    /// - `vector_components` ([UsfOrNormalScalar; D]): Vector component payload.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Domain
    /// - Components can come from either branch of `UsfOrNormalScalar`.
    /// # Panics
    /// - Panics if this backend does not support the selected domain branch.
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
    /// - Fixed-size array result of type `[OutScalar; D]`, projected according to `output_mode`.
    ///
    /// # Domain
    /// - Output projection policy is selected via `output_mode`.
    fn to_vector_components<OutScalar: ScalarContract>(&self, _output_mode: OutputMode) -> [OutScalar; D] {
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
    /// - Unary operation on the current backend's representation.
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
    /// - Vector result converted into `Out`.
    ///
    /// # Domain
    /// - `rhs` can use any domain branch exposed by `V: VectorContract<D>`.
    /// # Panics
    /// - Panics if this backend cannot evaluate the selected operand/output domains.
    fn add<V: VectorContract<D>, Out: VectorContract<D>>(&self, _rhs: V) -> Out {
        todo!()
    }

    /// Subtracts a vector in either domain.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (V): Right-hand-side operand.
    ///
    /// # Returns
    /// - Vector result converted into `Out`.
    ///
    /// # Domain
    /// - `rhs` can use any domain branch exposed by `V: VectorContract<D>`.
    /// # Panics
    /// - Panics if this backend cannot evaluate the selected operand/output domains.
    fn sub<V: VectorContract<D>, Out: VectorContract<D>>(&self, _rhs: V) -> Out {
        todo!()
    }

    /// Multiplies component-wise by a vector in either domain.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (V): Right-hand-side operand.
    ///
    /// # Returns
    /// - Vector result converted into `Out`.
    ///
    /// # Domain
    /// - `rhs` can use any domain branch exposed by `V: VectorContract<D>`.
    /// # Panics
    /// - Panics if this backend cannot evaluate the selected operand/output domains.
    fn component_mul<V: VectorContract<D>, Out: VectorContract<D>>(&self, _rhs: V) -> Out {
        todo!()
    }

    /// Divides component-wise by a vector in either domain.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (V): Right-hand-side operand.
    ///
    /// # Returns
    /// - Vector result converted into `Out`.
    ///
    /// # Domain
    /// - `rhs` can use any domain branch exposed by `V: VectorContract<D>`.
    /// # Panics
    /// - Panics if this backend cannot evaluate the selected operand/output domains.
    /// - Panics if any corresponding vector component in `rhs` is zero.
    fn component_div<V: VectorContract<D>, Out: VectorContract<D>>(&self, _rhs: V) -> Out {
        todo!()
    }

    /// Computes component-wise remainder with a vector in either domain.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (V): Right-hand-side operand.
    ///
    /// # Returns
    /// - Vector result converted into `Out`.
    ///
    /// # Domain
    /// - `rhs` can use any domain branch exposed by `V: VectorContract<D>`.
    /// # Panics
    /// - Panics if this backend cannot evaluate the selected operand/output domains.
    /// - Panics if any corresponding vector component in `rhs` is zero.
    fn component_rem<V: VectorContract<D>, Out: VectorContract<D>>(&self, _rhs: V) -> Out {
        todo!()
    }

    /// Returns component-wise minimum.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (V): Right-hand-side operand.
    ///
    /// # Returns
    /// - Vector result converted into `Out`.
    ///
    /// # Domain
    /// - `rhs` can use any domain branch exposed by `V: VectorContract<D>`.
    /// # Panics
    /// - Panics if this backend cannot evaluate the selected operand/output domains.
    fn min<V: VectorContract<D>, Out: VectorContract<D>>(&self, _rhs: V) -> Out {
        todo!()
    }

    /// Returns component-wise maximum.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (V): Right-hand-side operand.
    ///
    /// # Returns
    /// - Vector result converted into `Out`.
    ///
    /// # Domain
    /// - `rhs` can use any domain branch exposed by `V: VectorContract<D>`.
    /// # Panics
    /// - Panics if this backend cannot evaluate the selected operand/output domains.
    fn max<V: VectorContract<D>, Out: VectorContract<D>>(&self, _rhs: V) -> Out {
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
    /// - Vector result converted into `Out`.
    ///
    /// # Domain
    /// - `lo` and `hi` can use any domain branch exposed by `V: VectorContract<D>`.
    /// # Panics
    /// - Panics if this backend cannot evaluate the selected operand/output domains.
    /// - Panics if any vector component has `lo > hi`.
    fn clamp<V: VectorContract<D>, Out: VectorContract<D>>(&self, _lo: V, _hi: V) -> Out {
        todo!()
    }

    /// Performs linear interpolation.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (V): Right-hand-side operand.
    /// - `t` (UsfOrNormalFractionalScalar): Interpolation factor.
    ///
    /// # Returns
    /// - Interpolated vector converted into `Out`.
    ///
    /// # Domain
    /// - `rhs` can use any domain branch exposed by `V: VectorContract<D>`.
    /// - `t` can use either branch of `UsfOrNormalFractionalScalar`.
    /// # Panics
    /// - Panics if this backend cannot evaluate the selected operand/output domains.
    fn lerp<V: VectorContract<D>, Out: VectorContract<D>>(&self, _rhs: V, _t: UsfOrNormalFractionalScalar) -> Out {
        todo!()
    }

    /// Performs smoothstep interpolation.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (V): Right-hand-side operand.
    /// - `t` (UsfOrNormalFractionalScalar): Interpolation factor.
    ///
    /// # Returns
    /// - Smoothed interpolation result converted into `Out`.
    ///
    /// # Domain
    /// - `rhs` can use any domain branch exposed by `V: VectorContract<D>`.
    /// - `t` can use either branch of `UsfOrNormalFractionalScalar`.
    /// # Panics
    /// - Panics if this backend cannot evaluate the selected operand/output domains.
    fn smoothstep<V: VectorContract<D>, Out: VectorContract<D>>(&self, _rhs: V, _t: UsfOrNormalFractionalScalar) -> Out {
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
    /// - Dot-product scalar converted into `OutFractional`.
    ///
    /// # Domain
    /// - `rhs` can use any domain branch exposed by `V: VectorContract<D>`.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    /// - Panics when `output_mode.domain == OutputDomain::Usf` and `output_mode.quality_constraint == OutputQualityConstraint::AllowLossy`, because USF output never uses lossy projection.
    /// - Panics when `output_mode.domain == OutputDomain::Normal` and `output_mode.quality_constraint == OutputQualityConstraint::RequireLossless` but the projection loses precision or range.
    fn dot<V: VectorContract<D>, OutFractional: FractionalScalarContract>(&self, _rhs: V, _output_mode: OutputMode) -> OutFractional {
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
    /// - Euclidean distance converted into `OutFractional`.
    ///
    /// # Domain
    /// - `rhs` can use any domain branch exposed by `V: VectorContract<D>`.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    /// - Panics when `output_mode.domain == OutputDomain::Usf` and `output_mode.quality_constraint == OutputQualityConstraint::AllowLossy`, because USF output never uses lossy projection.
    /// - Panics when `output_mode.domain == OutputDomain::Normal` and `output_mode.quality_constraint == OutputQualityConstraint::RequireLossless` but the projection loses precision or range.
    fn distance<V: VectorContract<D>, OutFractional: FractionalScalarContract>(&self, _rhs: V, _output_mode: OutputMode) -> OutFractional {
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
    /// - Angle value converted into `OutFractional`.
    ///
    /// # Domain
    /// - `rhs` can use any domain branch exposed by `V: VectorContract<D>`.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    /// - Panics if either vector has zero length.
    /// - Panics when `output_mode.domain == OutputDomain::Usf` and `output_mode.quality_constraint == OutputQualityConstraint::AllowLossy`, because USF output never uses lossy projection.
    /// - Panics when `output_mode.domain == OutputDomain::Normal` and `output_mode.quality_constraint == OutputQualityConstraint::RequireLossless` but the projection loses precision or range.
    fn angle_between<V: VectorContract<D>, OutFractional: FractionalScalarContract>(&self, _rhs: V, _output_mode: OutputMode) -> OutFractional {
        todo!()
    }

    /// Projects this value onto the provided operand.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `onto` (V): Projection target.
    ///
    /// # Returns
    /// - Projection result converted into `Out`.
    ///
    /// # Domain
    /// - `onto` can use any domain branch exposed by `V: VectorContract<D>`.
    /// # Panics
    /// - Panics if this backend cannot evaluate the selected operand/output domains.
    /// - Panics if `onto` is the zero vector.
    fn project<V: VectorContract<D>, Out: VectorContract<D>>(&self, _onto: V) -> Out {
        todo!()
    }

    /// Computes the rejection of this value from the provided operand.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `onto` (V): Projection target.
    ///
    /// # Returns
    /// - Rejection result converted into `Out`.
    ///
    /// # Domain
    /// - `onto` can use any domain branch exposed by `V: VectorContract<D>`.
    /// # Panics
    /// - Panics if this backend cannot evaluate the selected operand/output domains.
    /// - Panics if `onto` is the zero vector.
    fn reject<V: VectorContract<D>, Out: VectorContract<D>>(&self, _onto: V) -> Out {
        todo!()
    }

    /// Reflects this value around the provided normal.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `normal` (V): Normal vector.
    ///
    /// # Returns
    /// - Reflection result converted into `Out`.
    ///
    /// # Domain
    /// - `normal` can use any domain branch exposed by `V: VectorContract<D>`.
    /// # Panics
    /// - Panics if this backend cannot evaluate the selected operand/output domains.
    /// - Panics if `normal` is the zero vector.
    fn reflect<V: VectorContract<D>, Out: VectorContract<D>>(&self, _normal: V) -> Out {
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
    /// - Component-wise fused multiply-add result converted into `Out`.
    ///
    /// # Domain
    /// - `b` and `c` can use any domain branch exposed by `V: VectorContract<D>`.
    /// # Panics
    /// - Panics if this backend cannot evaluate the selected operand/output domains.
    fn fma<V: VectorContract<D>, Out: VectorContract<D>>(&self, _b: V, _c: V) -> Out {
        todo!()
    }

    /// Adds scalar to each vector component.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (UsfOrNormalScalar): Right-hand-side operand.
    ///
    /// # Returns
    /// - Vector result converted into `Out`.
    ///
    /// # Domain
    /// - `rhs` can use either branch of `UsfOrNormalScalar`.
    /// # Panics
    /// - Panics if this backend cannot evaluate the selected operand/output domains.
    fn add_scalar<Out: VectorContract<D>>(&self, _rhs: UsfOrNormalScalar) -> Out {
        todo!()
    }

    /// Subtracts scalar from each vector component.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (UsfOrNormalScalar): Right-hand-side operand.
    ///
    /// # Returns
    /// - Vector result converted into `Out`.
    ///
    /// # Domain
    /// - `rhs` can use either branch of `UsfOrNormalScalar`.
    /// # Panics
    /// - Panics if this backend cannot evaluate the selected operand/output domains.
    fn sub_scalar<Out: VectorContract<D>>(&self, _rhs: UsfOrNormalScalar) -> Out {
        todo!()
    }

    /// Multiplies each vector component by scalar.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (UsfOrNormalScalar): Right-hand-side operand.
    ///
    /// # Returns
    /// - Vector result converted into `Out`.
    ///
    /// # Domain
    /// - `rhs` can use either branch of `UsfOrNormalScalar`.
    /// # Panics
    /// - Panics if this backend cannot evaluate the selected operand/output domains.
    fn mul_scalar<Out: VectorContract<D>>(&self, _rhs: UsfOrNormalScalar) -> Out {
        todo!()
    }

    /// Divides each vector component by scalar.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (UsfOrNormalScalar): Right-hand-side operand.
    ///
    /// # Returns
    /// - Vector result converted into `Out`.
    ///
    /// # Domain
    /// - `rhs` can use either branch of `UsfOrNormalScalar`.
    /// # Panics
    /// - Panics if this backend cannot evaluate the selected operand/output domains.
    /// - Panics if `rhs` is zero.
    fn div_scalar<Out: VectorContract<D>>(&self, _rhs: UsfOrNormalScalar) -> Out {
        todo!()
    }

    /// Scales this vector by scalar factor.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (UsfOrNormalScalar): Right-hand-side operand.
    ///
    /// # Returns
    /// - Vector result converted into `Out`.
    ///
    /// # Domain
    /// - `rhs` can use either branch of `UsfOrNormalScalar`.
    /// # Panics
    /// - Panics if this backend cannot evaluate the selected operand/output domains.
    fn scale<Out: VectorContract<D>>(&self, _rhs: UsfOrNormalScalar) -> Out {
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
    /// - Length value converted into `OutFractional`.
    ///
    /// # Domain
    /// - Output projection is selected via `output_mode`.
    /// # Panics
    /// - Panics when `output_mode.domain == OutputDomain::Usf` and `output_mode.quality_constraint == OutputQualityConstraint::AllowLossy`, because USF output never uses lossy projection.
    /// - Panics when `output_mode.domain == OutputDomain::Normal` and `output_mode.quality_constraint == OutputQualityConstraint::RequireLossless` but the projection loses precision or range.
    fn get_length<OutFractional: FractionalScalarContract>(&self, _output_mode: OutputMode) -> OutFractional {
        todo!()
    }

    /// Returns squared vector length in requested output mode.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `output_mode` (OutputMode): Output domain/quality projection policy.
    ///
    /// # Returns
    /// - Squared-length value converted into `OutFractional`.
    ///
    /// # Domain
    /// - Output projection is selected via `output_mode`.
    /// # Panics
    /// - Panics when `output_mode.domain == OutputDomain::Usf` and `output_mode.quality_constraint == OutputQualityConstraint::AllowLossy`, because USF output never uses lossy projection.
    /// - Panics when `output_mode.domain == OutputDomain::Normal` and `output_mode.quality_constraint == OutputQualityConstraint::RequireLossless` but the projection loses precision or range.
    fn get_length_squared<OutFractional: FractionalScalarContract>(&self, _output_mode: OutputMode) -> OutFractional {
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
    /// - Component value converted into `OutScalar`.
    ///
    /// # Domain
    /// - Output projection is selected via `output_mode`.
    /// # Panics
    /// - Panics if `index` is out of bounds.
    /// - Panics when `output_mode.domain == OutputDomain::Usf` and `output_mode.quality_constraint == OutputQualityConstraint::AllowLossy`, because USF output never uses lossy projection.
    /// - Panics when `output_mode.domain == OutputDomain::Normal` and `output_mode.quality_constraint == OutputQualityConstraint::RequireLossless` but component projection loses precision or range.
    fn get_vector_component<OutScalar: ScalarContract>(&self, _index: usize, _output_mode: OutputMode) -> OutScalar {
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
    /// - `x` component converted into `OutScalar`.
    fn get_x<OutScalar: ScalarContract>(&self, _output_mode: OutputMode) -> OutScalar {
        todo!()
    }

    /// Returns `y` vector component.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `output_mode` (OutputMode): Output domain/quality projection policy.
    ///
    /// # Returns
    /// - `y` component converted into `OutScalar`.
    fn get_y<OutScalar: ScalarContract>(&self, _output_mode: OutputMode) -> OutScalar {
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
    /// - `z` component converted into `OutScalar`.
    fn get_z<OutScalar: ScalarContract>(&self, _output_mode: OutputMode) -> OutScalar {
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
    /// - `w` component converted into `OutScalar`.
    fn get_w<OutScalar: ScalarContract>(&self, _output_mode: OutputMode) -> OutScalar {
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
    /// - Perpendicular-dot scalar converted into `OutFractional`.
    ///
    /// # Panics
    /// - Panics when `output_mode` requests an unsupported projection policy.
    fn perp_dot<Rhs: VectorContract<2>, OutFractional: FractionalScalarContract>(&self, _rhs: Rhs, _output_mode: OutputMode) -> OutFractional {
        todo!()
    }

    /// Builds unit direction from angle.
    ///
    /// # Parameters
    /// - `angle_rad` (UsfOrNormalFractionalScalar): Angle in radians.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn from_angle(_angle_rad: UsfOrNormalFractionalScalar) -> Self {
        todo!()
    }

    /// Returns polar angle.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `output_mode` (OutputMode): Output domain/quality projection policy.
    ///
    /// # Returns
    /// - Polar angle converted into `OutFractional`.
    ///
    /// # Panics
    /// - Panics when `output_mode` requests an unsupported projection policy.
    fn angle<OutFractional: FractionalScalarContract>(&self, _output_mode: OutputMode) -> OutFractional {
        todo!()
    }

    /// Rotates by angle.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `angle_rad` (UsfOrNormalFractionalScalar): Angle in radians.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn rotate(&self, _angle_rad: UsfOrNormalFractionalScalar) -> Self {
        todo!()
    }

    /// Converts to `(radius, angle)`.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `output_mode` (OutputMode): Output domain/quality projection policy.
    ///
    /// # Returns
    /// - Polar tuple `(radius, angle)` with both entries converted into `OutFractional`.
    ///
    /// # Panics
    /// - Panics when `output_mode` requests an unsupported projection policy.
    fn to_polar<OutFractional: FractionalScalarContract>(&self, _output_mode: OutputMode) -> (OutFractional, OutFractional) {
        todo!()
    }

    /// Builds from `(radius, angle)`.
    ///
    /// # Parameters
    /// - `radius` (UsfOrNormalFractionalScalar): Radius value.
    /// - `angle_rad` (UsfOrNormalFractionalScalar): Angle in radians.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn from_polar(_radius: UsfOrNormalFractionalScalar, _angle_rad: UsfOrNormalFractionalScalar) -> Self {
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
    /// - Cross-product vector converted into `Out`.
    ///
    /// # Domain
    /// - `rhs` can use any domain branch exposed by `Rhs: VectorContract<3>`.
    /// # Panics
    /// - Panics if this backend cannot evaluate the selected operand/output domains.
    fn cross<Rhs: VectorContract<3>, Out: VectorContract<3>>(&self, _rhs: Rhs) -> Out {
        todo!()
    }

    /// Computes normalized cross product.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (Rhs): Right-hand-side operand.
    ///
    /// # Returns
    /// - Normalized cross-product vector converted into `Out`.
    fn cross_normalized<Rhs: VectorContract<3>, Out: VectorContract<3>>(&self, _rhs: Rhs) -> Out {
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
    /// - Triple-product scalar converted into `OutFractional`.
    ///
    /// # Panics
    /// - Panics when `output_mode` requests an unsupported projection policy.
    fn triple_product<B: VectorContract<3>, C: VectorContract<3>, OutFractional: FractionalScalarContract>(
        &self,
        _b: B,
        _c: C,
        _output_mode: OutputMode,
    ) -> OutFractional {
        todo!()
    }

    /// Projects onto plane.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `plane_normal` (PlaneNormal): Plane normal vector.
    ///
    /// # Returns
    /// - Plane projection result converted into `Out`.
    fn project_on_plane<PlaneNormal: VectorContract<3>, Out: VectorContract<3>>(&self, _plane_normal: PlaneNormal) -> Out {
        todo!()
    }

    /// Reflects on plane.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `plane_normal` (PlaneNormal): Plane normal vector.
    ///
    /// # Returns
    /// - Plane reflection result converted into `Out`.
    fn reflect_on_plane<PlaneNormal: VectorContract<3>, Out: VectorContract<3>>(&self, _plane_normal: PlaneNormal) -> Out {
        todo!()
    }

    /// Rotates around axis.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `axis` (Axis): Axis vector.
    /// - `angle_rad` (UsfOrNormalFractionalScalar): Angle in radians.
    ///
    /// # Returns
    /// - Rotated vector converted into `Out`.
    fn rotate_around_axis<Axis: VectorContract<3>, Out: VectorContract<3>>(&self, _axis: Axis, _angle_rad: UsfOrNormalFractionalScalar) -> Out {
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
    /// - Signed angle converted into `OutFractional`.
    ///
    /// # Panics
    /// - Panics when `_axis` has zero length.
    /// - Panics when `output_mode` requests an unsupported projection policy.
    fn signed_angle<Rhs: VectorContract<3>, Axis: VectorContract<3>, OutFractional: FractionalScalarContract>(
        &self,
        _rhs: Rhs,
        _axis: Axis,
        _output_mode: OutputMode,
    ) -> OutFractional {
        todo!()
    }

    /// Converts to spherical coordinates.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `output_mode` (OutputMode): Output domain/quality projection policy.
    ///
    /// # Returns
    /// - Spherical tuple `(radius, azimuth, inclination)` with each entry converted into `OutFractional`.
    ///
    /// # Panics
    /// - Panics when `output_mode` requests an unsupported projection policy.
    fn to_spherical<OutFractional: FractionalScalarContract>(&self, _output_mode: OutputMode) -> (OutFractional, OutFractional, OutFractional) {
        todo!()
    }

    /// Builds from spherical coordinates.
    ///
    /// # Parameters
    /// - `radius` (UsfOrNormalFractionalScalar): Radius value.
    /// - `azimuth` (UsfOrNormalFractionalScalar): Azimuth angle in radians.
    /// - `inclination` (UsfOrNormalFractionalScalar): Inclination angle in radians.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn from_spherical(_radius: UsfOrNormalFractionalScalar, _azimuth: UsfOrNormalFractionalScalar, _inclination: UsfOrNormalFractionalScalar) -> Self {
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
    /// - XYZ slice converted into `Out`.
    fn xyz<Out: VectorContract<3>>(&self, _output_mode: OutputMode) -> Out {
        todo!()
    }

    /// Returns copy with replaced `w`.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `w` (UsfOrNormalScalar): W component value.
    ///
    /// # Returns
    /// - Copy with replaced `w`, converted into `Out`.
    fn with_w<Out: VectorContract<4>>(&self, _w: UsfOrNormalScalar) -> Out {
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
    /// - XYZ-only dot-product scalar converted into `OutFractional`.
    ///
    /// # Panics
    /// - Panics when `output_mode` requests an unsupported projection policy.
    fn dot3<Rhs: VectorContract<4>, OutFractional: FractionalScalarContract>(&self, _rhs: Rhs, _output_mode: OutputMode) -> OutFractional {
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
    /// - `Point(OutVec3)` when strict point dehomogenization succeeds.
    /// - `Direction(OutVec3)` only if a backend explicitly encodes strict direction branches instead of panicking.
    ///
    /// # Panics
    /// - Panics when `w == 0` (direction/point-at-infinity) under strict point dehomogenization.
    /// - Panics when `w` is non-finite under strict point dehomogenization mode.
    fn homogenized_to_vec3_strict<OutVec3: VectorContract<3>>(&self) -> HomogeneousPointOrDirection<OutVec3> {
        todo!()
    }

    /// Non-panicking dehomogenization policy:
    /// - if `w == 0` or non-finite, treat value as direction branch.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - `Point(OutVec3)` when `w` denotes a finite point.
    /// - `Direction(OutVec3)` when `w` denotes a direction/point-at-infinity or non-finite branch.
    fn homogenized_to_vec3_or_direction<OutVec3: VectorContract<3>>(&self) -> HomogeneousPointOrDirection<OutVec3> {
        todo!()
    }

    /// Returns `(xyz, is_direction)` where `is_direction == true` means `w == 0` or non-finite
    /// under the configured classification mode.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - Tuple `(OutVec3, bool)` where the boolean indicates direction-branch classification.
    fn dehomogenize_point_vs_direction<OutVec3: VectorContract<3>>(&self) -> (OutVec3, bool) {
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
