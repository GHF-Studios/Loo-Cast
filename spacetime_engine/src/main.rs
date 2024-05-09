extern crate spacetime_engine;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use spacetime_engine::SpacetimeEnginePlugins;

// TODO: Fix chunk loading by implementing serializable proxies for all necessary rapier components (necessary as of now)
// TODO: Implement sub-chunking/fields
// TODO: Implement gravity via sub-chunking/fields
// TODO: Implement electromagnetism via sub-chunking/fields
// TODO: Implement planets via gravity
// TODO: Implement magnets via electromagnetism
// TODO: Implement stars via gravity and electromagnetism

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(SpacetimeEnginePlugins)
        .add_systems(Startup, startup)
        .run();
}

fn startup(mut rapier_configuration: ResMut<RapierConfiguration>) {
    rapier_configuration.gravity = Vec2::new(0.0, 0.0);
}