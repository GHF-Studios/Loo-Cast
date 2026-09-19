//! Semantic voxel mutations and disposable bounded dense-chart projections.

use super::*;

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

    /// Returns the authoritative sample produced by this edit at one canonical
    /// semantic query point.
    pub fn apply_to_sample(self, point: VoxelQueryPosition, sample: VoxelSample) -> VoxelSample {
        let Some(local) = self.localized(point, 0.0) else {
            return sample;
        };
        local.apply_to_sample(Vec3::ZERO, sample)
    }

    /// Projects this semantic edit into one bounded dense-representation chart.
    /// The projection is disposable; the returned value never becomes edit
    /// identity or persistence state.
    pub(crate) fn localized(
        self,
        anchor: VoxelQueryPosition,
        extra_extent: f32,
    ) -> Option<VoxelLocalEdit> {
        let brush = match self {
            Self::Add { brush, .. } | Self::Remove { brush } | Self::Paint { brush, .. } => brush,
        };
        let max_abs = brush.radius() + EDIT_INFLUENCE_MARGIN + extra_extent.max(0.0) + 1.0;
        let center = brush.center().relative_to(anchor, max_abs).ok()?;
        let radius = brush.radius();

        Some(match self {
            Self::Add { material, .. } => VoxelLocalEdit::Add {
                center,
                radius,
                material,
            },
            Self::Remove { .. } => VoxelLocalEdit::Remove { center, radius },
            Self::Paint { material, .. } => VoxelLocalEdit::Paint {
                center,
                radius,
                material,
            },
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct VoxelLocalBounds {
    pub(crate) min: Vec3,
    pub(crate) max: Vec3,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum VoxelLocalEdit {
    Add {
        center: Vec3,
        radius: f32,
        material: VoxelMaterialId,
    },
    Remove {
        center: Vec3,
        radius: f32,
    },
    Paint {
        center: Vec3,
        radius: f32,
        material: VoxelMaterialId,
    },
}

impl VoxelLocalEdit {
    pub(crate) fn influence_bounds(self) -> VoxelLocalBounds {
        let (center, radius, margin) = match self {
            Self::Add { center, radius, .. } | Self::Remove { center, radius } => {
                (center, radius, EDIT_INFLUENCE_MARGIN)
            }
            Self::Paint { center, radius, .. } => (center, radius, 0.0),
        };
        let extent = Vec3::splat(radius + margin);
        VoxelLocalBounds {
            min: center - extent,
            max: center + extent,
        }
    }

    pub(crate) fn apply_to_sample(self, point: Vec3, mut sample: VoxelSample) -> VoxelSample {
        let (center, radius) = match self {
            Self::Add { center, radius, .. }
            | Self::Remove { center, radius }
            | Self::Paint { center, radius, .. } => (center, radius),
        };
        let brush_distance = point.distance(center) - radius;

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
