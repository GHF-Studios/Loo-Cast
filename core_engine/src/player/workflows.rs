use core_engine_macros::define_workflow_mod_OLD;

pub mod player {
    use bevy::prelude:: * ;
    pub const NAME: &str = "Player";
    pub struct PlayerWorkflowsPlugin;
    
    impl Plugin for PlayerWorkflowsPlugin {
        fn build(&self,app: &mut App){
            app.add_systems(PreStartup,register_workflow_type_module).add_plugins(crate::player::workflows::player::spawn_player::SpawnPlayerWorkflowPlugin).add_plugins(crate::player::workflows::player::despawn_player::DespawnPlayerWorkflowPlugin);
        }
    
        }
    fn register_workflow_type_module(mut workflow_type_module_registry:ResMut<crate::workflow::resources::WorkflowTypeModuleRegistry>){
        workflow_type_module_registry.register(crate::workflow::types::WorkflowTypeModule {
            name:"Player",workflow_types:vec![spawn_player::TypeOE::create_workflow(),despawn_player::TypeE::create_workflow()],
        });
    }
    pub mod spawn_player {
        pub const NAME: &str = "SpawnPlayer";
        pub async fn run() -> Result<<TypeOE as crate::workflow::traits::WorkflowTypeOE> ::Output, <TypeOE as crate::workflow::traits::WorkflowTypeOE> ::Error>{
            crate::workflow::functions::run_workflow_oe:: <TypeOE>().await
        }
        pub(crate)struct SpawnPlayerWorkflowPlugin;
        
        impl bevy::prelude::Plugin for SpawnPlayerWorkflowPlugin {
            fn build(&self,app: &mut bevy::prelude::App){
                use bevy::prelude::IntoSystemConfigs;
                app.insert_resource(stages::validate_and_spawn_and_wait::core_types::StageBuffer::default()).insert_resource(stages::validate_and_spawn_and_wait::core_types::take_fill_workflow_stage_buffer_receiver()).add_systems(bevy::prelude::Update,stages::validate_and_spawn_and_wait::core_types::receive_ecs_while_stages_to_ecs_while_buffers_system.before(stages::validate_and_spawn_and_wait::core_functions::poll_ecs_while_stage_buffer_system)).add_systems(bevy::prelude::Update,stages::validate_and_spawn_and_wait::core_functions::poll_ecs_while_stage_buffer_system);
            }
        
            }
        #[derive(std::fmt::Debug,thiserror::Error)]
        pub enum Error {
            ValidateAndSpawnAndWaitError(self::stages::validate_and_spawn_and_wait::core_types::Error)
        }
        impl std::fmt::Display for Error {
            fn fmt(&self,f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f,"{:?}",self)
            }
        
            }
        pub struct TypeOE;
        
        impl crate::workflow::traits::WorkflowTypeOE for TypeOE {
            type Output = self::stages::validate_and_spawn_and_wait::core_types::Output;
            type Error = Error;
            const MODULE_NAME: &'static str = super::NAME;
            const WORKFLOW_NAME: &'static str = self::NAME;
        }
        impl TypeOE {
            pub fn create_workflow() -> crate::workflow::types::WorkflowType {
                crate::workflow::types::WorkflowType {
                    name:self::NAME,stages:vec![crate::workflow::stage::Stage::EcsWhile(crate::workflow::stage::StageEcsWhile {
                        index:0,name:"ValidateAndSpawnAndWait",signature:crate::workflow::stage::StageSignature::OE,handle_ecs_while_setup_response:Box::new(|module_name: &'static str,workflow_name: &'static str,response:Option<crate::debug::types::AnySendSyncPremiumBox> ,setup_sender:crossbeam_channel::Sender<crate::workflow::events::StageSetupEvent> ,failure_sender:Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>> , |{
                            Box::new(move|stage:crate::workflow::stage::StageEcsWhile|{
                                let response = response.expect("EcsWhile stages with state and error must have a response");
                                let result:Result<crate::player::workflows::player::spawn_player::stages::validate_and_spawn_and_wait::core_types::State,crate::player::workflows::player::spawn_player::stages::validate_and_spawn_and_wait::core_types::Error>  = response.into_inner();
                                match result {
                                    Ok(state) => {
                                        let state = Some(crate::debug::types::AnySendSyncPremiumBox::new(state,"crate::player::workflows::player::spawn_player::stages::validate_and_spawn_and_wait::core_types::State".to_string()));
                                        if let Err(send_err) = setup_sender.send(crate::workflow::events::StageSetupEvent {
                                            ty:crate::workflow::stage::StageType::EcsWhile,module_name,workflow_name,current_stage:0,stage_return:crate::workflow::stage::Stage::EcsWhile(stage),stage_state:state,
                                        }){
                                            unreachable!("EcsWhile response handler error: Setup event send error: {}",send_err);
                                        }
                                    }Err(error) => {
                                        let error = crate::workflow::response::TypedWorkflowResponseE {
                                            module_name,workflow_name,result:Err(crate::debug::types::AnySendSyncPremiumBox::new(crate::player::workflows::player::spawn_player::Error::ValidateAndSpawnAndWaitError(error),"crate::player::workflows::player::spawn_player::stages::validate_and_spawn_and_wait::core_types::Error".to_string()))
                                        };
                                        let error = Some(crate::debug::types::AnySendSyncPremiumBox::new(error,"crate::player::workflows::player::spawn_player::stages::validate_and_spawn_and_wait::core_types::Error".to_string()));
                                        let failure_sender = match failure_sender {
                                            Some(failure_sender) => failure_sender,None => {
                                                unreachable!("EcsWhile response handler error: Failure event send error: No failure sender provided");
                                            }
                                        };
                                        if let Err(send_err) = failure_sender.send(crate::workflow::events::StageFailureEvent {
                                            ty:crate::workflow::stage::StageType::EcsWhile,module_name,workflow_name,current_stage:0,stage_return:crate::workflow::stage::Stage::EcsWhile(stage),stage_error:error,
                                        }){
                                            unreachable!("EcsWhile response handler error: Failure event send error: {}",send_err);
                                        }
                                    }
                                }
                            })
                        }),handle_ecs_while_run_response:Box::new(|module_name: &'static str,workflow_name: &'static str,response:Option<crate::debug::types::AnySendSyncPremiumBox> ,wait_sender:crossbeam_channel::Sender<crate::workflow::events::StageWaitEvent> ,completion_sender:crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent> ,failure_sender:Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>> |{
                            Box::new(move|stage:crate::workflow::stage::StageEcsWhile, |{
                                let response = response.expect("EcsWhile stages with output and error (last stage) must have a response");
                                let outcome_result:Result<crate::workflow::types::Outcome<crate::player::workflows::player::spawn_player::stages::validate_and_spawn_and_wait::core_types::State,crate::player::workflows::player::spawn_player::stages::validate_and_spawn_and_wait::core_types::Output> ,crate::player::workflows::player::spawn_player::stages::validate_and_spawn_and_wait::core_types::Error>  = response.into_inner();
                                match outcome_result {
                                    Ok(outcome) => {
                                        match outcome {
                                            crate::workflow::types::Outcome::Wait(state) => {
                                                let state = Some(crate::debug::types::AnySendSyncPremiumBox::new(state,"crate::player::workflows::player::spawn_player::stages::validate_and_spawn_and_wait::core_types::State".to_string()));
                                                if let Err(send_err) = wait_sender.send(crate::workflow::events::StageWaitEvent {
                                                    ty:crate::workflow::stage::StageType::EcsWhile,module_name,workflow_name,current_stage:0,stage_return:crate::workflow::stage::Stage::EcsWhile(stage),stage_state:state,
                                                }){
                                                    unreachable!("EcsWhile response handler error: Wait event send error: {}",send_err);
                                                }
                                            },crate::workflow::types::Outcome::Done(output) => {
                                                let output = Some(crate::debug::types::AnySendSyncPremiumBox::new(output,"crate::player::workflows::player::spawn_player::stages::validate_and_spawn_and_wait::core_types::Output".to_string()));
                                                if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                                                    ty:crate::workflow::stage::StageType::EcsWhile,module_name,workflow_name,current_stage:0,stage_return:crate::workflow::stage::Stage::EcsWhile(stage),stage_output:output,
                                                }){
                                                    unreachable!("EcsWhile response handler error: Completion event send error: {}",send_err);
                                                }
                                            }
                                        }
                                    }Err(error) => {
                                        let error = crate::workflow::response::TypedWorkflowResponseOE {
                                            module_name,workflow_name,result:Err(crate::debug::types::AnySendSyncPremiumBox::new(crate::player::workflows::player::spawn_player::Error::ValidateAndSpawnAndWaitError(error),"crate::player::workflows::player::spawn_player::stages::validate_and_spawn_and_wait::core_types::Error".to_string()))
                                        };
                                        let error = Some(crate::debug::types::AnySendSyncPremiumBox::new(error,"crate::player::workflows::player::spawn_player::stages::validate_and_spawn_and_wait::core_types::Error".to_string()));
                                        let failure_sender = match failure_sender {
                                            Some(failure_sender) => failure_sender,None => {
                                                unreachable!("Ecs response handler error: Failure event send error: No failure sender provided");
                                            }
                                        };
                                        if let Err(send_err) = failure_sender.send(crate::workflow::events::StageFailureEvent {
                                            ty:crate::workflow::stage::StageType::EcsWhile,module_name,workflow_name,current_stage:0,stage_return:crate::workflow::stage::Stage::EcsWhile(stage),stage_error:error,
                                        }){
                                            unreachable!("EcsWhile response handler error: Failure event send error: {}",send_err);
                                        }
                                    }
                                }
                            })
                        }),setup_sender:crate::workflow::channels::get_stage_setup_sender().clone(),wait_sender:crate::workflow::channels::get_stage_wait_sender().clone(),completion_sender:crate::workflow::channels::get_stage_completion_sender().clone(),failure_sender:Some(crate::workflow::channels::get_stage_failure_sender().clone()),
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
            pub(super)use bevy::prelude::{
                Commands,Entity,Query,Res,ResMut
            };
            pub(super)use crate::{
                player::bundles::PlayerBundle,player::components::Player,follower::components::FollowerTarget,
            };
        }pub mod user_items {
            use super::workflow_imports:: * ;
        }pub mod stages {
            pub mod validate_and_spawn_and_wait {
                pub const NAME: &str = "ValidateAndSpawnAndWait";
                pub mod core_types {
                    use super::super::super::workflow_imports:: * ;
                    use bevy::prelude:: * ;
                    use thiserror::Error;
                    #[repr(C)]
                    pub struct State {
                        pub player_entity:Entity,
                    }
                    #[repr(C)]
                    pub struct Output {
                        pub player_entity:Entity,
                    }
                    #[derive(std::fmt::Debug,Error)]
                    pub enum Error {
                        PlayerAlreadySpawned
                    }
                    impl std::fmt::Display for Error {
                        fn fmt(&self,f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                            write!(f,"{:?}",self)
                        }
                    
                        }
                    #[derive(bevy::ecs::system::SystemParam)]
                    pub struct MainAccess<'w,'s>{
                        pub commands:Commands<'w,'s> ,pub player_query:Query<'w,'s, &'static Player> ,
                    }
                    static FILL_WORKFLOW_STAGE_BUFFER_SENDER:std::sync::OnceLock<FillWorkflowStageEcsWhileBufferEventSender>  = std::sync::OnceLock::new();
                    static FILL_WORKFLOW_STAGE_BUFFER_RECEIVER_CACHE:std::sync::OnceLock<std::sync::Mutex<Option<FillWorkflowStageEcsWhileBufferEventReceiver>> >  = std::sync::OnceLock::new();
                    pub fn pre_initialize_fill_workflow_stage_buffer_channel() -> FillWorkflowStageEcsWhileBufferEventSender {
                        let(tx,rx) = crossbeam_channel::bounded(1);
                        let sender = FillWorkflowStageEcsWhileBufferEventSender {
                            module_name:"player",workflow_name:"spawn_player",stage_index:0usize,sender:tx,
                        };
                        let receiver = FillWorkflowStageEcsWhileBufferEventReceiver(rx);
                        FILL_WORKFLOW_STAGE_BUFFER_SENDER.set(sender.clone()).expect("Sender already initialized!");
                        FILL_WORKFLOW_STAGE_BUFFER_RECEIVER_CACHE.set(std::sync::Mutex::new(Some(receiver))).expect("Receiver cache already initialized");
                        sender
                    }
                    pub fn take_fill_workflow_stage_buffer_receiver() -> FillWorkflowStageEcsWhileBufferEventReceiver {
                        let cache = FILL_WORKFLOW_STAGE_BUFFER_RECEIVER_CACHE.get().expect("Receiver cache not initialized");
                        let mut guard = cache.lock().unwrap();
                        guard.take().expect("Receiver already taken or never initialized")
                    }
                    pub fn get_fill_workflow_stage_buffer_sender() -> FillWorkflowStageEcsWhileBufferEventSender {
                        let sender = FILL_WORKFLOW_STAGE_BUFFER_SENDER.get().expect("Sender not initialized!");
                        let sender:Box<dyn crate::DynFillWorkflowStageEcsWhileBufferEventSender>  = dyn_clone::clone_box(sender);
                        if let Some(sender) = sender.as_any_ref().downcast_ref:: <FillWorkflowStageEcsWhileBufferEventSender>(){
                            sender.clone()
                        }else {
                            unreachable!("Sender was not the expected concrete type!");
                        }
                    }
                    pub fn receive_ecs_while_stages_to_ecs_while_buffers_system(mut receiver:ResMut<FillWorkflowStageEcsWhileBufferEventReceiver> ,mut buffer:ResMut<StageBuffer>){
                        match receiver.0.try_recv(){
                            Ok(event) => buffer.fill(event.module_name,event.workflow_name,event.stage_index,event.stage,event.stage_data),
                            Err(err) => match err {
                                crossbeam_channel::TryRecvError::Empty => {}
                                ,
                                crossbeam_channel::TryRecvError::Disconnected => {
                                    unreachable!("Receiver disconnected");
                                }
                            
                                }
                        
                            }
                    }
                    pub struct FillWorkflowStageEcsWhileBufferEvent {
                        module_name: &'static str,workflow_name: &'static str,stage_index:usize,stage:crate::workflow::stage::StageEcsWhile,stage_data:Option<crate::debug::types::AnySendSyncPremiumBox> ,
                    }
                    impl crate::FillWorkflowStageEcsWhileBufferEventMarker for FillWorkflowStageEcsWhileBufferEvent{}
                    
                    #[derive(Resource,Debug)]
                    pub struct FillWorkflowStageEcsWhileBufferEventReceiver(pub crossbeam_channel::Receiver<FillWorkflowStageEcsWhileBufferEvent>);
                    
                    #[derive(Clone,Debug)]
                    pub struct FillWorkflowStageEcsWhileBufferEventSender {
                        module_name: &'static str,workflow_name: &'static str,stage_index:usize,sender:crossbeam_channel::Sender<FillWorkflowStageEcsWhileBufferEvent>
                    }
                    impl crate::DynFillWorkflowStageEcsWhileBufferEventSender for FillWorkflowStageEcsWhileBufferEventSender {
                        fn module_name(&self) ->  &'static str {
                            self.module_name
                        }
                        fn workflow_name(&self) ->  &'static str {
                            self.workflow_name
                        }
                        fn stage_index(&self) -> usize {
                            self.stage_index
                        }
                        fn send(&self,module_name: &'static str,workflow_name: &'static str,stage_index:usize,stage:crate::workflow::stage::StageEcsWhile,stage_buffer:Option<crate::debug::types::AnySendSyncPremiumBox>){
                            let event = FillWorkflowStageEcsWhileBufferEvent {
                                module_name,workflow_name,stage_index,stage,stage_data:stage_buffer
                            };
                            if let Err(err) = self.sender.send(event){
                                unreachable!("Failed to send FillWorkflowStageEcsWhileBufferEvent: {}",err);
                            };
                        }
                        fn as_any_ref(&self) ->  &dyn std::any::Any {
                            self
                        }
                    
                        }
                    #[derive(Resource,Default)]
                    pub enum StageBuffer {
                        #[default]
                        None,Some {
                            module_name: &'static str,workflow_name: &'static str,stage_index:usize,stage:crate::workflow::stage::StageEcsWhile,stage_data:Option<crate::debug::types::AnySendSyncPremiumBox> ,
                        }
                    }
                    impl StageBuffer {
                        pub fn fill(&mut self,module_name: &'static str,workflow_name: &'static str,stage_index:usize,stage:crate::workflow::stage::StageEcsWhile,stage_data:Option<crate::debug::types::AnySendSyncPremiumBox> ,){
                            match std::mem::take(self){
                                StageBuffer::None => {
                                    *self = StageBuffer::Some {
                                        module_name,workflow_name,stage_index,stage,stage_data,
                                    }
                                },
                                StageBuffer::Some {
                                    ..
                                } => unreachable!("StageEcsWhile buffer is not empty")
                            
                                }
                        }
                        pub fn empty(&mut self,) -> (&'static str, &'static str,usize,crate::workflow::stage::StageEcsWhile,Option<crate::debug::types::AnySendSyncPremiumBox> ,){
                            match std::mem::take(self){
                                StageBuffer::None => {
                                    unreachable!("StageEcsWhile buffer is not filled");
                                }
                                StageBuffer::Some {
                                    module_name,workflow_name,stage_index,stage,stage_data,
                                } => {
                                    (module_name,workflow_name,stage_index,stage,stage_data,)
                                }
                            
                                }
                        }
                        pub fn is_empty(&self) -> bool {
                            matches!(self,StageBuffer::None)
                        }
                    
                        }
                
                    }pub mod core_functions {
                    use super::super::super::workflow_imports:: * ;
                    use super::core_types:: * ;
                    pub fn poll_ecs_while_stage_buffer_system(mut stage_buffer:bevy::prelude::ResMut<StageBuffer> ,mut workflow_map:ResMut<crate::workflow::resources::WorkflowMap> ,main_access:MainAccess){
                        if stage_buffer.is_empty(){
                            return;
                        }let setup_sender = crate::workflow::channels::get_stage_setup_sender();
                        let wait_sender = crate::workflow::channels::get_stage_wait_sender();
                        let completion_sender = crate::workflow::channels::get_stage_completion_sender();
                        let failure_sender = crate::workflow::channels::get_stage_failure_sender();
                        let(module_name,workflow_name,current_stage,mut stage,data_buffer) = stage_buffer.empty();
                        let workflow_instance = workflow_map.map.get_mut(module_name).and_then(|workflows|workflows.get_mut(workflow_name)).unwrap();
                        let workflow_state =  &mut workflow_instance.state();
                        let(stage_initialized,stage_completed) = match workflow_state {
                            crate::workflow::types::WorkflowState::Requested => {
                                unreachable!("Unexpected workflow state. Expected 'crate::workflow::types::WorkflowState::Processing', got '{:?}'",workflow_instance.state());
                            }
                            crate::workflow::types::WorkflowState::Processing {
                                current_stage:_,current_stage_type:_,stage_initialized,stage_completed,
                            } => (stage_initialized,stage_completed),
                        
                            };
                        if*stage_completed {
                            return;
                        }if! *stage_initialized {
                            let handle_ecs_while_setup_response =  &mut stage.handle_ecs_while_setup_response;
                            let response = setup_ecs_while(main_access);
                            let handler = (handle_ecs_while_setup_response)(module_name,workflow_name,response,setup_sender,Some(failure_sender),);
                            let handle = crate::workflow::statics::TOKIO_RUNTIME.lock().unwrap().handle().clone();
                            handle.spawn(async move {
                                handler(stage);
                            });
                            *stage_initialized = true;
                        }else {
                            let handle_ecs_while_run_response =  &mut stage.handle_ecs_while_run_response;
                            let state = data_buffer;
                            let response = run_ecs_while(state,main_access);
                            let handler = (handle_ecs_while_run_response)(module_name,workflow_name,response,wait_sender,completion_sender,Some(failure_sender),);
                            let handle = crate::workflow::statics::TOKIO_RUNTIME.lock().unwrap().handle().clone();
                            handle.spawn(async move {
                                handler(stage);
                            });
                        }
                    }
                    fn setup_ecs_while(main_access:MainAccess) -> Option<crate::debug::types::AnySendSyncPremiumBox>{
                        let result = setup_ecs_while_inner(main_access);
                        Some(crate::debug::types::AnySendSyncPremiumBox::new(result,format!("Result<{}, {}>","crate::player::workflows::player::spawn_player::stages::validate_and_spawn_and_wait::core_types::State".to_string(),"crate::player::workflows::player::spawn_player::stages::validate_and_spawn_and_wait::core_types::Error".to_string()).to_string()))
                    }
                    fn setup_ecs_while_inner(main_access:MainAccess) -> Result<State,Error>{
                        let mut commands = main_access.commands;
                        let player_query = main_access.player_query;
                        if!player_query.is_empty(){
                            return Err(Error::PlayerAlreadySpawned);
                        }let player_bundle = PlayerBundle::default();
                        let player_entity = player_bundle.chunk_loader().chunk_owner_id().entity();
                        commands.entity(player_entity).insert((player_bundle,FollowerTarget {
                            id:"main_camera".to_string(),
                        },));
                        bevy::prelude::debug!("Spawned player entity: {:?}",player_entity);
                        Ok(State {
                            player_entity
                        })
                    }
                    fn run_ecs_while(state:Option<crate::debug::types::AnySendSyncPremiumBox> ,main_access:MainAccess) -> Option<crate::debug::types::AnySendSyncPremiumBox>{
                        let state = state.unwrap().into_inner:: <State>();
                        let outcome_result = run_ecs_while_inner(state,main_access);
                        Some(crate::debug::types::AnySendSyncPremiumBox::new(outcome_result,format!("Result<Outcome<{}, {}>, {}>","crate::player::workflows::player::spawn_player::stages::validate_and_spawn_and_wait::core_types::State".to_string(),"crate::player::workflows::player::spawn_player::stages::validate_and_spawn_and_wait::core_types::Output".to_string(),"crate::player::workflows::player::spawn_player::stages::validate_and_spawn_and_wait::core_types::Error".to_string()).to_string()))
                    }
                    fn run_ecs_while_inner(state:State,main_access:MainAccess) -> Result<Outcome<State,Output> ,Error>{
                        let mut commands = main_access.commands;
                        if commands.get_entity(state.player_entity).is_some(){
                            bevy::prelude::debug!("Player entity is ready: {:?}",state.player_entity);
                            Ok(Done(Output {
                                player_entity:state.player_entity
                            }))
                        }else {
                            bevy::prelude::debug!("Player entity is not ready yet: {:?}",state.player_entity);
                            Ok(Wait(state))
                        }
                    }
                
                    }
            }
        }
    }pub mod despawn_player {
        pub const NAME: &str = "DespawnPlayer";
        pub async fn run() -> Result<(), <TypeE as crate::workflow::traits::WorkflowTypeE> ::Error>{
            crate::workflow::functions::run_workflow_e:: <TypeE>().await
        }
        pub(crate)struct DespawnPlayerWorkflowPlugin;
        
        impl bevy::prelude::Plugin for DespawnPlayerWorkflowPlugin {
            fn build(&self,app: &mut bevy::prelude::App){
                use bevy::prelude::IntoSystemConfigs;
                app.insert_resource(stages::validate_and_despawn_and_wait::core_types::StageBuffer::default()).insert_resource(stages::validate_and_despawn_and_wait::core_types::take_fill_workflow_stage_buffer_receiver()).add_systems(bevy::prelude::Update,stages::validate_and_despawn_and_wait::core_types::receive_ecs_while_stages_to_ecs_while_buffers_system.before(stages::validate_and_despawn_and_wait::core_functions::poll_ecs_while_stage_buffer_system)).add_systems(bevy::prelude::Update,stages::validate_and_despawn_and_wait::core_functions::poll_ecs_while_stage_buffer_system);
            }
        
            }
        #[derive(std::fmt::Debug,thiserror::Error)]
        pub enum Error {
            ValidateAndDespawnAndWaitError(self::stages::validate_and_despawn_and_wait::core_types::Error)
        }
        impl std::fmt::Display for Error {
            fn fmt(&self,f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f,"{:?}",self)
            }
        
            }
        pub struct TypeE;
        
        impl crate::workflow::traits::WorkflowTypeE for TypeE {
            type Error = Error;
            const MODULE_NAME: &'static str = super::NAME;
            const WORKFLOW_NAME: &'static str = self::NAME;
        }
        impl TypeE {
            pub fn create_workflow() -> crate::workflow::types::WorkflowType {
                crate::workflow::types::WorkflowType {
                    name:self::NAME,stages:vec![crate::workflow::stage::Stage::EcsWhile(crate::workflow::stage::StageEcsWhile {
                        index:0,name:"ValidateAndDespawnAndWait",signature:crate::workflow::stage::StageSignature::E,handle_ecs_while_setup_response:Box::new(|module_name: &'static str,workflow_name: &'static str,response:Option<crate::debug::types::AnySendSyncPremiumBox> ,setup_sender:crossbeam_channel::Sender<crate::workflow::events::StageSetupEvent> ,failure_sender:Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>> , |{
                            Box::new(move|stage:crate::workflow::stage::StageEcsWhile|{
                                let response = response.expect("EcsWhile stages with state and error must have a response");
                                let result:Result<crate::player::workflows::player::despawn_player::stages::validate_and_despawn_and_wait::core_types::State,crate::player::workflows::player::despawn_player::stages::validate_and_despawn_and_wait::core_types::Error>  = response.into_inner();
                                match result {
                                    Ok(state) => {
                                        let state = Some(crate::debug::types::AnySendSyncPremiumBox::new(state,"crate::player::workflows::player::despawn_player::stages::validate_and_despawn_and_wait::core_types::State".to_string()));
                                        if let Err(send_err) = setup_sender.send(crate::workflow::events::StageSetupEvent {
                                            ty:crate::workflow::stage::StageType::EcsWhile,module_name,workflow_name,current_stage:0,stage_return:crate::workflow::stage::Stage::EcsWhile(stage),stage_state:state,
                                        }){
                                            unreachable!("EcsWhile response handler error: Setup event send error: {}",send_err);
                                        }
                                    }Err(error) => {
                                        let error = crate::workflow::response::TypedWorkflowResponseE {
                                            module_name,workflow_name,result:Err(crate::debug::types::AnySendSyncPremiumBox::new(crate::player::workflows::player::despawn_player::Error::ValidateAndDespawnAndWaitError(error),"crate::player::workflows::player::despawn_player::stages::validate_and_despawn_and_wait::core_types::Error".to_string()))
                                        };
                                        let error = Some(crate::debug::types::AnySendSyncPremiumBox::new(error,"crate::player::workflows::player::despawn_player::stages::validate_and_despawn_and_wait::core_types::Error".to_string()));
                                        let failure_sender = match failure_sender {
                                            Some(failure_sender) => failure_sender,None => {
                                                unreachable!("EcsWhile response handler error: Failure event send error: No failure sender provided");
                                            }
                                        };
                                        if let Err(send_err) = failure_sender.send(crate::workflow::events::StageFailureEvent {
                                            ty:crate::workflow::stage::StageType::EcsWhile,module_name,workflow_name,current_stage:0,stage_return:crate::workflow::stage::Stage::EcsWhile(stage),stage_error:error,
                                        }){
                                            unreachable!("EcsWhile response handler error: Failure event send error: {}",send_err);
                                        }
                                    }
                                }
                            })
                        }),handle_ecs_while_run_response:Box::new(|module_name: &'static str,workflow_name: &'static str,response:Option<crate::debug::types::AnySendSyncPremiumBox> ,wait_sender:crossbeam_channel::Sender<crate::workflow::events::StageWaitEvent> ,completion_sender:crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent> ,failure_sender:Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>> |{
                            Box::new(move|stage:crate::workflow::stage::StageEcsWhile|{
                                let response = response.expect("EcsWhile stages with error must have a response");
                                let outcome_result:Result<crate::workflow::types::Outcome<crate::player::workflows::player::despawn_player::stages::validate_and_despawn_and_wait::core_types::State,()> ,crate::player::workflows::player::despawn_player::stages::validate_and_despawn_and_wait::core_types::Error>  = response.into_inner();
                                match outcome_result {
                                    Ok(outcome) => {
                                        match outcome {
                                            crate::workflow::types::Outcome::Wait(state) => {
                                                let state = Some(crate::debug::types::AnySendSyncPremiumBox::new(state,"crate::player::workflows::player::despawn_player::stages::validate_and_despawn_and_wait::core_types::State".to_string()));
                                                if let Err(send_err) = wait_sender.send(crate::workflow::events::StageWaitEvent {
                                                    ty:crate::workflow::stage::StageType::EcsWhile,module_name,workflow_name,current_stage:0,stage_return:crate::workflow::stage::Stage::EcsWhile(stage),stage_state:state,
                                                }){
                                                    unreachable!("EcsWhile response handler error: Wait event send error: {}",send_err);
                                                }
                                            }crate::workflow::types::Outcome::Done(_) => {
                                                if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                                                    ty:crate::workflow::stage::StageType::EcsWhile,module_name,workflow_name,current_stage:0,stage_return:crate::workflow::stage::Stage::EcsWhile(stage),stage_output:None,
                                                }){
                                                    unreachable!("EcsWhile response handler error: Completion event send error: {}",send_err);
                                                }
                                            }
                                        }
                                    }Err(error) => {
                                        let error = crate::player::workflows::player::despawn_player::Error::ValidateAndDespawnAndWaitError(error);
                                        let error = Some(crate::debug::types::AnySendSyncPremiumBox::new(error,"crate::player::workflows::player::despawn_player::stages::validate_and_despawn_and_wait::core_types::Error".to_string()));
                                        let failure_sender = match failure_sender {
                                            Some(failure_sender) => failure_sender,None => {
                                                unreachable!("Ecs response handler error: Failure event send error: No failure sender provided");
                                            }
                                        };
                                        if let Err(send_err) = failure_sender.send(crate::workflow::events::StageFailureEvent {
                                            ty:crate::workflow::stage::StageType::EcsWhile,module_name,workflow_name,current_stage:0,stage_return:crate::workflow::stage::Stage::EcsWhile(stage),stage_error:error,
                                        }){
                                            unreachable!("EcsWhile response handler error: Failure event send error: {}",send_err);
                                        }
                                    }
                                }
                            })
                        }),setup_sender:crate::workflow::channels::get_stage_setup_sender().clone(),wait_sender:crate::workflow::channels::get_stage_wait_sender().clone(),completion_sender:crate::workflow::channels::get_stage_completion_sender().clone(),failure_sender:Some(crate::workflow::channels::get_stage_failure_sender().clone()),
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
            pub(super)use bevy::prelude::{
                Commands,Entity,Query,Res,ResMut,debug,DespawnRecursiveExt
            };
            pub(super)use crate::{
                chunk_loader::components::ChunkLoader,player::bundles::PlayerBundle,player::components::Player,follower::components::FollowerTarget,utils::DropHook,
            };
        }pub mod user_items {
            use super::workflow_imports:: * ;
        }pub mod stages {
            pub mod validate_and_despawn_and_wait {
                pub const NAME: &str = "ValidateAndDespawnAndWait";
                pub mod core_types {
                    use super::super::super::workflow_imports:: * ;
                    use bevy::prelude:: * ;
                    use thiserror::Error;
                    #[repr(C)]
                    pub struct State{}
                    
                    #[derive(std::fmt::Debug,Error)]
                    pub enum Error {
                        PlayerAlreadyMarkedForDespawn,PlayerAlreadyDespawned,
                    }
                    impl std::fmt::Display for Error {
                        fn fmt(&self,f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                            write!(f,"{:?}",self)
                        }
                    
                        }
                    #[derive(bevy::ecs::system::SystemParam)]
                    pub struct MainAccess<'w,'s>{
                        pub commands:Commands<'w,'s> ,pub chunk_loader_with_drop_hook_query:Query<'w,'s,Entity,(With<Player> ,Without<DropHook<ChunkLoader> >)> ,pub chunk_loader_without_drop_hook_query:Query<'w,'s,Entity,(With<Player> ,With<DropHook<ChunkLoader> >)> ,
                    }
                    static FILL_WORKFLOW_STAGE_BUFFER_SENDER:std::sync::OnceLock<FillWorkflowStageEcsWhileBufferEventSender>  = std::sync::OnceLock::new();
                    static FILL_WORKFLOW_STAGE_BUFFER_RECEIVER_CACHE:std::sync::OnceLock<std::sync::Mutex<Option<FillWorkflowStageEcsWhileBufferEventReceiver>> >  = std::sync::OnceLock::new();
                    pub fn pre_initialize_fill_workflow_stage_buffer_channel() -> FillWorkflowStageEcsWhileBufferEventSender {
                        let(tx,rx) = crossbeam_channel::bounded(1);
                        let sender = FillWorkflowStageEcsWhileBufferEventSender {
                            module_name:"player",workflow_name:"despawn_player",stage_index:0usize,sender:tx,
                        };
                        let receiver = FillWorkflowStageEcsWhileBufferEventReceiver(rx);
                        FILL_WORKFLOW_STAGE_BUFFER_SENDER.set(sender.clone()).expect("Sender already initialized!");
                        FILL_WORKFLOW_STAGE_BUFFER_RECEIVER_CACHE.set(std::sync::Mutex::new(Some(receiver))).expect("Receiver cache already initialized");
                        sender
                    }
                    pub fn take_fill_workflow_stage_buffer_receiver() -> FillWorkflowStageEcsWhileBufferEventReceiver {
                        let cache = FILL_WORKFLOW_STAGE_BUFFER_RECEIVER_CACHE.get().expect("Receiver cache not initialized");
                        let mut guard = cache.lock().unwrap();
                        guard.take().expect("Receiver already taken or never initialized")
                    }
                    pub fn get_fill_workflow_stage_buffer_sender() -> FillWorkflowStageEcsWhileBufferEventSender {
                        let sender = FILL_WORKFLOW_STAGE_BUFFER_SENDER.get().expect("Sender not initialized!");
                        let sender:Box<dyn crate::DynFillWorkflowStageEcsWhileBufferEventSender>  = dyn_clone::clone_box(sender);
                        if let Some(sender) = sender.as_any_ref().downcast_ref:: <FillWorkflowStageEcsWhileBufferEventSender>(){
                            sender.clone()
                        }else {
                            unreachable!("Sender was not the expected concrete type!");
                        }
                    }
                    pub fn receive_ecs_while_stages_to_ecs_while_buffers_system(mut receiver:ResMut<FillWorkflowStageEcsWhileBufferEventReceiver> ,mut buffer:ResMut<StageBuffer>){
                        match receiver.0.try_recv(){
                            Ok(event) => buffer.fill(event.module_name,event.workflow_name,event.stage_index,event.stage,event.stage_data),
                            Err(err) => match err {
                                crossbeam_channel::TryRecvError::Empty => {}
                                ,
                                crossbeam_channel::TryRecvError::Disconnected => {
                                    unreachable!("Receiver disconnected");
                                }
                            
                                }
                        
                            }
                    }
                    pub struct FillWorkflowStageEcsWhileBufferEvent {
                        module_name: &'static str,workflow_name: &'static str,stage_index:usize,stage:crate::workflow::stage::StageEcsWhile,stage_data:Option<crate::debug::types::AnySendSyncPremiumBox> ,
                    }
                    impl crate::FillWorkflowStageEcsWhileBufferEventMarker for FillWorkflowStageEcsWhileBufferEvent{}
                    
                    #[derive(Resource,Debug)]
                    pub struct FillWorkflowStageEcsWhileBufferEventReceiver(pub crossbeam_channel::Receiver<FillWorkflowStageEcsWhileBufferEvent>);
                    
                    #[derive(Clone,Debug)]
                    pub struct FillWorkflowStageEcsWhileBufferEventSender {
                        module_name: &'static str,workflow_name: &'static str,stage_index:usize,sender:crossbeam_channel::Sender<FillWorkflowStageEcsWhileBufferEvent>
                    }
                    impl crate::DynFillWorkflowStageEcsWhileBufferEventSender for FillWorkflowStageEcsWhileBufferEventSender {
                        fn module_name(&self) ->  &'static str {
                            self.module_name
                        }
                        fn workflow_name(&self) ->  &'static str {
                            self.workflow_name
                        }
                        fn stage_index(&self) -> usize {
                            self.stage_index
                        }
                        fn send(&self,module_name: &'static str,workflow_name: &'static str,stage_index:usize,stage:crate::workflow::stage::StageEcsWhile,stage_buffer:Option<crate::debug::types::AnySendSyncPremiumBox>){
                            let event = FillWorkflowStageEcsWhileBufferEvent {
                                module_name,workflow_name,stage_index,stage,stage_data:stage_buffer
                            };
                            if let Err(err) = self.sender.send(event){
                                unreachable!("Failed to send FillWorkflowStageEcsWhileBufferEvent: {}",err);
                            };
                        }
                        fn as_any_ref(&self) ->  &dyn std::any::Any {
                            self
                        }
                    
                        }
                    #[derive(Resource,Default)]
                    pub enum StageBuffer {
                        #[default]
                        None,Some {
                            module_name: &'static str,workflow_name: &'static str,stage_index:usize,stage:crate::workflow::stage::StageEcsWhile,stage_data:Option<crate::debug::types::AnySendSyncPremiumBox> ,
                        }
                    }
                    impl StageBuffer {
                        pub fn fill(&mut self,module_name: &'static str,workflow_name: &'static str,stage_index:usize,stage:crate::workflow::stage::StageEcsWhile,stage_data:Option<crate::debug::types::AnySendSyncPremiumBox> ,){
                            match std::mem::take(self){
                                StageBuffer::None => {
                                    *self = StageBuffer::Some {
                                        module_name,workflow_name,stage_index,stage,stage_data,
                                    }
                                },
                                StageBuffer::Some {
                                    ..
                                } => unreachable!("StageEcsWhile buffer is not empty")
                            
                                }
                        }
                        pub fn empty(&mut self,) -> (&'static str, &'static str,usize,crate::workflow::stage::StageEcsWhile,Option<crate::debug::types::AnySendSyncPremiumBox> ,){
                            match std::mem::take(self){
                                StageBuffer::None => {
                                    unreachable!("StageEcsWhile buffer is not filled");
                                }
                                StageBuffer::Some {
                                    module_name,workflow_name,stage_index,stage,stage_data,
                                } => {
                                    (module_name,workflow_name,stage_index,stage,stage_data,)
                                }
                            
                                }
                        }
                        pub fn is_empty(&self) -> bool {
                            matches!(self,StageBuffer::None)
                        }
                    
                        }
                
                    }pub mod core_functions {
                    use super::super::super::workflow_imports:: * ;
                    use super::core_types:: * ;
                    pub fn poll_ecs_while_stage_buffer_system(mut stage_buffer:bevy::prelude::ResMut<StageBuffer> ,mut workflow_map:ResMut<crate::workflow::resources::WorkflowMap> ,main_access:MainAccess){
                        if stage_buffer.is_empty(){
                            return;
                        }let setup_sender = crate::workflow::channels::get_stage_setup_sender();
                        let wait_sender = crate::workflow::channels::get_stage_wait_sender();
                        let completion_sender = crate::workflow::channels::get_stage_completion_sender();
                        let failure_sender = crate::workflow::channels::get_stage_failure_sender();
                        let(module_name,workflow_name,current_stage,mut stage,data_buffer) = stage_buffer.empty();
                        let workflow_instance = workflow_map.map.get_mut(module_name).and_then(|workflows|workflows.get_mut(workflow_name)).unwrap();
                        let workflow_state =  &mut workflow_instance.state();
                        let(stage_initialized,stage_completed) = match workflow_state {
                            crate::workflow::types::WorkflowState::Requested => {
                                unreachable!("Unexpected workflow state. Expected 'crate::workflow::types::WorkflowState::Processing', got '{:?}'",workflow_instance.state());
                            }
                            crate::workflow::types::WorkflowState::Processing {
                                current_stage:_,current_stage_type:_,stage_initialized,stage_completed,
                            } => (stage_initialized,stage_completed),
                        
                            };
                        if*stage_completed {
                            return;
                        }if! *stage_initialized {
                            let handle_ecs_while_setup_response =  &mut stage.handle_ecs_while_setup_response;
                            let response = setup_ecs_while(main_access);
                            let handler = (handle_ecs_while_setup_response)(module_name,workflow_name,response,setup_sender,Some(failure_sender),);
                            let handle = crate::workflow::statics::TOKIO_RUNTIME.lock().unwrap().handle().clone();
                            handle.spawn(async move {
                                handler(stage);
                            });
                            *stage_initialized = true;
                        }else {
                            let handle_ecs_while_run_response =  &mut stage.handle_ecs_while_run_response;
                            let state = data_buffer;
                            let response = run_ecs_while(state,main_access);
                            let handler = (handle_ecs_while_run_response)(module_name,workflow_name,response,wait_sender,completion_sender,Some(failure_sender),);
                            let handle = crate::workflow::statics::TOKIO_RUNTIME.lock().unwrap().handle().clone();
                            handle.spawn(async move {
                                handler(stage);
                            });
                        }
                    }
                    fn setup_ecs_while(main_access:MainAccess) -> Option<crate::debug::types::AnySendSyncPremiumBox>{
                        let result = setup_ecs_while_inner(main_access);
                        Some(crate::debug::types::AnySendSyncPremiumBox::new(result,format!("Result<{}, {}>","crate::player::workflows::player::despawn_player::stages::validate_and_despawn_and_wait::core_types::State".to_string(),"crate::player::workflows::player::despawn_player::stages::validate_and_despawn_and_wait::core_types::Error".to_string()).to_string()))
                    }
                    fn setup_ecs_while_inner(main_access:MainAccess) -> Result<State,Error>{
                        let mut commands = main_access.commands;
                        let chunk_loader_with_drop_hook_query = main_access.chunk_loader_with_drop_hook_query;
                        let chunk_loader_without_drop_hook_query = main_access.chunk_loader_without_drop_hook_query;
                        match(chunk_loader_with_drop_hook_query.get_single().is_err(),chunk_loader_without_drop_hook_query.get_single().is_err()){
                            (true,true) => {
                                unreachable!()
                            },
                            (true,false) => {}
                            ,
                            (false,true) => {
                                return Err(Error::PlayerAlreadyMarkedForDespawn);
                            },
                            (false,false) => {
                                return Err(Error::PlayerAlreadyDespawned);
                            }
                        
                            }let player_entity = chunk_loader_without_drop_hook_query.single();
                        commands.entity(player_entity).insert(DropHook:: <ChunkLoader> ::default());
                        debug!("Marked player entity for despawning");
                        Ok(State{}
                        )
                    }
                    fn run_ecs_while(state:Option<crate::debug::types::AnySendSyncPremiumBox> ,main_access:MainAccess) -> Option<crate::debug::types::AnySendSyncPremiumBox>{
                        let state = state.unwrap().into_inner:: <State>();
                        let outcome_result = run_ecs_while_inner(state,main_access);
                        Some(crate::debug::types::AnySendSyncPremiumBox::new(outcome_result,format!("Result<Outcome<{}, ()>, {}>","crate::player::workflows::player::despawn_player::stages::validate_and_despawn_and_wait::core_types::State".to_string(),"crate::player::workflows::player::despawn_player::stages::validate_and_despawn_and_wait::core_types::Error".to_string()).to_string()))
                    }
                    fn run_ecs_while_inner(state:State,main_access:MainAccess) -> Result<Outcome<State,()> ,Error>{
                        let mut commands = main_access.commands;
                        let chunk_loader_with_drop_hook_query = main_access.chunk_loader_with_drop_hook_query;
                        let chunk_loader_without_drop_hook_query = main_access.chunk_loader_without_drop_hook_query;
                        match(chunk_loader_with_drop_hook_query.get_single().is_err(),chunk_loader_without_drop_hook_query.get_single().is_err()){
                            (true,true) => {
                                unreachable!()
                            },
                            (true,false) => {}
                            ,
                            (false,true) => return Ok(Wait(State{}
                            )),
                            (false,false) => {
                                return Err(Error::PlayerAlreadyDespawned);
                            }
                        
                            }let player_entity = chunk_loader_without_drop_hook_query.single();
                        commands.entity(player_entity).despawn_recursive();
                        debug!("Despawned player entity: {:?}",player_entity);
                        Ok(Done(()))
                    }
                
                    }
            }
        }
    }
}
