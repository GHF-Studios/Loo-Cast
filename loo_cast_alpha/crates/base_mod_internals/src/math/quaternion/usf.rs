use super::super::scalar::usf::UsfScalar;
pub use super::aliases::{UsfOrNormalMat3, UsfOrNormalQuaternion};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UsfQuaternion {
    // High-precision quaternion representation for cross-scale/ultra-precision workflows.
    // Rotation usage still expects unit normalization semantics.
    pub x: UsfScalar,
    pub y: UsfScalar,
    pub z: UsfScalar,
    pub w: UsfScalar,
}

impl super::shared::QuaternionCoreOps for UsfQuaternion {}

impl super::shared::QuaternionFieldOps for UsfQuaternion {}

impl super::shared::QuaternionBridgeOps for UsfQuaternion {}
