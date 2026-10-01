//! Spacetime Engine.
//!
//! The engine owns reusable simulation, spatial, physics, rendering-adjacent,
//! diagnostics, developer-tooling, and world-generation infrastructure used by
//! Loo Cast. Game-specific orchestration lives under [`game`]; reusable engine
//! domains should not depend on it unless they are explicit adapters.
//!
//! ECS entities are runtime storage and manifestation machinery. They are not
//! automatically the semantic identity of every simulated value.

pub mod console;
pub mod config;
pub mod devtools;
pub mod diagnostics;
pub mod ecs;
pub mod game;
pub mod geometry;
pub mod input_focus;
pub mod physics;
pub mod portal;
pub mod procedural_assets;
pub mod usf;
pub mod spatial;
pub mod thermal;
pub mod ui;
pub mod view;
pub mod voxel;
pub mod worldgen;

pub use spacetime_engine_macros::{Inspect, conflict};

/// Concrete application type exposed by Spacetime Engine's current Rust host
/// contract.
///
/// Vapor resolves semantic Engine/Game composition; this type remains
/// Engine-owned implementation vocabulary rather than a universal Vapor ABI.
pub type EngineApp = bevy::prelude::App;

/// Current Loo-Cast scenes have very few clusterable lights but still paid
/// Bevy 0.19's multi-pass GPU clustering preparation every frame. Prefer the
/// supported CPU clustering path until scene light cardinality justifies the
/// GPU path again.
fn configure_sparse_scene_clustering(
    mut settings: bevy::prelude::ResMut<bevy::light::cluster::GlobalClusterSettings>,
) {
    settings.gpu_clustering = None;
}

/// Run one statically composed Spacetime Engine game.
///
/// Engine infrastructure is installed first, then the selected Game is allowed
/// to extend the fully established Engine application.
pub fn run(install_game: impl FnOnce(&mut EngineApp)) {
    use bevy::prelude::*;

    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(bevy::log::LogPlugin {
        custom_layer: console::console_log_layer,
        ..default()
    }))
        .add_plugins((
            config::EngineConfigPlugin,
            ecs::component_conflict::ComponentConflictPlugin,
            geometry::AuthoredGeometryPlugin,
            physics::SpacetimePhysicsPlugin,
            voxel::VoxelPlugin,
            ui::UiFoundationPlugin,
            diagnostics::RuntimeDiagnosticsPlugin,
            devtools::DeveloperToolsPlugin,
            console::DeveloperConsolePlugin,
        ));

    install_game(&mut app);

    // `PbrPlugin::finish` installs GlobalClusterSettings after plugin build;
    // Startup is deliberately late enough to override that default.
    app.add_systems(Startup, configure_sparse_scene_clustering);

    app.run();
}
