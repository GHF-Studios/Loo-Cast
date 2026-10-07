//! Spacetime Engine.
//!
//! ## Integration
//!
//! SpacetimeEnginePlugin installs reusable engine facilities; engine_app supplies the standard Bevy
//! host, and run lets the selected game extend that app before execution.
//!
//! ## Module map
//!
//! - `config`: Typed layered runtime configuration.
//! - `console`: Shared developer command ingress and structured diagnostic transport.
//! - `devtools`: Developer-facing inspection, UI, editor interaction and spatial visualization.
//! - `diagnostics`: Runtime diagnostics data owned independently from developer UI.
//! - `ecs`: Shared ECS identity, manifestation, constituency, and component-conflict facilities.
//! - `game`: Loo Cast game composition built on Spacetime Engine domains.
//! - `geometry`: Hot-reloadable authored geometry asset/compiler/runtime facility.
//! - `gpu`: Reusable GPU-offload coordination and asynchronous completion facilities.
//! - `input_focus`: Generic arbitration for UI/tools that temporarily own pointer or gameplay
//!   input.
//! - `physics`: Physics integration owned by Spacetime Engine.
//! - `portal`: Ordinary geometric portals.
//! - `procedural_assets`: Replaceable runtime-generated presentation assets.
//! - `reconstructible`: Generic pacing for disposable/reconstructible main-thread work.
//! - `spatial`: USF semantic spatial identity projected into bounded local runtime coordinates.
//! - `thermal`: Reusable systemic thermal state, spatial refinement and combustion.
//! - `ui`: Shared presentation policy for ordinary game UI and Developer UI.
//! - `usf`: Canonical Universal Simulation Framework spatial-number algebra.
//! - `view`: Viewport and coordinate-space contracts shared by gameplay and tooling.
//! - `voxel`: Semantic voxel authority, scale realizations, and disposable materialization
//!   backends.
//! - `worldgen`: Sparse typed phenomenon evaluation across canonical USF spatial scopes.
//!
//! The plugin or configure entrypoint here wires the child systems and resources.
//!

//!
//! The engine owns reusable simulation, spatial, physics, rendering-adjacent,
//! diagnostics, developer-tooling, and world-generation infrastructure used by
//! Loo Cast. Game-specific orchestration lives under [`game`]; reusable engine
//! domains should not depend on it unless they are explicit adapters.
//!
//! ECS entities are runtime storage and manifestation machinery. They are not
//! automatically the semantic identity of every simulated value.

pub mod config;
pub mod console;
pub mod devtools;
pub mod diagnostics;
pub mod ecs;
pub mod game;
pub mod geometry;
pub mod gpu;
pub mod input_focus;
pub mod physics;
pub mod portal;
pub mod procedural_assets;
pub mod reconstructible;
pub mod spatial;
pub mod thermal;
pub mod ui;
pub mod usf;
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

/// Installs the reusable Spacetime Engine domains into an existing Bevy app.
///
/// Host/window/logging plugins remain application-host policy; this plugin owns
/// engine facilities and their internal ordering.
pub struct SpacetimeEnginePlugin;

impl bevy::prelude::Plugin for SpacetimeEnginePlugin {
    fn build(&self, app: &mut EngineApp) {
        use bevy::prelude::*;

        app.add_plugins((
            config::EngineConfigPlugin,
            ecs::component_conflict::ComponentConflictPlugin,
            geometry::AuthoredGeometryPlugin,
            physics::SpacetimePhysicsPlugin,
            reconstructible::ReconstructibleWorkPlugin,
            voxel::VoxelPlugin,
            ui::UiFoundationPlugin,
            diagnostics::RuntimeDiagnosticsPlugin,
            devtools::DeveloperToolsPlugin,
            console::DeveloperConsolePlugin,
        ))
        // `PbrPlugin::finish` installs GlobalClusterSettings after plugin build;
        // Startup is deliberately late enough to override that default.
        .add_systems(Startup, configure_sparse_scene_clustering);
    }
}

/// Construct the standard desktop/runtime host with all engine facilities.
///
/// Callers may extend the returned app with games/adapters before running it;
/// they never need to reproduce the engine plugin list.
pub fn engine_app() -> EngineApp {
    use bevy::prelude::*;

    let mut app = App::new();
    app
        .add_plugins(
            DefaultPlugins
                .set(bevy::log::LogPlugin {
                    custom_layer: console::console_log_layer,
                    ..default()
                })
                .set(bevy::window::WindowPlugin {
                    primary_window: Some(bevy::window::Window {
                        present_mode: bevy::window::PresentMode::AutoNoVsync,
                        desired_maximum_frame_latency: std::num::NonZeroU32::new(3),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(SpacetimeEnginePlugin);
    app
}

/// Run one statically composed Spacetime Engine game.
pub fn run(install_game: impl FnOnce(&mut EngineApp)) {
    let mut app = engine_app();
    install_game(&mut app);
    app.run();
}
