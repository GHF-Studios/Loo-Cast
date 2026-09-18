//! Runtime diagnostics data owned independently from developer UI.
//!
//! Diagnostics sample runtime/world state and expose a typed snapshot resource.
//! They do not own developer controls, HUDs, Inspector sections, or World Draw.

use std::collections::{BTreeMap, HashMap};

use bevy::{
    diagnostic::{
        DiagnosticsStore, FrameTimeDiagnosticsPlugin, SystemInformationDiagnosticsPlugin,
    },
    ecs::{component::ComponentId, resource::IS_RESOURCE},
    prelude::*,
};

use crate::devtools::DeveloperArtifact;
use vapor_telemetry::TelemetryEmitter;

const RUNTIME_SAMPLE_INTERVAL_SECONDS: f32 = 0.25;
const WORLD_SAMPLE_INTERVAL_SECONDS: f32 = 5.0;
const FRAME_HISTORY_LENGTH: usize = 600;
const TOP_COMPONENT_MEMORY_ENTRIES: usize = 24;

#[derive(Debug, Default, Clone, Copy)]
pub struct FrameRuntimeDiagnostics {
    pub fps: Option<f64>,
    pub frame_time_ms: Option<f64>,
    pub average_frame_time_ms: Option<f64>,
    pub one_percent_low_fps: Option<f64>,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct WorldRuntimeDiagnostics {
    pub entities: usize,
    pub component_instances: usize,
    pub archetypes: usize,
}

/// Logical live ECS payload for one component type.
///
/// `inline_bytes` is the component's registered in-ECS layout size multiplied by
/// its live instance count. It deliberately does not attempt to include heap-owned
/// allocations inside values such as `Vec`, `String`, maps, or custom allocators.
#[derive(Debug, Clone)]
pub struct ComponentMemoryDiagnostics {
    pub name: String,
    pub instances: usize,
    pub inline_size_bytes: usize,
    pub inline_bytes: usize,
}

#[derive(Resource, Debug, Default, Clone)]
pub struct EcsMemoryDiagnostics {
    pub inline_component_bytes: usize,
    pub largest_components: Vec<ComponentMemoryDiagnostics>,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct SystemRuntimeDiagnostics {
    pub process_cpu_percent: Option<f64>,
    pub system_cpu_percent: Option<f64>,
    pub process_memory_gib: Option<f64>,
    pub system_memory_percent: Option<f64>,
}

/// Latest low-frequency runtime diagnostics snapshot.
///
/// The snapshot is intentionally typed rather than keyed by arbitrary strings.
/// Consumers such as a future diagnostics panel, logging, tests, or Vapor tooling
/// can read it without depending on developer UI or parsing formatted text.
#[derive(Resource, Debug, Default, Clone, Copy)]
pub struct RuntimeDiagnostics {
    pub frame: FrameRuntimeDiagnostics,
    pub world: WorldRuntimeDiagnostics,
    pub system: SystemRuntimeDiagnostics,
}

#[derive(Resource, Debug, Default)]
struct DiagnosticsCadence {
    runtime_elapsed: f32,
    world_elapsed: f32,
}

#[derive(Resource)]
struct VaporTelemetry(TelemetryEmitter);

pub struct RuntimeDiagnosticsPlugin;

impl Plugin for RuntimeDiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        if let Some(telemetry) = TelemetryEmitter::from_env("spacetime-engine") {
            app.insert_resource(VaporTelemetry(telemetry));
        }

        app.init_resource::<RuntimeDiagnostics>()
            .init_resource::<EcsMemoryDiagnostics>()
            .init_resource::<DiagnosticsCadence>()
            .add_plugins((
                FrameTimeDiagnosticsPlugin::new(FRAME_HISTORY_LENGTH),
                SystemInformationDiagnosticsPlugin,
            ))
            .add_systems(Update, collect_runtime_diagnostics)
            .add_systems(Last, collect_world_diagnostics);
    }
}

fn collect_runtime_diagnostics(
    time: Res<Time>,
    diagnostics: Res<DiagnosticsStore>,
    mut cadence: ResMut<DiagnosticsCadence>,
    mut snapshot: ResMut<RuntimeDiagnostics>,
    telemetry: Option<Res<VaporTelemetry>>,
) {
    cadence.runtime_elapsed += time.delta_secs();
    if cadence.runtime_elapsed < RUNTIME_SAMPLE_INTERVAL_SECONDS {
        return;
    }
    cadence.runtime_elapsed %= RUNTIME_SAMPLE_INTERVAL_SECONDS;

    let frame_time = diagnostics.get(&FrameTimeDiagnosticsPlugin::FRAME_TIME);
    snapshot.frame.fps = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|diagnostic| diagnostic.value());
    snapshot.frame.frame_time_ms = frame_time.and_then(|diagnostic| diagnostic.value());
    snapshot.frame.average_frame_time_ms = frame_time.and_then(|diagnostic| diagnostic.average());

    let mut frame_times = frame_time
        .into_iter()
        .flat_map(|diagnostic| diagnostic.values().copied())
        .filter(|value| value.is_finite() && *value > 0.0)
        .collect::<Vec<_>>();
    snapshot.frame.one_percent_low_fps = low_fps(&mut frame_times, 0.01);

    snapshot.system.process_cpu_percent = diagnostic_value(
        &diagnostics,
        &SystemInformationDiagnosticsPlugin::PROCESS_CPU_USAGE,
    );
    snapshot.system.system_cpu_percent = diagnostic_value(
        &diagnostics,
        &SystemInformationDiagnosticsPlugin::SYSTEM_CPU_USAGE,
    );
    snapshot.system.process_memory_gib = diagnostic_value(
        &diagnostics,
        &SystemInformationDiagnosticsPlugin::PROCESS_MEM_USAGE,
    );
    snapshot.system.system_memory_percent = diagnostic_value(
        &diagnostics,
        &SystemInformationDiagnosticsPlugin::SYSTEM_MEM_USAGE,
    );

    if let Some(telemetry) = telemetry {
        publish_runtime_telemetry(&telemetry, &snapshot);
    }
}

fn collect_world_diagnostics(world: &mut World) {
    let dt = world.resource::<Time>().delta_secs();
    {
        let mut cadence = world.resource_mut::<DiagnosticsCadence>();
        cadence.world_elapsed += dt;
        if cadence.world_elapsed < WORLD_SAMPLE_INTERVAL_SECONDS {
            return;
        }
        cadence.world_elapsed %= WORLD_SAMPLE_INTERVAL_SECONDS;
    }

    let developer_artifact = world.components().component_id::<DeveloperArtifact>();
    let mut result = WorldRuntimeDiagnostics::default();
    let mut component_instances = vec![0usize; world.components().len()];

    for archetype in world.archetypes().iter() {
        if archetype.contains(IS_RESOURCE)
            || developer_artifact.is_some_and(|id| archetype.contains(id))
            || archetype.is_empty()
        {
            continue;
        }

        let entity_count = archetype.len() as usize;
        result.archetypes += 1;
        result.entities += entity_count;
        result.component_instances += entity_count * archetype.component_count();

        for component_id in archetype.iter_components() {
            component_instances[component_id.index()] += entity_count;
        }
    }

    world.resource_mut::<RuntimeDiagnostics>().world = result;

    let mut inline_component_bytes = 0usize;
    let mut largest_components = component_instances
        .into_iter()
        .enumerate()
        .filter_map(|(index, instances)| {
            if instances == 0 {
                return None;
            }
            let info = world.components().get_info(ComponentId::new(index))?;
            let inline_size_bytes = info.layout().size();
            let inline_bytes = inline_size_bytes.saturating_mul(instances);
            inline_component_bytes = inline_component_bytes.saturating_add(inline_bytes);

            Some(ComponentMemoryDiagnostics {
                name: info.name().to_string(),
                instances,
                inline_size_bytes,
                inline_bytes,
            })
        })
        .collect::<Vec<_>>();

    largest_components.sort_by(|a, b| {
        b.inline_bytes
            .cmp(&a.inline_bytes)
            .then_with(|| b.instances.cmp(&a.instances))
            .then_with(|| a.name.cmp(&b.name))
    });
    largest_components.truncate(TOP_COMPONENT_MEMORY_ENTRIES);

    *world.resource_mut::<EcsMemoryDiagnostics>() = EcsMemoryDiagnostics {
        inline_component_bytes,
        largest_components,
    };

    publish_world_telemetry(world);
}

fn publish_runtime_telemetry(telemetry: &VaporTelemetry, runtime: &RuntimeDiagnostics) {
    let mut metrics = BTreeMap::new();
    insert_metric(&mut metrics, "frame.fps", runtime.frame.fps);
    insert_metric(&mut metrics, "frame.time-ms", runtime.frame.frame_time_ms);
    insert_metric(
        &mut metrics,
        "frame.average-time-ms",
        runtime.frame.average_frame_time_ms,
    );
    insert_metric(
        &mut metrics,
        "frame.one-percent-low-fps",
        runtime.frame.one_percent_low_fps,
    );
    insert_metric(
        &mut metrics,
        "system.process-cpu-percent",
        runtime.system.process_cpu_percent,
    );
    insert_metric(
        &mut metrics,
        "system.cpu-percent",
        runtime.system.system_cpu_percent,
    );
    insert_metric(
        &mut metrics,
        "system.process-memory-gib",
        runtime.system.process_memory_gib,
    );
    insert_metric(
        &mut metrics,
        "system.memory-percent",
        runtime.system.system_memory_percent,
    );
    telemetry.0.publish_metrics(metrics);
}

fn publish_world_telemetry(world: &World) {
    let Some(telemetry) = world.get_resource::<VaporTelemetry>() else {
        return;
    };
    let runtime = world.resource::<RuntimeDiagnostics>();
    let memory = world.resource::<EcsMemoryDiagnostics>();
    let mut metrics = BTreeMap::new();

    metrics.insert("world.entities".to_owned(), runtime.world.entities as f64);
    metrics.insert(
        "world.component-instances".to_owned(),
        runtime.world.component_instances as f64,
    );
    metrics.insert(
        "world.archetypes".to_owned(),
        runtime.world.archetypes as f64,
    );
    metrics.insert(
        "ecs.inline-component-bytes".to_owned(),
        memory.inline_component_bytes as f64,
    );
    telemetry.0.publish_metrics(metrics);

    let components = memory
        .largest_components
        .iter()
        .map(|component| {
            serde_json::json!({
                "name": &component.name,
                "instances": component.instances,
                "inline_size_bytes": component.inline_size_bytes,
                "inline_bytes": component.inline_bytes,
            })
        })
        .collect();
    telemetry
        .0
        .publish_snapshot("ecs.component-memory", serde_json::Value::Array(components));
}

fn insert_metric(metrics: &mut BTreeMap<String, f64>, name: &str, value: Option<f64>) {
    if let Some(value) = value {
        metrics.insert(name.to_owned(), value);
    }
}

fn diagnostic_value(
    diagnostics: &DiagnosticsStore,
    path: &bevy::diagnostic::DiagnosticPath,
) -> Option<f64> {
    diagnostics
        .get(path)
        .and_then(|diagnostic| diagnostic.value())
}

fn low_fps(frame_times: &mut [f64], fraction: f64) -> Option<f64> {
    if frame_times.is_empty() {
        return None;
    }

    let count = ((frame_times.len() as f64 * fraction).ceil() as usize).clamp(1, frame_times.len());
    if count < frame_times.len() {
        frame_times.select_nth_unstable_by(count - 1, |left, right| right.total_cmp(left));
    }
    let average_ms = frame_times[..count].iter().sum::<f64>() / count as f64;

    (average_ms > 0.0).then_some(1_000.0 / average_ms)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_percent_low_uses_slowest_frame_times() {
        let mut frame_times = vec![10.0; 99];
        frame_times.push(50.0);
        assert_eq!(low_fps(&mut frame_times, 0.01), Some(20.0));
    }
}
