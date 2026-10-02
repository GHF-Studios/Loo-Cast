//! Canonical safe-spawn resolution for procedural body surfaces.
//!
//! Authored fixture directions are hints, not guaranteed spawn coordinates.
//! This resolver searches a bounded deterministic neighborhood on the canonical
//! celestial field and chooses a locally gentle/continuous surface candidate.
//!
//! It intentionally depends on semantic terrain only. Mesh/collider residency
//! is handled separately by the existing coverage-gated interaction transition.

use bevy::{math::DVec3, prelude::*};

use crate::{
    physics::PhysicalBoxHull,
    spatial::{UsfPosition, UsfSemanticFrame},
    voxel::CelestialVoxelField,
};

use super::BodySurfaceSite;

const SEARCH_RING_METRES: [f64; 8] = [
    0.0,
    2_000.0,
    5_000.0,
    10_000.0,
    20_000.0,
    40_000.0,
    80_000.0,
    160_000.0,
];
const RING_SAMPLES: usize = 12;
const IDEAL_MAX_SLOPE_DEGREES: f64 = 18.0;
const FALLBACK_MAX_SLOPE_DEGREES: f64 = 32.0;
const MIN_PROBE_RADIUS_METRES: f64 = 75.0;
const MAX_PROBE_RADIUS_METRES: f64 = 500.0;

#[derive(Debug, Clone, Copy)]
struct SpawnCandidate {
    direction_local: Vec3,
    surface_local_metres: DVec3,
    normal_local: Vec3,
    distance_from_hint_metres: f64,
    slope_degrees: f64,
    roughness_metres: f64,
    score: f64,
}

fn tangent_basis(direction: Vec3) -> Option<(Vec3, Vec3)> {
    let direction = direction.normalize_or_zero();
    if direction == Vec3::ZERO {
        return None;
    }
    let reference = if direction.y.abs() < 0.9 { Vec3::Y } else { Vec3::X };
    let tangent_u = direction.cross(reference).normalize_or_zero();
    if tangent_u == Vec3::ZERO {
        return None;
    }
    let tangent_v = direction.cross(tangent_u).normalize_or_zero();
    (tangent_v != Vec3::ZERO).then_some((tangent_u, tangent_v))
}

fn angular_offset(
    direction: Vec3,
    tangent: Vec3,
    arc_metres: f64,
    radius_metres: f64,
) -> Vec3 {
    let angle = (arc_metres / radius_metres.max(1.0))
        .clamp(0.0, std::f64::consts::FRAC_PI_2) as f32;
    (direction * angle.cos() + tangent * angle.sin()).normalize_or_zero()
}

fn sample_candidate(
    field: CelestialVoxelField,
    direction_local: Vec3,
    distance_from_hint_metres: f64,
    probe_radius_metres: f64,
) -> Option<SpawnCandidate> {
    let direction_local = direction_local.normalize_or_zero();
    let (tangent_u, tangent_v) = tangent_basis(direction_local)?;
    let center = field.surface_local_metres(direction_local).ok()?;

    let sample_direction = |tangent: Vec3, signed_distance: f64| {
        angular_offset(
            direction_local,
            tangent,
            signed_distance,
            field.radius_metres(),
        )
    };
    let sample_point = |direction: Vec3| field.surface_local_metres(direction).ok();

    let u_plus = sample_point(sample_direction(tangent_u, probe_radius_metres))?;
    let u_minus = sample_point(sample_direction(tangent_u, -probe_radius_metres))?;
    let v_plus = sample_point(sample_direction(tangent_v, probe_radius_metres))?;
    let v_minus = sample_point(sample_direction(tangent_v, -probe_radius_metres))?;

    let du = u_plus - u_minus;
    let dv = v_plus - v_minus;
    let mut normal = du.cross(dv).normalize_or_zero();
    if normal == DVec3::ZERO {
        return None;
    }

    let radial = DVec3::new(
        f64::from(direction_local.x),
        f64::from(direction_local.y),
        f64::from(direction_local.z),
    );
    if normal.dot(radial) < 0.0 {
        normal = -normal;
    }

    let normal_local = Vec3::new(normal.x as f32, normal.y as f32, normal.z as f32)
        .normalize_or_zero();
    if normal_local == Vec3::ZERO {
        return None;
    }

    let slope_cos = f64::from(normal_local.dot(direction_local)).clamp(-1.0, 1.0);
    let slope_degrees = slope_cos.acos().to_degrees();

    let roughness_metres = [u_plus, u_minus, v_plus, v_minus]
        .into_iter()
        .map(|point| (point - center).dot(normal).abs())
        .fold(0.0_f64, f64::max);

    let distance_term = distance_from_hint_metres / 40_000.0;
    let slope_term = (slope_degrees / IDEAL_MAX_SLOPE_DEGREES).powi(2) * 4.0;
    let roughness_term = roughness_metres / probe_radius_metres.max(1.0) * 12.0;
    let score = distance_term + slope_term + roughness_term;

    Some(SpawnCandidate {
        direction_local,
        surface_local_metres: center,
        normal_local,
        distance_from_hint_metres,
        slope_degrees,
        roughness_metres,
        score,
    })
}

fn preferred_ring_direction(
    preferred: Vec3,
    tangent_u: Vec3,
    tangent_v: Vec3,
    distance_metres: f64,
    phase: usize,
    radius_metres: f64,
) -> Vec3 {
    if distance_metres <= f64::EPSILON {
        return preferred;
    }
    let theta = std::f32::consts::TAU * phase as f32 / RING_SAMPLES as f32;
    let ring_tangent =
        (tangent_u * theta.cos() + tangent_v * theta.sin()).normalize_or_zero();
    angular_offset(preferred, ring_tangent, distance_metres, radius_metres)
}

/// Resolve one authored body-surface hint to a locally safe canonical spawn.
///
/// Candidate safety is judged from the actual local terrain normal.
///
/// The returned [`BodySurfaceSite::up`] is deliberately the body-radial
/// direction through the selected canonical surface point. Spawn clearance is
/// therefore applied along one stable radial ray: a requested 25 m air gap
/// remains ~25 m above the same semantic surface sample even on steep terrain.
pub(super) fn resolve_good_spawn(
    hint: BodySurfaceSite,
    body_origin: UsfPosition,
    body_frame: UsfSemanticFrame,
    field: CelestialVoxelField,
    hull: PhysicalBoxHull,
) -> Option<BodySurfaceSite> {
    let preferred_world = hint.up().normalize_or_zero();
    let preferred_local = body_frame
        .world_direction_to_local(preferred_world)
        .normalize_or_zero();
    let (basis_u, basis_v) = tangent_basis(preferred_local)?;

    let probe_radius_metres = (f64::from(hull.bounding_radius_metres()) * 8.0)
        .clamp(MIN_PROBE_RADIUS_METRES, MAX_PROBE_RADIUS_METRES);

    let roughness_limit_metres =
        (probe_radius_metres * 0.08).max(f64::from(hull.bounding_radius_metres()) * 0.5);

    let mut best_ideal = None::<SpawnCandidate>;
    let mut best_fallback = None::<SpawnCandidate>;
    let mut best_any = None::<SpawnCandidate>;

    for distance_metres in SEARCH_RING_METRES {
        let sample_count = if distance_metres <= f64::EPSILON {
            1
        } else {
            RING_SAMPLES
        };

        for phase in 0..sample_count {
            let direction = preferred_ring_direction(
                preferred_local,
                basis_u,
                basis_v,
                distance_metres,
                phase,
                field.radius_metres(),
            );
            let Some(candidate) = sample_candidate(
                field,
                direction,
                distance_metres,
                probe_radius_metres,
            ) else {
                continue;
            };

            if best_any.is_none_or(|current| candidate.score < current.score) {
                best_any = Some(candidate);
            }

            if candidate.slope_degrees <= FALLBACK_MAX_SLOPE_DEGREES
                && candidate.roughness_metres <= roughness_limit_metres * 2.0
                && best_fallback.is_none_or(|current| candidate.score < current.score)
            {
                best_fallback = Some(candidate);
            }

            if candidate.slope_degrees <= IDEAL_MAX_SLOPE_DEGREES
                && candidate.roughness_metres <= roughness_limit_metres
                && best_ideal.is_none_or(|current| candidate.score < current.score)
            {
                best_ideal = Some(candidate);
            }
        }
    }

    let selected = best_ideal.or(best_fallback).or(best_any)?;

    let surface = body_frame
        .local_metres_to_world(body_origin, selected.surface_local_metres)
        .ok()?;
    let up_world = body_frame
        .local_direction_to_world(selected.direction_local)
        .normalize_or_zero();
    let resolved = BodySurfaceSite::new(hint.body(), surface, up_world, hint.scale())?;

    info!(
        body = ?hint.body(),
        distance_from_hint_metres = selected.distance_from_hint_metres,
        slope_degrees = selected.slope_degrees,
        roughness_metres = selected.roughness_metres,
        probe_radius_metres,
        local_direction = ?selected.direction_local,
        "resolved canonical good spawn"
    );

    Some(resolved)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        spatial::SpatialScale,
        voxel::CelestialBodyProfile,
    };

    #[test]
    fn rocky_spawn_resolution_is_deterministic() {
        let field = CelestialVoxelField::new(
            6_371_000.0,
            SpatialScale::new(6).unwrap(),
            SpatialScale::ZERO,
            0x4541_5254,
            CelestialBodyProfile::Rocky,
        );
        let origin = UsfPosition::zero(SpatialScale::MIN);
        let frame = UsfSemanticFrame::identity();
        let surface = field.surface_position(&origin, frame, Vec3::Y).unwrap();
        let hint = BodySurfaceSite::new(
            Entity::PLACEHOLDER,
            surface,
            Vec3::Y,
            SpatialScale::ZERO,
        )
        .unwrap();
        let hull =
            PhysicalBoxHull::from_size_metres(Vec3::new(4.0, 2.0, 8.0));

        let a = resolve_good_spawn(hint, origin, frame, field, hull).unwrap();
        let b = resolve_good_spawn(hint, origin, frame, field, hull).unwrap();
        assert_eq!(a.surface(), b.surface());
        assert_eq!(a.up(), b.up());

        let radial_native = a
            .surface()
            .relative_at_scale_bounded_f64(
                &origin,
                SpatialScale::ZERO,
                f64::MAX,
            )
            .unwrap();
        let radial = Vec3::new(
            radial_native.x as f32,
            radial_native.y as f32,
            radial_native.z as f32,
        )
        .normalize_or_zero();
        assert!(
            a.up().dot(radial) > 0.9999,
            "spawn clearance must follow the body-radial ray"
        );
    }
}
