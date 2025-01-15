use bevy::prelude::*;
use crate::chunk::enums::ChunkActionPriority;

use super::components::ChunkComponent;
use super::enums::ChunkAction;
use super::functions::{chunk_pos_to_world, despawn_chunk, spawn_chunk, transfer_chunk_ownership, world_pos_to_chunk};
use super::{ChunkActionBuffer, ChunkManager};

pub(in crate) fn update_chunk_system(
    chunk_query: Query<(Entity, &Transform, &ChunkComponent)>,
) {
    for (_, transform, chunk) in chunk_query.iter() {
        let world_pos = transform.translation.truncate();
        let chunk_pos = world_pos_to_chunk(world_pos);

        assert_eq!(
            chunk.coord, 
            chunk_pos, 
            "Attempted to move chunk entity"
        );
        assert_eq!(
            chunk_pos_to_world(chunk.coord),
            world_pos,
            "Attempted to move chunk entity"
        );
    }
}

const MAX_CHUNK_ACTIONS_PER_UPDATE_CYCLE: usize = 10;

pub(in crate) fn process_chunk_actions(
    mut commands: Commands,
    mut chunk_query: Query<(Entity, &mut ChunkComponent)>,
    mut chunk_manager: ResMut<ChunkManager>,
    mut chunk_action_buffer: ResMut<ChunkActionBuffer>,
) {
    let mut chunk_actions: Vec<((i32, i32), (ChunkActionPriority, ChunkAction))> = chunk_action_buffer
        .actions
        .iter()
        .map(|(key, action)| (*key, action.clone()))
        .collect();
    let total_actions = chunk_actions.len();

    if total_actions == 0 {
        return;
    }

    chunk_actions.sort_by(|a, b| b.1.0.cmp(&a.1.0));

    for (total_actions_processed, (chunk_coord, (chunk_action_priority, chunk_action))) in chunk_actions.into_iter().enumerate() {
        if chunk_action_priority != ChunkActionPriority::Realtime && total_actions_processed >= MAX_CHUNK_ACTIONS_PER_UPDATE_CYCLE {
            warn!("Reached max actions ({:?}) per update cycle. Deferring remaining actions.", MAX_CHUNK_ACTIONS_PER_UPDATE_CYCLE);
            break;
        }

        match chunk_action {
            ChunkAction::Spawn { coord, owner, .. } => {
                if let Err(err) =
                    spawn_chunk(&mut commands, &mut chunk_manager, &mut chunk_action_buffer, coord, owner)
                {
                    panic!("Failed to spawn chunk '{:?}': {:?}", coord, err);
                }
            }
            ChunkAction::Despawn { coord, .. } => {
                if let Err(err) = despawn_chunk(
                    &mut commands,
                    &mut chunk_manager,
                    &mut chunk_action_buffer,
                    &mut chunk_query,
                    coord,
                ) {
                    panic!("Failed to despawn chunk '{:?}': {:?}", coord, err);
                }
            }
            ChunkAction::TransferOwnership { coord, new_owner, .. } => {
                if let Err(err) = transfer_chunk_ownership(
                    &mut chunk_manager,
                    &mut chunk_action_buffer,
                    &mut chunk_query,
                    coord,
                    new_owner,
                ) {
                    panic!("Failed to transfer ownership of chunk '{:?}': {:?}", coord, err);
                }
            }
        }

        chunk_action_buffer.remove_action(&chunk_coord);
    }
}