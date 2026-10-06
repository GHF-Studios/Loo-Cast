//! Controlled request for additional capability refinement.

use bevy::prelude::*;

use super::super::SpatialScale;

/// Requested additional spatial detail for capability-specific realization.
///
/// This is not another generic interest volume. `UsfRefinementPlan` turns the
/// requested tip into a current-relative ancestor branch over the Scale Slices
/// supported by each capability.
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct SpatialRefinementDemand {
    minimum_scale: Option<SpatialScale>,
    half_extent_native: Vec3,
    half_extent_metres: Option<Vec3>,
}

impl SpatialRefinementDemand {
    pub fn cuboid(half_extent_native: Vec3) -> Self {
        Self {
            minimum_scale: None,
            half_extent_native: sanitize_extent(half_extent_native),
            half_extent_metres: None,
        }
    }

    pub fn cuboid_metres(half_extent_metres: Vec3) -> Self {
        let half_extent_metres = sanitize_extent(half_extent_metres);
        Self {
            minimum_scale: None,
            half_extent_native: half_extent_metres,
            half_extent_metres: Some(half_extent_metres),
        }
    }

    pub const fn minimum_scale(&self) -> Option<SpatialScale> {
        self.minimum_scale
    }

    pub const fn half_extent_native(&self) -> Vec3 {
        self.half_extent_native
    }

    pub fn half_extent_native_at(&self, scale: SpatialScale) -> Vec3 {
        self.half_extent_metres
            .map_or(self.half_extent_native, |metres| {
                Vec3::new(
                    scale.metres_to_native_f32(metres.x),
                    scale.metres_to_native_f32(metres.y),
                    scale.metres_to_native_f32(metres.z),
                )
            })
    }

    pub fn request_through(&mut self, scale: SpatialScale) {
        self.minimum_scale = Some(scale);
    }

    pub fn clear(&mut self) {
        self.minimum_scale = None;
    }
}

fn sanitize_extent(value: Vec3) -> Vec3 {
    Vec3::new(
        sanitize_axis(value.x),
        sanitize_axis(value.y),
        sanitize_axis(value.z),
    )
}

fn sanitize_axis(value: f32) -> f32 {
    if value.is_finite() {
        value.max(0.0)
    } else {
        0.0
    }
}
