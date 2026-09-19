//! Analytic local shapes used as semantic voxel-edit brushes.

use super::*;

/// Analytic shape used by a voxel edit. Shape parameters remain bounded/local;
/// only the center carries canonical semantic location.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VoxelBrush {
    Sphere {
        center: VoxelQueryPosition,
        radius: f32,
    },
}

impl VoxelBrush {
    pub fn sphere(center: VoxelQueryPosition, radius: f32) -> Self {
        Self::Sphere {
            center,
            radius: radius.max(0.0),
        }
    }

    pub const fn center(self) -> VoxelQueryPosition {
        match self {
            Self::Sphere { center, .. } => center,
        }
    }

    pub const fn radius(self) -> f32 {
        match self {
            Self::Sphere { radius, .. } => radius,
        }
    }

    #[inline]
    pub fn signed_distance(self, point: VoxelQueryPosition) -> f32 {
        let limit = self.radius() + EDIT_INFLUENCE_MARGIN + 1.0;
        let Ok(delta) = point.relative_to(self.center(), limit) else {
            return f32::INFINITY;
        };
        delta.length() - self.radius()
    }

    pub fn bounds(self) -> VoxelBounds {
        let radius = Vec3::splat(self.radius());
        VoxelBounds::new(self.center(), -radius, radius)
    }
}
