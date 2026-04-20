#![allow(dead_code)]

use super::super::aliases::OutputMode;
use super::super::matrix::shared::SquareMatrixContract;
use super::super::scalar::aliases::{UsfOrNormalDecimalScalar, UsfOrNormalScalar};
use crate::utils::one_of::OneOf2;

pub trait QuaternionCoreOps<Vector3, Matrix3>: Clone + Sized {
    /// Returns identity quaternion.
    fn identity() -> Self {
        todo!()
    }
    /// Builds quaternion from components.
    fn from_xyzw(_x: UsfOrNormalDecimalScalar, _y: UsfOrNormalDecimalScalar, _z: UsfOrNormalDecimalScalar, _w: UsfOrNormalDecimalScalar) -> Self {
        todo!()
    }
    /// Returns quaternion components in requested output mode.
    fn to_xyzw(&self, _output_mode: OutputMode) -> [UsfOrNormalDecimalScalar; 4] {
        todo!()
    }
    /// Returns normalized quaternion.
    fn normalize(&self) -> Self {
        todo!()
    }
    /// Returns quaternion conjugate.
    fn conjugate(&self) -> Self {
        todo!()
    }
    /// Returns quaternion inverse.
    fn inverse(&self) -> Self {
        todo!()
    }
    /// Adds quaternion or scalar operand.
    fn add(&self, _rhs: OneOf2<Self, UsfOrNormalScalar>) -> Self {
        todo!()
    }
    /// Subtracts quaternion or scalar operand.
    fn sub(&self, _rhs: OneOf2<Self, UsfOrNormalScalar>) -> Self {
        todo!()
    }
    /// Multiplies quaternion or scalar operand.
    fn mul(&self, _rhs: OneOf2<Self, UsfOrNormalScalar>) -> Self {
        todo!()
    }
    /// Divides quaternion or scalar operand.
    fn div(&self, _rhs: OneOf2<Self, UsfOrNormalScalar>) -> Self {
        todo!()
    }
    /// Computes remainder with quaternion or scalar operand.
    fn rem(&self, _rhs: OneOf2<Self, UsfOrNormalScalar>) -> Self {
        todo!()
    }
    /// Returns component-wise minimum.
    fn min(&self, _rhs: Self) -> Self {
        todo!()
    }
    /// Returns component-wise maximum.
    fn max(&self, _rhs: Self) -> Self {
        todo!()
    }
    /// Clamps each quaternion component to `[lo, hi]`.
    fn clamp(&self, _lo: Self, _hi: Self) -> Self {
        todo!()
    }
    /// Performs linear interpolation.
    fn lerp(&self, _rhs: Self, _t: UsfOrNormalDecimalScalar) -> Self {
        todo!()
    }
    /// Performs smoothstep interpolation.
    fn smoothstep(&self, _rhs: Self, _t: UsfOrNormalDecimalScalar) -> Self {
        todo!()
    }
    /// Computes dot product in requested output mode.
    fn dot(&self, _rhs: Self, _output_mode: OutputMode) -> UsfOrNormalDecimalScalar {
        todo!()
    }
    /// Rotates a 3D vector.
    fn rotate_vec3(&self, _rhs: Vector3) -> Vector3 {
        todo!()
    }
    /// Builds quaternion from axis-angle.
    fn from_axis_angle(_axis: Vector3, _angle_rad: UsfOrNormalDecimalScalar) -> Self {
        todo!()
    }
    /// Converts quaternion to axis-angle.
    fn to_axis_angle(&self, _output_mode: OutputMode) -> (Vector3, UsfOrNormalDecimalScalar) {
        todo!()
    }
    /// Builds quaternion from XYZ Euler angles.
    fn from_euler_xyz(_x_rad: UsfOrNormalDecimalScalar, _y_rad: UsfOrNormalDecimalScalar, _z_rad: UsfOrNormalDecimalScalar) -> Self {
        todo!()
    }
    /// Converts quaternion to XYZ Euler angles.
    fn to_euler_xyz(&self, _output_mode: OutputMode) -> [UsfOrNormalDecimalScalar; 3] {
        todo!()
    }
    /// Performs spherical interpolation.
    fn slerp(&self, _rhs: Self, _t: UsfOrNormalDecimalScalar) -> Self {
        todo!()
    }
    /// Performs normalized linear interpolation.
    fn nlerp(&self, _rhs: Self, _t: UsfOrNormalDecimalScalar) -> Self {
        todo!()
    }
    /// Converts quaternion to 3x3 matrix representation.
    fn to_mat3<Matrix3B: SquareMatrixContract<Vector3, 3>>(&self, _output_mode: OutputMode) -> OneOf2<Matrix3, Matrix3B> {
        todo!()
    }
    /// Builds quaternion from 3x3 matrix representation.
    fn from_mat3<Matrix3B: SquareMatrixContract<Vector3, 3>>(_value: OneOf2<Matrix3, Matrix3B>) -> Self {
        todo!()
    }
}

pub trait QuaternionFieldOps: Clone + Sized {
    /// Returns `x` component.
    fn get_x(&self, _output_mode: OutputMode) -> UsfOrNormalScalar {
        todo!()
    }
    /// Returns `y` component.
    fn get_y(&self, _output_mode: OutputMode) -> UsfOrNormalScalar {
        todo!()
    }
    /// Returns `z` component.
    fn get_z(&self, _output_mode: OutputMode) -> UsfOrNormalScalar {
        todo!()
    }
    /// Returns `w` component.
    fn get_w(&self, _output_mode: OutputMode) -> UsfOrNormalScalar {
        todo!()
    }
    /// Sets `x` component.
    fn set_x(&mut self, _value: UsfOrNormalScalar) {
        todo!()
    }
    /// Sets `y` component.
    fn set_y(&mut self, _value: UsfOrNormalScalar) {
        todo!()
    }
    /// Sets `z` component.
    fn set_z(&mut self, _value: UsfOrNormalScalar) {
        todo!()
    }
    /// Sets `w` component.
    fn set_w(&mut self, _value: UsfOrNormalScalar) {
        todo!()
    }
}

pub trait QuaternionBridgeOps<Vector3, Matrix3>: QuaternionCoreOps<Vector3, Matrix3> {}

pub trait QuaternionContract<Vector3, Matrix3>: QuaternionCoreOps<Vector3, Matrix3> + QuaternionFieldOps + QuaternionBridgeOps<Vector3, Matrix3> {}
impl<T, Vector3, Matrix3> QuaternionContract<Vector3, Matrix3> for T where
    T: QuaternionCoreOps<Vector3, Matrix3> + QuaternionFieldOps + QuaternionBridgeOps<Vector3, Matrix3>
{
}

pub trait QuaternionAnyContract: Clone + Sized {}
impl<T, Vector3, Matrix3> QuaternionAnyContract for T where T: QuaternionContract<Vector3, Matrix3> {}
