pub mod stages;
pub mod imports {
    pub use super::user_items::*;
    pub use crate::workflow::types::{Outcome, Outcome::Done, Outcome::Wait};
    pub use bevy::prelude::World;
}
pub mod user_items {
    use super::imports::*;
}

pub const NAME: &str = stringify!("SpawnDebugUI");
pub async fn run(){
    crate::workflow::functions::run_workflow:: <Type>().await
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
                index:0,name:stringify!("ValidateAndSpawn"),signature:crate::workflow::stage::StageSignature::None,run_ecs:Box::new(self::stages::validate_and_spawn::core_functions::run_ecs)as Box<dyn FnMut(Option<Box<dyn std::any::Any+Send+Sync>> ,Box<dyn std::any::Any+Send+Sync>)->Option<Box<dyn std::any::Any+Send+Sync>> +Send+Sync> ,handle_ecs_run_response:Box::new(|module_name: &'static str,workflow_name: &'static str,_response:Option<Box<dyn std::any::Any+Send+Sync>> ,completion_sender:crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent> ,_failure_sender:Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>> |{
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
