//! Engine config override bindings for voxel work budgets.

use super::parse_usize;
use crate::config::{
    EngineConfig, EngineConfigOverrides, VoxelManifestationConfig, VoxelStreamingConfig,
};
use crate::console::{
    AppRuntimeVariableExt, RuntimeVariableAuthority, RuntimeVariableBinding, RuntimeVariableDomain,
    RuntimeVariableSpec, RuntimeVariableValueType,
};
use bevy::prelude::*;

pub(super) fn register(app: &mut App) {
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
        || {
            VoxelStreamingConfig::default()
                .default_load_budget_per_frame
                .to_string()
        },
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
        || {
            VoxelStreamingConfig::default()
                .generation_publish_budget_per_frame
                .to_string()
        },
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
        || {
            VoxelStreamingConfig::default()
                .warm_inactive_materialization_limit
                .to_string()
        },
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
        || {
            VoxelStreamingConfig::default()
                .max_chunks_per_generation_task
                .to_string()
        },
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
        || {
            VoxelManifestationConfig::default()
                .rebuild_budget_per_frame
                .to_string()
        },
    ));
}
fn get_voxel_default_load_budget(world: &mut World) -> Result<String, String> {
    let override_value = world
        .resource::<EngineConfigOverrides>()
        .voxel
        .streaming
        .default_load_budget_per_frame;
    Ok(override_value
        .unwrap_or(
            world
                .resource::<EngineConfig>()
                .voxel
                .streaming
                .default_load_budget_per_frame,
        )
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
