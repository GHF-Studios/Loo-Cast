//! Runtime diagnostics data owned independently from developer UI.
//!
//! Diagnostics sample runtime/world state and expose a typed snapshot resource.
//! They do not own developer controls, HUDs, Inspector sections, or World Draw.

use bevy::{
    diagnostic::{
        DiagnosticsStore, FrameTimeDiagnosticsPlugin, SystemInformationDiagnosticsPlugin,
    },
    ecs::resource::IS_RESOURCE,
    prelude::*,
};

use crate::devtools::DeveloperArtifact;

const SAMPLE_INTERVAL_SECONDS: f32 = 1.0;
const FRAME_HISTORY_LENGTH: usize = 600;

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

pub struct RuntimeDiagnosticsPlugin;

impl Plugin for RuntimeDiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RuntimeDiagnostics>()
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
) {
    cadence.runtime_elapsed += time.delta_secs();
    if cadence.runtime_elapsed < SAMPLE_INTERVAL_SECONDS {
        return;
    }
    cadence.runtime_elapsed %= SAMPLE_INTERVAL_SECONDS;

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
    frame_times.sort_by(|a, b| b.total_cmp(a));
    snapshot.frame.one_percent_low_fps = low_fps_sorted(&frame_times, 0.01);

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
}

fn collect_world_diagnostics(world: &mut World) {
    let dt = world.resource::<Time>().delta_secs();
    {
        let mut cadence = world.resource_mut::<DiagnosticsCadence>();
        cadence.world_elapsed += dt;
        if cadence.world_elapsed < SAMPLE_INTERVAL_SECONDS {
            return;
        }
        cadence.world_elapsed %= SAMPLE_INTERVAL_SECONDS;
    }

    let developer_artifact = world.components().component_id::<DeveloperArtifact>();
    let mut result = WorldRuntimeDiagnostics::default();

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
        result.component_instances += entity_count * archetype.components().iter().count();
    }

    world.resource_mut::<RuntimeDiagnostics>().world = result;
}

fn diagnostic_value(
    diagnostics: &DiagnosticsStore,
    path: &bevy::diagnostic::DiagnosticPath,
) -> Option<f64> {
    diagnostics.get(path).and_then(|diagnostic| diagnostic.value())
}

fn low_fps_sorted(frame_times_descending: &[f64], fraction: f64) -> Option<f64> {
    if frame_times_descending.is_empty() {
        return None;
    }

    let count = ((frame_times_descending.len() as f64 * fraction).ceil() as usize)
        .clamp(1, frame_times_descending.len());
    let average_ms =
        frame_times_descending[..count].iter().sum::<f64>() / count as f64;

    (average_ms > 0.0).then_some(1_000.0 / average_ms)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_percent_low_uses_slowest_frame_times() {
        let mut frame_times = vec![10.0; 99];
        frame_times.push(50.0);
        frame_times.sort_by(|a, b| b.partial_cmp(a).unwrap());

        assert_eq!(low_fps_sorted(&frame_times, 0.01), Some(20.0));
    }
}
