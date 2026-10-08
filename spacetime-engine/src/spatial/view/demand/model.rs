//! Bounded observer relevance and the published demand snapshot.

use crate::spatial::{SpatialScale, UsfPosition};
use bevy::{
    camera::primitives::{Aabb, Frustum},
    math::{Affine3A, DVec3, Vec3A},
    prelude::*,
};

const MIN_PROJECTED_CELL_RADIUS_PIXELS: f32 = 0.75;
const VIEW_RELATIVE_BOUND_NATIVE: f32 = 1_000_000.0;

#[derive(Debug, Clone)]
pub struct UsfViewDemand {
    pub(super) source: Entity,
    pub(super) anchor: UsfPosition,
    pub(super) velocity_metres_per_second: DVec3,
    pub(super) projection_eye_offset_metres: DVec3,
    pub(super) finest_scale: SpatialScale,
    pub(super) camera_translation: Vec3,
    pub(super) camera_rotation: Quat,
    pub(super) frustum: Frustum,
    pub(super) perspective: bool,
    pub(super) perspective_fov: Option<f32>,
    pub(super) perspective_aspect_ratio: Option<f32>,
    pub(super) pixels_per_radian: Option<f32>,
}

impl UsfViewDemand {
    pub const fn source(&self) -> Entity {
        self.source
    }
    pub const fn anchor(&self) -> UsfPosition {
        self.anchor
    }
    pub const fn velocity_metres_per_second(&self) -> DVec3 {
        self.velocity_metres_per_second
    }
    pub const fn finest_scale(&self) -> SpatialScale {
        self.finest_scale
    }

    /// Physical camera-eye offset from the semantic observer anchor in SI metres.
    pub const fn projection_eye_offset_metres(&self) -> DVec3 {
        self.projection_eye_offset_metres
    }

    /// Perspective camera basis plus exact horizontal/vertical half angles.
    pub fn perspective_basis_and_half_angles(&self) -> Option<(DVec3, DVec3, DVec3, f64, f64)> {
        if !self.perspective {
            return None;
        }

        let vertical_fov = f64::from(self.perspective_fov?);
        let aspect = f64::from(self.perspective_aspect_ratio?);
        if !vertical_fov.is_finite()
            || vertical_fov <= 0.0
            || vertical_fov >= std::f64::consts::PI
            || !aspect.is_finite()
            || aspect <= 0.0
        {
            return None;
        }

        let vertical_half = vertical_fov * 0.5;
        let horizontal_half = (vertical_half.tan() * aspect).atan();

        let forward = self.camera_rotation * Vec3::NEG_Z;
        let right = self.camera_rotation * Vec3::X;
        let up = self.camera_rotation * Vec3::Y;
        if !forward.is_finite() || !right.is_finite() || !up.is_finite() {
            return None;
        }

        debug_assert!(vertical_half > 0.0);
        debug_assert!(horizontal_half > 0.0);

        let dvec = |v: Vec3| DVec3::new(f64::from(v.x), f64::from(v.y), f64::from(v.z));
        Some((
            dvec(forward),
            dvec(right),
            dvec(up),
            horizontal_half,
            vertical_half,
        ))
    }

    /// Camera density is a presentation fact, not semantic Scale authority.
    ///
    /// Binary terrain LOD consumes this directly for screen-space error.
    pub const fn pixels_per_radian_for_presentation_resolution(&self) -> Option<f32> {
        self.pixels_per_radian
    }

    pub fn requests_scale(&self, scale: SpatialScale) -> bool {
        scale >= self.finest_scale
    }

    /// Tests one scale-native cell against observer relevance without converting
    /// a potentially enormous scale gap into one render-space float.
    ///
    /// Perspective frustum side planes are invariant under uniform positive
    /// scaling about the camera apex. Near/far planes are deliberately ignored:
    /// renderer clip distances do not own semantic residency.
    pub fn intersects_native_aabb(
        &self,
        scale: SpatialScale,
        center: &UsfPosition,
        half_extent_native: Vec3,
    ) -> bool {
        if !self.requests_scale(scale) {
            return false;
        }
        self.intersects_presentation_native_aabb(scale, center, half_extent_native)
    }

    /// View-only relevance for persistent contextual presentation.
    ///
    /// Unlike capability demand, scenery authored at one bounded coarse Scale
    /// remains a valid representation when the observer zooms to a finer or
    /// coarser Scale. This reuses the same scale-invariant frustum/significance
    /// test without letting view Scale decide semantic residency.
    pub fn intersects_presentation_native_aabb(
        &self,
        scale: SpatialScale,
        center: &UsfPosition,
        half_extent_native: Vec3,
    ) -> bool {
        let half_extent_native = half_extent_native.abs();
        let bound = VIEW_RELATIVE_BOUND_NATIVE.max(half_extent_native.length() + 1.0);
        let Ok(relative) = center.relative_at_scale_bounded(&self.anchor, scale, bound) else {
            return false;
        };

        let radius_native = half_extent_native.length();
        let distance_native = relative.length();

        if let Some(pixels_per_radian) = self.pixels_per_radian
            && distance_native > radius_native.max(f32::EPSILON)
        {
            let angular_radius = (radius_native / distance_native).clamp(0.0, 1.0).asin();
            if angular_radius * pixels_per_radian < MIN_PROJECTED_CELL_RADIUS_PIXELS {
                return false;
            }
        }

        // Orthographic/custom projections stay conservative until they have
        // their own scale-invariant domain test.
        if !self.perspective {
            return true;
        }

        let aabb = Aabb {
            center: Vec3A::ZERO,
            half_extents: Vec3A::from(half_extent_native),
        };
        let world_from_local = Affine3A::from_translation(self.camera_translation + relative);

        self.frustum
            .intersects_obb(&aabb, &world_from_local, false, false)
    }
}

#[derive(Resource, Debug, Default)]
pub struct UsfViewDemandSnapshot {
    pub(super) revision: u64,
    pub(super) entries: Vec<UsfViewDemand>,
    pub(super) scratch: Vec<UsfViewDemand>,
}

impl UsfViewDemandSnapshot {
    pub const fn revision(&self) -> u64 {
        self.revision
    }

    pub fn iter(&self) -> impl ExactSizeIterator<Item = &UsfViewDemand> {
        self.entries.iter()
    }

    pub fn get(&self, source: Entity) -> Option<&UsfViewDemand> {
        self.entries.iter().find(|entry| entry.source == source)
    }
}

impl UsfViewDemand {
    pub(super) fn same_observer_state(&self, other: &Self) -> bool {
        self.source == other.source
            && self.anchor == other.anchor
            && self.finest_scale == other.finest_scale
            && self.projection_eye_offset_metres == other.projection_eye_offset_metres
            && self.camera_translation == other.camera_translation
            && self.camera_rotation == other.camera_rotation
            && self.perspective == other.perspective
            && self.perspective_fov == other.perspective_fov
            && self.perspective_aspect_ratio == other.perspective_aspect_ratio
            && self.pixels_per_radian == other.pixels_per_radian
    }
}
