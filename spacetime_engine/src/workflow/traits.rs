use std::any::Any;

use bevy::ecs::world::World;
use crossbeam_channel::Sender;

use super::types::WorkflowStageOutcome;

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

// --- Ecs Workflow Stages ---
pub trait WorkflowStageEcs {
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_ecs(&mut self, world: &mut World);
    fn handle_ecs_response(
        &mut self,
        // TODO: DROPOFF 2: A lot of traits affected
        stage_response: Option<Box<dyn Any + Send + Sync>>, 
        completion_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
        failure_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>
    );
}
pub trait WorkflowStageEcsE {
    type Error: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_ecs(&mut self, world: &mut World) -> Result<(), Self::Error>;
    fn handle_ecs_response(
        &mut self,
        stage_response: Option<Box<dyn Any + Send + Sync>>, 
        completion_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
        failure_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>
    );
}
pub trait WorkflowStageEcsO {
    type Output: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_ecs(&mut self, world: &mut World) -> Self::Output;
    fn handle_ecs_response(
        &mut self,
        stage_response: Option<Box<dyn Any + Send + Sync>>, 
        completion_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
        failure_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>
    );
}
pub trait WorkflowStageEcsOE {
    type Output: 'static + Send + Sync;
    type Error: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_ecs(&mut self, world: &mut World) -> Result<Self::Output, Self::Error>;
    fn handle_ecs_response(
        &mut self,
        stage_response: Option<Box<dyn Any + Send + Sync>>, 
        completion_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
        failure_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>
    );
}
pub trait WorkflowStageEcsI {
    type Input: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_ecs(&mut self, input: Self::Input, world: &mut World);
    fn handle_ecs_response(
        &mut self,
        stage_response: Option<Box<dyn Any + Send + Sync>>, 
        completion_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
        failure_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>
    );
}
pub trait WorkflowStageEcsIE {
    type Input: 'static + Send + Sync;
    type Error: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_ecs(&mut self, input: Self::Input, world: &mut World) -> Result<(), Self::Error>;
    fn handle_ecs_response(
        &mut self,
        stage_response: Option<Box<dyn Any + Send + Sync>>, 
        completion_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
        failure_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>
    );
}
pub trait WorkflowStageEcsIO {
    type Input: 'static + Send + Sync;
    type Output: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_ecs(&mut self, input: Self::Input, world: &mut World) -> Self::Output;
    fn handle_ecs_response(
        &mut self,
        stage_response: Option<Box<dyn Any + Send + Sync>>, 
        completion_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
        failure_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>
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
    fn handle_ecs_response(
        &mut self,
        stage_response: Option<Box<dyn Any + Send + Sync>>, 
        completion_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
        failure_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>
    );
}

// --- Render Workflow Stages ---
pub trait WorkflowStageRender {
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, world: &mut World);
    fn handle_render_response(
        &mut self,
        stage_response: Option<Box<dyn Any + Send + Sync>>, 
        completion_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
        failure_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>
    );
}
pub trait WorkflowStageRenderE {
    type Error: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, world: &mut World) -> Result<(), Self::Error>;
    fn handle_render_response(
        &mut self,
        stage_response: Option<Box<dyn Any + Send + Sync>>, 
        completion_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
        failure_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>
    );
}
pub trait WorkflowStageRenderO {
    type Output: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, world: &mut World) -> Self::Output;
    fn handle_render_response(
        &mut self,
        stage_response: Option<Box<dyn Any + Send + Sync>>, 
        completion_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
        failure_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>
    );
}
pub trait WorkflowStageRenderOE {
    type Output: 'static + Send + Sync;
    type Error: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, world: &mut World) -> Result<Self::Output, Self::Error>;
    fn handle_render_response(
        &mut self,
        stage_response: Option<Box<dyn Any + Send + Sync>>, 
        completion_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
        failure_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>
    );
}
pub trait WorkflowStageRenderI {
    type Input: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, input: Self::Input, world: &mut World);
    fn handle_render_response(
        &mut self,
        stage_response: Option<Box<dyn Any + Send + Sync>>, 
        completion_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
        failure_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>
    );
}
pub trait WorkflowStageRenderIE {
    type Input: 'static + Send + Sync;
    type Error: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, input: Self::Input, world: &mut World) -> Result<(), Self::Error>;
    fn handle_render_response(
        &mut self,
        stage_response: Option<Box<dyn Any + Send + Sync>>, 
        completion_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
        failure_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>
    );
}
pub trait WorkflowStageRenderIO {
    type Input: 'static + Send + Sync;
    type Output: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, input: Self::Input, world: &mut World) -> Self::Output;
    fn handle_render_response(
        &mut self,
        stage_response: Option<Box<dyn Any + Send + Sync>>, 
        completion_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
        failure_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>
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
    fn handle_render_response(
        &mut self,
        stage_response: Option<Box<dyn Any + Send + Sync>>, 
        completion_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
        failure_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>
    );
}

// --- Async Workflow Stages ---
pub trait WorkflowStageAsync {
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, world: &mut World);
    fn handle_async_response(
        &mut self,
        stage_response: Option<Box<dyn Any + Send + Sync>>, 
        completion_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
        failure_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>
    );
}
pub trait WorkflowStageAsyncE {
    type Error: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, world: &mut World) -> Result<(), Self::Error>;
    fn handle_async_response(
        &mut self,
        stage_response: Option<Box<dyn Any + Send + Sync>>, 
        completion_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
        failure_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>
    );
}
pub trait WorkflowStageAsyncO {
    type Output: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, world: &mut World) -> Self::Output;
    fn handle_async_response(
        &mut self,
        stage_response: Option<Box<dyn Any + Send + Sync>>, 
        completion_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
        failure_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>
    );
}
pub trait WorkflowStageAsyncOE {
    type Output: 'static + Send + Sync;
    type Error: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, world: &mut World) -> Result<Self::Output, Self::Error>;
    fn handle_async_response(
        &mut self,
        stage_response: Option<Box<dyn Any + Send + Sync>>, 
        completion_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
        failure_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>
    );
}
pub trait WorkflowStageAsyncI {
    type Input: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, input: Self::Input, world: &mut World);
    fn handle_async_response(
        &mut self,
        stage_response: Option<Box<dyn Any + Send + Sync>>, 
        completion_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
        failure_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>
    );
}
pub trait WorkflowStageAsyncIE {
    type Input: 'static + Send + Sync;
    type Error: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, input: Self::Input, world: &mut World) -> Result<(), Self::Error>;
    fn handle_async_response(
        &mut self,
        stage_response: Option<Box<dyn Any + Send + Sync>>, 
        completion_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
        failure_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>
    );
}
pub trait WorkflowStageAsyncIO {
    type Input: 'static + Send + Sync;
    type Output: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, input: Self::Input, world: &mut World) -> Self::Output;
    fn handle_async_response(
        &mut self,
        stage_response: Option<Box<dyn Any + Send + Sync>>, 
        completion_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
        failure_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>
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
    fn handle_async_response(
        &mut self,
        stage_response: Option<Box<dyn Any + Send + Sync>>, 
        completion_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
        failure_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>
    );
}

// --- Ecs While Workflow Stages ---
pub trait WorkflowStageEcsWhile {
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_ecs_while(&mut self, world: &mut World);
    fn run_ecs_while(&mut self, world: &mut World) -> WorkflowStageOutcome<(), ()>;
    fn handle_ecs_while_response(
        &mut self,
        stage_response: Option<Box<dyn Any + Send + Sync>>, 
        completion_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
        failure_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>
    );
}
pub trait WorkflowStageEcsWhileE {
    type Error: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_ecs_while(&mut self, world: &mut World);
    fn run_ecs_while(
        &mut self,
        world: &mut World,
    ) -> Result<WorkflowStageOutcome<(), ()>, Self::Error>;
    fn handle_ecs_while_response(
        &mut self,
        stage_response: Option<Box<dyn Any + Send + Sync>>, 
        completion_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
        failure_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>
    );
}
pub trait WorkflowStageEcsWhileO {
    type Output: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_ecs_while(&mut self, world: &mut World);
    fn run_ecs_while(&mut self, world: &mut World) -> WorkflowStageOutcome<(), Self::Output>;
    fn handle_ecs_while_response(
        &mut self,
        stage_response: Option<Box<dyn Any + Send + Sync>>, 
        completion_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
        failure_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>
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
    ) -> Result<WorkflowStageOutcome<(), Self::Output>, Self::Error>;
    fn handle_ecs_while_response(
        &mut self,
        stage_response: Option<Box<dyn Any + Send + Sync>>, 
        completion_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
        failure_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>
    );
}
pub trait WorkflowStageEcsWhileS {
    type State: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_ecs_while(&mut self, world: &mut World) -> Self::State;
    fn run_ecs_while(
        &mut self,
        state: Self::State,
        world: &mut World,
    ) -> WorkflowStageOutcome<Self::State, ()>;
    fn handle_ecs_while_response(
        &mut self,
        stage_response: Option<Box<dyn Any + Send + Sync>>, 
        completion_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
        failure_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>
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
    ) -> Result<WorkflowStageOutcome<Self::State, ()>, Self::Error>;
    fn handle_ecs_while_response(
        &mut self,
        stage_response: Option<Box<dyn Any + Send + Sync>>, 
        completion_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
        failure_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>
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
    ) -> WorkflowStageOutcome<Self::State, Self::Output>;
    fn handle_ecs_while_response(
        &mut self,
        stage_response: Option<Box<dyn Any + Send + Sync>>, 
        completion_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
        failure_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>
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
    ) -> Result<WorkflowStageOutcome<Self::State, Self::Output>, Self::Error>;
    fn handle_ecs_while_response(
        &mut self,
        stage_response: Option<Box<dyn Any + Send + Sync>>, 
        completion_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
        failure_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>
    );
}
pub trait WorkflowStageEcsWhileI {
    type Input: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_ecs_while(&mut self, input: Self::Input, world: &mut World);
    fn run_ecs_while(&mut self, world: &mut World) -> WorkflowStageOutcome<(), ()>;
    fn handle_ecs_while_response(
        &mut self,
        stage_response: Option<Box<dyn Any + Send + Sync>>, 
        completion_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
        failure_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>
    );
}
pub trait WorkflowStageEcsWhileIE {
    type Input: 'static + Send + Sync;
    type Error: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_ecs_while(&mut self, input: Self::Input, world: &mut World);
    fn run_ecs_while(
        &mut self,
        world: &mut World,
    ) -> Result<WorkflowStageOutcome<(), ()>, Self::Error>;
    fn handle_ecs_while_response(
        &mut self,
        stage_response: Option<Box<dyn Any + Send + Sync>>, 
        completion_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
        failure_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>
    );
}
pub trait WorkflowStageEcsWhileIO {
    type Input: 'static + Send + Sync;
    type Output: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_ecs_while(&mut self, input: Self::Input, world: &mut World);
    fn run_ecs_while(&mut self, world: &mut World) -> WorkflowStageOutcome<(), Self::Output>;
    fn handle_ecs_while_response(
        &mut self,
        stage_response: Option<Box<dyn Any + Send + Sync>>, 
        completion_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
        failure_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>
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
    ) -> Result<WorkflowStageOutcome<(), Self::Output>, Self::Error>;
    fn handle_ecs_while_response(
        &mut self,
        stage_response: Option<Box<dyn Any + Send + Sync>>, 
        completion_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
        failure_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>
    );
}
pub trait WorkflowStageEcsWhileIS {
    type Input: 'static + Send + Sync;
    type State: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_ecs_while(&mut self, input: Self::Input, world: &mut World) -> Self::State;
    fn run_ecs_while(
        &mut self,
        state: Self::State,
        world: &mut World,
    ) -> WorkflowStageOutcome<Self::State, ()>;
    fn handle_ecs_while_response(
        &mut self,
        stage_response: Option<Box<dyn Any + Send + Sync>>, 
        completion_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
        failure_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>
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
    ) -> Result<WorkflowStageOutcome<Self::State, ()>, Self::Error>;
    fn handle_ecs_while_response(
        &mut self,
        stage_response: Option<Box<dyn Any + Send + Sync>>, 
        completion_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
        failure_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>
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
    ) -> WorkflowStageOutcome<Self::State, Self::Output>;
    fn handle_ecs_while_response(
        &mut self,
        stage_response: Option<Box<dyn Any + Send + Sync>>, 
        completion_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
        failure_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>
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
    ) -> Result<WorkflowStageOutcome<Self::State, Self::Output>, Self::Error>;
    fn handle_ecs_while_response(
        &mut self,
        stage_response: Option<Box<dyn Any + Send + Sync>>, 
        completion_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
        failure_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>
    );
}

// --- Render While Workflow Stages ---
pub trait WorkflowStageRenderWhile {
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_render_while(&mut self, world: &mut World);
    fn run_render_while(&mut self, world: &mut World) -> WorkflowStageOutcome<(), ()>;
    fn handle_ecs_while_response(
        &mut self,
        stage_response: Option<Box<dyn Any + Send + Sync>>, 
        completion_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
        failure_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>
    );
}
pub trait WorkflowStageRenderWhileE {
    type Error: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_render_while(&mut self, world: &mut World);
    fn run_render_while(
        &mut self,
        world: &mut World,
    ) -> Result<WorkflowStageOutcome<(), ()>, Self::Error>;
    fn handle_ecs_while_response(
        &mut self,
        stage_response: Option<Box<dyn Any + Send + Sync>>, 
        completion_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
        failure_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>
    );
}
pub trait WorkflowStageRenderWhileO {
    type Output: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_render_while(&mut self, world: &mut World);
    fn run_render_while(&mut self, world: &mut World) -> WorkflowStageOutcome<(), Self::Output>;
    fn handle_ecs_while_response(
        &mut self,
        stage_response: Option<Box<dyn Any + Send + Sync>>, 
        completion_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
        failure_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>
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
    ) -> Result<WorkflowStageOutcome<(), Self::Output>, Self::Error>;
    fn handle_ecs_while_response(
        &mut self,
        stage_response: Option<Box<dyn Any + Send + Sync>>, 
        completion_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
        failure_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>
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
    ) -> WorkflowStageOutcome<Self::State, ()>;
    fn handle_ecs_while_response(
        &mut self,
        stage_response: Option<Box<dyn Any + Send + Sync>>, 
        completion_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
        failure_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>
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
    ) -> Result<WorkflowStageOutcome<Self::State, ()>, Self::Error>;
    fn handle_ecs_while_response(
        &mut self,
        stage_response: Option<Box<dyn Any + Send + Sync>>, 
        completion_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
        failure_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>
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
    ) -> WorkflowStageOutcome<Self::State, Self::Output>;
    fn handle_ecs_while_response(
        &mut self,
        stage_response: Option<Box<dyn Any + Send + Sync>>, 
        completion_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
        failure_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>
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
    ) -> Result<WorkflowStageOutcome<Self::State, Self::Output>, Self::Error>;
    fn handle_ecs_while_response(
        &mut self,
        stage_response: Option<Box<dyn Any + Send + Sync>>, 
        completion_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
        failure_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>
    );
}
pub trait WorkflowStageRenderWhileI {
    type Input: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_render_while(&mut self, input: Self::Input, world: &mut World);
    fn run_render_while(&mut self, world: &mut World) -> WorkflowStageOutcome<(), ()>;
    fn handle_ecs_while_response(
        &mut self,
        stage_response: Option<Box<dyn Any + Send + Sync>>, 
        completion_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
        failure_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>
    );
}
pub trait WorkflowStageRenderWhileIE {
    type Input: 'static + Send + Sync;
    type Error: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_render_while(&mut self, input: Self::Input, world: &mut World);
    fn run_render_while(
        &mut self,
        world: &mut World,
    ) -> Result<WorkflowStageOutcome<(), ()>, Self::Error>;
    fn handle_ecs_while_response(
        &mut self,
        stage_response: Option<Box<dyn Any + Send + Sync>>, 
        completion_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
        failure_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>
    );
}
pub trait WorkflowStageRenderWhileIO {
    type Input: 'static + Send + Sync;
    type Output: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_render_while(&mut self, input: Self::Input, world: &mut World);
    fn run_render_while(&mut self, world: &mut World) -> WorkflowStageOutcome<(), Self::Output>;
    fn handle_ecs_while_response(
        &mut self,
        stage_response: Option<Box<dyn Any + Send + Sync>>, 
        completion_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
        failure_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>
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
    ) -> Result<WorkflowStageOutcome<(), Self::Output>, Self::Error>;
    fn handle_ecs_while_response(
        &mut self,
        stage_response: Option<Box<dyn Any + Send + Sync>>, 
        completion_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
        failure_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>
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
    ) -> WorkflowStageOutcome<Self::State, ()>;
    fn handle_ecs_while_response(
        &mut self,
        stage_response: Option<Box<dyn Any + Send + Sync>>, 
        completion_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
        failure_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>
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
    ) -> Result<WorkflowStageOutcome<Self::State, ()>, Self::Error>;
    fn handle_ecs_while_response(
        &mut self,
        stage_response: Option<Box<dyn Any + Send + Sync>>, 
        completion_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
        failure_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>
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
    ) -> WorkflowStageOutcome<Self::State, Self::Output>;
    fn handle_ecs_while_response(
        &mut self,
        stage_response: Option<Box<dyn Any + Send + Sync>>, 
        completion_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
        failure_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>
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
    ) -> Result<WorkflowStageOutcome<Self::State, Self::Output>, Self::Error>;
    fn handle_ecs_while_response(
        &mut self,
        stage_response: Option<Box<dyn Any + Send + Sync>>, 
        completion_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>, 
        failure_sender: Sender<(&str, &str, usize, super::stage::WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>
    );
}
