use bevy::prelude::*;

use spacetime_engine::{
    debug::SpacetimeDebugPlugin, ecs::component_conflict::ComponentConflictPlugin,
    game::TestGamePlugin,
    geometry::AuthoredGeometryPlugin, physics::SpacetimePhysicsPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            ComponentConflictPlugin,
            AuthoredGeometryPlugin,
            SpacetimePhysicsPlugin,
            TestGamePlugin,
            SpacetimeDebugPlugin,
        ))
        .run();
}
