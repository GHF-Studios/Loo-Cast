extern crate spacetime_engine;

use std::env;

use bevy::{log::LogPlugin, prelude::*};
use bevy_rapier2d::prelude::*;
use spacetime_engine::entity::structs::EntityPosition;
use spacetime_engine::operations::structs::InstanceID;
use spacetime_engine::operations::traits::InstanceRegistryKey;
use spacetime_engine::SpacetimeEnginePlugins;
use spacetime_engine::operations::singletons::*;
use spacetime_engine::entity::operations::*;
use spacetime_engine::chunk::operations::*;
use spacetime_engine::chunk_actor::operations::*;
use spacetime_engine::chunk_loader::operations::*;

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
    env::set_var("RUST_BACKTRACE", "1");
    
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

fn startup() {
    // Create operation A
    let entity_position = EntityPosition(Vec2::new(0.0, 0.0));
    let create_entity_args = CreateEntityArgs { entity_position };
    let create_entity_operation = CreateEntity::new(create_entity_args, Some(|result| {
        if let CreateEntityResult::Ok{ entity_id } = result {
            // Create operation AA
            let upgrade_to_chunk_actor_args = UpgradeToChunkActorArgs {
                    target_entity_id: entity_id,
                    chunk_actor_start_chunk_id: InstanceID::new(1),
            };
            let upgrade_to_chunk_actor_operation = UpgradeToChunkActor::new(upgrade_to_chunk_actor_args, None);

            // Queue operation AA
            let mut operation_queue = OPERATION_QUEUE.lock().unwrap();
            operation_queue.add_operation(Box::new(upgrade_to_chunk_actor_operation));
            drop(operation_queue);
        };
    }));

    // Queue operation A
    let mut operation_queue = OPERATION_QUEUE.lock().unwrap();
    operation_queue.add_operation(Box::new(create_entity_operation));
    drop(operation_queue);
}