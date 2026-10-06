//! Observer-scale, pace, and cruise console adapters.

use super::super::{
    control::LocalControlSubject,
    flight::{AttitudeAutopilot, PilotAttitudeLaw},
    locomotion::{
        ControlledSubjectLocomotion, DeveloperMotionOverride, FlightControlIntent,
        LocomotionCapabilities, MotionExecution,
    },
    navigation::{
        AdaptiveCruise, PrimaryBodyContext, TravelAssistance, TravelAssistanceState,
        TravelAssistanceTransitionReason, TravelPace, TravelState,
    },
};
use super::primary_view_context;
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

    let mut query = world.query_filtered::<&mut TravelPace, With<LocalControlSubject>>();
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
        "manual locomotion pace = {:.3}x",
        speed.multiplier,
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
        &LocomotionCapabilities,
        &TravelState,
        &mut TravelAssistanceState,
        &mut AdaptiveCruise,
    ), With<LocalControlSubject>>();
    let Some((capabilities, travel, mut assistance, mut cruise)) = query.iter_mut(world).next()
    else {
        return ConsoleCommandResult::error("player locomotion state is unavailable");
    };

    let active = match invocation.args().first().map(String::as_str) {
        None => assistance.mode() != TravelAssistance::Cruise,
        Some(value) if value.eq_ignore_ascii_case("on") => true,
        Some(value) if value.eq_ignore_ascii_case("off") => false,
        Some(value) => {
            return ConsoleCommandResult::error(format!(
                "invalid Cruise state `{value}`; expected on or off"
            ));
        }
    };

    if active {
        if !capabilities.cruise() || !travel.cruise_entry_available {
            return ConsoleCommandResult::error(
                "Cruise is unavailable for this subject or approach",
            );
        }
        assistance.engage_cruise();
    } else {
        assistance.disengage(TravelAssistanceTransitionReason::PilotDisengaged);
    }
    if !active {
        cruise.throttle = 0.0;
        cruise.speed_scale0 = 0.0;
    }

    ConsoleCommandResult::success_and_return_to_gameplay(if active {
        "adaptive Cruise enabled — W/S throttle, mouse steers"
    } else {
        "adaptive Cruise disabled"
    })
}

pub(super) fn attitude_command(
    world: &mut World,
    invocation: &ConsoleCommandInvocation,
) -> ConsoleCommandResult {
    if invocation.args().len() > 1 {
        return ConsoleCommandResult::error("usage: attitude [hold|view]");
    }
    let mut query = world.query_filtered::<&mut PilotAttitudeLaw, With<LocalControlSubject>>();
    let Some(mut law) = query.iter_mut(world).next() else {
        return ConsoleCommandResult::error("controlled attitude law is unavailable");
    };
    if let Some(value) = invocation.args().first() {
        *law = match value.to_ascii_lowercase().as_str() {
            "hold" => PilotAttitudeLaw::Hold,
            "view" | "follow" => PilotAttitudeLaw::FollowView,
            _ => return ConsoleCommandResult::error("usage: attitude [hold|view]"),
        };
    }
    ConsoleCommandResult::success_and_return_to_gameplay(format!(
        "pilot attitude law = {}",
        law.label(),
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

pub(super) fn autopilot_command(
    world: &mut World,
    invocation: &ConsoleCommandInvocation,
) -> ConsoleCommandResult {
    if invocation.args().len() > 1 {
        return ConsoleCommandResult::error("usage: autopilot [off|hold|prograde]");
    }
    let mut query = world.query_filtered::<(&Transform, &LocomotionCapabilities, &mut AttitudeAutopilot), With<LocalControlSubject>>();
    let Some((body, capabilities, mut autopilot)) = query.iter_mut(world).next() else {
        return ConsoleCommandResult::error("controlled autopilot state is unavailable");
    };
    if !capabilities.inertial_flight() {
        return ConsoleCommandResult::error("autopilot requires a flight subject");
    }
    if let Some(value) = invocation.args().first() {
        match value.to_ascii_lowercase().as_str() {
            "off" => autopilot.disengage(),
            "hold" => autopilot.hold(body.rotation),
            "prograde" => autopilot.point_prograde(),
            _ => return ConsoleCommandResult::error("usage: autopilot [off|hold|prograde]"),
        }
    }
    ConsoleCommandResult::success_and_return_to_gameplay(format!(
        "attitude autopilot = {:?}",
        autopilot.mode()
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
        "subject {entity:?}\nintent axes={:?} attitude={:?} boost={} active={}\ncontrol law={} autopilot={:?}\nnavigation={:?} primary={:?}\nregime={:?} request={:?}\nexecution={:?} collision={:?} authority={:?}\ncanonical speed={:.3} m/s angular={:?} rad/s\ncontext scale={} clearance={:.3} m gravity={:.3} m/s²\ncapabilities character={} flight={} thrust={} rcs={} landing={} cruise={}\ndeveloper override={:?}",
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
        motion.speed_metres_per_second(),
        motion.angular_velocity_radians_per_second(),
        layer.scale(),
        primary.clearance_metres(),
        gravity.magnitude_metres_per_second2(),
        capabilities.character_enabled(),
        capabilities.inertial_flight(),
        capabilities.main_propulsion(),
        capabilities.reaction_control(),
        capabilities.landing(),
        capabilities.cruise(),
        override_,
    ))
}
