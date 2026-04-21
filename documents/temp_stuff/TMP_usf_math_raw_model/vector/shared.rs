#![allow(dead_code)]

//! Shared vector contracts for both USF and normal vector surfaces.
//!
//! Facade-first rule:
//! - These traits are semantic contracts, not final end-user APIs.
//! - Rhai exposure should happen through monomorphized facades/bindings.
//!
//! Kind/repr mechanism:
//! - Mixed-repr vector/scalar operands are represented with `UsfOrNormal*` aliases.
//! - Output projection requests are represented with `OpMode`.
//! - Invalid kind/repr combinations are guarded by panic-fast checks.
//! - Operation-intrinsic mode variance should be expressed with `op_policy::OpPolicy<T>`.
//!
//! Method doc schema:
//! - Summary line: describe intent and core working principle.
//! - `# Parameters`: document each argument and expected role.
//! - `# Returns`: document the returned value and shape/branch semantics.
//! - Optional `# Repr` section for mixed-repr semantics.
//! - Optional `# Panics` section for runtime guard clauses and undefined math states.

use super::super::op_mode::OpMode;
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
/// - Methods use mixed-repr aliases for inputs and allow generic output codomains where needed.
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
/// use crate::usf::math::op_mode::OpMode;
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
/// fn length_into_fractional<V, OutFractional>(v: &V, mode: OpMode) -> OutFractional
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
    /// # Repr
    /// - `value` can come from either branch of `UsfOrNormalScalar`.
    /// # Panics
    /// - Panics if this backend does not support the selected repr branch.
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
    /// # Repr
    /// - Components can come from either branch of `UsfOrNormalScalar`.
    /// # Panics
    /// - Panics if this backend does not support the selected repr branch.
    /// - Panics if `D < 2` is rejected by runtime validation.
    fn from_vector_components(_vector_components: [UsfOrNormalScalar; D]) -> Self {
        todo!()
    }

    /// Returns vector component array representation.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `op_mode` (OpMode): Output kind/repr projection policy.
    ///
    /// # Returns
    /// - Fixed-size array result of type `[OutScalar; D]`, projected according to `op_mode`.
    ///
    /// # Repr
    /// - Output projection policy is selected via `op_mode`.
    fn to_vector_components<OutScalar: ScalarContract>(&self, _op_mode: OpMode) -> [OutScalar; D] {
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
    /// # Repr
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

    /// Adds a vector in either repr.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (V): Right-hand-side operand.
    ///
    /// # Returns
    /// - Vector result converted into `Out`.
    ///
    /// # Repr
    /// - `rhs` can use any repr branch exposed by `V: VectorContract<D>`.
    /// # Panics
    /// - Panics if this backend cannot evaluate the selected operand/output repr branches.
    fn add<V: VectorContract<D>, Out: VectorContract<D>>(&self, _rhs: V) -> Out {
        todo!()
    }

    /// Subtracts a vector in either repr.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (V): Right-hand-side operand.
    ///
    /// # Returns
    /// - Vector result converted into `Out`.
    ///
    /// # Repr
    /// - `rhs` can use any repr branch exposed by `V: VectorContract<D>`.
    /// # Panics
    /// - Panics if this backend cannot evaluate the selected operand/output repr branches.
    fn sub<V: VectorContract<D>, Out: VectorContract<D>>(&self, _rhs: V) -> Out {
        todo!()
    }

    /// Multiplies component-wise by a vector in either repr.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (V): Right-hand-side operand.
    ///
    /// # Returns
    /// - Vector result converted into `Out`.
    ///
    /// # Repr
    /// - `rhs` can use any repr branch exposed by `V: VectorContract<D>`.
    /// # Panics
    /// - Panics if this backend cannot evaluate the selected operand/output repr branches.
    fn component_mul<V: VectorContract<D>, Out: VectorContract<D>>(&self, _rhs: V) -> Out {
        todo!()
    }

    /// Divides component-wise by a vector in either repr.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (V): Right-hand-side operand.
    ///
    /// # Returns
    /// - Vector result converted into `Out`.
    ///
    /// # Repr
    /// - `rhs` can use any repr branch exposed by `V: VectorContract<D>`.
    /// # Panics
    /// - Panics if this backend cannot evaluate the selected operand/output repr branches.
    /// - Panics if any corresponding vector component in `rhs` is zero.
    fn component_div<V: VectorContract<D>, Out: VectorContract<D>>(&self, _rhs: V) -> Out {
        todo!()
    }

    /// Computes component-wise remainder with a vector in either repr.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (V): Right-hand-side operand.
    ///
    /// # Returns
    /// - Vector result converted into `Out`.
    ///
    /// # Repr
    /// - `rhs` can use any repr branch exposed by `V: VectorContract<D>`.
    /// # Panics
    /// - Panics if this backend cannot evaluate the selected operand/output repr branches.
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
    /// # Repr
    /// - `rhs` can use any repr branch exposed by `V: VectorContract<D>`.
    /// # Panics
    /// - Panics if this backend cannot evaluate the selected operand/output repr branches.
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
    /// # Repr
    /// - `rhs` can use any repr branch exposed by `V: VectorContract<D>`.
    /// # Panics
    /// - Panics if this backend cannot evaluate the selected operand/output repr branches.
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
    /// # Repr
    /// - `lo` and `hi` can use any repr branch exposed by `V: VectorContract<D>`.
    /// # Panics
    /// - Panics if this backend cannot evaluate the selected operand/output repr branches.
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
    /// # Repr
    /// - `rhs` can use any repr branch exposed by `V: VectorContract<D>`.
    /// - `t` can use either branch of `UsfOrNormalFractionalScalar`.
    /// # Panics
    /// - Panics if this backend cannot evaluate the selected operand/output repr branches.
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
    /// # Repr
    /// - `rhs` can use any repr branch exposed by `V: VectorContract<D>`.
    /// - `t` can use either branch of `UsfOrNormalFractionalScalar`.
    /// # Panics
    /// - Panics if this backend cannot evaluate the selected operand/output repr branches.
    fn smoothstep<V: VectorContract<D>, Out: VectorContract<D>>(&self, _rhs: V, _t: UsfOrNormalFractionalScalar) -> Out {
        todo!()
    }

    /// Computes dot product in requested op mode.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (V): Right-hand-side operand.
    /// - `op_mode` (OpMode): Output kind/repr projection policy.
    ///
    /// # Returns
    /// - Dot-product scalar converted into `OutFractional`.
    ///
    /// # Repr
    /// - `rhs` can use any repr branch exposed by `V: VectorContract<D>`.
    /// # Panics
    /// - Panics if repr selection is invalid for this backend.
    fn dot<V: VectorContract<D>, OutFractional: FractionalScalarContract>(&self, _rhs: V, _op_mode: OpMode) -> OutFractional {
        todo!()
    }

    /// Computes Euclidean distance in requested op mode.
    /// Output behavior:
    /// - Computes using canonical USF working precision.
    /// - Projects the result into `op_mode.repr`.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (V): Right-hand-side operand.
    /// - `op_mode` (OpMode): Output kind/repr projection policy.
    ///
    /// # Returns
    /// - Euclidean distance converted into `OutFractional`.
    ///
    /// # Repr
    /// - `rhs` can use any repr branch exposed by `V: VectorContract<D>`.
    /// # Panics
    /// - Panics if repr selection is invalid for this backend.
    fn distance<V: VectorContract<D>, OutFractional: FractionalScalarContract>(&self, _rhs: V, _op_mode: OpMode) -> OutFractional {
        todo!()
    }

    /// Computes angle between vectors in requested op mode.
    /// Output behavior:
    /// - Computes using canonical USF working precision.
    /// - Projects the result into `op_mode.repr`.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (V): Right-hand-side operand.
    /// - `op_mode` (OpMode): Output kind/repr projection policy.
    ///
    /// # Returns
    /// - Angle value converted into `OutFractional`.
    ///
    /// # Repr
    /// - `rhs` can use any repr branch exposed by `V: VectorContract<D>`.
    /// # Panics
    /// - Panics if repr selection is invalid for this backend.
    /// - Panics if either vector has zero length.
    fn angle_between<V: VectorContract<D>, OutFractional: FractionalScalarContract>(&self, _rhs: V, _op_mode: OpMode) -> OutFractional {
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
    /// # Repr
    /// - `onto` can use any repr branch exposed by `V: VectorContract<D>`.
    /// # Panics
    /// - Panics if this backend cannot evaluate the selected operand/output repr branches.
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
    /// # Repr
    /// - `onto` can use any repr branch exposed by `V: VectorContract<D>`.
    /// # Panics
    /// - Panics if this backend cannot evaluate the selected operand/output repr branches.
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
    /// # Repr
    /// - `normal` can use any repr branch exposed by `V: VectorContract<D>`.
    /// # Panics
    /// - Panics if this backend cannot evaluate the selected operand/output repr branches.
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
    /// # Repr
    /// - `b` and `c` can use any repr branch exposed by `V: VectorContract<D>`.
    /// # Panics
    /// - Panics if this backend cannot evaluate the selected operand/output repr branches.
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
    /// # Repr
    /// - `rhs` can use either branch of `UsfOrNormalScalar`.
    /// # Panics
    /// - Panics if this backend cannot evaluate the selected operand/output repr branches.
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
    /// # Repr
    /// - `rhs` can use either branch of `UsfOrNormalScalar`.
    /// # Panics
    /// - Panics if this backend cannot evaluate the selected operand/output repr branches.
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
    /// # Repr
    /// - `rhs` can use either branch of `UsfOrNormalScalar`.
    /// # Panics
    /// - Panics if this backend cannot evaluate the selected operand/output repr branches.
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
    /// # Repr
    /// - `rhs` can use either branch of `UsfOrNormalScalar`.
    /// # Panics
    /// - Panics if this backend cannot evaluate the selected operand/output repr branches.
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
    /// # Repr
    /// - `rhs` can use either branch of `UsfOrNormalScalar`.
    /// # Panics
    /// - Panics if this backend cannot evaluate the selected operand/output repr branches.
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

    /// Returns vector length in requested op mode.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `op_mode` (OpMode): Output kind/repr projection policy.
    ///
    /// # Returns
    /// - Length value converted into `OutFractional`.
    ///
    /// # Repr
    /// - Output projection is selected via `op_mode`.
    fn get_length<OutFractional: FractionalScalarContract>(&self, _op_mode: OpMode) -> OutFractional {
        todo!()
    }

    /// Returns squared vector length in requested op mode.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `op_mode` (OpMode): Output kind/repr projection policy.
    ///
    /// # Returns
    /// - Squared-length value converted into `OutFractional`.
    ///
    /// # Repr
    /// - Output projection is selected via `op_mode`.
    fn get_length_squared<OutFractional: FractionalScalarContract>(&self, _op_mode: OpMode) -> OutFractional {
        todo!()
    }

    /// Returns vector component at index in requested op mode.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `index` (usize): Zero-based index.
    /// - `op_mode` (OpMode): Output kind/repr projection policy.
    ///
    /// # Returns
    /// - Component value converted into `OutScalar`.
    ///
    /// # Repr
    /// - Output projection is selected via `op_mode`.
    /// # Panics
    /// - Panics if `index` is out of bounds.
    fn get_vector_component<OutScalar: ScalarContract>(&self, _index: usize, _op_mode: OpMode) -> OutScalar {
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
    /// # Repr
    /// - Accepts `{value: Usf}` and `{value: Normal}`.
    /// - Disallowed combinations: none; all repr values are accepted.
    /// # Panics
    /// - Panics if `index` is out of bounds.
    /// - Panics if repr selection is invalid for this backend.
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
    /// - `op_mode` (OpMode): Output kind/repr projection policy.
    ///
    /// # Returns
    /// - `x` component converted into `OutScalar`.
    fn get_x<OutScalar: ScalarContract>(&self, _op_mode: OpMode) -> OutScalar {
        todo!()
    }

    /// Returns `y` vector component.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `op_mode` (OpMode): Output kind/repr projection policy.
    ///
    /// # Returns
    /// - `y` component converted into `OutScalar`.
    fn get_y<OutScalar: ScalarContract>(&self, _op_mode: OpMode) -> OutScalar {
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
    /// - `op_mode` (OpMode): Output kind/repr projection policy.
    ///
    /// # Returns
    /// - `z` component converted into `OutScalar`.
    fn get_z<OutScalar: ScalarContract>(&self, _op_mode: OpMode) -> OutScalar {
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
    /// - `op_mode` (OpMode): Output kind/repr projection policy.
    ///
    /// # Returns
    /// - `w` component converted into `OutScalar`.
    fn get_w<OutScalar: ScalarContract>(&self, _op_mode: OpMode) -> OutScalar {
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
    /// - `op_mode` (OpMode): Output kind/repr projection policy.
    ///
    /// # Returns
    /// - Perpendicular-dot scalar converted into `OutFractional`.
    ///
    /// # Panics
    /// - Panics when `op_mode` requests an unsupported kind/repr projection.
    fn perp_dot<Rhs: VectorContract<2>, OutFractional: FractionalScalarContract>(&self, _rhs: Rhs, _op_mode: OpMode) -> OutFractional {
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
    /// - `op_mode` (OpMode): Output kind/repr projection policy.
    ///
    /// # Returns
    /// - Polar angle converted into `OutFractional`.
    ///
    /// # Panics
    /// - Panics when `op_mode` requests an unsupported kind/repr projection.
    fn angle<OutFractional: FractionalScalarContract>(&self, _op_mode: OpMode) -> OutFractional {
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
    /// - `op_mode` (OpMode): Output kind/repr projection policy.
    ///
    /// # Returns
    /// - Polar tuple `(radius, angle)` with both entries converted into `OutFractional`.
    ///
    /// # Panics
    /// - Panics when `op_mode` requests an unsupported kind/repr projection.
    fn to_polar<OutFractional: FractionalScalarContract>(&self, _op_mode: OpMode) -> (OutFractional, OutFractional) {
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
    /// # Repr
    /// - `rhs` can use any repr branch exposed by `Rhs: VectorContract<3>`.
    /// # Panics
    /// - Panics if this backend cannot evaluate the selected operand/output repr branches.
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
    /// - `op_mode` (OpMode): Output kind/repr projection policy.
    ///
    /// # Returns
    /// - Triple-product scalar converted into `OutFractional`.
    ///
    /// # Panics
    /// - Panics when `op_mode` requests an unsupported kind/repr projection.
    fn triple_product<B: VectorContract<3>, C: VectorContract<3>, OutFractional: FractionalScalarContract>(
        &self,
        _b: B,
        _c: C,
        _op_mode: OpMode,
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
    /// - `op_mode` (OpMode): Output kind/repr projection policy.
    ///
    /// # Returns
    /// - Signed angle converted into `OutFractional`.
    ///
    /// # Panics
    /// - Panics when `_axis` has zero length.
    /// - Panics when `op_mode` requests an unsupported kind/repr projection.
    fn signed_angle<Rhs: VectorContract<3>, Axis: VectorContract<3>, OutFractional: FractionalScalarContract>(
        &self,
        _rhs: Rhs,
        _axis: Axis,
        _op_mode: OpMode,
    ) -> OutFractional {
        todo!()
    }

    /// Converts to spherical coordinates.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `op_mode` (OpMode): Output kind/repr projection policy.
    ///
    /// # Returns
    /// - Spherical tuple `(radius, azimuth, inclination)` with each entry converted into `OutFractional`.
    ///
    /// # Panics
    /// - Panics when `op_mode` requests an unsupported kind/repr projection.
    fn to_spherical<OutFractional: FractionalScalarContract>(&self, _op_mode: OpMode) -> (OutFractional, OutFractional, OutFractional) {
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
    /// - `op_mode` (OpMode): Output kind/repr projection policy.
    ///
    /// # Returns
    /// - XYZ slice converted into `Out`.
    fn xyz<Out: VectorContract<3>>(&self, _op_mode: OpMode) -> Out {
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
    /// - `op_mode` (OpMode): Output kind/repr projection policy.
    ///
    /// # Returns
    /// - XYZ-only dot-product scalar converted into `OutFractional`.
    ///
    /// # Panics
    /// - Panics when `op_mode` requests an unsupported kind/repr projection.
    fn dot3<Rhs: VectorContract<4>, OutFractional: FractionalScalarContract>(&self, _rhs: Rhs, _op_mode: OpMode) -> OutFractional {
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
