pub type NormalTranslation3d = bevy::math::Vec3;
pub type NormalRotation3d = bevy::math::Quat;
pub type NormalScale3d = bevy::math::Vec3;
pub type NormalTransform3d = bevy::prelude::Transform;

impl super::shared::Translation3dCoreOps for NormalTranslation3d {}
impl super::shared::Translation3dFieldOps for NormalTranslation3d {}
impl super::shared::Translation3dBridgeOps for NormalTranslation3d {}

impl super::shared::Rotation3dCoreOps for NormalRotation3d {}
impl super::shared::Rotation3dFieldOps for NormalRotation3d {}
impl super::shared::Rotation3dBridgeOps for NormalRotation3d {}

impl super::shared::Scale3dCoreOps for NormalScale3d {}
impl super::shared::Scale3dFieldOps for NormalScale3d {}
impl super::shared::Scale3dBridgeOps for NormalScale3d {}

impl super::shared::Transform3dCoreOps for NormalTransform3d {}
impl super::shared::Transform3dFieldOps for NormalTransform3d {}
impl super::shared::Transform3dBridgeOps for NormalTransform3d {}
