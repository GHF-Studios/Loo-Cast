#![allow(dead_code)]

use super::super::aliases::OutputMode;
use super::super::field::Field;
use super::super::scalar::aliases::{UsfOrNormalDecimalScalar, UsfOrNormalScalar};
use super::super::scalar::usf::UsfScalar;
pub use super::aliases::{UsfOrNormalVector, VectorOrScalar};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UsfVector<const D: usize> {
    // CONTRACT: D >= 2. D == 1 is scalar-equivalent and forbidden by model contract.
    pub(super) vector_components: Field<[UsfScalar; D]>,
}

pub type UsfVector2d = UsfVector<2>;
pub type UsfVector3d = UsfVector<3>;
pub type UsfVector4d = UsfVector<4>;

impl<const D: usize> super::shared::VectorCoreOps<D> for UsfVector<D> {}

impl super::shared::Vector2dFieldOps for UsfVector<2> {}
impl super::shared::Vector3dFieldOps for UsfVector<3> {}
impl super::shared::Vector4dFieldOps for UsfVector<4> {}

impl super::shared::Vector2dCoreOps for UsfVector<2> {}
impl super::shared::Vector3dCoreOps for UsfVector<3> {}
impl super::shared::Vector4dCoreOps for UsfVector<4> {}

impl<const D: usize> super::shared::VectorBridgeOps<D> for UsfVector<D> {}
impl super::shared::Vector4dBridgeOps for UsfVector<4> {}
