use bevy::prelude::*;

use super::components::ChunkComponent;
use super::enums::ChunkAction;
use super::functions::{chunk_pos_to_world, despawn_chunk, spawn_chunk, transfer_chunk_ownership, world_pos_to_chunk};
use super::ChunkActionBuffer;
use super::ChunkManager;

pub(in crate) fn update_chunk_system(
    chunk_query: Query<(Entity, &Transform, &ChunkComponent)>,
) {
    for (_, transform, chunk) in chunk_query.iter() {
        let world_pos = transform.translation.truncate();
        let chunk_pos = world_pos_to_chunk(world_pos);

        if chunk.coord != chunk_pos {
            panic!("Attempted to move chunk entity");
        }

        if chunk_pos_to_world(chunk.coord) != world_pos {
            panic!("Attempted to move chunk entity");
        }
    }
}

pub(in crate) fn process_chunk_actions(
    mut commands: Commands,
    mut chunk_query: Query<(Entity, &mut ChunkComponent)>,
    mut chunk_manager: ResMut<ChunkManager>,
    mut chunk_action_buffer: ResMut<ChunkActionBuffer>,
) {
    let mut chunk_actions: Vec<ChunkAction> = chunk_action_buffer.0.values().cloned().collect();
    while !chunk_actions.is_empty() {
        match chunk_actions.pop().unwrap() {
            ChunkAction::Spawn { coord, owner, .. } => {
                match spawn_chunk(&mut commands, &mut chunk_manager, &mut chunk_action_buffer, coord, owner) {
                    Ok(_) => {},
                    Err(err) => {
                        panic!("Failed to spawn chunk '{:?}': {:?}", coord, err);
                    }
                }
            },
            ChunkAction::Despawn { coord } => {
                match despawn_chunk(&mut commands, &mut chunk_manager, &mut chunk_action_buffer, &mut chunk_query, coord) {
                    Ok(_) => {},
                    Err(err) => {
                        panic!("Failed to despawn chunk '{:?}': {:?}", coord, err);
                    }
                }
            },
            ChunkAction::TransferOwnership { coord, new_owner, .. } => {
                match transfer_chunk_ownership(&mut chunk_manager, &mut chunk_action_buffer, &mut chunk_query, coord, new_owner) {
                    Ok(_) => {},
                    Err(err) => {
                        panic!("Failed to transfer ownership of chunk '{:?}': {:?}", coord, err);
                    }
                }
            }
        }
    }
}