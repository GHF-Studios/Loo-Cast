use spacetime_engine_macros::define_workflow_mod;

pub mod chunk {
    pub const NAME: &str = stringify!("Chunk");
    pub mod spawn_chunk {
        pub const NAME: &str = stringify!("SpawnChunk");
        #[derive(std::fmt::Debug,thiserror::Error)]
        pub enum Error {
            ValidateAndSpawnError(self::stages::validate_and_spawn::core_types::Error)
        }
        impl std::fmt::Display for Error {
            fn fmt(&self,f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f,"{:?}",self)
            }
        
            }
        pub struct Type;
        
        impl crate::workflow::traits::WorkflowTypeIE for Type {
            type Input = self::stages::validate_and_spawn::core_types::Input;
            type Error = Error;
            const MODULE_NAME: &'static str = super::NAME;
            const WORKFLOW_NAME: &'static str = self::NAME;
        }
        pub mod workflow_imports {
            pub use super::user_items:: * ;
            pub use crate::workflow::types::{
                Outcome,Outcome::Wait,Outcome::Done
            };
            pub use bevy::prelude::World;
            pub(super)use bevy::prelude::{
                Entity,Handle,Image,Query,ResMut,Transform,SpriteBundle
            };
            pub(super)use bevy::ecs::system::SystemState;
            pub(super)use crate::chunk::{
                components::ChunkComponent,resources::ChunkManager,functions::chunk_pos_to_world
            };
            pub(super)use crate::config::statics::CONFIG;
        }pub mod user_items {
            use super::workflow_imports:: * ;
        }pub mod stages {
            pub mod validate_and_spawn {
                pub const NAME: &str = stringify!("ValidateAndSpawn");
                pub mod core_types {
                    use super::super::super::workflow_imports:: * ;
                    use thiserror::Error;
                    pub struct Input {
                        pub chunk_coord:(i32,i32),pub chunk_owner:Option<Entity> ,pub metric_texture:Handle<Image>
                    }
                    #[derive(std::fmt::Debug,Error)]
                    pub enum Error {
                        ChunkAlreadyLoaded {
                            chunk_coord:(i32,i32)
                        },
                    }
                    impl std::fmt::Display for Error {
                        fn fmt(&self,f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                            write!(f,"{:?}",self)
                        }
                    
                        }
                
                    }pub mod core_functions {
                    use super::super::super::workflow_imports:: * ;
                    use super::core_types:: * ;
                    pub fn run_ecs(input:Input,world: &mut World) -> Result<(),Error>{
                        let chunk_coord = input.chunk_coord;
                        let chunk_owner = input.chunk_owner;
                        let metric_texture = input.metric_texture.clone();
                        let mut system_state = SystemState:: <Query:: < &ChunkComponent> > ::new(world);
                        let chunk_query = system_state.get(world);
                        if chunk_query.iter().any(|chunk|chunk.coord==chunk_coord){
                            return Err(Error::ChunkAlreadyLoaded {
                                chunk_coord
                            });
                        }let default_chunk_z = CONFIG.get:: <f32>("chunk/default_z");
                        let chunk_transform = Transform {
                            translation:chunk_pos_to_world(chunk_coord).extend(default_chunk_z), ..Default::default()
                        };
                        world.spawn((SpriteBundle {
                            texture:metric_texture,transform:chunk_transform, ..Default::default()
                        },ChunkComponent {
                            coord:chunk_coord,owner:chunk_owner,
                        },));
                        let mut chunk_manager = SystemState:: <ResMut:: <ChunkManager> > ::new(world).get_mut(world);
                        chunk_manager.loaded_chunks.insert(chunk_coord);
                        if let Some(owner) = chunk_owner {
                            chunk_manager.owned_chunks.insert(chunk_coord,owner);
                        }Ok(())
                    }
                
                    }
            }
        }
    }pub mod despawn_chunk {
        pub const NAME: &str = stringify!("DespawnChunk");
        #[derive(std::fmt::Debug,thiserror::Error)]
        pub enum Error {
            FindAndDespawnError(self::stages::find_and_despawn::core_types::Error)
        }
        impl std::fmt::Display for Error {
            fn fmt(&self,f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f,"{:?}",self)
            }
        
            }
        pub struct Type;
        
        impl crate::workflow::traits::WorkflowTypeIE for Type {
            type Input = self::stages::find_and_despawn::core_types::Input;
            type Error = Error;
            const MODULE_NAME: &'static str = super::NAME;
            const WORKFLOW_NAME: &'static str = self::NAME;
        }
        pub mod workflow_imports {
            pub use super::user_items:: * ;
            pub use crate::workflow::types::{
                Outcome,Outcome::Wait,Outcome::Done
            };
            pub use bevy::prelude::World;
            pub(super)use bevy::prelude::{
                Entity,Query,ResMut,DespawnRecursiveExt
            };
            pub(super)use bevy::ecs::system::SystemState;
            pub(super)use crate::chunk::{
                components::ChunkComponent,resources::ChunkManager
            };
        }pub mod user_items {
            use super::workflow_imports:: * ;
        }pub mod stages {
            pub mod find_and_despawn {
                pub const NAME: &str = stringify!("FindAndDespawn");
                pub mod core_types {
                    use super::super::super::workflow_imports:: * ;
                    use thiserror::Error;
                    pub struct Input {
                        pub chunk_coord:(i32,i32)
                    }
                    #[derive(std::fmt::Debug,Error)]
                    pub enum Error {
                        ChunkNotLoaded {
                            chunk_coord:(i32,i32)
                        },
                    }
                    impl std::fmt::Display for Error {
                        fn fmt(&self,f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                            write!(f,"{:?}",self)
                        }
                    
                        }
                
                    }pub mod core_functions {
                    use super::super::super::workflow_imports:: * ;
                    use super::core_types:: * ;
                    pub fn run_ecs(input:Input,world: &mut World) -> Result<(),Error>{
                        let chunk_coord = input.chunk_coord;
                        let mut system_state:SystemState<(Query<(Entity, &ChunkComponent)> ,ResMut<ChunkManager> ,)>  = SystemState::new(world);
                        let(chunk_query,mut chunk_manager) = system_state.get_mut(world);
                        if let Some((entity,_)) = chunk_query.iter().find(|(_,chunk)|chunk.coord==chunk_coord){
                            chunk_manager.loaded_chunks.remove(&chunk_coord);
                            chunk_manager.owned_chunks.remove(&chunk_coord);
                            world.entity_mut(entity).despawn_recursive();
                            Ok(())
                        }else {
                            Err(Error::ChunkNotLoaded {
                                chunk_coord
                            })
                        }
                    }
                
                    }
            }
        }
    }pub mod transfer_chunk_ownership {
        pub const NAME: &str = stringify!("TransferChunkOwnership");
        #[derive(std::fmt::Debug,thiserror::Error)]
        pub enum Error {
            FindAndTransferOwnershipError(self::stages::find_and_transfer_ownership::core_types::Error)
        }
        impl std::fmt::Display for Error {
            fn fmt(&self,f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f,"{:?}",self)
            }
        
            }
        pub struct Type;
        
        impl crate::workflow::traits::WorkflowTypeIE for Type {
            type Input = self::stages::find_and_transfer_ownership::core_types::Input;
            type Error = Error;
            const MODULE_NAME: &'static str = super::NAME;
            const WORKFLOW_NAME: &'static str = self::NAME;
        }
        pub mod workflow_imports {
            pub use super::user_items:: * ;
            pub use crate::workflow::types::{
                Outcome,Outcome::Wait,Outcome::Done
            };
            pub use bevy::prelude::World;
            pub(super)use bevy::prelude::{
                Entity,Query,ResMut
            };
            pub(super)use bevy::ecs::system::SystemState;
            pub(super)use crate::chunk::{
                components::ChunkComponent,resources::ChunkManager
            };
        }pub mod user_items {
            use super::workflow_imports:: * ;
        }pub mod stages {
            pub mod find_and_transfer_ownership {
                pub const NAME: &str = stringify!("FindAndTransferOwnership");
                pub mod core_types {
                    use super::super::super::workflow_imports:: * ;
                    use thiserror::Error;
                    pub struct Input {
                        pub chunk_coord:(i32,i32),pub new_owner:Entity
                    }
                    #[derive(std::fmt::Debug,Error)]
                    pub enum Error {
                        ChunkNotLoaded {
                            chunk_coord:(i32,i32)
                        },
                    }
                    impl std::fmt::Display for Error {
                        fn fmt(&self,f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                            write!(f,"{:?}",self)
                        }
                    
                        }
                
                    }pub mod core_functions {
                    use super::super::super::workflow_imports:: * ;
                    use super::core_types:: * ;
                    pub fn run_ecs(input:Input,world: &mut World) -> Result<(),Error>{
                        let chunk_coord = input.chunk_coord;
                        let new_owner = input.new_owner;
                        let mut system_state:SystemState<(Query< &mut ChunkComponent> ,ResMut<ChunkManager> ,)>  = SystemState::new(world);
                        let(mut chunk_query,mut chunk_manager) = system_state.get_mut(world);
                        if let Some(mut chunk) = chunk_query.iter_mut().find(|chunk|chunk.coord==chunk_coord){
                            if chunk.owner.is_some(){
                                chunk_manager.owned_chunks.remove(&chunk_coord);
                            }chunk.owner = Some(new_owner);
                            chunk_manager.owned_chunks.insert(chunk_coord,new_owner);
                            Ok(())
                        }else {
                            Err(Error::ChunkNotLoaded {
                                chunk_coord
                            })
                        }
                    }
                
                    }
            }
        }
    }
}