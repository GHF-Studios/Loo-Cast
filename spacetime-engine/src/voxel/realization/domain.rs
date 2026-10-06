//! Voxel capability participation and local realization policy.

use super::*;

const DEFAULT_REFINEMENT_ACTIVATION_NATIVE: f32 = 8_192.0;
const DEFAULT_LOCAL_PATCH_HALF_EXTENT_NATIVE: f32 = 32.0;

/// Scale-Slice participation and current voxel realization policy.
///
/// The masks describe mechanism capability. Activation radius and patch extent
/// are representation policy, not semantic body identity.
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct VoxelScaleDomain {
    realization_slices: UsfChartMask,
    collision_slices: UsfChartMask,
    editing_slices: UsfChartMask,
    refinement_activation_native: f32,
    local_patch_half_extent_native: f32,
}

impl VoxelScaleDomain {
    pub fn contiguous(lower: SpatialScale, upper: SpatialScale) -> Self {
        Self {
            realization_slices: UsfChartMask::inclusive_range(lower, upper),
            collision_slices: UsfChartMask::NONE,
            editing_slices: UsfChartMask::NONE,
            refinement_activation_native: DEFAULT_REFINEMENT_ACTIVATION_NATIVE,
            local_patch_half_extent_native: DEFAULT_LOCAL_PATCH_HALF_EXTENT_NATIVE,
        }
    }

    pub const fn realization_slices(self) -> UsfChartMask {
        self.realization_slices
    }

    pub const fn collision_slices(self) -> UsfChartMask {
        self.collision_slices
    }

    pub const fn editing_slices(self) -> UsfChartMask {
        self.editing_slices
    }

    pub const fn refinement_activation_native(self) -> f32 {
        self.refinement_activation_native
    }

    pub const fn local_patch_half_extent_native(self) -> f32 {
        self.local_patch_half_extent_native
    }

    pub const fn realizes(self, scale: SpatialScale) -> bool {
        self.realization_slices.contains(scale)
    }

    pub const fn collides(self, scale: SpatialScale) -> bool {
        self.collision_slices.contains(scale)
    }

    pub const fn editable(self, scale: SpatialScale) -> bool {
        self.editing_slices.contains(scale)
    }

    pub fn with_collision_slices(mut self, slices: UsfChartMask) -> Self {
        self.collision_slices = slices;
        self
    }

    pub fn with_editing_slices(mut self, slices: UsfChartMask) -> Self {
        self.editing_slices = slices;
        self
    }

    pub fn with_refinement_activation_native(mut self, radius: f32) -> Self {
        if radius.is_finite() && radius > 0.0 {
            self.refinement_activation_native = radius;
        }
        self
    }

    pub fn with_local_patch_half_extent_native(mut self, extent: f32) -> Self {
        if extent.is_finite() && extent > 0.0 {
            self.local_patch_half_extent_native = extent;
        }
        self
    }
}
