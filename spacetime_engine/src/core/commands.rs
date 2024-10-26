use bevy::prelude::*;
use futures::future::join_all;
use crate::camera_2d_bundle::structs::Camera2DBundle;
use crate::chunk::components::Chunk;
use crate::chunk::structs::ChunkPosition;
use crate::chunk_actor::components::ChunkActor;
use crate::chunk_loader::components::ChunkLoader;
use crate::command::structs::Command;
use crate::entity::structs::EntityPosition;
use crate::chunk::commands::*;
use crate::chunk_actor::commands::*;
use crate::camera::commands::*;
use crate::math::structs::I16Vec2;
use crate::operation::structs::Operation;
use crate::player::components::Player;
use super::singletons::LOCKING_HIERARCHY;
use super::structs::*;
use super::traits::*;
use super::wrappers::*;

pub async fn startup() {
    init_locking_hierarchy().await;
    spawn_main_camera().await;
    spawn_start_chunks(2).await;
    spawn_start_chunk_actors(2).await;
}

pub async fn init_locking_hierarchy() {
    let mut locking_hierarchy = LOCKING_HIERARCHY.lock().unwrap();

    let core_type_path = AbsoluteLockingPath::new().push(LockingPathSegment::new_static::<LockingBranchNode<Type>>("core"));
    let core_type_node = LockingBranchNode::<Type>::new(Type::new::<Core>());
    locking_hierarchy.insert(core_type_path.unwrap(), Box::new(core_type_node)).unwrap();
    
    let operations_type_path = AbsoluteLockingPath::new().push(LockingPathSegment::new_static::<LockingBranchNode<Type>>("operation"));
    let operations_type_node = LockingBranchNode::<Type>::new(Type::new::<Operation>());
    locking_hierarchy.insert(operations_type_path.unwrap(), Box::new(operations_type_node)).unwrap();
    
    let commands_type_path = AbsoluteLockingPath::new().push(LockingPathSegment::new_static::<LockingBranchNode<Type>>("command"));
    let commands_type_node = LockingBranchNode::<Type>::new(Type::new::<Command>());
    locking_hierarchy.insert(commands_type_path.unwrap(), Box::new(commands_type_node)).unwrap();

    let entity_type_path = AbsoluteLockingPath::new().push(LockingPathSegment::new_static::<LockingBranchNode<Type>>("entity"));
    let entity_type_node = LockingBranchNode::<Type>::new(Type::new::<Entity>());
    locking_hierarchy.insert(entity_type_path.unwrap(), Box::new(entity_type_node)).unwrap();

    let chunk_type_path = AbsoluteLockingPath::new().push(LockingPathSegment::new_static::<LockingBranchNode<Type>>("chunk"));
    let chunk_type_node = LockingBranchNode::<Type>::new(Type::new::<Chunk>());
    locking_hierarchy.insert(chunk_type_path.unwrap(), Box::new(chunk_type_node)).unwrap();

    let chunk_actor_type_path = AbsoluteLockingPath::new().push(LockingPathSegment::new_static::<LockingBranchNode<Type>>("chunk_actor"));
    let chunk_actor_type_node = LockingBranchNode::<Type>::new(Type::new::<ChunkActor>());
    locking_hierarchy.insert(chunk_actor_type_path.unwrap(), Box::new(chunk_actor_type_node)).unwrap();

    let chunk_loader_type_path = AbsoluteLockingPath::new().push(LockingPathSegment::new_static::<LockingBranchNode<Type>>("chunk_loader"));
    let chunk_loader_type_node = LockingBranchNode::<Type>::new(Type::new::<ChunkLoader>());
    locking_hierarchy.insert(chunk_loader_type_path.unwrap(), Box::new(chunk_loader_type_node)).unwrap();

    let camera_type_path = AbsoluteLockingPath::new().push(LockingPathSegment::new_static::<LockingBranchNode<Type>>("camera"));
    let camera_type_node = LockingBranchNode::<Type>::new(Type::new::<Camera>());
    locking_hierarchy.insert(camera_type_path.unwrap(), Box::new(camera_type_node)).unwrap();

    let camera_2d_bundle_type_path = AbsoluteLockingPath::new().push(LockingPathSegment::new_static::<LockingBranchNode<Type>>("camera_2d_bundle"));
    let camera_2d_bundle_type_node = LockingBranchNode::<Type>::new(Type::new::<Camera2DBundle>());
    locking_hierarchy.insert(camera_2d_bundle_type_path.unwrap(), Box::new(camera_2d_bundle_type_node)).unwrap();

    let player_type_path = AbsoluteLockingPath::new().push(LockingPathSegment::new_static::<LockingBranchNode<Type>>("player"));
    let player_type_node = LockingBranchNode::<Type>::new(Type::new::<Player>());
    locking_hierarchy.insert(player_type_path.unwrap(), Box::new(player_type_node)).unwrap();

    let sprite_bundle_type_path = AbsoluteLockingPath::new().push(LockingPathSegment::new_static::<LockingBranchNode<Type>>("sprite_bundle"));
    let sprite_bundle_type_node = LockingBranchNode::<Type>::new(Type::new::<SpriteBundle>());
    locking_hierarchy.insert(sprite_bundle_type_path.unwrap(), Box::new(sprite_bundle_type_node)).unwrap();
}

async fn spawn_main_camera() {
    let entity_position = EntityPosition(Vec2::new(0.0, 0.0));
    if let Err(e) = spawn_camera(entity_position).await {
        error!("Error spawning camera: {:?}", e);
    } else {
        debug!("Spawned camera at {:?}", entity_position);
    }
}

async fn spawn_start_chunks(range: i16) {
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

async fn spawn_start_chunk_actors(range: i16) {
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