//! Constructive edits over the authoritative voxel field.

use bevy::prelude::Vec3;

use super::{VoxelMaterialId, VoxelSample};

/// Analytic shape used by a voxel edit.
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
    /// Returns the authoritative sample produced by this edit at `point`.
    pub fn apply_to_sample(self, point: Vec3, mut sample: VoxelSample) -> VoxelSample {
        let brush_distance = match self {
            Self::Add { brush, .. } | Self::Remove { brush } | Self::Paint { brush, .. } => {
                brush.signed_distance(point)
            }
        };

        match self {
            Self::Add { material, .. } => {
                if brush_distance < sample.distance.0 {
                    sample.distance.0 = brush_distance;
                    if sample.distance.is_solid() {
                        sample.material = material;
                    }
                }
            }
            Self::Remove { .. } => {
                sample.distance.0 = sample.distance.0.max(-brush_distance);
                if sample.distance.is_empty() {
                    sample.material = VoxelMaterialId::VOID;
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
    fn sphere_add_uses_sdf_union() {
        let edit = VoxelEdit::Add {
            brush: VoxelBrush::sphere(Vec3::ZERO, 2.0),
            material: VoxelMaterialId::ROCK,
        };
        let sample = edit.apply_to_sample(Vec3::ZERO, VoxelSample::empty(100.0));

        assert_eq!(sample.distance.0, -2.0);
        assert_eq!(sample.material, VoxelMaterialId::ROCK);
    }

    #[test]
    fn sphere_remove_uses_sdf_difference() {
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
}
