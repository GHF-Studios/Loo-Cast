pub mod stages;
pub mod imports {
    pub use super::user_items::*;
    pub use crate::workflow::types::{Outcome, Outcome::Done, Outcome::Wait};
    pub use bevy::prelude::World;
}
pub mod user_items {
    use super::imports::*;
}

pub const NAME: &str = stringify!("GenerateTexture");

pub async fn run(input: <TypeIOE as crate::workflow::traits::WorkflowTypeIOE> ::Input) -> Result<<TypeIOE as crate::workflow::traits::WorkflowTypeIOE> ::Output, <TypeIOE as crate::workflow::traits::WorkflowTypeIOE> ::Error>{
    crate::workflow::functions::run_workflow_ioe:: <TypeIOE>(input).await
}

#[derive(std::fmt::Debug,thiserror::Error)]
pub enum Error {
    PrepareRequestError(self::stages::prepare_request::core_types::Error),WaitForComputeError(self::stages::wait_for_compute::core_types::Error)
}
impl std::fmt::Display for Error {
    fn fmt(&self,f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:?}",self)
    }
}

pub struct TypeIOE;
impl crate::workflow::traits::WorkflowTypeIOE for TypeIOE {
    type Input = self::stages::prepare_request::core_types::Input;
    type Output = self::stages::wait_for_compute::core_types::Output;
    type Error = Error;
    const MODULE_NAME: &'static str = super::NAME;
    const WORKFLOW_NAME: &'static str = self::NAME;
}
impl TypeIOE {
    pub fn create_workflow() -> crate::workflow::types::WorkflowType {
        crate::workflow::types::WorkflowType {
            name:self::NAME,stages:vec![crate::workflow::stage::Stage::Ecs(crate::workflow::stage::StageEcs {
                index:0,name:stringify!("PrepareRequest"),signature:crate::workflow::stage::StageSignature::IOE,run_ecs:Box::new(self::stages::prepare_request::core_functions::run_ecs)as Box<dyn FnMut(Option<Box<dyn std::any::Any+Send+Sync>> ,Box<dyn std::any::Any+Send+Sync>)->Option<Box<dyn std::any::Any+Send+Sync>> +Send+Sync> ,handle_ecs_run_response:Box::new(|module_name: &'static str,workflow_name: &'static str,response:Option<Box<dyn std::any::Any+Send+Sync>> ,completion_sender:crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent> ,failure_sender:Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>> |{
                    Box::new(move|stage:crate::workflow::stage::StageEcs|{
                        let response = response.expect("Ecs stages with output and error must have a response");
                        let result_data:Result<crate::gpu::workflows::gpu::generate_texture::stages::prepare_request::core_types::Output,crate::gpu::workflows::gpu::generate_texture::stages::prepare_request::core_types::Error>  =  *response.downcast().expect("Failed to downcast response result data");
                        match result_data {
                            Ok(output) => {
                                let output:crate::gpu::workflows::gpu::generate_texture::stages::get_texture_view::core_types::Input = unsafe {
                                    std::mem::transmute(output)
                                };
                                let output = Some(Box::new(output)as Box<dyn std::any::Any+Send+Sync>);
                                if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                                    ty:crate::workflow::stage::StageType::Ecs,module_name,workflow_name,current_stage:0,stage_return:crate::workflow::stage::Stage::Ecs(stage),stage_output:output,
                                }){
                                    unreachable!("Ecs response handler error: Completion event send error: {}",send_err);
                                }
                            }Err(error) => {
                                let error = crate::workflow::response::TypedWorkflowResponseOE(Err(Box::new(crate::gpu::workflows::gpu::generate_texture::Error::PrepareRequestError(error))));
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
            }),crate::workflow::stage::Stage::RenderWhile(crate::workflow::stage::StageRenderWhile {
                index:1,name:stringify!("GetTextureView"),signature:crate::workflow::stage::StageSignature::IO,setup_render_while:Box::new(self::stages::get_texture_view::core_functions::setup_render_while)as Box<dyn FnMut(Option<Box<dyn std::any::Any+Send+Sync>> ,Box<dyn std::any::Any+Send+Sync>)->Option<Box<dyn std::any::Any+Send+Sync>> +Send+Sync> ,run_render_while:Box::new(self::stages::get_texture_view::core_functions::run_render_while)as Box<dyn FnMut(Option<Box<dyn std::any::Any+Send+Sync>> ,Box<dyn std::any::Any+Send+Sync>)->Box<dyn std::any::Any+Send+Sync> +Send+Sync> ,handle_render_while_setup_response:Box::new(|module_name: &'static str,workflow_name: &'static str,response:Option<Box<dyn std::any::Any+Send+Sync>> ,setup_sender:crossbeam_channel::Sender<crate::workflow::events::StageSetupEvent> ,_failure_sender:Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>> |{
                    Box::new(move|stage:crate::workflow::stage::StageRenderWhile|{
                        let response = response.expect("RenderWhile stages with state must have a response");
                        let state:crate::gpu::workflows::gpu::generate_texture::stages::get_texture_view::core_types::State =  *response.downcast().expect("Failed to downcast response result data");
                        let state = Some(Box::new(state)as Box<dyn std::any::Any+Send+Sync>);
                        if let Err(send_err) = setup_sender.send(crate::workflow::events::StageSetupEvent {
                            ty:crate::workflow::stage::StageType::RenderWhile,module_name,workflow_name,current_stage:1,stage_return:crate::workflow::stage::Stage::RenderWhile(stage),stage_state:state,
                        }){
                            unreachable!("RenderWhile response handler error: Setup event send error: {}",send_err);
                        }
                    })
                }),handle_render_while_run_response:Box::new(|module_name: &'static str,workflow_name: &'static str,response:Option<Box<dyn std::any::Any+Send+Sync>> ,wait_sender:crossbeam_channel::Sender<crate::workflow::events::StageWaitEvent> ,completion_sender:crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent> ,_failure_sender:Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>> |{
                    Box::new(move|stage:crate::workflow::stage::StageRenderWhile|{
                        let response = response.expect("RenderWhile stages with output must have a response");
                        let outcome:crate::workflow::types::Outcome<crate::gpu::workflows::gpu::generate_texture::stages::get_texture_view::core_types::State,crate::gpu::workflows::gpu::generate_texture::stages::get_texture_view::core_types::Output>  =  *response.downcast().expect("Failed to downcast response outcome data");
                        match outcome {
                            crate::workflow::types::Outcome::Wait(state) => {
                                let state = Some(Box::new(state)as Box<dyn std::any::Any+Send+Sync>);
                                if let Err(send_err) = wait_sender.send(crate::workflow::events::StageWaitEvent {
                                    ty:crate::workflow::stage::StageType::RenderWhile,module_name,workflow_name,current_stage:1,stage_return:crate::workflow::stage::Stage::RenderWhile(stage),stage_state:state,
                                }){
                                    unreachable!("RenderWhile response handler error: Wait event send error: {}",send_err);
                                }
                            },crate::workflow::types::Outcome::Done(output) => {
                                let output:crate::gpu::workflows::gpu::generate_texture::stages::dispatch_compute::core_types::Input = unsafe {
                                    std::mem::transmute(output)
                                };
                                let output = Some(Box::new(output)as Box<dyn std::any::Any+Send+Sync>);
                                if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                                    ty:crate::workflow::stage::StageType::RenderWhile,module_name,workflow_name,current_stage:1,stage_return:crate::workflow::stage::Stage::RenderWhile(stage),stage_output:output,
                                }){
                                    unreachable!("RenderWhile response handler error: Completion event send error: {}",send_err);
                                }
                            }
                        }
                    })
                }),setup_sender:crate::workflow::channels::get_stage_setup_sender().clone(),wait_sender:crate::workflow::channels::get_stage_wait_sender().clone(),completion_sender:crate::workflow::channels::get_stage_completion_sender().clone(),failure_sender:None,
            }),crate::workflow::stage::Stage::Render(crate::workflow::stage::StageRender {
                index:2,name:stringify!("DispatchCompute"),signature:crate::workflow::stage::StageSignature::IO,run_render:Box::new(self::stages::dispatch_compute::core_functions::run_render)as Box<dyn FnMut(Option<Box<dyn std::any::Any+Send+Sync>> ,Box<dyn std::any::Any+Send+Sync>)->Option<Box<dyn std::any::Any+Send+Sync>> +Send+Sync> ,handle_render_run_response:Box::new(|module_name: &'static str,workflow_name: &'static str,response:Option<Box<dyn std::any::Any+Send+Sync>> ,completion_sender:crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent> ,_failure_sender:Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>> |{
                    Box::new(move|stage:crate::workflow::stage::StageRender|{
                        let response = response.expect("Render stages with output (last stage) must have a response");
                        let output:crate::gpu::workflows::gpu::generate_texture::stages::dispatch_compute::core_types::Output =  *response.downcast().expect("Failed to downcast response output data");
                        let output:crate::gpu::workflows::gpu::generate_texture::stages::wait_for_compute::core_types::Input = unsafe {
                            std::mem::transmute(output)
                        };
                        let output = Some(Box::new(output)as Box<dyn std::any::Any+Send+Sync>);
                        if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                            ty:crate::workflow::stage::StageType::Render,module_name,workflow_name,current_stage:2,stage_return:crate::workflow::stage::Stage::Render(stage),stage_output:output,
                        }){
                            unreachable!("Render response handler error: Completion event send error: {}",send_err);
                        }
                    })
                }),completion_sender:crate::workflow::channels::get_stage_completion_sender().clone(),failure_sender:None,
            }),crate::workflow::stage::Stage::EcsWhile(crate::workflow::stage::StageEcsWhile {
                index:3,name:stringify!("WaitForCompute"),signature:crate::workflow::stage::StageSignature::IOE,setup_ecs_while:Box::new(self::stages::wait_for_compute::core_functions::setup_ecs_while)as Box<dyn FnMut(Option<Box<dyn std::any::Any+Send+Sync>> ,Box<dyn std::any::Any+Send+Sync>)->Option<Box<dyn std::any::Any+Send+Sync>> +Send+Sync> ,run_ecs_while:Box::new(self::stages::wait_for_compute::core_functions::run_ecs_while)as Box<dyn FnMut(Option<Box<dyn std::any::Any+Send+Sync>> ,Box<dyn std::any::Any+Send+Sync>)->Box<dyn std::any::Any+Send+Sync> +Send+Sync> ,handle_ecs_while_setup_response:Box::new(|module_name: &'static str,workflow_name: &'static str,response:Option<Box<dyn std::any::Any+Send+Sync>> ,setup_sender:crossbeam_channel::Sender<crate::workflow::events::StageSetupEvent> ,failure_sender:Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>> , |{
                    Box::new(move|stage:crate::workflow::stage::StageEcsWhile|{
                        let response = response.expect("EcsWhile stages with state and error must have a response");
                        let result:Result<crate::gpu::workflows::gpu::generate_texture::stages::wait_for_compute::core_types::State,crate::gpu::workflows::gpu::generate_texture::stages::wait_for_compute::core_types::Error>  =  *response.downcast().expect("Failed to downcast response result data");
                        match result {
                            Ok(state) => {
                                let state = Some(Box::new(state)as Box<dyn std::any::Any+Send+Sync>);
                                if let Err(send_err) = setup_sender.send(crate::workflow::events::StageSetupEvent {
                                    ty:crate::workflow::stage::StageType::EcsWhile,module_name,workflow_name,current_stage:3,stage_return:crate::workflow::stage::Stage::EcsWhile(stage),stage_state:state,
                                }){
                                    unreachable!("EcsWhile response handler error: Setup event send error: {}",send_err);
                                }
                            }Err(error) => {
                                let error = crate::workflow::response::TypedWorkflowResponseE(Err(Box::new(crate::gpu::workflows::gpu::generate_texture::Error::WaitForComputeError(error))));
                                let error = Some(Box::new(error)as Box<dyn std::any::Any+Send+Sync>);
                                let failure_sender = match failure_sender {
                                    Some(failure_sender) => failure_sender,None => {
                                        unreachable!("EcsWhile response handler error: Failure event send error: No failure sender provided");
                                    }
                                };
                                if let Err(send_err) = failure_sender.send(crate::workflow::events::StageFailureEvent {
                                    ty:crate::workflow::stage::StageType::EcsWhile,module_name,workflow_name,current_stage:3,stage_return:crate::workflow::stage::Stage::EcsWhile(stage),stage_error:error,
                                }){
                                    unreachable!("EcsWhile response handler error: Failure event send error: {}",send_err);
                                }
                            }
                        }
                    })
                }),handle_ecs_while_run_response:Box::new(|module_name: &'static str,workflow_name: &'static str,response:Option<Box<dyn std::any::Any+Send+Sync>> ,wait_sender:crossbeam_channel::Sender<crate::workflow::events::StageWaitEvent> ,completion_sender:crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent> ,failure_sender:Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>> |{
                    Box::new(move|stage:crate::workflow::stage::StageEcsWhile, |{
                        let response = response.expect("EcsWhile stages with output and error (last stage) must have a response");
                        let outcome_result:Result<crate::workflow::types::Outcome<crate::gpu::workflows::gpu::generate_texture::stages::wait_for_compute::core_types::State,crate::gpu::workflows::gpu::generate_texture::stages::wait_for_compute::core_types::Output> ,crate::gpu::workflows::gpu::generate_texture::stages::wait_for_compute::core_types::Error>  =  *response.downcast().expect("Failed to downcast response result data");
                        match outcome_result {
                            Ok(outcome) => {
                                match outcome {
                                    crate::workflow::types::Outcome::Wait(state) => {
                                        let state = Some(Box::new(state)as Box<dyn std::any::Any+Send+Sync>);
                                        if let Err(send_err) = wait_sender.send(crate::workflow::events::StageWaitEvent {
                                            ty:crate::workflow::stage::StageType::EcsWhile,module_name,workflow_name,current_stage:3,stage_return:crate::workflow::stage::Stage::EcsWhile(stage),stage_state:state,
                                        }){
                                            unreachable!("EcsWhile response handler error: Wait event send error: {}",send_err);
                                        }
                                    },crate::workflow::types::Outcome::Done(output) => {
                                        let output = Some(Box::new(output)as Box<dyn std::any::Any+Send+Sync>);
                                        if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                                            ty:crate::workflow::stage::StageType::EcsWhile,module_name,workflow_name,current_stage:3,stage_return:crate::workflow::stage::Stage::EcsWhile(stage),stage_output:output,
                                        }){
                                            unreachable!("EcsWhile response handler error: Completion event send error: {}",send_err);
                                        }
                                    }
                                }
                            }Err(error) => {
                                let error = crate::workflow::response::TypedWorkflowResponseOE(Err(Box::new(crate::gpu::workflows::gpu::generate_texture::Error::WaitForComputeError(error))));
                                let error = Some(Box::new(error)as Box<dyn std::any::Any+Send+Sync>);
                                let failure_sender = match failure_sender {
                                    Some(failure_sender) => failure_sender,None => {
                                        unreachable!("Ecs response handler error: Failure event send error: No failure sender provided");
                                    }
                                };
                                if let Err(send_err) = failure_sender.send(crate::workflow::events::StageFailureEvent {
                                    ty:crate::workflow::stage::StageType::EcsWhile,module_name,workflow_name,current_stage:3,stage_return:crate::workflow::stage::Stage::EcsWhile(stage),stage_error:error,
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
