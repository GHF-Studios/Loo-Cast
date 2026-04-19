#![allow(dead_code)]

use super::super::matrix::shared::SquareMatrixContract;
use super::super::scalar::shared::ScalarContract;
use crate::utils::one_of::OneOf2;

pub trait QuaternionCoreOps<Scalar: ScalarContract, Vector3, Matrix3>: Clone + Sized {
    fn identity() -> Self {
        todo!()
    }
    fn from_xyzw<ScalarB: ScalarContract>(
        _x: OneOf2<Scalar, ScalarB>,
        _y: OneOf2<Scalar, ScalarB>,
        _z: OneOf2<Scalar, ScalarB>,
        _w: OneOf2<Scalar, ScalarB>,
    ) -> Self {
        todo!()
    }
    fn to_xyzw<ScalarB: ScalarContract>(&self) -> [OneOf2<Scalar, ScalarB>; 4] {
        todo!()
    }
    fn normalize(&self) -> Self {
        todo!()
    }
    fn conjugate(&self) -> Self {
        todo!()
    }
    fn inverse(&self) -> Self {
        todo!()
    }
    fn add<ScalarB: ScalarContract>(&self, _rhs: OneOf2<Self, OneOf2<Scalar, ScalarB>>) -> Self {
        todo!()
    }
    fn sub<ScalarB: ScalarContract>(&self, _rhs: OneOf2<Self, OneOf2<Scalar, ScalarB>>) -> Self {
        todo!()
    }
    fn mul<ScalarB: ScalarContract>(&self, _rhs: OneOf2<Self, OneOf2<Scalar, ScalarB>>) -> Self {
        todo!()
    }
    fn div<ScalarB: ScalarContract>(&self, _rhs: OneOf2<Self, OneOf2<Scalar, ScalarB>>) -> Self {
        todo!()
    }
    fn rem<ScalarB: ScalarContract>(&self, _rhs: OneOf2<Self, OneOf2<Scalar, ScalarB>>) -> Self {
        todo!()
    }
    fn min(&self, _rhs: Self) -> Self {
        todo!()
    }
    fn max(&self, _rhs: Self) -> Self {
        todo!()
    }
    fn clamp(&self, _lo: Self, _hi: Self) -> Self {
        todo!()
    }
    fn lerp<ScalarB: ScalarContract>(&self, _rhs: Self, _t: OneOf2<Scalar, ScalarB>) -> Self {
        todo!()
    }
    fn smoothstep<ScalarB: ScalarContract>(&self, _rhs: Self, _t: OneOf2<Scalar, ScalarB>) -> Self {
        todo!()
    }
    fn dot<ScalarB: ScalarContract>(&self, _rhs: Self) -> OneOf2<Scalar, ScalarB> {
        todo!()
    }
    fn rotate_vec3(&self, _rhs: Vector3) -> Vector3 {
        todo!()
    }
    fn from_axis_angle<ScalarB: ScalarContract>(_axis: Vector3, _angle_rad: OneOf2<Scalar, ScalarB>) -> Self {
        todo!()
    }
    fn to_axis_angle<ScalarB: ScalarContract>(&self) -> (Vector3, OneOf2<Scalar, ScalarB>) {
        todo!()
    }
    fn from_euler_xyz<ScalarB: ScalarContract>(_x_rad: OneOf2<Scalar, ScalarB>, _y_rad: OneOf2<Scalar, ScalarB>, _z_rad: OneOf2<Scalar, ScalarB>) -> Self {
        todo!()
    }
    fn to_euler_xyz<ScalarB: ScalarContract>(&self) -> [OneOf2<Scalar, ScalarB>; 3] {
        todo!()
    }
    fn slerp<ScalarB: ScalarContract>(&self, _rhs: Self, _t: OneOf2<Scalar, ScalarB>) -> Self {
        todo!()
    }
    fn nlerp<ScalarB: ScalarContract>(&self, _rhs: Self, _t: OneOf2<Scalar, ScalarB>) -> Self {
        todo!()
    }
    fn to_mat3<Matrix3B: SquareMatrixContract<Scalar, Vector3, 3>>(&self) -> OneOf2<Matrix3, Matrix3B> {
        todo!()
    }
    fn from_mat3<Matrix3B: SquareMatrixContract<Scalar, Vector3, 3>>(_value: OneOf2<Matrix3, Matrix3B>) -> Self {
        todo!()
    }
}

pub trait QuaternionFieldOps<Scalar: ScalarContract>: Clone + Sized {
    fn get_x(&self) -> Scalar {
        todo!()
    }
    fn get_y(&self) -> Scalar {
        todo!()
    }
    fn get_z(&self) -> Scalar {
        todo!()
    }
    fn get_w(&self) -> Scalar {
        todo!()
    }
    fn set_x(&mut self, _value: Scalar) {
        todo!()
    }
    fn set_y(&mut self, _value: Scalar) {
        todo!()
    }
    fn set_z(&mut self, _value: Scalar) {
        todo!()
    }
    fn set_w(&mut self, _value: Scalar) {
        todo!()
    }
}

pub trait QuaternionBridgeOps<Scalar: ScalarContract, Vector3, Matrix3>: QuaternionCoreOps<Scalar, Vector3, Matrix3> {}

pub trait QuaternionContract<Scalar: ScalarContract, Vector3, Matrix3>:
    QuaternionCoreOps<Scalar, Vector3, Matrix3> + QuaternionFieldOps<Scalar> + QuaternionBridgeOps<Scalar, Vector3, Matrix3>
{
}
impl<T, Scalar: ScalarContract, Vector3, Matrix3> QuaternionContract<Scalar, Vector3, Matrix3> for T where
    T: QuaternionCoreOps<Scalar, Vector3, Matrix3> + QuaternionFieldOps<Scalar> + QuaternionBridgeOps<Scalar, Vector3, Matrix3>
{
}

pub trait QuaternionAnyContract: Clone + Sized {}
impl<T, Scalar: ScalarContract, Vector3, Matrix3> QuaternionAnyContract for T where T: QuaternionContract<Scalar, Vector3, Matrix3> {}
