//! Constructive edits over the authoritative voxel field.

use bevy::prelude::Vec3;

use super::{VoxelMaterialId, VoxelSample};

/// Number of world units around an analytic brush surface in which CSG edits
/// are allowed to update signed-distance values.
///
/// A true global SDF Boolean can change distance magnitudes arbitrarily far
/// from the edited surface. That is mathematically useful, but it defeats local
/// sparse editing. A narrow influence band preserves enough exterior/interior
/// distance data for smooth extraction while giving each edit finite bounds.
pub const EDIT_INFLUENCE_MARGIN: f32 = 2.0;

/// Axis-aligned bounds in the current `VoxelWorld`-local compatibility chart.
///
/// Pass B replaces this flat addressing authority with a canonical semantic
/// scope; keeping it explicit here prevents this bounded local type from being
/// mistaken for universe-wide identity during Pass A.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VoxelBounds {
    pub min: Vec3,
    pub max: Vec3,
}

impl VoxelBounds {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self {
            min: min.min(max),
            max: min.max(max),
        }
    }

    pub fn expanded(self, amount: f32) -> Self {
        let amount = Vec3::splat(amount.max(0.0));
        Self::new(self.min - amount, self.max + amount)
    }

    pub fn intersects(self, other: Self) -> bool {
        self.min.cmple(other.max).all() && self.max.cmpge(other.min).all()
    }

    pub fn contains(self, point: Vec3) -> bool {
        point.cmpge(self.min).all() && point.cmple(self.max).all()
    }
}

/// Analytic shape used by a voxel edit. Brush centers are likewise expressed
/// in the current bounded `VoxelWorld`-local compatibility chart until Pass B.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VoxelBrush {
    Sphere { center: Vec3, radius: f32 },
}

impl VoxelBrush {
    pub const fn sphere(center: Vec3, radius: f32) -> Self {
        Self::Sphere { center, radius }
    }

    #[inline]
    pub fn signed_distance(self, point: Vec3) -> f32 {
        match self {
            Self::Sphere { center, radius } => point.distance(center) - radius.max(0.0),
        }
    }

    pub fn bounds(self) -> VoxelBounds {
        match self {
            Self::Sphere { center, radius } => {
                let radius = Vec3::splat(radius.max(0.0));
                VoxelBounds::new(center - radius, center + radius)
            }
        }
    }
}

/// Semantic mutation of volumetric matter.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VoxelEdit {
    Add {
        brush: VoxelBrush,
        material: VoxelMaterialId,
    },
    Remove {
        brush: VoxelBrush,
    },
    Paint {
        brush: VoxelBrush,
        material: VoxelMaterialId,
    },
}

impl VoxelEdit {
    pub fn influence_bounds(self) -> VoxelBounds {
        match self {
            Self::Add { brush, .. } | Self::Remove { brush } => {
                brush.bounds().expanded(EDIT_INFLUENCE_MARGIN)
            }
            Self::Paint { brush, .. } => brush.bounds(),
        }
    }

    /// Returns the authoritative sample produced by this edit at `point`.
    pub fn apply_to_sample(self, point: Vec3, mut sample: VoxelSample) -> VoxelSample {
        let brush_distance = match self {
            Self::Add { brush, .. } | Self::Remove { brush } | Self::Paint { brush, .. } => {
                brush.signed_distance(point)
            }
        };

        match self {
            Self::Add { material, .. } => {
                if brush_distance <= EDIT_INFLUENCE_MARGIN && brush_distance < sample.distance.0 {
                    sample.distance.0 = brush_distance;
                    if sample.distance.is_solid() {
                        sample.material = material;
                    }
                }
            }
            Self::Remove { .. } => {
                if brush_distance <= EDIT_INFLUENCE_MARGIN {
                    sample.distance.0 = sample.distance.0.max(-brush_distance);
                    if sample.distance.is_empty() {
                        sample.material = VoxelMaterialId::VOID;
                    }
                }
            }
            Self::Paint { material, .. } => {
                if brush_distance <= 0.0 && sample.distance.is_solid() {
                    sample.material = material;
                }
            }
        }

        sample
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sphere_add_uses_sdf_union_near_the_brush() {
        let edit = VoxelEdit::Add {
            brush: VoxelBrush::sphere(Vec3::ZERO, 2.0),
            material: VoxelMaterialId::ROCK,
        };
        let sample = edit.apply_to_sample(Vec3::ZERO, VoxelSample::empty(100.0));

        assert_eq!(sample.distance.0, -2.0);
        assert_eq!(sample.material, VoxelMaterialId::ROCK);
    }

    #[test]
    fn sphere_remove_uses_sdf_difference_near_the_brush() {
        let edit = VoxelEdit::Remove {
            brush: VoxelBrush::sphere(Vec3::ZERO, 2.0),
        };
        let sample = edit.apply_to_sample(
            Vec3::ZERO,
            VoxelSample::new(-10.0, VoxelMaterialId::ROCK),
        );

        assert_eq!(sample.distance.0, 2.0);
        assert_eq!(sample.material, VoxelMaterialId::VOID);
    }

    #[test]
    fn edits_do_not_rewrite_the_field_outside_their_influence_band() {
        let before = VoxelSample::empty(100.0);
        let edit = VoxelEdit::Add {
            brush: VoxelBrush::sphere(Vec3::ZERO, 2.0),
            material: VoxelMaterialId::ROCK,
        };

        assert_eq!(
            edit.apply_to_sample(Vec3::new(20.0, 0.0, 0.0), before),
            before
        );
    }
}
