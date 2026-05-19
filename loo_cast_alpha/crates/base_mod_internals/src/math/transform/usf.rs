use super::super::field::Field;
use super::super::quaternion::usf::UsfQuaternion;
use super::super::vector::usf::UsfVector;

#[derive(Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct UsfTranslation3d(pub Field<UsfVector<3>>);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UsfRotation3d(pub Field<UsfQuaternion>);

#[derive(Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct UsfScale3d(pub Field<UsfVector<3>>);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UsfTransform3d {
    pub translation: Field<UsfTranslation3d>,
    pub rotation: Field<UsfRotation3d>,
    pub scale: Field<UsfScale3d>,
}

impl super::shared::Translation3dCoreOps for UsfTranslation3d {}
impl super::shared::Translation3dFieldOps for UsfTranslation3d {}
impl super::shared::Translation3dBridgeOps for UsfTranslation3d {}

impl super::shared::Rotation3dCoreOps for UsfRotation3d {}
impl super::shared::Rotation3dFieldOps for UsfRotation3d {}
impl super::shared::Rotation3dBridgeOps for UsfRotation3d {}

impl super::shared::Scale3dCoreOps for UsfScale3d {}
impl super::shared::Scale3dFieldOps for UsfScale3d {}
impl super::shared::Scale3dBridgeOps for UsfScale3d {}

impl super::shared::Transform3dCoreOps for UsfTransform3d {}
impl super::shared::Transform3dFieldOps for UsfTransform3d {}
impl super::shared::Transform3dBridgeOps for UsfTransform3d {}
