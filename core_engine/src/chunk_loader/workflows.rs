use core_engine_macros::define_workflow_mod_OLD;

define_workflow_mod_OLD! {
    name: "ChunkLoader",
    workflows: [
        CategorizeChunks {
            user_imports: {
                use bevy::prelude::*;
                use std::collections::HashSet;

                use crate::chunk_loader::workflows::chunk_loader::{
                    load_chunks::user_items::LoadChunkInput,
                    unload_chunks::user_items::UnloadChunkInput
                };
                use crate::chunk_loader::components::ChunkLoaderComponent;
                use crate::chunk::resources::ChunkManager;
                use crate::chunk::intent::ActionIntent;
                use crate::chunk::functions::{world_pos_to_chunk, calculate_chunks_in_radius, calculate_chunk_distance_from_owner};
                use crate::chunk::resources::ActionIntentCommitBuffer;
            },
            user_items: {},
            stages: [
                Categorize: Ecs {
                    core_types: [
                        struct MainAccess<'w, 's> {
                            chunk_loader_query: Query<'w, 's, (&'static Transform, &'static ChunkLoaderComponent)>,
                            chunk_manager: Res<'w, ChunkManager>,
                        }

                        struct Output {
                            load_chunk_inputs: Vec<LoadChunkInput>,
                            unload_chunk_inputs: Vec<UnloadChunkInput>
                        }
                    ],
                    core_functions: [
                        fn RunEcs |main_access| -> Output {
                            let chunk_loader_query = main_access.chunk_loader_query;
                            let chunk_manager = main_access.chunk_manager;

                            let mut load_chunk_inputs = Vec::new();
                            let mut unload_chunk_inputs = Vec::new();

                            for (transform, chunk_loader) in chunk_loader_query.iter() {
                                let position = transform.translation.truncate();
                                let radius = chunk_loader.radius;
                                
                                let chunk_owner_id = chunk_loader.owner_id();
                                let chunk_owner_entity = chunk_owner_id.entity();

                                let target_chunks = calculate_chunks_in_radius(position, radius)
                                    .into_iter()
                                    .collect::<HashSet<(i32, i32)>>();

                                let current_chunks: HashSet<(i32, i32)> = chunk_manager
                                    .owned_chunks
                                    .iter()
                                    .filter_map(|(chunk, owner_id)| {
                                        if owner_id == chunk_owner_id {
                                            Some(*chunk)
                                        } else {
                                            None
                                        }
                                    })
                                    .collect();
                                
                                let chunks_to_load: Vec<_> = target_chunks.difference(&current_chunks).cloned().collect();
                                let chunks_to_unload: Vec<_> = current_chunks.difference(&target_chunks).cloned().collect();

                                for chunk_coord in chunks_to_load {
                                    let chunk_loader_distance_squared =
                                        calculate_chunk_distance_from_owner(&chunk_coord, &world_pos_to_chunk(position));
                                    let chunk_loader_radius_squared = radius * radius;

                                    load_chunk_inputs.push(crate::chunk_loader::workflows::chunk_loader::load_chunks::user_items::LoadChunkInput {
                                        owner_id: chunk_owner_id.clone(),
                                        chunk_coord,
                                        chunk_loader_distance_squared,
                                        chunk_loader_radius_squared,
                                    });
                                }

                                for chunk_coord in chunks_to_unload {
                                    warn!("omg we got chunk unloading before gta 6!!!111!!!11!!1");
                                    let chunk_loader_distance_squared =
                                        calculate_chunk_distance_from_owner(&chunk_coord, &world_pos_to_chunk(position));
                                    let chunk_loader_radius_squared = radius * radius;

                                    unload_chunk_inputs.push(crate::chunk_loader::workflows::chunk_loader::unload_chunks::user_items::UnloadChunkInput {
                                        owner_id: chunk_owner_id.clone(),
                                        chunk_coord,
                                        chunk_loader_distance_squared,
                                        chunk_loader_radius_squared,
                                    });
                                }
                            }

                            Output { load_chunk_inputs, unload_chunk_inputs }
                        }
                    ]
                }
            ]
        }

        OnRemoveChunkLoader {
            user_imports: {
                use bevy::prelude::*;

                use crate::chunk::components::ChunkComponent;
                use crate::chunk::intent::ActionIntent;
                use crate::chunk::functions::*;
                use crate::chunk::resources::{ChunkManager, ActionIntentCommitBuffer};
                use crate::chunk::types::ChunkOwnerId;
                use crate::chunk_loader::components::ChunkLoaderComponent;
                use crate::chunk_loader::workflows::chunk_loader::unload_chunks::user_items::UnloadChunkInput;
            },
            user_items: {},
            stages: [
                ExtractUnloadChunkInputs: Ecs {
                    core_types: [
                        struct MainAccess<'w, 's> {
                            chunk_manager: Res<'w, ChunkManager>,
                            phantom_data: std::marker::PhantomData<&'s ()>,
                        }
                        struct Input {
                            chunk_owner_id: ChunkOwnerId,
                            chunk_loader_position: Vec2,
                            chunk_loader_radius: u32,
                        }
                        struct Output {
                            unload_chunk_inputs: Vec<UnloadChunkInput>,
                        }
                    ],
                    core_functions: [
                        fn RunEcs |input, main_access| -> Output {
                            let chunk_manager = main_access.chunk_manager;

                            let chunk_owner_id = input.chunk_owner_id;
                            let position = input.chunk_loader_position;
                            let radius = input.chunk_loader_radius;

                            let mut unload_chunk_inputs = Vec::new();

                            let chunks_to_despawn: Vec<&(i32, i32)> = chunk_manager
                                .owned_chunks
                                .iter()
                                .filter_map(|(chunk, owner_id)| {
                                    if owner_id == &chunk_owner_id {
                                        Some(chunk)
                                    } else {
                                        None
                                    }
                                })
                                .collect();

                            for chunk_coord in chunks_to_despawn {
                                let chunk_loader_distance_squared =
                                    calculate_chunk_distance_from_owner(chunk_coord, &world_pos_to_chunk(position));
                                let chunk_loader_radius_squared = radius * radius;

                                unload_chunk_inputs.push(UnloadChunkInput {
                                    owner_id: chunk_owner_id.clone(),
                                    chunk_coord: *chunk_coord,
                                    chunk_loader_distance_squared,
                                    chunk_loader_radius_squared,
                                });
                            }
                            
                            debug!("UnloadChunkInputs on remove: {:?}", unload_chunk_inputs.len());

                            Output { unload_chunk_inputs }
                        }
                    ]
                }
            ],
        }

        LoadChunks {
            user_imports: {
                use bevy::prelude::{Entity, Res, ResMut, Query};
            
                use crate::chunk::{
                    intent::{ActionIntent, ActionPriority, resolve_intent, ResolvedActionIntent},
                    resources::{ChunkManager, ActionIntentBuffer, ActionIntentCommitBuffer},
                    components::ChunkComponent,
                    types::ChunkOwnerId,
                };
                use crate::config::statics::CONFIG;
            },
            user_items: {
                pub struct LoadChunkInput {
                    pub owner_id: ChunkOwnerId,
                    pub chunk_coord: (i32, i32),
                    pub chunk_loader_distance_squared: u32,
                    pub chunk_loader_radius_squared: u32,
                }
            
                pub struct SpawnChunkState {
                    pub coord: (i32, i32),
                    pub is_spawned: bool,
                }
            
                pub struct TransferChunkOwnershipState {
                    pub coord: (i32, i32),
                    pub owner_id: ChunkOwnerId,
                    pub is_ownership_transfered: bool,
                }
            
                pub fn calculate_spawn_priority(
                    distance_squared: u32,
                    radius_squared: u32,
                ) -> ActionPriority {
                    let normalized_distance = distance_squared as f64 / radius_squared as f64;
                    let priority_value = (i64::MAX as f64 * (1.0 - normalized_distance)) as i64;
                
                    ActionPriority::Deferred(priority_value)
                }
            },
            stages: [
                ValidateAndLoadAndWait: EcsWhile {
                    core_types: [
                        struct MainAccess<'w, 's> {
                            chunk_query: Query<'w, 's, &'static ChunkComponent>,
                            chunk_manager: Res<'w, ChunkManager>,
                            action_intent_commit_buffer: ResMut<'w, ActionIntentCommitBuffer>,
                            action_intent_buffer: ResMut<'w, ActionIntentBuffer>,
                            phantom_data: std::marker::PhantomData<&'s ()>,
                        }
                        struct Input {
                            inputs: Vec<LoadChunkInput>,
                        }
                        struct State {
                            spawn_chunk_states: Vec<SpawnChunkState>,
                            transfer_chunk_ownership_states: Vec<TransferChunkOwnershipState>,
                        }
                    ],
                    core_functions: [
                        fn SetupEcsWhile |input, main_access| -> State {
                            let chunk_manager = main_access.chunk_manager;
                            let mut action_intent_commit_buffer = main_access.action_intent_commit_buffer;
                            let mut action_intent_buffer = main_access.action_intent_buffer;
                        
                            let mut spawn_chunk_states = Vec::new();
                            let mut transfer_chunk_ownership_states = Vec::new();
                        
                            for input in input.inputs {
                                let owner_id = input.owner_id;
                                let coord = input.chunk_coord;
                            
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
                                        priority: calculate_spawn_priority(
                                            input.chunk_loader_distance_squared,
                                            input.chunk_loader_radius_squared,
                                        ),
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
                                        ActionIntent::Spawn { .. } => {
                                            action_intent_commit_buffer.commit_intent(action);
                                            spawn_chunk_states.push(SpawnChunkState { coord, is_spawned: false });
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
                                        action_intent_buffer.cancel_intent(&coord);
                                    }
                                    ResolvedActionIntent::DiscardIncoming(reason) => {
                                        // Optionally: log warning or metrics here
                                        continue;
                                    }
                                    ResolvedActionIntent::Error(error) => {
                                        panic!("Intent resolution failed: {:?}", error);
                                    }
                                }
                            }
                        
                            State {
                                spawn_chunk_states,
                                transfer_chunk_ownership_states,
                            }
                        }
                    
                        fn RunEcsWhile |state, main_access| -> Outcome<State, ()> {
                            let chunk_query = main_access.chunk_query;
                        
                            let spawn_chunk_states = state.spawn_chunk_states.into_iter().map(|mut s| {
                                if chunk_query.iter().any(|chunk| chunk.coord == s.coord) {
                                    s.is_spawned = true;
                                }
                                s
                            }).collect::<Vec<_>>();
                        
                            let transfer_chunk_ownership_states = state.transfer_chunk_ownership_states.into_iter().map(|mut s| {
                                if let Some(chunk) = chunk_query.iter().find(|chunk| chunk.coord == s.coord) {
                                    if chunk.owner_id.as_ref().expect("Unreachable state: Chunk has no owner_id") == &s.owner_id {
                                        s.is_ownership_transfered = true;
                                    }
                                }
                                s
                            }).collect::<Vec<_>>();
                        
                            let is_done = spawn_chunk_states.iter().all(|s| s.is_spawned)
                                && transfer_chunk_ownership_states.iter().all(|s| s.is_ownership_transfered);
                        
                            if is_done {
                                Outcome::Done(())
                            } else {
                                Outcome::Wait(State {
                                    spawn_chunk_states,
                                    transfer_chunk_ownership_states,
                                })
                            }
                        }
                    ]
                }
            ]
        }

        UnloadChunks {
            user_imports: {
                use bevy::prelude::{Res, ResMut, Entity, Transform, Query, Vec2};
            
                use crate::chunk::{
                    components::ChunkComponent,
                    intent::{ActionIntent, ActionPriority, resolve_intent, ResolvedActionIntent, State as ChunkState},
                    resources::{ChunkManager, ActionIntentBuffer, ActionIntentCommitBuffer},
                    functions::world_pos_to_chunk,
                    types::ChunkOwnerId,
                };
                use crate::chunk_loader::components::ChunkLoaderComponent;
            },
            user_items: {
                pub struct UnloadChunkInput {
                    pub owner_id: ChunkOwnerId,
                    pub chunk_coord: (i32, i32),
                    pub chunk_loader_distance_squared: u32,
                    pub chunk_loader_radius_squared: u32,
                }
            
                pub struct DespawnChunkState {
                    pub coord: (i32, i32),
                    pub is_despawned: bool,
                }
            
                pub struct TransferChunkOwnershipState {
                    pub coord: (i32, i32),
                    pub owner_id: ChunkOwnerId,
                    pub is_ownership_transfered: bool,
                }
            
                pub fn calculate_despawn_priority(distance_squared: u32, radius_squared: u32) -> ActionPriority {
                    let normalized_distance = distance_squared as f64 / radius_squared as f64;
                    let priority_value = (normalized_distance * i64::MAX as f64) as i64;
                    ActionPriority::Deferred(priority_value)
                }
            
                pub fn is_chunk_in_loader_range(chunk_coord: &(i32, i32), loader_position: Vec2, loader_radius: u32) -> bool {
                    let (loader_chunk_x, loader_chunk_y) = world_pos_to_chunk(loader_position);
                    let dx = chunk_coord.0 - loader_chunk_x;
                    let dy = chunk_coord.1 - loader_chunk_y;
                    let distance_squared = dx * dx + dy * dy;
                    let radius_squared = (loader_radius as i32) * (loader_radius as i32);
                    distance_squared <= radius_squared
                }
            },
            stages: [
                UnloadAndWait: EcsWhile {
                    core_types: [
                        struct MainAccess<'w, 's> {
                            chunk_manager: Res<'w, ChunkManager>,
                            action_intent_commit_buffer: ResMut<'w, ActionIntentCommitBuffer>,
                            action_intent_buffer: ResMut<'w, ActionIntentBuffer>,
                            chunk_query: Query<'w, 's, &'static ChunkComponent>,
                            chunk_loader_query: Query<'w, 's, (&'static Transform, &'static ChunkLoaderComponent)>,
                        }
                        struct Input {
                            inputs: Vec<UnloadChunkInput>,
                        }
                        struct State {
                            despawn_chunk_states: Vec<DespawnChunkState>,
                            transfer_chunk_ownership_states: Vec<TransferChunkOwnershipState>,
                        }
                    ],
                    core_functions: [
                        fn SetupEcsWhile |input, main_access| -> State {
                            let chunk_manager = main_access.chunk_manager;
                            let mut action_intent_commit_buffer = main_access.action_intent_commit_buffer;
                            let mut action_intent_buffer = main_access.action_intent_buffer;
                            let chunk_query = main_access.chunk_query;
                            let chunk_loader_query = main_access.chunk_loader_query;
                        
                            let mut despawn_chunk_states = Vec::new();
                            let mut transfer_chunk_ownership_states = Vec::new();
                        
                            for input in input.inputs {
                                let owner_id = input.owner_id;
                                let coord = input.chunk_coord;
                                let distance_squared = input.chunk_loader_distance_squared;
                                let radius_squared = input.chunk_loader_radius_squared;
                            
                                let is_loaded = chunk_manager.is_loaded(&coord);
                                if !is_loaded {
                                    continue;
                                }
                            
                                let committed = action_intent_commit_buffer.get(&coord);
                                let buffered = action_intent_buffer.get(&coord);
                            
                                let chunk_state = if let Some(owner_id) = chunk_manager.owned_chunks.get(&coord) {
                                    ChunkState::Owned(owner_id.clone())
                                } else {
                                    unreachable!("Unreachable state: Chunk is absent")
                                };
                            
                                let chunk = match chunk_query.iter().find(|chunk| chunk.coord == coord) {
                                    Some(c) => c,
                                    None => unreachable!("Unreachable state: Chunk is absent"),
                                };
                            
                                let transfer_candidate = chunk_loader_query
                                    .iter()
                                    .find_map(|(transform, loader)| {
                                        if loader.owner_id() == chunk.owner_id() {
                                            return None;
                                        }

                                        if is_chunk_in_loader_range(&coord, transform.translation.truncate(), loader.radius) {
                                            Some(loader.owner_id())
                                        } else {
                                            None
                                        }
                                    });
                                
                                let proposed_intent = match transfer_candidate {
                                    Some(new_owner_id) => ActionIntent::TransferOwnership {
                                        new_owner_id: new_owner_id.clone(),
                                        coord,
                                        priority: ActionPriority::Realtime,
                                    },
                                    None => ActionIntent::Despawn {
                                        owner_id,
                                        coord,
                                        priority: calculate_despawn_priority(distance_squared, radius_squared),
                                    },
                                };
                            
                                let resolution = resolve_intent(&chunk_state, committed, buffered, proposed_intent.clone());
                            
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
                                        continue;
                                    }
                                    ResolvedActionIntent::Error(error) => {
                                        panic!("UnloadChunks resolution error: {:?}", error);
                                    }
                                }
                            }
                        
                            State {
                                despawn_chunk_states,
                                transfer_chunk_ownership_states,
                            }
                        }
                    
                        fn RunEcsWhile |state, main_access| -> Outcome<State, ()> {
                            let chunk_query = main_access.chunk_query;
                        
                            let despawn_chunk_states = state.despawn_chunk_states.into_iter().map(|mut s| {
                                if chunk_query.iter().all(|chunk| chunk.coord != s.coord) {
                                    s.is_despawned = true;
                                }
                                s
                            }).collect::<Vec<_>>();
                        
                            let transfer_chunk_ownership_states = state.transfer_chunk_ownership_states.into_iter().map(|mut s| {
                                if let Some(chunk) = chunk_query.iter().find(|chunk| chunk.coord == s.coord) {
                                    if *chunk.owner_id() == s.owner_id {
                                        s.is_ownership_transfered = true;
                                    }
                                }
                                s
                            }).collect::<Vec<_>>();
                        
                            let is_done = despawn_chunk_states.iter().all(|s| s.is_despawned)
                                && transfer_chunk_ownership_states.iter().all(|s| s.is_ownership_transfered);
                        
                            if is_done {
                                Outcome::Done(())
                            } else {
                                Outcome::Wait(State {
                                    despawn_chunk_states,
                                    transfer_chunk_ownership_states,
                                })
                            }
                        }
                    ]
                }
            ]
        }
    ]
}
