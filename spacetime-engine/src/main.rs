use bevy::prelude::*;

use spacetime_engine::{
    devtools::DeveloperToolsPlugin,
    ecs::component_conflict::ComponentConflictPlugin,
    game::{TestGameDeveloperToolsPlugin, TestGameObservabilityPlugin, TestGamePlugin},
    geometry::AuthoredGeometryPlugin,
    observability::ObservabilityPlugin,
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
            DeveloperToolsPlugin,
            TestGameDeveloperToolsPlugin,
            ObservabilityPlugin,
            TestGameObservabilityPlugin,
        ))
        .run();
}
