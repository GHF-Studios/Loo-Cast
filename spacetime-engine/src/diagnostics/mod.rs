//! Runtime diagnostics data owned independently from developer UI.
//!
//! Diagnostics sample runtime/semantic state and expose typed observations.
//! They do not own developer controls, HUDs, Inspector sections, or World Draw.
//!
//! ## Module map
//!
//! - `model`: Typed runtime snapshots consumed by tooling and telemetry.
//! - `physics`: Bounded Avian runtime diagnostics.
//! - `runtime`: Frame/runtime/system diagnostic sampling and low-frame-rate calculation.
//! - `telemetry`: Vapor telemetry publication for typed runtime/world diagnostic snapshots.
//! - `world`: Low-frequency ECS/world structure and inline-memory sampling.
//!
//! The plugin or configure entrypoint here wires the child systems and resources.
//!

use std::collections::BTreeMap;

use avian3d::prelude::{
    Collider, ContactGraph, PhysicsSchedule, PhysicsStepSystems, RigidBody, Sleeping,
};
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

mod model;
pub use model::{
    ComponentMemoryDiagnostics, EcsMemoryDiagnostics, FrameRuntimeDiagnostics,
    PhysicsRuntimeDiagnostics, RuntimeDiagnostics, SystemRuntimeDiagnostics,
    WorldRuntimeDiagnostics,
};

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

use runtime::sample_runtime_diagnostics;
use telemetry::{publish_runtime_telemetry, publish_world_telemetry};
use world::sample_world_diagnostics;

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
                (physics::finalize_physics_frame, sample_runtime_diagnostics).chain(),
            )
            .add_systems(Last, sample_world_diagnostics);
    }
}
