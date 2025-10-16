// Imports
use bevy::prelude::{warn, Query, Res, ResMut, Transform, Vec2};
use std::collections::HashSet;

use crate::{chunk::{
    components::Chunk, intent::{ActionIntent, ActionPriority, ResolutionWarning, ResolvedActionIntent, State as ChunkState, resolve_intent}, resources::{ActionIntentBuffer, ActionIntentCommitBuffer, ChunkManager, GridOriginOffset}, traits::Vec2Ext, types::{ChunkOwnerId, GridCoord}
}, utils::i128vec2::I128Vec2};
use crate::chunk_loader::components::ChunkLoader;
use crate::workflow::types::Outcome;

// Items
pub struct UnloadChunkInput {
    pub owner_id: ChunkOwnerId,
    pub grid_coord: GridCoord,
    pub chunk_loader_distance_squared: u32,
    pub chunk_loader_radius_squared: u32,
}

pub struct DespawnChunkState {
    pub coord: GridCoord,
    pub is_despawned: bool,
}

pub struct TransferChunkOwnershipState {
    pub coord: GridCoord,
    pub owner_id: ChunkOwnerId,
    pub is_ownership_transfered: bool,
}

pub fn calculate_despawn_priority(distance_squared: u32, radius_squared: u32) -> ActionPriority {
    let normalized_distance = distance_squared as f64 / radius_squared as f64;
    let priority_value = (normalized_distance * i64::MAX as f64) as i64;
    ActionPriority::Deferred(priority_value)
}

pub fn is_chunk_in_loader_range(grid_origin_offset: I128Vec2, grid_coord: &GridCoord, loader_position: Vec2, loader_radius: u32) -> bool {
    let loader_grid_coord = loader_position.to_grid_coord(grid_coord.scale, grid_origin_offset);
    let dx = grid_coord.xy.x - loader_grid_coord.xy.x;
    let dy = grid_coord.xy.y - loader_grid_coord.xy.y;
    let distance_squared = dx * dx + dy * dy;
    let radius_squared = ((loader_radius as i32) * (loader_radius as i32)) as i128;
    distance_squared <= radius_squared
}

// Core Types
#[derive(bevy::ecs::system::SystemParam)]
pub struct MainAccess<'w, 's> {
    pub chunk_manager: Res<'w, ChunkManager>,
    pub action_intent_commit_buffer: ResMut<'w, ActionIntentCommitBuffer>,
    pub action_intent_buffer: ResMut<'w, ActionIntentBuffer>,
    pub chunk_query: Query<'w, 's, &'static Chunk>,
    pub chunk_loader_query: Query<'w, 's, (&'static Transform, &'static ChunkLoader)>,
    pub grid_origin_offset: Res<'w, GridOriginOffset>,
}

pub struct Input {
    pub inputs: Vec<UnloadChunkInput>,
}

pub struct State {
    pub despawn_chunk_states: Vec<DespawnChunkState>,
    pub transfer_chunk_ownership_states: Vec<TransferChunkOwnershipState>,
}

// Core Functions
pub fn setup_ecs_while(input: Input, main_access: MainAccess) -> State {
    // warn!("Setting up UnloadChunks");
    let chunk_manager = main_access.chunk_manager;
    let mut action_intent_commit_buffer = main_access.action_intent_commit_buffer;
    let mut action_intent_buffer = main_access.action_intent_buffer;
    let chunk_query = main_access.chunk_query;
    let chunk_loader_query = main_access.chunk_loader_query;
    let grid_origin_offset = main_access.grid_origin_offset;

    let mut despawn_chunk_states = Vec::new();
    let mut transfer_chunk_ownership_states = Vec::new();
    let mut affected_owners = HashSet::new();

    for input in input.inputs {
        let owner_id = input.owner_id;
        let coord = input.grid_coord;
        let distance_squared = input.chunk_loader_distance_squared;
        let radius_squared = input.chunk_loader_radius_squared;

        affected_owners.insert(owner_id.clone());

        let is_loaded = chunk_manager.is_loaded(&coord);
        if !is_loaded {
            warn!("UnloadChunks received an input for a chunk that is not loaded at {:?}. Ignoring.", coord);
            continue;
        }

        let committed = action_intent_commit_buffer.get(&coord);
        let buffered = action_intent_buffer.get(&coord);

        let chunk_state = if let Some(owner_id) = chunk_manager.owned_chunks.get(&coord) {
            ChunkState::Owned(owner_id.clone())
        } else {
            unreachable!("Unreachable state: Chunk is absent")
        };

        let (transfer_candidate, is_chunk_existing) = match chunk_query.iter().find(|chunk| chunk.coord == coord) {
            Some(chunk) => {
                let tc = chunk_loader_query.iter().find_map(|(transform, loader)| {
                    if loader.id() == chunk.owner_id() {
                        None
                    } else if is_chunk_in_loader_range(grid_origin_offset.0, &coord, transform.translation.truncate(), loader.radius) {
                        Some(loader.id())
                    } else {
                        None
                    }
                });

                (tc, true)
            }
            None => (None, false),
        };

        let proposed_intent = match (transfer_candidate, is_chunk_existing) {
            (None, false) => None,
            (None, true) => Some(ActionIntent::Despawn {
                owner_id,
                coord,
                priority: calculate_despawn_priority(distance_squared, radius_squared),
            }),
            (Some(_), false) => None,
            (Some(new_owner_id), true) => Some(ActionIntent::TransferOwnership {
                new_owner_id: new_owner_id.clone(),
                coord,
                priority: ActionPriority::Realtime,
            }),
        };

        let resolution = match proposed_intent {
            Some(proposed_intent) => resolve_intent(&chunk_state, committed, buffered, proposed_intent.clone()),
            None => ResolvedActionIntent::DiscardIncoming(ResolutionWarning::RedundantIntent),
        };

        match resolution {
            ResolvedActionIntent::PushCommit(action) => match action.clone() {
                ActionIntent::Despawn { .. } => {
                    action_intent_commit_buffer.commit_intent(action);
                    despawn_chunk_states.push(DespawnChunkState {
                        coord,
                        is_despawned: false,
                    });
                }
                ActionIntent::TransferOwnership { new_owner_id, .. } => {
                    action_intent_commit_buffer.commit_intent(action);
                    transfer_chunk_ownership_states.push(TransferChunkOwnershipState {
                        coord,
                        owner_id: new_owner_id,
                        is_ownership_transfered: false,
                    });
                }
                ActionIntent::Spawn { .. } => {
                    panic!("UnloadChunks should never emit or commit a Spawn intent.");
                }
            },
            ResolvedActionIntent::PushBuffer(action) => match action.clone() {
                ActionIntent::Despawn { .. } => {
                    action_intent_buffer.buffer_intent(action);
                }
                ActionIntent::TransferOwnership { .. } => {
                    action_intent_buffer.buffer_intent(action);
                }
                ActionIntent::Spawn { .. } => {
                    panic!("UnloadChunks should never buffer a Spawn intent.");
                }
            },
            ResolvedActionIntent::CancelIntent => {
                action_intent_buffer.cancel_intent(&coord);
            }
            ResolvedActionIntent::DiscardIncoming(reason) => {
                warn!("UnloadChunks intent was discarded: {:?}", reason);
                continue;
            }
            ResolvedActionIntent::Error(error) => {
                panic!("UnloadChunks resolution error: {:?}", error);
            }
        }
    }

    for _affected_owner in affected_owners {
        // warn!("Setup UnloadChunks for {:?}", affected_owner.id());
    }

    State {
        despawn_chunk_states,
        transfer_chunk_ownership_states,
    }
}

pub fn run_ecs_while(state: State, main_access: MainAccess) -> Outcome<State, ()> {
    // warn!("Running UnloadChunks");
    let chunk_query = main_access.chunk_query;

    let despawn_chunk_states = state
        .despawn_chunk_states
        .into_iter()
        .map(|mut s| {
            if chunk_query.iter().all(|chunk| chunk.coord != s.coord) {
                s.is_despawned = true;
            }
            s
        })
        .collect::<Vec<_>>();

    let transfer_chunk_ownership_states = state
        .transfer_chunk_ownership_states
        .into_iter()
        .map(|mut s| {
            if let Some(chunk) = chunk_query.iter().find(|chunk| chunk.coord == s.coord) {
                if *chunk.owner_id() == s.owner_id {
                    s.is_ownership_transfered = true;
                }
            }
            s
        })
        .collect::<Vec<_>>();

    let is_done = despawn_chunk_states.iter().all(|s| s.is_despawned) && transfer_chunk_ownership_states.iter().all(|s| s.is_ownership_transfered);

    if is_done {
        let unloaded_chunks_count = despawn_chunk_states.len() + transfer_chunk_ownership_states.len();
        if unloaded_chunks_count != 0 {
            // warn!("Ran UnloadChunks for # of chunks: {}", unloaded_chunks_count);
        }

        Outcome::Done(())
    } else {
        let not_despawned: Vec<_> = despawn_chunk_states.iter().filter(|d| !d.is_despawned).map(|s| s.coord).collect();

        let not_transferred: Vec<_> = transfer_chunk_ownership_states
            .iter()
            .filter(|ot| !ot.is_ownership_transfered)
            .map(|s| s.coord)
            .collect();

        if !not_despawned.is_empty() {
            warn!("Waiting: {} chunks still not despawned: {:?})", not_despawned.len(), not_despawned);
        }

        if !not_transferred.is_empty() {
            warn!("Waiting: {} chunks still not transferred: {:?})", not_transferred.len(), not_transferred);
        }

        Outcome::Wait(State {
            despawn_chunk_states,
            transfer_chunk_ownership_states,
        })
    }
}
