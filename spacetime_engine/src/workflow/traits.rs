use bevy::ecs::world::World;

use super::types::Outcome;

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
}
pub trait WorkflowStageEcsE {
    type Error: 'static + Send + Sync;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_ecs(&mut self, world: &mut World) -> Result<(), Self::Error>;
}
pub trait WorkflowStageEcsO {
    type Output: 'static + Send + Sync;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_ecs(&mut self, world: &mut World) -> Self::Output;
}
pub trait WorkflowStageEcsOE {
    type Output: 'static + Send + Sync;
    type Error: 'static + Send + Sync;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_ecs(&mut self, world: &mut World) -> Result<Self::Output, Self::Error>;
}
pub trait WorkflowStageEcsI {
    type Input: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_ecs(&mut self, input: Self::Input, world: &mut World);
}
pub trait WorkflowStageEcsIE {
    type Input: 'static + Send + Sync;
    type Error: 'static + Send + Sync;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_ecs(&mut self, input: Self::Input, world: &mut World) -> Result<(), Self::Error>;
}
pub trait WorkflowStageEcsIO {
    type Input: 'static + Send + Sync;
    type Output: 'static + Send + Sync;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_ecs(&mut self, input: Self::Input, world: &mut World) -> Self::Output;
}
pub trait WorkflowStageEcsIOE {
    type Input: 'static + Send + Sync;
    type Output: 'static + Send + Sync;
    type Error: 'static + Send + Sync;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_ecs(&mut self, input: Self::Input, world: &mut World) -> Result<Self::Output, Self::Error>;
}

// --- Render Workflow Stages ---
pub trait WorkflowStageRender {
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, world: &mut World);
}
pub trait WorkflowStageRenderE {
    type Error: 'static + Send + Sync;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, world: &mut World) -> Result<(), Self::Error>;
}
pub trait WorkflowStageRenderO {
    type Output: 'static + Send + Sync;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, world: &mut World) -> Self::Output;
}
pub trait WorkflowStageRenderOE {
    type Output: 'static + Send + Sync;
    type Error: 'static + Send + Sync;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, world: &mut World) -> Result<Self::Output, Self::Error>;
}
pub trait WorkflowStageRenderI {
    type Input: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, input: Self::Input, world: &mut World);
}
pub trait WorkflowStageRenderIE {
    type Input: 'static + Send + Sync;
    type Error: 'static + Send + Sync;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, input: Self::Input, world: &mut World) -> Result<(), Self::Error>;
}
pub trait WorkflowStageRenderIO {
    type Input: 'static + Send + Sync;
    type Output: 'static + Send + Sync;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, input: Self::Input, world: &mut World) -> Self::Output;
}
pub trait WorkflowStageRenderIOE {
    type Input: 'static + Send + Sync;
    type Output: 'static + Send + Sync;
    type Error: 'static + Send + Sync;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, input: Self::Input, world: &mut World) -> Result<Self::Output, Self::Error>;
}



// --- Async Workflow Stages ---
pub trait WorkflowStageAsync {
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, world: &mut World);
}
pub trait WorkflowStageAsyncE {
    type Error: 'static + Send + Sync;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, world: &mut World) -> Result<(), Self::Error>;
}
pub trait WorkflowStageAsyncO {
    type Output: 'static + Send + Sync;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, world: &mut World) -> Self::Output;
}
pub trait WorkflowStageAsyncOE {
    type Output: 'static + Send + Sync;
    type Error: 'static + Send + Sync;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, world: &mut World) -> Result<Self::Output, Self::Error>;
}
pub trait WorkflowStageAsyncI {
    type Input: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, input: Self::Input, world: &mut World);
}
pub trait WorkflowStageAsyncIE {
    type Input: 'static + Send + Sync;
    type Error: 'static + Send + Sync;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, input: Self::Input, world: &mut World) -> Result<(), Self::Error>;
}
pub trait WorkflowStageAsyncIO {
    type Input: 'static + Send + Sync;
    type Output: 'static + Send + Sync;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, input: Self::Input, world: &mut World) -> Self::Output;
}
pub trait WorkflowStageAsyncIOE {
    type Input: 'static + Send + Sync;
    type Output: 'static + Send + Sync;
    type Error: 'static + Send + Sync;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, input: Self::Input, world: &mut World) -> Result<Self::Output, Self::Error>;
}



// --- Ecs While Workflow Stages ---
pub trait WorkflowStageEcsWhile {
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_ecs_while(&mut self, world: &mut World);
    fn run_ecs_while(&mut self, world: &mut World) -> Outcome<(), ()>;
}
pub trait WorkflowStageEcsWhileE {
    type Error: 'static + Send + Sync;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_ecs_while(&mut self, world: &mut World);
    fn run_ecs_while(&mut self, world: &mut World) -> Result<Outcome<(), ()>, Self::Error>;
}
pub trait WorkflowStageEcsWhileO {
    type Output: 'static + Send + Sync;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_ecs_while(&mut self, world: &mut World);
    fn run_ecs_while(&mut self, world: &mut World) -> Outcome<(), Self::Output>;
}
pub trait WorkflowStageEcsWhileOE {
    type Output: 'static + Send + Sync;
    type Error: 'static + Send + Sync;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_ecs_while(&mut self, world: &mut World);
    fn run_ecs_while(&mut self, world: &mut World) -> Result<Outcome<(), Self::Output>, Self::Error>;
}
pub trait WorkflowStageEcsWhileS {
    type State: 'static + Send + Sync;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_ecs_while(&mut self, world: &mut World) -> Self::State;
    fn run_ecs_while(&mut self, state: Self::State, world: &mut World) -> Outcome<Self::State, ()>;
}
pub trait WorkflowStageEcsWhileSE {
    type State: 'static + Send + Sync;
    type Error: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_ecs_while(&mut self, world: &mut World) -> Result<Self::State, Self::Error>;
    fn run_ecs_while(&mut self, state: Self::State, world: &mut World) -> Result<Outcome<Self::State, ()>, Self::Error>;
}
pub trait WorkflowStageEcsWhileSO {
    type State: 'static + Send + Sync;
    type Output: 'static + Send + Sync;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_ecs_while(&mut self, world: &mut World) -> Self::State;
    fn run_ecs_while(&mut self, state: Self::State, world: &mut World) -> Outcome<Self::State, Self::Output>;
}
pub trait WorkflowStageEcsWhileSOE {
    type State: 'static + Send + Sync;
    type Output: 'static + Send + Sync;
    type Error: 'static + Send + Sync;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_ecs_while(&mut self, world: &mut World) -> Result<Self::State, Self::Error>;
    fn run_ecs_while(&mut self, state: Self::State, world: &mut World) -> Result<Outcome<Self::State, Self::Output>, Self::Error>;
}
pub trait WorkflowStageEcsWhileI {
    type Input: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_ecs_while(&mut self, input: Self::Input, world: &mut World);
    fn run_ecs_while(&mut self, world: &mut World) -> Outcome<(), ()>;
}
pub trait WorkflowStageEcsWhileIE {
    type Input: 'static + Send + Sync;
    type Error: 'static + Send + Sync;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_ecs_while(&mut self, input: Self::Input, world: &mut World);
    fn run_ecs_while(&mut self, world: &mut World) -> Result<Outcome<(), ()>, Self::Error>;
}
pub trait WorkflowStageEcsWhileIO {
    type Input: 'static + Send + Sync;
    type Output: 'static + Send + Sync;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_ecs_while(&mut self, input: Self::Input, world: &mut World);
    fn run_ecs_while(&mut self, world: &mut World) -> Outcome<(), Self::Output>;
}
pub trait WorkflowStageEcsWhileIOE {
    type Input: 'static + Send + Sync;
    type Output: 'static + Send + Sync;
    type Error: 'static + Send + Sync;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_ecs_while(&mut self, input: Self::Input, world: &mut World);
    fn run_ecs_while(&mut self, world: &mut World) -> Result<Outcome<(), Self::Output>, Self::Error>;
}
pub trait WorkflowStageEcsWhileIS {
    type Input: 'static + Send + Sync;
    type State: 'static + Send + Sync;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_ecs_while(&mut self, input: Self::Input, world: &mut World) -> Self::State;
    fn run_ecs_while(&mut self, state: Self::State, world: &mut World) -> Outcome<Self::State, ()>;
}
pub trait WorkflowStageEcsWhileISE {
    type Input: 'static + Send + Sync;
    type State: 'static + Send + Sync;
    type Error: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_ecs_while(&mut self, input: Self::Input, world: &mut World) -> Result<Self::State, Self::Error>;
    fn run_ecs_while(&mut self, state: Self::State, world: &mut World) -> Result<Outcome<Self::State, ()>, Self::Error>;
}
pub trait WorkflowStageEcsWhileISO {
    type Input: 'static + Send + Sync;
    type State: 'static + Send + Sync;
    type Output: 'static + Send + Sync;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_ecs_while(&mut self, input: Self::Input, world: &mut World) -> Self::State;
    fn run_ecs_while(&mut self, state: Self::State, world: &mut World) -> Outcome<Self::State, Self::Output>;
}
pub trait WorkflowStageEcsWhileISOE {
    type Input: 'static + Send + Sync;
    type State: 'static + Send + Sync;
    type Output: 'static + Send + Sync;
    type Error: 'static + Send + Sync;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_ecs_while(&mut self, input: Self::Input, world: &mut World) -> Result<Self::State, Self::Error>;
    fn run_ecs_while(&mut self, state: Self::State, world: &mut World) -> Result<Outcome<Self::State, Self::Output>, Self::Error>;
}



// --- Render While Workflow Stages ---
pub trait WorkflowStageRenderWhile {
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_render_while(&mut self, world: &mut World);
    fn run_render_while(&mut self, world: &mut World) -> Outcome<(), ()>;
}
pub trait WorkflowStageRenderWhileE {
    type Error: 'static + Send + Sync;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_render_while(&mut self, world: &mut World);
    fn run_render_while(&mut self, world: &mut World) -> Result<Outcome<(), ()>, Self::Error>;
}
pub trait WorkflowStageRenderWhileO {
    type Output: 'static + Send + Sync;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_render_while(&mut self, world: &mut World);
    fn run_render_while(&mut self, world: &mut World) -> Outcome<(), Self::Output>;
}
pub trait WorkflowStageRenderWhileOE {
    type Output: 'static + Send + Sync;
    type Error: 'static + Send + Sync;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_render_while(&mut self, world: &mut World);
    fn run_render_while(&mut self, world: &mut World) -> Result<Outcome<(), Self::Output>, Self::Error>;
}
pub trait WorkflowStageRenderWhileS {
    type State: 'static + Send + Sync;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_render_while(&mut self, world: &mut World) -> Self::State;
    fn run_render_while(&mut self, state: Self::State, world: &mut World) -> Outcome<Self::State, ()>;
}
pub trait WorkflowStageRenderWhileSE {
    type State: 'static + Send + Sync;
    type Error: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_render_while(&mut self, world: &mut World) -> Result<Self::State, Self::Error>;
    fn run_render_while(&mut self, state: Self::State, world: &mut World) -> Result<Outcome<Self::State, ()>, Self::Error>;
}
pub trait WorkflowStageRenderWhileSO {
    type State: 'static + Send + Sync;
    type Output: 'static + Send + Sync;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_render_while(&mut self, world: &mut World) -> Self::State;
    fn run_render_while(&mut self, state: Self::State, world: &mut World) -> Outcome<Self::State, Self::Output>;
}
pub trait WorkflowStageRenderWhileSOE {
    type State: 'static + Send + Sync;
    type Output: 'static + Send + Sync;
    type Error: 'static + Send + Sync;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_render_while(&mut self, world: &mut World) -> Result<Self::State, Self::Error>;
    fn run_render_while(&mut self, state: Self::State, world: &mut World) -> Result<Outcome<Self::State, Self::Output>, Self::Error>;
}
pub trait WorkflowStageRenderWhileI {
    type Input: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_render_while(&mut self, input: Self::Input, world: &mut World);
    fn run_render_while(&mut self, world: &mut World) -> Outcome<(), ()>;
}
pub trait WorkflowStageRenderWhileIE {
    type Input: 'static + Send + Sync;
    type Error: 'static + Send + Sync;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_render_while(&mut self, input: Self::Input, world: &mut World);
    fn run_render_while(&mut self, world: &mut World) -> Result<Outcome<(), ()>, Self::Error>;
}
pub trait WorkflowStageRenderWhileIO {
    type Input: 'static + Send + Sync;
    type Output: 'static + Send + Sync;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_render_while(&mut self, input: Self::Input, world: &mut World);
    fn run_render_while(&mut self, world: &mut World) -> Outcome<(), Self::Output>;
}
pub trait WorkflowStageRenderWhileIOE {
    type Input: 'static + Send + Sync;
    type Output: 'static + Send + Sync;
    type Error: 'static + Send + Sync;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_render_while(&mut self, input: Self::Input, world: &mut World);
    fn run_render_while(&mut self, world: &mut World) -> Result<Outcome<(), Self::Output>, Self::Error>;
}
pub trait WorkflowStageRenderWhileIS {
    type Input: 'static + Send + Sync;
    type State: 'static + Send + Sync;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_render_while(&mut self, input: Self::Input, world: &mut World) -> Self::State;
    fn run_render_while(&mut self, state: Self::State, world: &mut World) -> Outcome<Self::State, ()>;
}
pub trait WorkflowStageRenderWhileISE {
    type Input: 'static + Send + Sync;
    type State: 'static + Send + Sync;
    type Error: 'static + Send + Sync;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_render_while(&mut self, input: Self::Input, world: &mut World) -> Result<Self::State, Self::Error>;
    fn run_render_while(&mut self, state: Self::State, world: &mut World) -> Result<Outcome<Self::State, ()>, Self::Error>;
}
pub trait WorkflowStageRenderWhileISO {
    type Input: 'static + Send + Sync;
    type State: 'static + Send + Sync;
    type Output: 'static + Send + Sync;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_render_while(&mut self, input: Self::Input, world: &mut World) -> Self::State;
    fn run_render_while(&mut self, state: Self::State, world: &mut World) -> Outcome<Self::State, Self::Output>;
}
pub trait WorkflowStageRenderWhileISOE {
    type Input: 'static + Send + Sync;
    type State: 'static + Send + Sync;
    type Output: 'static + Send + Sync;
    type Error: 'static + Send + Sync;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_render_while(&mut self, input: Self::Input, world: &mut World) -> Result<Self::State, Self::Error>;
    fn run_render_while(&mut self, state: Self::State, world: &mut World) -> Result<Outcome<Self::State, Self::Output>, Self::Error>;
}
