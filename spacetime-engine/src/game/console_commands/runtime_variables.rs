//! Game-owned bindings onto the generic typed runtime-variable console substrate.
//!
//! #56/#57 game-runtime-variable-bindings-v2
//!
//! These are adapters into existing typed owners: EngineConfigOverrides,
//! CharacterMovementConfig, DebugFreecam and generic #40 presentation policy.
//! They do not create a parallel string config system and never transfer
//! interaction/collision/refinement authority to the freecam.

use bevy::prelude::*;

use crate::{
    config::{
        EngineConfig, EngineConfigOverrides, VoxelManifestationConfig,
        VoxelStreamingConfig,
    },
    console::{
        AppConsoleExt, AppRuntimeVariableExt, ConsoleArgumentCompletion,
        ConsoleCommandInvocation, ConsoleCommandResult, ConsoleCommandSpec,
        RuntimeVariableAuthority, RuntimeVariableBinding, RuntimeVariableDomain,
        RuntimeVariableSpec, RuntimeVariableValueType,
    },
    game::{
        control::LocalControlSubject,
        player::{DebugFreecam, FreecamControlPolicy, FreecamProjectionPolicy},
    },
    physics::character::CharacterMovementConfig,
    spatial::UsfViewDemandMode,
};

pub(super) fn configure(app: &mut App) {
    register_voxel_variables(app);
    register_character_variables(app);
    register_freecam_variables(app);

    app.register_console_command_with_completion(
        ConsoleCommandSpec {
            name: "debug freecam",
            aliases: &["freecam"],
            usage: "debug freecam [on|off|toggle]",
            summary: "Detach/restore the local debug camera without moving gameplay authority.",
        },
        &[ConsoleArgumentCompletion::Static(&["on", "off", "toggle"])],
        debug_freecam_command,
    );
}

fn register_voxel_variables(app: &mut App) {
    app.register_runtime_variable(RuntimeVariableBinding::new(
        RuntimeVariableSpec {
            path: "voxel.streaming.default_load_budget_per_frame",
            summary: "Default number of demanded voxel loads admitted per frame.",
            value_type: RuntimeVariableValueType::Usize,
            units: Some("chunks/frame"),
            authority: RuntimeVariableAuthority::EngineConfigOverride,
            domain: RuntimeVariableDomain::Range {
                minimum: 1.0,
                maximum: None,
            },
        },
        get_voxel_default_load_budget,
        set_voxel_default_load_budget,
        reset_voxel_default_load_budget,
        || VoxelStreamingConfig::default().default_load_budget_per_frame.to_string(),
    ))
    .register_runtime_variable(RuntimeVariableBinding::new(
        RuntimeVariableSpec {
            path: "voxel.streaming.generation_publish_budget_per_frame",
            summary: "Maximum generated voxel results published per frame.",
            value_type: RuntimeVariableValueType::Usize,
            units: Some("chunks/frame"),
            authority: RuntimeVariableAuthority::EngineConfigOverride,
            domain: RuntimeVariableDomain::Range {
                minimum: 1.0,
                maximum: None,
            },
        },
        get_voxel_publish_budget,
        set_voxel_publish_budget,
        reset_voxel_publish_budget,
        || VoxelStreamingConfig::default().generation_publish_budget_per_frame.to_string(),
    ))
    .register_runtime_variable(RuntimeVariableBinding::new(
        RuntimeVariableSpec {
            path: "voxel.streaming.warm_inactive_materialization_limit",
            summary: "Warm inactive voxel materializations retained before retirement.",
            value_type: RuntimeVariableValueType::Usize,
            units: Some("chunks"),
            authority: RuntimeVariableAuthority::EngineConfigOverride,
            domain: RuntimeVariableDomain::Range {
                minimum: 0.0,
                maximum: None,
            },
        },
        get_voxel_warm_limit,
        set_voxel_warm_limit,
        reset_voxel_warm_limit,
        || VoxelStreamingConfig::default().warm_inactive_materialization_limit.to_string(),
    ))
    .register_runtime_variable(RuntimeVariableBinding::new(
        RuntimeVariableSpec {
            path: "voxel.streaming.max_chunks_per_generation_task",
            summary: "Maximum chunks grouped into one background generation task.",
            value_type: RuntimeVariableValueType::Usize,
            units: Some("chunks/task"),
            authority: RuntimeVariableAuthority::EngineConfigOverride,
            domain: RuntimeVariableDomain::Range {
                minimum: 1.0,
                maximum: None,
            },
        },
        get_voxel_task_limit,
        set_voxel_task_limit,
        reset_voxel_task_limit,
        || VoxelStreamingConfig::default().max_chunks_per_generation_task.to_string(),
    ))
    .register_runtime_variable(RuntimeVariableBinding::new(
        RuntimeVariableSpec {
            path: "voxel.manifestation.rebuild_budget_per_frame",
            summary: "Voxel manifestation rebuilds admitted per frame.",
            value_type: RuntimeVariableValueType::Usize,
            units: Some("rebuilds/frame"),
            authority: RuntimeVariableAuthority::EngineConfigOverride,
            domain: RuntimeVariableDomain::Range {
                minimum: 1.0,
                maximum: None,
            },
        },
        get_voxel_rebuild_budget,
        set_voxel_rebuild_budget,
        reset_voxel_rebuild_budget,
        || VoxelManifestationConfig::default().rebuild_budget_per_frame.to_string(),
    ));
}

fn register_character_variables(app: &mut App) {
    app.register_runtime_variable(RuntimeVariableBinding::new(
        RuntimeVariableSpec {
            path: "player.movement.max_ground_speed_mps",
            summary: "Canonical character maximum commanded ground speed.",
            value_type: RuntimeVariableValueType::F32,
            units: Some("m/s"),
            authority: RuntimeVariableAuthority::LocalDeveloperControl,
            domain: RuntimeVariableDomain::Range {
                minimum: 0.0,
                maximum: None,
            },
        },
        get_max_ground_speed,
        set_max_ground_speed,
        reset_max_ground_speed,
        || CharacterMovementConfig::default().max_ground_speed.to_string(),
    ))
    .register_runtime_variable(RuntimeVariableBinding::new(
        RuntimeVariableSpec {
            path: "player.movement.ground_acceleration",
            summary: "Canonical character ground acceleration coefficient.",
            value_type: RuntimeVariableValueType::F32,
            units: None,
            authority: RuntimeVariableAuthority::LocalDeveloperControl,
            domain: RuntimeVariableDomain::Range {
                minimum: 0.0,
                maximum: None,
            },
        },
        get_ground_acceleration,
        set_ground_acceleration,
        reset_ground_acceleration,
        || CharacterMovementConfig::default().ground_acceleration.to_string(),
    ))
    .register_runtime_variable(RuntimeVariableBinding::new(
        RuntimeVariableSpec {
            path: "player.movement.air_acceleration",
            summary: "Canonical character air acceleration / air-strafe response coefficient.",
            value_type: RuntimeVariableValueType::F32,
            units: None,
            authority: RuntimeVariableAuthority::LocalDeveloperControl,
            domain: RuntimeVariableDomain::Range {
                minimum: 0.0,
                maximum: None,
            },
        },
        get_air_acceleration,
        set_air_acceleration,
        reset_air_acceleration,
        || CharacterMovementConfig::default().air_acceleration.to_string(),
    ))
    .register_runtime_variable(RuntimeVariableBinding::new(
        RuntimeVariableSpec {
            path: "player.movement.air_wish_speed_cap_mps",
            summary: "Canonical character air wish-speed cap; `none` disables the cap.",
            value_type: RuntimeVariableValueType::OptionalF32,
            units: Some("m/s"),
            authority: RuntimeVariableAuthority::LocalDeveloperControl,
            domain: RuntimeVariableDomain::Any,
        },
        get_air_wish_speed_cap,
        set_air_wish_speed_cap,
        reset_air_wish_speed_cap,
        || format_optional_f32(CharacterMovementConfig::default().air_wish_speed_cap),
    ));
}

fn register_freecam_variables(app: &mut App) {
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

fn debug_freecam_command(
    world: &mut World,
    invocation: &ConsoleCommandInvocation,
) -> ConsoleCommandResult {
    if invocation.args().len() > 1 {
        return ConsoleCommandResult::error("usage: debug freecam [on|off|toggle]");
    }

    let current = world.resource::<DebugFreecam>().enabled();
    let enabled = match invocation.args().first().map(String::as_str) {
        None | Some("toggle") => !current,
        Some(value) if value.eq_ignore_ascii_case("on") => true,
        Some(value) if value.eq_ignore_ascii_case("off") => false,
        Some(value) => {
            return ConsoleCommandResult::error(format!(
                "invalid freecam state `{value}`; expected on, off or toggle"
            ));
        }
    };
    world.resource_mut::<DebugFreecam>().set_enabled(enabled);

    let settings = world.resource::<DebugFreecam>();
    ConsoleCommandResult::success_and_return_to_gameplay(format!(
        "debug freecam {} | control={} projection={} view-demand={} | dense gameplay demand unchanged",
        if enabled { "enabled" } else { "disabled" },
        settings.control_policy().label(),
        settings.projection_policy().label(),
        settings.view_demand_mode().label(),
    ))
}

fn parse_usize(raw: &str, field: &str, minimum: usize) -> Result<usize, String> {
    let value = raw
        .parse::<usize>()
        .map_err(|_| format!("{field} must be an unsigned integer"))?;
    if value < minimum {
        return Err(format!("{field} must be >= {minimum}"));
    }
    Ok(value)
}

fn parse_nonnegative_f32(raw: &str, field: &str) -> Result<f32, String> {
    let value = raw
        .parse::<f32>()
        .map_err(|_| format!("{field} must be a number"))?;
    if !value.is_finite() || value < 0.0 {
        return Err(format!("{field} must be finite and non-negative"));
    }
    Ok(value)
}

fn parse_positive_f32(raw: &str, field: &str) -> Result<f32, String> {
    let value = raw
        .parse::<f32>()
        .map_err(|_| format!("{field} must be a number"))?;
    if !value.is_finite() || value <= 0.0 {
        return Err(format!("{field} must be finite and > 0"));
    }
    Ok(value)
}

fn parse_bool(raw: &str, field: &str) -> Result<bool, String> {
    match raw.to_ascii_lowercase().as_str() {
        "true" | "on" | "1" => Ok(true),
        "false" | "off" | "0" => Ok(false),
        _ => Err(format!("{field} must be true/false or on/off")),
    }
}

fn character_config(world: &mut World) -> Result<CharacterMovementConfig, String> {
    let mut query =
        world.query_filtered::<&CharacterMovementConfig, With<LocalControlSubject>>();
    query
        .iter(world)
        .next()
        .cloned()
        .ok_or_else(|| "controlled character movement config is unavailable".to_string())
}

fn with_character_config(
    world: &mut World,
    update: impl FnOnce(&mut CharacterMovementConfig),
) -> Result<(), String> {
    let mut query =
        world.query_filtered::<&mut CharacterMovementConfig, With<LocalControlSubject>>();
    let Some(mut config) = query.iter_mut(world).next() else {
        return Err("controlled character movement config is unavailable".to_string());
    };
    update(&mut config);
    Ok(())
}

fn get_max_ground_speed(world: &mut World) -> Result<String, String> {
    Ok(character_config(world)?.max_ground_speed.to_string())
}
fn set_max_ground_speed(world: &mut World, raw: &str) -> Result<(), String> {
    let value = parse_nonnegative_f32(raw, "max ground speed")?;
    with_character_config(world, |config| config.max_ground_speed = value)
}
fn reset_max_ground_speed(world: &mut World) -> Result<(), String> {
    let value = CharacterMovementConfig::default().max_ground_speed;
    with_character_config(world, |config| config.max_ground_speed = value)
}

fn get_ground_acceleration(world: &mut World) -> Result<String, String> {
    Ok(character_config(world)?.ground_acceleration.to_string())
}
fn set_ground_acceleration(world: &mut World, raw: &str) -> Result<(), String> {
    let value = parse_nonnegative_f32(raw, "ground acceleration")?;
    with_character_config(world, |config| config.ground_acceleration = value)
}
fn reset_ground_acceleration(world: &mut World) -> Result<(), String> {
    let value = CharacterMovementConfig::default().ground_acceleration;
    with_character_config(world, |config| config.ground_acceleration = value)
}

fn get_air_acceleration(world: &mut World) -> Result<String, String> {
    Ok(character_config(world)?.air_acceleration.to_string())
}
fn set_air_acceleration(world: &mut World, raw: &str) -> Result<(), String> {
    let value = parse_nonnegative_f32(raw, "air acceleration")?;
    with_character_config(world, |config| config.air_acceleration = value)
}
fn reset_air_acceleration(world: &mut World) -> Result<(), String> {
    let value = CharacterMovementConfig::default().air_acceleration;
    with_character_config(world, |config| config.air_acceleration = value)
}

fn format_optional_f32(value: Option<f32>) -> String {
    value.map_or_else(|| "none".to_string(), |value| value.to_string())
}
fn get_air_wish_speed_cap(world: &mut World) -> Result<String, String> {
    Ok(format_optional_f32(
        character_config(world)?.air_wish_speed_cap,
    ))
}
fn set_air_wish_speed_cap(world: &mut World, raw: &str) -> Result<(), String> {
    let value = if raw.eq_ignore_ascii_case("none") {
        None
    } else {
        Some(parse_nonnegative_f32(raw, "air wish-speed cap")?)
    };
    with_character_config(world, |config| config.air_wish_speed_cap = value)
}
fn reset_air_wish_speed_cap(world: &mut World) -> Result<(), String> {
    let value = CharacterMovementConfig::default().air_wish_speed_cap;
    with_character_config(world, |config| config.air_wish_speed_cap = value)
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
    Ok(world.resource::<DebugFreecam>().boost_multiplier().to_string())
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
    Ok(world.resource::<DebugFreecam>().control_policy().label().to_string())
}
fn set_freecam_control_policy(world: &mut World, raw: &str) -> Result<(), String> {
    let Some(policy) = FreecamControlPolicy::parse(raw) else {
        return Err("freecam control policy must be exclusive or passthrough".to_string());
    };
    world.resource_mut::<DebugFreecam>().set_control_policy(policy);
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
    world.resource_mut::<DebugFreecam>().set_view_demand_mode(mode);
    Ok(())
}
fn reset_freecam_view_demand(world: &mut World) -> Result<(), String> {
    world
        .resource_mut::<DebugFreecam>()
        .set_view_demand_mode(UsfViewDemandMode::Frozen);
    Ok(())
}

fn get_voxel_default_load_budget(world: &mut World) -> Result<String, String> {
    let override_value = world
        .resource::<EngineConfigOverrides>()
        .voxel
        .streaming
        .default_load_budget_per_frame;
    Ok(override_value
        .unwrap_or(world.resource::<EngineConfig>().voxel.streaming.default_load_budget_per_frame)
        .to_string())
}
fn set_voxel_default_load_budget(world: &mut World, raw: &str) -> Result<(), String> {
    let value = parse_usize(raw, "default load budget", 1)?;
    world
        .resource_mut::<EngineConfigOverrides>()
        .voxel
        .streaming
        .default_load_budget_per_frame = Some(value);
    Ok(())
}
fn reset_voxel_default_load_budget(world: &mut World) -> Result<(), String> {
    world
        .resource_mut::<EngineConfigOverrides>()
        .voxel
        .streaming
        .default_load_budget_per_frame = None;
    Ok(())
}

fn get_voxel_publish_budget(world: &mut World) -> Result<String, String> {
    let override_value = world
        .resource::<EngineConfigOverrides>()
        .voxel
        .streaming
        .generation_publish_budget_per_frame;
    Ok(override_value
        .unwrap_or(
            world
                .resource::<EngineConfig>()
                .voxel
                .streaming
                .generation_publish_budget_per_frame,
        )
        .to_string())
}
fn set_voxel_publish_budget(world: &mut World, raw: &str) -> Result<(), String> {
    let value = parse_usize(raw, "generation publish budget", 1)?;
    world
        .resource_mut::<EngineConfigOverrides>()
        .voxel
        .streaming
        .generation_publish_budget_per_frame = Some(value);
    Ok(())
}
fn reset_voxel_publish_budget(world: &mut World) -> Result<(), String> {
    world
        .resource_mut::<EngineConfigOverrides>()
        .voxel
        .streaming
        .generation_publish_budget_per_frame = None;
    Ok(())
}

fn get_voxel_warm_limit(world: &mut World) -> Result<String, String> {
    let override_value = world
        .resource::<EngineConfigOverrides>()
        .voxel
        .streaming
        .warm_inactive_materialization_limit;
    Ok(override_value
        .unwrap_or(
            world
                .resource::<EngineConfig>()
                .voxel
                .streaming
                .warm_inactive_materialization_limit,
        )
        .to_string())
}
fn set_voxel_warm_limit(world: &mut World, raw: &str) -> Result<(), String> {
    let value = parse_usize(raw, "warm inactive materialization limit", 0)?;
    world
        .resource_mut::<EngineConfigOverrides>()
        .voxel
        .streaming
        .warm_inactive_materialization_limit = Some(value);
    Ok(())
}
fn reset_voxel_warm_limit(world: &mut World) -> Result<(), String> {
    world
        .resource_mut::<EngineConfigOverrides>()
        .voxel
        .streaming
        .warm_inactive_materialization_limit = None;
    Ok(())
}

fn get_voxel_task_limit(world: &mut World) -> Result<String, String> {
    let override_value = world
        .resource::<EngineConfigOverrides>()
        .voxel
        .streaming
        .max_chunks_per_generation_task;
    Ok(override_value
        .unwrap_or(
            world
                .resource::<EngineConfig>()
                .voxel
                .streaming
                .max_chunks_per_generation_task,
        )
        .to_string())
}
fn set_voxel_task_limit(world: &mut World, raw: &str) -> Result<(), String> {
    let value = parse_usize(raw, "max chunks per generation task", 1)?;
    world
        .resource_mut::<EngineConfigOverrides>()
        .voxel
        .streaming
        .max_chunks_per_generation_task = Some(value);
    Ok(())
}
fn reset_voxel_task_limit(world: &mut World) -> Result<(), String> {
    world
        .resource_mut::<EngineConfigOverrides>()
        .voxel
        .streaming
        .max_chunks_per_generation_task = None;
    Ok(())
}

fn get_voxel_rebuild_budget(world: &mut World) -> Result<String, String> {
    let override_value = world
        .resource::<EngineConfigOverrides>()
        .voxel
        .manifestation
        .rebuild_budget_per_frame;
    Ok(override_value
        .unwrap_or(
            world
                .resource::<EngineConfig>()
                .voxel
                .manifestation
                .rebuild_budget_per_frame,
        )
        .to_string())
}
fn set_voxel_rebuild_budget(world: &mut World, raw: &str) -> Result<(), String> {
    let value = parse_usize(raw, "voxel rebuild budget", 1)?;
    world
        .resource_mut::<EngineConfigOverrides>()
        .voxel
        .manifestation
        .rebuild_budget_per_frame = Some(value);
    Ok(())
}
fn reset_voxel_rebuild_budget(world: &mut World) -> Result<(), String> {
    world
        .resource_mut::<EngineConfigOverrides>()
        .voxel
        .manifestation
        .rebuild_budget_per_frame = None;
    Ok(())
}
