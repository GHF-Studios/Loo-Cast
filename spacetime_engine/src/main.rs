extern crate spacetime_engine;

use bevy::prelude::*;
use bevy::log::LogPlugin;
use bevy_rapier2d::prelude::*;
use spacetime_engine::chunk::structs::ChunkPosition;
use spacetime_engine::entity::structs::EntityPosition;
use spacetime_engine::math::structs::I16Vec2;
use spacetime_engine::operations::singletons::TOKIO_RUNTIME;
use spacetime_engine::{entity, SpacetimeEnginePlugins};
use spacetime_engine::chunk::commands::*;
use spacetime_engine::chunk_actor::commands::*;

// Primary tasks
// TODO: Implement chunk loaders

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
    //env::set_var("RUST_BACKTRACE", "1");
    
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            filter: "info,spacetime_engine=debug".into(),
            level: bevy::log::Level::INFO,
            ..Default::default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(SpacetimeEnginePlugins)
        .add_systems(PreStartup, pre_startup)
        .add_systems(Startup, startup)
        .run();
}

fn pre_startup(mut rapier_configuration: ResMut<RapierConfiguration>) {
    rapier_configuration.gravity = Vec2::new(0.0, 0.0);
}

fn startup(mut commands: Commands) {
    let runtime = TOKIO_RUNTIME.lock().unwrap();

    runtime.spawn(async {
        spacetime_engine::core::commands::startup().await;
    });

    // spawn a camera
    // TODO: Make this more permanent
    commands.spawn(Camera2dBundle::default());
}
