//! Console bindings for typed voxel work-budget overrides.

use super::parse_usize;
use crate::config::{EngineConfig, EngineConfigOverrides};
use crate::console::{
    AppRuntimeVariableExt, RuntimeVariableAuthority, RuntimeVariableBinding, RuntimeVariableDomain,
    RuntimeVariableSpec, RuntimeVariableValueType,
};
use bevy::prelude::*;

// Each binding reads the resolved config, writes only its typed override, and
// clears that override on reset. Keep those operations together so a setting
// cannot accidentally read one field while writing another.
macro_rules! bind_usize {
    ($app:expr, $section:ident.$field:ident, $summary:literal, $units:literal, $label:literal, $minimum:literal) => {
        $app.register_runtime_variable(RuntimeVariableBinding::new(
            RuntimeVariableSpec {
                path: concat!("voxel.", stringify!($section), ".", stringify!($field)),
                summary: $summary,
                value_type: RuntimeVariableValueType::Usize,
                units: Some($units),
                authority: RuntimeVariableAuthority::EngineConfigOverride,
                domain: RuntimeVariableDomain::Range {
                    minimum: $minimum as f64,
                    maximum: None,
                },
            },
            |world: &mut World| {
                let override_value = world
                    .resource::<EngineConfigOverrides>()
                    .voxel
                    .$section
                    .$field;
                let value = override_value
                    .unwrap_or(world.resource::<EngineConfig>().voxel.$section.$field);
                Ok(value.to_string())
            },
            |world: &mut World, raw: &str| {
                let value = parse_usize(raw, $label, $minimum)?;
                world
                    .resource_mut::<EngineConfigOverrides>()
                    .voxel
                    .$section
                    .$field = Some(value);
                Ok(())
            },
            |world: &mut World| {
                world
                    .resource_mut::<EngineConfigOverrides>()
                    .voxel
                    .$section
                    .$field = None;
                Ok(())
            },
            || EngineConfig::default().voxel.$section.$field.to_string(),
        ));
    };
}

pub(super) fn register(app: &mut App) {
    bind_usize!(
        app,
        streaming.default_load_budget_per_frame,
        "Default number of demanded voxel loads admitted per frame.",
        "chunks/frame",
        "default load budget",
        1
    );
    bind_usize!(
        app,
        streaming.generation_publish_budget_per_frame,
        "Maximum generated voxel results published per frame.",
        "chunks/frame",
        "generation publish budget",
        1
    );
    bind_usize!(
        app,
        streaming.warm_inactive_materialization_limit,
        "Warm inactive voxel materializations retained before retirement.",
        "chunks",
        "warm inactive materialization limit",
        0
    );
    bind_usize!(
        app,
        streaming.max_chunks_per_generation_task,
        "Maximum chunks grouped into one background generation task.",
        "chunks/task",
        "max chunks per generation task",
        1
    );
    bind_usize!(
        app,
        manifestation.rebuild_budget_per_frame,
        "Voxel manifestation rebuilds admitted per frame.",
        "rebuilds/frame",
        "voxel rebuild budget",
        1
    );
}
