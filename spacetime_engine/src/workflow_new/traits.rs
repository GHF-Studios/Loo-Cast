// --- Ecs Workflow Stages ---
pub trait WorkflowStageEcs___ {
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_ecs(&mut self, world: &mut World);
}
pub trait WorkflowStageEcs__E {
    type Error;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_ecs(&mut self, world: &mut World) -> Result<(), Self::Error>;
}
pub trait WorkflowStageEcs_O_ {
    type Output;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_ecs(&mut self, world: &mut World) -> Self::Output;
}
pub trait WorkflowStageEcs_OE {
    type Output;
    type Error;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_ecs(&mut self, world: &mut World) -> Result<Self::Output, Self::Error>;
}
pub trait WorkflowStageEcsI__ {
    type Input;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_ecs(&mut self, input: Self::Input, world: &mut World);
}
pub trait WorkflowStageEcsI_E {
    type Input;
    type Error;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_ecs(&mut self, input: Self::Input, world: &mut World) -> Result<(), Self::Error>;
}
pub trait WorkflowStageEcsIO_ {
    type Input;
    type Output;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_ecs(&mut self, input: Self::Input, world: &mut World) -> Self::Output;
}
pub trait WorkflowStageEcsIOE {
    type Input;
    type Output;
    type Error;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_ecs(&mut self, input: Self::Input, world: &mut World) -> Result<Self::Output, Self::Error>;
}



// --- Render Workflow Stages ---
pub trait WorkflowStageRender___ {
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, world: &mut World);
}
pub trait WorkflowStageRender__E {
    type Error;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, world: &mut World) -> Result<(), Self::Error>;
}
pub trait WorkflowStageRender_O_ {
    type Output;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, world: &mut World) -> Self::Output;
}
pub trait WorkflowStageRender_OE {
    type Output;
    type Error;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, world: &mut World) -> Result<Self::Output, Self::Error>;
}
pub trait WorkflowStageRenderI__ {
    type Input;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, input: Self::Input, world: &mut World);
}
pub trait WorkflowStageRenderI_E {
    type Input;
    type Error;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, input: Self::Input, world: &mut World) -> Result<(), Self::Error>;
}
pub trait WorkflowStageRenderIO_ {
    type Input;
    type Output;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, input: Self::Input, world: &mut World) -> Self::Output;
}
pub trait WorkflowStageRenderIOE {
    type Input;
    type Output;
    type Error;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, input: Self::Input, world: &mut World) -> Result<Self::Output, Self::Error>;
}



// --- Async Workflow Stages ---
pub trait WorkflowStageRender___ {
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, world: &mut World);
}
pub trait WorkflowStageRender__E {
    type Error;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, world: &mut World) -> Result<(), Self::Error>;
}
pub trait WorkflowStageRender_O_ {
    type Output;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, world: &mut World) -> Self::Output;
}
pub trait WorkflowStageRender_OE {
    type Output;
    type Error;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, world: &mut World) -> Result<Self::Output, Self::Error>;
}
pub trait WorkflowStageRenderI__ {
    type Input;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, input: Self::Input, world: &mut World);
}
pub trait WorkflowStageRenderI_E {
    type Input;
    type Error;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, input: Self::Input, world: &mut World) -> Result<(), Self::Error>;
}
pub trait WorkflowStageRenderIO_ {
    type Input;
    type Output;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, input: Self::Input, world: &mut World) -> Self::Output;
}
pub trait WorkflowStageRenderIOE {
    type Input;
    type Output;
    type Error;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn run_render(&mut self, input: Self::Input, world: &mut World) -> Result<Self::Output, Self::Error>;
}



// --- Ecs While Workflow Stages ---
pub trait WorkflowStageEcsWhile____ {
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_ecs_while(&mut self, world: &mut World);
    fn run_ecs_while(&mut self, world: &mut World) -> Outcome<(), ()>;
}
pub trait WorkflowStageEcsWhile___E {
    type Error;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_ecs_while(&mut self, world: &mut World);
    fn run_ecs_while(&mut self, world: &mut World) -> Result<Outcome<(), ()>, Self::Error>;
}
pub trait WorkflowStageEcsWhile__O_ {
    type Output;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_ecs_while(&mut self, world: &mut World);
    fn run_ecs_while(&mut self, world: &mut World) -> Outcome<(), Self::Output>;
}
pub trait WorkflowStageEcsWhile__OE {
    type Output;
    type Error;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_ecs_while(&mut self, world: &mut World);
    fn run_ecs_while(&mut self, world: &mut World) -> Result<Outcome<(), Self::Output>, Self::Error>;
}
pub trait WorkflowStageEcsWhile_S__ {
    type State;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_ecs_while(&mut self, world: &mut World) -> Self::State;
    fn run_ecs_while(&mut self, state: Self::State, world: &mut World) -> Outcome<Self::State, ()>;
}
pub trait WorkflowStageEcsWhile_S_E {
    type State;
    type Error;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_ecs_while(&mut self, world: &mut World) -> Result<Self::State, Self::Error>;
    fn run_ecs_while(&mut self, state: Self::State, world: &mut World) -> Result<Outcome<Self::State, ()>, Self::Error>;
}
pub trait WorkflowStageEcsWhile_SO_ {
    type State;
    type Output;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_ecs_while(&mut self, world: &mut World) -> Self::State;
    fn run_ecs_while(&mut self, state: Self::State, world: &mut World) -> Outcome<Self::State, Self::Output>;
}
pub trait WorkflowStageEcsWhile_SOE {
    type State;
    type Output;
    type Error;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_ecs_while(&mut self, world: &mut World) -> Result<Self::State, Self::Error>;
    fn run_ecs_while(&mut self, state: Self::State, world: &mut World) -> Result<Outcome<Self::State, Self::Output>, Self::Error>;
}
pub trait WorkflowStageEcsWhileI___ {
    type Input;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_ecs_while(&mut self, input: Self::Input, world: &mut World);
    fn run_ecs_while(&mut self, world: &mut World) -> Outcome<(), ()>;
}
pub trait WorkflowStageEcsWhileI__E {
    type Input;
    type Error;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_ecs_while(&mut self, input: Self::Input, world: &mut World);
    fn run_ecs_while(&mut self, world: &mut World) -> Result<Outcome<(), ()>, Self::Error>;
}
pub trait WorkflowStageEcsWhileI_O_ {
    type Input;
    type Output;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_ecs_while(&mut self, input: Self::Input, world: &mut World);
    fn run_ecs_while(&mut self, world: &mut World) -> Outcome<(), Self::Output>;
}
pub trait WorkflowStageEcsWhileI_OE {
    type Input;
    type Output;
    type Error;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_ecs_while(&mut self, input: Self::Input, world: &mut World);
    fn run_ecs_while(&mut self, world: &mut World) -> Result<Outcome<(), Self::Output>, Self::Error>;
}
pub trait WorkflowStageEcsWhileIS__ {
    type Input;
    type State;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_ecs_while(&mut self, input: Self::Input, world: &mut World) -> Self::State;
    fn run_ecs_while(&mut self, state: Self::State, world: &mut World) -> Outcome<Self::State, ()>;
}
pub trait WorkflowStageEcsWhileIS_E {
    type Input;
    type State;
    type Error;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_ecs_while(&mut self, input: Self::Input, world: &mut World) -> Result<Self::State, Self::Error>;
    fn run_ecs_while(&mut self, state: Self::State, world: &mut World) -> Result<Outcome<Self::State, ()>, Self::Error>;
}
pub trait WorkflowStageEcsWhileISO_ {
    type Input;
    type State;
    type Output;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_ecs_while(&mut self, input: Self::Input, world: &mut World) -> Self::State;
    fn run_ecs_while(&mut self, state: Self::State, world: &mut World) -> Outcome<Self::State, Self::Output>;
}
pub trait WorkflowStageEcsWhileISOE {
    type Input;
    type State;
    type Output;
    type Error;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_ecs_while(&mut self, input: Self::Input, world: &mut World) -> Result<Self::State, Self::Error>;
    fn run_ecs_while(&mut self, state: Self::State, world: &mut World) -> Result<Outcome<Self::State, Self::Output>, Self::Error>;
}



// --- Render While Workflow Stages ---
pub trait WorkflowStageRenderWhile____ {
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_render_while(&mut self, world: &mut World);
    fn run_render_while(&mut self, world: &mut World) -> Outcome<(), ()>;
}
pub trait WorkflowStageRenderWhile___E {
    type Error;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_render_while(&mut self, world: &mut World);
    fn run_render_while(&mut self, world: &mut World) -> Result<Outcome<(), ()>, Self::Error>;
}
pub trait WorkflowStageRenderWhile__O_ {
    type Output;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_render_while(&mut self, world: &mut World);
    fn run_render_while(&mut self, world: &mut World) -> Outcome<(), Self::Output>;
}
pub trait WorkflowStageRenderWhile__OE {
    type Output;
    type Error;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_render_while(&mut self, world: &mut World);
    fn run_render_while(&mut self, world: &mut World) -> Result<Outcome<(), Self::Output>, Self::Error>;
}
pub trait WorkflowStageRenderWhile_S__ {
    type State;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_render_while(&mut self, world: &mut World) -> Self::State;
    fn run_render_while(&mut self, state: Self::State, world: &mut World) -> Outcome<Self::State, ()>;
}
pub trait WorkflowStageRenderWhile_S_E {
    type State;
    type Error;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_render_while(&mut self, world: &mut World) -> Result<Self::State, Self::Error>;
    fn run_render_while(&mut self, state: Self::State, world: &mut World) -> Result<Outcome<Self::State, ()>, Self::Error>;
}
pub trait WorkflowStageRenderWhile_SO_ {
    type State;
    type Output;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_render_while(&mut self, world: &mut World) -> Self::State;
    fn run_render_while(&mut self, state: Self::State, world: &mut World) -> Outcome<Self::State, Self::Output>;
}
pub trait WorkflowStageRenderWhile_SOE {
    type State;
    type Output;
    type Error;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_render_while(&mut self, world: &mut World) -> Result<Self::State, Self::Error>;
    fn run_render_while(&mut self, state: Self::State, world: &mut World) -> Result<Outcome<Self::State, Self::Output>, Self::Error>;
}
pub trait WorkflowStageRenderWhileI___ {
    type Input;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_render_while(&mut self, input: Self::Input, world: &mut World);
    fn run_render_while(&mut self, world: &mut World) -> Outcome<(), ()>;
}
pub trait WorkflowStageRenderWhileI__E {
    type Input;
    type Error;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_render_while(&mut self, input: Self::Input, world: &mut World);
    fn run_render_while(&mut self, world: &mut World) -> Result<Outcome<(), ()>, Self::Error>;
}
pub trait WorkflowStageRenderWhileI_O_ {
    type Input;
    type Output;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_render_while(&mut self, input: Self::Input, world: &mut World);
    fn run_render_while(&mut self, world: &mut World) -> Outcome<(), Self::Output>;
}
pub trait WorkflowStageRenderWhileI_OE {
    type Input;
    type Output;
    type Error;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_render_while(&mut self, input: Self::Input, world: &mut World);
    fn run_render_while(&mut self, world: &mut World) -> Result<Outcome<(), Self::Output>, Self::Error>;
}
pub trait WorkflowStageRenderWhileIS__ {
    type Input;
    type State;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_render_while(&mut self, input: Self::Input, world: &mut World) -> Self::State;
    fn run_render_while(&mut self, state: Self::State, world: &mut World) -> Outcome<Self::State, ()>;
}
pub trait WorkflowStageRenderWhileIS_E {
    type Input;
    type State;
    type Error;

    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_render_while(&mut self, input: Self::Input, world: &mut World) -> Result<Self::State, Self::Error>;
    fn run_render_while(&mut self, state: Self::State, world: &mut World) -> Result<Outcome<Self::State, ()>, Self::Error>;
}
pub trait WorkflowStageRenderWhileISO_ {
    type Input;
    type State;
    type Output;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_render_while(&mut self, input: Self::Input, world: &mut World) -> Self::State;
    fn run_render_while(&mut self, state: Self::State, world: &mut World) -> Outcome<Self::State, Self::Output>;
}
pub trait WorkflowStageRenderWhileISOE {
    type Input;
    type State;
    type Output;
    type Error;
    
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn index(&self) -> usize;
    fn setup_render_while(&mut self, input: Self::Input, world: &mut World) -> Result<Self::State, Self::Error>;
    fn run_render_while(&mut self, state: Self::State, world: &mut World) -> Result<Outcome<Self::State, Self::Output>, Self::Error>;
}
