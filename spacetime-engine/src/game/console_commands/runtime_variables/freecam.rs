//! Detached camera controls and their presentation-only demand policy.

use super::{parse_bool, parse_nonnegative_f32, parse_positive_f32};
use crate::console::{
    AppRuntimeVariableExt, RuntimeVariableAuthority, RuntimeVariableBinding, RuntimeVariableDomain,
    RuntimeVariableSpec, RuntimeVariableValueType,
};
use crate::{
    console::{ConsoleCommandInvocation, ConsoleCommandResult},
    game::{
        devtools::lab,
        player::{DebugFreecam, FreecamControlPolicy, FreecamProjectionPolicy},
    },
    spatial::UsfViewDemandMode,
};
use bevy::prelude::*;

pub(super) fn register(app: &mut App) {
    app.register_runtime_variable(RuntimeVariableBinding::new(
        RuntimeVariableSpec {
            path: "debug.freecam.enabled",
            summary: "Whether the detached local debug camera is active.",
            value_type: RuntimeVariableValueType::Boolean,
            units: None,
            authority: RuntimeVariableAuthority::LocalDeveloperControl,
            domain: RuntimeVariableDomain::Choices(&["true", "false", "on", "off"]),
        },
        get_freecam_enabled,
        set_freecam_enabled,
        reset_freecam_enabled,
        || "false".to_string(),
    ))
    .register_runtime_variable(RuntimeVariableBinding::new(
        RuntimeVariableSpec {
            path: "debug.freecam.translation_speed_mps",
            summary: "Base translation speed of the detached debug camera.",
            value_type: RuntimeVariableValueType::F32,
            units: Some("m/s"),
            authority: RuntimeVariableAuthority::LocalDeveloperControl,
            domain: RuntimeVariableDomain::Range {
                minimum: 0.0,
                maximum: None,
            },
        },
        get_freecam_speed,
        set_freecam_speed,
        reset_freecam_speed,
        || DebugFreecam::DEFAULT_TRANSLATION_SPEED_MPS.to_string(),
    ))
    .register_runtime_variable(RuntimeVariableBinding::new(
        RuntimeVariableSpec {
            path: "debug.freecam.boost_multiplier",
            summary: "Shift/FastModifier speed multiplier for the detached debug camera.",
            value_type: RuntimeVariableValueType::F32,
            units: Some("x"),
            authority: RuntimeVariableAuthority::LocalDeveloperControl,
            domain: RuntimeVariableDomain::Range {
                minimum: 0.000_001,
                maximum: None,
            },
        },
        get_freecam_boost,
        set_freecam_boost,
        reset_freecam_boost,
        || DebugFreecam::DEFAULT_BOOST_MULTIPLIER.to_string(),
    ))
    .register_runtime_variable(RuntimeVariableBinding::new(
        RuntimeVariableSpec {
            path: "debug.freecam.control_policy",
            summary: "Whether freecam exclusively consumes controls or passes them through to gameplay.",
            value_type: RuntimeVariableValueType::Enum,
            units: None,
            authority: RuntimeVariableAuthority::LocalDeveloperControl,
            domain: RuntimeVariableDomain::Choices(&["exclusive", "passthrough"]),
        },
        get_freecam_control_policy,
        set_freecam_control_policy,
        reset_freecam_control_policy,
        || FreecamControlPolicy::Exclusive.label().to_string(),
    ))
    .register_runtime_variable(RuntimeVariableBinding::new(
        RuntimeVariableSpec {
            path: "debug.freecam.projection_policy",
            summary: "How the USF presentation projection follows the detached camera.",
            value_type: RuntimeVariableValueType::Enum,
            units: None,
            authority: RuntimeVariableAuthority::LocalDeveloperControl,
            domain: RuntimeVariableDomain::Choices(&["follow", "frozen", "disabled"]),
        },
        get_freecam_projection_policy,
        set_freecam_projection_policy,
        reset_freecam_projection_policy,
        || FreecamProjectionPolicy::Follow.label().to_string(),
    ))
    .register_runtime_variable(RuntimeVariableBinding::new(
        RuntimeVariableSpec {
            path: "debug.freecam.view_demand",
            summary: "Presentation-demand policy while freecam is enabled; dense world demand remains gameplay-owned.",
            value_type: RuntimeVariableValueType::Enum,
            units: None,
            authority: RuntimeVariableAuthority::LocalDeveloperControl,
            domain: RuntimeVariableDomain::Choices(&["live", "frozen", "disabled"]),
        },
        get_freecam_view_demand,
        set_freecam_view_demand,
        reset_freecam_view_demand,
        || UsfViewDemandMode::Frozen.label().to_string(),
    ));
}
pub(super) fn debug_freecam_command(
    world: &mut World,
    invocation: &ConsoleCommandInvocation,
) -> ConsoleCommandResult {
    if invocation.args().len() > 1 {
        return ConsoleCommandResult::error("usage: debug freecam [on|off|toggle]");
    }

    let preset_active = lab::preset_active(world, "freecam");
    let manually_enabled = world.resource::<DebugFreecam>().enabled();
    let enable = match invocation.args().first().map(String::as_str) {
        None | Some("toggle") => !(preset_active || manually_enabled),
        Some(value) if value.eq_ignore_ascii_case("on") => true,
        Some(value) if value.eq_ignore_ascii_case("off") => false,
        Some(value) => {
            return ConsoleCommandResult::error(format!(
                "invalid freecam state `{value}`; expected on, off or toggle"
            ));
        }
    };

    let result = if enable {
        lab::apply_preset(world, "freecam")
    } else if preset_active {
        lab::clear_preset(world, Some("freecam"))
    } else {
        world.resource_mut::<DebugFreecam>().set_enabled(false);
        Ok(true)
    };
    if let Err(error) = result {
        return ConsoleCommandResult::error(error);
    }

    let settings = world.resource::<DebugFreecam>();
    ConsoleCommandResult::success_and_return_to_gameplay(format!(
        "debug freecam {} | preset={} control={} projection={} view-demand={} | dense gameplay demand unchanged",
        if settings.enabled() {
            "enabled"
        } else {
            "disabled"
        },
        if lab::preset_active(world, "freecam") {
            "active"
        } else {
            "inactive"
        },
        settings.control_policy().label(),
        settings.projection_policy().label(),
        settings.view_demand_mode().label(),
    ))
}
fn get_freecam_enabled(world: &mut World) -> Result<String, String> {
    Ok(world.resource::<DebugFreecam>().enabled().to_string())
}
fn set_freecam_enabled(world: &mut World, raw: &str) -> Result<(), String> {
    let enabled = parse_bool(raw, "debug.freecam.enabled")?;
    world.resource_mut::<DebugFreecam>().set_enabled(enabled);
    Ok(())
}
fn reset_freecam_enabled(world: &mut World) -> Result<(), String> {
    world.resource_mut::<DebugFreecam>().set_enabled(false);
    Ok(())
}
fn get_freecam_speed(world: &mut World) -> Result<String, String> {
    Ok(world
        .resource::<DebugFreecam>()
        .translation_speed_mps()
        .to_string())
}
fn set_freecam_speed(world: &mut World, raw: &str) -> Result<(), String> {
    let value = parse_nonnegative_f32(raw, "freecam translation speed")?;
    world
        .resource_mut::<DebugFreecam>()
        .set_translation_speed_mps(value)
}
fn reset_freecam_speed(world: &mut World) -> Result<(), String> {
    world
        .resource_mut::<DebugFreecam>()
        .set_translation_speed_mps(DebugFreecam::DEFAULT_TRANSLATION_SPEED_MPS)
}
fn get_freecam_boost(world: &mut World) -> Result<String, String> {
    Ok(world
        .resource::<DebugFreecam>()
        .boost_multiplier()
        .to_string())
}
fn set_freecam_boost(world: &mut World, raw: &str) -> Result<(), String> {
    let value = parse_positive_f32(raw, "freecam boost multiplier")?;
    world
        .resource_mut::<DebugFreecam>()
        .set_boost_multiplier(value)
}
fn reset_freecam_boost(world: &mut World) -> Result<(), String> {
    world
        .resource_mut::<DebugFreecam>()
        .set_boost_multiplier(DebugFreecam::DEFAULT_BOOST_MULTIPLIER)
}

fn get_freecam_control_policy(world: &mut World) -> Result<String, String> {
    Ok(world
        .resource::<DebugFreecam>()
        .control_policy()
        .label()
        .to_string())
}
fn set_freecam_control_policy(world: &mut World, raw: &str) -> Result<(), String> {
    let Some(policy) = FreecamControlPolicy::parse(raw) else {
        return Err("freecam control policy must be exclusive or passthrough".to_string());
    };
    world
        .resource_mut::<DebugFreecam>()
        .set_control_policy(policy);
    Ok(())
}
fn reset_freecam_control_policy(world: &mut World) -> Result<(), String> {
    world
        .resource_mut::<DebugFreecam>()
        .set_control_policy(FreecamControlPolicy::Exclusive);
    Ok(())
}

fn get_freecam_projection_policy(world: &mut World) -> Result<String, String> {
    Ok(world
        .resource::<DebugFreecam>()
        .projection_policy()
        .label()
        .to_string())
}
fn set_freecam_projection_policy(world: &mut World, raw: &str) -> Result<(), String> {
    let Some(policy) = FreecamProjectionPolicy::parse(raw) else {
        return Err("freecam projection policy must be follow, frozen or disabled".to_string());
    };
    world
        .resource_mut::<DebugFreecam>()
        .set_projection_policy(policy);
    Ok(())
}
fn reset_freecam_projection_policy(world: &mut World) -> Result<(), String> {
    world
        .resource_mut::<DebugFreecam>()
        .set_projection_policy(FreecamProjectionPolicy::Follow);
    Ok(())
}

fn get_freecam_view_demand(world: &mut World) -> Result<String, String> {
    Ok(world
        .resource::<DebugFreecam>()
        .view_demand_mode()
        .label()
        .to_string())
}
fn set_freecam_view_demand(world: &mut World, raw: &str) -> Result<(), String> {
    let Some(mode) = UsfViewDemandMode::parse(raw) else {
        return Err("freecam view demand must be live, frozen or disabled".to_string());
    };
    world
        .resource_mut::<DebugFreecam>()
        .set_view_demand_mode(mode);
    Ok(())
}
fn reset_freecam_view_demand(world: &mut World) -> Result<(), String> {
    world
        .resource_mut::<DebugFreecam>()
        .set_view_demand_mode(UsfViewDemandMode::Frozen);
    Ok(())
}
