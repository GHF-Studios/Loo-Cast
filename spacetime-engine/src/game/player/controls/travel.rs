//! Canonical controlled-subject travel policy.
//!
//! Navigation observes semantic structure. This module converts that observation
//! into one canonical movement envelope. Motion kernels only convert metres/s
//! into their current Scale Slice at the final numerical boundary.

use super::*;

const PLANETARY_HANDOFF_RADIUS_FRACTION: f64 = 0.12;
const PLANETARY_HANDOFF_MIN_METRES: f64 = 20_000.0;
const PLANETARY_HANDOFF_MAX_METRES: f64 = 750_000.0;

const MANUAL_FALLBACK_METRES_PER_SECOND: f64 = 100.0;
const MANUAL_MIN_METRES_PER_SECOND: f64 = 8.0;
const MANUAL_MAX_METRES_PER_SECOND: f64 = 2_500.0;
const MANUAL_CHARACTERISTIC_TRAVERSAL_SECONDS: f64 = 90.0;

const CRUISE_FALLBACK_DEFAULT_METRES_PER_SECOND: f64 = 250_000_000.0;
const CRUISE_FALLBACK_MAX_METRES_PER_SECOND: f64 = 10_000_000_000.0;

// Cruise is a game-scale travel regime, not a conventional rocket simulation.
// High canonical deceleration capability lets it remain genuinely fast until
// semantic structure becomes relevant while still converging smoothly into the
// body-relative capture envelope.
const CRUISE_BRAKING_ACCELERATION_METRES_PER_SECOND2: f64 = 60_000.0;
const PLANETARY_CAPTURE_SPEED_MIN_METRES_PER_SECOND: f64 = 250.0;
const PLANETARY_CAPTURE_SPEED_MAX_METRES_PER_SECOND: f64 = 2_500.0;

const MAX_MEDIUM_ENTRY_HORIZON_SECONDS: f64 = 4.0;
const DEFAULT_MEDIUM_ENTRY_HORIZON_SECONDS: f64 = 12.0;
const MAX_MEDIUM_FEATURE_HORIZON_SECONDS: f64 = 1.5;
const DEFAULT_MEDIUM_FEATURE_HORIZON_SECONDS: f64 = 5.0;
const MEDIUM_MIN_RESISTANCE: f64 = 0.01;

const LOOKAHEAD_SECONDS: f64 = 8.0;
const APPROACH_RESOLUTION_DIVISOR: f64 = 4.0;

const PLANETARY_RELEASE_MULTIPLIER: f64 = 1.75;
const LOCAL_FLIGHT_RADIUS_FRACTION: f64 = 0.02;
const LOCAL_FLIGHT_MIN_METRES: f64 = 10_000.0;
const LOCAL_FLIGHT_MAX_METRES: f64 = 75_000.0;
const LOCAL_FLIGHT_RELEASE_MULTIPLIER: f64 = 2.0;

pub(in crate::game::player) fn planetary_handoff_clearance(radius_metres: f64) -> f64 {
    (radius_metres * PLANETARY_HANDOFF_RADIUS_FRACTION)
        .clamp(PLANETARY_HANDOFF_MIN_METRES, PLANETARY_HANDOFF_MAX_METRES)
}

/// Cruise relinquishes authority at the planetary handoff boundary itself.
/// This is a state transition, not an emergency stop.
pub(in crate::game::player) fn critical_dropout_clearance(radius_metres: f64) -> f64 {
    planetary_handoff_clearance(radius_metres)
}

/// Outer hysteresis boundary for leaving body-relative flight.
pub(in crate::game::player) fn planetary_release_clearance(radius_metres: f64) -> f64 {
    planetary_handoff_clearance(radius_metres) * PLANETARY_RELEASE_MULTIPLIER
}

/// Local maneuvering becomes meaningful only near the body's surface/structure.
///
/// This is canonical gameplay policy. It is deliberately unrelated to the
/// current Scale Slice.
pub(in crate::game::player) fn local_flight_capture_clearance(radius_metres: f64) -> f64 {
    (radius_metres * LOCAL_FLIGHT_RADIUS_FRACTION)
        .clamp(LOCAL_FLIGHT_MIN_METRES, LOCAL_FLIGHT_MAX_METRES)
}

/// Outer hysteresis boundary for remaining in Local Flight after capture.
pub(in crate::game::player) fn local_flight_release_clearance(radius_metres: f64) -> f64 {
    local_flight_capture_clearance(radius_metres) * LOCAL_FLIGHT_RELEASE_MULTIPLIER
}

pub(in crate::game::player) fn sync_travel_envelope(
    frame: Res<UsfSpatialFrame>,
    player: Single<
        (
            &Transform,
            &UsfScaleLayer,
            &UsfNavigationContext,
            &UsfTravelNeighborhood,
            &PlayerAdaptiveCruise,
            &mut PlayerTravelEnvelope,
        ),
        With<Player>,
    >,
) {
    let (body, layer, navigation, neighborhood, cruise, mut envelope) = player.into_inner();
    let Ok(position) = frame
        .origin()
        .translated_at_scale(layer.scale(), body.translation)
    else {
        return;
    };

    envelope.manual_speed_metres_per_second = match navigation.kind() {
        UsfNavigationContextKind::Fallback => MANUAL_FALLBACK_METRES_PER_SECOND,
        _ => (navigation.characteristic_length_scale0()
            / MANUAL_CHARACTERISTIC_TRAVERSAL_SECONDS)
            .clamp(MANUAL_MIN_METRES_PER_SECOND, MANUAL_MAX_METRES_PER_SECOND),
    };

    let mut cruise_max = CRUISE_FALLBACK_MAX_METRES_PER_SECOND;
    let mut cruise_default = CRUISE_FALLBACK_DEFAULT_METRES_PER_SECOND;
    let mut medium_cap = None::<f64>;
    let mut nearest_hard_clearance = None::<f64>;

    for influence in neighborhood.influences() {
        let Some(measurement) = influence.measure_from(&position) else {
            continue;
        };

        match influence.kind() {
            UsfTravelInfluenceKind::HardBody => {
                let clearance = measurement.boundary_clearance_scale0();
                nearest_hard_clearance = Some(
                    nearest_hard_clearance
                        .map_or(clearance, |current| current.min(clearance)),
                );

                let handoff =
                    planetary_handoff_clearance(measurement.extent_radius_scale0());
                let capture = planetary_capture_speed(measurement.extent_radius_scale0());

                cruise_max = cruise_max.min(hard_body_speed_limit(
                    clearance,
                    handoff,
                    capture,
                    CRUISE_BRAKING_ACCELERATION_METRES_PER_SECOND2,
                ));
                cruise_default = cruise_default.min(hard_body_speed_limit(
                    clearance,
                    handoff,
                    capture * 0.55,
                    CRUISE_BRAKING_ACCELERATION_METRES_PER_SECOND2 * 0.55,
                ));
            }
            UsfTravelInfluenceKind::Medium(medium) => {
                let resistance = medium.traversal_resistance();
                if resistance < MEDIUM_MIN_RESISTANCE {
                    continue;
                }

                let maximum = medium_speed_limit(
                    measurement.boundary_clearance_scale0(),
                    measurement.characteristic_scale0(),
                    measurement.inside(),
                    resistance,
                    MAX_MEDIUM_ENTRY_HORIZON_SECONDS,
                    MAX_MEDIUM_FEATURE_HORIZON_SECONDS,
                );
                let preferred = medium_speed_limit(
                    measurement.boundary_clearance_scale0(),
                    measurement.characteristic_scale0(),
                    measurement.inside(),
                    resistance,
                    DEFAULT_MEDIUM_ENTRY_HORIZON_SECONDS,
                    DEFAULT_MEDIUM_FEATURE_HORIZON_SECONDS,
                );

                medium_cap = Some(medium_cap.map_or(maximum, |current| current.min(maximum)));
                cruise_max = cruise_max.min(maximum);
                cruise_default = cruise_default.min(preferred);
            }
            UsfTravelInfluenceKind::Region => {}
        }
    }

    cruise_max = cruise_max.max(MANUAL_MIN_METRES_PER_SECOND);
    cruise_default = cruise_default
        .max(MANUAL_MIN_METRES_PER_SECOND)
        .min(cruise_max);

    envelope.cruise_max_speed_metres_per_second = cruise_max;
    envelope.cruise_default_speed_metres_per_second = cruise_default;
    envelope.medium_speed_cap_metres_per_second = medium_cap;

    let current_motion_speed = if cruise.speed_scale0 > 0.0 {
        cruise.speed_scale0.min(cruise_max)
    } else {
        envelope.manual_speed_metres_per_second
    };
    let braking_distance = current_motion_speed * current_motion_speed
        / (2.0 * CRUISE_BRAKING_ACCELERATION_METRES_PER_SECOND2);
    envelope.lookahead_metres =
        (current_motion_speed * LOOKAHEAD_SECONDS + braking_distance).max(1.0);

    envelope.required_resolution_metres = nearest_hard_clearance
        .map(|clearance| (clearance / APPROACH_RESOLUTION_DIVISOR).max(1.0))
        .unwrap_or_else(|| {
            (navigation.characteristic_length_scale0() / APPROACH_RESOLUTION_DIVISOR)
                .max(1.0)
        });
}

fn planetary_capture_speed(radius_metres: f64) -> f64 {
    radius_metres
        .sqrt()
        .clamp(
            PLANETARY_CAPTURE_SPEED_MIN_METRES_PER_SECOND,
            PLANETARY_CAPTURE_SPEED_MAX_METRES_PER_SECOND,
        )
}

fn hard_body_speed_limit(
    clearance_metres: f64,
    handoff_clearance_metres: f64,
    handoff_speed_metres_per_second: f64,
    braking_acceleration_metres_per_second2: f64,
) -> f64 {
    let braking_distance = (clearance_metres - handoff_clearance_metres).max(0.0);
    (handoff_speed_metres_per_second * handoff_speed_metres_per_second
        + 2.0 * braking_acceleration_metres_per_second2.max(0.0) * braking_distance)
        .sqrt()
}

fn medium_speed_limit(
    boundary_clearance_metres: f64,
    characteristic_metres: f64,
    inside: bool,
    resistance: f64,
    entry_horizon_seconds: f64,
    feature_horizon_seconds: f64,
) -> f64 {
    let resistance_factor = 0.25 + resistance.clamp(0.0, 1.0) * 3.75;
    let interior_limit =
        characteristic_metres / (feature_horizon_seconds * resistance_factor);

    if inside {
        interior_limit
    } else {
        interior_limit + boundary_clearance_metres / entry_horizon_seconds
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handoff_is_capture_boundary_not_inner_emergency_stop() {
        let radius = 1_700_000.0;
        assert_eq!(
            critical_dropout_clearance(radius),
            planetary_handoff_clearance(radius)
        );
    }

    #[test]
    fn hard_body_limit_converges_to_capture_speed() {
        let handoff = 100_000.0;
        let capture = 1_500.0;
        let at_handoff = hard_body_speed_limit(handoff, handoff, capture, 120.0);
        let farther = hard_body_speed_limit(1_000_000.0, handoff, capture, 120.0);

        assert!((at_handoff - capture).abs() < 1.0e-6);
        assert!(farther > at_handoff);
    }
}
