#![allow(dead_code)]

use super::super::field::Field;
use super::super::quaternion::usf::UsfQuaternion;
use super::super::scalar::usf::UsfScalar;
use super::super::vector::usf::UsfVector;
pub use super::aliases::{UsfOrNormalRotationQuaternion, UsfOrNormalTranslationVector};

#[derive(Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct UsfTranslation<const D: usize>(pub(super) Field<UsfVector<D>>);

#[derive(Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct UsfRotation(pub(super) Field<UsfQuaternion>);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UsfScale {
    pub(super) x: Field<UsfScalar>,
    pub(super) y: Field<UsfScalar>,
    pub(super) z: Field<UsfScalar>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UsfTransform {
    pub(super) translation: Field<UsfTranslation<3>>,
    pub(super) rotation: Field<UsfRotation>,
    pub(super) scale: Field<UsfScale>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UsfIsometry<const D: usize>;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UsfProjection<const D: usize>;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UsfFrame<const D: usize>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UsfVelocity<const D: usize>;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UsfAngularVelocity;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UsfAcceleration<const D: usize>;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UsfAngularAcceleration;

impl<const D: usize> super::shared::TranslationCoreOps<D> for UsfTranslation<D> {}
impl<const D: usize> super::shared::TranslationFieldOps<D> for UsfTranslation<D> {}
impl<const D: usize> super::shared::TranslationBridgeOps<D> for UsfTranslation<D> {}

impl super::shared::RotationCoreOps for UsfRotation {}
impl super::shared::RotationFieldOps for UsfRotation {}
impl super::shared::RotationBridgeOps for UsfRotation {}

impl super::shared::ScaleCoreOps for UsfScale {}
impl super::shared::ScaleFieldOps for UsfScale {}
impl super::shared::ScaleBridgeOps for UsfScale {}

impl super::shared::TransformCoreOps for UsfTransform {}
impl super::shared::TransformFieldOps for UsfTransform {}
impl super::shared::TransformBridgeOps for UsfTransform {}
