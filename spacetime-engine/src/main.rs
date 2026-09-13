use bevy::prelude::*;

use spacetime_engine::{
    ecs::component_conflict::ComponentConflictPlugin,
    game::{TestGameObservabilityPlugin, TestGamePlugin},
    geometry::AuthoredGeometryPlugin,
    observability::ObservabilityPlugin,
    physics::SpacetimePhysicsPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            ComponentConflictPlugin,
            AuthoredGeometryPlugin,
            SpacetimePhysicsPlugin,
            TestGamePlugin,
            ObservabilityPlugin,
            TestGameObservabilityPlugin,
        ))
        .run();
}
