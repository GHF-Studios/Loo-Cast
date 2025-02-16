pub mod errors;
pub mod events;
pub mod functions;
pub mod resources;
pub mod systems;
pub mod types;

pub mod stage_io;
pub mod stage;
pub mod target;

use bevy::{prelude::*, render::{Render, RenderApp}};
use bevy_consumable_event::ConsumableEventApp;
use events::*;
use resources::*;
use systems::*;

pub const DEBUG_ACTION_MODULE: &str = "GPU";
pub const DEBUG_ACTION_NAME: &str = "SetupTextureGenerator";
pub const DEBUG_LOGGING_ENABLED: bool = true;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PreUpdateSystems;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UpdateSystems;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PostUpdateSystems;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExtractSystems;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct RenderSystems;

pub(in crate) struct WorkflowPlugin;
impl Plugin for WorkflowPlugin {
    fn build(&self, app: &mut App) {
        let (ecs_sender, ecs_receiver) = crossbeam_channel::unbounded();
        let ecs_sender = EcsStageCompletionEventSender(ecs_sender);
        let ecs_receiver = EcsStageCompletionEventReceiver(ecs_receiver);

        let (ecs_while_sender, ecs_while_receiver) = crossbeam_channel::unbounded();
        let ecs_while_sender = EcsWhileStageCompletionEventSender(ecs_while_sender);
        let ecs_while_receiver = EcsWhileStageCompletionEventReceiver(ecs_while_receiver);

        let (render_sender, render_receiver) = crossbeam_channel::unbounded();
        let render_sender = RenderStageCompletionEventSender(render_sender);
        let render_receiver = WorkflowStageCompletionEventReceiverRender(render_receiver);

        let (render_while_sender, render_while_receiver) = crossbeam_channel::unbounded();
        let render_while_sender = WorkflowStageCompletionEventSenderRenderWhile(render_while_sender);
        let render_while_receiver = WorkflowStageCompletionEventReceiverRenderWhile(render_while_receiver);
        
        let (async_sender, async_receiver) = crossbeam_channel::unbounded();
        let async_sender = WorkflowStageCompletionEventSenderAsync(async_sender);
        let async_receiver = WorkflowStageCompletionEventReceiverAsync(async_receiver);

        app
            .add_event::<WorkflowStageCompletionEvent>()
            // TODO: Make persistent if the need arises
            .add_consumable_event::<WorkflowStageCompletionEvent>()
            .insert_resource(ecs_sender)
            .insert_resource(ecs_receiver)
            .insert_resource(ecs_while_sender)
            .insert_resource(ecs_while_receiver)
            .insert_resource(render_sender)
            .insert_resource(render_receiver)
            .insert_resource(render_while_sender)
            .insert_resource(render_while_receiver)
            .insert_resource(async_sender)
            .insert_resource(async_receiver)
            .insert_resource(WorkflowTypeModuleRegistry::default())
            .insert_resource(WorkflowRequestBuffer::default())
            .insert_resource(WorkflowMap::default())
            .insert_resource(EcsStageCompletionEventQueue::default())
            .insert_resource(EcsWhileStageCompletionEventQueue::default())
            .insert_resource(RenderStageCompletionEventQueue::default())
            .insert_resource(RenderWhileStageCompletionEventQueue::default())
            .insert_resource(AsyncStageCompletionEventQueue::default())
            .add_systems(PreUpdate, workflow_request_system)
            .add_systems(PreUpdate, (
                handle_ecs_stage_completion_event_system,
                handle_ecs_while_stage_completion_event_system,
                handle_render_stage_completion_event_system, 
                handle_render_while_stage_completion_event_system,
                handle_async_stage_completion_event_system
            ).in_set(PreUpdateSystems).after(workflow_request_system))
            .add_systems(Update, (
                execute_ecs_stages_system,
                execute_ecs_while_stages_system,
                execute_async_stages_system
            ).in_set(UpdateSystems))
            .add_systems(PostUpdate, (
                stage_execution_cleanup_system,
                workflow_progression_system.after(stage_execution_cleanup_system)
            ).in_set(PostUpdateSystems));

        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .add_systems(ExtractSchedule, (
                extract_render_stage_queue_system,
                extract_render_while_stage_queue_system
            ).in_set(ExtractSystems))
            .add_systems(Render, (
                execute_render_stages_system.after(extract_render_stage_queue_system),
                execute_render_while_stages_system.after(extract_render_while_stage_queue_system)
            ).in_set(RenderSystems));
    }
}