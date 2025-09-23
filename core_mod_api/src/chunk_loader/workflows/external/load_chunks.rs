// Imports
use bevy::prelude::{debug, warn, Entity, Query, Res, ResMut};
use std::collections::HashSet;

use crate::chunk::{
    components::Chunk,
    intent::{resolve_intent, ActionIntent, ActionPriority, ResolvedActionIntent},
    resources::{ActionIntentBuffer, ActionIntentCommitBuffer, ChunkManager},
    types::ChunkOwnerId,
};
use crate::chunk_loader::components::ChunkLoader;
use crate::config::statics::CONFIG;
use crate::usf::scale::Scale;
use crate::utils::components::InitHook;
use crate::workflow::types::Outcome;

// Items
pub struct LoadChunkInput<S: Scale> {
    pub owner_id: ChunkOwnerId<S>,
    pub chunk_coord: (i32, i32),
    pub chunk_loader_distance_squared: u32,
    pub chunk_loader_radius_squared: u32,
}

pub struct SpawnChunkState<S: Scale> {
    pub coord: (i32, i32),
    pub owner_id: ChunkOwnerId<S>,
    pub is_spawned: bool,
}

pub struct TransferChunkOwnershipState<S: Scale> {
    pub coord: (i32, i32),
    pub owner_id: ChunkOwnerId<S>,
    pub is_ownership_transfered: bool,
}

pub fn calculate_spawn_priority(distance_squared: u32, radius_squared: u32) -> ActionPriority {
    let normalized_distance = distance_squared as f64 / radius_squared as f64;
    let priority_value = (i64::MAX as f64 * (1.0 - normalized_distance)) as i64;

    ActionPriority::Deferred(priority_value)
}

// Core Types
pub struct MainAccess<'w, 's, S: Scale> {
    pub chunk_query: Query<'w, 's, &'static Chunk<S>>,
    pub chunk_manager: Res<'w, ChunkManager<S>>,
    pub action_intent_commit_buffer: ResMut<'w, ActionIntentCommitBuffer<S>>,
    pub action_intent_buffer: ResMut<'w, ActionIntentBuffer<S>>,
}

pub struct Input<S: Scale> {
    pub inputs: Vec<LoadChunkInput<S>>,
}

pub struct State<S: Scale> {
    pub spawn_chunk_states: Vec<SpawnChunkState<S>>,
    pub transfer_chunk_ownership_states: Vec<TransferChunkOwnershipState<S>>,
}

// Core Functions
pub fn setup_ecs_while<S: Scale>(input: Input<S>, main_access: MainAccess<S>) -> State<S> {
    let chunk_manager = main_access.chunk_manager;
    let mut action_intent_commit_buffer = main_access.action_intent_commit_buffer;
    let mut action_intent_buffer = main_access.action_intent_buffer;

    let mut spawn_chunk_states = Vec::new();
    let mut transfer_chunk_ownership_states = Vec::new();
    let mut affected_owners = HashSet::new();

    for input in input.inputs {
        let owner_id = input.owner_id;
        let coord = input.chunk_coord;

        affected_owners.insert(owner_id.clone());

        let is_loaded = chunk_manager.is_loaded(&coord);
        let is_owned = chunk_manager.is_owned(&coord);

        let committed = action_intent_commit_buffer.get(&coord);
        let buffered = action_intent_buffer.get(&coord);
        let chunk_state = if is_loaded {
            chunk_manager.owned_chunks.get(&coord).map_or_else(
                || panic!("Invariant violated: Loaded chunk with no owner_id."),
                |owner_id| crate::chunk::intent::State::Owned(owner_id.clone()),
            )
        } else {
            crate::chunk::intent::State::Absent
        };

        let proposed_intent = if !is_loaded {
            ActionIntent::Spawn {
                owner_id,
                coord,
                priority: calculate_spawn_priority(input.chunk_loader_distance_squared, input.chunk_loader_radius_squared),
            }
        } else if !is_owned {
            ActionIntent::TransferOwnership {
                new_owner_id: owner_id,
                coord,
                priority: ActionPriority::Realtime,
            }
        } else {
            continue; // Nothing to do
        };

        let resolution = resolve_intent(&chunk_state, committed, buffered, proposed_intent.clone());

        match resolution {
            ResolvedActionIntent::PushCommit(action) => match action.clone() {
                ActionIntent::Spawn { owner_id, .. } => {
                    action_intent_commit_buffer.commit_intent(action);
                    spawn_chunk_states.push(SpawnChunkState {
                        coord,
                        owner_id,
                        is_spawned: false,
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
                ActionIntent::Despawn { .. } => {
                    panic!("LoadChunks received a Despawn intent to commit. Invalid logic path.");
                }
            },
            ResolvedActionIntent::PushBuffer(action) => match action.clone() {
                ActionIntent::Spawn { .. } => {
                    action_intent_buffer.buffer_intent(action);
                }
                ActionIntent::TransferOwnership { .. } => {
                    action_intent_buffer.buffer_intent(action);
                }
                ActionIntent::Despawn { .. } => {
                    panic!("LoadChunks received a Despawn intent to buffer. Invalid logic path.");
                }
            },
            ResolvedActionIntent::CancelIntent => {
                warn!("LoadChunks cancelling intent for chunk at {:?}", coord);
                action_intent_buffer.cancel_intent(&coord);
            }
            ResolvedActionIntent::DiscardIncoming(reason) => {
                warn!("LoadChunks intent was discarded for chunk at {:?}: {:?}", coord, reason);
                continue;
            }
            ResolvedActionIntent::Error(error) => {
                panic!("Intent resolution failed: {:?}", error);
            }
        }
    }

    for _affected_owner in affected_owners {
        // warn!("Setup LoadChunks for {:?}", affected_owner.id());
    }

    State {
        spawn_chunk_states,
        transfer_chunk_ownership_states,
    }
}

pub fn run_ecs_while<S: Scale>(state: State<S>, main_access: MainAccess<S>) -> Outcome<State<S>, ()> {
    let chunk_query = main_access.chunk_query;

    let spawn_chunk_states = state
        .spawn_chunk_states
        .into_iter()
        .map(|mut s| {
            if chunk_query.iter().any(|chunk| chunk.coord == s.coord) {
                s.is_spawned = true;
            }
            s
        })
        .collect::<Vec<_>>();

    let transfer_chunk_ownership_states = state
        .transfer_chunk_ownership_states
        .into_iter()
        .map(|mut s| {
            if let Some(chunk) = chunk_query.iter().find(|chunk| chunk.coord == s.coord) {
                if chunk.owner_id.as_ref().expect("Unreachable state: Chunk has no owner_id") == &s.owner_id {
                    s.is_ownership_transfered = true;
                }
            }
            s
        })
        .collect::<Vec<_>>();

    let is_done = spawn_chunk_states.iter().all(|s| s.is_spawned) && transfer_chunk_ownership_states.iter().all(|s| s.is_ownership_transfered);

    if is_done {
        // let loaded_chunks_count = spawn_chunk_states.len() + transfer_chunk_ownership_states.len();
        //
        // warn!("Ran LoadChunks for # of chunks: {}", loaded_chunks_count);

        Outcome::Done(())
    } else {
        // let not_spawned: Vec<_> = spawn_chunk_states
        //     .iter()
        //     .filter(|s| !s.is_spawned)
        //     .map(|s| s.coord)
        //     .collect();
        //
        // let not_transferred: Vec<_> = transfer_chunk_ownership_states
        //     .iter()
        //     .filter(|ot| !ot.is_ownership_transfered)
        //     .map(|s| s.coord)
        //     .collect();
        //
        // if !not_spawned.is_empty() {
        //     warn!(
        //         "Waiting: {} chunks still not spawned: {:?})",
        //         not_spawned.len(),
        //         not_spawned
        //     );
        // }
        //
        // if !not_transferred.is_empty() {
        //     warn!(
        //         "Waiting: {} chunks still not transferred: {:?})",
        //         not_transferred.len(),
        //         not_transferred
        //     );
        // }

        Outcome::Wait(State {
            spawn_chunk_states,
            transfer_chunk_ownership_states,
        })
    }
}
