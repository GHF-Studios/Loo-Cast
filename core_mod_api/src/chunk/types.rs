use bevy::prelude::*;
use tokio::task::JoinHandle;

use crate::workflow::composite_workflow_context::ScopedCompositeWorkflowContext;

#[derive(Reflect)]
pub struct ChunkActionWorkflowHandles {
    #[reflect(ignore)]
    pub spawn: Option<JoinHandle<ScopedCompositeWorkflowContext>>,
    #[reflect(ignore)]
    pub despawn: Option<JoinHandle<ScopedCompositeWorkflowContext>>,
    #[reflect(ignore)]
    pub transfer: Option<JoinHandle<ScopedCompositeWorkflowContext>>,
}
