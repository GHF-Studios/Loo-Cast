use spacetime_engine_macros::define_workflow_mod_OLD;

pub mod chunk {
    use bevy::prelude:: * ;
    pub const NAME: &str = stringify!("Chunk");
    pub struct ChunkWorkflowPlugin;
    
    impl Plugin for ChunkWorkflowPlugin {
        fn build(&self,app: &mut App){
            app.add_systems(PreStartup,register_workflow_type_module);
        }
    
        }
    fn register_workflow_type_module(mut workflow_type_module_registry:ResMut<crate::workflow::resources::WorkflowTypeModuleRegistry>){
        workflow_type_module_registry.register(crate::workflow::types::WorkflowTypeModule {
            name:stringify!("Chunk"),workflow_types:vec![spawn_chunk::TypeIE::create_workflow(),despawn_chunk::TypeIE::create_workflow(),transfer_chunk_ownership::TypeIE::create_workflow()],
        });
    }
    pub mod spawn_chunk {
        pub const NAME: &str = stringify!("SpawnChunk");
        pub async fn run(input: <TypeIE as crate::workflow::traits::WorkflowTypeIE> ::Input) -> Result<(), <TypeIE as crate::workflow::traits::WorkflowTypeIE> ::Error>{
            crate::workflow::functions::run_workflow_ie:: <TypeIE>(input).await
        }
        #[derive(std::fmt::Debug,thiserror::Error)]
        pub enum Error {
            ValidateAndSpawnError(self::stages::validate_and_spawn::core_types::Error)
        }
        impl std::fmt::Display for Error {
            fn fmt(&self,f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f,"{:?}",self)
            }
        
            }
        pub struct TypeIE;
        
        impl crate::workflow::traits::WorkflowTypeIE for TypeIE {
            type Input = self::stages::validate_and_spawn::core_types::Input;
            type Error = Error;
            const MODULE_NAME: &'static str = super::NAME;
            const WORKFLOW_NAME: &'static str = self::NAME;
        }
        impl TypeIE {
            pub fn create_workflow() -> crate::workflow::types::WorkflowType {
                crate::workflow::types::WorkflowType {
                    name:self::NAME,stages:vec![crate::workflow::stage::Stage::Ecs(crate::workflow::stage::StageEcs {
                        index:0,name:stringify!("ValidateAndSpawn"),signature:crate::workflow::stage::StageSignature::IE,run_ecs:Box::new(self::stages::validate_and_spawn::core_functions::run_ecs)as Box<dyn FnMut(Option<Box<dyn std::any::Any+Send+Sync>> ,Box<dyn std::any::Any+Send+Sync>)->Option<Box<dyn std::any::Any+Send+Sync>> +Send+Sync> ,handle_ecs_run_response:Box::new(|module_name: &'static str,workflow_name: &'static str,response:Option<Box<dyn std::any::Any+Send+Sync>> ,completion_sender:crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent> ,failure_sender:Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>> |{
                            Box::new(move|stage:crate::workflow::stage::StageEcs|{
                                let response = response.expect("Ecs stages with error must have a response");
                                let result:Result<(),crate::chunk::workflows::chunk::spawn_chunk::stages::validate_and_spawn::core_types::Error>  =  *response.downcast().expect("Failed to downcast response result data");
                                match result {
                                    Ok(_) => {
                                        if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                                            ty:crate::workflow::stage::StageType::Ecs,module_name,workflow_name,current_stage:0,stage_return:crate::workflow::stage::Stage::Ecs(stage),stage_output:None,
                                        }){
                                            unreachable!("Ecs response handler error: Completion event send error: {}",send_err);
                                        }
                                    }Err(error) => {
                                        let error = crate::workflow::response::TypedWorkflowResponseE(Err(Box::new(crate::chunk::workflows::chunk::spawn_chunk::Error::ValidateAndSpawnError(error))));
                                        let error = Some(Box::new(error)as Box<dyn std::any::Any+Send+Sync>);
                                        let failure_sender = match failure_sender {
                                            Some(failure_sender) => failure_sender,None => {
                                                unreachable!("Ecs response handler error: Failure event send error: No failure sender provided");
                                            }
                                        };
                                        if let Err(send_err) = failure_sender.send(crate::workflow::events::StageFailureEvent {
                                            ty:crate::workflow::stage::StageType::Ecs,module_name,workflow_name,current_stage:0,stage_return:crate::workflow::stage::Stage::Ecs(stage),stage_error:error,
                                        }){
                                            unreachable!("Ecs response handler error: Failure event send error: {}",send_err);
                                        }
                                    }
                                }
                            })
                        }),completion_sender:crate::workflow::channels::get_stage_completion_sender().clone(),failure_sender:Some(crate::workflow::channels::get_stage_failure_sender().clone()),
                    })],
                }
            }
        
            }
        pub mod workflow_imports {
            pub use super::user_items:: * ;
            pub use crate::workflow::types::{
                Outcome,Outcome::Wait,Outcome::Done
            };
            pub use bevy::prelude::World;
            pub(super)use bevy::prelude:: * ;
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
                    use bevy::prelude:: * ;
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
                    #[derive(bevy::ecs::system::SystemParam)]
                    pub struct MainAccess<'w,'s>{
                        pub commands:Commands<'w,'s> ,pub chunk_query:Query<'w,'s, &'static ChunkComponent> ,pub chunk_manager:ResMut<'w,ChunkManager> ,
                    }
                
                    }pub mod core_functions {
                    use super::super::super::workflow_imports:: * ;
                    use super::core_types:: * ;
                    pub fn run_ecs(input:Option<Box<dyn std::any::Any+Send+Sync>> ,main_access:Box<dyn std::any::Any+Send+Sync>) -> Option<Box<dyn std::any::Any+Send+Sync>>{
                        let input = input.unwrap().downcast:: <Input>().unwrap();
                        let main_access = main_access.downcast:: <MainAccess>().unwrap();
                        let result = run_ecs_inner(*input, *main_access);
                        Some(Box::new(result))
                    }
                    fn run_ecs_inner(input:Input,main_access:MainAccess) -> Result<(),Error>{
                        let chunk_coord = input.chunk_coord;
                        let chunk_owner = input.chunk_owner;
                        let metric_texture = input.metric_texture.clone();
                        let mut commands = main_access.commands;
                        let chunk_query = main_access.chunk_query;
                        let mut chunk_manager = main_access.chunk_manager;
                        if chunk_query.iter().any(|chunk|chunk.coord==chunk_coord){
                            return Err(Error::ChunkAlreadyLoaded {
                                chunk_coord
                            });
                        }let default_chunk_z = CONFIG.get:: <f32>("chunk/default_z");
                        let chunk_transform = Transform {
                            translation:chunk_pos_to_world(chunk_coord).extend(default_chunk_z), ..Default::default()
                        };
                        commands.spawn((SpriteBundle {
                            texture:metric_texture,transform:chunk_transform, ..Default::default()
                        },ChunkComponent {
                            coord:chunk_coord,owner:chunk_owner,
                        },));
                        chunk_manager.loaded_chunks.insert(chunk_coord);
                        if let Some(owner) = chunk_owner {
                            chunk_manager.owned_chunks.insert(chunk_coord,owner);
                        }Ok(())
                    }
                
                    }
            }
        }
    }
    pub mod despawn_chunk {
        pub const NAME: &str = stringify!("DespawnChunk");
        pub async fn run(input: <TypeIE as crate::workflow::traits::WorkflowTypeIE> ::Input) -> Result<(), <TypeIE as crate::workflow::traits::WorkflowTypeIE> ::Error>{
            crate::workflow::functions::run_workflow_ie:: <TypeIE>(input).await
        }
        #[derive(std::fmt::Debug,thiserror::Error)]
        pub enum Error {
            FindAndDespawnError(self::stages::find_and_despawn::core_types::Error)
        }
        impl std::fmt::Display for Error {
            fn fmt(&self,f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f,"{:?}",self)
            }
        
            }
        pub struct TypeIE;
        
        impl crate::workflow::traits::WorkflowTypeIE for TypeIE {
            type Input = self::stages::find_and_despawn::core_types::Input;
            type Error = Error;
            const MODULE_NAME: &'static str = super::NAME;
            const WORKFLOW_NAME: &'static str = self::NAME;
        }
        impl TypeIE {
            pub fn create_workflow() -> crate::workflow::types::WorkflowType {
                crate::workflow::types::WorkflowType {
                    name:self::NAME,stages:vec![crate::workflow::stage::Stage::Ecs(crate::workflow::stage::StageEcs {
                        index:0,name:stringify!("FindAndDespawn"),signature:crate::workflow::stage::StageSignature::IE,run_ecs:Box::new(self::stages::find_and_despawn::core_functions::run_ecs)as Box<dyn FnMut(Option<Box<dyn std::any::Any+Send+Sync>> ,Box<dyn std::any::Any+Send+Sync>)->Option<Box<dyn std::any::Any+Send+Sync>> +Send+Sync> ,handle_ecs_run_response:Box::new(|module_name: &'static str,workflow_name: &'static str,response:Option<Box<dyn std::any::Any+Send+Sync>> ,completion_sender:crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent> ,failure_sender:Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>> |{
                            Box::new(move|stage:crate::workflow::stage::StageEcs|{
                                let response = response.expect("Ecs stages with error must have a response");
                                let result:Result<(),crate::chunk::workflows::chunk::despawn_chunk::stages::find_and_despawn::core_types::Error>  =  *response.downcast().expect("Failed to downcast response result data");
                                match result {
                                    Ok(_) => {
                                        if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                                            ty:crate::workflow::stage::StageType::Ecs,module_name,workflow_name,current_stage:0,stage_return:crate::workflow::stage::Stage::Ecs(stage),stage_output:None,
                                        }){
                                            unreachable!("Ecs response handler error: Completion event send error: {}",send_err);
                                        }
                                    }Err(error) => {
                                        let error = crate::workflow::response::TypedWorkflowResponseE(Err(Box::new(crate::chunk::workflows::chunk::despawn_chunk::Error::FindAndDespawnError(error))));
                                        let error = Some(Box::new(error)as Box<dyn std::any::Any+Send+Sync>);
                                        let failure_sender = match failure_sender {
                                            Some(failure_sender) => failure_sender,None => {
                                                unreachable!("Ecs response handler error: Failure event send error: No failure sender provided");
                                            }
                                        };
                                        if let Err(send_err) = failure_sender.send(crate::workflow::events::StageFailureEvent {
                                            ty:crate::workflow::stage::StageType::Ecs,module_name,workflow_name,current_stage:0,stage_return:crate::workflow::stage::Stage::Ecs(stage),stage_error:error,
                                        }){
                                            unreachable!("Ecs response handler error: Failure event send error: {}",send_err);
                                        }
                                    }
                                }
                            })
                        }),completion_sender:crate::workflow::channels::get_stage_completion_sender().clone(),failure_sender:Some(crate::workflow::channels::get_stage_failure_sender().clone()),
                    })],
                }
            }
        
            }
        pub mod workflow_imports {
            pub use super::user_items:: * ;
            pub use crate::workflow::types::{
                Outcome,Outcome::Wait,Outcome::Done
            };
            pub use bevy::prelude::World;
            pub(super)use bevy::prelude:: * ;
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
                    use bevy::prelude:: * ;
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
                    #[derive(bevy::ecs::system::SystemParam)]
                    pub struct MainAccess<'w,'s>{
                        pub commands:Commands<'w,'s> ,pub chunk_query:Query<'w,'s,(Entity, &'static ChunkComponent)> ,pub chunk_manager:ResMut<'w,ChunkManager> ,
                    }
                
                    }pub mod core_functions {
                    use super::super::super::workflow_imports:: * ;
                    use super::core_types:: * ;
                    pub fn run_ecs(input:Option<Box<dyn std::any::Any+Send+Sync>> ,main_access:Box<dyn std::any::Any+Send+Sync>) -> Option<Box<dyn std::any::Any+Send+Sync>>{
                        let input = input.unwrap().downcast:: <Input>().unwrap();
                        let main_access = main_access.downcast:: <MainAccess>().unwrap();
                        let result = run_ecs_inner(*input, *main_access);
                        Some(Box::new(result))
                    }
                    fn run_ecs_inner(input:Input,main_access:MainAccess) -> Result<(),Error>{
                        let chunk_coord = input.chunk_coord;
                        let mut commands = main_access.commands;
                        let chunk_query = main_access.chunk_query;
                        let mut chunk_manager = main_access.chunk_manager;
                        if let Some((entity,_)) = chunk_query.iter().find(|(_,chunk)|chunk.coord==chunk_coord){
                            chunk_manager.loaded_chunks.remove(&chunk_coord);
                            chunk_manager.owned_chunks.remove(&chunk_coord);
                            commands.entity(entity).despawn_recursive();
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
    pub mod transfer_chunk_ownership {
        pub const NAME: &str = stringify!("TransferChunkOwnership");
        pub async fn run(input: <TypeIE as crate::workflow::traits::WorkflowTypeIE> ::Input) -> Result<(), <TypeIE as crate::workflow::traits::WorkflowTypeIE> ::Error>{
            crate::workflow::functions::run_workflow_ie:: <TypeIE>(input).await
        }
        #[derive(std::fmt::Debug,thiserror::Error)]
        pub enum Error {
            FindAndTransferOwnershipError(self::stages::find_and_transfer_ownership::core_types::Error)
        }
        impl std::fmt::Display for Error {
            fn fmt(&self,f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f,"{:?}",self)
            }
        
            }
        pub struct TypeIE;
        
        impl crate::workflow::traits::WorkflowTypeIE for TypeIE {
            type Input = self::stages::find_and_transfer_ownership::core_types::Input;
            type Error = Error;
            const MODULE_NAME: &'static str = super::NAME;
            const WORKFLOW_NAME: &'static str = self::NAME;
        }
        impl TypeIE {
            pub fn create_workflow() -> crate::workflow::types::WorkflowType {
                crate::workflow::types::WorkflowType {
                    name:self::NAME,stages:vec![crate::workflow::stage::Stage::Ecs(crate::workflow::stage::StageEcs {
                        index:0,name:stringify!("FindAndTransferOwnership"),signature:crate::workflow::stage::StageSignature::IE,run_ecs:Box::new(self::stages::find_and_transfer_ownership::core_functions::run_ecs)as Box<dyn FnMut(Option<Box<dyn std::any::Any+Send+Sync>> ,Box<dyn std::any::Any+Send+Sync>)->Option<Box<dyn std::any::Any+Send+Sync>> +Send+Sync> ,handle_ecs_run_response:Box::new(|module_name: &'static str,workflow_name: &'static str,response:Option<Box<dyn std::any::Any+Send+Sync>> ,completion_sender:crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent> ,failure_sender:Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>> |{
                            Box::new(move|stage:crate::workflow::stage::StageEcs|{
                                let response = response.expect("Ecs stages with error must have a response");
                                let result:Result<(),crate::chunk::workflows::chunk::transfer_chunk_ownership::stages::find_and_transfer_ownership::core_types::Error>  =  *response.downcast().expect("Failed to downcast response result data");
                                match result {
                                    Ok(_) => {
                                        if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                                            ty:crate::workflow::stage::StageType::Ecs,module_name,workflow_name,current_stage:0,stage_return:crate::workflow::stage::Stage::Ecs(stage),stage_output:None,
                                        }){
                                            unreachable!("Ecs response handler error: Completion event send error: {}",send_err);
                                        }
                                    }Err(error) => {
                                        let error = crate::workflow::response::TypedWorkflowResponseE(Err(Box::new(crate::chunk::workflows::chunk::transfer_chunk_ownership::Error::FindAndTransferOwnershipError(error))));
                                        let error = Some(Box::new(error)as Box<dyn std::any::Any+Send+Sync>);
                                        let failure_sender = match failure_sender {
                                            Some(failure_sender) => failure_sender,None => {
                                                unreachable!("Ecs response handler error: Failure event send error: No failure sender provided");
                                            }
                                        };
                                        if let Err(send_err) = failure_sender.send(crate::workflow::events::StageFailureEvent {
                                            ty:crate::workflow::stage::StageType::Ecs,module_name,workflow_name,current_stage:0,stage_return:crate::workflow::stage::Stage::Ecs(stage),stage_error:error,
                                        }){
                                            unreachable!("Ecs response handler error: Failure event send error: {}",send_err);
                                        }
                                    }
                                }
                            })
                        }),completion_sender:crate::workflow::channels::get_stage_completion_sender().clone(),failure_sender:Some(crate::workflow::channels::get_stage_failure_sender().clone()),
                    })],
                }
            }
        
            }
        pub mod workflow_imports {
            pub use super::user_items:: * ;
            pub use crate::workflow::types::{
                Outcome,Outcome::Wait,Outcome::Done
            };
            pub use bevy::prelude::World;
            pub(super)use bevy::prelude:: * ;
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
                    use bevy::prelude:: * ;
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
                    #[derive(bevy::ecs::system::SystemParam)]
                    pub struct MainAccess<'w,'s>{
                        pub chunk_query:Query<'w,'s, &'static mut ChunkComponent> ,pub chunk_manager:ResMut<'w,ChunkManager> ,
                    }
                
                    }pub mod core_functions {
                    use super::super::super::workflow_imports:: * ;
                    use super::core_types:: * ;
                    pub fn run_ecs(input:Option<Box<dyn std::any::Any+Send+Sync>> ,main_access:Box<dyn std::any::Any+Send+Sync>) -> Option<Box<dyn std::any::Any+Send+Sync>>{
                        let input = input.unwrap().downcast:: <Input>().unwrap();
                        let main_access = main_access.downcast:: <MainAccess>().unwrap();
                        let result = run_ecs_inner(*input, *main_access);
                        Some(Box::new(result))
                    }
                    fn run_ecs_inner(input:Input,main_access:MainAccess) -> Result<(),Error>{
                        let chunk_coord = input.chunk_coord;
                        let new_owner = input.new_owner;
                        let mut chunk_query = main_access.chunk_query;
                        let mut chunk_manager = main_access.chunk_manager;
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
