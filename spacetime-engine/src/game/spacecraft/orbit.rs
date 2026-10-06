//! Derived orbital telemetry for spacecraft presentation.

use super::*;
use crate::game::orbit::{KeplerianElements, OrbitalStateVector};

#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct SpacecraftOrbit {
    pub valid: bool,
    pub bound: bool,
    pub altitude_metres: f64,
    pub speed_metres_per_second: f64,
    pub vertical_speed_metres_per_second: f64,
    pub horizontal_speed_metres_per_second: f64,
    pub semi_major_axis_metres: f64,
    pub eccentricity: f64,
    pub inclination_radians: f64,
    pub longitude_ascending_node_radians: f64,
    pub argument_periapsis_radians: f64,
    pub true_anomaly_radians: f64,
    pub periapsis_altitude_metres: f64,
    pub apoapsis_altitude_metres: f64,
    pub period_seconds: f64,
    pub time_to_periapsis_seconds: f64,
    pub time_to_apoapsis_seconds: f64,
}

impl Default for SpacecraftOrbit {
    fn default() -> Self {
        Self {
            valid: false,
            bound: false,
            altitude_metres: 0.0,
            speed_metres_per_second: 0.0,
            vertical_speed_metres_per_second: 0.0,
            horizontal_speed_metres_per_second: 0.0,
            semi_major_axis_metres: f64::INFINITY,
            eccentricity: 0.0,
            inclination_radians: 0.0,
            longitude_ascending_node_radians: 0.0,
            argument_periapsis_radians: 0.0,
            true_anomaly_radians: 0.0,
            periapsis_altitude_metres: f64::INFINITY,
            apoapsis_altitude_metres: f64::INFINITY,
            period_seconds: f64::INFINITY,
            time_to_periapsis_seconds: f64::INFINITY,
            time_to_apoapsis_seconds: f64::INFINITY,
        }
    }
}

/// Presentation telemetry only. Missing owners, out-of-range positions and
/// degenerate orbital states invalidate the sample without moving the ship.
pub(super) fn sync_spacecraft_orbit(
    ownership: UsfOwnershipQuery,
    semantic_positions: Query<&UsfPosition>,
    semantic_motions: Query<&UsfCanonicalMotion>,
    gravity_sources: Query<&RadialGravitySource>,
    mut ships: Query<
        (
            Entity,
            &UsfCanonicalMotion,
            &PrimaryBodyContext,
            &mut SpacecraftOrbit,
        ),
        With<SpacecraftManifestation>,
    >,
) {
    for (ship_entity, motion, primary, mut orbit) in &mut ships {
        orbit.valid = false;
        let Some(primary_entity) = primary.entity() else {
            continue;
        };
        let Ok(gravity) = gravity_sources.get(primary_entity).copied() else {
            continue;
        };
        if gravity.surface_gravity_metres_per_second2() <= 0.0 {
            continue;
        }
        let Some(ship_semantic) = ownership.semantic_of(ship_entity) else {
            continue;
        };
        let Ok(position) = semantic_positions.get(ship_semantic).copied() else {
            continue;
        };
        let Ok(primary_position) = semantic_positions.get(primary_entity).copied() else {
            continue;
        };
        let field_scale = gravity.field_scale();
        let Ok(relative_native) =
            position.relative_at_scale_bounded_f64(&primary_position, field_scale, f64::MAX)
        else {
            continue;
        };
        let r = relative_native * field_scale.metres_per_native();
        let radius = r.length();
        if radius <= f64::EPSILON {
            continue;
        }
        let primary_velocity = semantic_motions
            .get(primary_entity)
            .map(|m| m.velocity_metres_per_second())
            .unwrap_or(DVec3::ZERO);
        let v = motion.velocity_metres_per_second() - primary_velocity;
        let mu = gravity.gravitational_parameter_metres3_per_second2();
        let body_radius = gravity.radius_metres();
        let state = OrbitalStateVector::new(r, v);
        let Some(elements) = KeplerianElements::from_state(state, mu, motion.epoch_seconds())
        else {
            continue;
        };
        let radial = r / radius;
        let vertical = v.dot(radial);
        let horizontal = (v.length_squared() - vertical * vertical).max(0.0).sqrt();
        let peri = elements
            .periapsis_radius_metres(mu, state)
            .unwrap_or(f64::INFINITY);
        let apo = elements.apoapsis_radius_metres().unwrap_or(f64::INFINITY);
        *orbit = SpacecraftOrbit {
            valid: true,
            bound: elements.is_bound(),
            altitude_metres: radius - body_radius,
            speed_metres_per_second: v.length(),
            vertical_speed_metres_per_second: vertical,
            horizontal_speed_metres_per_second: horizontal,
            semi_major_axis_metres: elements.semi_major_axis_metres,
            eccentricity: elements.eccentricity,
            inclination_radians: elements.inclination_radians,
            longitude_ascending_node_radians: elements.longitude_ascending_node_radians,
            argument_periapsis_radians: elements.argument_periapsis_radians,
            true_anomaly_radians: true_anomaly(state, elements),
            periapsis_altitude_metres: peri - body_radius,
            apoapsis_altitude_metres: if apo.is_finite() {
                apo - body_radius
            } else {
                f64::INFINITY
            },
            period_seconds: elements.period_seconds(mu).unwrap_or(f64::INFINITY),
            time_to_periapsis_seconds: elements
                .time_to_periapsis_seconds(mu)
                .unwrap_or(f64::INFINITY),
            time_to_apoapsis_seconds: elements
                .time_to_apoapsis_seconds(mu)
                .unwrap_or(f64::INFINITY),
        };
    }
}
fn true_anomaly(state: OrbitalStateVector, elements: KeplerianElements) -> f64 {
    let r = state.position_metres;
    let h = r.cross(state.velocity_metres_per_second).normalize();
    let raw = DVec3::Y.cross(h);
    let node = if raw.length_squared() > 1.0e-20 {
        raw.normalize()
    } else {
        DVec3::X
    };
    let q = h.cross(node).normalize();
    let (sa, ca) = elements.argument_periapsis_radians.sin_cos();
    let p = (node * ca + q * sa).normalize();
    h.dot(p.cross(r.normalize()))
        .atan2(p.dot(r.normalize()))
        .rem_euclid(std::f64::consts::TAU)
}
