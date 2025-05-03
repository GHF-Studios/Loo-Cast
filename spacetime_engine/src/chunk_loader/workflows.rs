use spacetime_engine_macros::define_workflow_mod_OLD;

define_workflow_mod_OLD! {
    name: "ChunkLoader",
    workflows: [
        ValidateOnUpdateChunkLoader {
            user_imports: {
                use bevy::prelude::*;
                use std::collections::HashSet;
                
                use crate::chunk::enums::ChunkAction;
                use crate::chunk::functions::calculate_chunks_in_radius;
                use crate::chunk_loader::components::ChunkLoaderComponent;
                use crate::chunk::resources::ChunkActionBuffer;
            },
            user_items: {},
            stages: [
                Validate: Ecs {
                    core_types: [
                        struct MainAccess<'w, 's> {
                            chunk_loader_query: Query<'w, 's, (Entity, &'static Transform, &'static ChunkLoaderComponent)>,
                            chunk_action_buffer: ResMut<'w, ChunkActionBuffer>,
                        }

                        enum Error {
                            InvalidChunkActions { 
                                entries: Vec<((i32, i32), u32)>
                            },
                        }
                    ],
                    core_functions: [
                        fn RunEcs |main_access| -> Result<(), Error> {
                            let chunk_loader_query = main_access.chunk_loader_query;
                            let mut chunk_action_buffer = main_access.chunk_action_buffer;

                            for (_, transform, chunk_loader) in chunk_loader_query.iter() {
                                let position = transform.translation.truncate();
                                let radius = chunk_loader.radius;
                                let loader_range = calculate_chunks_in_radius(position, radius)
                                    .into_iter()
                                    .collect::<HashSet<(i32, i32)>>();
                        
                                let mut invalid_actions = vec![];
                                for (chunk_coord, action) in chunk_action_buffer.iter() {
                                    match action {
                                        ChunkAction::Spawn { .. } => {
                                            if !loader_range.contains(chunk_coord) {
                                                invalid_actions.push(*chunk_coord);
                                            }
                                        }
                                        ChunkAction::Despawn { .. } => {
                                            if loader_range.contains(chunk_coord) {
                                                invalid_actions.push(*chunk_coord);
                                            }
                                        }
                                        ChunkAction::TransferOwnership { .. } => {}
                                    }
                                }
                        
                                let mut invalid_chunk_actions = Vec::new();
                                for chunk_coord in invalid_actions {
                                    chunk_action_buffer.remove_action(&chunk_coord);
                                    invalid_chunk_actions.push((chunk_coord, chunk_loader.id));
                                }
                                return Err(Error::InvalidChunkActions { entries: invalid_chunk_actions });
                            }

                            Ok(())
                        }
                    ]
                }
            ]
        }

        ValidateOnRemoveChunkLoader {
            user_imports: {
                
            }
        }

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

        LoadChunks {
            user_imports: {
                use bevy::prelude::*;

                use crate::chunk::{enums::{ChunkAction, ChunkActionPriority}, resources::{ChunkManager, ChunkActionBuffer}};
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
                ValidateAndLoad: Ecs {
                    core_types: [
                        struct MainAccess<'w, 's> {
                            chunk_manager: Res<'w, ChunkManager>,
                            chunk_action_buffer: ResMut<'w, ChunkActionBuffer>,
                            phantom_data: std::marker::PhantomData<&'s ()>,
                        }
                        struct Input {
                            inputs: Vec<LoadChunkInput>,
                        }
                    ],
                    core_functions: [
                        fn RunEcs |input, main_access| {
                            let chunk_manager = main_access.chunk_manager;
                            let mut chunk_action_buffer = main_access.chunk_action_buffer;

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
                                    }
                                } else if !is_owned && !is_despawning && !is_transfering_ownership && chunk_owner.is_some() {
                                    chunk_action_buffer.add_action(ChunkAction::TransferOwnership {
                                        requester_id,
                                        coord: chunk_coord,
                                        new_owner: chunk_owner.unwrap(),
                                        priority: ChunkActionPriority::Realtime,
                                    });
                                }
                            }
                        }
                    ]
                }
            ]
        }

        UnloadChunks {
            user_imports: {
                use bevy::prelude::*;

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
                Unload: Ecs {
                    core_types: [
                        struct MainAccess<'w, 's> {
                            chunk_manager: Res<'w, ChunkManager>,
                            chunk_action_buffer: ResMut<'w, ChunkActionBuffer>,
                            chunk_query: Query<'w, 's, (Entity, &'static ChunkComponent)>,
                            chunk_loader_query: Query<'w, 's, (Entity, &'static Transform, &'static ChunkLoaderComponent)>,
                        }
                        struct Input {
                            inputs: Vec<UnloadChunkInput>,
                        }
                    ],
                    core_functions: [
                        fn RunEcs |input, main_access| {
                            let chunk_manager = main_access.chunk_manager;
                            let mut chunk_action_buffer = main_access.chunk_action_buffer;
                            let chunk_query = main_access.chunk_query;
                            let chunk_loader_query = main_access.chunk_loader_query;

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
                                        .find(|(_, chunk)| chunk.coord == chunk_coord)
                                    {
                                        Some((_, chunk)) => chunk,
                                        None => {
                                            error!(
                                                "Skipping unload for chunk '{:?}': it is already despawned",
                                                chunk_coord
                                            );
                                            return;
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
                                        }
                                    };
                                }
                            }
                        }
                    ]
                }
            ]
        }
    ]
}