//! Body-local camera demand for the reconstructible clipmap frontier.

use bevy::math::DVec3;

use crate::spatial::{UsfSemanticFrame, UsfViewDemand};
use crate::voxel::CelestialVoxelField;

use super::topology::CelestialClipmapBlockKey;

const CLIPMAP_FRUSTUM_PREFETCH_MARGIN_RADIANS: f64 =
    0.261_799_387_799_149_4;
const CLIPMAP_FRUSTUM_REPLAN_RADIANS: f64 =
    0.130_899_693_899_574_7;
const CLIPMAP_HORIZON_MARGIN_RADIANS: f64 =
    0.008_726_646_259_971_648;
const CLIPMAP_OCCLUDER_SAFETY_METRES: f64 = 128.0;

#[derive(Debug, Clone, Copy)]
struct ClipmapBodyFrustum {
    side_normals_local: [DVec3; 4],
    horizontal_half_angle: f64,
    vertical_half_angle: f64,
}

impl ClipmapBodyFrustum {
    fn new(
        forward_local: DVec3,
        right_local: DVec3,
        up_local: DVec3,
        horizontal_half_angle: f64,
        vertical_half_angle: f64,
    ) -> Option<Self> {
        let forward = forward_local.try_normalize()?;
        let right = right_local.try_normalize()?;
        let up = up_local.try_normalize()?;
        let horizontal = (
            horizontal_half_angle + CLIPMAP_FRUSTUM_PREFETCH_MARGIN_RADIANS
        )
            .min(std::f64::consts::PI - 1.0e-4);
        let vertical = (
            vertical_half_angle + CLIPMAP_FRUSTUM_PREFETCH_MARGIN_RADIANS
        )
            .min(std::f64::consts::PI - 1.0e-4);

        let (sh, ch) = horizontal.sin_cos();
        let (sv, cv) = vertical.sin_cos();
        let side_normals_local = [
            (forward * sh + right * ch).normalize(),
            (forward * sh - right * ch).normalize(),
            (forward * sv + up * cv).normalize(),
            (forward * sv - up * cv).normalize(),
        ];

        debug_assert!(forward.is_finite());
        debug_assert!(side_normals_local.iter().all(|n| n.is_finite()));

        Some(Self {
            side_normals_local,
            horizontal_half_angle: horizontal,
            vertical_half_angle: vertical,
        })
    }

    fn intersects_sphere(&self, relative_center: DVec3, radius: f64) -> bool {
        if !relative_center.is_finite()
            || !radius.is_finite()
            || radius < 0.0
        {
            return true;
        }

        self.side_normals_local
            .iter()
            .all(|normal| normal.dot(relative_center) >= -radius)
    }

    fn requires_refresh(&self, next: &Self) -> bool {
        // Forward alone misses roll, which can expose a new corner of a
        // rectangular frustum while its look direction stays unchanged.
        let basis_changed = self
            .side_normals_local
            .iter()
            .zip(next.side_normals_local.iter())
            .any(|(current, next)| {
                current.dot(*next).clamp(-1.0, 1.0)
                    < CLIPMAP_FRUSTUM_REPLAN_RADIANS.cos()
            });
        let shape_changed =
            (self.horizontal_half_angle - next.horizontal_half_angle).abs()
                > 0.02
            || (self.vertical_half_angle - next.vertical_half_angle).abs()
                > 0.02;

        basis_changed || shape_changed
    }

    fn result_still_covers(&self, next: &Self) -> bool {
        let basis_ok = self
            .side_normals_local
            .iter()
            .zip(next.side_normals_local.iter())
            .all(|(current, next)| {
                current.dot(*next).clamp(-1.0, 1.0)
                    >= CLIPMAP_FRUSTUM_REPLAN_RADIANS.cos()
            });

        basis_ok
            && next.horizontal_half_angle
                <= self.horizontal_half_angle + 0.02
            && next.vertical_half_angle
                <= self.vertical_half_angle + 0.02
    }
}

#[derive(Debug, Clone, Copy)]
pub(super) struct ClipmapVisibilityDemand {
    actual_eye_local: DVec3,
    prefetch_eye_local: DVec3,
    frustum: Option<ClipmapBodyFrustum>,
    solid_occluder_radius_metres: f64,
}

impl ClipmapVisibilityDemand {
    pub(super) fn new(
        field: CelestialVoxelField,
        actual_observer_local: DVec3,
        predicted_observer_local: DVec3,
        body_frame: &UsfSemanticFrame,
        view: &UsfViewDemand,
    ) -> Self {
        let local_from_world = body_frame.orientation().conjugate();
        let eye_offset = local_from_world * view.projection_eye_offset_metres();
        let actual_eye_local = actual_observer_local + eye_offset;
        let prefetch_eye_local = predicted_observer_local + eye_offset;

        let frustum = view
            .perspective_basis_and_half_angles()
            .and_then(|(forward, right, up, horizontal, vertical)| {
                ClipmapBodyFrustum::new(
                    local_from_world * forward,
                    local_from_world * right,
                    local_from_world * up,
                    horizontal,
                    vertical,
                )
            });

        let (surface_minimum, _) =
            field.conservative_surface_radius_bounds_metres();
        let solid_occluder_radius_metres = (
            surface_minimum
                - field.volumetric_surface_inward_support_metres()
                - CLIPMAP_OCCLUDER_SAFETY_METRES
        )
            .max(0.0);

        debug_assert!(actual_eye_local.is_finite());
        debug_assert!(prefetch_eye_local.is_finite());
        debug_assert!(solid_occluder_radius_metres.is_finite());

        Self {
            actual_eye_local,
            prefetch_eye_local,
            frustum,
            solid_occluder_radius_metres,
        }
    }

    pub(super) fn demands_block(&self, key: CelestialClipmapBlockKey) -> bool {
        self.demands_block_from_eye(key, self.actual_eye_local)
            || self.demands_block_from_eye(key, self.prefetch_eye_local)
    }

    fn demands_block_from_eye(&self, key: CelestialClipmapBlockKey, eye_local: DVec3) -> bool {
        let center = key.center_local_metres();
        let radius = key.half_extent_metres().length();
        let relative = center - eye_local;

        if let Some(frustum) = &self.frustum
            && !frustum.intersects_sphere(relative, radius)
        {
            return false;
        }

        !self.fully_occluded_by_planet(eye_local, center, radius)
    }

    fn fully_occluded_by_planet(
        &self,
        eye_local: DVec3,
        target_center: DVec3,
        target_radius: f64,
    ) -> bool {
        let occluder = self.solid_occluder_radius_metres;
        let observer_radius = eye_local.length();
        if occluder <= 0.0
            || observer_radius <= occluder + target_radius
        {
            return false;
        }

        let relative = target_center - eye_local;
        let target_distance = relative.length();
        if target_distance <= target_radius.max(f64::EPSILON) {
            return false;
        }

        let body_direction = -eye_local / observer_radius;
        let target_direction = relative / target_distance;
        let occluder_angle =
            (occluder / observer_radius).clamp(0.0, 1.0).asin();
        let target_angle =
            (target_radius / target_distance).clamp(0.0, 1.0).asin();
        let containment = occluder_angle
            - target_angle
            - CLIPMAP_HORIZON_MARGIN_RADIANS;
        if containment <= 0.0 {
            return false;
        }

        let angularly_hidden =
            body_direction.dot(target_direction) >= containment.cos();
        let tangent_distance = (
            observer_radius * observer_radius - occluder * occluder
        )
            .max(0.0)
            .sqrt();
        let behind_tangent =
            target_distance - target_radius > tangent_distance;

        angularly_hidden && behind_tangent
    }

    pub(super) fn requires_refresh(&self, next: &Self, fine_extent: f64) -> bool {
        let eye_moved = (self.actual_eye_local - next.actual_eye_local).length()
            > fine_extent.max(1.0)
            || (self.prefetch_eye_local - next.prefetch_eye_local).length()
                > fine_extent.max(1.0);
        let frustum_changed = match (&self.frustum, &next.frustum) {
            (Some(current), Some(next)) => current.requires_refresh(next),
            (None, None) => false,
            _ => true,
        };

        eye_moved || frustum_changed
    }

    pub(super) fn result_still_relevant_to(&self, next: &Self) -> bool {
        match (&self.frustum, &next.frustum) {
            (Some(current), Some(next)) => current.result_still_covers(next),
            (None, None) => true,
            _ => false,
        }
    }
}

