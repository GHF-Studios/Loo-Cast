pub mod stages;
pub mod imports {
    pub use super::user_items::*;
    pub use crate::workflow::types::{Outcome, Outcome::Done, Outcome::Wait};
    pub use bevy::prelude::World;
}
pub mod user_items {
    use super::imports::*;
}

pub const NAME: &str = stringify!("SetupTextureGenerator");

pub async fn run(input: <TypeIE as crate::workflow::traits::WorkflowTypeIE> ::Input) -> Result<(), <TypeIE as crate::workflow::traits::WorkflowTypeIE> ::Error>{
    crate::workflow::functions::run_workflow_ie:: <TypeIE>(input).await
}

#[derive(std::fmt::Debug,thiserror::Error)]
pub enum Error {
    SetupPhase1Error(self::stages::setup_phase1::core_types::Error),SetupPhase2Error(self::stages::setup_phase2::core_types::Error)
}
impl std::fmt::Display for Error {
    fn fmt(&self,f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:?}",self)
    }
}

pub struct TypeIE;
impl crate::workflow::traits::WorkflowTypeIE for TypeIE {
    type Input = self::stages::setup_phase1::core_types::Input;
    type Error = Error;
    const MODULE_NAME: &'static str = super::NAME;
    const WORKFLOW_NAME: &'static str = self::NAME;
}
impl TypeIE {
    pub fn create_workflow() -> crate::workflow::types::WorkflowType {
        crate::workflow::types::WorkflowType {
            name:self::NAME,stages:vec![crate::workflow::stage::Stage::Ecs(crate::workflow::stage::StageEcs {
                index:0,name:stringify!("SetupPhase1"),signature:crate::workflow::stage::StageSignature::IOE,run_ecs:Box::new(self::stages::setup_phase1::core_functions::run_ecs)as Box<dyn FnMut(Option<Box<dyn std::any::Any+Send+Sync>> ,Box<dyn std::any::Any+Send+Sync>)->Option<Box<dyn std::any::Any+Send+Sync>> +Send+Sync> ,handle_ecs_run_response:Box::new(|module_name: &'static str,workflow_name: &'static str,response:Option<Box<dyn std::any::Any+Send+Sync>> ,completion_sender:crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent> ,failure_sender:Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>> |{
                    Box::new(move|stage:crate::workflow::stage::StageEcs|{
                        let response = response.expect("Ecs stages with output and error must have a response");
                        let result_data:Result<crate::gpu::workflows::gpu::setup_texture_generator::stages::setup_phase1::core_types::Output,crate::gpu::workflows::gpu::setup_texture_generator::stages::setup_phase1::core_types::Error>  =  *response.downcast().expect("Failed to downcast response result data");
                        match result_data {
                            Ok(output) => {
                                let output:crate::gpu::workflows::gpu::setup_texture_generator::stages::setup_phase2::core_types::Input = unsafe {
                                    std::mem::transmute(output)
                                };
                                let output = Some(Box::new(output)as Box<dyn std::any::Any+Send+Sync>);
                                if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                                    ty:crate::workflow::stage::StageType::Ecs,module_name,workflow_name,current_stage:0,stage_return:crate::workflow::stage::Stage::Ecs(stage),stage_output:output,
                                }){
                                    unreachable!("Ecs response handler error: Completion event send error: {}",send_err);
                                }
                            }Err(error) => {
                                let error = crate::workflow::response::TypedWorkflowResponseOE(Err(Box::new(crate::gpu::workflows::gpu::setup_texture_generator::Error::SetupPhase1Error(error))));
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
                index:1,name:stringify!("SetupPhase2"),signature:crate::workflow::stage::StageSignature::IOE,setup_render_while:Box::new(self::stages::setup_phase2::core_functions::setup_render_while)as Box<dyn FnMut(Option<Box<dyn std::any::Any+Send+Sync>> ,Box<dyn std::any::Any+Send+Sync>)->Option<Box<dyn std::any::Any+Send+Sync>> +Send+Sync> ,run_render_while:Box::new(self::stages::setup_phase2::core_functions::run_render_while)as Box<dyn FnMut(Option<Box<dyn std::any::Any+Send+Sync>> ,Box<dyn std::any::Any+Send+Sync>)->Box<dyn std::any::Any+Send+Sync> +Send+Sync> ,handle_render_while_setup_response:Box::new(|module_name: &'static str,workflow_name: &'static str,response:Option<Box<dyn std::any::Any+Send+Sync>> ,setup_sender:crossbeam_channel::Sender<crate::workflow::events::StageSetupEvent> ,failure_sender:Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>> , |{
                    Box::new(move|stage:crate::workflow::stage::StageRenderWhile|{
                        let response = response.expect("RenderWhile stages with state and error must have a response");
                        let result:Result<crate::gpu::workflows::gpu::setup_texture_generator::stages::setup_phase2::core_types::State,crate::gpu::workflows::gpu::setup_texture_generator::stages::setup_phase2::core_types::Error>  =  *response.downcast().expect("Failed to downcast response result data");
                        match result {
                            Ok(state) => {
                                let state = Some(Box::new(state)as Box<dyn std::any::Any+Send+Sync>);
                                if let Err(send_err) = setup_sender.send(crate::workflow::events::StageSetupEvent {
                                    ty:crate::workflow::stage::StageType::RenderWhile,module_name,workflow_name,current_stage:1,stage_return:crate::workflow::stage::Stage::RenderWhile(stage),stage_state:state,
                                }){
                                    unreachable!("RenderWhile response handler error: Setup event send error: {}",send_err);
                                }
                            }Err(error) => {
                                let error = crate::workflow::response::TypedWorkflowResponseE(Err(Box::new(crate::gpu::workflows::gpu::setup_texture_generator::Error::SetupPhase2Error(error))));
                                let error = Some(Box::new(error)as Box<dyn std::any::Any+Send+Sync>);
                                let failure_sender = match failure_sender {
                                    Some(failure_sender) => failure_sender,None => {
                                        unreachable!("RenderWhile response handler error: Failure event send error: No failure sender provided");
                                    }
                                };
                                if let Err(send_err) = failure_sender.send(crate::workflow::events::StageFailureEvent {
                                    ty:crate::workflow::stage::StageType::RenderWhile,module_name,workflow_name,current_stage:1,stage_return:crate::workflow::stage::Stage::RenderWhile(stage),stage_error:error,
                                }){
                                    unreachable!("RenderWhile response handler error: Failure event send error: {}",send_err);
                                }
                            }
                        }
                    })
                }),handle_render_while_run_response:Box::new(|module_name: &'static str,workflow_name: &'static str,response:Option<Box<dyn std::any::Any+Send+Sync>> ,wait_sender:crossbeam_channel::Sender<crate::workflow::events::StageWaitEvent> ,completion_sender:crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent> ,failure_sender:Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>> |{
                    Box::new(move|stage:crate::workflow::stage::StageRenderWhile|{
                        let response = response.expect("RenderWhile stages with output and error must have a response");
                        let outcome_result:Result<crate::workflow::types::Outcome<crate::gpu::workflows::gpu::setup_texture_generator::stages::setup_phase2::core_types::State,crate::gpu::workflows::gpu::setup_texture_generator::stages::setup_phase2::core_types::Output> ,crate::gpu::workflows::gpu::setup_texture_generator::stages::setup_phase2::core_types::Error>  =  *response.downcast().expect("Failed to downcast response result data");
                        match outcome_result {
                            Ok(outcome) => {
                                match outcome {
                                    crate::workflow::types::Outcome::Wait(state) => {
                                        let state = Some(Box::new(state)as Box<dyn std::any::Any+Send+Sync>);
                                        if let Err(send_err) = wait_sender.send(crate::workflow::events::StageWaitEvent {
                                            ty:crate::workflow::stage::StageType::RenderWhile,module_name,workflow_name,current_stage:1,stage_return:crate::workflow::stage::Stage::RenderWhile(stage),stage_state:state,
                                        }){
                                            unreachable!("RenderWhile response handler error: Wait event send error: {}",send_err);
                                        }
                                    },crate::workflow::types::Outcome::Done(output) => {
                                        let output:crate::gpu::workflows::gpu::setup_texture_generator::stages::setup_phase3::core_types::Input = unsafe {
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
                            }Err(error) => {
                                let error = crate::workflow::response::TypedWorkflowResponseOE(Err(Box::new(crate::gpu::workflows::gpu::setup_texture_generator::Error::SetupPhase2Error(error))));
                                let error = Some(Box::new(error)as Box<dyn std::any::Any+Send+Sync>);
                                let failure_sender = match failure_sender {
                                    Some(failure_sender) => failure_sender,None => {
                                        unreachable!("Ecs response handler error: Failure event send error: No failure sender provided");
                                    }
                                };
                                if let Err(send_err) = failure_sender.send(crate::workflow::events::StageFailureEvent {
                                    ty:crate::workflow::stage::StageType::RenderWhile,module_name,workflow_name,current_stage:1,stage_return:crate::workflow::stage::Stage::RenderWhile(stage),stage_error:error,
                                }){
                                    unreachable!("RenderWhile response handler error: Failure event send error: {}",send_err);
                                }
                            }
                        }
                    })
                }),setup_sender:crate::workflow::channels::get_stage_setup_sender().clone(),wait_sender:crate::workflow::channels::get_stage_wait_sender().clone(),completion_sender:crate::workflow::channels::get_stage_completion_sender().clone(),failure_sender:Some(crate::workflow::channels::get_stage_failure_sender().clone()),
            }),crate::workflow::stage::Stage::Ecs(crate::workflow::stage::StageEcs {
                index:2,name:stringify!("SetupPhase3"),signature:crate::workflow::stage::StageSignature::I,run_ecs:Box::new(self::stages::setup_phase3::core_functions::run_ecs)as Box<dyn FnMut(Option<Box<dyn std::any::Any+Send+Sync>> ,Box<dyn std::any::Any+Send+Sync>)->Option<Box<dyn std::any::Any+Send+Sync>> +Send+Sync> ,handle_ecs_run_response:Box::new(|module_name: &'static str,workflow_name: &'static str,_response:Option<Box<dyn std::any::Any+Send+Sync>> ,completion_sender:crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent> ,_failure_sender:Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>> |{
                    Box::new(move|stage:crate::workflow::stage::StageEcs|{
                        if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                            ty:crate::workflow::stage::StageType::Ecs,module_name,workflow_name,current_stage:2,stage_return:crate::workflow::stage::Stage::Ecs(stage),stage_output:None,
                        }){
                            unreachable!("Ecs response handler error: Completion event send error: {}",send_err);
                        }
                    })
                }),completion_sender:crate::workflow::channels::get_stage_completion_sender().clone(),failure_sender:None,
            })],
        }
    }
}
