use bevy::prelude::*;
use futures::future::join_all;
use crate::command::wrappers::CommandType;
use crate::structs::*;
use crate::chunk::structs::ChunkPosition;
use crate::entity::structs::EntityPosition;
use crate::chunk::commands::*;
use crate::chunk_actor::commands::*;
use crate::camera::commands::*;
use crate::math::structs::I16Vec2;
use super::singletons::*;
use super::structs::AbsoluteLockingPath;
use super::structs::LockingPathSegment;
use super::traits::*;
use super::wrappers::CoreCommandTypeRegistry;

pub(in crate) async fn pre_startup() {
    let mut locking_hierarchy = LOCKING_HIERARCHY.lock().unwrap();
    
    let core_path = AbsoluteLockingPath::new_from_literal("core");
    let core_mutex = locking_hierarchy.get_node_raw(core_path.clone()).unwrap();

    let command_type_registry_path_segment = LockingPathSegment::new_string("command_type_registry");
    let command_type_registry_path = core_path.clone().push(command_type_registry_path_segment).unwrap();
    let command_type_registry_data = CoreCommandTypeRegistry::new();
    locking_hierarchy.insert_branch::<MainType, CoreCommandTypeRegistry, CommandType>(core_path, core_mutex, command_type_registry_path_segment, command_type_registry_data).unwrap();
    locking_hierarchy.pre_startup::<CoreCommandTypeRegistry>(command_type_registry_path).unwrap();
}

pub(in crate) async fn startup() {
    let mut locking_hierarchy = LOCKING_HIERARCHY.lock().unwrap();

    let command_type_registry_path = AbsoluteLockingPath::new_from_literal("core/command_type_registry");
    locking_hierarchy.startup::<CoreCommandTypeRegistry>(command_type_registry_path).unwrap();

    let runtime = TOKIO_RUNTIME.lock().unwrap();
    runtime.spawn(async {
        spawn_main_camera().await;
        spawn_start_chunks(2).await;
        spawn_start_chunk_actors(2).await;
    });
}

pub(in crate) async fn post_startup() {
    let mut locking_hierarchy = LOCKING_HIERARCHY.lock().unwrap();

    let command_type_registry_path = AbsoluteLockingPath::new_from_literal("core/command_type_registry");
    locking_hierarchy.post_startup::<CoreCommandTypeRegistry>(command_type_registry_path).unwrap();
}

pub async fn spawn_main_camera() {
    let entity_position = EntityPosition(Vec2::new(0.0, 0.0));
    if let Err(e) = spawn_camera(entity_position).await {
        error!("Error spawning camera: {:?}", e);
    } else {
        debug!("Spawned camera at {:?}", entity_position);
    }
}

pub async fn spawn_start_chunks(range: i16) {
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
}

pub async fn spawn_start_chunk_actors(range: i16) {
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
}