mod chunk_workflows {
    pub const NAME: &str = "Chunk";
    pub mod spawn_chunk {
        pub const NAME: &str = "SpawnChunk";
        pub mod vorkflow_imports {
            // Automatic imports
            pub use super::user_types::*;
            pub use super::user_functions::*;
            
            pub use crate::vorkflow::types::{Outcome, Outcome::Wait, Outcome::Done};

            // User imports
            pub use bevy::prelude::{Entity, Handle, Image, World, Query, ResMut, Transform, SpriteBundle};
            pub use bevy::ecs::system::SystemState;

            pub(crate) use crate::chunk::{components::ChunkComponent, resources::ChunkManager, functions::chunk_pos_to_world};
            pub use crate::config::statics::CONFIG;
        }
        pub mod user_types {}
        pub mod user_functions {}
        pub mod stages {
            pub mod validate_and_spawn {
                pub mod core_types {
                    pub use super::super::super::vorkflow_imports::*;

                    pub struct Input { 
                        pub chunk_coord: (i32, i32), 
                        pub chunk_owner: Option<Entity>, 
                        pub metric_texture: Handle<Image> 
                    }
                    pub enum Error {
                        ChunkAlreadyLoaded { chunk_coord: (i32, i32) },
                    }
                }
                pub mod core_functions {
                    use super::core_types::*;

                    pub fn run_ecs(input: Input, world: &mut World) -> Result<(), Error> {
                        let chunk_coord = input.chunk_coord;
                        let chunk_owner = input.chunk_owner;
                        let metric_texture = input.metric_texture;

                        let mut system_state = SystemState::<Query::<&ChunkComponent>>::new(world);
                        let chunk_query = system_state.get(world);

                        if chunk_query.iter().any(|chunk| chunk.coord == chunk_coord) {
                            return Err(Error::ChunkAlreadyLoaded { chunk_coord });
                        }

                        let default_chunk_z = CONFIG.get::<f32>("chunk/default_z");
                        let chunk_transform = Transform {
                            translation: chunk_pos_to_world(chunk_coord).extend(default_chunk_z),
                            ..Default::default()
                        };

                        world.spawn((
                            SpriteBundle {
                                texture: metric_texture.clone(),
                                transform: chunk_transform,
                                ..Default::default()
                            },
                            ChunkComponent {
                                coord: chunk_coord,
                                owner: chunk_owner,
                            },
                        ));

                        let mut chunk_manager = SystemState::<ResMut::<ChunkManager>>::new(world).get_mut(world);
                        chunk_manager.loaded_chunks.insert(chunk_coord);
                        if let Some(owner) = chunk_owner {
                            chunk_manager.owned_chunks.insert(chunk_coord, owner);
                        }

                        Ok(())
                    }
                }
            }
        }
    }
    pub mod despawn_chunk {
        pub const NAME: &str = "DespawnChunk";
        pub mod vorkflow_imports {
            // Automatic imports
            pub use super::user_types::*;
            pub use super::user_functions::*;
            
            pub use crate::vorkflow::types::{Outcome, Outcome::Wait, Outcome::Done};

            // User imports
            pub use bevy::prelude::{Entity, World, Query, ResMut, DespawnRecursiveExt};
            pub use bevy::ecs::system::SystemState;

            pub(crate) use crate::chunk::{components::ChunkComponent, resources::ChunkManager};
        }
        pub mod user_types {}
        pub mod user_functions {}
        pub mod stages {
            pub mod find_and_despawn {
                pub mod core_types {
                    pub use super::super::super::vorkflow_imports::*;
                    
                    pub struct Input {
                        pub chunk_coord: (i32, i32) 
                    }
                    pub enum Error {
                        ChunkNotLoaded { chunk_coord: (i32, i32) },
                    }
                }
                pub mod core_functions {
                    use super::core_types::*;

                    pub fn run_ecs(input: Input, world: &mut World) -> Result<(), Error> {
                        let chunk_coord = input.chunk_coord;

                        let mut system_state: SystemState<(
                            Query<(Entity, &ChunkComponent)>,
                            ResMut<ChunkManager>,
                        )> = SystemState::new(world);
                        let (mut chunk_query, mut chunk_manager) = system_state.get_mut(world);

                        if let Some((entity, _)) = chunk_query.iter().find(|(_, chunk)| chunk.coord == chunk_coord) {
                            chunk_manager.loaded_chunks.remove(&chunk_coord);
                            chunk_manager.owned_chunks.remove(&chunk_coord);

                            world.entity_mut(entity).despawn_recursive();

                            Ok(())
                        } else {
                            Err(Error::ChunkNotLoaded { chunk_coord })
                        }
                    }
                }
            }
        }
    }
    pub mod transfer_chunk_ownership {
        pub const NAME: &str = "TransferChunkOwnership";
        pub mod vorkflow_imports {
            // Automatic imports
            pub use super::user_types::*;
            pub use super::user_functions::*;
            
            pub use crate::vorkflow::types::{Outcome, Outcome::Wait, Outcome::Done};

            // User imports
            pub use bevy::prelude::{Entity, World, Query, ResMut};
            pub use bevy::ecs::system::SystemState;

            pub(crate) use crate::chunk::{components::ChunkComponent, resources::ChunkManager};
        }
        pub mod user_types {}
        pub mod user_functions {}
        pub mod stages {
            pub mod find_and_transfer_ownership {
                pub mod core_types {
                    pub use super::super::super::vorkflow_imports::*;
                    
                    pub struct Input {
                        pub chunk_coord: (i32, i32), 
                        pub new_owner: Entity 
                    }
                    pub enum Error {
                        ChunkNotLoaded { chunk_coord: (i32, i32) },
                    }
                }
                pub mod core_functions {
                    use super::core_types::*;

                    pub fn run_ecs(input: Input, world: &mut World) -> Result<(), Error> {
                        let chunk_coord = input.chunk_coord;
                        let new_owner = input.new_owner;

                        let mut system_state: SystemState<(
                            Query<&mut ChunkComponent>,
                            ResMut<ChunkManager>,
                        )> = SystemState::new(world);
                        let (mut chunk_query, mut chunk_manager) = system_state.get_mut(world);

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
                }
            }
        }
    }
}