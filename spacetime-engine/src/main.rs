use bevy::prelude::*;

use spacetime_engine::{
    devtools::DeveloperToolsPlugin,
    diagnostics::RuntimeDiagnosticsPlugin,
    ecs::component_conflict::ComponentConflictPlugin,
    game::{TestGameDeveloperToolsPlugin, TestGamePlugin},
    geometry::AuthoredGeometryPlugin,
    physics::SpacetimePhysicsPlugin,
    ui::UiFoundationPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            ComponentConflictPlugin,
            AuthoredGeometryPlugin,
            SpacetimePhysicsPlugin,
            TestGamePlugin,
            UiFoundationPlugin,
            RuntimeDiagnosticsPlugin,
            DeveloperToolsPlugin,
            TestGameDeveloperToolsPlugin,
        ))
        .run();
}
