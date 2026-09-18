//! Constructive edits over the authoritative voxel field.

use bevy::prelude::Vec3;

use crate::spatial::{UsfPosition, UsfPositionError};

use super::{VoxelMaterialId, VoxelSample};

/// Number of leaf-native units around an analytic brush surface in which CSG
/// edits are allowed to update signed-distance values.
///
/// A true global SDF Boolean can change distance magnitudes arbitrarily far
/// from the edited surface. That is mathematically useful, but it defeats local
/// sparse editing. A narrow influence band preserves enough exterior/interior
/// distance data for smooth extraction while giving each edit finite bounds.
pub const EDIT_INFLUENCE_MARGIN: f32 = 2.0;

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

#[cfg(test)]
mod tests {
    use super::*;

    fn position(local: Vec3) -> VoxelQueryPosition {
        VoxelQueryPosition::from_scale0_local(local).unwrap()
    }

    #[test]
    fn canonical_bounds_cross_a_usf_digit_boundary_without_flat_coordinates() {
        let anchor = position(Vec3::new(499.0, 0.0, 0.0));
        let bounds = VoxelBounds::new(anchor, Vec3::splat(-3.0), Vec3::splat(3.0));
        let point = anchor.translated(Vec3::new(2.5, 0.0, 0.0)).unwrap();

        assert!(bounds.contains(point));
    }

    #[test]
    fn sphere_add_uses_sdf_union_near_the_brush() {
        let center = position(Vec3::ZERO);
        let edit = VoxelEdit::Add {
            brush: VoxelBrush::sphere(center, 2.0),
            material: VoxelMaterialId::ROCK,
        };
        let sample = edit.apply_to_sample(center, VoxelSample::empty(100.0));

        assert_eq!(sample.distance.0, -2.0);
        assert_eq!(sample.material, VoxelMaterialId::ROCK);
    }

    #[test]
    fn sphere_remove_uses_sdf_difference_near_the_brush() {
        let center = position(Vec3::ZERO);
        let edit = VoxelEdit::Remove {
            brush: VoxelBrush::sphere(center, 2.0),
        };
        let sample = edit.apply_to_sample(center, VoxelSample::new(-10.0, VoxelMaterialId::ROCK));

        assert_eq!(sample.distance.0, 2.0);
        assert_eq!(sample.material, VoxelMaterialId::VOID);
    }

    #[test]
    fn edits_do_not_rewrite_the_field_outside_their_influence_band() {
        let center = position(Vec3::ZERO);
        let before = VoxelSample::empty(100.0);
        let edit = VoxelEdit::Add {
            brush: VoxelBrush::sphere(center, 2.0),
            material: VoxelMaterialId::ROCK,
        };
        let far = center.translated(Vec3::new(20.0, 0.0, 0.0)).unwrap();

        assert_eq!(edit.apply_to_sample(far, before), before);
    }
}
