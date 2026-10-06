//! Observer-scale, pace, and cruise console adapters.

use super::super::{
    control::LocalControlSubject,
    locomotion::{ControlledSubjectLocomotion, LocomotionRegime, LocomotionRequest},
    navigation::{AdaptiveCruise, TravelPace},
};
use super::primary_view_context;
use crate::{
    console::{ConsoleCommandInvocation, ConsoleCommandResult},
    spatial::{UsfPrimaryInteractionSlice, UsfViewContext, UsfViewRenderAnchor},
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

    let mut query = world.query_filtered::<
        (&mut ControlledSubjectLocomotion, &mut AdaptiveCruise),
        With<LocalControlSubject>,
    >();
    let Some((mut locomotion, mut cruise)) = query.iter_mut(world).next() else {
        return ConsoleCommandResult::error("player locomotion state is unavailable");
    };

    let active = match invocation.args().first().map(String::as_str) {
        None => locomotion.request() != LocomotionRequest::Regime(LocomotionRegime::Cruise),
        Some(value) if value.eq_ignore_ascii_case("on") => true,
        Some(value) if value.eq_ignore_ascii_case("off") => false,
        Some(value) => {
            return ConsoleCommandResult::error(format!(
                "invalid Cruise state `{value}`; expected on or off"
            ));
        }
    };

    if active {
        locomotion.request_regime(LocomotionRegime::Cruise);
        locomotion.set_thrusters_enabled(false);
    } else {
        locomotion.request_automatic();
    }
    cruise.throttle = 0.0;
    cruise.speed_scale0 = 0.0;

    ConsoleCommandResult::success_and_return_to_gameplay(if active {
        "adaptive Cruise enabled — W/S throttle, mouse steers"
    } else {
        "adaptive Cruise disabled"
    })
}
