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

#[macro_export]
macro_rules! workflow_stage_core_types_util {
    () => {
        use bevy::ecs::system::SystemParam;

        use super::core_functions::*;
        use super::super::super::user_imports::*;
        use super::super::super::user_items::*;
    };
}

#[macro_export]
macro_rules! workflow_stage_core_functions_util {
    () => {
        use bevy::ecs::system::SystemParam;

        use super::core_types::*;
        use super::super::super::user_imports::*;
        use super::super::super::user_items::*;
    };
}
pub(crate) struct WorkflowPlugin;
impl Plugin for WorkflowPlugin {
    fn build(&self, app: &mut App) {
        let (setup_receiver, wait_receiver, completion_receiver, failure_receiver) =
            initialize_stage_channels();
        let setup_receiver = StageSetupEventReceiver(setup_receiver);
        let wait_receiver = StageWaitEventReceiver(wait_receiver);
        let completion_receiver = StageCompletionEventReceiver(completion_receiver);
        let failure_receiver = StageFailureEventReceiver(failure_receiver);

        let (workflow_request_receiver, workflow_response_sender) = initialize_channels();
        let (workflow_request_e_receiver, workflow_response_e_sender) = initialize_e_channels();
        let (workflow_request_o_receiver, workflow_response_o_sender) = initialize_o_channels();
        let (workflow_request_oe_receiver, workflow_response_oe_sender) = initialize_oe_channels();
        let (workflow_request_i_receiver, workflow_response_i_sender) = initialize_i_channels();
        let (workflow_request_ie_receiver, workflow_response_ie_sender) = initialize_ie_channels();
        let (workflow_request_io_receiver, workflow_response_io_sender) = initialize_io_channels();
        let (workflow_request_ioe_receiver, workflow_response_ioe_sender) =
            initialize_ioe_channels();

        app.add_event::<StageInitializationEvent>()
            .add_event::<StageSetupEvent>()
            .add_event::<StageWaitEvent>()
            .add_event::<StageCompletionEvent>()
            .add_event::<StageFailureEvent>()
            .add_persistent_consumable_event::<StageInitializationEvent>()
            .add_persistent_consumable_event::<StageSetupEvent>()
            .add_persistent_consumable_event::<StageWaitEvent>()
            .add_persistent_consumable_event::<StageCompletionEvent>()
            .add_persistent_consumable_event::<StageFailureEvent>()
            .insert_resource(WorkflowTypeModuleRegistry::default())
            .insert_resource(WorkflowRequestBuffer::default())
            .insert_resource(WorkflowMap::default())
            .insert_resource(EcsStageBuffer::default())
            .insert_resource(EcsWhileStageBuffer::default())
            .insert_resource(RenderStageBuffer::default())
            .insert_resource(RenderWhileStageBuffer::default())
            .insert_resource(AsyncStageBuffer::default())
            .insert_resource(setup_receiver)
            .insert_resource(wait_receiver)
            .insert_resource(completion_receiver)
            .insert_resource(failure_receiver)
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
                    (
                        workflow_request_relay_system,
                        workflow_request_e_relay_system,
                        workflow_request_o_relay_system,
                        workflow_request_oe_relay_system,
                        workflow_request_i_relay_system,
                        workflow_request_ie_relay_system,
                        workflow_request_io_relay_system,
                        workflow_request_ioe_relay_system,
                    )
                        .before(stage_setup_relay_system),
                    (
                        stage_setup_relay_system,
                        stage_wait_relay_system,
                        stage_completion_relay_system,
                        stage_failure_relay_system,
                    )
                        .before(workflow_request_system),
                    workflow_request_system,
                    workflow_initialization_system.after(workflow_request_system),
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
                    workflow_wait_handling_system,
                    workflow_completion_handling_system,
                    workflow_failure_handling_system,
                ),
            );

        let render_app = app.sub_app_mut(RenderApp);
        render_app
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
