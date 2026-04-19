#![allow(dead_code)]

use super::super::scalar::shared::ScalarContract;
use crate::utils::one_of::OneOf2;

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

pub trait VectorCoreOps<Scalar: ScalarContract, const D: usize>: Clone + Sized {
    fn zero() -> Self {
        todo!()
    }
    fn one() -> Self {
        todo!()
    }
    fn splat(_value: Scalar) -> Self {
        todo!()
    }
    fn from_lanes(_lanes: [Scalar; D]) -> Self {
        todo!()
    }
    fn to_lanes(&self) -> [Scalar; D] {
        todo!()
    }
    fn normalize(&self) -> Self {
        todo!()
    }
    fn floor(&self) -> Self {
        todo!()
    }
    fn ceil(&self) -> Self {
        todo!()
    }
    fn round(&self) -> Self {
        todo!()
    }
    fn fract(&self) -> Self {
        todo!()
    }
    fn neg(&self) -> Self {
        todo!()
    }
    fn abs(&self) -> Self {
        todo!()
    }
    fn add(&self, _rhs: OneOf2<Self, Scalar>) -> Self {
        todo!()
    }
    fn sub(&self, _rhs: OneOf2<Self, Scalar>) -> Self {
        todo!()
    }
    fn mul(&self, _rhs: OneOf2<Self, Scalar>) -> Self {
        todo!()
    }
    fn div(&self, _rhs: OneOf2<Self, Scalar>) -> Self {
        todo!()
    }
    fn rem(&self, _rhs: OneOf2<Self, Scalar>) -> Self {
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
    fn distance<ScalarB: ScalarContract>(&self, _rhs: Self) -> OneOf2<Scalar, ScalarB> {
        todo!()
    }
    fn angle_between<ScalarB: ScalarContract>(&self, _rhs: Self) -> OneOf2<Scalar, ScalarB> {
        todo!()
    }
    fn project(&self, _onto: Self) -> Self {
        todo!()
    }
    fn reject(&self, _onto: Self) -> Self {
        todo!()
    }
    fn reflect(&self, _normal: Self) -> Self {
        todo!()
    }
    fn mul_elem(&self, _rhs: Self) -> Self {
        todo!()
    }
    fn div_elem(&self, _rhs: Self) -> Self {
        todo!()
    }
    fn fma(&self, _b: Self, _c: Self) -> Self {
        todo!()
    }
    fn scale(&self, _rhs: Scalar) -> Self {
        todo!()
    }
    fn get_dimension(&self) -> usize {
        todo!()
    }
    fn get_length<ScalarB: ScalarContract>(&self) -> OneOf2<Scalar, ScalarB> {
        todo!()
    }
    fn get_length_squared<ScalarB: ScalarContract>(&self) -> OneOf2<Scalar, ScalarB> {
        todo!()
    }
    fn get_lane(&self, _index: usize) -> Scalar {
        todo!()
    }
    fn set_lane(&mut self, _index: usize, _value: Scalar) {
        todo!()
    }
}

pub trait VectorBridgeOps<Scalar: ScalarContract, const D: usize>: VectorCoreOps<Scalar, D> {}

pub trait VectorFieldOps<Scalar: ScalarContract, const D: usize>: VectorCoreOps<Scalar, D> {}
impl<T, Scalar: ScalarContract, const D: usize> VectorFieldOps<Scalar, D> for T where T: VectorCoreOps<Scalar, D> {}

pub trait Vector2dFieldOps<Scalar: ScalarContract>: Clone + Sized {
    fn get_x(&self) -> Scalar {
        todo!()
    }
    fn get_y(&self) -> Scalar {
        todo!()
    }
    fn set_x(&mut self, _value: Scalar) {
        todo!()
    }
    fn set_y(&mut self, _value: Scalar) {
        todo!()
    }
}

pub trait Vector3dFieldOps<Scalar: ScalarContract>: Vector2dFieldOps<Scalar> {
    fn get_z(&self) -> Scalar {
        todo!()
    }
    fn set_z(&mut self, _value: Scalar) {
        todo!()
    }
}

pub trait Vector4dFieldOps<Scalar: ScalarContract>: Vector3dFieldOps<Scalar> {
    fn get_w(&self) -> Scalar {
        todo!()
    }
    fn set_w(&mut self, _value: Scalar) {
        todo!()
    }
}

pub trait Vector2dCoreOps<Scalar: ScalarContract>: Vector2dFieldOps<Scalar> + VectorCoreOps<Scalar, 2> {
    fn perp_ccw(&self) -> Self {
        todo!()
    }
    fn perp_cw(&self) -> Self {
        todo!()
    }
    fn perp_dot(&self, _rhs: Self) -> Scalar {
        todo!()
    }
    fn from_angle(_angle_rad: Scalar) -> Self {
        todo!()
    }
    fn angle(&self) -> Scalar {
        todo!()
    }
    fn rotate(&self, _angle_rad: Scalar) -> Self {
        todo!()
    }
    fn to_polar(&self) -> (Scalar, Scalar) {
        todo!()
    }
    fn from_polar(_radius: Scalar, _angle_rad: Scalar) -> Self {
        todo!()
    }
}

pub trait Vector3dCoreOps<Scalar: ScalarContract>: Vector3dFieldOps<Scalar> + VectorCoreOps<Scalar, 3> {
    fn cross(&self, _rhs: Self) -> Self {
        todo!()
    }
    fn cross_normalized(&self, _rhs: Self) -> Self {
        todo!()
    }
    fn triple_product(&self, _b: Self, _c: Self) -> Scalar {
        todo!()
    }
    fn project_on_plane(&self, _plane_normal: Self) -> Self {
        todo!()
    }
    fn reflect_on_plane(&self, _plane_normal: Self) -> Self {
        todo!()
    }
    fn rotate_around_axis(&self, _axis: Self, _angle_rad: Scalar) -> Self {
        todo!()
    }
    fn signed_angle(&self, _rhs: Self, _axis: Self) -> Scalar {
        todo!()
    }
    fn to_spherical(&self) -> (Scalar, Scalar, Scalar) {
        todo!()
    }
    fn from_spherical(_radius: Scalar, _azimuth: Scalar, _inclination: Scalar) -> Self {
        todo!()
    }
}

pub trait Vector4dCoreOps<Scalar: ScalarContract, Vector3d>: Vector4dFieldOps<Scalar> + VectorCoreOps<Scalar, 4> {
    fn from_vec3_w(_xyz: Vector3d, _w: Scalar) -> Self {
        todo!()
    }
    fn xyz(&self) -> Vector3d {
        todo!()
    }
    fn with_w(&self, _w: Scalar) -> Self {
        todo!()
    }
    fn dot3(&self, _rhs: Self) -> Scalar {
        todo!()
    }
}

pub trait Vector4dBridgeOps<Scalar: ScalarContract, Vector3d>: Vector4dCoreOps<Scalar, Vector3d> + VectorBridgeOps<Scalar, 4> {
    fn classify_homogeneous_w(&self) -> HomogeneousWState {
        todo!()
    }
    /// # Panics
    /// - Panics when `w == 0` (direction/point-at-infinity) under strict point dehomogenization.
    /// - Panics when `w` is non-finite under strict point dehomogenization mode.
    fn homogenized_to_vec3_strict(&self) -> HomogeneousPointOrDirection<Vector3d> {
        todo!()
    }
    /// Non-panicking dehomogenization policy:
    /// - if `w == 0` or non-finite, treat value as direction branch.
    fn homogenized_to_vec3_or_direction(&self) -> HomogeneousPointOrDirection<Vector3d> {
        todo!()
    }
    /// Returns `(xyz, is_direction)` where `is_direction == true` means `w == 0` or non-finite
    /// under the configured classification mode.
    fn dehomogenize_point_vs_direction(&self) -> (Vector3d, bool) {
        todo!()
    }
}

pub trait VectorContract<Scalar: ScalarContract, const D: usize>: VectorCoreOps<Scalar, D> + VectorFieldOps<Scalar, D> + VectorBridgeOps<Scalar, D> {}
impl<T, Scalar: ScalarContract, const D: usize> VectorContract<Scalar, D> for T where
    T: VectorCoreOps<Scalar, D> + VectorFieldOps<Scalar, D> + VectorBridgeOps<Scalar, D>
{
}

pub trait Vector2dContract<Scalar: ScalarContract>: Vector2dCoreOps<Scalar> + Vector2dFieldOps<Scalar> {}
impl<T, Scalar: ScalarContract> Vector2dContract<Scalar> for T where T: Vector2dCoreOps<Scalar> + Vector2dFieldOps<Scalar> {}

pub trait Vector3dContract<Scalar: ScalarContract>: Vector3dCoreOps<Scalar> + Vector3dFieldOps<Scalar> {}
impl<T, Scalar: ScalarContract> Vector3dContract<Scalar> for T where T: Vector3dCoreOps<Scalar> + Vector3dFieldOps<Scalar> {}

pub trait Vector4dContract<Scalar: ScalarContract, Vector3d>:
    Vector4dCoreOps<Scalar, Vector3d> + Vector4dFieldOps<Scalar> + Vector4dBridgeOps<Scalar, Vector3d>
{
}
impl<T, Scalar: ScalarContract, Vector3d> Vector4dContract<Scalar, Vector3d> for T where
    T: Vector4dCoreOps<Scalar, Vector3d> + Vector4dFieldOps<Scalar> + Vector4dBridgeOps<Scalar, Vector3d>
{
}
