use bevy::prelude::*;
use super::components::ChunkComponent;
use super::enums::ChunkAction;
use super::functions::{chunk_pos_to_world, despawn_chunk, spawn_chunk, transfer_chunk_ownership, world_pos_to_chunk};
use super::{ChunkActionBuffer, ChunkManager};

// Verifies the integrity of chunk positions in the ECS.
pub(in crate) fn update_chunk_system(
    chunk_query: Query<(Entity, &Transform, &ChunkComponent)>,
) {
    for (_, transform, chunk) in chunk_query.iter() {
        let world_pos = transform.translation.truncate();
        let chunk_pos = world_pos_to_chunk(world_pos);

        // Ensure chunks haven't been moved incorrectly
        assert_eq!(chunk.coord, chunk_pos, "Attempted to move chunk entity");
        assert_eq!(
            chunk_pos_to_world(chunk.coord),
            world_pos,
            "Attempted to move chunk entity"
        );
    }
}

// Chunk action processing configuration constants.
const CHUNK_ACTION_LOG_THRESHOLD: usize = 1;
const MAX_ACTIONS_PER_FRAME: usize = 1000;

type MaxChunkActions = usize;
type ProcessingIteration = usize;

/// Processes queued chunk actions (spawning, despawning, ownership transfer).
pub(in crate) fn process_chunk_actions(
    mut commands: Commands,
    mut chunk_query: Query<(Entity, &mut ChunkComponent)>,
    mut chunk_manager: ResMut<ChunkManager>,
    mut chunk_action_buffer: ResMut<ChunkActionBuffer>,
    mut max_chunk_actions: Local<MaxChunkActions>,
    mut processing_iteration: Local<ProcessingIteration>,
) {
    // Gather actions from the buffer.
    let chunk_actions: Vec<((i32, i32), ChunkAction)> = chunk_action_buffer
        .0
        .iter()
        .map(|(key, action)| (*key, action.clone()))
        .collect();
    let total_actions = chunk_actions.len();

    // Exit early if no actions are queued.
    if total_actions == 0 {
        return;
    }

    // Update and log the maximum number of actions seen in a frame.
    if total_actions > *max_chunk_actions {
        *max_chunk_actions = total_actions;
        warn!("New record: Processing {:?} chunk actions", total_actions);
    }

    // Log actions if they exceed a defined threshold.
    if total_actions > CHUNK_ACTION_LOG_THRESHOLD {
        debug!(
            "[Frame {:?}] Processing {:?} chunk actions",
            *processing_iteration, total_actions
        );
    }

    // Count the types of actions for logging purposes.
    let (mut spawn_count, mut despawn_count, mut transfer_count) = (0, 0, 0);
    for (_, action) in &chunk_actions {
        match action {
            ChunkAction::Spawn { .. } => spawn_count += 1,
            ChunkAction::Despawn { .. } => despawn_count += 1,
            ChunkAction::TransferOwnership { .. } => transfer_count += 1,
        }
    }
    debug!(
        "[Frame {:?}] Action breakdown: Spawns = {:?}, Despawns = {:?}, Transfers = {:?}",
        *processing_iteration, spawn_count, despawn_count, transfer_count
    );

    // Determine if truncation is needed for logging.
    let truncation_limit = CHUNK_ACTION_LOG_THRESHOLD * 2;
    let truncation_start = CHUNK_ACTION_LOG_THRESHOLD;
    let truncation_end = total_actions - CHUNK_ACTION_LOG_THRESHOLD;
    let mut truncation_logged = false;

    // Process actions with a per-frame limit.
    let mut actions_processed = 0;
    for (index, (key, chunk_action)) in chunk_actions.into_iter().enumerate() {
        if actions_processed >= MAX_ACTIONS_PER_FRAME {
            debug!(
                "[Frame {:?}] Reached max actions ({:?}) per frame. Deferring remaining actions.",
                *processing_iteration, MAX_ACTIONS_PER_FRAME
            );
            break;
        }

        // Skip redundant logging for truncated actions.
        if truncation_limit <= total_actions
            && index >= truncation_start
            && index < truncation_end
        {
            if !truncation_logged {
                warn!(
                    "[Frame {:?}] Skipping debug logs for {:?} actions...",
                    *processing_iteration, truncation_end - truncation_start
                );
                truncation_logged = true;
            }
            continue;
        }

        debug!(
            "[Frame {:?}, Action {:?}] Processing chunk action",
            *processing_iteration, index
        );

        // Handle individual chunk actions.
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

        // Remove the processed action and increment the counter.
        chunk_action_buffer.0.remove(&key);
        actions_processed += 1;
    }

    // Increment the processing iteration and log the results.
    *processing_iteration += 1;
    debug!(
        "[Frame {:?}] Successfully processed {:?}/{:?} actions",
        *processing_iteration, actions_processed, total_actions
    );
}
