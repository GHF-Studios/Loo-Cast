use tokio::task::JoinHandle;

use crate::workflow::composite_workflow_context::ScopedCompositeWorkflowContext;

pub struct ChunkActionWorkflowHandles {
    pub spawn: Option<JoinHandle<ScopedCompositeWorkflowContext>>,
    pub despawn: Option<JoinHandle<ScopedCompositeWorkflowContext>>,
    pub transfer: Option<JoinHandle<ScopedCompositeWorkflowContext>>,
}
