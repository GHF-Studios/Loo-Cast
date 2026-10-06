//! Developer locomotion override binding.

use crate::console::{
    AppRuntimeVariableExt, RuntimeVariableAuthority, RuntimeVariableBinding, RuntimeVariableDomain,
    RuntimeVariableSpec, RuntimeVariableValueType,
};
use crate::game::{
    control::LocalControlSubject,
    locomotion::{LocomotionCapabilities, LocomotionRegime, LocomotionRegimeOverride},
};
use bevy::prelude::*;

pub(super) fn register(app: &mut App) {
    app.register_runtime_variable(RuntimeVariableBinding::new(
        RuntimeVariableSpec {
            path: "debug.locomotion.regime",
            summary: "Temporary locomotion resolver override; `automatic` restores normal policy.",
            value_type: RuntimeVariableValueType::Enum,
            units: None,
            authority: RuntimeVariableAuthority::LocalDeveloperControl,
            domain: RuntimeVariableDomain::Choices(&["automatic", "on_foot", "spacecraft_flight"]),
        },
        get_debug_locomotion_regime,
        set_debug_locomotion_regime,
        reset_debug_locomotion_regime,
        || "automatic".to_string(),
    ));
}
fn debug_regime_label(regime: LocomotionRegime) -> &'static str {
    match regime {
        LocomotionRegime::OnFoot => "on_foot",
        LocomotionRegime::SpacecraftFlight => "spacecraft_flight",
    }
}

fn parse_debug_regime(raw: &str) -> Result<Option<LocomotionRegime>, String> {
    let normalized = raw.trim().to_ascii_lowercase().replace('-', "_");
    Ok(match normalized.as_str() {
        "automatic" | "auto" | "none" | "off" => None,
        "on_foot" | "onfoot" | "foot" => Some(LocomotionRegime::OnFoot),
        "spacecraft_flight" | "flight" => Some(LocomotionRegime::SpacecraftFlight),
        _ => {
            return Err(format!(
                "debug.locomotion.regime must be automatic, on_foot or spacecraft_flight; got `{raw}`"
            ));
        }
    })
}

fn controlled_locomotion_target(
    world: &mut World,
) -> Result<(Entity, LocomotionCapabilities), String> {
    let mut query =
        world.query_filtered::<(Entity, &LocomotionCapabilities), With<LocalControlSubject>>();
    query
        .iter(world)
        .next()
        .map(|(entity, capabilities)| (entity, *capabilities))
        .ok_or_else(|| "controlled locomotion subject is unavailable".to_string())
}

fn get_debug_locomotion_regime(world: &mut World) -> Result<String, String> {
    let (entity, _) = controlled_locomotion_target(world)?;
    Ok(world
        .get::<LocomotionRegimeOverride>(entity)
        .copied()
        .map_or("automatic", |value| debug_regime_label(value.regime()))
        .to_string())
}

fn set_debug_locomotion_regime(world: &mut World, raw: &str) -> Result<(), String> {
    let requested = parse_debug_regime(raw)?;
    let (entity, capabilities) = controlled_locomotion_target(world)?;

    match requested {
        None => {
            world
                .entity_mut(entity)
                .remove::<LocomotionRegimeOverride>();
        }
        Some(regime) => {
            if !capabilities.supports_regime(regime) {
                return Err(format!(
                    "controlled subject does not support forced locomotion regime `{}`",
                    debug_regime_label(regime)
                ));
            }
            world
                .entity_mut(entity)
                .insert(LocomotionRegimeOverride::new(regime));
        }
    }
    Ok(())
}

fn reset_debug_locomotion_regime(world: &mut World) -> Result<(), String> {
    set_debug_locomotion_regime(world, "automatic")
}
