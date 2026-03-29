use std::collections::VecDeque;

use crate::bevy::ecs::system::SystemState;
use crate::bevy::prelude::*;
use crate::bevy::render::MainWorld;
use bevy_consumable_message::{ConsumableMessageReader, ConsumableMessageWriter};

use crate::{utils::premium_box::AnySendSyncPremiumBox, workflow::response::*};

use super::{channels::*, instance::*, messages::*, resources::*, stage::Stage, types::*};

pub(super) fn build_stage_sender_caches() -> (
    EcsStageSenderCache,
    RenderStageSenderCache,
    AsyncStageSenderCache,
    EcsWhileStageSenderCache,
    RenderWhileStageSenderCache,
) {
    let modules_metadata = crate::get_workflow_modules_metadata();
    let mut ecs = EcsStageSenderCache::default();
    let mut render = RenderStageSenderCache::default();
    let mut async_stage = AsyncStageSenderCache::default();
    let mut ecs_while = EcsWhileStageSenderCache::default();
    let mut render_while = RenderWhileStageSenderCache::default();

    for module_metadata in modules_metadata.iter() {
        for workflow_metadata in module_metadata.workflows.iter() {
            for stage_metadata in workflow_metadata.stages.iter() {
                match stage_metadata {
                    crate::WorkflowStageMetadata::Ecs { name, sender } => {
                        ecs.0.insert((module_metadata.name, workflow_metadata.name, *name), sender.clone());
                    }
                    crate::WorkflowStageMetadata::Render { name, sender } => {
                        render.0.insert((module_metadata.name, workflow_metadata.name, *name), sender.clone());
                    }
                    crate::WorkflowStageMetadata::Async { name, sender } => {
                        async_stage
                            .0
                            .insert((module_metadata.name, workflow_metadata.name, *name), sender.clone());
                    }
                    crate::WorkflowStageMetadata::EcsWhile { name, sender } => {
                        ecs_while
                            .0
                            .insert((module_metadata.name, workflow_metadata.name, *name), sender.clone());
                    }
                    crate::WorkflowStageMetadata::RenderWhile { name, sender } => {
                        render_while
                            .0
                            .insert((module_metadata.name, workflow_metadata.name, *name), sender.clone());
                    }
                }
            }
        }
    }

    (ecs, render, async_stage, ecs_while, render_while)
}

pub(super) fn extract_render_stage_buffer_system(world: &mut World) {
    let mut main_world = SystemState::<ResMut<MainWorld>>::new(world).get_mut(world);
    let extracted_buffer = match main_world.get_resource_mut::<RenderStageBuffer>() {
        Some(mut buffer) => std::mem::take(&mut *buffer),
        None => unreachable!("Render stage buffer resource not found"),
    };

    match world.get_resource_mut::<RenderStageBuffer>() {
        Some(mut buffer) => {
            buffer.0.extend(extracted_buffer.0);
        }
        None => {
            world.insert_resource(extracted_buffer);
        }
    }
}
pub(super) fn extract_render_while_workflow_state_extract_system(world: &mut World) {
    let main_world = SystemState::<Res<MainWorld>>::new(world).get_mut(world);
    let render_while_workflow_state_extract: RenderWhileWorkflowStateExtract = main_world.resource::<WorkflowMap>().into();

    match world.get_resource_mut::<RenderWhileWorkflowStateExtract>() {
        Some(mut resource) => {
            *resource = render_while_workflow_state_extract;
        }
        None => {
            world.insert_resource(render_while_workflow_state_extract);
        }
    }
}
pub(super) fn extract_render_while_stage_buffer_system(world: &mut World) {
    let mut main_world = SystemState::<ResMut<MainWorld>>::new(world).get_mut(world);
    let extracted_buffer = match main_world.get_resource_mut::<RenderWhileStageBuffer>() {
        Some(mut buffer) => std::mem::take(&mut *buffer),
        None => unreachable!("Render while stage buffer resource not found"),
    };

    match world.get_resource_mut::<RenderWhileStageBuffer>() {
        Some(mut buffer) => {
            buffer.0.extend(extracted_buffer.0);
        }
        None => {
            world.insert_resource(extracted_buffer);
        }
    }
}

pub(super) fn send_ecs_stages_to_ecs_buffers_system(mut buffer: ResMut<EcsStageBuffer>, sender_cache: Res<EcsStageSenderCache>) {
    let drained_buffer = { std::mem::take(&mut buffer.0) };

    for (module_name, workflow_name, current_stage, stage, data_buffer) in drained_buffer {
        let sender = sender_cache
            .0
            .get(&(module_name, workflow_name, stage.name))
            .unwrap_or_else(|| unreachable!("Ecs stage sender not found for {module_name}::{workflow_name}::{}", stage.name));

        sender.send(module_name, workflow_name, current_stage, stage, data_buffer);
    }
}
pub(super) fn send_render_stages_to_render_buffers_system(mut buffer: ResMut<RenderStageBuffer>, sender_cache: Res<RenderStageSenderCache>) {
    let drained_buffer = { std::mem::take(&mut buffer.0) };

    for (module_name, workflow_name, current_stage, stage, data_buffer) in drained_buffer {
        let sender = sender_cache
            .0
            .get(&(module_name, workflow_name, stage.name))
            .unwrap_or_else(|| unreachable!("Render stage sender not found for {module_name}::{workflow_name}::{}", stage.name));

        sender.send(module_name, workflow_name, current_stage, stage, data_buffer);
    }
}
pub(super) fn send_async_stages_to_async_buffers_system(mut buffer: ResMut<AsyncStageBuffer>, sender_cache: Res<AsyncStageSenderCache>) {
    let drained_buffer = { std::mem::take(&mut buffer.0) };

    for (module_name, workflow_name, current_stage, stage, data_buffer) in drained_buffer {
        let sender = sender_cache
            .0
            .get(&(module_name, workflow_name, stage.name))
            .unwrap_or_else(|| unreachable!("Async stage sender not found for {module_name}::{workflow_name}::{}", stage.name));

        sender.send(module_name, workflow_name, current_stage, stage, data_buffer);
    }
}
pub(super) fn send_ecs_while_stages_to_ecs_while_buffers_system(
    mut buffer: ResMut<EcsWhileStageBuffer>,
    sender_cache: Res<EcsWhileStageSenderCache>,
) {
    let drained_buffer = { std::mem::take(&mut buffer.0) };

    for (module_name, workflow_name, current_stage, stage, data_buffer) in drained_buffer {
        let sender = sender_cache
            .0
            .get(&(module_name, workflow_name, stage.name))
            .unwrap_or_else(|| unreachable!("EcsWhile stage sender not found for {module_name}::{workflow_name}::{}", stage.name));

        sender.send(module_name, workflow_name, current_stage, stage, data_buffer);
    }
}
pub(super) fn send_render_while_stages_to_render_while_buffers_system(
    mut buffer: ResMut<RenderWhileStageBuffer>,
    sender_cache: Res<RenderWhileStageSenderCache>,
) {
    let drained_buffer = { std::mem::take(&mut buffer.0) };

    for (module_name, workflow_name, current_stage, stage, data_buffer) in drained_buffer {
        let sender = sender_cache
            .0
            .get(&(module_name, workflow_name, stage.name))
            .unwrap_or_else(|| unreachable!("RenderWhile stage sender not found for {module_name}::{workflow_name}::{}", stage.name));

        sender.send(module_name, workflow_name, current_stage, stage, data_buffer);
    }
}

/// Note: We actually convert the message from 'setup' to 'wait', seeing as the message handler logic from post-setup is identical to that of post-run
pub(super) fn stage_setup_relay_system(
    stage_message_receiver: Res<StageSetupMessageReceiver>,
    mut stage_message_writer: ConsumableMessageWriter<StageWaitMessage>,
) {
    while let Ok(StageSetupMessage {
        ty,
        module_name,
        workflow_name,
        current_stage,
        stage_return,
        stage_state,
    }) = stage_message_receiver.0.try_recv()
    {
        stage_message_writer.send(StageWaitMessage {
            ty,
            module_name,
            workflow_name,
            current_stage,
            stage_return,
            stage_state,
        });
    }
}
pub(super) fn stage_wait_relay_system(
    stage_message_receiver: Res<StageWaitMessageReceiver>,
    mut stage_message_writer: ConsumableMessageWriter<StageWaitMessage>,
) {
    while let Ok(StageWaitMessage {
        ty,
        module_name,
        workflow_name,
        current_stage,
        stage_return,
        stage_state,
    }) = stage_message_receiver.0.try_recv()
    {
        stage_message_writer.send(StageWaitMessage {
            ty,
            module_name,
            workflow_name,
            current_stage,
            stage_return,
            stage_state,
        });
    }
}
pub(super) fn stage_completion_relay_system(
    stage_message_receiver: Res<StageCompletionMessageReceiver>,
    mut stage_message_writer: ConsumableMessageWriter<StageCompletionMessage>,
) {
    while let Ok(StageCompletionMessage {
        ty,
        module_name,
        workflow_name,
        current_stage,
        stage_output,
        stage_return,
    }) = stage_message_receiver.0.try_recv()
    {
        stage_message_writer.send(StageCompletionMessage {
            ty,
            module_name,
            workflow_name,
            current_stage,
            stage_output,
            stage_return,
        });
    }
}
pub(super) fn stage_failure_relay_system(
    stage_message_receiver: Res<StageFailureMessageReceiver>,
    mut stage_message_writer: ConsumableMessageWriter<StageFailureMessage>,
) {
    while let Ok(StageFailureMessage {
        ty,
        module_name,
        workflow_name,
        current_stage,
        stage_error,
        stage_return,
    }) = stage_message_receiver.0.try_recv()
    {
        stage_message_writer.send(StageFailureMessage {
            ty,
            module_name,
            workflow_name,
            current_stage,
            stage_error,
            stage_return,
        });
    }
}

pub(super) struct RetryRequest {
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub action: Box<dyn FnOnce(&mut WorkflowMap) + Send + Sync>,
}

pub(super) fn workflow_request_relay_system(world: &mut World) {
    let mut system_state: SystemState<(
        Local<VecDeque<RetryRequest>>,
        Local<VecDeque<RetryRequest>>,
        ResMut<WorkflowTypeModuleRegistry>,
        ResMut<WorkflowMap>,
        ResMut<WorkflowRequestReceiver>,
        ResMut<WorkflowResponseSender>,
    )> = SystemState::new(world);

    let (mut try_now, mut retry_next_frame, workflow_registry, mut workflow_map, mut workflow_request_receiver, workflow_response_sender) =
        system_state.get_mut(world);

    while let Ok(request) = workflow_request_receiver.0.try_recv() {
        let module_name = request.module_name;
        let workflow_name = request.workflow_name;
        let request_id = request.request_id;
        let num_stages = workflow_registry.get_workflow_type(module_name, workflow_name).unwrap().stages.len();

        let response_sender = workflow_response_sender.0.clone();

        let action = Box::new(move |workflow_map: &mut WorkflowMap| {
            workflow_map.insert_workflow(WorkflowInstance::new_request(
                module_name,
                workflow_name,
                request_id,
                num_stages,
                Box::new(move |response| {
                    let response: TypedWorkflowResponse = response.into_inner();
                    response_sender.send(TypedWorkflowResponseEnvelope { request_id, response }).unwrap();
                }),
            ));
        });

        try_now.push_back(RetryRequest {
            module_name,
            workflow_name,
            action,
        });
    }

    while let Some(retry) = try_now.pop_front() {
        if !workflow_map.has_workflow(retry.module_name, retry.workflow_name) {
            (retry.action)(&mut workflow_map);
        } else {
            retry_next_frame.push_back(retry);
        }
    }

    std::mem::swap(&mut retry_next_frame, &mut try_now);
}

pub(super) fn workflow_request_e_relay_system(world: &mut World) {
    let mut system_state: SystemState<(
        Local<VecDeque<RetryRequest>>,
        Local<VecDeque<RetryRequest>>,
        ResMut<WorkflowTypeModuleRegistry>,
        ResMut<WorkflowMap>,
        ResMut<WorkflowRequestEReceiver>,
        ResMut<WorkflowResponseESender>,
    )> = SystemState::new(world);

    let (mut try_now, mut retry_next_frame, workflow_registry, mut workflow_map, mut workflow_request_receiver, workflow_response_sender) =
        system_state.get_mut(world);

    while let Ok(request) = workflow_request_receiver.0.try_recv() {
        let module_name = request.module_name;
        let workflow_name = request.workflow_name;
        let request_id = request.request_id;
        let num_stages = workflow_registry.get_workflow_type(module_name, workflow_name).unwrap().stages.len();

        let response_sender = workflow_response_sender.0.clone();

        let action = Box::new(move |workflow_map: &mut WorkflowMap| {
            workflow_map.insert_workflow(WorkflowInstance::new_request_e(
                module_name,
                workflow_name,
                request_id,
                num_stages,
                Box::new(move |response| {
                    let response: TypedWorkflowResponseE = response.into_inner();
                    response_sender.send(TypedWorkflowResponseEEnvelope { request_id, response }).unwrap();
                }),
            ));
        });

        try_now.push_back(RetryRequest {
            module_name,
            workflow_name,
            action,
        });
    }

    while let Some(retry) = try_now.pop_front() {
        if !workflow_map.has_workflow(retry.module_name, retry.workflow_name) {
            (retry.action)(&mut workflow_map);
        } else {
            retry_next_frame.push_back(retry);
        }
    }

    std::mem::swap(&mut retry_next_frame, &mut try_now);
}

pub(super) fn workflow_request_o_relay_system(world: &mut World) {
    let mut system_state: SystemState<(
        Local<VecDeque<RetryRequest>>,
        Local<VecDeque<RetryRequest>>,
        ResMut<WorkflowTypeModuleRegistry>,
        ResMut<WorkflowMap>,
        ResMut<WorkflowRequestOReceiver>,
        ResMut<WorkflowResponseOSender>,
    )> = SystemState::new(world);

    let (mut try_now, mut retry_next_frame, workflow_registry, mut workflow_map, mut workflow_request_receiver, workflow_response_sender) =
        system_state.get_mut(world);

    while let Ok(request) = workflow_request_receiver.0.try_recv() {
        let module_name = request.module_name;
        let workflow_name = request.workflow_name;
        let request_id = request.request_id;
        let num_stages = workflow_registry.get_workflow_type(module_name, workflow_name).unwrap().stages.len();

        // warn!("Received workflow_request_o for '{}::{}'", module_name, workflow_name);

        let response_sender = workflow_response_sender.0.clone();

        let action = Box::new(move |workflow_map: &mut WorkflowMap| {
            workflow_map.insert_workflow(WorkflowInstance::new_request_o(
                module_name,
                workflow_name,
                request_id,
                num_stages,
                Box::new(move |response| {
                    let response: TypedWorkflowResponseO = response.into_inner();
                    response_sender.send(TypedWorkflowResponseOEnvelope { request_id, response }).unwrap();
                }),
            ));
        });

        try_now.push_back(RetryRequest {
            module_name,
            workflow_name,
            action,
        });
    }

    while let Some(retry) = try_now.pop_front() {
        if !workflow_map.has_workflow(retry.module_name, retry.workflow_name) {
            (retry.action)(&mut workflow_map);
        } else {
            retry_next_frame.push_back(retry);
        }
    }

    std::mem::swap(&mut retry_next_frame, &mut try_now);
}

pub(super) fn workflow_request_oe_relay_system(world: &mut World) {
    let mut system_state: SystemState<(
        Local<VecDeque<RetryRequest>>,
        Local<VecDeque<RetryRequest>>,
        ResMut<WorkflowTypeModuleRegistry>,
        ResMut<WorkflowMap>,
        ResMut<WorkflowRequestOEReceiver>,
        ResMut<WorkflowResponseOESender>,
    )> = SystemState::new(world);

    let (mut try_now, mut retry_next_frame, workflow_registry, mut workflow_map, mut workflow_request_receiver, workflow_response_sender) =
        system_state.get_mut(world);

    while let Ok(request) = workflow_request_receiver.0.try_recv() {
        let module_name = request.module_name;
        let workflow_name = request.workflow_name;
        let request_id = request.request_id;
        let num_stages = workflow_registry.get_workflow_type(module_name, workflow_name).unwrap().stages.len();

        let response_sender = workflow_response_sender.0.clone();

        let action = Box::new(move |workflow_map: &mut WorkflowMap| {
            workflow_map.insert_workflow(WorkflowInstance::new_request_oe(
                module_name,
                workflow_name,
                request_id,
                num_stages,
                Box::new(move |response| {
                    let response: TypedWorkflowResponseOE = response.into_inner();
                    response_sender.send(TypedWorkflowResponseOEEnvelope { request_id, response }).unwrap();
                }),
            ));
        });

        try_now.push_back(RetryRequest {
            module_name,
            workflow_name,
            action,
        });
    }

    while let Some(retry) = try_now.pop_front() {
        if !workflow_map.has_workflow(retry.module_name, retry.workflow_name) {
            (retry.action)(&mut workflow_map);
        } else {
            retry_next_frame.push_back(retry);
        }
    }

    std::mem::swap(&mut retry_next_frame, &mut try_now);
}

pub(super) fn workflow_request_i_relay_system(world: &mut World) {
    let mut system_state: SystemState<(
        Local<VecDeque<RetryRequest>>,
        Local<VecDeque<RetryRequest>>,
        ResMut<WorkflowTypeModuleRegistry>,
        ResMut<WorkflowMap>,
        ResMut<WorkflowRequestIReceiver>,
        ResMut<WorkflowResponseISender>,
    )> = SystemState::new(world);

    let (mut try_now, mut retry_next_frame, workflow_registry, mut workflow_map, mut workflow_request_receiver, workflow_response_sender) =
        system_state.get_mut(world);

    while let Ok(request) = workflow_request_receiver.0.try_recv() {
        let module_name = request.module_name;
        let workflow_name = request.workflow_name;
        let request_id = request.request_id;
        let input = request.input;
        let num_stages = workflow_registry.get_workflow_type(module_name, workflow_name).unwrap().stages.len();

        let response_sender = workflow_response_sender.0.clone();

        let action = Box::new(move |workflow_map: &mut WorkflowMap| {
            workflow_map.insert_workflow(WorkflowInstance::new_request_i(
                module_name,
                workflow_name,
                request_id,
                input,
                num_stages,
                Box::new(move |response| {
                    let response: TypedWorkflowResponse = response.into_inner();
                    response_sender.send(TypedWorkflowResponseEnvelope { request_id, response }).unwrap();
                }),
            ));
        });

        try_now.push_back(RetryRequest {
            module_name,
            workflow_name,
            action,
        });
    }

    while let Some(retry) = try_now.pop_front() {
        if !workflow_map.has_workflow(retry.module_name, retry.workflow_name) {
            (retry.action)(&mut workflow_map);
        } else {
            retry_next_frame.push_back(retry);
        }
    }

    std::mem::swap(&mut retry_next_frame, &mut try_now);
}

pub(super) fn workflow_request_ie_relay_system(world: &mut World) {
    let mut system_state: SystemState<(
        Local<VecDeque<RetryRequest>>,
        Local<VecDeque<RetryRequest>>,
        ResMut<WorkflowTypeModuleRegistry>,
        ResMut<WorkflowMap>,
        ResMut<WorkflowRequestIEReceiver>,
        ResMut<WorkflowResponseIESender>,
    )> = SystemState::new(world);

    let (mut try_now, mut retry_next_frame, workflow_registry, mut workflow_map, mut workflow_request_receiver, workflow_response_sender) =
        system_state.get_mut(world);

    while let Ok(request) = workflow_request_receiver.0.try_recv() {
        let module_name = request.module_name;
        let workflow_name = request.workflow_name;
        let request_id = request.request_id;
        let input = request.input;
        let num_stages = workflow_registry.get_workflow_type(module_name, workflow_name).unwrap().stages.len();

        let response_sender = workflow_response_sender.0.clone();

        let action = Box::new(move |workflow_map: &mut WorkflowMap| {
            workflow_map.insert_workflow(WorkflowInstance::new_request_ie(
                module_name,
                workflow_name,
                request_id,
                input,
                num_stages,
                Box::new(move |response| {
                    let response: TypedWorkflowResponseE = response.into_inner();
                    response_sender.send(TypedWorkflowResponseEEnvelope { request_id, response }).unwrap();
                }),
            ));
        });

        try_now.push_back(RetryRequest {
            module_name,
            workflow_name,
            action,
        });
    }

    while let Some(retry) = try_now.pop_front() {
        if !workflow_map.has_workflow(retry.module_name, retry.workflow_name) {
            (retry.action)(&mut workflow_map);
        } else {
            retry_next_frame.push_back(retry);
        }
    }

    std::mem::swap(&mut retry_next_frame, &mut try_now);
}

pub(super) fn workflow_request_io_relay_system(world: &mut World) {
    let mut system_state: SystemState<(
        Local<VecDeque<RetryRequest>>,
        Local<VecDeque<RetryRequest>>,
        ResMut<WorkflowTypeModuleRegistry>,
        ResMut<WorkflowMap>,
        ResMut<WorkflowRequestIOReceiver>,
        ResMut<WorkflowResponseIOSender>,
    )> = SystemState::new(world);

    let (mut try_now, mut retry_next_frame, workflow_registry, mut workflow_map, mut workflow_request_receiver, workflow_response_sender) =
        system_state.get_mut(world);

    while let Ok(request) = workflow_request_receiver.0.try_recv() {
        let module_name = request.module_name;
        let workflow_name = request.workflow_name;
        let request_id = request.request_id;
        let input = request.input;
        let num_stages = workflow_registry.get_workflow_type(module_name, workflow_name).unwrap().stages.len();

        let response_sender = workflow_response_sender.0.clone();

        let action = Box::new(move |workflow_map: &mut WorkflowMap| {
            workflow_map.insert_workflow(WorkflowInstance::new_request_io(
                module_name,
                workflow_name,
                request_id,
                input,
                num_stages,
                Box::new(move |response| {
                    let response: TypedWorkflowResponseO = response.into_inner();
                    response_sender.send(TypedWorkflowResponseOEnvelope { request_id, response }).unwrap();
                }),
            ));
        });

        try_now.push_back(RetryRequest {
            module_name,
            workflow_name,
            action,
        });
    }

    while let Some(retry) = try_now.pop_front() {
        if !workflow_map.has_workflow(retry.module_name, retry.workflow_name) {
            (retry.action)(&mut workflow_map);
        } else {
            retry_next_frame.push_back(retry);
        }
    }

    std::mem::swap(&mut retry_next_frame, &mut try_now);
}

pub(super) fn workflow_request_ioe_relay_system(world: &mut World) {
    let mut system_state: SystemState<(
        Local<VecDeque<RetryRequest>>,
        Local<VecDeque<RetryRequest>>,
        ResMut<WorkflowTypeModuleRegistry>,
        ResMut<WorkflowMap>,
        ResMut<WorkflowRequestIOEReceiver>,
        ResMut<WorkflowResponseIOESender>,
    )> = SystemState::new(world);

    let (mut try_now, mut retry_next_frame, workflow_registry, mut workflow_map, mut workflow_request_receiver, workflow_response_sender) =
        system_state.get_mut(world);

    while let Ok(request) = workflow_request_receiver.0.try_recv() {
        let module_name = request.module_name;
        let workflow_name = request.workflow_name;
        let request_id = request.request_id;
        let input = request.input;
        let num_stages = workflow_registry.get_workflow_type(module_name, workflow_name).unwrap().stages.len();

        let response_sender = workflow_response_sender.0.clone();

        let action = Box::new(move |workflow_map: &mut WorkflowMap| {
            workflow_map.insert_workflow(WorkflowInstance::new_request_ioe(
                module_name,
                workflow_name,
                request_id,
                input,
                num_stages,
                Box::new(move |response| {
                    let response: TypedWorkflowResponseOE = response.into_inner();
                    response_sender.send(TypedWorkflowResponseOEEnvelope { request_id, response }).unwrap();
                }),
            ));
        });

        try_now.push_back(RetryRequest {
            module_name,
            workflow_name,
            action,
        });
    }

    while let Some(retry) = try_now.pop_front() {
        if !workflow_map.has_workflow(retry.module_name, retry.workflow_name) {
            (retry.action)(&mut workflow_map);
        } else {
            retry_next_frame.push_back(retry);
        }
    }

    std::mem::swap(&mut retry_next_frame, &mut try_now);
}

pub(super) fn workflow_request_system(world: &mut World) {
    let mut system_state: SystemState<(
        ResMut<WorkflowMap>,
        Res<WorkflowTypeModuleRegistry>,
        ConsumableMessageWriter<StageInitializationMessage>,
    )> = SystemState::new(world);
    let (mut workflow_map, workflow_type_module_registry, _) = system_state.get_mut(world);

    // TODO: MINOR: Duplicate to other relevant places: Rely less on std::mem::take/replace and more on optional resource queries
    let mut stolen_workflow_map = std::mem::take(&mut *workflow_map);

    let mut stage_initialization_messages = Vec::new();

    for (module_name, workflows) in stolen_workflow_map.map.iter_mut() {
        for (workflow_name, instance) in workflows.iter_mut() {
            match instance.state_mut() {
                WorkflowState::Requested => {
                    let current_stage_type = workflow_type_module_registry.get_workflow_type(module_name, workflow_name).unwrap().stages[0].get_type();

                    let input = instance.take_data_buffer();

                    *instance.state_mut() = WorkflowState::Processing {
                        current_stage: 0,
                        current_stage_type,
                        stage_initialized: false,
                        stage_completed: false,
                    };

                    stage_initialization_messages.push(StageInitializationMessage {
                        module_name,
                        workflow_name,
                        stage_input: input,
                    });
                }
                WorkflowState::Processing { .. } => {
                    // Skip this workflow instance, as it is already being processed
                }
            }
        }
    }

    let (mut workflow_map, _, mut stage_initialization_message_writer) = system_state.get_mut(world);

    *workflow_map = stolen_workflow_map;

    for message in stage_initialization_messages {
        stage_initialization_message_writer.send(message);
    }
}

pub(super) fn workflow_initialization_system(world: &mut World) {
    let mut system_state: SystemState<(
        ResMut<WorkflowMap>,
        ResMut<WorkflowTypeModuleRegistry>,
        ResMut<EcsStageBuffer>,
        ResMut<RenderStageBuffer>,
        ResMut<AsyncStageBuffer>,
        ResMut<EcsWhileStageBuffer>,
        ResMut<RenderWhileStageBuffer>,
        ConsumableMessageReader<StageInitializationMessage>,
    )> = SystemState::new(world);
    let (
        mut workflow_map,
        mut workflow_type_module_registry,
        mut ecs_stage_buffer,
        mut render_stage_buffer,
        mut async_stage_buffer,
        mut ecs_while_stage_buffer,
        mut render_while_stage_buffer,
        mut stage_initialization_message_reader,
    ) = system_state.get_mut(world);

    for message in stage_initialization_message_reader.read() {
        let message = message.consume();

        let module_name = message.module_name;
        let workflow_name = message.workflow_name;
        let stage_input = message.stage_input;

        let workflow_instance = if let Some(workflows) = workflow_map.map.get_mut(module_name) {
            if let Some(instance) = workflows.get_mut(workflow_name) {
                instance
            } else {
                unreachable!("Workflow instance not found for module '{}' and name '{}'", module_name, workflow_name);
            }
        } else {
            unreachable!("Workflow instance module not found for name '{}'", module_name);
        };

        let current_state = workflow_instance.state();
        let current_stage = current_state.current_stage();

        let workflow_type = workflow_type_module_registry.get_workflow_type_mut(module_name, workflow_name).unwrap();

        let stage = std::mem::replace(
            &mut workflow_type.stages[current_stage],
            Stage::Ecs(super::stage::StageEcs {
                index: 0,
                name: "placeholder",
                stage_signature: super::stage::StageSignature::None,
                handle_ecs_run_response: Box::new(|_, _, _, _, _| unreachable!()),
                completion_sender: get_stage_completion_sender().clone(),
                failure_sender: None,
            }),
        );

        match stage {
            Stage::Ecs(stage) => {
                ecs_stage_buffer.0.push((module_name, workflow_name, current_stage, stage, stage_input));
            }
            Stage::Render(stage) => {
                render_stage_buffer.0.push((module_name, workflow_name, current_stage, stage, stage_input));
            }
            Stage::Async(stage) => {
                async_stage_buffer.0.push((module_name, workflow_name, current_stage, stage, stage_input));
            }
            Stage::EcsWhile(stage) => {
                ecs_while_stage_buffer.0.push((module_name, workflow_name, current_stage, stage, stage_input));
            }
            Stage::RenderWhile(stage) => {
                render_while_stage_buffer
                    .0
                    .push((module_name, workflow_name, current_stage, stage, stage_input));
            }
        };
    }
}

pub(super) fn workflow_wait_handling_system(world: &mut World) {
    let mut system_state: SystemState<(
        ResMut<WorkflowMap>,
        ResMut<EcsWhileStageBuffer>,
        ResMut<RenderWhileStageBuffer>,
        ConsumableMessageReader<StageWaitMessage>,
    )> = SystemState::new(world);
    let (mut workflow_map, mut ecs_while_stage_buffer, mut render_while_stage_buffer, mut stage_wait_message_reader) = system_state.get_mut(world);

    for message in stage_wait_message_reader.read() {
        let message = message.consume();
        let module_name = message.module_name;
        let workflow_name = message.workflow_name;
        let current_stage = message.current_stage;
        let stage_return = message.stage_return;
        let stage_state = message.stage_state;

        let workflow_instance = if let Some(workflows) = workflow_map.map.get_mut(module_name) {
            if let Some(instance) = workflows.get_mut(workflow_name) {
                instance
            } else {
                unreachable!("Workflow instance not found for module '{}' and name '{}'", module_name, workflow_name);
            }
        } else {
            unreachable!("Workflow instance module not found for name '{}'", module_name);
        };

        let (stage_initialized, stage_completed) = match workflow_instance.state_mut() {
            WorkflowState::Requested => {
                unreachable!(
                    "Workflow wait handling error: Unexpected workflow state. Expected 'WorkflowState::Processing', got '{:?}'",
                    workflow_instance.state()
                )
            }
            WorkflowState::Processing {
                current_stage: _,
                current_stage_type: _,
                stage_initialized,
                stage_completed,
            } => (stage_initialized, stage_completed),
        };

        *stage_initialized = true;
        *stage_completed = false;

        match stage_return {
            Stage::Ecs(_stage) => {
                unreachable!("Workflow wait handling error: Stage type 'Ecs' does not support waiting");
            }
            Stage::Render(_stage) => {
                unreachable!("Workflow wait handling error: Stage type 'Ecs' does not support waiting");
            }
            Stage::Async(_stage) => {
                unreachable!("Workflow wait handling error: Stage type 'Ecs' does not support waiting");
            }
            Stage::EcsWhile(stage) => {
                ecs_while_stage_buffer.0.push((module_name, workflow_name, current_stage, stage, stage_state));
            }
            Stage::RenderWhile(stage) => {
                render_while_stage_buffer
                    .0
                    .push((module_name, workflow_name, current_stage, stage, stage_state));
            }
        };
    }
}

pub(super) fn workflow_completion_handling_system(world: &mut World) {
    let mut system_state: SystemState<(
        ResMut<WorkflowMap>,
        ResMut<WorkflowTypeModuleRegistry>,
        ResMut<EcsStageBuffer>,
        ResMut<RenderStageBuffer>,
        ResMut<AsyncStageBuffer>,
        ResMut<EcsWhileStageBuffer>,
        ResMut<RenderWhileStageBuffer>,
        ConsumableMessageReader<StageCompletionMessage>,
    )> = SystemState::new(world);
    let (
        mut workflow_map,
        mut workflow_type_module_registry,
        mut ecs_stage_buffer,
        mut render_stage_buffer,
        mut async_stage_buffer,
        mut ecs_while_stage_buffer,
        mut render_while_stage_buffer,
        mut stage_completion_message_reader,
    ) = system_state.get_mut(world);

    let mut intermediate_stage_completions = Vec::new();
    let mut final_stage_completions = Vec::new();

    for message in stage_completion_message_reader.read() {
        let message = message.consume();
        let module_name = message.module_name;
        let workflow_name = message.workflow_name;
        let current_stage = message.current_stage;
        let stage_output = message.stage_output;
        let stage = message.stage_return;

        if let Some(workflows) = workflow_map.map.get_mut(module_name) {
            if let Some(instance) = workflows.get_mut(workflow_name) {
                let workflow_type = workflow_type_module_registry.get_workflow_type_mut(module_name, workflow_name).unwrap();
                let stage_count = workflow_type.stages.len();

                match instance.state_mut() {
                    WorkflowState::Processing {
                        current_stage: other_current_stage,
                        current_stage_type: _,
                        stage_completed: completed,
                        ..
                    } => {
                        if current_stage != *other_current_stage {
                            unreachable!(
                                "Unexpected workflow state. Completion message is at stage '{}', but the workflow instance is at stage '{}'",
                                current_stage, other_current_stage
                            );
                        }
                        if *completed {
                            unreachable!(
                                "Unexpected workflow state. Workflow '{}' in module '{}' is already completed.",
                                workflow_name, module_name
                            );
                        }

                        *completed = true;
                    }
                    state => unreachable!("Unexpected workflow state. Expected 'WorkflowState::Processing(_)', got '{:?}'", state),
                }

                if current_stage + 1 < stage_count {
                    intermediate_stage_completions.push((module_name, workflow_name, current_stage, stage_output));
                } else {
                    final_stage_completions.push((module_name, workflow_name, current_stage, instance.take_callback(), stage_output));
                }

                workflow_type.stages[current_stage] = stage;
            }
        }
    }

    // Handle intermediate stage completions
    for (module_name, workflow_name, stage_index, stage_output) in intermediate_stage_completions {
        if let Some(workflows) = workflow_map.map.get_mut(module_name) {
            if let Some(instance) = workflows.get_mut(workflow_name) {
                let workflow_type = workflow_type_module_registry.get_workflow_type_mut(module_name, workflow_name).unwrap();

                let new_stage_index = stage_index + 1;
                let new_stage = std::mem::replace(
                    &mut workflow_type.stages[new_stage_index],
                    Stage::Ecs(super::stage::StageEcs {
                        index: 0,
                        name: "placeholder",
                        stage_signature: super::stage::StageSignature::None,
                        handle_ecs_run_response: Box::new(|_, _, _, _, _| unreachable!()),
                        completion_sender: get_stage_completion_sender().clone(),
                        failure_sender: None,
                    }),
                );
                let new_stage_type = new_stage.get_type();

                *instance.state_mut() = WorkflowState::Processing {
                    current_stage: new_stage_index,
                    current_stage_type: new_stage_type,
                    stage_initialized: false,
                    stage_completed: false,
                };

                match new_stage {
                    Stage::Ecs(stage) => {
                        ecs_stage_buffer.0.push((module_name, workflow_name, new_stage_index, stage, stage_output));
                    }
                    Stage::Render(stage) => {
                        render_stage_buffer.0.push((module_name, workflow_name, new_stage_index, stage, stage_output));
                    }
                    Stage::Async(stage) => {
                        async_stage_buffer.0.push((module_name, workflow_name, new_stage_index, stage, stage_output));
                    }
                    Stage::EcsWhile(stage) => {
                        ecs_while_stage_buffer
                            .0
                            .push((module_name, workflow_name, new_stage_index, stage, stage_output));
                    }
                    Stage::RenderWhile(stage) => {
                        render_while_stage_buffer
                            .0
                            .push((module_name, workflow_name, new_stage_index, stage, stage_output));
                    }
                };
            }
        }
    }

    // Handle final stage completions
    for (module_name, workflow_name, _current_stage, callback, output) in final_stage_completions {
        if let Some(workflows) = workflow_map.map.get_mut(module_name) {
            workflows.remove(workflow_name);

            match callback {
                WorkflowCallback::None(callback) => match output {
                    Some(_output) => {
                        let response = TypedWorkflowResponse { module_name, workflow_name };
                        let boxed_response = AnySendSyncPremiumBox::new(response, format!("{module_name}::{workflow_name}::TypedResponse"));
                        callback(boxed_response);
                    }
                    None => {
                        unreachable!("Workflow success callback error: Expected Some(output), but got None.")
                    }
                },

                WorkflowCallback::E(callback) => match output {
                    Some(_output) => {
                        let response = TypedWorkflowResponseE {
                            module_name,
                            workflow_name,
                            result: Ok(()),
                        };
                        let boxed_response = AnySendSyncPremiumBox::new(response, format!("{module_name}::{workflow_name}::TypedResponse-[E]"));
                        callback(boxed_response);
                    }
                    None => {
                        unreachable!("Workflow success callback error: Expected Some(output), but got None.")
                    }
                },

                WorkflowCallback::O(callback) => match output {
                    Some(output) => {
                        let response = TypedWorkflowResponseO {
                            module_name,
                            workflow_name,
                            output,
                        };
                        let boxed_response = AnySendSyncPremiumBox::new(response, format!("{module_name}::{workflow_name}::TypedResponse-O"));
                        callback(boxed_response);
                    }
                    None => {
                        unreachable!("Workflow success callback error: Expected Some(output), but got None.")
                    }
                },

                WorkflowCallback::OE(callback) => match output {
                    Some(output) => {
                        let response = TypedWorkflowResponseOE {
                            module_name,
                            workflow_name,
                            result: Ok(output),
                        };
                        let boxed_response = AnySendSyncPremiumBox::new(response, format!("{module_name}::{workflow_name}::TypedResponse-O[E]"));
                        callback(boxed_response);
                    }
                    None => {
                        unreachable!("Workflow success callback error: Expected Some(output), but got None.")
                    }
                },

                WorkflowCallback::I(callback) => match output {
                    Some(_output) => {
                        let response = TypedWorkflowResponse { module_name, workflow_name };
                        let boxed_response = AnySendSyncPremiumBox::new(response, format!("{module_name}::{workflow_name}::TypedResponse-(I)"));
                        callback(boxed_response);
                    }
                    None => {
                        unreachable!("Workflow success callback error: Expected Some(output), but got None.")
                    }
                },

                WorkflowCallback::IE(callback) => match output {
                    Some(_output) => {
                        let response = TypedWorkflowResponseE {
                            module_name,
                            workflow_name,
                            result: Ok(()),
                        };
                        let boxed_response = AnySendSyncPremiumBox::new(response, format!("{module_name}::{workflow_name}::TypedResponse-(I)[E]"));
                        callback(boxed_response);
                    }
                    None => {
                        unreachable!("Workflow success callback error: Expected Some(output), but got None.")
                    }
                },

                WorkflowCallback::IO(callback) => match output {
                    Some(output) => {
                        let response = TypedWorkflowResponseO {
                            module_name,
                            workflow_name,
                            output,
                        };
                        let boxed_response = AnySendSyncPremiumBox::new(response, format!("{module_name}::{workflow_name}::TypedResponse-(I)O"));
                        callback(boxed_response);
                    }
                    None => {
                        unreachable!("Workflow success callback error: Expected Some(output), but got None.")
                    }
                },

                WorkflowCallback::IOE(callback) => match output {
                    Some(output) => {
                        let response = TypedWorkflowResponseOE {
                            module_name,
                            workflow_name,
                            result: Ok(output),
                        };
                        let boxed_response = AnySendSyncPremiumBox::new(response, format!("{module_name}::{workflow_name}::TypedResponse-(I)O[E]"));
                        callback(boxed_response);
                    }
                    None => {
                        unreachable!("Workflow success callback error: Expected Some(output), but got None.")
                    }
                },
            };
        }
    }
}

pub(super) fn workflow_failure_handling_system(world: &mut World) {
    let mut system_state: SystemState<(
        ResMut<WorkflowMap>,
        ResMut<WorkflowTypeModuleRegistry>,
        ConsumableMessageReader<StageFailureMessage>,
    )> = SystemState::new(world);
    let (mut workflow_map, mut workflow_type_module_registry, mut stage_failure_message_reader) = system_state.get_mut(world);

    let mut stage_failures = Vec::new();

    for message in stage_failure_message_reader.read() {
        let message = message.consume();
        let module_name = message.module_name;
        let workflow_name = message.workflow_name;
        let current_stage = message.current_stage;
        let stage_error = message.stage_error;
        let stage = message.stage_return;

        let workflow_type = workflow_type_module_registry.get_workflow_type_mut(module_name, workflow_name).unwrap();

        stage_failures.push((module_name, workflow_name, stage_error));

        workflow_type.stages[current_stage] = stage;
    }

    for (module_name, workflow_name, error) in stage_failures {
        if let Some(workflows) = workflow_map.map.get_mut(module_name) {
            if let Some(instance) = workflows.get_mut(workflow_name) {
                let callback = instance.take_callback();

                match callback {
                    WorkflowCallback::None(_callback) => {
                        unreachable!("Workflow failure callback error: Stage type 'None' does not support failure handling");
                    }

                    WorkflowCallback::E(callback) => match error {
                        Some(error) => {
                            let response = TypedWorkflowResponseE {
                                module_name,
                                workflow_name,
                                result: Err(error),
                            };
                            let boxed_response = AnySendSyncPremiumBox::new(response, format!("{module_name}::{workflow_name}::TypedResponse-E"));
                            callback(boxed_response);
                        }
                        None => {
                            unreachable!("Workflow failure callback error: Expected Some(error), but got None.")
                        }
                    },

                    WorkflowCallback::O(_callback) => {
                        unreachable!("Workflow failure callback error: Stage type 'O' does not support failure handling");
                    }

                    WorkflowCallback::OE(callback) => match error {
                        Some(error) => {
                            let response = TypedWorkflowResponseOE {
                                module_name,
                                workflow_name,
                                result: Err(error),
                            };
                            let boxed_response = AnySendSyncPremiumBox::new(response, format!("{module_name}::{workflow_name}::TypedResponse-[O]E"));
                            callback(boxed_response);
                        }
                        None => {
                            unreachable!("Workflow failure callback error: Expected Some(error), but got None.")
                        }
                    },

                    WorkflowCallback::I(_callback) => {
                        unreachable!("Workflow failure callback error: Stage type 'I' does not support failure handling");
                    }

                    WorkflowCallback::IE(callback) => match error {
                        Some(error) => {
                            let response = TypedWorkflowResponseE {
                                module_name,
                                workflow_name,
                                result: Err(error),
                            };
                            let boxed_response = AnySendSyncPremiumBox::new(response, format!("{module_name}::{workflow_name}::TypedResponse-(I)E"));
                            callback(boxed_response);
                        }
                        None => {
                            unreachable!("Workflow failure callback error: Expected Some(error), but got None.")
                        }
                    },

                    WorkflowCallback::IO(_callback) => {
                        unreachable!("Workflow failure callback error: Stage type 'IO' does not support failure handling");
                    }

                    WorkflowCallback::IOE(callback) => match error {
                        Some(error) => {
                            let response = TypedWorkflowResponseOE {
                                module_name,
                                workflow_name,
                                result: Err(error),
                            };
                            let boxed_response = AnySendSyncPremiumBox::new(response, format!("{module_name}::{workflow_name}::TypedResponse-(I)[O]E"));
                            callback(boxed_response);
                        }
                        None => {
                            unreachable!("Workflow failure callback error: Expected Some(error), but got None.")
                        }
                    },
                };
            }

            workflows.remove(workflow_name);
        }
    }
}
