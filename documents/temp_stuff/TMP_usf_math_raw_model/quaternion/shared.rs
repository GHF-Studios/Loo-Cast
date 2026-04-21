#![allow(dead_code)]

//! Shared quaternion contracts for USF/normal quaternion surfaces.
//!
//! Facade-first rule:
//! - This contract layer is the semantic source of truth.
//! - Script-facing APIs should be emitted by facade layers over concrete monomorphizations.
//!
//! Domain/quality mechanism:
//! - Mixed-domain operands use `UsfOrNormal*` aliases.
//! - Output projection selection uses `OutputMode`.
//! - Invalid domain-quality combinations panic fast.
//!
//! Method doc schema:
//! - Summary line: describe intent and core working principle.
//! - `# Parameters`: document each argument and expected role.
//! - `# Returns`: document the returned value and shape/branch semantics.
//! - Optional `# Domain` section for mixed-domain semantics.
//! - Optional `# Panics` section for runtime guard clauses and undefined math states.

use super::super::aliases::OutputMode;
use super::super::scalar::aliases::{UsfOrNormalDecimalScalar, UsfOrNormalScalar};
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
    /// - `x` (UsfOrNormalDecimalScalar): X component value.
    /// - `y` (UsfOrNormalDecimalScalar): Y component value.
    /// - `z` (UsfOrNormalDecimalScalar): Z component value.
    /// - `w` (UsfOrNormalDecimalScalar): W component value.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Domain
    /// - Allowed: each component independently in `{Usf, Normal}`.
    /// - Disallowed combinations: none; all domain combinations are accepted.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    /// - Panics if the input quaternion cannot be normalized into a valid rotation state.
    fn from_xyzw(_x: UsfOrNormalDecimalScalar, _y: UsfOrNormalDecimalScalar, _z: UsfOrNormalDecimalScalar, _w: UsfOrNormalDecimalScalar) -> Self {
        todo!()
    }

    /// Returns quaternion components in requested output mode.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `output_mode` (OutputMode): Output domain/quality projection policy.
    ///
    /// # Returns
    /// - Fixed-size array result of type `[UsfOrNormalDecimalScalar; 4]`.
    ///
    /// # Domain
    /// - Output projection is selected via `output_mode`.
    /// # Panics
    /// - Panics when `output_mode.domain == OutputDomain::Usf` and `output_mode.quality_constraint == OutputQualityConstraint::AllowLossy`, because USF output never uses lossy projection.
    /// - Panics when `output_mode.domain == OutputDomain::Normal` and `output_mode.quality_constraint == OutputQualityConstraint::RequireLossless` but component projection loses precision or range.
    fn to_xyzw(&self, _output_mode: OutputMode) -> [UsfOrNormalDecimalScalar; 4] {
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
    /// # Domain
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
    /// # Domain
    /// - Allowed: `{self: Usf}`.
    /// - Disallowed combinations: not applicable in this unary concrete `UsfQuaternion` API.
    /// # Panics
    /// - Panics if quaternion norm is zero.
    fn inverse(&self) -> Self {
        todo!()
    }

    /// Adds quaternion from either domain component-wise.
    /// Raw component op: does not guarantee a unit quaternion result.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (UsfOrNormalQuaternion): Right-hand-side operand.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Domain
    /// - Accepts `{self: Usf, rhs: Usf}` and `{self: Usf, rhs: Normal}`.
    /// - Disallowed combinations: none; all domain pairs are accepted.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    fn component_add(&self, _rhs: UsfOrNormalQuaternion) -> Self {
        todo!()
    }

    /// Subtracts quaternion from either domain component-wise.
    /// Raw component op: does not guarantee a unit quaternion result.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (UsfOrNormalQuaternion): Right-hand-side operand.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Domain
    /// - Accepts `{self: Usf, rhs: Usf}` and `{self: Usf, rhs: Normal}`.
    /// - Disallowed combinations: none; all domain pairs are accepted.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    fn component_sub(&self, _rhs: UsfOrNormalQuaternion) -> Self {
        todo!()
    }

    /// Applies Hamilton product with quaternion from either domain.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (UsfOrNormalQuaternion): Right-hand-side operand.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Domain
    /// - Accepts `{self: Usf, rhs: Usf}` and `{self: Usf, rhs: Normal}`.
    /// - Disallowed combinations: none; all domain pairs are accepted.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    fn hamilton_mul(&self, _rhs: UsfOrNormalQuaternion) -> Self {
        todo!()
    }

    /// Applies Hamilton division with quaternion from either domain.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (UsfOrNormalQuaternion): Right-hand-side operand.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Domain
    /// - Accepts `{self: Usf, rhs: Usf}` and `{self: Usf, rhs: Normal}`.
    /// - Disallowed combinations: none; all domain pairs are accepted.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
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
    /// # Domain
    /// - Accepts `{self: Usf, scalar: Usf}` and `{self: Usf, scalar: Normal}`.
    /// - Disallowed combinations: none; all domain pairs are accepted.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
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
    /// # Domain
    /// - Accepts `{self: Usf, scalar: Usf}` and `{self: Usf, scalar: Normal}`.
    /// - Disallowed combinations: none; all domain pairs are accepted.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
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
    /// # Domain
    /// - Accepts `{self: Usf, rhs: Usf}` and `{self: Usf, rhs: Normal}`.
    /// - Disallowed combinations: none; all domain pairs are accepted.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
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
    /// # Domain
    /// - Accepts `{self: Usf, rhs: Usf}` and `{self: Usf, rhs: Normal}`.
    /// - Disallowed combinations: none; all domain pairs are accepted.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
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
    /// # Domain
    /// - Accepts all `{lo, hi}` pairings in `{Usf, Normal} × {Usf, Normal}`.
    /// - Disallowed combinations: none; all domain pairs are accepted.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    /// - Panics if any quaternion component has `lo > hi`.
    fn component_clamp(&self, _lo: UsfOrNormalQuaternion, _hi: UsfOrNormalQuaternion) -> Self {
        todo!()
    }

    /// Performs per-component linear interpolation (not spherical rotation interpolation).
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (UsfOrNormalQuaternion): Right-hand-side operand.
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
    fn component_lerp(&self, _rhs: UsfOrNormalQuaternion, _t: UsfOrNormalDecimalScalar) -> Self {
        todo!()
    }

    /// Performs per-component smoothstep interpolation (not spherical rotation interpolation).
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (UsfOrNormalQuaternion): Right-hand-side operand.
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
    fn component_smoothstep(&self, _rhs: UsfOrNormalQuaternion, _t: UsfOrNormalDecimalScalar) -> Self {
        todo!()
    }

    /// Computes quaternion dot product in requested output mode.
    /// Output behavior:
    /// - Computes using canonical USF working precision.
    /// - Projects the result into `output_mode.domain`.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (UsfOrNormalQuaternion): Right-hand-side operand.
    /// - `output_mode` (OutputMode): Output domain/quality projection policy.
    ///
    /// # Returns
    /// - Computed result of type `UsfOrNormalDecimalScalar`.
    ///
    /// # Domain
    /// - Accepts `{self: Usf, rhs: Usf}` and `{self: Usf, rhs: Normal}`.
    /// - Disallowed combinations: none; all domain pairs are accepted.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    /// - Panics when `output_mode.domain == OutputDomain::Usf` and `output_mode.quality_constraint == OutputQualityConstraint::AllowLossy`, because USF output never uses lossy projection.
    /// - Panics when `output_mode.domain == OutputDomain::Normal` and `output_mode.quality_constraint == OutputQualityConstraint::RequireLossless` but the projection loses precision or range.
    fn dot(&self, _rhs: UsfOrNormalQuaternion, _output_mode: OutputMode) -> UsfOrNormalDecimalScalar {
        todo!()
    }

    /// Rotates a 3D vector by this quaternion.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (UsfOrNormalVector<3>): Right-hand-side operand.
    /// - `output_mode` (OutputMode): Output domain/quality projection policy.
    ///
    /// # Returns
    /// - Computed result of type `UsfOrNormalVector<3>`.
    ///
    /// # Domain
    /// - Accepts `{self: Usf, rhs_vector: Usf}` and `{self: Usf, rhs_vector: Normal}`.
    /// - Disallowed combinations: none; all domain pairs are accepted.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    /// - Panics if `self` is not a valid normalized rotation quaternion.
    /// - Panics when `output_mode.domain == OutputDomain::Usf` and `output_mode.quality_constraint == OutputQualityConstraint::AllowLossy`, because USF output never uses lossy projection.
    /// - Panics when `output_mode.domain == OutputDomain::Normal` and `output_mode.quality_constraint == OutputQualityConstraint::RequireLossless` but the projection loses precision or range.
    fn rotate_vec3(&self, _rhs: UsfOrNormalVector<3>, _output_mode: OutputMode) -> UsfOrNormalVector<3> {
        todo!()
    }

    /// Builds a value from axis angle.
    ///
    /// # Parameters
    /// - `axis` (UsfOrNormalVector<3>): Axis vector.
    /// - `angle_rad` (UsfOrNormalDecimalScalar): Angle in radians.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Domain
    /// - Allowed: `{axis: Usf}` and `{axis: Normal}`.
    /// - Allowed: `{angle_rad: Usf}` and `{angle_rad: Normal}`.
    /// - Disallowed combinations: none; all domain combinations are accepted.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    /// - Panics if `axis` is zero-length.
    fn from_axis_angle(_axis: UsfOrNormalVector<3>, _angle_rad: UsfOrNormalDecimalScalar) -> Self {
        todo!()
    }

    /// Converts quaternion to axis-angle.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `output_mode` (OutputMode): Output domain/quality projection policy.
    ///
    /// # Returns
    /// - Tuple result of type `(UsfOrNormalVector<3>, UsfOrNormalDecimalScalar)`.
    ///
    /// # Domain
    /// - Output projection is selected via `output_mode`.
    /// # Panics
    /// - Panics if `self` is not a valid normalized rotation quaternion.
    /// - Panics when `output_mode.domain == OutputDomain::Usf` and `output_mode.quality_constraint == OutputQualityConstraint::AllowLossy`, because USF output never uses lossy projection.
    /// - Panics when `output_mode.domain == OutputDomain::Normal` and `output_mode.quality_constraint == OutputQualityConstraint::RequireLossless` but the projection loses precision or range.
    fn to_axis_angle(&self, _output_mode: OutputMode) -> (UsfOrNormalVector<3>, UsfOrNormalDecimalScalar) {
        todo!()
    }

    /// Builds quaternion from XYZ Euler angles.
    ///
    /// # Parameters
    /// - `x_rad` (UsfOrNormalDecimalScalar): Parameter `x_rad`.
    /// - `y_rad` (UsfOrNormalDecimalScalar): Parameter `y_rad`.
    /// - `z_rad` (UsfOrNormalDecimalScalar): Parameter `z_rad`.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Domain
    /// - Allowed: each Euler component independently in `{Usf, Normal}`.
    /// - Disallowed combinations: none; all domain combinations are accepted.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    fn from_euler_xyz(_x_rad: UsfOrNormalDecimalScalar, _y_rad: UsfOrNormalDecimalScalar, _z_rad: UsfOrNormalDecimalScalar) -> Self {
        todo!()
    }

    /// Converts quaternion to XYZ Euler angles.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `output_mode` (OutputMode): Output domain/quality projection policy.
    ///
    /// # Returns
    /// - Fixed-size array result of type `[UsfOrNormalDecimalScalar; 3]`.
    ///
    /// # Domain
    /// - Output projection is selected via `output_mode`.
    /// # Panics
    /// - Panics if `self` is not a valid normalized rotation quaternion.
    /// - Panics when `output_mode.domain == OutputDomain::Usf` and `output_mode.quality_constraint == OutputQualityConstraint::AllowLossy`, because USF output never uses lossy projection.
    /// - Panics when `output_mode.domain == OutputDomain::Normal` and `output_mode.quality_constraint == OutputQualityConstraint::RequireLossless` but the projection loses precision or range.
    fn to_euler_xyz(&self, _output_mode: OutputMode) -> [UsfOrNormalDecimalScalar; 3] {
        todo!()
    }

    /// Performs spherical linear interpolation between quaternions.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (UsfOrNormalQuaternion): Right-hand-side operand.
    /// - `t` (UsfOrNormalDecimalScalar): Interpolation factor.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Domain
    /// - Allowed: `{rhs: Usf}` and `{rhs: Normal}`.
    /// - Allowed: `{t: Usf}` and `{t: Normal}`.
    /// - Disallowed combinations: none; all domain combinations are accepted.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    /// - Panics if interpolation endpoints are invalid rotation quaternions.
    /// - Panics if interpolation path is undefined for the endpoint pair.
    fn slerp(&self, _rhs: UsfOrNormalQuaternion, _t: UsfOrNormalDecimalScalar) -> Self {
        todo!()
    }

    /// Performs normalized linear interpolation between quaternions.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `rhs` (UsfOrNormalQuaternion): Right-hand-side operand.
    /// - `t` (UsfOrNormalDecimalScalar): Interpolation factor.
    ///
    /// # Returns
    /// - A new value of the same concrete type.
    ///
    /// # Domain
    /// - Allowed: `{rhs: Usf}` and `{rhs: Normal}`.
    /// - Allowed: `{t: Usf}` and `{t: Normal}`.
    /// - Disallowed combinations: none; all domain combinations are accepted.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
    /// - Panics if interpolation endpoints are invalid rotation quaternions.
    /// - Panics if normalized interpolation produces a zero norm.
    fn nlerp(&self, _rhs: UsfOrNormalQuaternion, _t: UsfOrNormalDecimalScalar) -> Self {
        todo!()
    }

    /// Converts quaternion to 3x3 matrix representation.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `output_mode` (OutputMode): Output domain/quality projection policy.
    ///
    /// # Returns
    /// - Computed result of type `UsfOrNormalMat3`.
    ///
    /// # Domain
    /// - Output projection is selected via `output_mode`.
    /// # Panics
    /// - Panics if `self` is not a valid normalized rotation quaternion.
    /// - Panics when `output_mode.domain == OutputDomain::Usf` and `output_mode.quality_constraint == OutputQualityConstraint::AllowLossy`, because USF output never uses lossy projection.
    /// - Panics when `output_mode.domain == OutputDomain::Normal` and `output_mode.quality_constraint == OutputQualityConstraint::RequireLossless` but matrix projection loses precision or range.
    fn to_mat3(&self, _output_mode: OutputMode) -> UsfOrNormalMat3 {
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
    /// # Domain
    /// - Allowed: `{value: Usf}` and `{value: Normal}`.
    /// - Disallowed combinations: none; all domain values are accepted.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
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
    /// - `output_mode` (OutputMode): Output domain/quality projection policy.
    ///
    /// # Returns
    /// - Computed result of type `UsfOrNormalScalar`.
    ///
    /// # Domain
    /// - Output projection is selected via `output_mode`.
    /// # Panics
    /// - Panics when `output_mode.domain == OutputDomain::Usf` and `output_mode.quality_constraint == OutputQualityConstraint::AllowLossy`, because USF output never uses lossy projection.
    /// - Panics when `output_mode.domain == OutputDomain::Normal` and `output_mode.quality_constraint == OutputQualityConstraint::RequireLossless` but component projection loses precision or range.
    fn get_x(&self, _output_mode: OutputMode) -> UsfOrNormalScalar {
        todo!()
    }

    /// Returns `y` component.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `output_mode` (OutputMode): Output domain/quality projection policy.
    ///
    /// # Returns
    /// - Computed result of type `UsfOrNormalScalar`.
    ///
    /// # Domain
    /// - Output projection is selected via `output_mode`.
    /// # Panics
    /// - Panics when `output_mode.domain == OutputDomain::Usf` and `output_mode.quality_constraint == OutputQualityConstraint::AllowLossy`, because USF output never uses lossy projection.
    /// - Panics when `output_mode.domain == OutputDomain::Normal` and `output_mode.quality_constraint == OutputQualityConstraint::RequireLossless` but component projection loses precision or range.
    fn get_y(&self, _output_mode: OutputMode) -> UsfOrNormalScalar {
        todo!()
    }

    /// Returns `z` component.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `output_mode` (OutputMode): Output domain/quality projection policy.
    ///
    /// # Returns
    /// - Computed result of type `UsfOrNormalScalar`.
    ///
    /// # Domain
    /// - Output projection is selected via `output_mode`.
    /// # Panics
    /// - Panics when `output_mode.domain == OutputDomain::Usf` and `output_mode.quality_constraint == OutputQualityConstraint::AllowLossy`, because USF output never uses lossy projection.
    /// - Panics when `output_mode.domain == OutputDomain::Normal` and `output_mode.quality_constraint == OutputQualityConstraint::RequireLossless` but component projection loses precision or range.
    fn get_z(&self, _output_mode: OutputMode) -> UsfOrNormalScalar {
        todo!()
    }

    /// Returns `w` component.
    ///
    /// # Parameters
    /// - `self`: Receiver value.
    /// - `output_mode` (OutputMode): Output domain/quality projection policy.
    ///
    /// # Returns
    /// - Computed result of type `UsfOrNormalScalar`.
    ///
    /// # Domain
    /// - Output projection is selected via `output_mode`.
    /// # Panics
    /// - Panics when `output_mode.domain == OutputDomain::Usf` and `output_mode.quality_constraint == OutputQualityConstraint::AllowLossy`, because USF output never uses lossy projection.
    /// - Panics when `output_mode.domain == OutputDomain::Normal` and `output_mode.quality_constraint == OutputQualityConstraint::RequireLossless` but component projection loses precision or range.
    fn get_w(&self, _output_mode: OutputMode) -> UsfOrNormalScalar {
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
    /// # Domain
    /// - Allowed: `{value: Usf}` and `{value: Normal}`.
    /// - Disallowed combinations: none; all domain values are accepted.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
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
    /// # Domain
    /// - Allowed: `{value: Usf}` and `{value: Normal}`.
    /// - Disallowed combinations: none; all domain values are accepted.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
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
    /// # Domain
    /// - Allowed: `{value: Usf}` and `{value: Normal}`.
    /// - Disallowed combinations: none; all domain values are accepted.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
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
    /// # Domain
    /// - Allowed: `{value: Usf}` and `{value: Normal}`.
    /// - Disallowed combinations: none; all domain values are accepted.
    /// # Panics
    /// - Panics if domain selection is invalid for this backend.
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
