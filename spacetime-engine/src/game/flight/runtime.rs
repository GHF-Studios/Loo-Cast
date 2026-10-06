//! Read-only flight telemetry materialization from resolved domain state.

use super::model::*;
use crate::{
    game::{
        control::LocalControlSubject,
        locomotion::{ControlledSubjectLocomotion, DetailedBodyScale},
        navigation::{
            AdaptiveCruise, PrimaryBodyContext, TravelAssistance, TravelAssistanceState,
            TravelState,
        },
        surface::SurfaceContext,
    },
    physics::gravity::GravitySample,
    spatial::{UsfCanonicalMotion, UsfScaleLayer},
};
use bevy::prelude::*;

pub(super) fn sync_flight_telemetry(
    mut subjects: Query<
        (
            &ControlledSubjectLocomotion,
            &DetailedBodyScale,
            &UsfScaleLayer,
            &UsfCanonicalMotion,
            &AdaptiveCruise,
            &TravelAssistanceState,
            &TravelState,
            &GravitySample,
            &PrimaryBodyContext,
            &SurfaceContext,
            Option<&FlightContactState>,
            Option<&FlightLandingOpportunity>,
            Option<&FlightSafetyState>,
            &mut FlightTelemetry,
        ),
        With<LocalControlSubject>,
    >,
) {
    for (
        locomotion,
        detailed,
        layer,
        motion,
        cruise,
        assistance,
        travel,
        gravity,
        primary,
        surface,
        contact,
        landing,
        safety,
        mut telemetry,
    ) in &mut subjects
    {
        let mode = FlightMode::from_locomotion(locomotion.regime());
        let contact = contact.copied().unwrap_or_default();
        let landing = landing.copied().unwrap_or_default();
        let safety = safety.copied().unwrap_or_default();

        telemetry.active = mode.is_some() || contact.is_landed();
        telemetry.mode = mode;
        telemetry.assistance = assistance.mode();
        telemetry.contact = contact;
        telemetry.landing_available = landing.available();
        telemetry.safety = safety.level();
        telemetry.speed_metres_per_second = motion.speed_metres_per_second();
        telemetry.throttle = if assistance.mode() == TravelAssistance::Cruise {
            cruise.throttle
        } else {
            0.0
        };
        telemetry.thrusters_enabled = locomotion.thrusters_enabled();
        telemetry.rcs_enabled = locomotion.rcs_enabled();
        telemetry.interaction_scale = layer.scale();
        telemetry.detailed_interaction = layer.scale() == detailed.0;
        telemetry.primary_body = surface.body().or(primary.entity());
        telemetry.surface_clearance_metres = surface.clearance_metres();
        telemetry.surface_collision_ready = surface.collision_ready();
        telemetry.local_gravity_metres_per_second2 = gravity.magnitude_metres_per_second2();
        telemetry.planetary_handoff_clearance_metres = travel.planetary_handoff_clearance_scale0;
        telemetry.planetary_handoff_available = travel.planetary_handoff_available;
        telemetry.dropout_required =
            travel.critical_dropout || safety.level() == FlightSafetyLevel::Emergency;
        telemetry.time_to_contact_seconds = safety.time_to_contact_seconds();
        telemetry.closing_speed_metres_per_second = safety.closing_speed_metres_per_second();
        telemetry.required_deceleration_metres_per_second2 =
            safety.required_deceleration_metres_per_second2();
    }
}
