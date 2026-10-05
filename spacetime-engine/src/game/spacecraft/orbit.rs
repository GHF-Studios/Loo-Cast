//! Derived orbital telemetry for spacecraft presentation.

use super::*;

#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct SpacecraftOrbit {
    pub valid: bool,
    pub bound: bool,
    pub altitude_metres: f64,
    pub speed_metres_per_second: f64,
    pub semi_major_axis_metres: f64,
    pub eccentricity: f64,
    pub inclination_radians: f64,
    pub longitude_ascending_node_radians: f64,
    pub argument_periapsis_radians: f64,
    pub true_anomaly_radians: f64,
    pub periapsis_altitude_metres: f64,
    pub apoapsis_altitude_metres: f64,
}

impl Default for SpacecraftOrbit {
    fn default() -> Self {
        Self {
            valid: false,
            bound: false,
            altitude_metres: 0.0,
            speed_metres_per_second: 0.0,
            semi_major_axis_metres: f64::INFINITY,
            eccentricity: 0.0,
            inclination_radians: 0.0,
            longitude_ascending_node_radians: 0.0,
            argument_periapsis_radians: 0.0,
            true_anomaly_radians: 0.0,
            periapsis_altitude_metres: f64::INFINITY,
            apoapsis_altitude_metres: f64::INFINITY,
        }
    }
}

pub(super) fn sync_spacecraft_orbit(
    ownership: UsfOwnershipQuery,
    semantic_positions: Query<&UsfPosition>,
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

        let field_scale = gravity.field_scale();
        let bound = gravity.radius_metres() * 16.0;
        let bound_native = field_scale
            .scale0_to_native_f64(bound)
            .min(f64::from(f32::MAX)) as f32;
        let Ok(relative) =
            position.relative_at_scale_bounded(&primary.center(), field_scale, bound_native)
        else {
            continue;
        };

        let field_metres_per_native = field_scale.scale0_units_per_native();
        let r = DVec3::new(
            f64::from(relative.x),
            f64::from(relative.y),
            f64::from(relative.z),
        ) * field_metres_per_native;
        let radius_from_center = r.length();
        if radius_from_center <= f64::EPSILON {
            continue;
        }

        let v = motion.velocity_metres_per_second();
        let body_radius = gravity.radius_metres();
        let mu = f64::from(gravity.surface_gravity_metres_per_second2()) * body_radius.powi(2);
        if !mu.is_finite() || mu <= f64::EPSILON {
            continue;
        }

        let h = r.cross(v);
        let h_len = h.length();
        if h_len <= f64::EPSILON {
            continue;
        }

        let e_vec = v.cross(h) / mu - r / radius_from_center;
        let eccentricity = e_vec.length();
        let specific_energy = 0.5 * v.length_squared() - mu / radius_from_center;
        let semi_major_axis = if specific_energy.abs() > 1.0e-12 {
            -mu / (2.0 * specific_energy)
        } else {
            f64::INFINITY
        };
        let bound = specific_energy < 0.0 && eccentricity < 1.0;

        let reference_normal = DVec3::Y;
        let node = reference_normal.cross(h);
        let node_len = node.length();
        let inclination = (h.dot(reference_normal) / h_len).clamp(-1.0, 1.0).acos();

        let longitude_ascending_node = if node_len > 1.0e-12 {
            node.z.atan2(node.x).rem_euclid(std::f64::consts::TAU)
        } else {
            0.0
        };

        let argument_periapsis = if node_len > 1.0e-12 && eccentricity > 1.0e-12 {
            let mut angle = (node.dot(e_vec) / (node_len * eccentricity))
                .clamp(-1.0, 1.0)
                .acos();
            if e_vec.y < 0.0 {
                angle = std::f64::consts::TAU - angle;
            }
            angle
        } else {
            0.0
        };

        let true_anomaly = if eccentricity > 1.0e-12 {
            let mut angle = (e_vec.dot(r) / (eccentricity * radius_from_center))
                .clamp(-1.0, 1.0)
                .acos();
            if r.dot(v) < 0.0 {
                angle = std::f64::consts::TAU - angle;
            }
            angle
        } else {
            0.0
        };

        let periapsis_radius = if semi_major_axis.is_finite() {
            semi_major_axis * (1.0 - eccentricity)
        } else {
            h_len * h_len / (mu * (1.0 + eccentricity))
        };
        let apoapsis_radius = if bound && semi_major_axis.is_finite() {
            semi_major_axis * (1.0 + eccentricity)
        } else {
            f64::INFINITY
        };

        *orbit = SpacecraftOrbit {
            valid: true,
            bound,
            altitude_metres: radius_from_center - body_radius,
            speed_metres_per_second: v.length(),
            semi_major_axis_metres: semi_major_axis,
            eccentricity,
            inclination_radians: inclination,
            longitude_ascending_node_radians: longitude_ascending_node,
            argument_periapsis_radians: argument_periapsis,
            true_anomaly_radians: true_anomaly,
            periapsis_altitude_metres: periapsis_radius - body_radius,
            apoapsis_altitude_metres: if apoapsis_radius.is_finite() {
                apoapsis_radius - body_radius
            } else {
                f64::INFINITY
            },
        };
    }
}
