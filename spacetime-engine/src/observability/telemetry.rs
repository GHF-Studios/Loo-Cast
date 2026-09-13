//! Temporary data-only telemetry provider.
//!
//! Stage 4 deliberately removes the old performance HUD and its generic control
//! graph. Metrics continue sampling at a conservative fixed cadence so Stage 5
//! can move the data model into `diagnostics` without another behavior dependency.

use std::collections::{HashMap, HashSet};

use bevy::{
    diagnostic::{
        DiagnosticsStore, FrameTimeDiagnosticsPlugin, SystemInformationDiagnosticsPlugin,
    },
    ecs::resource::IS_RESOURCE,
    prelude::*,
};

use super::{DebugArtifact, DebugId};

const TELEMETRY_REFRESH_SECONDS: f32 = 1.0;

const FPS: DebugId = DebugId("metric.frame.fps");
const FRAME_MS: DebugId = DebugId("metric.frame.ms");
const FRAME_MIN_MS: DebugId = DebugId("metric.frame.min_ms");
const FRAME_AVG_MS: DebugId = DebugId("metric.frame.avg_ms");
const FRAME_MAX_MS: DebugId = DebugId("metric.frame.max_ms");
const FPS_1_LOW: DebugId = DebugId("metric.frame.fps_1_low");
const FPS_POINT_1_LOW: DebugId = DebugId("metric.frame.fps_0_1_low");

const ENTITY_COUNT: DebugId = DebugId("metric.ecs.entities");
const COMPONENT_INSTANCES: DebugId = DebugId("metric.ecs.component_instances");
const COMPONENT_TYPES: DebugId = DebugId("metric.ecs.component_types");
const RESOURCE_COUNT: DebugId = DebugId("metric.ecs.resources");
const ARCHETYPE_COUNT: DebugId = DebugId("metric.ecs.archetypes");

const PROCESS_CPU: DebugId = DebugId("metric.system.process_cpu");
const SYSTEM_CPU: DebugId = DebugId("metric.system.cpu");
const PROCESS_RAM: DebugId = DebugId("metric.system.process_ram_gib");
const SYSTEM_RAM: DebugId = DebugId("metric.system.ram_percent");

#[derive(Debug, Clone, Copy)]
pub enum MetricUnit {
    Fps,
    Milliseconds,
    Count,
    Percent,
    Gibibytes,
}

#[derive(Debug, Clone)]
pub struct DebugMetricSpec {
    pub id: DebugId,
    pub label: &'static str,
    pub unit: MetricUnit,
}

#[derive(Resource, Debug, Default)]
pub struct DebugMetrics {
    specs: HashMap<DebugId, DebugMetricSpec>,
    values: HashMap<DebugId, f64>,
}

impl DebugMetrics {
    pub fn register(&mut self, spec: DebugMetricSpec) {
        self.specs.entry(spec.id).or_insert(spec);
    }

    pub fn set(&mut self, id: DebugId, value: f64) {
        if value.is_finite() {
            self.values.insert(id, value);
        }
    }

    pub fn value(&self, id: DebugId) -> Option<f64> {
        self.values.get(&id).copied()
    }
}

#[derive(Resource, Debug, Default)]
struct TelemetryCadence {
    derived_elapsed: f32,
    ecs_elapsed: f32,
}

pub(super) fn configure(app: &mut App) {
    app.init_resource::<DebugMetrics>()
        .init_resource::<TelemetryCadence>()
        .add_plugins((
            FrameTimeDiagnosticsPlugin::new(3_600),
            SystemInformationDiagnosticsPlugin,
        ))
        .add_systems(Startup, register_metrics)
        .add_systems(Update, collect_bevy_metrics)
        .add_systems(Last, collect_ecs_metrics);
}

fn register_metrics(mut metrics: ResMut<DebugMetrics>) {
    for spec in [
        DebugMetricSpec { id: FPS, label: "FPS", unit: MetricUnit::Fps },
        DebugMetricSpec { id: FRAME_MS, label: "Frame time", unit: MetricUnit::Milliseconds },
        DebugMetricSpec { id: FRAME_MIN_MS, label: "Frame min", unit: MetricUnit::Milliseconds },
        DebugMetricSpec { id: FRAME_AVG_MS, label: "Frame avg", unit: MetricUnit::Milliseconds },
        DebugMetricSpec { id: FRAME_MAX_MS, label: "Frame max", unit: MetricUnit::Milliseconds },
        DebugMetricSpec { id: FPS_1_LOW, label: "FPS 1% low", unit: MetricUnit::Fps },
        DebugMetricSpec { id: FPS_POINT_1_LOW, label: "FPS 0.1% low", unit: MetricUnit::Fps },
        DebugMetricSpec { id: ENTITY_COUNT, label: "Entities", unit: MetricUnit::Count },
        DebugMetricSpec { id: COMPONENT_INSTANCES, label: "Component instances", unit: MetricUnit::Count },
        DebugMetricSpec { id: COMPONENT_TYPES, label: "Component types", unit: MetricUnit::Count },
        DebugMetricSpec { id: RESOURCE_COUNT, label: "Resources", unit: MetricUnit::Count },
        DebugMetricSpec { id: ARCHETYPE_COUNT, label: "Archetypes", unit: MetricUnit::Count },
        DebugMetricSpec { id: PROCESS_CPU, label: "Process CPU", unit: MetricUnit::Percent },
        DebugMetricSpec { id: SYSTEM_CPU, label: "System CPU", unit: MetricUnit::Percent },
        DebugMetricSpec { id: PROCESS_RAM, label: "Process RAM", unit: MetricUnit::Gibibytes },
        DebugMetricSpec { id: SYSTEM_RAM, label: "System RAM", unit: MetricUnit::Percent },
    ] {
        metrics.register(spec);
    }
}

fn collect_bevy_metrics(
    time: Res<Time>,
    diagnostics: Res<DiagnosticsStore>,
    mut cadence: ResMut<TelemetryCadence>,
    mut metrics: ResMut<DebugMetrics>,
) {
    cadence.derived_elapsed += time.delta_secs();
    if cadence.derived_elapsed < TELEMETRY_REFRESH_SECONDS {
        return;
    }
    cadence.derived_elapsed %= TELEMETRY_REFRESH_SECONDS;

    let frame_time = diagnostics.get(&FrameTimeDiagnosticsPlugin::FRAME_TIME);
    if let Some(value) = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|diagnostic| diagnostic.value())
    {
        metrics.set(FPS, value);
    }
    if let Some(value) = frame_time.and_then(|diagnostic| diagnostic.value()) {
        metrics.set(FRAME_MS, value);
    }
    if let Some(value) = frame_time.and_then(|diagnostic| diagnostic.average()) {
        metrics.set(FRAME_AVG_MS, value);
    }

    let mut frame_times = frame_time
        .into_iter()
        .flat_map(|diagnostic| diagnostic.values().copied())
        .filter(|value| value.is_finite() && *value > 0.0)
        .collect::<Vec<_>>();

    if let Some(value) = frame_times.iter().copied().reduce(f64::min) {
        metrics.set(FRAME_MIN_MS, value);
    }
    if let Some(value) = frame_times.iter().copied().reduce(f64::max) {
        metrics.set(FRAME_MAX_MS, value);
    }
    if !frame_times.is_empty() {
        frame_times.sort_by(|a, b| b.total_cmp(a));
        if let Some(value) = low_fps_sorted(&frame_times, 0.01) {
            metrics.set(FPS_1_LOW, value);
        }
        if let Some(value) = low_fps_sorted(&frame_times, 0.001) {
            metrics.set(FPS_POINT_1_LOW, value);
        }
    }

    for (id, path) in [
        (PROCESS_CPU, &SystemInformationDiagnosticsPlugin::PROCESS_CPU_USAGE),
        (SYSTEM_CPU, &SystemInformationDiagnosticsPlugin::SYSTEM_CPU_USAGE),
        (PROCESS_RAM, &SystemInformationDiagnosticsPlugin::PROCESS_MEM_USAGE),
        (SYSTEM_RAM, &SystemInformationDiagnosticsPlugin::SYSTEM_MEM_USAGE),
    ] {
        if let Some(value) = diagnostics.get(path).and_then(|diagnostic| diagnostic.value()) {
            metrics.set(id, value);
        }
    }
}

fn collect_ecs_metrics(world: &mut World) {
    let dt = world.resource::<Time>().delta_secs();
    {
        let mut cadence = world.resource_mut::<TelemetryCadence>();
        cadence.ecs_elapsed += dt;
        if cadence.ecs_elapsed < TELEMETRY_REFRESH_SECONDS {
            return;
        }
        cadence.ecs_elapsed %= TELEMETRY_REFRESH_SECONDS;
    }

    let debug_artifact = world.components().component_id::<DebugArtifact>();
    let resource_ids = world
        .iter_resources()
        .map(|(info, _)| info.id())
        .collect::<HashSet<_>>();

    let mut entities = 0usize;
    let mut component_instances = 0usize;
    let mut archetype_count = 0usize;

    for archetype in world.archetypes().iter() {
        if archetype.contains(IS_RESOURCE)
            || debug_artifact.is_some_and(|id| archetype.contains(id))
        {
            continue;
        }
        if archetype.is_empty() {
            continue;
        }

        archetype_count += 1;
        entities += archetype.len() as usize;
        let component_count = archetype
            .components()
            .iter()
            .filter(|component| !resource_ids.contains(component))
            .count();
        component_instances += archetype.len() as usize * component_count;
    }

    let component_types = world
        .components()
        .iter_registered()
        .filter(|info| {
            info.id() != IS_RESOURCE
                && !resource_ids.contains(&info.id())
                && !info.name().starts_with("spacetime_engine::observability::")
        })
        .count();

    let resource_count = world
        .iter_resources()
        .filter(|(info, _)| !info.name().starts_with("spacetime_engine::observability::"))
        .count();

    let mut metrics = world.resource_mut::<DebugMetrics>();
    metrics.set(ENTITY_COUNT, entities as f64);
    metrics.set(COMPONENT_INSTANCES, component_instances as f64);
    metrics.set(COMPONENT_TYPES, component_types as f64);
    metrics.set(RESOURCE_COUNT, resource_count as f64);
    metrics.set(ARCHETYPE_COUNT, archetype_count as f64);
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
