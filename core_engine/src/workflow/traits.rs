use std::any::Any;

use bevy::ecs::world::World;
use crossbeam_channel::Sender;

use super::types::Outcome;

// TODO: MAYBE: THIS: Rename all to "Workflow"
// --- Workflow Types ---
pub trait WorkflowType {
    const MODULE_NAME: &'static str;
    const WORKFLOW_NAME: &'static str;
}
pub trait WorkflowTypeE {
    type Error: 'static + Send + Sync;

    const MODULE_NAME: &'static str;
    const WORKFLOW_NAME: &'static str;
}
pub trait WorkflowTypeO {
    type Output: 'static + Send + Sync;

    const MODULE_NAME: &'static str;
    const WORKFLOW_NAME: &'static str;
}
pub trait WorkflowTypeOE {
    type Output: 'static + Send + Sync;
    type Error: 'static + Send + Sync;

    const MODULE_NAME: &'static str;
    const WORKFLOW_NAME: &'static str;
}
pub trait WorkflowTypeI {
    type Input: 'static + Send + Sync;

    const MODULE_NAME: &'static str;
    const WORKFLOW_NAME: &'static str;
}
pub trait WorkflowTypeIE {
    type Input: 'static + Send + Sync;
    type Error: 'static + Send + Sync;

    const MODULE_NAME: &'static str;
    const WORKFLOW_NAME: &'static str;
}
pub trait WorkflowTypeIO {
    type Input: 'static + Send + Sync;
    type Output: 'static + Send + Sync;

    const MODULE_NAME: &'static str;
    const WORKFLOW_NAME: &'static str;
}
pub trait WorkflowTypeIOE {
    type Input: 'static + Send + Sync;
    type Output: 'static + Send + Sync;
    type Error: 'static + Send + Sync;

    const MODULE_NAME: &'static str;
    const WORKFLOW_NAME: &'static str;
}

// TODO: MAYBE: THAT: Rename all to "WorkflowStage*Type"
// --- Ecs Workflow Stages ---
pub trait WorkflowStageEcs {
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_ecs(&mut self, world: &mut World);
    fn handle_ecs_run_response(
        &mut self,
        stage_response: Option<crate::debug::types::AnySendSyncNamedBox>,
        completion_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageEcs,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
        failure_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageEcs,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
    );
}
pub trait WorkflowStageEcsE {
    type Error: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_ecs(&mut self, world: &mut World) -> Result<(), Self::Error>;
    fn handle_ecs_run_response(
        &mut self,
        stage_response: Option<crate::debug::types::AnySendSyncNamedBox>,
        completion_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageEcs,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
        failure_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageEcs,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
    );
}
pub trait WorkflowStageEcsO {
    type Output: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_ecs(&mut self, world: &mut World) -> Self::Output;
    fn handle_ecs_run_response(
        &mut self,
        stage_response: Option<crate::debug::types::AnySendSyncNamedBox>,
        completion_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageEcs,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
        failure_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageEcs,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
    );
}
pub trait WorkflowStageEcsOE {
    type Output: 'static + Send + Sync;
    type Error: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_ecs(&mut self, world: &mut World) -> Result<Self::Output, Self::Error>;
    fn handle_ecs_run_response(
        &mut self,
        stage_response: Option<crate::debug::types::AnySendSyncNamedBox>,
        completion_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageEcs,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
        failure_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageEcs,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
    );
}
pub trait WorkflowStageEcsI {
    type Input: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_ecs(&mut self, input: Self::Input, world: &mut World);
    fn handle_ecs_run_response(
        &mut self,
        stage_response: Option<crate::debug::types::AnySendSyncNamedBox>,
        completion_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageEcs,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
        failure_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageEcs,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
    );
}
pub trait WorkflowStageEcsIE {
    type Input: 'static + Send + Sync;
    type Error: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_ecs(&mut self, input: Self::Input, world: &mut World) -> Result<(), Self::Error>;
    fn handle_ecs_run_response(
        &mut self,
        stage_response: Option<crate::debug::types::AnySendSyncNamedBox>,
        completion_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageEcs,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
        failure_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageEcs,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
    );
}
pub trait WorkflowStageEcsIO {
    type Input: 'static + Send + Sync;
    type Output: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_ecs(&mut self, input: Self::Input, world: &mut World) -> Self::Output;
    fn handle_ecs_run_response(
        &mut self,
        stage_response: Option<crate::debug::types::AnySendSyncNamedBox>,
        completion_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageEcs,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
        failure_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageEcs,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
    );
}
pub trait WorkflowStageEcsIOE {
    type Input: 'static + Send + Sync;
    type Output: 'static + Send + Sync;
    type Error: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_ecs(
        &mut self,
        input: Self::Input,
        world: &mut World,
    ) -> Result<Self::Output, Self::Error>;
    fn handle_ecs_run_response(
        &mut self,
        stage_response: Option<crate::debug::types::AnySendSyncNamedBox>,
        completion_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageEcs,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
        failure_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageEcs,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
    );
}

// --- Render Workflow Stages ---
pub trait WorkflowStageRender {
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, world: &mut World);
    fn handle_render_run_response(
        &mut self,
        stage_response: Option<crate::debug::types::AnySendSyncNamedBox>,
        completion_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageRender,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
        failure_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageRender,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
    );
}
pub trait WorkflowStageRenderE {
    type Error: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, world: &mut World) -> Result<(), Self::Error>;
    fn handle_render_run_response(
        &mut self,
        stage_response: Option<crate::debug::types::AnySendSyncNamedBox>,
        completion_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageRender,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
        failure_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageRender,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
    );
}
pub trait WorkflowStageRenderO {
    type Output: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, world: &mut World) -> Self::Output;
    fn handle_render_run_response(
        &mut self,
        stage_response: Option<crate::debug::types::AnySendSyncNamedBox>,
        completion_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageRender,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
        failure_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageRender,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
    );
}
pub trait WorkflowStageRenderOE {
    type Output: 'static + Send + Sync;
    type Error: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, world: &mut World) -> Result<Self::Output, Self::Error>;
    fn handle_render_run_response(
        &mut self,
        stage_response: Option<crate::debug::types::AnySendSyncNamedBox>,
        completion_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageRender,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
        failure_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageRender,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
    );
}
pub trait WorkflowStageRenderI {
    type Input: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, input: Self::Input, world: &mut World);
    fn handle_render_run_response(
        &mut self,
        stage_response: Option<crate::debug::types::AnySendSyncNamedBox>,
        completion_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageRender,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
        failure_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageRender,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
    );
}
pub trait WorkflowStageRenderIE {
    type Input: 'static + Send + Sync;
    type Error: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, input: Self::Input, world: &mut World) -> Result<(), Self::Error>;
    fn handle_render_run_response(
        &mut self,
        stage_response: Option<crate::debug::types::AnySendSyncNamedBox>,
        completion_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageRender,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
        failure_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageRender,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
    );
}
pub trait WorkflowStageRenderIO {
    type Input: 'static + Send + Sync;
    type Output: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, input: Self::Input, world: &mut World) -> Self::Output;
    fn handle_render_run_response(
        &mut self,
        stage_response: Option<crate::debug::types::AnySendSyncNamedBox>,
        completion_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageRender,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
        failure_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageRender,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
    );
}
pub trait WorkflowStageRenderIOE {
    type Input: 'static + Send + Sync;
    type Output: 'static + Send + Sync;
    type Error: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(
        &mut self,
        input: Self::Input,
        world: &mut World,
    ) -> Result<Self::Output, Self::Error>;
    fn handle_render_run_response(
        &mut self,
        stage_response: Option<crate::debug::types::AnySendSyncNamedBox>,
        completion_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageRender,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
        failure_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageRender,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
    );
}

// --- Async Workflow Stages ---
pub trait WorkflowStageAsync {
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, world: &mut World);
    fn handle_async_run_response(
        &mut self,
        stage_response: Option<crate::debug::types::AnySendSyncNamedBox>,
        completion_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageAsync,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
        failure_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageAsync,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
    );
}
pub trait WorkflowStageAsyncE {
    type Error: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, world: &mut World) -> Result<(), Self::Error>;
    fn handle_async_run_response(
        &mut self,
        stage_response: Option<crate::debug::types::AnySendSyncNamedBox>,
        completion_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageAsync,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
        failure_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageAsync,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
    );
}
pub trait WorkflowStageAsyncO {
    type Output: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, world: &mut World) -> Self::Output;
    fn handle_async_run_response(
        &mut self,
        stage_response: Option<crate::debug::types::AnySendSyncNamedBox>,
        completion_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageAsync,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
        failure_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageAsync,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
    );
}
pub trait WorkflowStageAsyncOE {
    type Output: 'static + Send + Sync;
    type Error: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, world: &mut World) -> Result<Self::Output, Self::Error>;
    fn handle_async_run_response(
        &mut self,
        stage_response: Option<crate::debug::types::AnySendSyncNamedBox>,
        completion_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageAsync,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
        failure_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageAsync,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
    );
}
pub trait WorkflowStageAsyncI {
    type Input: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, input: Self::Input, world: &mut World);
    fn handle_async_run_response(
        &mut self,
        stage_response: Option<crate::debug::types::AnySendSyncNamedBox>,
        completion_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageAsync,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
        failure_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageAsync,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
    );
}
pub trait WorkflowStageAsyncIE {
    type Input: 'static + Send + Sync;
    type Error: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, input: Self::Input, world: &mut World) -> Result<(), Self::Error>;
    fn handle_async_run_response(
        &mut self,
        stage_response: Option<crate::debug::types::AnySendSyncNamedBox>,
        completion_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageAsync,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
        failure_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageAsync,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
    );
}
pub trait WorkflowStageAsyncIO {
    type Input: 'static + Send + Sync;
    type Output: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, input: Self::Input, world: &mut World) -> Self::Output;
    fn handle_async_run_response(
        &mut self,
        stage_response: Option<crate::debug::types::AnySendSyncNamedBox>,
        completion_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageAsync,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
        failure_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageAsync,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
    );
}
pub trait WorkflowStageAsyncIOE {
    type Input: 'static + Send + Sync;
    type Output: 'static + Send + Sync;
    type Error: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(
        &mut self,
        input: Self::Input,
        world: &mut World,
    ) -> Result<Self::Output, Self::Error>;
    fn handle_async_run_response(
        &mut self,
        stage_response: Option<crate::debug::types::AnySendSyncNamedBox>,
        completion_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageAsync,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
        failure_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageAsync,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
    );
}

// --- Ecs While Workflow Stages ---
pub trait WorkflowStageEcsWhile {
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_ecs_while(&mut self, world: &mut World);
    fn run_ecs_while(&mut self, world: &mut World) -> Outcome<(), ()>;
    fn handle_ecs_while_run_response(
        &mut self,
        stage_response: Option<crate::debug::types::AnySendSyncNamedBox>,
        completion_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageEcsWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
        failure_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageEcsWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
    );
}
pub trait WorkflowStageEcsWhileE {
    type Error: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_ecs_while(&mut self, world: &mut World);
    fn run_ecs_while(&mut self, world: &mut World) -> Result<Outcome<(), ()>, Self::Error>;
    fn handle_ecs_while_run_response(
        &mut self,
        stage_response: Option<crate::debug::types::AnySendSyncNamedBox>,
        completion_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageEcsWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
        failure_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageEcsWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
    );
}
pub trait WorkflowStageEcsWhileO {
    type Output: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_ecs_while(&mut self, world: &mut World);
    fn run_ecs_while(&mut self, world: &mut World) -> Outcome<(), Self::Output>;
    fn handle_ecs_while_run_response(
        &mut self,
        stage_response: Option<crate::debug::types::AnySendSyncNamedBox>,
        completion_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageEcsWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
        failure_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageEcsWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
    );
}
pub trait WorkflowStageEcsWhileOE {
    type Output: 'static + Send + Sync;
    type Error: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_ecs_while(&mut self, world: &mut World);
    fn run_ecs_while(
        &mut self,
        world: &mut World,
    ) -> Result<Outcome<(), Self::Output>, Self::Error>;
    fn handle_ecs_while_run_response(
        &mut self,
        stage_response: Option<crate::debug::types::AnySendSyncNamedBox>,
        completion_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageEcsWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
        failure_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageEcsWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
    );
}
pub trait WorkflowStageEcsWhileS {
    type State: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_ecs_while(&mut self, world: &mut World) -> Self::State;
    fn run_ecs_while(&mut self, state: Self::State, world: &mut World) -> Outcome<Self::State, ()>;
    fn handle_ecs_while_run_response(
        &mut self,
        stage_response: Option<crate::debug::types::AnySendSyncNamedBox>,
        completion_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageEcsWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
        failure_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageEcsWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
    );
}
pub trait WorkflowStageEcsWhileSE {
    type State: 'static + Send + Sync;
    type Error: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_ecs_while(&mut self, world: &mut World) -> Result<Self::State, Self::Error>;
    fn run_ecs_while(
        &mut self,
        state: Self::State,
        world: &mut World,
    ) -> Result<Outcome<Self::State, ()>, Self::Error>;
    fn handle_ecs_while_run_response(
        &mut self,
        stage_response: Option<crate::debug::types::AnySendSyncNamedBox>,
        completion_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageEcsWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
        failure_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageEcsWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
    );
}
pub trait WorkflowStageEcsWhileSO {
    type State: 'static + Send + Sync;
    type Output: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_ecs_while(&mut self, world: &mut World) -> Self::State;
    fn run_ecs_while(
        &mut self,
        state: Self::State,
        world: &mut World,
    ) -> Outcome<Self::State, Self::Output>;
    fn handle_ecs_while_run_response(
        &mut self,
        stage_response: Option<crate::debug::types::AnySendSyncNamedBox>,
        completion_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageEcsWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
        failure_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageEcsWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
    );
}
pub trait WorkflowStageEcsWhileSOE {
    type State: 'static + Send + Sync;
    type Output: 'static + Send + Sync;
    type Error: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_ecs_while(&mut self, world: &mut World) -> Result<Self::State, Self::Error>;
    fn run_ecs_while(
        &mut self,
        state: Self::State,
        world: &mut World,
    ) -> Result<Outcome<Self::State, Self::Output>, Self::Error>;
    fn handle_ecs_while_run_response(
        &mut self,
        stage_response: Option<crate::debug::types::AnySendSyncNamedBox>,
        completion_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageEcsWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
        failure_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageEcsWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
    );
}
pub trait WorkflowStageEcsWhileI {
    type Input: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_ecs_while(&mut self, input: Self::Input, world: &mut World);
    fn run_ecs_while(&mut self, world: &mut World) -> Outcome<(), ()>;
    fn handle_ecs_while_run_response(
        &mut self,
        stage_response: Option<crate::debug::types::AnySendSyncNamedBox>,
        completion_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageEcsWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
        failure_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageEcsWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
    );
}
pub trait WorkflowStageEcsWhileIE {
    type Input: 'static + Send + Sync;
    type Error: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_ecs_while(&mut self, input: Self::Input, world: &mut World);
    fn run_ecs_while(&mut self, world: &mut World) -> Result<Outcome<(), ()>, Self::Error>;
    fn handle_ecs_while_run_response(
        &mut self,
        stage_response: Option<crate::debug::types::AnySendSyncNamedBox>,
        completion_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageEcsWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
        failure_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageEcsWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
    );
}
pub trait WorkflowStageEcsWhileIO {
    type Input: 'static + Send + Sync;
    type Output: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_ecs_while(&mut self, input: Self::Input, world: &mut World);
    fn run_ecs_while(&mut self, world: &mut World) -> Outcome<(), Self::Output>;
    fn handle_ecs_while_run_response(
        &mut self,
        stage_response: Option<crate::debug::types::AnySendSyncNamedBox>,
        completion_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageEcsWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
        failure_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageEcsWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
    );
}
pub trait WorkflowStageEcsWhileIOE {
    type Input: 'static + Send + Sync;
    type Output: 'static + Send + Sync;
    type Error: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_ecs_while(&mut self, input: Self::Input, world: &mut World);
    fn run_ecs_while(
        &mut self,
        world: &mut World,
    ) -> Result<Outcome<(), Self::Output>, Self::Error>;
    fn handle_ecs_while_run_response(
        &mut self,
        stage_response: Option<crate::debug::types::AnySendSyncNamedBox>,
        completion_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageEcsWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
        failure_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageEcsWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
    );
}
pub trait WorkflowStageEcsWhileIS {
    type Input: 'static + Send + Sync;
    type State: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_ecs_while(&mut self, input: Self::Input, world: &mut World) -> Self::State;
    fn run_ecs_while(&mut self, state: Self::State, world: &mut World) -> Outcome<Self::State, ()>;
    fn handle_ecs_while_run_response(
        &mut self,
        stage_response: Option<crate::debug::types::AnySendSyncNamedBox>,
        completion_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageEcsWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
        failure_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageEcsWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
    );
}
pub trait WorkflowStageEcsWhileISE {
    type Input: 'static + Send + Sync;
    type State: 'static + Send + Sync;
    type Error: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_ecs_while(
        &mut self,
        input: Self::Input,
        world: &mut World,
    ) -> Result<Self::State, Self::Error>;
    fn run_ecs_while(
        &mut self,
        state: Self::State,
        world: &mut World,
    ) -> Result<Outcome<Self::State, ()>, Self::Error>;
    fn handle_ecs_while_run_response(
        &mut self,
        stage_response: Option<crate::debug::types::AnySendSyncNamedBox>,
        completion_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageEcsWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
        failure_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageEcsWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
    );
}
pub trait WorkflowStageEcsWhileISO {
    type Input: 'static + Send + Sync;
    type State: 'static + Send + Sync;
    type Output: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_ecs_while(&mut self, input: Self::Input, world: &mut World) -> Self::State;
    fn run_ecs_while(
        &mut self,
        state: Self::State,
        world: &mut World,
    ) -> Outcome<Self::State, Self::Output>;
    fn handle_ecs_while_run_response(
        &mut self,
        stage_response: Option<crate::debug::types::AnySendSyncNamedBox>,
        completion_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageEcsWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
        failure_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageEcsWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
    );
}
pub trait WorkflowStageEcsWhileISOE {
    type Input: 'static + Send + Sync;
    type State: 'static + Send + Sync;
    type Output: 'static + Send + Sync;
    type Error: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_ecs_while(
        &mut self,
        input: Self::Input,
        world: &mut World,
    ) -> Result<Self::State, Self::Error>;
    fn run_ecs_while(
        &mut self,
        state: Self::State,
        world: &mut World,
    ) -> Result<Outcome<Self::State, Self::Output>, Self::Error>;
    fn handle_ecs_while_run_response(
        &mut self,
        stage_response: Option<crate::debug::types::AnySendSyncNamedBox>,
        completion_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageEcsWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
        failure_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageEcsWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
    );
}

// --- Render While Workflow Stages ---
pub trait WorkflowStageRenderWhile {
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_render_while(&mut self, world: &mut World);
    fn run_render_while(&mut self, world: &mut World) -> Outcome<(), ()>;
    fn handle_ecs_while_run_response(
        &mut self,
        stage_response: Option<crate::debug::types::AnySendSyncNamedBox>,
        completion_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageRenderWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
        failure_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageRenderWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
    );
}
pub trait WorkflowStageRenderWhileE {
    type Error: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_render_while(&mut self, world: &mut World);
    fn run_render_while(&mut self, world: &mut World) -> Result<Outcome<(), ()>, Self::Error>;
    fn handle_ecs_while_run_response(
        &mut self,
        stage_response: Option<crate::debug::types::AnySendSyncNamedBox>,
        completion_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageRenderWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
        failure_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageRenderWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
    );
}
pub trait WorkflowStageRenderWhileO {
    type Output: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_render_while(&mut self, world: &mut World);
    fn run_render_while(&mut self, world: &mut World) -> Outcome<(), Self::Output>;
    fn handle_ecs_while_run_response(
        &mut self,
        stage_response: Option<crate::debug::types::AnySendSyncNamedBox>,
        completion_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageRenderWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
        failure_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageRenderWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
    );
}
pub trait WorkflowStageRenderWhileOE {
    type Output: 'static + Send + Sync;
    type Error: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_render_while(&mut self, world: &mut World);
    fn run_render_while(
        &mut self,
        world: &mut World,
    ) -> Result<Outcome<(), Self::Output>, Self::Error>;
    fn handle_ecs_while_run_response(
        &mut self,
        stage_response: Option<crate::debug::types::AnySendSyncNamedBox>,
        completion_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageRenderWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
        failure_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageRenderWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
    );
}
pub trait WorkflowStageRenderWhileS {
    type State: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_render_while(&mut self, world: &mut World) -> Self::State;
    fn run_render_while(
        &mut self,
        state: Self::State,
        world: &mut World,
    ) -> Outcome<Self::State, ()>;
    fn handle_ecs_while_run_response(
        &mut self,
        stage_response: Option<crate::debug::types::AnySendSyncNamedBox>,
        completion_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageRenderWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
        failure_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageRenderWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
    );
}
pub trait WorkflowStageRenderWhileSE {
    type State: 'static + Send + Sync;
    type Error: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_render_while(&mut self, world: &mut World) -> Result<Self::State, Self::Error>;
    fn run_render_while(
        &mut self,
        state: Self::State,
        world: &mut World,
    ) -> Result<Outcome<Self::State, ()>, Self::Error>;
    fn handle_ecs_while_run_response(
        &mut self,
        stage_response: Option<crate::debug::types::AnySendSyncNamedBox>,
        completion_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageRenderWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
        failure_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageRenderWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
    );
}
pub trait WorkflowStageRenderWhileSO {
    type State: 'static + Send + Sync;
    type Output: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_render_while(&mut self, world: &mut World) -> Self::State;
    fn run_render_while(
        &mut self,
        state: Self::State,
        world: &mut World,
    ) -> Outcome<Self::State, Self::Output>;
    fn handle_ecs_while_run_response(
        &mut self,
        stage_response: Option<crate::debug::types::AnySendSyncNamedBox>,
        completion_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageRenderWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
        failure_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageRenderWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
    );
}
pub trait WorkflowStageRenderWhileSOE {
    type State: 'static + Send + Sync;
    type Output: 'static + Send + Sync;
    type Error: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_render_while(&mut self, world: &mut World) -> Result<Self::State, Self::Error>;
    fn run_render_while(
        &mut self,
        state: Self::State,
        world: &mut World,
    ) -> Result<Outcome<Self::State, Self::Output>, Self::Error>;
    fn handle_ecs_while_run_response(
        &mut self,
        stage_response: Option<crate::debug::types::AnySendSyncNamedBox>,
        completion_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageRenderWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
        failure_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageRenderWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
    );
}
pub trait WorkflowStageRenderWhileI {
    type Input: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_render_while(&mut self, input: Self::Input, world: &mut World);
    fn run_render_while(&mut self, world: &mut World) -> Outcome<(), ()>;
    fn handle_ecs_while_run_response(
        &mut self,
        stage_response: Option<crate::debug::types::AnySendSyncNamedBox>,
        completion_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageRenderWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
        failure_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageRenderWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
    );
}
pub trait WorkflowStageRenderWhileIE {
    type Input: 'static + Send + Sync;
    type Error: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_render_while(&mut self, input: Self::Input, world: &mut World);
    fn run_render_while(&mut self, world: &mut World) -> Result<Outcome<(), ()>, Self::Error>;
    fn handle_ecs_while_run_response(
        &mut self,
        stage_response: Option<crate::debug::types::AnySendSyncNamedBox>,
        completion_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageRenderWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
        failure_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageRenderWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
    );
}
pub trait WorkflowStageRenderWhileIO {
    type Input: 'static + Send + Sync;
    type Output: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_render_while(&mut self, input: Self::Input, world: &mut World);
    fn run_render_while(&mut self, world: &mut World) -> Outcome<(), Self::Output>;
    fn handle_ecs_while_run_response(
        &mut self,
        stage_response: Option<crate::debug::types::AnySendSyncNamedBox>,
        completion_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageRenderWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
        failure_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageRenderWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
    );
}
pub trait WorkflowStageRenderWhileIOE {
    type Input: 'static + Send + Sync;
    type Output: 'static + Send + Sync;
    type Error: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_render_while(&mut self, input: Self::Input, world: &mut World);
    fn run_render_while(
        &mut self,
        world: &mut World,
    ) -> Result<Outcome<(), Self::Output>, Self::Error>;
    fn handle_ecs_while_run_response(
        &mut self,
        stage_response: Option<crate::debug::types::AnySendSyncNamedBox>,
        completion_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageRenderWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
        failure_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageRenderWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
    );
}
pub trait WorkflowStageRenderWhileIS {
    type Input: 'static + Send + Sync;
    type State: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_render_while(&mut self, input: Self::Input, world: &mut World) -> Self::State;
    fn run_render_while(
        &mut self,
        state: Self::State,
        world: &mut World,
    ) -> Outcome<Self::State, ()>;
    fn handle_ecs_while_run_response(
        &mut self,
        stage_response: Option<crate::debug::types::AnySendSyncNamedBox>,
        completion_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageRenderWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
        failure_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageRenderWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
    );
}
pub trait WorkflowStageRenderWhileISE {
    type Input: 'static + Send + Sync;
    type State: 'static + Send + Sync;
    type Error: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_render_while(
        &mut self,
        input: Self::Input,
        world: &mut World,
    ) -> Result<Self::State, Self::Error>;
    fn run_render_while(
        &mut self,
        state: Self::State,
        world: &mut World,
    ) -> Result<Outcome<Self::State, ()>, Self::Error>;
    fn handle_ecs_while_run_response(
        &mut self,
        stage_response: Option<crate::debug::types::AnySendSyncNamedBox>,
        completion_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageRenderWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
        failure_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageRenderWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
    );
}
pub trait WorkflowStageRenderWhileISO {
    type Input: 'static + Send + Sync;
    type State: 'static + Send + Sync;
    type Output: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_render_while(&mut self, input: Self::Input, world: &mut World) -> Self::State;
    fn run_render_while(
        &mut self,
        state: Self::State,
        world: &mut World,
    ) -> Outcome<Self::State, Self::Output>;
    fn handle_ecs_while_run_response(
        &mut self,
        stage_response: Option<crate::debug::types::AnySendSyncNamedBox>,
        completion_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageRenderWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
        failure_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageRenderWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
    );
}
pub trait WorkflowStageRenderWhileISOE {
    type Input: 'static + Send + Sync;
    type State: 'static + Send + Sync;
    type Output: 'static + Send + Sync;
    type Error: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_render_while(
        &mut self,
        input: Self::Input,
        world: &mut World,
    ) -> Result<Self::State, Self::Error>;
    fn run_render_while(
        &mut self,
        state: Self::State,
        world: &mut World,
    ) -> Result<Outcome<Self::State, Self::Output>, Self::Error>;
    fn handle_ecs_while_run_response(
        &mut self,
        stage_response: Option<crate::debug::types::AnySendSyncNamedBox>,
        completion_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageRenderWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
        failure_sender: Sender<(
            &str,
            &str,
            usize,
            super::stage::StageRenderWhile,
            Option<crate::debug::types::AnySendSyncNamedBox>,
        )>,
    );
}
