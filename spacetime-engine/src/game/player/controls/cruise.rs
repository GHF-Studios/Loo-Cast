//! Adaptive long-distance free-flight.
//!
//! Cruise speed is canonical (S0 units/s), while runtime displacement is
//! projected into whichever USF chart currently owns interaction.

use bevy::math::DVec3;

use super::*;

const THROTTLE_RATE_PER_SECOND: f32 = 0.45;
const SPEED_RESPONSE: f64 = 1.4;

// Temporary global pacing knob while we tune the qualitative Cruise model.
// Keep all environment-relative speed relationships intact, but make the
// resulting effective movement human-observable during development.
const CRUISE_PACING_FACTOR: f64 = 1.0e-4;

// Hard boundaries are object-relative. The maximum envelope is deliberately
// more aggressive than the engagement default.
const MAX_HARD_APPROACH_HORIZON_SECONDS: f64 = 4.0;
const DEFAULT_HARD_APPROACH_HORIZON_SECONDS: f64 = 12.0;
const MAX_HARD_RADIUS_SPEED_FLOOR: f64 = 0.005;
const DEFAULT_HARD_RADIUS_SPEED_FLOOR: f64 = 0.001;

// Traversable media are feature-relative instead. Their *outer radius is not a
// wall*. Cruise only needs to slow enough to resolve meaningful structure as it
// approaches/enters the volume, and can remain extremely fast inside a huge,
// smooth, sparse nebula.
const MAX_MEDIUM_ENTRY_HORIZON_SECONDS: f64 = 4.0;
const DEFAULT_MEDIUM_ENTRY_HORIZON_SECONDS: f64 = 12.0;
const MAX_MEDIUM_FEATURE_HORIZON_SECONDS: f64 = 1.5;
const DEFAULT_MEDIUM_FEATURE_HORIZON_SECONDS: f64 = 5.0;
const MEDIUM_MIN_RESISTANCE: f64 = 0.01;

// If no speed-constraining semantic influence is available yet, remain useful
// without reverting to an effectively unbounded 1e30 S0/s target.
const FALLBACK_MAX_NATIVE_PER_SECOND: f64 = 2.0;
const FALLBACK_DEFAULT_NATIVE_PER_SECOND: f64 = 0.25;
const FALLBACK_MIN_MAX_SPEED_SCALE0: f64 = 30_000.0;
const FALLBACK_MIN_DEFAULT_SPEED_SCALE0: f64 = 10_000.0;

// Canonical speed may lead the *actual runtime chart* by only this many
// decades. This guards numeric/runtime displacement without coupling movement
// speed to presentation zoom or forcing a chart transition.
const CHART_SPEED_HEADROOM_DECADES: i32 = 2;

#[derive(Debug, Clone, Copy)]
struct CruiseSpeedEnvelope {
    max_speed_scale0: f64,
    default_speed_scale0: f64,
    nearest_hard_clearance_scale0: Option<f64>,
    medium_speed_cap_scale0: Option<f64>,
}

impl CruiseSpeedEnvelope {
    fn scaled(self, factor: f64) -> Self {
        debug_assert!(factor.is_finite() && factor >= 0.0);
        Self {
            max_speed_scale0: self.max_speed_scale0 * factor,
            default_speed_scale0: self.default_speed_scale0 * factor,
            nearest_hard_clearance_scale0: self.nearest_hard_clearance_scale0,
            medium_speed_cap_scale0: self.medium_speed_cap_scale0.map(|speed| speed * factor),
        }
    }
}

pub(in crate::game::player) fn adaptive_cruise_movement(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    capture: Res<CursorCapture>,
    presentation: Res<PrimaryViewPresentation>,
    frames: Res<UsfScaleLayerFrames>,
    influences: Query<(Entity, &UsfTravelInfluence)>,
    mut was_active: Local<bool>,
    player: Single<
        (
            &mut Transform,
            &UsfScaleLayer,
            &CharacterControlFrame,
            Option<&PlayerDead>,
            &PlayerAim,
            &mut PlayerAdaptiveCruise,
            &mut UsfTravelNeighborhood,
            Option<&mut LinearVelocity>,
        ),
        With<Player>,
    >,
) {
    let (
        mut body,
        layer,
        control,
        dead,
        aim,
        mut cruise,
        mut neighborhood,
        velocity,
    ) = player.into_inner();

    if !cruise.active {
        *was_active = false;
        return;
    }
    if dead.is_some() || presentation.is_embedded() || gameplay_suppressed(&keyboard, &capture) {
        if let Some(mut velocity) = velocity {
            velocity.0 = Vec3::ZERO;
        }
        return;
    }

    let dt = time.delta_secs().max(0.0);
    if dt <= 0.0 {
        return;
    }

    let just_engaged = !*was_active;
    *was_active = true;

    let player_scale = layer.scale();
    let player_absolute = frames.absolute(player_scale, body.translation);
    neighborhood.advance(dt);
    if just_engaged || neighborhood.needs_refresh(player_absolute, player_scale) {
        neighborhood.refresh(
            player_absolute,
            player_scale,
            &frames,
            influences.iter().map(|(entity, influence)| (entity, *influence)),
        );
    }

    let envelope =
        cruise_speed_envelope(player_absolute, player_scale, &frames, &neighborhood)
            .scaled(CRUISE_PACING_FACTOR);
    cruise.speed_cap_scale0 = envelope.max_speed_scale0;
    cruise.default_speed_scale0 = envelope.default_speed_scale0;
    cruise.nearest_hard_clearance_scale0 = envelope.nearest_hard_clearance_scale0;
    cruise.medium_speed_cap_scale0 = envelope.medium_speed_cap_scale0;

    if just_engaged {
        cruise.throttle = throttle_for_speed(
            envelope.default_speed_scale0,
            envelope.max_speed_scale0,
        );
    }

    let throttle_delta =
        keyboard.pressed(KeyCode::KeyW) as i8 - keyboard.pressed(KeyCode::KeyS) as i8;
    cruise.throttle =
        (cruise.throttle + throttle_delta as f32 * THROTTLE_RATE_PER_SECOND * dt)
            .clamp(0.0, 1.0);

    let requested_environmental =
        envelope.max_speed_scale0 * f64::from(cruise.throttle.powf(2.0));
    let chart_safe_cap = chart_safe_speed_cap(player_scale);
    let requested = requested_environmental.min(chart_safe_cap);
    cruise.speed_scale0 = smooth_log_value(cruise.speed_scale0, requested, dt, SPEED_RESPONSE);

    let native_denominator = 10.0_f64.powi(player_scale.exponent() as i32);
    let native_speed = (cruise.speed_scale0 / native_denominator)
        .clamp(0.0, f32::MAX as f64) as f32;
    let direction = (control.rotation() * aim.local_rotation() * Vec3::NEG_Z).normalize_or_zero();
    body.translation += direction * native_speed * dt;

    if let Some(mut velocity) = velocity {
        velocity.0 = Vec3::ZERO;
    }

    // Deliberately do not mutate the observer's UsfViewContext here. Travel speed and
    // presentation/interaction scale are separate concerns. Automatic scale
    // following can return later once chart transitions are independently solid.
}

fn cruise_speed_envelope(
    player_absolute: DVec3,
    player_scale: SpatialScale,
    frames: &UsfScaleLayerFrames,
    neighborhood: &UsfTravelNeighborhood,
) -> CruiseSpeedEnvelope {
    let mut max_speed = f64::INFINITY;
    let mut default_speed = f64::INFINITY;
    let mut nearest_hard_clearance = None::<f64>;
    let mut medium_speed_cap = None::<f64>;
    let mut constrained = false;

    for influence in neighborhood.influences() {
        let Some(measurement) = influence.measure_from(player_absolute, player_scale, frames) else {
            continue;
        };

        match influence.kind() {
            UsfTravelInfluenceKind::HardBody => {
                constrained = true;
                let clearance = measurement.boundary_clearance_scale0();
                let radius = measurement.extent_radius_scale0();
                nearest_hard_clearance = Some(
                    nearest_hard_clearance.map_or(clearance, |current| current.min(clearance)),
                );
                max_speed = max_speed.min(hard_body_speed_limit(
                    clearance,
                    radius,
                    MAX_HARD_APPROACH_HORIZON_SECONDS,
                    MAX_HARD_RADIUS_SPEED_FLOOR,
                ));
                default_speed = default_speed.min(hard_body_speed_limit(
                    clearance,
                    radius,
                    DEFAULT_HARD_APPROACH_HORIZON_SECONDS,
                    DEFAULT_HARD_RADIUS_SPEED_FLOOR,
                ));
            }
            UsfTravelInfluenceKind::Medium(medium) => {
                let resistance = medium.traversal_resistance();
                if resistance < MEDIUM_MIN_RESISTANCE {
                    continue;
                }
                constrained = true;
                let maximum = medium_speed_limit(
                    measurement.boundary_clearance_scale0(),
                    measurement.characteristic_scale0(),
                    measurement.inside(),
                    resistance,
                    MAX_MEDIUM_ENTRY_HORIZON_SECONDS,
                    MAX_MEDIUM_FEATURE_HORIZON_SECONDS,
                );
                let default = medium_speed_limit(
                    measurement.boundary_clearance_scale0(),
                    measurement.characteristic_scale0(),
                    measurement.inside(),
                    resistance,
                    DEFAULT_MEDIUM_ENTRY_HORIZON_SECONDS,
                    DEFAULT_MEDIUM_FEATURE_HORIZON_SECONDS,
                );
                medium_speed_cap = Some(
                    medium_speed_cap.map_or(maximum, |current| current.min(maximum)),
                );
                max_speed = max_speed.min(maximum);
                default_speed = default_speed.min(default);
            }
            UsfTravelInfluenceKind::Region => {
                // Regions are discovery/query context, not obstacles. Future
                // neighbourhood refresh can use them to request finer child
                // structure from worldgen/spatial indices.
            }
        }
    }

    if !constrained {
        return fallback_speed_envelope(player_scale);
    }

    CruiseSpeedEnvelope {
        max_speed_scale0: max_speed,
        default_speed_scale0: default_speed.min(max_speed),
        nearest_hard_clearance_scale0: nearest_hard_clearance,
        medium_speed_cap_scale0: medium_speed_cap,
    }
}

fn hard_body_speed_limit(
    clearance_scale0: f64,
    radius_scale0: f64,
    approach_horizon_seconds: f64,
    radius_speed_floor: f64,
) -> f64 {
    (clearance_scale0 / approach_horizon_seconds).max(radius_scale0 * radius_speed_floor)
}

/// Speed limit for a traversable volume.
///
/// Inside the medium, the limit depends on its local feature size and traversal
/// resistance. Outside it, boundary clearance adds progressively more headroom
/// so Cruise begins decelerating before entry rather than treating the boundary
/// as a collision surface.
fn medium_speed_limit(
    boundary_clearance_scale0: f64,
    characteristic_scale0: f64,
    inside: bool,
    resistance: f64,
    entry_horizon_seconds: f64,
    feature_horizon_seconds: f64,
) -> f64 {
    let resistance_factor = 0.25 + resistance.clamp(0.0, 1.0) * 3.75;
    let interior_limit = characteristic_scale0 / (feature_horizon_seconds * resistance_factor);

    if inside {
        interior_limit
    } else {
        interior_limit + boundary_clearance_scale0 / entry_horizon_seconds
    }
}

fn fallback_speed_envelope(scale: SpatialScale) -> CruiseSpeedEnvelope {
    let scale0_per_native = 10.0_f64.powi(scale.exponent() as i32);
    let max_speed =
        (scale0_per_native * FALLBACK_MAX_NATIVE_PER_SECOND).max(FALLBACK_MIN_MAX_SPEED_SCALE0);
    let default_speed = (scale0_per_native * FALLBACK_DEFAULT_NATIVE_PER_SECOND)
        .max(FALLBACK_MIN_DEFAULT_SPEED_SCALE0)
        .min(max_speed);

    CruiseSpeedEnvelope {
        max_speed_scale0: max_speed,
        default_speed_scale0: default_speed,
        nearest_hard_clearance_scale0: None,
        medium_speed_cap_scale0: None,
    }
}

fn throttle_for_speed(speed_scale0: f64, max_speed_scale0: f64) -> f32 {
    if !speed_scale0.is_finite() || !max_speed_scale0.is_finite() || max_speed_scale0 <= 0.0 {
        return 0.0;
    }
    (speed_scale0 / max_speed_scale0).clamp(0.0, 1.0).sqrt() as f32
}

fn chart_safe_speed_cap(scale: SpatialScale) -> f64 {
    10.0_f64.powi(scale.exponent() as i32 + CHART_SPEED_HEADROOM_DECADES)
}

fn smooth_log_value(current: f64, target: f64, dt: f32, response: f64) -> f64 {
    let current_log = (1.0 + current.max(0.0)).log10();
    let target_log = (1.0 + target.max(0.0)).log10();
    let alpha = 1.0 - (-response * f64::from(dt)).exp();
    10.0_f64.powf(current_log + (target_log - current_log) * alpha) - 1.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hard_body_envelope_scales_with_object_size() {
        let earth_radius = 6_371_000.0;
        let moon_radius = 1_737_000.0;

        let earth = hard_body_speed_limit(
            0.0,
            earth_radius,
            MAX_HARD_APPROACH_HORIZON_SECONDS,
            MAX_HARD_RADIUS_SPEED_FLOOR,
        );
        let moon = hard_body_speed_limit(
            0.0,
            moon_radius,
            MAX_HARD_APPROACH_HORIZON_SECONDS,
            MAX_HARD_RADIUS_SPEED_FLOOR,
        );

        assert!(earth > moon);
        assert!((earth / moon - earth_radius / moon_radius).abs() < 1.0e-9);
    }

    #[test]
    fn medium_boundary_is_not_a_wall() {
        let outside_limit = medium_speed_limit(20.0, 10.0, false, 0.5, 4.0, 1.5);
        let inside_limit = medium_speed_limit(0.0, 10.0, true, 0.5, 4.0, 1.5);

        assert!(outside_limit > inside_limit);
        assert!(inside_limit > 0.0);
    }

    #[test]
    fn medium_speed_is_independent_of_enclosing_extent() {
        // The medium speed function deliberately receives boundary proximity and
        // local feature size, not the enclosing volume radius. A nebula can be
        // enormous without that fact alone making traversal slow.
        let a = medium_speed_limit(0.0, 5.0, true, 0.5, 4.0, 1.5);
        let b = medium_speed_limit(0.0, 5.0, true, 0.5, 4.0, 1.5);
        assert_eq!(a, b);
    }

    #[test]
    fn engagement_throttle_requests_default_speed() {
        let maximum = 80_000.0;
        let default = 20_000.0;
        let throttle = throttle_for_speed(default, maximum);
        let requested = maximum * f64::from(throttle.powf(2.0));

        assert!((requested - default).abs() < 1.0);
    }

    #[test]
    fn fallback_is_finite_and_scale_sensitive() {
        let local = fallback_speed_envelope(SpatialScale::ZERO);
        let system = fallback_speed_envelope(SpatialScale::new(8).unwrap());

        assert!(local.max_speed_scale0.is_finite());
        assert!(local.default_speed_scale0 <= local.max_speed_scale0);
        assert!(system.max_speed_scale0 > local.max_speed_scale0);
        assert!(system.default_speed_scale0 <= system.max_speed_scale0);
    }
}
