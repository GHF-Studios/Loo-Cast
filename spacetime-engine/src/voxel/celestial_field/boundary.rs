//! Volumetric zero-crossing projection and travel-boundary queries.
//!
//! Projection samples the canonical field; the authored radial surface is a
//! fallback only when the iterative volumetric solve does not converge.

use bevy::{math::DVec3, prelude::*};

use crate::spatial::{SpatialScale, UsfPosition, UsfSemanticFrame};

use super::CelestialVoxelField;

const MAX_ITERATIONS: usize = 16;
const TARGET_RESIDUAL_METRES: f64 = 0.05;
const ACCEPTABLE_RESIDUAL_METRES: f64 = 2.0;
const MIN_GRADIENT_EPSILON_METRES: f64 = 0.25;
const MAX_GRADIENT_EPSILON_METRES: f64 = 16.0;
const MAX_PROJECTION_STEP_METRES: f64 = 250_000.0;

impl CelestialVoxelField {
    fn normalized_field_gradient_local_metres(
        self,
        point: DVec3,
        epsilon_metres: f64,
    ) -> Option<DVec3> {
        if !point.is_finite() || !epsilon_metres.is_finite() || epsilon_metres <= 0.0 {
            return None;
        }

        let e = epsilon_metres;
        let dx = self.signed_distance_local_metres(point + DVec3::X * e)?
            - self.signed_distance_local_metres(point - DVec3::X * e)?;
        let dy = self.signed_distance_local_metres(point + DVec3::Y * e)?
            - self.signed_distance_local_metres(point - DVec3::Y * e)?;
        let dz = self.signed_distance_local_metres(point + DVec3::Z * e)?
            - self.signed_distance_local_metres(point - DVec3::Z * e)?;

        let gradient = DVec3::new(dx, dy, dz) / (2.0 * e);
        let length = gradient.length();
        if !length.is_finite() || length <= 1.0e-9 {
            return None;
        }
        Some(gradient / length)
    }

    /// Projects a body-local point onto the nearest reachable zero crossing of
    /// the canonical volumetric SDF.
    ///
    /// Unlike `surface_position` / `surface_near`, this follows cave walls and
    /// other non-radial volumetric boundaries because it samples the complete
    /// semantic field.
    ///
    /// The returned signed distance is the INITIAL clearance at `point`, not
    /// the tiny final projection residual.
    pub(crate) fn nearest_boundary_local_metres(
        self,
        point: DVec3,
        max_abs_distance_metres: f64,
    ) -> Option<(DVec3, f64)> {
        if !point.is_finite()
            || !max_abs_distance_metres.is_finite()
            || max_abs_distance_metres < 0.0
        {
            return None;
        }

        let initial_distance = self.signed_distance_local_metres(point)?;
        if !initial_distance.is_finite() || initial_distance.abs() > max_abs_distance_metres {
            return None;
        }

        let mut projected = point;
        let mut residual = initial_distance;

        for _ in 0..MAX_ITERATIONS {
            if residual.abs() <= TARGET_RESIDUAL_METRES {
                return Some((projected, initial_distance));
            }

            let epsilon = (residual.abs() * 0.05)
                .clamp(MIN_GRADIENT_EPSILON_METRES, MAX_GRADIENT_EPSILON_METRES);

            let normal = self
                .normalized_field_gradient_local_metres(projected, epsilon)
                .or_else(|| {
                    let radius = projected.length();
                    (radius.is_finite() && radius > f64::EPSILON).then_some(projected / radius)
                })?;

            let step = residual.clamp(-MAX_PROJECTION_STEP_METRES, MAX_PROJECTION_STEP_METRES);
            projected -= normal * step;
            residual = self.signed_distance_local_metres(projected)?;
        }

        if residual.abs() <= ACCEPTABLE_RESIDUAL_METRES {
            return Some((projected, initial_distance));
        }

        // Robust fallback for pathological/noisy gradients far from caves:
        // recover the authored outer surface rather than dropping all terrain
        // presentation/demand.
        let radius = point.length();
        if !radius.is_finite() || radius <= f64::EPSILON {
            return None;
        }
        let direction = Vec3::new(
            (point.x / radius) as f32,
            (point.y / radius) as f32,
            (point.z / radius) as f32,
        )
        .normalize_or_zero();
        if direction == Vec3::ZERO {
            return None;
        }
        let outer = self.surface_local_metres(direction).ok()?;
        Some((outer, initial_distance))
    }

    /// Canonical/world-space adapter around nearest volumetric boundary
    /// projection.
    pub(crate) fn boundary_near(
        self,
        body_origin: &UsfPosition,
        body_frame: UsfSemanticFrame,
        point: &UsfPosition,
        max_abs_distance_metres: f64,
    ) -> Option<(UsfPosition, f64)> {
        let local = body_frame
            .world_to_local_metres(body_origin, point, SpatialScale::ZERO, f64::MAX)
            .ok()?;
        let (boundary_local, signed_clearance_metres) =
            self.nearest_boundary_local_metres(local, max_abs_distance_metres)?;
        let boundary = body_frame
            .local_metres_to_world(*body_origin, boundary_local)
            .ok()?;
        Some((boundary, signed_clearance_metres))
    }
}
