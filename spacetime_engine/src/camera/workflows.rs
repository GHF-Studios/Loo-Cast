use spacetime_engine_macros::define_workflow_mod_OLD;

pub mod camera {
    use bevy::prelude:: * ;
    pub const NAME: &str = "Camera";
    pub struct CameraWorkflowsPlugin;
    
    impl Plugin for CameraWorkflowsPlugin {
        fn build(&self,app: &mut App){
            app.add_systems(PreStartup,register_workflow_type_module).add_plugins(crate::camera::workflows::camera::spawn_main_camera::SpawnMainCameraWorkflowPlugin);
        }
    
        }
    fn register_workflow_type_module(mut workflow_type_module_registry:ResMut<crate::workflow::resources::WorkflowTypeModuleRegistry>){
        workflow_type_module_registry.register(crate::workflow::types::WorkflowTypeModule {
            name:"Camera",workflow_types:vec![spawn_main_camera::Type::create_workflow()],
        });
    }
    pub mod spawn_main_camera {
        pub const NAME: &str = "SpawnMainCamera";
        pub async fn run(){
            crate::workflow::functions::run_workflow:: <Type>().await
        }
        pub(crate)struct SpawnMainCameraWorkflowPlugin;
        
        impl bevy::prelude::Plugin for SpawnMainCameraWorkflowPlugin {
            fn build(&self,app: &mut bevy::prelude::App){
                use bevy::prelude::IntoSystemConfigs;
                app.insert_resource(stages::spawn::core_types::StageBuffer::default()).insert_resource(stages::spawn::core_types::take_fill_workflow_stage_buffer_receiver()).add_systems(bevy::prelude::Update,stages::spawn::core_types::receive_ecs_stages_to_ecs_buffers_system.before(stages::spawn::core_functions::poll_ecs_stage_buffer_system)).add_systems(bevy::prelude::Update,stages::spawn::core_functions::poll_ecs_stage_buffer_system);
            }
        
            }
        pub struct Type;
        
        impl crate::workflow::traits::WorkflowType for Type {
            const MODULE_NAME: &'static str = super::NAME;
            const WORKFLOW_NAME: &'static str = self::NAME;
        }
        impl Type {
            pub fn create_workflow() -> crate::workflow::types::WorkflowType {
                crate::workflow::types::WorkflowType {
                    name:self::NAME,stages:vec![crate::workflow::stage::Stage::Ecs(crate::workflow::stage::StageEcs {
                        index:0,name:"Spawn",signature:crate::workflow::stage::StageSignature::None,handle_ecs_run_response:Box::new(|module_name: &'static str,workflow_name: &'static str,_response:Option<Box<dyn std::any::Any+Send+Sync>> ,completion_sender:crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent> ,_failure_sender:Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>> |{
                            Box::new(move|stage:crate::workflow::stage::StageEcs|{
                                if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                                    ty:crate::workflow::stage::StageType::Ecs,module_name,workflow_name,current_stage:0,stage_return:crate::workflow::stage::Stage::Ecs(stage),stage_output:None,
                                }){
                                    unreachable!("Ecs response handler error: Completion event send error: {}",send_err);
                                }
                            })
                        }),completion_sender:crate::workflow::channels::get_stage_completion_sender().clone(),failure_sender:None,
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
            pub(super)use crate::camera::components::MainCamera;
            pub(super)use crate::config::statics::CONFIG;
            pub(super)use crate::follower::components::{
                FollowerComponent,FollowerTargetComponent
            };
        }pub mod user_items {
            use super::workflow_imports:: * ;
        }pub mod stages {
            pub mod spawn {
                pub const NAME: &str = "Spawn";
                pub mod core_types {
                    use super::super::super::workflow_imports:: * ;
                    use bevy::prelude:: * ;
                    #[derive(bevy::ecs::system::SystemParam)]
                    pub struct MainAccess<'w,'s>{
                        pub commands:Commands<'w,'s>
                    }
                    static FILL_WORKFLOW_STAGE_BUFFER_SENDER:std::sync::OnceLock<FillWorkflowStageEcsBufferEventSender>  = std::sync::OnceLock::new();
                    static FILL_WORKFLOW_STAGE_BUFFER_RECEIVER_CACHE:std::sync::OnceLock<std::sync::Mutex<Option<FillWorkflowStageEcsBufferEventReceiver>> >  = std::sync::OnceLock::new();
                    pub fn pre_initialize_fill_workflow_stage_buffer_channel() -> FillWorkflowStageEcsBufferEventSender {
                        let(tx,rx) = crossbeam_channel::bounded(1);
                        let sender = FillWorkflowStageEcsBufferEventSender {
                            module_name:"camera",workflow_name:"spawn_main_camera",stage_index:0usize,sender:tx,
                        };
                        let receiver = FillWorkflowStageEcsBufferEventReceiver(rx);
                        FILL_WORKFLOW_STAGE_BUFFER_SENDER.set(sender.clone()).expect("Sender already initialized!");
                        FILL_WORKFLOW_STAGE_BUFFER_RECEIVER_CACHE.set(std::sync::Mutex::new(Some(receiver))).expect("Receiver cache already initialized");
                        sender
                    }
                    pub fn take_fill_workflow_stage_buffer_receiver() -> FillWorkflowStageEcsBufferEventReceiver {
                        let cache = FILL_WORKFLOW_STAGE_BUFFER_RECEIVER_CACHE.get().expect("Receiver cache not initialized");
                        let mut guard = cache.lock().unwrap();
                        guard.take().expect("Receiver already taken or never initialized")
                    }
                    pub fn get_fill_workflow_stage_buffer_sender() -> FillWorkflowStageEcsBufferEventSender {
                        let sender = FILL_WORKFLOW_STAGE_BUFFER_SENDER.get().expect("Sender not initialized!");
                        let sender:Box<dyn crate::DynFillWorkflowStageEcsBufferEventSender>  = dyn_clone::clone_box(sender);
                        if let Some(sender) = sender.as_any_ref().downcast_ref:: <FillWorkflowStageEcsBufferEventSender>(){
                            sender.clone()
                        }else {
                            panic!("Sender was not the expected concrete type!");
                        }
                    }
                    pub fn receive_ecs_stages_to_ecs_buffers_system(mut receiver:ResMut<FillWorkflowStageEcsBufferEventReceiver> ,mut buffer:ResMut<StageBuffer>){
                        match receiver.0.try_recv(){
                            Ok(event) => buffer.fill(event.module_name,event.workflow_name,event.stage_index,event.stage,event.stage_data),
                            Err(err) => match err {
                                crossbeam_channel::TryRecvError::Empty => {}
                                ,
                                crossbeam_channel::TryRecvError::Disconnected => {
                                    panic!("Receiver disconnected");
                                }
                            
                                }
                        
                            }
                    }
                    pub struct FillWorkflowStageEcsBufferEvent {
                        module_name: &'static str,workflow_name: &'static str,stage_index:usize,stage:crate::workflow::stage::StageEcs,stage_data:Option<Box<dyn std::any::Any+Send+Sync>> ,
                    }
                    impl crate::FillWorkflowStageEcsBufferEventMarker for FillWorkflowStageEcsBufferEvent{}
                    
                    #[derive(Resource,Debug)]
                    pub struct FillWorkflowStageEcsBufferEventReceiver(pub crossbeam_channel::Receiver<FillWorkflowStageEcsBufferEvent>);
                    
                    #[derive(Clone,Debug)]
                    pub struct FillWorkflowStageEcsBufferEventSender {
                        module_name: &'static str,workflow_name: &'static str,stage_index:usize,sender:crossbeam_channel::Sender<FillWorkflowStageEcsBufferEvent>
                    }
                    impl crate::DynFillWorkflowStageEcsBufferEventSender for FillWorkflowStageEcsBufferEventSender {
                        fn module_name(&self) ->  &'static str {
                            self.module_name
                        }
                        fn workflow_name(&self) ->  &'static str {
                            self.workflow_name
                        }
                        fn stage_index(&self) -> usize {
                            self.stage_index
                        }
                        fn send(&self,module_name: &'static str,workflow_name: &'static str,stage_index:usize,stage:crate::workflow::stage::StageEcs,stage_buffer:Option<Box<dyn std::any::Any+Send+Sync>>){
                            let event = FillWorkflowStageEcsBufferEvent {
                                module_name,workflow_name,stage_index,stage,stage_data:stage_buffer
                            };
                            if let Err(err) = self.sender.send(event){
                                unreachable!("Failed to send FillWorkflowStageEcsBufferEvent: {}",err);
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
                            module_name: &'static str,workflow_name: &'static str,stage_index:usize,stage:crate::workflow::stage::StageEcs,stage_data:Option<Box<dyn std::any::Any+Send+Sync>> ,
                        }
                    }
                    impl StageBuffer {
                        pub fn fill(&mut self,module_name: &'static str,workflow_name: &'static str,stage_index:usize,stage:crate::workflow::stage::StageEcs,stage_data:Option<Box<dyn std::any::Any+Send+Sync>> ,){
                            match std::mem::take(self){
                                StageBuffer::None => {
                                    *self = StageBuffer::Some {
                                        module_name,workflow_name,stage_index,stage,stage_data,
                                    }
                                },
                                StageBuffer::Some {
                                    ..
                                } => unreachable!("StageEcs buffer is not empty")
                            
                                }
                        }
                        pub fn empty(&mut self,) -> (&'static str, &'static str,usize,crate::workflow::stage::StageEcs,Option<Box<dyn std::any::Any+Send+Sync>> ,){
                            match std::mem::take(self){
                                StageBuffer::None => {
                                    unreachable!("StageEcs buffer is not filled");
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
                    pub fn poll_ecs_stage_buffer_system(mut stage_buffer:bevy::prelude::ResMut<StageBuffer> ,main_access:MainAccess){
                        if stage_buffer.is_empty(){
                            return;
                        }let completion_sender = crate::workflow::channels::get_stage_completion_sender();
                        let(module_name,workflow_name,current_stage,mut stage,_data_buffer) = stage_buffer.empty();
                        let handle_ecs_run_response =  &mut stage.handle_ecs_run_response;
                        let _response = run_ecs(main_access);
                        let handler = (handle_ecs_run_response)(module_name,workflow_name,None,completion_sender,None,);
                        let scoped_ctx = crate::workflow::composite_workflow_context::ScopedCompositeWorkflowContext::new();
                        let handle = crate::workflow::statics::TOKIO_RUNTIME.lock().unwrap().handle().clone();
                        handle.spawn(async move {
                            scoped_ctx.run(||async {
                                handler(stage);
                            }).await;
                        });
                    }
                    fn run_ecs(main_access:MainAccess) -> Option<Box<dyn std::any::Any+Send+Sync>>{
                        run_ecs_inner(main_access);
                        None
                    }
                    fn run_ecs_inner(main_access:MainAccess){
                        let mut commands = main_access.commands;
                        commands.spawn((Camera2dBundle::default(),MainCamera,FollowerComponent::new("main_camera".to_string(),Vec2::ZERO,CONFIG.get:: <f32>("camera/follow_smoothness"),),));
                    }
                
                    }
            }
        }
    }
}
