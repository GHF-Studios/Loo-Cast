#![allow(dead_code)]

pub type NormalTranslation3f32 = bevy::math::Vec3;
pub type NormalRotationf32 = bevy::math::Quat;
pub type NormalScalef32 = bevy::math::Vec3;
pub type NormalTransformf32 = bevy::prelude::Transform;

impl super::shared::TranslationCoreOps<3> for NormalTranslation3f32 {}
impl super::shared::TranslationFieldOps<3> for NormalTranslation3f32 {}
impl super::shared::TranslationBridgeOps<3> for NormalTranslation3f32 {}

impl super::shared::RotationCoreOps for NormalRotationf32 {}
impl super::shared::RotationFieldOps for NormalRotationf32 {}
impl super::shared::RotationBridgeOps for NormalRotationf32 {}

impl super::shared::ScaleCoreOps for NormalScalef32 {}
impl super::shared::ScaleFieldOps for NormalScalef32 {}
impl super::shared::ScaleBridgeOps for NormalScalef32 {}

impl super::shared::TransformCoreOps for NormalTransformf32 {}
impl super::shared::TransformFieldOps for NormalTransformf32 {}
impl super::shared::TransformBridgeOps for NormalTransformf32 {}
