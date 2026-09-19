//! Finite canonical voxel scopes represented by bounded local offsets.

use super::*;

/// Finite voxel scope anchored at one canonical semantic position.
///
/// `min` and `max` are deliberately small local offsets from `anchor`; they are
/// shape/scope parameters, never universe-wide coordinates. This lets a scope
/// cross any USF digit boundary without losing its semantic location.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VoxelBounds {
    anchor: VoxelQueryPosition,
    min: Vec3,
    max: Vec3,
}

impl VoxelBounds {
    pub fn new(anchor: VoxelQueryPosition, min: Vec3, max: Vec3) -> Self {
        Self {
            anchor,
            min: min.min(max),
            max: min.max(max),
        }
    }

    pub const fn anchor(self) -> VoxelQueryPosition {
        self.anchor
    }

    pub const fn min_offset(self) -> Vec3 {
        self.min
    }

    pub const fn max_offset(self) -> Vec3 {
        self.max
    }

    pub fn expanded(self, amount: f32) -> Self {
        let amount = Vec3::splat(amount.max(0.0));
        Self::new(self.anchor, self.min - amount, self.max + amount)
    }

    pub fn intersects(self, other: Self) -> bool {
        let reach = self.max_abs_extent() + other.max_abs_extent() + 1.0;
        let Ok(other_anchor) = other.anchor.relative_to(self.anchor, reach) else {
            return false;
        };
        let other_min = other_anchor + other.min;
        let other_max = other_anchor + other.max;
        self.min.cmple(other_max).all() && self.max.cmpge(other_min).all()
    }

    pub fn contains(self, point: VoxelQueryPosition) -> bool {
        let Ok(local) = point.relative_to(self.anchor, self.max_abs_extent() + 1.0) else {
            return false;
        };
        local.cmpge(self.min).all() && local.cmple(self.max).all()
    }

    fn max_abs_extent(self) -> f32 {
        self.min.abs().max(self.max.abs()).max_element()
    }
}
