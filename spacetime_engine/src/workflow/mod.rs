pub mod channels;
pub mod errors;
pub mod events;
pub mod functions;
pub mod resources;
pub mod statics;
pub mod systems;
pub mod traits;
pub mod types;

pub mod instance;
pub mod request;
pub mod response;
pub mod stage;

use bevy::{
    prelude::*,
    render::{Render, RenderApp},
};
use bevy_consumable_event::ConsumableEventApp;
use channels::*;
use events::*;
use resources::*;
use systems::*;

pub(crate) struct WorkflowPlugin;
impl Plugin for WorkflowPlugin {
    fn build(&self, app: &mut App) {
        let (
            render_workflow_state_extract_reintegration_event_sender,
            render_workflow_state_extract_reintegration_event_receiver,
        ) = crossbeam_channel::unbounded();
        let render_while_workflow_state_extract_reintegration_event_sender =
            RenderWhileWorkflowStateExtractReintegrationEventSender(
                render_workflow_state_extract_reintegration_event_sender,
            );
        let render_while_workflow_state_extract_reintegration_event_receiver =
            RenderWhileWorkflowStateExtractReintegrationEventReceiver(
                render_workflow_state_extract_reintegration_event_receiver,
            );

        // --- Stage completion channels
        let (ecs_completion_sender, ecs_completion_receiver) = crossbeam_channel::unbounded();
        let ecs_completion_sender = EcsStageCompletionEventSender(ecs_completion_sender);
        let ecs_completion_receiver = EcsStageCompletionEventReceiver(ecs_completion_receiver);

        let (render_completion_sender, render_completion_receiver) = crossbeam_channel::unbounded();
        let render_completion_sender = RenderStageCompletionEventSender(render_completion_sender);
        let render_completion_receiver =
            RenderStageCompletionEventReceiver(render_completion_receiver);

        let (async_completion_sender, async_completion_receiver) = crossbeam_channel::unbounded();
        let async_completion_sender = AsyncStageCompletionEventSender(async_completion_sender);
        let async_completion_receiver =
            AsyncStageCompletionEventReceiver(async_completion_receiver);

        let (render_while_completion_sender, render_while_completion_receiver) =
            crossbeam_channel::unbounded();
        let render_while_completion_sender =
            RenderWhileStageCompletionEventSender(render_while_completion_sender);
        let render_while_completion_receiver =
            RenderWhileStageCompletionEventReceiver(render_while_completion_receiver);

        let (ecs_while_completion_sender, ecs_while_completion_receiver) =
            crossbeam_channel::unbounded();
        let ecs_while_completion_sender =
            EcsWhileStageCompletionEventSender(ecs_while_completion_sender);
        let ecs_while_completion_receiver =
            EcsWhileStageCompletionEventReceiver(ecs_while_completion_receiver);

        // --- Stage failure channels
        let (ecs_failure_sender, ecs_failure_receiver) = crossbeam_channel::unbounded();
        let ecs_failure_sender = EcsStageFailureEventSender(ecs_failure_sender);
        let ecs_failure_receiver = EcsStageFailureEventReceiver(ecs_failure_receiver);

        let (render_failure_sender, render_failure_receiver) = crossbeam_channel::unbounded();
        let render_failure_sender = RenderStageFailureEventSender(render_failure_sender);
        let render_failure_receiver =
            RenderStageFailureEventReceiver(render_failure_receiver);

        let (async_failure_sender, async_failure_receiver) = crossbeam_channel::unbounded();
        let async_failure_sender = AsyncStageFailureEventSender(async_failure_sender);
        let async_failure_receiver =
            AsyncStageFailureEventReceiver(async_failure_receiver);

        let (ecs_while_failure_sender, ecs_while_failure_receiver) =
            crossbeam_channel::unbounded();
        let ecs_while_failure_sender =
            EcsWhileStageFailureEventSender(ecs_while_failure_sender);
        let ecs_while_failure_receiver =
            EcsWhileStageFailureEventReceiver(ecs_while_failure_receiver);

        let (render_while_failure_sender, render_while_failure_receiver) =
            crossbeam_channel::unbounded();
        let render_while_failure_sender =
            RenderWhileStageFailureEventSender(render_while_failure_sender);
        let render_while_failure_receiver =
            RenderWhileStageFailureEventReceiver(render_while_failure_receiver);

        let (workflow_request_receiver, workflow_response_sender) = initialize_channels();
        let (workflow_request_e_receiver, workflow_response_e_sender) = initialize_e_channels();
        let (workflow_request_o_receiver, workflow_response_o_sender) = initialize_o_channels();
        let (workflow_request_oe_receiver, workflow_response_oe_sender) = initialize_oe_channels();
        let (workflow_request_i_receiver, workflow_response_i_sender) = initialize_i_channels();
        let (workflow_request_ie_receiver, workflow_response_ie_sender) = initialize_ie_channels();
        let (workflow_request_io_receiver, workflow_response_io_sender) = initialize_io_channels();
        let (workflow_request_ioe_receiver, workflow_response_ioe_sender) =
            initialize_ioe_channels();

        app.add_event::<WorkflowStageInitializationEvent>()
            .add_event::<WorkflowStageCompletionEvent>()
            .add_persistent_consumable_event::<WorkflowStageInitializationEvent>()
            .add_persistent_consumable_event::<WorkflowStageCompletionEvent>()
            .insert_resource(WorkflowTypeModuleRegistry::default())
            .insert_resource(WorkflowRequestBuffer::default())
            .insert_resource(WorkflowMap::default())
            .insert_resource(EcsStageBuffer::default())
            .insert_resource(EcsWhileStageBuffer::default())
            .insert_resource(RenderStageBuffer::default())
            .insert_resource(RenderWhileStageBuffer::default())
            .insert_resource(AsyncStageBuffer::default())
            .insert_resource(render_while_workflow_state_extract_reintegration_event_receiver)
            .insert_resource(ecs_completion_sender)
            .insert_resource(ecs_completion_receiver)
            .insert_resource(ecs_while_completion_sender)
            .insert_resource(ecs_while_completion_receiver)
            .insert_resource(render_completion_receiver)
            .insert_resource(render_while_completion_receiver)
            .insert_resource(async_completion_sender)
            .insert_resource(async_completion_receiver)
            .insert_resource(ecs_failure_sender)
            .insert_resource(ecs_failure_receiver)
            .insert_resource(ecs_while_failure_sender)
            .insert_resource(ecs_while_failure_receiver)
            .insert_resource(render_failure_receiver)
            .insert_resource(render_while_failure_receiver)
            .insert_resource(async_failure_sender)
            .insert_resource(async_failure_receiver)
            .insert_resource(WorkflowRequestReceiver(workflow_request_receiver))
            .insert_resource(WorkflowRequestEReceiver(workflow_request_e_receiver))
            .insert_resource(WorkflowRequestOReceiver(workflow_request_o_receiver))
            .insert_resource(WorkflowRequestOEReceiver(workflow_request_oe_receiver))
            .insert_resource(WorkflowRequestIReceiver(workflow_request_i_receiver))
            .insert_resource(WorkflowRequestIEReceiver(workflow_request_ie_receiver))
            .insert_resource(WorkflowRequestIOReceiver(workflow_request_io_receiver))
            .insert_resource(WorkflowRequestIOEReceiver(workflow_request_ioe_receiver))
            .insert_resource(WorkflowResponseSender(workflow_response_sender))
            .insert_resource(WorkflowResponseESender(workflow_response_e_sender))
            .insert_resource(WorkflowResponseOSender(workflow_response_o_sender))
            .insert_resource(WorkflowResponseOESender(workflow_response_oe_sender))
            .insert_resource(WorkflowResponseISender(workflow_response_i_sender))
            .insert_resource(WorkflowResponseIESender(workflow_response_ie_sender))
            .insert_resource(WorkflowResponseIOSender(workflow_response_io_sender))
            .insert_resource(WorkflowResponseIOESender(workflow_response_ioe_sender))
            .add_systems(
                PreUpdate,
                (
                    handle_ecs_stage_completion_event_system,
                    handle_ecs_while_stage_completion_event_system,
                    handle_render_stage_completion_event_system,
                    handle_render_while_stage_completion_event_system,
                    handle_async_stage_completion_event_system,
                ),
            )
            .add_systems(
                Update,
                (
                    poll_ecs_stage_buffer_system,
                    poll_ecs_while_stage_buffer_system,
                    poll_async_stage_buffer_system,
                ),
            )
            .add_systems(
                PostUpdate,
                (
                    (
                        render_while_workflow_state_extract_reintegration_system,
                        workflow_request_relay_system,
                        workflow_request_e_relay_system,
                        workflow_request_o_relay_system,
                        workflow_request_oe_relay_system,
                        workflow_request_i_relay_system,
                        workflow_request_ie_relay_system,
                        workflow_request_io_relay_system,
                        workflow_request_ioe_relay_system,
                    )
                        .before(workflow_request_system),
                    workflow_request_system,
                    workflow_execution_system.after(workflow_request_system),
                    workflow_completion_handling_system.after(workflow_execution_system),
                ),
            );

        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .insert_resource(render_while_workflow_state_extract_reintegration_event_sender)
            .insert_resource(render_completion_sender)
            .insert_resource(render_while_completion_sender)
            .add_systems(
                ExtractSchedule,
                (
                    extract_render_stage_buffer_system,
                    extract_render_while_workflow_state_extract_system,
                    extract_render_while_stage_buffer_system,
                ),
            )
            .add_systems(
                Render,
                (
                    poll_render_stage_buffer_system,
                    poll_render_while_stage_buffer_system,
                ),
            );
    }
}
