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

    /// Explicitly projects one canonical voxel query into a realization chart.
    ///
    /// This changes representation precision, not semantic identity. Callers
    /// should keep the original canonical point when they still need finer
    /// information.
    pub fn reexpressed_at(
        self,
        scale: crate::spatial::SpatialScale,
    ) -> Result<Self, UsfPositionError> {
        self.0.reexpressed_at(scale).map(Self)
    }

    /// Measures this canonical point in the reference query's Scale Slice.
    ///
    /// Query points may carry finer semantic digits than the local dense
    /// realization. The reference chart owns the numerical units of the result.
    pub fn relative_to(self, origin: Self, max_abs: f32) -> Result<Vec3, UsfPositionError> {
        self.0.relative_at_scale_bounded(
            &origin.0,
            origin.usf().leaf_scale(),
            max_abs,
        )
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
