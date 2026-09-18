use bevy::prelude::*;

use spacetime_engine::{
    config::EngineConfigPlugin,
    devtools::DeveloperToolsPlugin,
    diagnostics::RuntimeDiagnosticsPlugin,
    ecs::component_conflict::ComponentConflictPlugin,
    game::{TestGameDeveloperToolsPlugin, TestGamePlugin},
    geometry::AuthoredGeometryPlugin,
    physics::SpacetimePhysicsPlugin,
    ui::UiFoundationPlugin,
    voxel::VoxelPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            EngineConfigPlugin,
            ComponentConflictPlugin,
            AuthoredGeometryPlugin,
            SpacetimePhysicsPlugin,
            VoxelPlugin,
            TestGamePlugin,
            UiFoundationPlugin,
            RuntimeDiagnosticsPlugin,
            DeveloperToolsPlugin,
            TestGameDeveloperToolsPlugin,
        ))
        .run();
}
