pub mod stages;
pub mod imports {
    pub use super::user_items::*;
    pub use crate::workflow::types::{Outcome, Outcome::Done, Outcome::Wait};
    pub use bevy::prelude::World;
}
pub mod user_items {
    use super::imports::*;
}

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
                            }
                            Err(error) => {
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
