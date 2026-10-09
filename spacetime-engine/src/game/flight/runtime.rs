//! Resolve flight control requests and publish derived flight telemetry.

use super::model::*;
use crate::{
    ecs::UsfOwnershipQuery,
    game::{
        control::LocalControlSubject,
        locomotion::{
            ControlledSubjectLocomotion, DetailedBodyScale, FlightActuation, FlightAttitudeCommand,
            FlightControlIntent, MotionExecution, MotionKernel,
        },
        navigation::{
            PrimaryBodyContext, TravelAssistance, TravelAssistanceState, TravelProfile, TravelState,
        },
        surface::SurfaceContext,
    },
    physics::gravity::GravitySample,
    spatial::{UsfCanonicalMotion, UsfPosition, UsfScaleLayer},
};
use bevy::{math::DVec3, prelude::*};

/// Publish approach risk before navigation resolves Lattice Cruise requests.
/// The semantic position is used for the far-field radial calculation so a
/// coarse f32 chart cannot collapse a nearby planetary direction to zero.
pub(super) fn evaluate_flight_safety(
    ownership: UsfOwnershipQuery,
    positions: Query<&UsfPosition>,
    mut subjects: Query<
        (
            Entity,
            &UsfCanonicalMotion,
            &PrimaryBodyContext,
            &SurfaceContext,
            &FlightSafetyProfile,
            &TravelProfile,
            &TravelAssistanceState,
            &FlightActuation,
            &mut FlightSafetyState,
        ),
        With<LocalControlSubject>,
    >,
) {
    for (
        entity,
        motion,
        primary,
        surface,
        profile,
        travel_profile,
        assistance,
        actuation,
        mut safety,
    ) in &mut subjects
    {
        let observed_surface = surface
            .clearance_metres()
            .filter(|clearance| clearance.is_finite())
            .map(|clearance| {
                let radial = surface.radial_outward();
                (
                    clearance,
                    DVec3::new(
                        f64::from(radial.x),
                        f64::from(radial.y),
                        f64::from(radial.z),
                    ),
                    surface.collision_ready(),
                    profile.preferred_contact_speed_metres_per_second(),
                )
            });
        let observed_body = ownership
            .semantic_of(entity)
            .and_then(|semantic| positions.get(semantic).ok())
            .filter(|_| primary.is_resolved())
            .and_then(|position| {
                let relative = position
                    .relative_at_scale_bounded_f64(
                        &primary.center(),
                        primary.reference_scale(),
                        f64::MAX,
                    )
                    .ok()?;
                let radial = relative.normalize_or_zero();
                let cruising = assistance.mode() == TravelAssistance::Cruise;
                let clearance = if cruising {
                    primary.clearance_metres()
                        - travel_profile.planetary_handoff_clearance(primary.radius_metres())
                } else {
                    primary.clearance_metres()
                };
                let target_speed = if cruising {
                    travel_profile.planetary_capture_speed(primary.radius_metres())
                } else {
                    profile.preferred_contact_speed_metres_per_second()
                };
                (radial != DVec3::ZERO).then_some((clearance, radial, false, target_speed))
            });
        let approach = match (observed_surface, observed_body) {
            (Some(surface), Some(body)) if body.0 < surface.0 => Some(body),
            (Some(surface), _) => Some(surface),
            (None, body) => body,
        };
        *safety = if let Some((clearance, radial, ready, target_speed)) = approach {
            let available_braking = if assistance.mode() == TravelAssistance::Cruise {
                travel_profile
                    .cruise
                    .braking_acceleration_metres_per_second2
            } else {
                let main = if actuation.thrusters_enabled() {
                    f64::from(travel_profile.flight.local_acceleration_metres_per_second2)
                } else {
                    0.0
                };
                let rcs = if actuation.rcs_enabled() {
                    f64::from(
                        travel_profile
                            .flight
                            .rcs_braking_acceleration_metres_per_second2,
                    )
                } else {
                    0.0
                };
                main.max(rcs)
            };
            FlightSafetyState::evaluate(
                clearance,
                radial,
                motion.velocity_metres_per_second(),
                ready,
                available_braking,
                target_speed,
                *profile,
            )
        } else {
            FlightSafetyState::default()
        };
    }
}

pub(super) fn resolve_flight_control_requests(
    mut requests: MessageReader<FlightControlRequest>,
    mut subjects: Query<(
        Entity,
        &Transform,
        &FlightCapabilities,
        &mut FlightActuation,
        &mut PilotAttitudeLaw,
        &mut AttitudeAutopilot,
    )>,
) {
    for request in requests.read() {
        let Ok((_, body, capabilities, mut actuation, mut law, mut autopilot)) =
            subjects.get_mut(request.entity())
        else {
            continue;
        };

        match request.command() {
            FlightControlCommand::SetPilotAttitudeLaw(requested) => *law = requested,
            FlightControlCommand::SetMainPropulsion(enabled) => {
                if capabilities.main_propulsion() {
                    actuation.set_thrusters_enabled(enabled);
                }
            }
            FlightControlCommand::SetReactionControl(enabled) => {
                if capabilities.reaction_control() {
                    actuation.set_rcs_enabled(enabled);
                }
            }
            FlightControlCommand::SetAngularAssist(enabled) => {
                if capabilities.reaction_control() {
                    actuation.set_angular_assist_enabled(enabled);
                }
            }
            FlightControlCommand::SetAutopilot(command) => match command {
                AttitudeAutopilotCommand::Off => autopilot.disengage(),
                AttitudeAutopilotCommand::HoldCurrent => autopilot.hold(body.rotation),
                AttitudeAutopilotCommand::Prograde => autopilot.point_prograde(),
            },
        }
    }
}

pub(super) fn apply_attitude_autopilot(
    mut subject: Single<
        (
            &Transform,
            &UsfCanonicalMotion,
            &MotionExecution,
            &AttitudeAutopilot,
            &mut FlightControlIntent,
        ),
        With<LocalControlSubject>,
    >,
) {
    let (body, motion, execution, autopilot, intent) = &mut *subject;
    if execution.kernel() != MotionKernel::InertialFlight {
        return;
    }
    let target = match autopilot.mode() {
        AttitudeAutopilotMode::Off => return,
        AttitudeAutopilotMode::Hold => autopilot.hold_target(),
        AttitudeAutopilotMode::Prograde => {
            let velocity = motion.velocity_metres_per_second();
            if velocity.length_squared() <= 1.0e-12 {
                body.rotation
            } else {
                let forward = body.rotation * Vec3::NEG_Z;
                let direction = velocity.normalize();
                let desired = Vec3::new(direction.x as f32, direction.y as f32, direction.z as f32);
                (Quat::from_rotation_arc(forward, desired) * body.rotation).normalize()
            }
        }
    };
    let axes = intent.translation_axes();
    let pace = intent.pace_multiplier();
    let boost = intent.boost();
    intent.set(
        axes,
        FlightAttitudeCommand::TargetOrientation(target),
        pace,
        boost,
    );
}

pub(super) fn sync_flight_telemetry(
    body: Single<&Transform, With<LocalControlSubject>>,
    mut subjects: Query<
        (
            &ControlledSubjectLocomotion,
            &FlightActuation,
            &DetailedBodyScale,
            &UsfScaleLayer,
            &UsfCanonicalMotion,
            &crate::game::locomotion::FlightThrottle,
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
        actuation,
        detailed,
        layer,
        motion,
        throttle,
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
        let mode = FlightMode::from_context(
            locomotion.regime(),
            telemetry.mode,
            primary.is_resolved().then(|| primary.clearance_metres()),
            primary.radius_metres(),
        );
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
        let forward = body.rotation * Vec3::NEG_Z;
        let forward = DVec3::new(
            f64::from(forward.x),
            f64::from(forward.y),
            f64::from(forward.z),
        );
        let velocity = motion.velocity_metres_per_second();
        telemetry.forward_speed_metres_per_second = velocity.dot(forward);
        telemetry.lateral_speed_metres_per_second =
            (velocity - forward * telemetry.forward_speed_metres_per_second).length();
        telemetry.throttle = throttle.value();
        telemetry.lattice_cooldown_seconds = assistance.cooldown_remaining_seconds();
        telemetry.lattice_charge_seconds = assistance.spool_remaining_seconds();
        telemetry.thrusters_enabled = actuation.thrusters_enabled();
        telemetry.rcs_enabled = actuation.rcs_enabled();
        telemetry.angular_assist_enabled = actuation.angular_assist_enabled();
        telemetry.interaction_scale = layer.scale();
        telemetry.detailed_interaction = layer.scale() == detailed.0;
        telemetry.primary_body = surface.body().or(primary.entity());
        telemetry.surface_clearance_metres = surface.clearance_metres();
        telemetry.surface_collision_ready = surface.collision_ready();
        telemetry.local_gravity_metres_per_second2 = gravity.magnitude_metres_per_second2();
        telemetry.planetary_handoff_clearance_metres = travel.planetary_handoff_clearance_metres;
        telemetry.planetary_handoff_available = travel.planetary_handoff_available;
        telemetry.dropout_required =
            travel.critical_dropout || safety.level() == FlightSafetyLevel::Emergency;
        telemetry.time_to_contact_seconds = safety.time_to_contact_seconds();
        telemetry.closing_speed_metres_per_second = safety.closing_speed_metres_per_second();
        telemetry.required_deceleration_metres_per_second2 =
            safety.required_deceleration_metres_per_second2();
    }
}
