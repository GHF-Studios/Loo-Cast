use spacetime_engine_macros::define_workflow_mod_OLD;

define_workflow_mod_OLD! {
    name: "Chunk",
    workflows: [
        SpawnChunk {
            user_imports: {
                use bevy::prelude::*;

                use crate::chunk::{components::ChunkComponent, resources::ChunkManager, functions::chunk_pos_to_world};
                use crate::config::statics::CONFIG;
            },
            user_items: {},
            stages: [
                ValidateAndSpawn: Ecs {
                    core_types: [
                        struct MainAccess<'w, 's> {
                            commands: Commands<'w, 's>,
                            chunk_query: Query<'w, 's, &'static ChunkComponent>,
                            chunk_manager: ResMut<'w, ChunkManager>,
                        }
                        struct Input {
                            chunk_coord: (i32, i32),
                            chunk_owner: Option<Entity>,
                            metric_texture: Handle<Image>
                        }
                        enum Error {
                            ChunkAlreadyLoaded { chunk_coord: (i32, i32) },
                        }
                    ],
                    core_functions: [
                        fn RunEcs |input, main_access| -> Result<(), Error> {
                            let chunk_coord = input.chunk_coord;
                            let chunk_owner = input.chunk_owner;
                            let metric_texture = input.metric_texture.clone();

                            let mut commands = main_access.commands;
                            let chunk_query = main_access.chunk_query;
                            let mut chunk_manager = main_access.chunk_manager;

                            if chunk_query.iter().any(|chunk| chunk.coord == chunk_coord) {
                                return Err(Error::ChunkAlreadyLoaded { chunk_coord });
                            }

                            let default_chunk_z = CONFIG.get::<f32>("chunk/default_z");
                            let chunk_transform = Transform {
                                translation: chunk_pos_to_world(chunk_coord).extend(default_chunk_z),
                                ..Default::default()
                            };

                            commands.spawn((
                                SpriteBundle {
                                    texture: metric_texture,
                                    transform: chunk_transform,
                                    ..Default::default()
                                },
                                ChunkComponent {
                                    coord: chunk_coord,
                                    owner: chunk_owner,
                                },
                            ));

                            chunk_manager.loaded_chunks.insert(chunk_coord);
                            if let Some(owner) = chunk_owner {
                                chunk_manager.owned_chunks.insert(chunk_coord, owner);
                            }

                            Ok(())
                        }
                    ]
                }
            ]
        }

        DespawnChunk {
            user_imports: {
                use bevy::prelude::*;

                use crate::chunk::{components::ChunkComponent, resources::ChunkManager};
            },
            user_items: {},
            stages: [
                FindAndDespawn: Ecs {
                    core_types: [
                        struct MainAccess<'w, 's> {
                            commands: Commands<'w, 's>,
                            chunk_query: Query<'w, 's, (Entity, &'static ChunkComponent)>,
                            chunk_manager: ResMut<'w, ChunkManager>,
                        }
                        struct Input {
                            chunk_coord: (i32, i32)
                        }
                        enum Error {
                            ChunkNotLoaded { chunk_coord: (i32, i32) },
                        }
                    ],
                    core_functions: [
                        fn RunEcs |input, main_access| -> Result<(), Error> {
                            let chunk_coord = input.chunk_coord;
                            
                            let mut commands = main_access.commands;
                            let chunk_query = main_access.chunk_query;
                            let mut chunk_manager = main_access.chunk_manager;

                            if let Some((entity, _)) = chunk_query.iter().find(|(_, chunk)| chunk.coord == chunk_coord) {
                                chunk_manager.loaded_chunks.remove(&chunk_coord);
                                chunk_manager.owned_chunks.remove(&chunk_coord);

                                commands.entity(entity).despawn_recursive();

                                Ok(())
                            } else {
                                Err(Error::ChunkNotLoaded { chunk_coord })
                            }
                        }
                    ]
                }
            ]
        }

        TransferChunkOwnership {
            user_imports: {
                use bevy::prelude::*;

                use crate::chunk::{components::ChunkComponent, resources::ChunkManager};
            },
            user_items: {},
            stages: [
                FindAndTransferOwnership: Ecs {
                    core_types: [
                        struct MainAccess<'w, 's> {
                            chunk_query: Query<'w, 's, &'static mut ChunkComponent>,
                            chunk_manager: ResMut<'w, ChunkManager>,
                        }
                        struct Input {
                            chunk_coord: (i32, i32),
                            new_owner: Entity
                        }
                        enum Error {
                            ChunkNotLoaded { chunk_coord: (i32, i32) },
                        }
                    ],
                    core_functions: [
                        fn RunEcs |input, main_access| -> Result<(), Error> {
                            let chunk_coord = input.chunk_coord;
                            let new_owner = input.new_owner;

                            let mut chunk_query = main_access.chunk_query;
                            let mut chunk_manager = main_access.chunk_manager;

                            if let Some(mut chunk) = chunk_query.iter_mut().find(|chunk| chunk.coord == chunk_coord) {
                                if chunk.owner.is_some() {
                                    chunk_manager.owned_chunks.remove(&chunk_coord);
                                }

                                chunk.owner = Some(new_owner);
                                chunk_manager.owned_chunks.insert(chunk_coord, new_owner);

                                Ok(())
                            } else {
                                Err(Error::ChunkNotLoaded { chunk_coord })
                            }
                        }
                    ]
                }
            ]
        }
    ]
}
