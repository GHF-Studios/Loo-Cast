//! Runtime diagnostics data owned independently from developer UI.
//!
//! Diagnostics sample runtime/world state and expose a typed snapshot resource.
//! They do not own developer controls, HUDs, Inspector sections, or World Draw.

use std::collections::BTreeMap;

use avian3d::prelude::{Collider, ContactGraph, PhysicsSchedule, PhysicsStepSystems, RigidBody, Sleeping};
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

#[derive(Debug, Default, Clone, Copy)]
pub struct PhysicsRuntimeDiagnostics {
    pub physics_steps_window: u64,
    pub render_frames_window: u64,
    pub steps_per_frame_last: u32,
    pub steps_per_frame_max: u32,
    pub steps_per_frame_average: f64,

    pub broad_phase_step_average_ms: f64,
    pub broad_phase_step_max_ms: f64,
    pub broad_phase_step_last_ms: f64,
    pub broad_phase_frame_average_ms: f64,
    pub broad_phase_frame_max_ms: f64,
    pub broad_phase_frame_last_ms: f64,

    pub narrow_phase_step_average_ms: f64,
    pub narrow_phase_step_max_ms: f64,
    pub narrow_phase_step_last_ms: f64,
    pub narrow_phase_frame_average_ms: f64,
    pub narrow_phase_frame_max_ms: f64,
    pub narrow_phase_frame_last_ms: f64,

    pub contact_count_step_average: f64,
    pub contact_count_step_max: u32,
    pub contact_count_step_last: u32,

    pub solver_step_average_ms: f64,
    pub solver_step_max_ms: f64,
    pub solver_step_last_ms: f64,
    pub solver_constraint_count_step_average: f64,
    pub solver_constraint_count_step_max: u32,
    pub solver_constraint_count_step_last: u32,

    pub active_contact_pairs: usize,
    pub active_touching_pairs: usize,
    pub sleeping_contact_pairs: usize,
    pub sleeping_touching_pairs: usize,

    pub dynamic_bodies: usize,
    pub kinematic_bodies: usize,
    pub static_bodies: usize,
    pub sleeping_bodies: usize,
    pub collider_count: usize,
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
    pub physics: PhysicsRuntimeDiagnostics,
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

mod physics;
mod runtime;
mod telemetry;
mod world;

use runtime::collect_runtime_diagnostics;
use telemetry::{publish_runtime_telemetry, publish_world_telemetry};
use world::collect_world_diagnostics;

impl Plugin for RuntimeDiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        if let Some(telemetry) = TelemetryEmitter::from_env("spacetime-engine") {
            app.insert_resource(VaporTelemetry(telemetry));
        }

        app.init_resource::<RuntimeDiagnostics>()
            .init_resource::<EcsMemoryDiagnostics>()
            .init_resource::<DiagnosticsCadence>()
            .init_resource::<physics::PhysicsDiagnosticsAccumulator>()
            .add_plugins((
                FrameTimeDiagnosticsPlugin::new(FRAME_HISTORY_LENGTH),
                SystemInformationDiagnosticsPlugin,
            ))
            .add_systems(
                PhysicsSchedule,
                physics::record_physics_step.after(PhysicsStepSystems::Last),
            )
            .add_systems(
                Update,
                (
                    physics::finalize_physics_frame,
                    collect_runtime_diagnostics,
                )
                    .chain(),
            )
            .add_systems(Last, collect_world_diagnostics);
    }
}

#[cfg(test)]
mod tests;
