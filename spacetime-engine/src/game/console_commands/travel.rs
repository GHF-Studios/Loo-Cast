//! Observer-scale, pace, and cruise console adapters.

use super::super::{
    control::LocalControlSubject,
    flight::{
        AttitudeAutopilot, AttitudeAutopilotCommand, FlightCapabilities, FlightControlCommand,
        FlightControlRequest, FlightSafetyLevel, FlightSafetyState, PilotAttitudeLaw,
    },
    locomotion::{
        ControlledSubjectLocomotion, DeveloperMotionOverride, FlightControlIntent,
        LocomotionCapabilities, MotionExecution,
    },
    navigation::{
        NavigationCapabilities, PrimaryBodyContext, TravelAssistance, TravelAssistanceRequest,
        TravelAssistanceState, TravelPace, TravelState,
    },
};
use super::primary_view_context;
use crate::game::{
    control::LocalViewTarget,
    player::{CameraMode, Player, PlayerCamera, ViewCameraProfile},
};
use crate::{
    console::{ConsoleCommandInvocation, ConsoleCommandResult},
    physics::gravity::GravitySample,
    spatial::{
        UsfCanonicalMotion, UsfPrimaryInteractionSlice, UsfScaleLayer, UsfViewContext,
        UsfViewRenderAnchor,
    },
};
use bevy::prelude::*;

pub(super) fn zoom_command(
    world: &mut World,
    invocation: &ConsoleCommandInvocation,
) -> ConsoleCommandResult {
    let Some(value) = invocation.args().first() else {
        let Some(view) = primary_view_context(world) else {
            return ConsoleCommandResult::error("primary USF view context is unavailable");
        };
        let interaction = *world.resource::<UsfPrimaryInteractionSlice>();
        return ConsoleCommandResult::success(format!(
            "observer scale = {:+.3} | interaction S{}{}",
            view.continuous_exponent(),
            interaction.scale(),
            if interaction.handoff_pending() {
                " (handoff pending)"
            } else {
                ""
            },
        ));
    };

    let raw = value
        .trim()
        .trim_start_matches('S')
        .trim_start_matches('s')
        .trim_start_matches('+');
    let Ok(exponent) = raw.parse::<f32>() else {
        return ConsoleCommandResult::error(format!("invalid observer scale `{value}`"));
    };
    if !exponent.is_finite() {
        return ConsoleCommandResult::error("observer scale must be finite");
    }

    {
        let mut query = world.query_filtered::<&mut UsfViewContext, With<UsfViewRenderAnchor>>();
        let Some(mut view) = query.iter_mut(world).next() else {
            return ConsoleCommandResult::error("primary USF view context is unavailable");
        };
        view.set_continuous_exponent(exponent);
    }
    let Some(view) = primary_view_context(world) else {
        return ConsoleCommandResult::error("primary USF view context is unavailable");
    };
    let interaction = *world.resource::<UsfPrimaryInteractionSlice>();
    ConsoleCommandResult::success(format!(
        "observer scale requested -> {:+.3} | interaction remains S{}{}",
        view.continuous_exponent(),
        interaction.scale(),
        if interaction.handoff_pending() {
            " (handoff pending)"
        } else {
            ""
        },
    ))
}

pub(super) fn speed_command(
    world: &mut World,
    invocation: &ConsoleCommandInvocation,
) -> ConsoleCommandResult {
    let requested = invocation.args().first().map(String::as_str);

    let mut query = world.query_filtered::<&mut TravelPace, With<Player>>();
    let Some(mut speed) = query.iter_mut(world).next() else {
        return ConsoleCommandResult::error("player travel-speed state is unavailable");
    };

    if invocation.args().len() > 1 {
        return ConsoleCommandResult::error("usage: speed [<multiplier>|reset]");
    }

    if let Some(raw) = requested {
        if raw.eq_ignore_ascii_case("reset") {
            *speed = TravelPace::default();
        } else {
            let Ok(parsed) = raw.parse::<f32>() else {
                return ConsoleCommandResult::error(format!("invalid speed multiplier `{raw}`"));
            };
            if !parsed.is_finite() || parsed < 0.0 {
                return ConsoleCommandResult::error(
                    "speed multiplier must be finite and non-negative",
                );
            }
            speed.multiplier = parsed;
        }
    }

    ConsoleCommandResult::success_and_return_to_gameplay(format!(
        "characteristic movement pace = {:.3}x",
        speed.multiplier,
    ))
}

pub(super) fn camera_command(
    world: &mut World,
    invocation: &ConsoleCommandInvocation,
) -> ConsoleCommandResult {
    if invocation.args().len() > 1 {
        return ConsoleCommandResult::error("usage: camera [cockpit|chase|orbit]");
    }
    let available = world
        .query_filtered::<&ViewCameraProfile, With<LocalViewTarget>>()
        .iter(world)
        .next()
        .map(|profile| profile.supports_mode(CameraMode::Orbit))
        .unwrap_or(false);
    let mut cameras = world.query::<(&mut PlayerCamera, &Transform)>();
    let Some((mut camera, transform)) = cameras.iter_mut(world).next() else {
        return ConsoleCommandResult::error("primary camera unavailable");
    };
    if let Some(raw) = invocation.args().first() {
        let requested = match raw.to_ascii_lowercase().as_str() {
            "cockpit" | "first" | "firstperson" => CameraMode::FirstPerson,
            "chase" | "third" | "thirdperson" => CameraMode::ThirdPerson,
            "orbit" => CameraMode::Orbit,
            _ => return ConsoleCommandResult::error("usage: camera [cockpit|chase|orbit]"),
        };
        if requested == CameraMode::Orbit && !available {
            return ConsoleCommandResult::error("orbit view is unavailable for this subject");
        }
        if requested == CameraMode::Orbit && camera.mode != CameraMode::Orbit {
            camera.orbit_rotation = transform.rotation;
        }
        camera.mode = requested;
    }
    ConsoleCommandResult::success_and_return_to_gameplay(format!(
        "camera = {}",
        match camera.mode {
            CameraMode::FirstPerson => "first person / cockpit",
            CameraMode::ThirdPerson => "third person / chase",
            CameraMode::Orbit => "orbit",
        }
    ))
}

pub(super) fn cruise_command(
    world: &mut World,
    invocation: &ConsoleCommandInvocation,
) -> ConsoleCommandResult {
    if invocation.args().len() > 1 {
        return ConsoleCommandResult::error("usage: cruise [on|off]");
    }

    let mut query = world.query_filtered::<(
        Entity,
        &NavigationCapabilities,
        &TravelState,
        &TravelAssistanceState,
        Option<&FlightSafetyState>,
    ), With<LocalControlSubject>>();
    let Some((entity, capabilities, travel, assistance, safety)) = query.iter(world).next() else {
        return ConsoleCommandResult::error("player locomotion state is unavailable");
    };

    let active = match invocation.args().first().map(String::as_str) {
        None => assistance.mode() != TravelAssistance::Cruise && !assistance.is_spooling(),
        Some(value) if value.eq_ignore_ascii_case("on") => true,
        Some(value) if value.eq_ignore_ascii_case("off") => false,
        Some(value) => {
            return ConsoleCommandResult::error(format!(
                "invalid Cruise state `{value}`; expected on or off"
            ));
        }
    };

    if active && !capabilities.cruise() {
        return ConsoleCommandResult::error("This subject has no Lattice Drive");
    }
    if active && assistance.is_spooling() {
        return ConsoleCommandResult::success("Lattice Drive is charging");
    }
    if active && !assistance.drive_ready() {
        return ConsoleCommandResult::error(format!(
            "Lattice Drive cooling ({:.1} s remaining)",
            assistance.cooldown_remaining_seconds()
        ));
    }
    if active
        && (!travel.cruise_entry_available
            || safety.is_some_and(|state| state.level() == FlightSafetyLevel::Emergency))
    {
        return ConsoleCommandResult::error("Lattice Cruise entry blocked by approach safety");
    }
    let requested = if active {
        TravelAssistance::Cruise
    } else {
        TravelAssistance::Manual
    };
    if world
        .write_message(TravelAssistanceRequest::set(entity, requested))
        .is_none()
    {
        return ConsoleCommandResult::error("travel assistance request channel is unavailable");
    }

    ConsoleCommandResult::success_and_return_to_gameplay(if active {
        "Lattice Drive charge requested — W/S adjusts throttle, mouse steers"
    } else {
        "Lattice Drive cancel or disengage requested"
    })
}

pub(super) fn attitude_command(
    world: &mut World,
    invocation: &ConsoleCommandInvocation,
) -> ConsoleCommandResult {
    if invocation.args().len() > 1 {
        return ConsoleCommandResult::error("usage: attitude [hold|manual]");
    }
    let mut query =
        world.query_filtered::<(Entity, &PilotAttitudeLaw), With<LocalControlSubject>>();
    let Some((entity, law)) = query.iter(world).next() else {
        return ConsoleCommandResult::error("controlled attitude law is unavailable");
    };
    let requested = if let Some(value) = invocation.args().first() {
        match value.to_ascii_lowercase().as_str() {
            "hold" => PilotAttitudeLaw::Hold,
            "manual" | "rate" | "view" | "follow" => PilotAttitudeLaw::ManualRate,
            _ => return ConsoleCommandResult::error("usage: attitude [hold|manual]"),
        }
    } else {
        *law
    };
    if requested != *law
        && world
            .write_message(FlightControlRequest::new(
                entity,
                FlightControlCommand::SetPilotAttitudeLaw(requested),
            ))
            .is_none()
    {
        return ConsoleCommandResult::error("flight-control request channel is unavailable");
    }
    ConsoleCommandResult::success_and_return_to_gameplay(format!(
        "pilot attitude law requested = {}",
        requested.label(),
    ))
}

pub(super) fn motion_override_command(
    world: &mut World,
    invocation: &ConsoleCommandInvocation,
) -> ConsoleCommandResult {
    let mut query =
        world.query_filtered::<(Entity, &LocomotionCapabilities), With<LocalControlSubject>>();
    let Some((entity, capabilities)) = query.iter(world).next() else {
        return ConsoleCommandResult::error("controlled subject is unavailable");
    };
    if !capabilities.inertial_flight() {
        return ConsoleCommandResult::error("developer motion override requires a flight subject");
    }
    let mut override_ = world
        .get::<DeveloperMotionOverride>(entity)
        .copied()
        .unwrap_or_default();
    let args = invocation.args();
    if args.is_empty() {
        return ConsoleCommandResult::success(format!(
            "developer motion override: collision={} gravity={}",
            if override_.ignore_collision() {
                "ignored"
            } else {
                "normal"
            },
            if override_.ignore_gravity() {
                "ignored"
            } else {
                "normal"
            },
        ));
    }
    if args.len() != 2 {
        return ConsoleCommandResult::error("usage: motionoverride <collision|gravity> <on|off>");
    }
    let enabled = match args[1].to_ascii_lowercase().as_str() {
        "on" => true,
        "off" => false,
        _ => return ConsoleCommandResult::error("expected on or off"),
    };
    match args[0].to_ascii_lowercase().as_str() {
        "collision" => override_.set_ignore_collision(enabled),
        "gravity" => override_.set_ignore_gravity(enabled),
        _ => return ConsoleCommandResult::error("expected collision or gravity"),
    }
    if override_.is_clear() {
        world.entity_mut(entity).remove::<DeveloperMotionOverride>();
    } else {
        world.entity_mut(entity).insert(override_);
    }
    ConsoleCommandResult::success_and_return_to_gameplay("developer motion override updated")
}

pub(super) fn debug_fly_command(
    world: &mut World,
    invocation: &ConsoleCommandInvocation,
) -> ConsoleCommandResult {
    if invocation.args().len() > 1 {
        return ConsoleCommandResult::error("usage: debugfly [on|off]");
    }
    let mut query =
        world.query_filtered::<(Entity, &LocomotionCapabilities), With<LocalControlSubject>>();
    let Some((entity, capabilities)) = query.iter(world).next() else {
        return ConsoleCommandResult::error("controlled subject unavailable");
    };
    if !capabilities.inertial_flight() {
        return ConsoleCommandResult::error("debug traversal requires a flight subject");
    }
    let mut override_ = world
        .get::<DeveloperMotionOverride>(entity)
        .copied()
        .unwrap_or_default();
    let requested = match invocation.args().first().map(String::as_str) {
        None => !override_.characteristic_traversal(),
        Some(raw) if raw.eq_ignore_ascii_case("on") => true,
        Some(raw) if raw.eq_ignore_ascii_case("off") => false,
        _ => return ConsoleCommandResult::error("usage: debugfly [on|off]"),
    };
    override_.set_characteristic_traversal(requested);
    if override_.is_clear() {
        world.entity_mut(entity).remove::<DeveloperMotionOverride>();
    } else {
        world.entity_mut(entity).insert(override_);
    }
    if requested {
        world.write_message(TravelAssistanceRequest::set(
            entity,
            TravelAssistance::Manual,
        ));
    }
    ConsoleCommandResult::success_and_return_to_gameplay(if requested {
        "debug traversal enabled; wheel changes characteristic pace"
    } else {
        "debug traversal disabled"
    })
}

pub(super) fn autopilot_command(
    world: &mut World,
    invocation: &ConsoleCommandInvocation,
) -> ConsoleCommandResult {
    if invocation.args().len() > 1 {
        return ConsoleCommandResult::error("usage: autopilot [off|hold|prograde]");
    }
    let mut query = world.query_filtered::<(
        Entity,
        &LocomotionCapabilities,
        &AttitudeAutopilot,
    ), With<LocalControlSubject>>();
    let Some((entity, capabilities, autopilot)) = query.iter(world).next() else {
        return ConsoleCommandResult::error("controlled autopilot state is unavailable");
    };
    if !capabilities.inertial_flight() {
        return ConsoleCommandResult::error("autopilot requires a flight subject");
    }
    let Some(raw) = invocation.args().first() else {
        return ConsoleCommandResult::success_and_return_to_gameplay(format!(
            "attitude autopilot = {:?}",
            autopilot.mode()
        ));
    };
    let requested = match raw.to_ascii_lowercase().as_str() {
        "off" => AttitudeAutopilotCommand::Off,
        "hold" => AttitudeAutopilotCommand::HoldCurrent,
        "prograde" => AttitudeAutopilotCommand::Prograde,
        _ => return ConsoleCommandResult::error("usage: autopilot [off|hold|prograde]"),
    };
    if world
        .write_message(FlightControlRequest::new(
            entity,
            FlightControlCommand::SetAutopilot(requested),
        ))
        .is_none()
    {
        return ConsoleCommandResult::error("flight-control request channel is unavailable");
    }
    ConsoleCommandResult::success_and_return_to_gameplay(format!(
        "attitude autopilot requested = {:?}",
        requested
    ))
}

pub(super) fn motion_stack_command(
    world: &mut World,
    invocation: &ConsoleCommandInvocation,
) -> ConsoleCommandResult {
    if !invocation.args().is_empty() {
        return ConsoleCommandResult::error("usage: motionstack");
    }
    let mut query = world.query_filtered::<(
        Entity,
        &LocomotionCapabilities,
        &FlightCapabilities,
        &NavigationCapabilities,
        &FlightControlIntent,
        &PilotAttitudeLaw,
        &AttitudeAutopilot,
        &TravelAssistanceState,
        &ControlledSubjectLocomotion,
        &MotionExecution,
        &UsfCanonicalMotion,
        &UsfScaleLayer,
        &PrimaryBodyContext,
        &GravitySample,
        Option<&DeveloperMotionOverride>,
    ), With<LocalControlSubject>>();
    let Some((
        entity,
        capabilities,
        flight_capabilities,
        navigation_capabilities,
        intent,
        law,
        autopilot,
        assistance,
        locomotion,
        execution,
        motion,
        layer,
        primary,
        gravity,
        override_,
    )) = query.iter(world).next()
    else {
        return ConsoleCommandResult::error("controlled motion stack is unavailable");
    };
    ConsoleCommandResult::success(format!(
        "subject {entity:?}\nintent axes={:?} attitude={:?} boost={} active={}\ncontrol law={} autopilot={:?}\nnavigation={:?} primary={:?}\nregime={:?} request={:?}\nexecution={:?} collision={:?} authority={:?} authority_reason={:?}\ncanonical speed={:.3} m/s angular={:?} rad/s\ncontext scale={} clearance={:.3} m gravity={:.3} m/s²\ncapabilities locomotion[character={} flight={}] flight[thrust={} rcs={} landing={}] navigation[cruise={}]\ndeveloper override={:?}",
        intent.translation_axes(),
        intent.attitude(),
        intent.boost(),
        intent.active(),
        law.label(),
        autopilot.mode(),
        assistance.mode(),
        primary.entity(),
        locomotion.regime(),
        locomotion.request(),
        execution.kernel(),
        execution.collision_policy(),
        motion.authority(),
        execution.authority_reason(),
        motion.speed_metres_per_second(),
        motion.angular_velocity_radians_per_second(),
        layer.scale(),
        primary.clearance_metres(),
        gravity.magnitude_metres_per_second2(),
        capabilities.character_enabled(),
        capabilities.inertial_flight(),
        flight_capabilities.main_propulsion(),
        flight_capabilities.reaction_control(),
        flight_capabilities.landing(),
        navigation_capabilities.cruise(),
        override_,
    ))
}
