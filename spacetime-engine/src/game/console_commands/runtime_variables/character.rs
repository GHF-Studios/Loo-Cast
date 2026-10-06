//! Controlled character movement bindings.

use super::parse_nonnegative_f32;
use crate::console::{
    AppRuntimeVariableExt, RuntimeVariableAuthority, RuntimeVariableBinding, RuntimeVariableDomain,
    RuntimeVariableSpec, RuntimeVariableValueType,
};
use crate::{game::control::LocalControlSubject, physics::character::CharacterMovementConfig};
use bevy::prelude::*;

pub(super) fn register(app: &mut App) {
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
        || {
            CharacterMovementConfig::default()
                .max_ground_speed
                .to_string()
        },
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
        || {
            CharacterMovementConfig::default()
                .ground_acceleration
                .to_string()
        },
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
        || {
            CharacterMovementConfig::default()
                .air_acceleration
                .to_string()
        },
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
fn character_config(world: &mut World) -> Result<CharacterMovementConfig, String> {
    let mut query = world.query_filtered::<&CharacterMovementConfig, With<LocalControlSubject>>();
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
