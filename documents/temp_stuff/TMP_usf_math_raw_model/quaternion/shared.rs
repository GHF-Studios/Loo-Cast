#![allow(dead_code)]

//! Shared quaternion contracts for USF/normal quaternion surfaces.
//!
//! Facade-first rule:
//! - This contract layer is the semantic source of truth.
//! - Script-facing APIs should be emitted by facade layers over concrete monomorphizations.
//!
//! Kind/repr mechanism:
//! - Mixed-repr operands use `UsfOrNormal*` aliases.
//! - Output projection selection uses `OpMode`.
//! - Invalid kind/repr combinations panic fast.
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
use super::super::vector::aliases::UsfOrNormalVector;
use super::aliases::{UsfOrNormalMat3, UsfOrNormalQuaternion};

/// Quaternion core operations including rotation-specific math and conversion helpers.
/// # Working Principle
/// - Quaternions are treated as rotation-capable carriers with both raw component and rotation ops.
/// - Methods separate component-wise operations from orientation-aware conversions/interpolation.
/// # Usage
/// - Implement this trait for concrete quaternion carriers.
/// - Use `QuaternionContract` bounds where consumers require core+field+bridge behavior.
pub trait QuaternionCoreOps: Clone + Sized {
    /// Returns identity quaternion.
    ///
    /// # Parameters
    /// - None.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn identity() -> Self {
        todo!()
    }

    /// Builds a value from xyzw.
    ///
    /// # Parameters
    /// - `x` (UsfOrNormalFractionalScalar): X component value.
    /// - `y` (UsfOrNormalFractionalScalar): Y component value.
    /// - `z` (UsfOrNormalFractionalScalar): Z component value.
    /// - `w` (UsfOrNormalFractionalScalar): W component value.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Repr
    /// - Allowed: each component independently in `{Usf, Normal}`.
    /// - Disallowed combinations: none; all repr combinations are accepted.
    /// # Panics
    /// - Panics if repr selection is invalid for this backend.
    /// - Panics if the input quaternion cannot be normalized into a valid rotation state.
    fn from_xyzw(_x: UsfOrNormalFractionalScalar, _y: UsfOrNormalFractionalScalar, _z: UsfOrNormalFractionalScalar, _w: UsfOrNormalFractionalScalar) -> Self {
        todo!()
    }

    /// Returns quaternion components in requested op mode.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `op_mode` (OpMode): Output kind/repr projection policy.
    ///
    /// # Returns
    /// - Fixed-size array result of type `[UsfOrNormalFractionalScalar; 4]`.
    ///
    /// # Repr
    /// - Output projection is selected via `op_mode`.
    fn to_xyzw(&self, _op_mode: OpMode) -> [UsfOrNormalFractionalScalar; 4] {
        todo!()
    }

    /// Returns normalized quaternion.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Repr
    /// - Allowed: `{self: Usf}`.
    /// - Disallowed combinations: not applicable in this unary concrete `UsfQuaternion` API.
    /// # Panics
    /// - Panics if quaternion norm is zero.
    fn normalize(&self) -> Self {
        todo!()
    }

    /// Returns quaternion conjugate.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    fn conjugate(&self) -> Self {
        todo!()
    }

    /// Returns inverse quaternion.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Repr
    /// - Allowed: `{self: Usf}`.
    /// - Disallowed combinations: not applicable in this unary concrete `UsfQuaternion` API.
    /// # Panics
    /// - Panics if quaternion norm is zero.
    fn inverse(&self) -> Self {
        todo!()
    }

    /// Adds quaternion from either repr component-wise.
    /// Raw component op: does not guarantee a unit quaternion result.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (UsfOrNormalQuaternion): Right-hand-side operand.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Repr
    /// - Accepts `{self: Usf, rhs: Usf}` and `{self: Usf, rhs: Normal}`.
    /// - Disallowed combinations: none; all repr pairs are accepted.
    /// # Panics
    /// - Panics if repr selection is invalid for this backend.
    fn component_add(&self, _rhs: UsfOrNormalQuaternion) -> Self {
        todo!()
    }

    /// Subtracts quaternion from either repr component-wise.
    /// Raw component op: does not guarantee a unit quaternion result.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (UsfOrNormalQuaternion): Right-hand-side operand.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Repr
    /// - Accepts `{self: Usf, rhs: Usf}` and `{self: Usf, rhs: Normal}`.
    /// - Disallowed combinations: none; all repr pairs are accepted.
    /// # Panics
    /// - Panics if repr selection is invalid for this backend.
    fn component_sub(&self, _rhs: UsfOrNormalQuaternion) -> Self {
        todo!()
    }

    /// Applies Hamilton product with quaternion from either repr.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (UsfOrNormalQuaternion): Right-hand-side operand.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Repr
    /// - Accepts `{self: Usf, rhs: Usf}` and `{self: Usf, rhs: Normal}`.
    /// - Disallowed combinations: none; all repr pairs are accepted.
    /// # Panics
    /// - Panics if repr selection is invalid for this backend.
    fn hamilton_mul(&self, _rhs: UsfOrNormalQuaternion) -> Self {
        todo!()
    }

    /// Applies Hamilton division with quaternion from either repr.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (UsfOrNormalQuaternion): Right-hand-side operand.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Repr
    /// - Accepts `{self: Usf, rhs: Usf}` and `{self: Usf, rhs: Normal}`.
    /// - Disallowed combinations: none; all repr pairs are accepted.
    /// # Panics
    /// - Panics if repr selection is invalid for this backend.
    /// - Panics if `rhs` represents a zero-norm divisor under quaternion division semantics.
    fn hamilton_div(&self, _rhs: UsfOrNormalQuaternion) -> Self {
        todo!()
    }

    /// Multiplies all quaternion components by scalar.
    /// Raw component op: does not guarantee a unit quaternion result.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (UsfOrNormalScalar): Right-hand-side operand.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Repr
    /// - Accepts `{self: Usf, scalar: Usf}` and `{self: Usf, scalar: Normal}`.
    /// - Disallowed combinations: none; all repr pairs are accepted.
    /// # Panics
    /// - Panics if repr selection is invalid for this backend.
    fn component_mul_scalar(&self, _rhs: UsfOrNormalScalar) -> Self {
        todo!()
    }

    /// Divides all quaternion components by scalar.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (UsfOrNormalScalar): Right-hand-side operand.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Repr
    /// - Accepts `{self: Usf, scalar: Usf}` and `{self: Usf, scalar: Normal}`.
    /// - Disallowed combinations: none; all repr pairs are accepted.
    /// # Panics
    /// - Panics if repr selection is invalid for this backend.
    /// - Panics if `rhs` is zero.
    fn component_div_scalar(&self, _rhs: UsfOrNormalScalar) -> Self {
        todo!()
    }

    /// Returns component-wise minimum.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (UsfOrNormalQuaternion): Right-hand-side operand.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Repr
    /// - Accepts `{self: Usf, rhs: Usf}` and `{self: Usf, rhs: Normal}`.
    /// - Disallowed combinations: none; all repr pairs are accepted.
    /// # Panics
    /// - Panics if repr selection is invalid for this backend.
    fn component_min(&self, _rhs: UsfOrNormalQuaternion) -> Self {
        todo!()
    }

    /// Returns component-wise maximum.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (UsfOrNormalQuaternion): Right-hand-side operand.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Repr
    /// - Accepts `{self: Usf, rhs: Usf}` and `{self: Usf, rhs: Normal}`.
    /// - Disallowed combinations: none; all repr pairs are accepted.
    /// # Panics
    /// - Panics if repr selection is invalid for this backend.
    fn component_max(&self, _rhs: UsfOrNormalQuaternion) -> Self {
        todo!()
    }

    /// Clamps each quaternion component to `[lo, hi]`.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `lo` (UsfOrNormalQuaternion): Lower bound.
    /// - `hi` (UsfOrNormalQuaternion): Upper bound.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Repr
    /// - Accepts all `{lo, hi}` pairings in `{Usf, Normal} × {Usf, Normal}`.
    /// - Disallowed combinations: none; all repr pairs are accepted.
    /// # Panics
    /// - Panics if repr selection is invalid for this backend.
    /// - Panics if any quaternion component has `lo > hi`.
    fn component_clamp(&self, _lo: UsfOrNormalQuaternion, _hi: UsfOrNormalQuaternion) -> Self {
        todo!()
    }

    /// Performs per-component linear interpolation (not spherical rotation interpolation).
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (UsfOrNormalQuaternion): Right-hand-side operand.
    /// - `t` (UsfOrNormalFractionalScalar): Interpolation factor.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Repr
    /// - Accepts `rhs` in `{Usf, Normal}`.
    /// - Accepts `t` in `{Usf, Normal}`.
    /// - Disallowed combinations: none; all repr pairs are accepted.
    /// # Panics
    /// - Panics if repr selection is invalid for this backend.
    fn component_lerp(&self, _rhs: UsfOrNormalQuaternion, _t: UsfOrNormalFractionalScalar) -> Self {
        todo!()
    }

    /// Performs per-component smoothstep interpolation (not spherical rotation interpolation).
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (UsfOrNormalQuaternion): Right-hand-side operand.
    /// - `t` (UsfOrNormalFractionalScalar): Interpolation factor.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Repr
    /// - Accepts `rhs` in `{Usf, Normal}`.
    /// - Accepts `t` in `{Usf, Normal}`.
    /// - Disallowed combinations: none; all repr pairs are accepted.
    /// # Panics
    /// - Panics if repr selection is invalid for this backend.
    fn component_smoothstep(&self, _rhs: UsfOrNormalQuaternion, _t: UsfOrNormalFractionalScalar) -> Self {
        todo!()
    }

    /// Computes quaternion dot product in requested op mode.
    /// Output behavior:
    /// - Computes using canonical USF working precision.
    /// - Projects the result into `op_mode.repr`.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (UsfOrNormalQuaternion): Right-hand-side operand.
    /// - `op_mode` (OpMode): Output kind/repr projection policy.
    ///
    /// # Returns
    /// - Computed result of type `UsfOrNormalFractionalScalar`.
    ///
    /// # Repr
    /// - Accepts `{self: Usf, rhs: Usf}` and `{self: Usf, rhs: Normal}`.
    /// - Disallowed combinations: none; all repr pairs are accepted.
    /// # Panics
    /// - Panics if repr selection is invalid for this backend.
    fn dot(&self, _rhs: UsfOrNormalQuaternion, _op_mode: OpMode) -> UsfOrNormalFractionalScalar {
        todo!()
    }

    /// Rotates a 3D vector by this quaternion.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (UsfOrNormalVector<3>): Right-hand-side operand.
    /// - `op_mode` (OpMode): Output kind/repr projection policy.
    ///
    /// # Returns
    /// - Computed result of type `UsfOrNormalVector<3>`.
    ///
    /// # Repr
    /// - Accepts `{self: Usf, rhs_vector: Usf}` and `{self: Usf, rhs_vector: Normal}`.
    /// - Disallowed combinations: none; all repr pairs are accepted.
    /// # Panics
    /// - Panics if repr selection is invalid for this backend.
    /// - Panics if `self` is not a valid normalized rotation quaternion.
    fn rotate_vec3(&self, _rhs: UsfOrNormalVector<3>, _op_mode: OpMode) -> UsfOrNormalVector<3> {
        todo!()
    }

    /// Builds a value from axis angle.
    ///
    /// # Parameters
    /// - `axis` (UsfOrNormalVector<3>): Axis vector.
    /// - `angle_rad` (UsfOrNormalFractionalScalar): Angle in radians.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Repr
    /// - Allowed: `{axis: Usf}` and `{axis: Normal}`.
    /// - Allowed: `{angle_rad: Usf}` and `{angle_rad: Normal}`.
    /// - Disallowed combinations: none; all repr combinations are accepted.
    /// # Panics
    /// - Panics if repr selection is invalid for this backend.
    /// - Panics if `axis` is zero-length.
    fn from_axis_angle(_axis: UsfOrNormalVector<3>, _angle_rad: UsfOrNormalFractionalScalar) -> Self {
        todo!()
    }

    /// Converts quaternion to axis-angle.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `op_mode` (OpMode): Output kind/repr projection policy.
    ///
    /// # Returns
    /// - Tuple result of type `(UsfOrNormalVector<3>, UsfOrNormalFractionalScalar)`.
    ///
    /// # Repr
    /// - Output projection is selected via `op_mode`.
    /// # Panics
    /// - Panics if `self` is not a valid normalized rotation quaternion.
    fn to_axis_angle(&self, _op_mode: OpMode) -> (UsfOrNormalVector<3>, UsfOrNormalFractionalScalar) {
        todo!()
    }

    /// Builds quaternion from XYZ Euler angles.
    ///
    /// # Parameters
    /// - `x_rad` (UsfOrNormalFractionalScalar): Parameter `x_rad`.
    /// - `y_rad` (UsfOrNormalFractionalScalar): Parameter `y_rad`.
    /// - `z_rad` (UsfOrNormalFractionalScalar): Parameter `z_rad`.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Repr
    /// - Allowed: each Euler component independently in `{Usf, Normal}`.
    /// - Disallowed combinations: none; all repr combinations are accepted.
    /// # Panics
    /// - Panics if repr selection is invalid for this backend.
    fn from_euler_xyz(_x_rad: UsfOrNormalFractionalScalar, _y_rad: UsfOrNormalFractionalScalar, _z_rad: UsfOrNormalFractionalScalar) -> Self {
        todo!()
    }

    /// Converts quaternion to XYZ Euler angles.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `op_mode` (OpMode): Output kind/repr projection policy.
    ///
    /// # Returns
    /// - Fixed-size array result of type `[UsfOrNormalFractionalScalar; 3]`.
    ///
    /// # Repr
    /// - Output projection is selected via `op_mode`.
    /// # Panics
    /// - Panics if `self` is not a valid normalized rotation quaternion.
    fn to_euler_xyz(&self, _op_mode: OpMode) -> [UsfOrNormalFractionalScalar; 3] {
        todo!()
    }

    /// Performs spherical linear interpolation between quaternions.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (UsfOrNormalQuaternion): Right-hand-side operand.
    /// - `t` (UsfOrNormalFractionalScalar): Interpolation factor.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Repr
    /// - Allowed: `{rhs: Usf}` and `{rhs: Normal}`.
    /// - Allowed: `{t: Usf}` and `{t: Normal}`.
    /// - Disallowed combinations: none; all repr combinations are accepted.
    /// # Panics
    /// - Panics if repr selection is invalid for this backend.
    /// - Panics if interpolation endpoints are invalid rotation quaternions.
    /// - Panics if interpolation path is undefined for the endpoint pair.
    fn slerp(&self, _rhs: UsfOrNormalQuaternion, _t: UsfOrNormalFractionalScalar) -> Self {
        todo!()
    }

    /// Performs normalized linear interpolation between quaternions.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (UsfOrNormalQuaternion): Right-hand-side operand.
    /// - `t` (UsfOrNormalFractionalScalar): Interpolation factor.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Repr
    /// - Allowed: `{rhs: Usf}` and `{rhs: Normal}`.
    /// - Allowed: `{t: Usf}` and `{t: Normal}`.
    /// - Disallowed combinations: none; all repr combinations are accepted.
    /// # Panics
    /// - Panics if repr selection is invalid for this backend.
    /// - Panics if interpolation endpoints are invalid rotation quaternions.
    /// - Panics if normalized interpolation produces a zero norm.
    fn nlerp(&self, _rhs: UsfOrNormalQuaternion, _t: UsfOrNormalFractionalScalar) -> Self {
        todo!()
    }

    /// Converts quaternion to 3x3 matrix representation.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `op_mode` (OpMode): Output kind/repr projection policy.
    ///
    /// # Returns
    /// - Computed result of type `UsfOrNormalMat3`.
    ///
    /// # Repr
    /// - Output projection is selected via `op_mode`.
    /// # Panics
    /// - Panics if `self` is not a valid normalized rotation quaternion.
    fn to_mat3(&self, _op_mode: OpMode) -> UsfOrNormalMat3 {
        todo!()
    }

    /// Builds a value from mat3.
    ///
    /// # Parameters
    /// - `value` (UsfOrNormalMat3): Input value for this operation.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Repr
    /// - Allowed: `{value: Usf}` and `{value: Normal}`.
    /// - Disallowed combinations: none; all repr values are accepted.
    /// # Panics
    /// - Panics if repr selection is invalid for this backend.
    /// - Panics if `value` is not a valid rotation matrix under strict rotation-matrix validation.
    fn from_mat3(_value: UsfOrNormalMat3) -> Self {
        todo!()
    }
}

/// Field-like quaternion component access contract.
pub trait QuaternionFieldOps: Clone + Sized {
    /// Returns `x` component.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `op_mode` (OpMode): Output kind/repr projection policy.
    ///
    /// # Returns
    /// - Computed result of type `UsfOrNormalScalar`.
    ///
    /// # Repr
    /// - Output projection is selected via `op_mode`.
    fn get_x(&self, _op_mode: OpMode) -> UsfOrNormalScalar {
        todo!()
    }

    /// Returns `y` component.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `op_mode` (OpMode): Output kind/repr projection policy.
    ///
    /// # Returns
    /// - Computed result of type `UsfOrNormalScalar`.
    ///
    /// # Repr
    /// - Output projection is selected via `op_mode`.
    fn get_y(&self, _op_mode: OpMode) -> UsfOrNormalScalar {
        todo!()
    }

    /// Returns `z` component.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `op_mode` (OpMode): Output kind/repr projection policy.
    ///
    /// # Returns
    /// - Computed result of type `UsfOrNormalScalar`.
    ///
    /// # Repr
    /// - Output projection is selected via `op_mode`.
    fn get_z(&self, _op_mode: OpMode) -> UsfOrNormalScalar {
        todo!()
    }

    /// Returns `w` component.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `op_mode` (OpMode): Output kind/repr projection policy.
    ///
    /// # Returns
    /// - Computed result of type `UsfOrNormalScalar`.
    ///
    /// # Repr
    /// - Output projection is selected via `op_mode`.
    fn get_w(&self, _op_mode: OpMode) -> UsfOrNormalScalar {
        todo!()
    }

    /// Sets `x` component.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `value` (UsfOrNormalScalar): Input value for this operation.
    ///
    /// # Returns
    /// - No value; mutates receiver state where applicable.
    ///
    /// # Repr
    /// - Allowed: `{value: Usf}` and `{value: Normal}`.
    /// - Disallowed combinations: none; all repr values are accepted.
    /// # Panics
    /// - Panics if repr selection is invalid for this backend.
    /// - Panics if the target quaternion component is immutable under runtime field mutability policy.
    fn set_x(&mut self, _value: UsfOrNormalScalar) {
        todo!()
    }

    /// Sets `y` component.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `value` (UsfOrNormalScalar): Input value for this operation.
    ///
    /// # Returns
    /// - No value; mutates receiver state where applicable.
    ///
    /// # Repr
    /// - Allowed: `{value: Usf}` and `{value: Normal}`.
    /// - Disallowed combinations: none; all repr values are accepted.
    /// # Panics
    /// - Panics if repr selection is invalid for this backend.
    /// - Panics if the target quaternion component is immutable under runtime field mutability policy.
    fn set_y(&mut self, _value: UsfOrNormalScalar) {
        todo!()
    }

    /// Sets `z` component.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `value` (UsfOrNormalScalar): Input value for this operation.
    ///
    /// # Returns
    /// - No value; mutates receiver state where applicable.
    ///
    /// # Repr
    /// - Allowed: `{value: Usf}` and `{value: Normal}`.
    /// - Disallowed combinations: none; all repr values are accepted.
    /// # Panics
    /// - Panics if repr selection is invalid for this backend.
    /// - Panics if the target quaternion component is immutable under runtime field mutability policy.
    fn set_z(&mut self, _value: UsfOrNormalScalar) {
        todo!()
    }

    /// Sets `w` component.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `value` (UsfOrNormalScalar): Input value for this operation.
    ///
    /// # Returns
    /// - No value; mutates receiver state where applicable.
    ///
    /// # Repr
    /// - Allowed: `{value: Usf}` and `{value: Normal}`.
    /// - Disallowed combinations: none; all repr values are accepted.
    /// # Panics
    /// - Panics if repr selection is invalid for this backend.
    /// - Panics if the target quaternion component is immutable under runtime field mutability policy.
    fn set_w(&mut self, _value: UsfOrNormalScalar) {
        todo!()
    }
}

/// Bridge-only extension point for quaternion surfaces.
pub trait QuaternionBridgeOps: QuaternionCoreOps {}

/// Full quaternion contract.
pub trait QuaternionContract: QuaternionCoreOps + QuaternionFieldOps + QuaternionBridgeOps {}
impl<T> QuaternionContract for T where T: QuaternionCoreOps + QuaternionFieldOps + QuaternionBridgeOps {}

/// Erased quaternion contract for generic bridge/facade plumbing.
pub trait QuaternionAnyContract: Clone + Sized {}
impl<T> QuaternionAnyContract for T where T: QuaternionContract {}
