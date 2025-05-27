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
                use crate::chunk::enums::ChunkAction;
                use crate::chunk::functions::{world_pos_to_chunk, calculate_chunks_in_radius, calculate_chunk_distance_from_owner};
                use crate::chunk::resources::ChunkActionBuffer;
            },
            user_items: {},
            stages: [
                Categorize: Ecs {
                    core_types: [
                        struct MainAccess<'w, 's> {
                            chunk_loader_query: Query<'w, 's, (Entity, &'static Transform, &'static ChunkLoaderComponent)>,
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

                            for (loader_entity, transform, chunk_loader) in chunk_loader_query.iter() {
                                let position = transform.translation.truncate();
                                let radius = chunk_loader.radius;

                                let target_chunks = calculate_chunks_in_radius(position, radius)
                                    .into_iter()
                                    .collect::<HashSet<(i32, i32)>>();

                                let current_chunks: HashSet<(i32, i32)> = chunk_manager
                                    .owned_chunks
                                    .iter()
                                    .filter_map(|(chunk, &owner)| {
                                        if owner == loader_entity {
                                            Some(*chunk)
                                        } else {
                                            None
                                        }
                                    })
                                    .collect();

                                let chunks_to_load: Vec<&(i32, i32)> = target_chunks.difference(&current_chunks).collect();
                                let chunks_to_unload: Vec<&(i32, i32)> =
                                    current_chunks.difference(&target_chunks).collect();

                                for chunk_coord in chunks_to_load {
                                    let chunk_loader_distance_squared =
                                        calculate_chunk_distance_from_owner(chunk_coord, &world_pos_to_chunk(position));
                                    let chunk_loader_radius_squared = radius * radius;

                                    load_chunk_inputs.push(crate::chunk_loader::workflows::chunk_loader::load_chunks::user_items::LoadChunkInput {
                                        requester_id: chunk_loader.id,
                                        chunk_coord: *chunk_coord,
                                        chunk_owner: Some(loader_entity),
                                        chunk_loader_distance_squared,
                                        chunk_loader_radius_squared,
                                    });
                                }

                                for chunk_coord in chunks_to_unload {
                                    let chunk_loader_distance_squared =
                                        calculate_chunk_distance_from_owner(chunk_coord, &world_pos_to_chunk(position));
                                    let chunk_loader_radius_squared = radius * radius;

                                    unload_chunk_inputs.push(crate::chunk_loader::workflows::chunk_loader::unload_chunks::user_items::UnloadChunkInput {
                                        requester_id: chunk_loader.id,
                                        chunk_coord: *chunk_coord,
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
                use crate::chunk::enums::ChunkAction;
                use crate::chunk::functions::*;
                use crate::chunk::resources::{ChunkManager, ChunkActionBuffer};
                use crate::chunk_loader::components::ChunkLoaderComponent;
                use crate::chunk_loader::workflows::chunk_loader::unload_chunks::user_items::UnloadChunkInput;
            },
            user_items: {},
            stages: [
                ExtractUnloadChunkInputs: Ecs {
                    core_types: [
                        struct MainAccess<'w, 's> {
                            chunk_manager: Res<'w, ChunkManager>,
                            chunk_action_buffer: ResMut<'w, ChunkActionBuffer>,
                            phantom_data: std::marker::PhantomData<&'s ()>,
                        }
                        struct Input {
                            chunk_loader_entity: Entity,
                            chunk_loader_id: u32,
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
                            let mut chunk_action_buffer = main_access.chunk_action_buffer;

                            let loader_entity = input.chunk_loader_entity;
                            let loader_id = input.chunk_loader_id;
                            let position = input.chunk_loader_position;
                            let radius = input.chunk_loader_radius;

                            let mut invalid_actions = vec![];
                            for (chunk_coord, action) in chunk_action_buffer
                                .iter()
                                .filter(|(_, action)| action.get_requester_id() == loader_id)
                            {
                                match action {
                                    ChunkAction::Spawn { .. } => {
                                        invalid_actions.push(*chunk_coord);
                                    }
                                    ChunkAction::Despawn { .. } => {}
                                    ChunkAction::TransferOwnership { .. } => {}
                                }
                            }

                            let mut invalid_chunk_actions = Vec::new();

                            #[allow(clippy::never_loop)]
                            for chunk_coord in invalid_actions {
                                chunk_action_buffer.remove_action(&chunk_coord);
                                invalid_chunk_actions.push((chunk_coord, loader_id));
                                unreachable!("Invalid ChunkActions deteced OnUpdate: {:?}", invalid_chunk_actions);
                            }

                            let chunks_to_despawn: Vec<&(i32, i32)> = chunk_manager
                                .owned_chunks
                                .iter()
                                .filter_map(|(chunk, &owner)| {
                                    if owner == loader_entity {
                                        chunk_action_buffer.remove_action(chunk);

                                        Some(chunk)
                                    } else {
                                        None
                                    }
                                })
                                .collect();

                            let mut unload_chunk_inputs = Vec::new();
                            for chunk_coord in chunks_to_despawn {
                                let chunk_loader_distance_squared =
                                    calculate_chunk_distance_from_owner(chunk_coord, &world_pos_to_chunk(position));
                                let chunk_loader_radius_squared = radius * radius;

                                unload_chunk_inputs.push(UnloadChunkInput {
                                    requester_id: loader_id,
                                    chunk_coord: *chunk_coord,
                                    chunk_loader_distance_squared,
                                    chunk_loader_radius_squared,
                                });
                            }

                            Output { unload_chunk_inputs }
                        }
                    ]
                }
            ],
        }

        LoadChunks {
            user_imports: {
                use bevy::prelude::{Entity, Res, ResMut, Query};

                use crate::chunk::{enums::{ChunkAction, ChunkActionPriority}, resources::{ChunkManager, ChunkActionBuffer}, components::ChunkComponent};
                use crate::config::statics::CONFIG;
            },
            user_items: {
                pub struct LoadChunkInput {
                    pub requester_id: u32,
                    pub chunk_coord: (i32, i32),
                    pub chunk_owner: Option<Entity>,
                    pub chunk_loader_distance_squared: u32,
                    pub chunk_loader_radius_squared: u32,
                }
                pub struct SpawnChunkState {
                    pub coord: (i32, i32),
                    pub is_spawned: bool,
                }
                pub struct TransferChunkOwnershipState {
                    pub coord: (i32, i32),
                    pub owner: Option<Entity>,
                    pub is_ownership_transfered: bool,
                }

                pub fn calculate_spawn_priority(
                    distance_squared: u32,
                    radius_squared: u32,
                    has_pending_despawn: bool,
                ) -> ChunkActionPriority {
                    let normalized_distance = distance_squared as f64 / radius_squared as f64;

                    // Lower priority if a despawn is pending
                    let adjustment = if has_pending_despawn { 0.5 } else { 1.0 };
                    let priority_value = (i64::MAX as f64 * (1.0 - normalized_distance) * adjustment) as i64;

                    ChunkActionPriority::Deferred(priority_value)
                }
            },
            stages: [
                ValidateAndLoadAndWait: EcsWhile {
                    core_types: [
                        struct MainAccess<'w, 's> {
                            chunk_query: Query<'w, 's, &'static ChunkComponent>,
                            chunk_manager: Res<'w, ChunkManager>,
                            chunk_action_buffer: ResMut<'w, ChunkActionBuffer>,
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
                            let mut chunk_action_buffer = main_access.chunk_action_buffer;

                            let mut spawn_chunk_states = Vec::new();
                            let mut transfer_chunk_ownership_states = Vec::new();

                            for input in input.inputs {
                                let requester_id = input.requester_id;
                                let chunk_coord = input.chunk_coord;
                                let chunk_owner = input.chunk_owner;
                                let chunk_loader_distance_squared = input.chunk_loader_distance_squared;
                                let chunk_loader_radius_squared = input.chunk_loader_radius_squared;

                                let is_loaded = chunk_manager.loaded_chunks.contains(&chunk_coord);
                                let is_owned = chunk_manager.owned_chunks.contains_key(&chunk_coord);
                                let (is_spawning, is_despawning, is_transfering_ownership) =
                                    chunk_action_buffer.get_action_states(&chunk_coord);

                                if !is_loaded {
                                    if !is_spawning && !is_despawning && !is_transfering_ownership {
                                        let has_pending_despawn = chunk_action_buffer.has_despawns();

                                        chunk_action_buffer.add_action(ChunkAction::Spawn {
                                            requester_id,
                                            coord: chunk_coord,
                                            new_owner: chunk_owner,
                                            priority: calculate_spawn_priority(
                                                chunk_loader_distance_squared,
                                                chunk_loader_radius_squared,
                                                has_pending_despawn,
                                            ),
                                        });
                                        spawn_chunk_states.push(SpawnChunkState {
                                            coord: chunk_coord,
                                            is_spawned: false
                                        });
                                    }
                                } else if !is_owned && !is_despawning && !is_transfering_ownership && chunk_owner.is_some() {
                                    chunk_action_buffer.add_action(ChunkAction::TransferOwnership {
                                        requester_id,
                                        coord: chunk_coord,
                                        new_owner: chunk_owner.unwrap(),
                                        priority: ChunkActionPriority::Realtime,
                                    });
                                    transfer_chunk_ownership_states.push(TransferChunkOwnershipState {
                                        coord: chunk_coord,
                                        owner: chunk_owner,
                                        is_ownership_transfered: false
                                    });
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
                                    if chunk.owner == s.owner {
                                        s.is_ownership_transfered = true;
                                    }
                                }

                                s
                            }).collect::<Vec<_>>();
                            let is_done = spawn_chunk_states.iter().all(|s| s.is_spawned) &&
                                transfer_chunk_ownership_states.iter().all(|s| s.is_ownership_transfered);

                            if is_done {
                                Outcome::Done(())
                            } else {
                                Outcome::Wait(State {
                                    spawn_chunk_states,
                                    transfer_chunk_ownership_states
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

                use crate::chunk::{components::ChunkComponent, enums::{ChunkAction, ChunkActionPriority}, resources::{ChunkManager, ChunkActionBuffer}, functions::world_pos_to_chunk};
                use crate::chunk_loader::components::ChunkLoaderComponent;
            },
            user_items: {
                pub struct UnloadChunkInput {
                    pub requester_id: u32,
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
                    pub owner: Option<Entity>,
                    pub is_ownership_transfered: bool,
                }

                pub fn calculate_despawn_priority(distance_squared: u32, radius_squared: u32) -> ChunkActionPriority {
                    let normalized_distance = distance_squared as f64 / radius_squared as f64;
                    let priority_value = (normalized_distance * i64::MAX as f64) as i64;

                    ChunkActionPriority::Deferred(priority_value)
                }

                pub fn is_chunk_in_loader_range(
                    chunk_coord: &(i32, i32),
                    loader_position: Vec2,
                    loader_radius: u32,
                ) -> bool {
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
                            chunk_action_buffer: ResMut<'w, ChunkActionBuffer>,
                            chunk_query: Query<'w, 's, &'static ChunkComponent>,
                            chunk_loader_query: Query<'w, 's, (Entity, &'static Transform, &'static ChunkLoaderComponent)>,
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
                            let mut chunk_action_buffer = main_access.chunk_action_buffer;
                            let chunk_query = main_access.chunk_query;
                            let chunk_loader_query = main_access.chunk_loader_query;

                            let mut despawn_chunk_states = Vec::new();
                            let mut transfer_chunk_ownership_states = Vec::new();

                            for input in input.inputs {
                                let requester_id = input.requester_id;
                                let chunk_coord = input.chunk_coord;
                                let chunk_loader_distance_squared = input.chunk_loader_distance_squared;
                                let chunk_loader_radius_squared = input.chunk_loader_radius_squared;

                                let is_loaded = chunk_manager.is_loaded(&chunk_coord);
                                let (is_spawning, is_despawning, is_transfering_ownership) =
                                    chunk_action_buffer.get_action_states(&chunk_coord);

                                if is_loaded && !is_spawning && !is_despawning && !is_transfering_ownership {
                                    let chunk = match chunk_query
                                        .iter()
                                        .find(|chunk| chunk.coord == chunk_coord)
                                    {
                                        Some(chunk) => chunk,
                                        None => {
                                            bevy::prelude::error!(
                                                "Skipping unload for chunk '{:?}': it is already despawned",
                                                chunk_coord
                                            );
                                            continue;
                                        }
                                    };

                                    match chunk_loader_query
                                        .iter()
                                        .find(|(loader_entity, transform, loader)| {
                                            if chunk
                                                .owner
                                                .is_some_and(|chunk_owner| chunk_owner == *loader_entity)
                                            {
                                                return false;
                                            }

                                            is_chunk_in_loader_range(
                                                &chunk_coord,
                                                transform.translation.truncate(),
                                                loader.radius,
                                            )
                                        }) {
                                        Some((new_owner, _, _)) => {
                                            chunk_action_buffer.add_action(ChunkAction::TransferOwnership {
                                                requester_id,
                                                coord: chunk_coord,
                                                new_owner,
                                                priority: ChunkActionPriority::Realtime,
                                            });
                                            transfer_chunk_ownership_states.push(TransferChunkOwnershipState {
                                                coord: chunk_coord,
                                                owner: Some(new_owner),
                                                is_ownership_transfered: false
                                            });
                                        }
                                        None => {
                                            chunk_action_buffer.add_action(ChunkAction::Despawn {
                                                requester_id,
                                                coord: chunk_coord,
                                                priority: calculate_despawn_priority(
                                                    chunk_loader_distance_squared,
                                                    chunk_loader_radius_squared,
                                                ),
                                            });
                                            despawn_chunk_states.push(DespawnChunkState {
                                                coord: chunk_coord,
                                                is_despawned: false
                                            });
                                        }
                                    };
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
                                if chunk_query.iter().any(|chunk| chunk.coord == s.coord) {
                                    s.is_despawned = true;
                                }

                                s
                            }).collect::<Vec<_>>();
                            let transfer_chunk_ownership_states = state.transfer_chunk_ownership_states.into_iter().map(|mut s| {
                                if let Some(chunk) = chunk_query.iter().find(|chunk| chunk.coord == s.coord) {
                                    if chunk.owner == s.owner {
                                        s.is_ownership_transfered = true;
                                    }
                                }

                                s
                            }).collect::<Vec<_>>();
                            let is_done = despawn_chunk_states.iter().all(|s| s.is_despawned) &&
                                transfer_chunk_ownership_states.iter().all(|s| s.is_ownership_transfered);

                            if is_done {
                                Outcome::Done(())
                            } else {
                                Outcome::Wait(State {
                                    despawn_chunk_states,
                                    transfer_chunk_ownership_states
                                })
                            }
                        }
                    ]
                }
            ]
        }
    ]
}
