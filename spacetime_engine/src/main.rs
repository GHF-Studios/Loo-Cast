extern crate spacetime_engine;

use std::collections::HashMap;
use std::env;

use bevy::{log::LogPlugin, prelude::*};
use bevy_rapier2d::prelude::*;
use spacetime_engine::chunk::structs::ChunkPosition;
use spacetime_engine::entity::structs::EntityPosition;
use spacetime_engine::math::structs::I16Vec2;
use spacetime_engine::operations::structs::{InstanceID, OperationQueue};
use spacetime_engine::operations::traits::*;
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

pub trait Script {
    fn run(&self, context: &mut ScriptContext);
}

pub struct ScriptContext {
}

impl ScriptContext {
    pub fn run_op<O, A, R>(&mut self, args: O::Args) -> O::Result
    where
        O: Operation<Args = A, Result = R>,
        A: OpArgs,
        R: OpResult,
    {
        // TODO: Somehow create a mapping to the actual operation queueing and execution, yk?
        unimplemented!();
    }
}


fn spawn_chunk_new(context: &mut ScriptContext) {
    let entity_id = match context.run_op::<CreateEntity, _, _>(CreateEntityArgs { entity_position: EntityPosition(Vec2::new(0.0, 0.0)) }) {
        CreateEntityResult::Ok{ entity_id } => entity_id,
        CreateEntityResult::Err(_) => {
            return;
        },
    };

    let _chunk_id = match context.run_op::<UpgradeToChunk, _, _>(UpgradeToChunkArgs { target_entity_id: entity_id, chunk_position: ChunkPosition(I16Vec2(0, 0)), chunk_owner: None }) {
        UpgradeToChunkResult::Ok{ chunk_id } => chunk_id,
        UpgradeToChunkResult::Err(_) => {
            return;
        },
    };
}

fn startup() {
    spawn_chunk();
    spawn_chunk_actor();
}

fn spawn_chunk() {
    // Create operation 'A'
    let entity_position = EntityPosition(Vec2::new(0.0, 0.0));
    let create_entity_args = CreateEntityArgs { entity_position };
    let create_entity_operation = CreateEntity::new(create_entity_args, Some(|result| {
        // Check operation 'A' result
        if let CreateEntityResult::Ok{ entity_id } = result {
            warn!("Successfully created entity '{}'!", entity_id);

            // Create operation 'B'
            let upgrade_to_chunk_args = UpgradeToChunkArgs {
                target_entity_id: entity_id,
                chunk_position: ChunkPosition(I16Vec2(0, 0)),
                chunk_owner: None
            };
            let upgrade_to_chunk_operation = UpgradeToChunk::new(upgrade_to_chunk_args, Some(|result| {
                // Check operation 'B' result
                if let UpgradeToChunkResult::Ok{ chunk_id } = result {
                    warn!("Successfully upgraded to chunk '{}'!", chunk_id);
                } else {
                    warn!("Failed to upgrade entity to chunk!");
                }
            }));

            warn!("Created chunk operation!");

            // Queue operation 'B'
            let mut operation_queue = OPERATION_QUEUE.lock().unwrap();
            operation_queue.add_operation(Box::new(upgrade_to_chunk_operation));
            drop(operation_queue);

            warn!("Queued chunk operation!");
        } else {
            warn!("Failed to create entity!");
        }
    }));

    warn!("Created entity operation!");

    // Queue operation 'A'
    let mut operation_queue = OPERATION_QUEUE.lock().unwrap();
    operation_queue.add_operation(Box::new(create_entity_operation));
    drop(operation_queue);

    warn!("Queued entity operation!");
}

fn spawn_chunk_actor() {
    // Create operation 'A'
    let entity_position = EntityPosition(Vec2::new(0.0, 0.0));
    let create_entity_args = CreateEntityArgs { entity_position };
    let create_entity_operation = CreateEntity::new(create_entity_args, Some(|result| {
        // Check operation 'A' result
        if let CreateEntityResult::Ok{ entity_id } = result {
            warn!("Successfully created entity '{}'!", entity_id);

            // Create operation 'B'
            let upgrade_to_chunk_actor_args = UpgradeToChunkActorArgs {
                    target_entity_id: entity_id,
                    chunk_actor_start_chunk_id: InstanceID::new(1),
            };
            let upgrade_to_chunk_actor_operation = UpgradeToChunkActor::new(upgrade_to_chunk_actor_args, Some(|result| {
                // Check operation 'B' result
                if let UpgradeToChunkActorResult::Ok{ chunk_actor_id } = result {
                    warn!("Successfully upgraded to chunk actor '{}'!", chunk_actor_id);
                } else {
                    warn!("Failed to upgrade entity to chunk actor!");
                }
            }));

            warn!("Created chunk actor operation!");

            // Queue operation 'B'
            let mut operation_queue = OPERATION_QUEUE.lock().unwrap();
            operation_queue.add_operation(Box::new(upgrade_to_chunk_actor_operation));
            drop(operation_queue);

            warn!("Queued chunk actor operation!");
        } else {
            warn!("Failed to create entity!");
        }
    }));

    warn!("Created entity operation!");

    // Queue operation 'A'
    let mut operation_queue = OPERATION_QUEUE.lock().unwrap();
    operation_queue.add_operation(Box::new(create_entity_operation));
    drop(operation_queue);

    warn!("Queued entity operation!");
}