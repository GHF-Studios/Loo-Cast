use core_engine_macros::define_workflow_mod_OLD;

define_workflow_mod_OLD! {
    name: "Chunk",
    workflows: [
        SpawnChunks {
            user_imports: {
                use bevy::prelude::{Commands, Entity, Query, Res, ResMut, Handle, Image, Transform, SpriteBundle};

                use crate::chunk::{components::ChunkComponent, resources::ChunkManager, functions::chunk_pos_to_world, types::ChunkOwnerId};
                use crate::config::statics::CONFIG;
            },
            user_items: {
                pub struct SpawnChunkInput {
                    pub chunk_coord: (i32, i32),
                    pub chunk_owner_id: ChunkOwnerId,
                    pub metric_texture: Handle<Image>
                }
                #[derive(Clone)]
                pub struct SpawnChunkState {
                    pub chunk_entity: Entity,
                    pub is_spawned: bool,
                }
            },
            stages: [
                ValidateAndSpawnAndWait: EcsWhile {
                    core_types: [
                        struct MainAccess<'w, 's> {
                            commands: Commands<'w, 's>,
                            chunk_query: Query<'w, 's, &'static ChunkComponent>,
                            chunk_manager: ResMut<'w, ChunkManager>,
                        }
                        struct Input {
                            inputs: Vec<SpawnChunkInput>,
                        }
                        struct State {
                            spawn_chunk_states: Vec<SpawnChunkState>,
                        }
                        struct Output {
                            spawned_chunk_entities: Vec<Entity>,
                        }
                        enum Error {
                            ChunkAlreadyLoaded { chunk_coord: (i32, i32) },
                        }
                    ],
                    core_functions: [
                        fn SetupEcsWhile |input, main_access| -> Result<State, Error> {
                            let mut commands = main_access.commands;
                            let chunk_query = main_access.chunk_query;
                            let mut chunk_manager = main_access.chunk_manager;

                            let mut spawn_chunk_states = Vec::new();

                            for input in input.inputs {
                                let chunk_coord = input.chunk_coord;
                                let chunk_owner_id = input.chunk_owner_id;
                                let metric_texture = input.metric_texture.clone();

                                if chunk_query.iter().any(|chunk| chunk.coord == chunk_coord) {
                                    return Err(Error::ChunkAlreadyLoaded { chunk_coord });
                                }

                                let default_chunk_z = CONFIG.get::<f32>("chunk/default_z");

                                let chunk_transform = Transform {
                                    translation: chunk_pos_to_world(chunk_coord).extend(default_chunk_z),
                                    ..Default::default()
                                };

                                commands.entity(chunk_owner_id.entity()).insert((
                                    SpriteBundle {
                                        texture: metric_texture,
                                        transform: chunk_transform,
                                        ..Default::default()
                                    },
                                    ChunkComponent {
                                        coord: chunk_coord,
                                        owner_id: Some(chunk_owner_id.clone()),
                                    },
                                )).id();

                                chunk_manager.loaded_chunks.insert(chunk_coord);
                                chunk_manager.owned_chunks.insert(chunk_coord, chunk_owner_id.clone());

                                spawn_chunk_states.push(SpawnChunkState {
                                    chunk_entity: chunk_owner_id.entity(),
                                    is_spawned: false,
                                });
                            }

                            Ok(State {
                                spawn_chunk_states
                            })
                        }

                        fn RunEcsWhile |state, main_access| -> Result<Outcome<State, Output>, Error> {
                            let mut commands = main_access.commands;

                            let spawn_chunk_states = state.spawn_chunk_states.into_iter().map(|mut spawn_chunk_state| {
                                if commands.get_entity(spawn_chunk_state.chunk_entity).is_some() {
                                    spawn_chunk_state.is_spawned = true;
                                }

                                spawn_chunk_state
                            }).collect::<Vec<_>>();
                            let is_done = spawn_chunk_states.iter().all(|spawn_chunk_state| spawn_chunk_state.is_spawned);

                            if is_done {
                                let spawned_chunk_entities = spawn_chunk_states.into_iter().map(|spawn_chunk_state| spawn_chunk_state.chunk_entity).collect();

                                Ok(Outcome::Done(Output {
                                    spawned_chunk_entities
                                }))
                            } else {
                                Ok(Outcome::Wait(State {
                                    spawn_chunk_states
                                }))
                            }
                        }
                    ]
                }
            ]
        }

        DespawnChunks {
            user_imports: {
                use bevy::prelude::{Res, ResMut, Commands, Query, Entity, DespawnRecursiveExt};

                use crate::chunk::{components::ChunkComponent, resources::ChunkManager};
            },
            user_items: {
                pub struct DespawnChunkInput {
                    pub chunk_coord: (i32, i32)
                }
                #[derive(Clone)]
                pub struct DespawnChunkState {
                    pub entity: Entity,
                    pub is_despawned: bool,
                }
            },
            stages: [
                FindAndDespawnAndWait: EcsWhile {
                    core_types: [
                        struct MainAccess<'w, 's> {
                            commands: Commands<'w, 's>,
                            chunk_query: Query<'w, 's, (Entity, &'static ChunkComponent)>,
                            chunk_manager: ResMut<'w, ChunkManager>,
                        }
                        struct Input {
                            inputs: Vec<DespawnChunkInput>,
                        }
                        struct State {
                            despawn_chunk_states: Vec<DespawnChunkState>
                        }
                        struct Output {
                            despawned_chunk_entities: Vec<Entity>
                        }
                        enum Error {
                            ChunkNotLoaded { chunk_coord: (i32, i32) },
                        }
                    ],
                    core_functions: [
                        fn SetupEcsWhile |input, main_access| -> Result<State, Error> {
                            let mut commands = main_access.commands;
                            let chunk_query = main_access.chunk_query;
                            let mut chunk_manager = main_access.chunk_manager;

                            let mut despawn_chunk_states = Vec::new();

                            for input in input.inputs {
                                let chunk_coord = input.chunk_coord;

                                if let Some((entity, _)) = chunk_query.iter().find(|(_, chunk)| chunk.coord == chunk_coord) {
                                    chunk_manager.loaded_chunks.remove(&chunk_coord);
                                    chunk_manager.owned_chunks.remove(&chunk_coord);

                                    let chunk_entity_commands = commands.entity(entity);
                                    despawn_chunk_states.push(DespawnChunkState {
                                        entity: chunk_entity_commands.id(),
                                        is_despawned: false,
                                    });
                                    chunk_entity_commands.despawn_recursive();
                                } else {
                                    return Err(Error::ChunkNotLoaded { chunk_coord });
                                }
                            }

                            Ok(State {
                                despawn_chunk_states
                            })
                        }

                        fn RunEcsWhile |state, main_access| -> Result<Outcome<State, Output>, Error> {
                            let mut commands = main_access.commands;

                            let despawn_chunk_states = state.despawn_chunk_states.into_iter().map(|mut despawn_chunk_state| {
                                if commands.get_entity(despawn_chunk_state.entity).is_none() {
                                    despawn_chunk_state.is_despawned = true;
                                }

                                despawn_chunk_state
                            }).collect::<Vec<_>>();
                            let is_done = despawn_chunk_states.iter().all(|despawn_chunk_state| despawn_chunk_state.is_despawned);

                            if is_done {
                                let despawned_chunk_entities = despawn_chunk_states.into_iter().map(|despawn_chunk_state| despawn_chunk_state.entity).collect();

                                Ok(Outcome::Done(Output {
                                    despawned_chunk_entities
                                }))
                            } else {
                                Ok(Outcome::Wait(State {
                                    despawn_chunk_states
                                }))
                            }
                        }
                    ]
                }
            ]
        }

        TransferChunkOwnerships {
            user_imports: {
                use bevy::prelude::{Res, ResMut, Entity};

                use crate::chunk::{components::ChunkComponent, resources::ChunkManager, types::ChunkOwnerId};
            },
            user_items: {
                pub struct TransferChunkOwnershipInput {
                    pub new_chunk_owner_id: ChunkOwnerId,
                    pub chunk_coord: (i32, i32),
                }
            },
            stages: [
                FindAndTransferOwnership: Ecs {
                    core_types: [
                        struct MainAccess<'w, 's> {
                            chunk_query: Query<'w, 's, (Entity, &'static mut ChunkComponent)>,
                            chunk_manager: ResMut<'w, ChunkManager>,
                        }
                        struct Input {
                            inputs: Vec<TransferChunkOwnershipInput>,
                        }
                        struct Output {
                            ownership_transfered_chunk_entities: Vec<Entity>
                        }
                        enum Error {
                            ChunkNotLoaded { chunk_coord: (i32, i32) },
                        }
                    ],
                    core_functions: [
                        fn RunEcs |input, main_access| -> Result<Output, Error> {
                            let mut chunk_query = main_access.chunk_query;
                            let mut chunk_manager = main_access.chunk_manager;

                            let mut chunk_entities = Vec::new();

                            for input in input.inputs {
                                let new_chunk_owner_id = input.new_chunk_owner_id;
                                let chunk_coord = input.chunk_coord;

                                if let Some((entity, mut chunk)) = chunk_query.iter_mut().find(|(_, chunk)| chunk.coord == chunk_coord) {
                                    if chunk.owner_id.is_some() {
                                        chunk_manager.owned_chunks.remove(&chunk_coord);
                                    }
                                    chunk.owner_id = Some(new_chunk_owner_id.clone());
                                    chunk_manager.owned_chunks.insert(chunk_coord, new_chunk_owner_id);

                                    chunk_entities.push(entity);
                                } else {
                                    return Err(Error::ChunkNotLoaded { chunk_coord });
                                }
                            }

                            Ok(Output {
                                ownership_transfered_chunk_entities: chunk_entities
                            })
                        }
                    ]
                }
            ]
        }
    ]
}
