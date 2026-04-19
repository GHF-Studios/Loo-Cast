#![allow(dead_code)]

use super::super::quaternion::normal::NormalQuaternion;
use super::super::scalar::normal::{NormalDecimalScalar, NormalScalar};
use super::super::vector::normal::NormalVector;

pub type NormalTranslation3f32 = bevy::math::Vec3;
pub type NormalRotationf32 = bevy::math::Quat;
pub type NormalScalef32 = bevy::math::Vec3;
pub type NormalTransformf32 = bevy::prelude::Transform;

impl super::shared::TranslationCoreOps<NormalScalar, NormalVector<3>, 3> for NormalTranslation3f32 {}
impl super::shared::TranslationFieldOps<NormalScalar, NormalVector<3>, 3> for NormalTranslation3f32 {}
impl super::shared::TranslationBridgeOps<NormalScalar, NormalVector<3>, 3> for NormalTranslation3f32 {}

impl super::shared::RotationCoreOps<NormalQuaternion> for NormalRotationf32 {}
impl super::shared::RotationFieldOps<NormalQuaternion> for NormalRotationf32 {}
impl super::shared::RotationBridgeOps<NormalQuaternion> for NormalRotationf32 {}

impl super::shared::ScaleCoreOps<NormalDecimalScalar> for NormalScalef32 {}
impl super::shared::ScaleFieldOps<NormalDecimalScalar> for NormalScalef32 {}
impl super::shared::ScaleBridgeOps<NormalDecimalScalar> for NormalScalef32 {}

impl super::shared::TransformCoreOps<NormalTranslation3f32, NormalRotationf32, NormalScalef32> for NormalTransformf32 {}
impl super::shared::TransformFieldOps<NormalTranslation3f32, NormalRotationf32, NormalScalef32> for NormalTransformf32 {}
impl super::shared::TransformBridgeOps<NormalTranslation3f32, NormalRotationf32, NormalScalef32> for NormalTransformf32 {}
