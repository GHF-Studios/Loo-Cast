#![allow(dead_code)]

use super::super::field::Field;
use super::super::scalar::normal::NormalDecimalScalar;
use super::super::scalar::usf::UsfScalar;
pub use super::aliases::{UsfOrNormalVector, VectorOrScalar};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UsfVector<const D: usize> {
    // CONTRACT: D >= 2. D == 1 is scalar-equivalent and forbidden by model contract.
    pub(super) lanes: Field<[UsfScalar; D]>,
}

pub type UsfVector2d = UsfVector<2>;
pub type UsfVector3d = UsfVector<3>;
pub type UsfVector4d = UsfVector<4>;

impl<const D: usize> UsfVector<D> {
    pub fn zero() -> Self {
        todo!()
    }
    pub fn one() -> Self {
        todo!()
    }
    pub fn splat(_value: UsfScalar) -> Self {
        todo!()
    }
    /// # Panics
    /// - Panics if `D < 2` is rejected by runtime validation.
    pub fn from_lanes(_lanes: [UsfScalar; D]) -> Self {
        todo!()
    }
    pub fn to_lanes(&self) -> [UsfScalar; D] {
        todo!()
    }
    /// # Panics
    /// - Panics if the vector has zero length.
    pub fn normalize(&self) -> Self {
        todo!()
    }
    pub fn floor(&self) -> Self {
        todo!()
    }
    pub fn ceil(&self) -> Self {
        todo!()
    }
    pub fn round(&self) -> Self {
        todo!()
    }
    pub fn fract(&self) -> Self {
        todo!()
    }
    pub fn neg(&self) -> Self {
        todo!()
    }
    pub fn abs(&self) -> Self {
        todo!()
    }
    pub fn add(&self, _rhs: UsfVector<D>) -> Self {
        todo!()
    }
    pub fn sub(&self, _rhs: UsfVector<D>) -> Self {
        todo!()
    }
    pub fn mul(&self, _rhs: UsfVector<D>) -> Self {
        todo!()
    }
    /// # Panics
    /// - Panics if any corresponding lane in `rhs` is zero.
    pub fn div(&self, _rhs: UsfVector<D>) -> Self {
        todo!()
    }
    /// # Panics
    /// - Panics if any corresponding lane in `rhs` is zero.
    pub fn rem(&self, _rhs: UsfVector<D>) -> Self {
        todo!()
    }
    pub fn min(&self, _rhs: UsfVector<D>) -> Self {
        todo!()
    }
    pub fn max(&self, _rhs: UsfVector<D>) -> Self {
        todo!()
    }
    /// # Panics
    /// - Panics if any lane has `lo > hi`.
    pub fn clamp(&self, _lo: UsfVector<D>, _hi: UsfVector<D>) -> Self {
        todo!()
    }
    pub fn lerp_normal(&self, _rhs: UsfVector<D>, _t: NormalDecimalScalar) -> Self {
        todo!()
    }
    pub fn lerp_usf(&self, _rhs: UsfVector<D>, _t: UsfScalar) -> Self {
        todo!()
    }
    pub fn smoothstep_normal(&self, _rhs: UsfVector<D>, _t: NormalDecimalScalar) -> Self {
        todo!()
    }
    pub fn smoothstep_usf(&self, _rhs: UsfVector<D>, _t: UsfScalar) -> Self {
        todo!()
    }
    pub fn dot_usf(&self, _rhs: UsfVector<D>) -> UsfScalar {
        todo!()
    }
    pub fn dot_normal(&self, _rhs: UsfVector<D>) -> NormalDecimalScalar {
        todo!()
    }
    pub fn distance_usf(&self, _rhs: UsfVector<D>) -> UsfScalar {
        todo!()
    }
    pub fn distance_normal(&self, _rhs: UsfVector<D>) -> NormalDecimalScalar {
        todo!()
    }
    /// # Panics
    /// - Panics if either vector has zero length.
    pub fn angle_between_usf(&self, _rhs: UsfVector<D>) -> UsfScalar {
        todo!()
    }
    /// # Panics
    /// - Panics if either vector has zero length.
    pub fn angle_between_normal(&self, _rhs: UsfVector<D>) -> NormalDecimalScalar {
        todo!()
    }
    /// # Panics
    /// - Panics if `onto` is the zero vector.
    pub fn project(&self, _onto: UsfVector<D>) -> Self {
        todo!()
    }
    /// # Panics
    /// - Panics if `onto` is the zero vector.
    pub fn reject(&self, _onto: UsfVector<D>) -> Self {
        todo!()
    }
    /// # Panics
    /// - Panics if `normal` is the zero vector.
    pub fn reflect(&self, _normal: UsfVector<D>) -> Self {
        todo!()
    }
    pub fn mul_elem(&self, _rhs: UsfVector<D>) -> Self {
        todo!()
    }
    /// # Panics
    /// - Panics if any corresponding lane in `rhs` is zero.
    pub fn div_elem(&self, _rhs: UsfVector<D>) -> Self {
        todo!()
    }
    pub fn fma(&self, _b: UsfVector<D>, _c: UsfVector<D>) -> Self {
        todo!()
    }
    pub fn add_scalar(&self, _rhs: UsfScalar) -> Self {
        todo!()
    }
    pub fn sub_scalar(&self, _rhs: UsfScalar) -> Self {
        todo!()
    }
    pub fn mul_scalar(&self, _rhs: UsfScalar) -> Self {
        todo!()
    }
    /// # Panics
    /// - Panics if `rhs` is zero.
    pub fn div_scalar(&self, _rhs: UsfScalar) -> Self {
        todo!()
    }
    pub fn scale(&self, _rhs: UsfScalar) -> Self {
        todo!()
    }
    pub fn get_dimension(&self) -> usize {
        todo!()
    }
    pub fn get_length_usf(&self) -> UsfScalar {
        todo!()
    }
    pub fn get_length_normal(&self) -> NormalDecimalScalar {
        todo!()
    }
    pub fn get_length_squared_usf(&self) -> UsfScalar {
        todo!()
    }
    pub fn get_length_squared_normal(&self) -> NormalDecimalScalar {
        todo!()
    }
    /// # Panics
    /// - Panics if `index` is out of bounds.
    pub fn get_lane(&self, _index: usize) -> UsfScalar {
        todo!()
    }
    /// # Panics
    /// - Panics if `index` is out of bounds.
    /// - Panics if the lane is immutable under runtime field mutability policy.
    pub fn set_lane(&mut self, _index: usize, _value: UsfScalar) {
        todo!()
    }
}

impl UsfVector<3> {
    pub fn cross(&self, _rhs: UsfVector<3>) -> Self {
        todo!()
    }
}

impl<const D: usize> super::shared::VectorCoreOps<UsfScalar, D> for UsfVector<D> {}

impl super::shared::Vector2dFieldOps<UsfScalar> for UsfVector<2> {}
impl super::shared::Vector3dFieldOps<UsfScalar> for UsfVector<3> {}
impl super::shared::Vector4dFieldOps<UsfScalar> for UsfVector<4> {}

impl super::shared::Vector2dCoreOps<UsfScalar> for UsfVector<2> {}
impl super::shared::Vector3dCoreOps<UsfScalar> for UsfVector<3> {}
impl super::shared::Vector4dCoreOps<UsfScalar, UsfVector<3>> for UsfVector<4> {}

impl<const D: usize> super::shared::VectorBridgeOps<UsfScalar, D> for UsfVector<D> {}
impl super::shared::Vector4dBridgeOps<UsfScalar, UsfVector<3>> for UsfVector<4> {}
