extern crate spacetime_engine;

use bevy::{log::LogPlugin, prelude::*};
use bevy_rapier2d::prelude::*;
use spacetime_engine::SpacetimeEnginePlugins;

// Fun tasks
// TODO: Implement inventory + hotbar, so that you can select different types of chunk actors to place. 

// Less fun tasks
// TODO: Implement sub-chunking/fields
// TODO: Implement gravity via sub-chunking/fields
// TODO: Implement electromagnetism via sub-chunking/fields
// TODO: Implement planets via gravity
// TODO: Implement magnets via electromagnetism
// TODO: Implement stars via gravity and electromagnetism

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            filter: "info,spacetime_engine=debug".into(),
            level: bevy::log::Level::INFO,
            ..Default::default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(SpacetimeEnginePlugins)
        .add_systems(PreStartup, pre_start)
        .run();
}

fn pre_start(mut rapier_configuration: ResMut<RapierConfiguration>) {
    rapier_configuration.gravity = Vec2::new(0.0, 0.0);
}