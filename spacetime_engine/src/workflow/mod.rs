pub mod errors;
pub mod events;
pub mod functions;
pub mod resources;
pub mod systems;
pub mod traits;
pub mod types;

pub mod instance;
pub mod io;
pub mod stage;
pub mod request;
pub mod response;

use bevy::{prelude::*, render::{Render, RenderApp}};
use bevy_consumable_event::ConsumableEventApp;
use events::*;
use resources::*;
use systems::*;

pub const DEBUG_ACTION_MODULE: &str = "GPU";
pub const DEBUG_ACTION_NAME: &str = "SetupTextureGenerator";
pub const DEBUG_LOGGING_ENABLED: bool = true;

pub(in crate) struct WorkflowPlugin;
impl Plugin for WorkflowPlugin {
    fn build(&self, app: &mut App) {
        let (ecs_completion_sender, ecs_completion_receiver) = crossbeam_channel::unbounded();
        let ecs_completion_sender = EcsStageCompletionEventSender(ecs_completion_sender);
        let ecs_completion_receiver = EcsStageCompletionEventReceiver(ecs_completion_receiver);

        let (ecs_while_completion_sender, ecs_while_completion_receiver) = crossbeam_channel::unbounded();
        let ecs_while_completion_sender = EcsWhileStageCompletionEventSender(ecs_while_completion_sender);
        let ecs_while_completion_receiver = EcsWhileStageCompletionEventReceiver(ecs_while_completion_receiver);

        let (render_completion_sender, render_completion_receiver) = crossbeam_channel::unbounded();
        let render_completion_sender = RenderStageCompletionEventSender(render_completion_sender);
        let render_completion_receiver = RenderStageCompletionEventReceiver(render_completion_receiver);

        let (render_while_completion_sender, render_while_completion_receiver) = crossbeam_channel::unbounded();
        let render_while_completion_sender = RenderWhileStageCompletionEventSender(render_while_completion_sender);
        let render_while_completion_receiver = RenderWhileStageCompletionEventReceiver(render_while_completion_receiver);

        let (async_completion_sender, async_completion_receiver) = crossbeam_channel::unbounded();
        let async_completion_sender = AsyncStageCompletionEventSender(async_completion_sender);
        let async_completion_receiver = AsyncStageCompletionEventReceiver(async_completion_receiver);

        app
            .add_event::<WorkflowStageInitializationEvent>()
            .add_event::<WorkflowStageCompletionEvent>()
            .add_persistent_consumable_event::<WorkflowStageInitializationEvent>()
            .add_persistent_consumable_event::<WorkflowStageCompletionEvent>()
            .insert_resource(ecs_completion_sender)
            .insert_resource(ecs_completion_receiver)
            .insert_resource(ecs_while_completion_sender)
            .insert_resource(ecs_while_completion_receiver)
            .insert_resource(render_completion_receiver)
            .insert_resource(render_while_completion_receiver)
            .insert_resource(async_completion_sender)
            .insert_resource(async_completion_receiver)
            .insert_resource(WorkflowTypeModuleRegistry::default())
            .insert_resource(WorkflowRequestBuffer::default())
            .insert_resource(WorkflowMap::default())
            .insert_resource(EcsStageBuffer::default())
            .insert_resource(EcsWhileStageBuffer::default())
            .insert_resource(RenderStageBuffer::default())
            .insert_resource(RenderWhileStageBuffer::default())
            .insert_resource(AsyncStageBuffer::default())
            .insert_resource(WorkflowRequestChannel::default())
            .insert_resource(WorkflowRequestEChannel::default())
            .insert_resource(WorkflowRequestOChannel::default())
            .insert_resource(WorkflowRequestOEChannel::default())
            .insert_resource(WorkflowRequestIChannel::default())
            .insert_resource(WorkflowRequestIEChannel::default())
            .insert_resource(WorkflowRequestIOChannel::default())
            .insert_resource(WorkflowRequestIOEChannel::default())
            .insert_resource(WorkflowResponseChannel::default())
            .insert_resource(WorkflowResponseEChannel::default())
            .insert_resource(WorkflowResponseOChannel::default())
            .insert_resource(WorkflowResponseOEChannel::default())
            .insert_resource(WorkflowResponseIChannel::default())
            .insert_resource(WorkflowResponseIEChannel::default())
            .insert_resource(WorkflowResponseIOChannel::default())
            .insert_resource(WorkflowResponseIOEChannel::default())
            .add_systems(PreUpdate, (
                handle_ecs_stage_completion_event_system,
                handle_ecs_while_stage_completion_event_system,
                handle_render_stage_completion_event_system,
                handle_render_while_stage_completion_event_system,
                handle_async_stage_completion_event_system,
            ))
            .add_systems(Update, (
                poll_ecs_stage_buffer_system,
                poll_ecs_while_stage_buffer_system,
                poll_async_stage_buffer_system
            ))
            .add_systems(PostUpdate, (
                (
                    workflow_request_relay_system, 
                    workflow_request_e_relay_system,
                    workflow_request_o_relay_system,
                    workflow_request_oe_relay_system,
                    workflow_request_i_relay_system,
                    workflow_request_ie_relay_system,
                    workflow_request_io_relay_system,
                    workflow_request_ioe_relay_system
                ).before(workflow_request_system),
                workflow_request_system,
                workflow_execution_system.after(workflow_request_system),
                workflow_completion_handling_system.after(workflow_execution_system)
            ));

        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .insert_resource(render_completion_sender)
            .insert_resource(render_while_completion_sender)
            .add_systems(ExtractSchedule, (
                extract_render_stage_buffer_system,
                extract_render_while_stage_buffer_system
            ))
            .add_systems(Render, (
                poll_render_stage_buffer_system,
                poll_render_while_stage_buffer_system
            ));
    }
}