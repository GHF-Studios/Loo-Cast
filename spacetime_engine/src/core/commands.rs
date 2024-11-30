use std::any::Any;

use bevy::prelude::*;
use futures::future::join_all;
use crate::singletons::LOCKING_HIERARCHY;
use crate::singletons::TOKIO_RUNTIME;
use crate::chunk::structs::ChunkPosition;
use crate::entity::structs::EntityPosition;
use crate::chunk::commands::*;
use crate::chunk_actor::commands::*;
use crate::camera::commands::*;
use crate::math::structs::I16Vec2;
use super::structs::AbsoluteLockingPath;
use super::structs::LockingPathSegment;
use super::traits::*;
use super::wrappers::CoreCommandTypeRegistry;

pub(in crate) async fn pre_startup() {
    let mut locking_hierarchy = LOCKING_HIERARCHY.lock().unwrap();
    
    let core_path = AbsoluteLockingPath::new_from_literal("core");
    let core_mutex = locking_hierarchy.get_node_raw(core_path.clone()).unwrap();

    let command_type_registry_path_segment = LockingPathSegment::new_string("command_types");
    let command_type_registry_path = core_path.clone().push(command_type_registry_path_segment).unwrap();
    let command_type_registry_data = CoreCommandTypeRegistry::new();
    locking_hierarchy.insert_branch(core_path, core_mutex, command_type_registry_path_segment, command_type_registry_data).unwrap();
    locking_hierarchy.pre_startup(command_type_registry_path).unwrap();
}

pub(in crate) async fn startup() {
    let mut locking_hierarchy = LOCKING_HIERARCHY.lock().unwrap();

    let command_type_registry_path = AbsoluteLockingPath::new_from_literal("core.command_types");
    locking_hierarchy.startup(command_type_registry_path).unwrap();

    let runtime = TOKIO_RUNTIME.lock().unwrap();
    runtime.spawn(async {
        spawn_main_camera(Box::new(())).await;
        spawn_start_chunks(Box::new(2)).await;
        spawn_start_chunk_actors(Box::new(2)).await;
    });
}

pub(in crate) async fn post_startup() {
    let mut locking_hierarchy = LOCKING_HIERARCHY.lock().unwrap();

    let command_type_registry_path = AbsoluteLockingPath::new_from_literal("core.command_types");
    locking_hierarchy.post_startup(command_type_registry_path).unwrap();
}

pub async fn spawn_main_camera(_params: Box<dyn Any>) -> Box<dyn Any> {
    let entity_position = EntityPosition(Vec2::new(0.0, 0.0));
    if let Err(e) = spawn_camera(entity_position).await {
        error!("Error spawning camera: {:?}", e);
    } else {
        debug!("Spawned camera at {:?}", entity_position);
    }

    Box::new(())
}

pub async fn spawn_start_chunks(params: Box<dyn Any>) -> Box<dyn Any> {
    let range = *params.downcast_ref::<i16>().unwrap();

    let mut tasks = Vec::new();

    for x in -range..=range {
        for y in -range..=range {
            let chunk_position = ChunkPosition(I16Vec2(x, y));

            let task = async move {
                if let Err(e) = spawn_chunk(chunk_position).await {
                    error!("Error spawning chunk: {:?}", e);
                } else {
                    debug!("Spawned chunk at {:?}", chunk_position);
                }
            };

            tasks.push(task);
        }
    }

    join_all(tasks).await;

    Box::new(())
}

pub async fn spawn_start_chunk_actors(params: Box<dyn Any>) -> Box<dyn Any> {
    let range = *params.downcast_ref::<i16>().unwrap();

    let mut tasks = Vec::new();

    for x in -range..=range {
        for y in -range..=range {
            let chunk_position = ChunkPosition(I16Vec2(x, y));
            let entity_position: EntityPosition = chunk_position.into();

            let task = async move {
                if let Err(e) = spawn_chunk_actor(entity_position).await {
                    error!("Error spawning chunk actor: {:?}", e);
                } else {
                    debug!("Spawned chunk actor at {:?}", entity_position);
                }
            };

            tasks.push(task);
        }
    }

    join_all(tasks).await;

    Box::new(())
}