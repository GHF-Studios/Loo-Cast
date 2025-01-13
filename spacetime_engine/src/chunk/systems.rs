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

type MaxChunkActions = usize;
type ProcessingIteration = usize;
const TRUNCATION_THRESHOLD: usize = 1;

pub(in crate) fn process_chunk_actions(
    mut commands: Commands,
    mut chunk_query: Query<(Entity, &mut ChunkComponent)>,
    mut chunk_manager: ResMut<ChunkManager>,
    mut chunk_action_buffer: ResMut<ChunkActionBuffer>,
    mut max_chunk_actions: Local<MaxChunkActions>,
    mut processing_iteration: Local<ProcessingIteration>,
) {
    const MAX_CHUNK_ACTIONS_PER_FRAME: usize = 100;

    // Flatten and clone all current actions in the buffer.
    let mut chunk_actions: Vec<ChunkAction> = chunk_action_buffer.0.values().cloned().collect();

    // Check if there are actions to process.
    let chunk_actions_count = chunk_actions.len();
    if chunk_actions_count == 0 {
        return;
    }

    if chunk_actions_count > *max_chunk_actions {
        *max_chunk_actions = chunk_actions_count;
        warn!(
            "New highscore: Processing {:?} chunk actions",
            chunk_actions_count
        );
    }

    // Calculate how many actions to process this frame.
    let actions_to_process = usize::min(chunk_actions_count, MAX_CHUNK_ACTIONS_PER_FRAME);

    let mut spawn_count = 0;
    let mut despawn_count = 0;
    let mut transfer_count = 0;

    // Process only up to the batch size.
    for chunk_action in chunk_actions.drain(0..actions_to_process) {
        match chunk_action {
            ChunkAction::Spawn { coord, owner, .. } => {
                spawn_count += 1;
                if let Err(err) = spawn_chunk(&mut commands, &mut chunk_manager, &mut chunk_action_buffer, coord, owner) {
                    panic!("Failed to spawn chunk '{:?}': {:?}", coord, err);
                }
            }
            ChunkAction::Despawn { coord } => {
                despawn_count += 1;
                if let Err(err) = despawn_chunk(&mut commands, &mut chunk_manager, &mut chunk_action_buffer, &mut chunk_query, coord) {
                    panic!("Failed to despawn chunk '{:?}': {:?}", coord, err);
                }
            }
            ChunkAction::TransferOwnership { coord, new_owner, .. } => {
                transfer_count += 1;
                if let Err(err) = transfer_chunk_ownership(&mut chunk_manager, &mut chunk_action_buffer, &mut chunk_query, coord, new_owner) {
                    panic!("Failed to transfer ownership of chunk '{:?}': {:?}", coord, err);
                }
            }
        }
    }

    debug!(
        "Processed {:?} chunk actions (Spawns: {:?}, Despawns: {:?}, Transfers: {:?}) in frame iteration {:?}",
        actions_to_process, spawn_count, despawn_count, transfer_count, *processing_iteration
    );

    // If there are unprocessed actions, add them back to the buffer.
    if !chunk_actions.is_empty() {
        // Overwrite the buffer with remaining actions.
        chunk_action_buffer.0 = chunk_actions
            .into_iter()
            .map(|action| (action.get_coord(), action))
            .collect();
    }

    // Increment the processing iteration.
    *processing_iteration += 1;
}

