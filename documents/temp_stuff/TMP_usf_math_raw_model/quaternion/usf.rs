#![allow(dead_code)]

use super::super::aliases::OutputMode;
use super::super::field::Field;
use super::super::scalar::aliases::{UsfOrNormalFractionalScalar, UsfOrNormalScalar};
use super::super::scalar::usf::UsfScalar;
use super::super::vector::aliases::UsfOrNormalVector;
pub use super::aliases::{UsfOrNormalMat3, UsfOrNormalQuaternion};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UsfQuaternion {
    // High-precision quaternion representation for cross-scale/ultra-precision workflows.
    // Rotation usage still expects unit normalization semantics.
    pub(super) x: Field<UsfScalar>,
    pub(super) y: Field<UsfScalar>,
    pub(super) z: Field<UsfScalar>,
    pub(super) w: Field<UsfScalar>,
}

impl super::shared::QuaternionCoreOps for UsfQuaternion {}

impl super::shared::QuaternionFieldOps for UsfQuaternion {}

impl super::shared::QuaternionBridgeOps for UsfQuaternion {}
