//! Canonical semantic positions used by voxel queries and edits.

use super::*;

/// Canonical semantic point used by voxel queries and edits.
///
/// This is deliberately not a runtime Bevy coordinate and not a flat voxel-grid
/// integer. Local dense representations project this point into their own small
/// bounded chart only while evaluating a query or edit.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VoxelQueryPosition(UsfPosition);

impl VoxelQueryPosition {
    pub const fn new(position: UsfPosition) -> Self {
        Self(position)
    }

    pub fn from_scale0_local(local_meters: Vec3) -> Result<Self, UsfPositionError> {
        UsfPosition::from_scale0_local(local_meters).map(Self)
    }

    pub const fn usf(self) -> UsfPosition {
        self.0
    }

    pub fn translated(self, delta: Vec3) -> Result<Self, UsfPositionError> {
        self.0.translated_native(delta).map(Self)
    }

    pub fn relative_to(self, origin: Self, max_abs: f32) -> Result<Vec3, UsfPositionError> {
        self.0.relative_native_bounded(&origin.0, max_abs)
    }
}

impl From<UsfPosition> for VoxelQueryPosition {
    fn from(value: UsfPosition) -> Self {
        Self::new(value)
    }
}

impl From<VoxelQueryPosition> for UsfPosition {
    fn from(value: VoxelQueryPosition) -> Self {
        value.usf()
    }
}
