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

// Primary tasks
// TODO: Fix bugs, try out different operation chainings
// TODO: Make operations async and use futures, yk?
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

fn new_new_startup() {

}

//fn new_startup() {
//    // Create operation 'A' and execute it synchronously
//    let entity_position = EntityPosition(Vec2::new(0.0, 0.0));
//    let create_entity_args = CreateEntityArgs { entity_position };
//    let create_entity_operation = CreateEntity::new(create_entity_args);
//
//    // Execute operation 'A' synchronously and get the result
//    let create_entity_result = execute_operation_sync(Box::new(create_entity_operation));
//
//    match create_entity_result {
//        CreateEntityResult::Ok { entity_id } => {
//            warn!("Successfully created entity '{}'!", entity_id);
//
//            // Create operation 'B' using the result from 'A'
//            let upgrade_to_chunk_actor_args = UpgradeToChunkActorArgs {
//                target_entity_id: entity_id,
//                chunk_actor_start_chunk_id: InstanceID::new(1),
//            };
//            let upgrade_to_chunk_actor_operation = UpgradeToChunkActor::new(upgrade_to_chunk_actor_args);
//
//            // Execute operation 'B' synchronously and get the result
//            let upgrade_result = execute_operation_sync(Box::new(upgrade_to_chunk_actor_operation));
//
//            match upgrade_result {
//                UpgradeToChunkActorResult::Ok { chunk_actor_id } => {
//                    warn!("Successfully upgraded to chunk actor '{}'!", chunk_actor_id);
//                }
//                _ => {
//                    warn!("Failed to upgrade entity to chunk actor!");
//                }
//            }
//        }
//        _ => {
//            warn!("Failed to create entity!");
//        }
//    }
//}

fn startup() {
    // Create operation 'A'
    let entity_position = EntityPosition(Vec2::new(0.0, 0.0));
    let create_entity_args = CreateEntityArgs { entity_position };
    let create_entity_operation = CreateEntity::new(create_entity_args, Some(|result| {
        // Check operation 'A' result
        if let CreateEntityResult::Ok{ entity_id } = result {
            // TODO: Remove this line
            warn!("Successfully created entity '{}'!", entity_id);

            // Create operation 'B'
            let upgrade_to_chunk_actor_args = UpgradeToChunkActorArgs {
                    target_entity_id: entity_id,
                    chunk_actor_start_chunk_id: InstanceID::new(1),
            };
            let upgrade_to_chunk_actor_operation = UpgradeToChunkActor::new(upgrade_to_chunk_actor_args, Some(|result| {
                // Check operation 'B' result
                if let UpgradeToChunkActorResult::Ok{ chunk_actor_id } = result {
                    // TODO: Remove this line
                    warn!("Successfully upgraded to chunk actor '{}'!", chunk_actor_id);
                } else {
                    // TODO: Remove this line
                    warn!("Failed to upgrade entity to chunk actor!");
                }
            }));

            // TODO: Remove this line
            warn!("Created chunk actor operation!");

            // Queue operation 'B'
            let mut operation_queue = OPERATION_QUEUE.lock().unwrap();
            operation_queue.add_operation(Box::new(upgrade_to_chunk_actor_operation));
            drop(operation_queue);

            // TODO: Remove this line
            warn!("Queued chunk actor operation!");
        } else {
            // TODO: Remove this line
            warn!("Failed to create entity!");
        }
    }));

    // TODO: Remove this line
    warn!("Created entity operation!");

    // Queue operation 'A'
    let mut operation_queue = OPERATION_QUEUE.lock().unwrap();
    operation_queue.add_operation(Box::new(create_entity_operation));
    drop(operation_queue);

    // TODO: Remove this line
    warn!("Queued entity operation!");
}