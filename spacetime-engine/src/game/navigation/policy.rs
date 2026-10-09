//! Canonical controlled-subject travel policy.
//!
//! Navigation observes semantic structure. Subject-owned [`TravelProfile`]
//! data converts those observations into one canonical SI movement envelope.
//! Motion kernels convert metres/s into Scale-Slice native units only at the
//! final numerical boundary.

use bevy::prelude::*;

use crate::{
    ecs::UsfOwnershipQuery,
    game::control::LocalControlSubject,
    spatial::{
        UsfNavigationContext, UsfNavigationContextKind, UsfPosition, UsfScaleLayer,
        UsfTravelInfluenceKind, UsfTravelNeighborhood,
    },
};

use super::{AdaptiveCruise, TravelEnvelope, TravelProfile};

pub(super) fn sync_travel_envelope(
    ownership: UsfOwnershipQuery,
    semantic_positions: Query<&UsfPosition>,
    subject: Single<
        (
            Entity,
            &UsfScaleLayer,
            &UsfNavigationContext,
            &UsfTravelNeighborhood,
            &TravelProfile,
            &AdaptiveCruise,
            &mut TravelEnvelope,
        ),
        With<LocalControlSubject>,
    >,
) {
    let (entity, layer, navigation, neighborhood, profile, cruise, mut envelope) =
        subject.into_inner();
    let Some(semantic) = ownership.semantic_of(entity) else {
        return;
    };
    let Ok(position) = semantic_positions.get(semantic) else {
        return;
    };

    envelope.manual_speed_metres_per_second = match navigation.kind() {
        UsfNavigationContextKind::Fallback => profile.manual.fallback_metres_per_second,
        _ => (navigation.characteristic_length_metres()
            / profile
                .manual
                .characteristic_traversal_seconds
                .max(f64::EPSILON))
        .clamp(
            profile.manual.minimum_metres_per_second,
            profile.manual.maximum_metres_per_second,
        ),
    };

    let mut cruise_max = profile.cruise.maximum_metres_per_second;
    let mut cruise_default = profile.cruise.default_metres_per_second;
    let mut medium_cap = None::<f64>;
    let mut nearest_hard_clearance = None::<f64>;

    for (_, _, influence, measurement) in neighborhood.measurements_from(position, layer.scale()) {
        match influence.kind() {
            UsfTravelInfluenceKind::HardBody => {
                let clearance = measurement.boundary_clearance_metres();
                nearest_hard_clearance = Some(
                    nearest_hard_clearance.map_or(clearance, |current| current.min(clearance)),
                );

                let handoff =
                    profile.planetary_handoff_clearance(measurement.extent_radius_metres());
                let capture = profile.planetary_capture_speed(measurement.extent_radius_metres());

                cruise_max = cruise_max.min(hard_body_speed_limit(
                    clearance,
                    handoff,
                    capture,
                    profile.cruise.braking_acceleration_metres_per_second2,
                ));
                cruise_default = cruise_default.min(hard_body_speed_limit(
                    clearance,
                    handoff,
                    capture * 0.55,
                    profile.cruise.braking_acceleration_metres_per_second2 * 0.55,
                ));
            }
            UsfTravelInfluenceKind::Medium(medium) => {
                let resistance = medium.traversal_resistance();
                if resistance < profile.cruise.medium_minimum_resistance {
                    continue;
                }

                let maximum = medium_speed_limit(
                    measurement.boundary_clearance_metres(),
                    measurement.characteristic_length_metres(),
                    measurement.inside(),
                    resistance,
                    profile.cruise.maximum_medium_entry_horizon_seconds,
                    profile.cruise.maximum_medium_feature_horizon_seconds,
                );
                let preferred = medium_speed_limit(
                    measurement.boundary_clearance_metres(),
                    measurement.characteristic_length_metres(),
                    measurement.inside(),
                    resistance,
                    profile.cruise.default_medium_entry_horizon_seconds,
                    profile.cruise.default_medium_feature_horizon_seconds,
                );

                medium_cap = Some(medium_cap.map_or(maximum, |current| current.min(maximum)));
                cruise_max = cruise_max.min(maximum);
                cruise_default = cruise_default.min(preferred);
            }
            UsfTravelInfluenceKind::Region => {}
        }
    }

    cruise_max = cruise_max.max(profile.manual.minimum_metres_per_second);
    cruise_default = cruise_default
        .max(profile.manual.minimum_metres_per_second)
        .min(cruise_max);

    envelope.cruise_max_speed_metres_per_second = cruise_max;
    envelope.cruise_default_speed_metres_per_second = cruise_default;
    envelope.medium_speed_cap_metres_per_second = medium_cap;

    let current_motion_speed = if cruise.speed_metres_per_second > 0.0 {
        cruise.speed_metres_per_second.min(cruise_max)
    } else {
        envelope.manual_speed_metres_per_second
    };
    let braking_acceleration = profile
        .cruise
        .braking_acceleration_metres_per_second2
        .max(f64::EPSILON);
    let braking_distance =
        current_motion_speed * current_motion_speed / (2.0 * braking_acceleration);
    envelope.lookahead_metres =
        (current_motion_speed * profile.cruise.lookahead_seconds + braking_distance).max(1.0);

    let divisor = profile.approach.resolution_divisor.max(f64::EPSILON);
    envelope.required_resolution_metres = nearest_hard_clearance
        .map(|clearance| (clearance / divisor).max(1.0))
        .unwrap_or_else(|| (navigation.characteristic_length_metres() / divisor).max(1.0));
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
        characteristic_metres / (feature_horizon_seconds.max(f64::EPSILON) * resistance_factor);

    if inside {
        interior_limit
    } else {
        interior_limit + boundary_clearance_metres / entry_horizon_seconds.max(f64::EPSILON)
    }
}
