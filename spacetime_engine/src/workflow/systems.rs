use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use bevy::render::MainWorld;
use bevy_consumable_event::{ConsumableEventReader, ConsumableEventWriter};

use crate::statics::TOKIO_RUNTIME;

use super::{channels::*, events::*, instance::*, resources::*, stage::Stage, types::*};

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
    let render_while_workflow_state_extract: RenderWhileWorkflowStateExtract =
        main_world.resource::<WorkflowMap>().into();

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

pub(super) fn poll_ecs_stage_buffer_system(world: &mut World) {
    let drained_buffer = {
        let mut buffer = world.resource_mut::<EcsStageBuffer>();
        std::mem::take(&mut buffer.0)
    };

    let completion_sender = get_stage_completion_sender();
    let failure_sender = get_stage_failure_sender();

    for (module_name, workflow_name, current_stage, mut stage, data_buffer) in drained_buffer {
        let run_ecs = &mut stage.run_ecs;
        let handle_ecs_response = &mut stage.handle_ecs_response;
        let completion_sender = completion_sender.clone();
        let failure_sender = if stage.signature.has_error() {
            Some(failure_sender.clone())
        } else {
            None
        };

        let input = data_buffer;
        let response = (run_ecs)(input, world);
        let handler = (handle_ecs_response)(
            module_name,
            workflow_name,
            response,
            completion_sender,
            failure_sender,
        );
        handler(stage);
    }
}
pub(super) fn poll_render_stage_buffer_system(world: &mut World) {
    let drained_buffer = {
        let mut buffer = world.resource_mut::<RenderStageBuffer>();
        std::mem::take(&mut buffer.0)
    };

    let completion_sender = get_stage_completion_sender();
    let failure_sender = get_stage_failure_sender();

    for (module_name, workflow_name, current_stage, mut stage, data_buffer) in drained_buffer {
        let run_render = &mut stage.run_render;
        let handle_render_response = &mut stage.handle_render_response;
        let completion_sender = completion_sender.clone();
        let failure_sender = if stage.signature.has_error() {
            Some(failure_sender.clone())
        } else {
            None
        };

        let input = data_buffer;
        let response = (run_render)(input, world);
        let handler = (handle_render_response)(
            module_name,
            workflow_name,
            response,
            completion_sender,
            failure_sender,
        );
        handler(stage);
    }
}
pub(super) fn poll_async_stage_buffer_system(world: &mut World) {
    let drained_buffer = {
        let mut buffer = world.resource_mut::<AsyncStageBuffer>();
        std::mem::take(&mut buffer.0)
    };

    let completion_sender = get_stage_completion_sender();
    let failure_sender = get_stage_failure_sender();

    for (module_name, workflow_name, current_stage, mut stage, data_buffer) in drained_buffer {
        let run_async = &mut stage.run_async;
        let completion_sender = completion_sender.clone();
        let failure_sender = if stage.signature.has_error() {
            Some(failure_sender.clone())
        } else {
            None
        };
        
        let input = data_buffer;
        let response_future = (run_async)(input);
        if let Err(err) = TOKIO_RUNTIME.lock().unwrap().block_on(async move {
            tokio::spawn(async move {
                let response = response_future.await;
                let handler = (stage.handle_async_response)(
                    module_name,
                    workflow_name,
                    response,
                    completion_sender,
                    failure_sender,
                );
                handler(stage);
            })
            .await
        }) {
            unreachable!("Async stage execution error: Task spawn error: {}", err);
        }
    }
}
pub(super) fn poll_ecs_while_stage_buffer_system(world: &mut World) {
    let drained_buffer = {
        let mut buffer = world.resource_mut::<EcsWhileStageBuffer>();
        std::mem::take(&mut buffer.0)
    };

    let wait_sender = get_stage_wait_sender();
    let completion_sender = get_stage_completion_sender();
    let failure_sender = get_stage_failure_sender();

    for (module_name, workflow_name, current_stage, mut stage, data_buffer) in drained_buffer {
        let run_ecs_while = &mut stage.run_ecs_while;
        let handle_ecs_while_response = &mut stage.handle_ecs_while_response;
        let completion_sender = completion_sender.clone();
        let failure_sender = if stage.signature.has_error() {
            Some(failure_sender.clone())
        } else {
            None
        };

        let state = {
            let mut workflow_map = SystemState::<ResMut<WorkflowMap>>::new(world).get_mut(world);
            let workflow_instance = workflow_map
                .map
                .get_mut(module_name)
                .and_then(|workflows| workflows.get_mut(workflow_name))
                .unwrap();

            if let WorkflowState::Processing {
                current_stage: _,
                current_stage_type: _,
                stage_initialized,
                stage_completed: _,
            } = &mut workflow_instance.state() {
                if !*stage_initialized {
                    let setup_ecs_while = &mut stage.setup_ecs_while;

                    let input = data_buffer;
                    let state = (setup_ecs_while)(input, world);

                    *stage_initialized = true;

                    state
                } else {
                    data_buffer
                }
            } else {
                unreachable!(
                    "Unexpected workflow state. Expected 'WorkflowState::Processing', got '{:?}'",
                    workflow_instance.state()
                );
            }
        };
        let response = (run_ecs_while)(state, world);
        let handler = (handle_ecs_while_response)(
            module_name,
            workflow_name,
            Some(response),
            wait_sender.clone(),
            completion_sender,
            failure_sender,
        );
        handler(stage);
    }
}
pub(super) fn poll_render_while_stage_buffer_system(world: &mut World) {
    let drained_buffer = {
        let mut buffer = world.resource_mut::<RenderWhileStageBuffer>();
        std::mem::take(&mut buffer.0)
    };

    let extract_reintegration_sender = world
        .get_resource::<RenderWhileWorkflowStateExtractReintegrationEventSender>()
        .unwrap()
        .0
        .clone();

    let wait_sender = get_stage_wait_sender();
    let completion_sender = get_stage_completion_sender();
    let failure_sender = get_stage_failure_sender();

    let mut remaining_buffer = Vec::new();

    for (module_name, workflow_name, current_stage, mut stage, data_buffer) in drained_buffer {
        let run_render_while = &mut stage.run_render_while;
        let handle_render_while_response = &mut stage.handle_render_while_response;
        let wait_sender = wait_sender.clone();
        let completion_sender = completion_sender.clone();
        let failure_sender = if stage.signature.has_error() {
            Some(failure_sender.clone())
        } else {
            None
        };

        let state = {
            let render_workflow_state_extract =
                SystemState::<ResMut<RenderWhileWorkflowStateExtract>>::new(world).get_mut(world);
            let stage_initialized = &mut render_workflow_state_extract
                .0
                .iter()
                .find(|(m, w, _, _)| m == &module_name && w == &workflow_name)
                .map(|(_, _, _, s)| *s);
    
            let stage_initialized = match stage_initialized {
                Some(stage_initialized) => stage_initialized,
                None => {
                    warn!("Render while stage buffer system error: Stage initialized state not found for workflow '{}', module '{}'", workflow_name, module_name);
                    remaining_buffer.push((
                        module_name,
                        workflow_name,
                        current_stage,
                        stage,
                        data_buffer,
                    ));
                    continue;
                }
            };

            if !*stage_initialized {
                let setup_render_while = &mut stage.setup_render_while;
                
                let input = data_buffer;
                let state = (setup_render_while)(input, world);

                *stage_initialized = true;

                extract_reintegration_sender
                    .send((module_name, workflow_name))
                    .unwrap();

                state
            } else {
                data_buffer
            }
        };
        let response = (run_render_while)(state, world);
        let handler = (handle_render_while_response)(
            module_name,
            workflow_name,
            Some(response),
            wait_sender.clone(),
            completion_sender,
            failure_sender,
        );
        handler(stage);
    }
}

pub(super) fn render_while_workflow_state_extract_reintegration_system(world: &mut World) {
    let mut system_state: SystemState<(
        ResMut<WorkflowMap>,
        ResMut<RenderWhileWorkflowStateExtractReintegrationEventReceiver>,
    )> = SystemState::new(world);
    let (mut workflow_map, render_while_workflow_state_extract_reintegration_event_receiver) =
        system_state.get_mut(world);

    while let Ok(event) = render_while_workflow_state_extract_reintegration_event_receiver
        .0
        .try_recv()
    {
        let module_name = event.0;
        let workflow_name = event.1;

        if let Some(workflow_instance) = workflow_map.get_workflow_mut(module_name, workflow_name) {
            if let WorkflowState::Processing {
                current_stage: _,
                current_stage_type: _,
                ref mut stage_initialized,
                stage_completed: _,
            } = workflow_instance.state()
            {
                *stage_initialized = true;
            } else {
                unreachable!("Render while workflow state reintegration error: Unexpected workflow state. Expected 'WorkflowState::Processing', got '{:?}'", workflow_instance.state());
            }
        } else {
            unreachable!("Render while workflow state reintegration error: Workflow '{}' in module '{}' not found.", workflow_name, module_name);
        }
    }
}

pub(super) fn stage_wait_relay_system(
    stage_event_receiver: Res<StageWaitEventReceiver>,
    mut stage_event_writer: ConsumableEventWriter<StageWaitEvent>,
) {
    while let Ok(StageWaitEvent {
        ty,
        module_name,
        workflow_name,
        current_stage,
        stage_return,
        stage_state,
    }) = stage_event_receiver.0.try_recv()
    {
        stage_event_writer.send(StageWaitEvent {
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
    stage_event_receiver: Res<StageCompletionEventReceiver>,
    mut stage_event_writer: ConsumableEventWriter<StageCompletionEvent>,
) {
    while let Ok(StageCompletionEvent {
        ty,
        module_name,
        workflow_name,
        current_stage,
        stage_output,
        stage_return,
    }) = stage_event_receiver.0.try_recv()
    {
        stage_event_writer.send(StageCompletionEvent {
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
    stage_event_receiver: Res<StageFailureEventReceiver>,
    mut stage_event_writer: ConsumableEventWriter<StageFailureEvent>,
) {
    while let Ok(StageFailureEvent {
        ty,
        module_name,
        workflow_name,
        current_stage,
        stage_error,
        stage_return,
    }) = stage_event_receiver.0.try_recv()
    {
        stage_event_writer.send(StageFailureEvent {
            ty,
            module_name,
            workflow_name,
            current_stage,
            stage_error,
            stage_return,
        });
    }
}

pub(super) fn workflow_request_relay_system(world: &mut World) {
    let mut system_state: SystemState<(
        ResMut<WorkflowTypeModuleRegistry>,
        ResMut<WorkflowMap>,
        ResMut<WorkflowRequestReceiver>,
        ResMut<WorkflowResponseSender>,
    )> = SystemState::new(world);
    let (
        workflow_registry,
        mut workflow_map,
        mut workflow_request_receiver,
        workflow_response_sender,
    ) = system_state.get_mut(world);

    while let Ok(request) = workflow_request_receiver.0.try_recv() {
        let module_name = request.module_name;
        let workflow_name = request.workflow_name;

        if workflow_map.has_workflow(module_name, workflow_name) {
            unreachable!(
                "Workflow request error: Workflow '{}' in module '{}' is already active.",
                workflow_name, module_name
            );
        }

        let workflow_type = workflow_registry
            .get_workflow_type(module_name, workflow_name)
            .unwrap();
        let num_stages = workflow_type.stages.len();
        let workflow_response_sender = workflow_response_sender.0.clone();

        workflow_map.insert_workflow(WorkflowInstance::new_request(
            module_name,
            workflow_name,
            num_stages,
            Box::new(move || {
                workflow_response_sender.send(()).unwrap();
            }),
        ));
    }
}
pub(super) fn workflow_request_e_relay_system(world: &mut World) {
    let mut system_state: SystemState<(
        ResMut<WorkflowTypeModuleRegistry>,
        ResMut<WorkflowMap>,
        ResMut<WorkflowRequestEReceiver>,
        ResMut<WorkflowResponseESender>,
    )> = SystemState::new(world);
    let (
        workflow_registry,
        mut workflow_map,
        mut workflow_request_receiver,
        workflow_response_sender,
    ) = system_state.get_mut(world);

    while let Ok(request) = workflow_request_receiver.0.try_recv() {
        let module_name = request.module_name;
        let workflow_name = request.workflow_name;

        if workflow_map.has_workflow(module_name, workflow_name) {
            unreachable!(
                "Workflow request error: Workflow '{}' in module '{}' is already active.",
                workflow_name, module_name
            );
        }

        let workflow_type = workflow_registry
            .get_workflow_type(module_name, workflow_name)
            .unwrap();
        let num_stages = workflow_type.stages.len();
        let workflow_response_sender = workflow_response_sender.0.clone();

        workflow_map.insert_workflow(WorkflowInstance::new_request_e(
            module_name,
            workflow_name,
            num_stages,
            Box::new(move |response| {
                let response = response.downcast().unwrap();
                workflow_response_sender.send(*response).unwrap();
            }),
        ));
    }
}
pub(super) fn workflow_request_o_relay_system(world: &mut World) {
    let mut system_state: SystemState<(
        ResMut<WorkflowTypeModuleRegistry>,
        ResMut<WorkflowMap>,
        ResMut<WorkflowRequestOReceiver>,
        ResMut<WorkflowResponseOSender>,
    )> = SystemState::new(world);
    let (
        workflow_registry,
        mut workflow_map,
        mut workflow_request_receiver,
        workflow_response_sender,
    ) = system_state.get_mut(world);

    while let Ok(request) = workflow_request_receiver.0.try_recv() {
        let module_name = request.module_name;
        let workflow_name = request.workflow_name;

        if workflow_map.has_workflow(module_name, workflow_name) {
            unreachable!(
                "Workflow request error: Workflow '{}' in module '{}' is already active.",
                workflow_name, module_name
            );
        }

        let workflow_type = workflow_registry
            .get_workflow_type(module_name, workflow_name)
            .unwrap();
        let num_stages = workflow_type.stages.len();
        let workflow_response_sender = workflow_response_sender.0.clone();

        workflow_map.insert_workflow(WorkflowInstance::new_request_o(
            module_name,
            workflow_name,
            num_stages,
            Box::new(move |response| {
                let response = response.downcast().unwrap();
                workflow_response_sender.send(*response).unwrap();
            }),
        ));
    }
}
pub(super) fn workflow_request_oe_relay_system(world: &mut World) {
    let mut system_state: SystemState<(
        ResMut<WorkflowTypeModuleRegistry>,
        ResMut<WorkflowMap>,
        ResMut<WorkflowRequestOEReceiver>,
        ResMut<WorkflowResponseOESender>,
    )> = SystemState::new(world);
    let (
        workflow_registry,
        mut workflow_map,
        mut workflow_request_receiver,
        workflow_response_sender,
    ) = system_state.get_mut(world);

    while let Ok(request) = workflow_request_receiver.0.try_recv() {
        let module_name = request.module_name;
        let workflow_name = request.workflow_name;

        if workflow_map.has_workflow(module_name, workflow_name) {
            unreachable!(
                "Workflow request error: Workflow '{}' in module '{}' is already active.",
                workflow_name, module_name
            );
        }

        let workflow_type = workflow_registry
            .get_workflow_type(module_name, workflow_name)
            .unwrap();
        let num_stages = workflow_type.stages.len();
        let workflow_response_sender = workflow_response_sender.0.clone();

        workflow_map.insert_workflow(WorkflowInstance::new_request_oe(
            module_name,
            workflow_name,
            num_stages,
            Box::new(move |response| {
                let response = response.downcast().unwrap();
                workflow_response_sender.send(*response).unwrap();
            }),
        ));
    }
}
pub(super) fn workflow_request_i_relay_system(world: &mut World) {
    let mut system_state: SystemState<(
        ResMut<WorkflowTypeModuleRegistry>,
        ResMut<WorkflowMap>,
        ResMut<WorkflowRequestIReceiver>,
        ResMut<WorkflowResponseISender>,
    )> = SystemState::new(world);
    let (
        workflow_registry,
        mut workflow_map,
        mut workflow_request_receiver,
        workflow_response_sender,
    ) = system_state.get_mut(world);

    while let Ok(request) = workflow_request_receiver.0.try_recv() {
        let module_name = request.module_name;
        let workflow_name = request.workflow_name;

        if workflow_map.has_workflow(module_name, workflow_name) {
            unreachable!(
                "Workflow request error: Workflow '{}' in module '{}' is already active.",
                workflow_name, module_name
            );
        }

        let workflow_type = workflow_registry
            .get_workflow_type(module_name, workflow_name)
            .unwrap();
        let num_stages = workflow_type.stages.len();
        let workflow_response_sender = workflow_response_sender.0.clone();

        workflow_map.insert_workflow(WorkflowInstance::new_request_i(
            module_name,
            workflow_name,
            request.input,
            num_stages,
            Box::new(move || {
                workflow_response_sender.send(()).unwrap();
            }),
        ));
    }
}
pub(super) fn workflow_request_ie_relay_system(world: &mut World) {
    let mut system_state: SystemState<(
        ResMut<WorkflowTypeModuleRegistry>,
        ResMut<WorkflowMap>,
        ResMut<WorkflowRequestIEReceiver>,
        ResMut<WorkflowResponseIESender>,
    )> = SystemState::new(world);
    let (
        workflow_registry,
        mut workflow_map,
        mut workflow_request_receiver,
        workflow_response_sender,
    ) = system_state.get_mut(world);

    while let Ok(request) = workflow_request_receiver.0.try_recv() {
        let module_name = request.module_name;
        let workflow_name = request.workflow_name;

        if workflow_map.has_workflow(module_name, workflow_name) {
            unreachable!(
                "Workflow request error: Workflow '{}' in module '{}' is already active.",
                workflow_name, module_name
            );
        }

        let workflow_type = workflow_registry
            .get_workflow_type(module_name, workflow_name)
            .unwrap();
        let num_stages = workflow_type.stages.len();
        let workflow_response_sender = workflow_response_sender.0.clone();

        workflow_map.insert_workflow(WorkflowInstance::new_request_ie(
            module_name,
            workflow_name,
            request.input,
            num_stages,
            Box::new(move |response| {
                let response = response.downcast().unwrap();
                workflow_response_sender.send(*response).unwrap();
            }),
        ));
    }
}
pub(super) fn workflow_request_io_relay_system(world: &mut World) {
    let mut system_state: SystemState<(
        ResMut<WorkflowTypeModuleRegistry>,
        ResMut<WorkflowMap>,
        ResMut<WorkflowRequestIOReceiver>,
        ResMut<WorkflowResponseIOSender>,
    )> = SystemState::new(world);
    let (
        workflow_registry,
        mut workflow_map,
        mut workflow_request_receiver,
        workflow_response_sender,
    ) = system_state.get_mut(world);

    while let Ok(request) = workflow_request_receiver.0.try_recv() {
        let module_name = request.module_name;
        let workflow_name = request.workflow_name;

        if workflow_map.has_workflow(module_name, workflow_name) {
            unreachable!(
                "Workflow request error: Workflow '{}' in module '{}' is already active.",
                workflow_name, module_name
            );
        }

        let workflow_type = workflow_registry
            .get_workflow_type(module_name, workflow_name)
            .unwrap();
        let num_stages = workflow_type.stages.len();
        let workflow_response_sender = workflow_response_sender.0.clone();

        workflow_map.insert_workflow(WorkflowInstance::new_request_io(
            module_name,
            workflow_name,
            request.input,
            num_stages,
            Box::new(move |response| {
                let response = response.downcast().unwrap();
                workflow_response_sender.send(*response).unwrap();
            }),
        ));
    }
}
pub(super) fn workflow_request_ioe_relay_system(world: &mut World) {
    let mut system_state: SystemState<(
        ResMut<WorkflowTypeModuleRegistry>,
        ResMut<WorkflowMap>,
        ResMut<WorkflowRequestIOEReceiver>,
        ResMut<WorkflowResponseIOESender>,
    )> = SystemState::new(world);
    let (
        workflow_registry,
        mut workflow_map,
        mut workflow_request_receiver,
        workflow_response_sender,
    ) = system_state.get_mut(world);

    while let Ok(request) = workflow_request_receiver.0.try_recv() {
        let module_name = request.module_name;
        let workflow_name = request.workflow_name;

        if workflow_map.has_workflow(module_name, workflow_name) {
            unreachable!(
                "Workflow request error: Workflow '{}' in module '{}' is already active.",
                workflow_name, module_name
            );
        }

        let workflow_type = workflow_registry
            .get_workflow_type(module_name, workflow_name)
            .unwrap();
        let num_stages = workflow_type.stages.len();
        let workflow_response_sender = workflow_response_sender.0.clone();

        workflow_map.insert_workflow(WorkflowInstance::new_request_ioe(
            module_name,
            workflow_name,
            request.input,
            num_stages,
            Box::new(move |response| {
                let response = response.downcast().unwrap();
                workflow_response_sender.send(*response).unwrap();
            }),
        ));
    }
}

pub(super) fn workflow_request_system(world: &mut World) {
    let mut system_state: SystemState<(
        ResMut<WorkflowMap>,
        Res<WorkflowTypeModuleRegistry>,
        ConsumableEventWriter<StageInitializationEvent>,
    )> = SystemState::new(world);
    let (mut workflow_map, workflow_type_module_registry, _) = system_state.get_mut(world);

    // TODO: Minor: Duplicate to other relevant places: Rely less on std::mem::take/replace and more on optional resource queries
    let mut stolen_workflow_map = std::mem::take(&mut *workflow_map);

    let mut stage_initialization_events = Vec::new();

    for (module_name, workflows) in stolen_workflow_map.map.iter_mut() {
        for (workflow_name, instance) in workflows.iter_mut() {
            match instance.state_mut() {
                WorkflowState::Requested => {
                    let current_stage_type = workflow_type_module_registry
                        .get_workflow_type(module_name, workflow_name)
                        .unwrap()
                        .stages[0]
                        .get_type();

                    let input = instance.take_data_buffer();

                    *instance.state_mut() = WorkflowState::Processing {
                        current_stage: 0,
                        current_stage_type,
                        stage_initialized: false,
                        stage_completed: false,
                    };

                    stage_initialization_events.push(StageInitializationEvent {
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

    let (mut workflow_map, _, mut stage_initialization_event_writer) = system_state.get_mut(world);

    *workflow_map = stolen_workflow_map;

    for event in stage_initialization_events {
        stage_initialization_event_writer.send(event);
    }
}

pub(super) fn workflow_execution_system(world: &mut World) {
    let mut system_state: SystemState<(
        ResMut<WorkflowMap>,
        ResMut<WorkflowTypeModuleRegistry>,
        ResMut<EcsStageBuffer>,
        ResMut<RenderStageBuffer>,
        ResMut<AsyncStageBuffer>,
        ResMut<EcsWhileStageBuffer>,
        ResMut<RenderWhileStageBuffer>,
        ConsumableEventReader<StageInitializationEvent>,
    )> = SystemState::new(world);
    let (
        mut workflow_map,
        mut workflow_type_module_registry,
        mut ecs_stage_buffer,
        mut render_stage_buffer,
        mut async_stage_buffer,
        mut ecs_while_stage_buffer,
        mut render_while_stage_buffer,
        mut stage_initialization_event_reader,
    ) = system_state.get_mut(world);

    for event in stage_initialization_event_reader.read() {
        let event = event.consume();

        let module_name = event.module_name;
        let workflow_name = event.workflow_name;
        let stage_input = event.stage_input;

        let workflow_instance = if let Some(workflows) = workflow_map.map.get_mut(module_name) {
            if let Some(instance) = workflows.get_mut(workflow_name) {
                instance
            } else {
                unreachable!(
                    "Workflow instance not found for module '{}' and name '{}'",
                    module_name, workflow_name
                );
            }
        } else {
            unreachable!(
                "Workflow instance module not found for name '{}'",
                module_name
            );
        };
        
        let current_state = workflow_instance.state();
        let current_stage = current_state.current_stage();

        let workflow_type = workflow_type_module_registry
            .get_workflow_type_mut(module_name, workflow_name)
            .unwrap();

        let stage = std::mem::replace(
            &mut workflow_type.stages[current_stage],
            Stage::Ecs(super::stage::StageEcs {
                index: 0,
                name: "placeholder",
                signature: super::stage::StageSignature::None,
                run_ecs: Box::new(|_, _| unreachable!()),
                handle_ecs_response: Box::new(|_, _, _, _, _| unreachable!()),
                completion_sender: get_stage_completion_sender().clone(),
                failure_sender: None,
            }),
        );

        match stage {
            Stage::Ecs(stage) => {
                ecs_stage_buffer.0.push((
                    module_name,
                    workflow_name,
                    current_stage,
                    stage,
                    stage_input,
                ));
            }
            Stage::Render(stage) => {
                render_stage_buffer.0.push((
                    module_name,
                    workflow_name,
                    current_stage,
                    stage,
                    stage_input,
                ));
            }
            Stage::Async(stage) => {
                async_stage_buffer.0.push((
                    module_name,
                    workflow_name,
                    current_stage,
                    stage,
                    stage_input,
                ));
            }
            Stage::EcsWhile(stage) => {
                ecs_while_stage_buffer.0.push((
                    module_name,
                    workflow_name,
                    current_stage,
                    stage,
                    stage_input,
                ));
            }
            Stage::RenderWhile(stage) => {
                render_while_stage_buffer.0.push((
                    module_name,
                    workflow_name,
                    current_stage,
                    stage,
                    stage_input,
                ));
            }
        }
    }
}

// TODO: MAJOR: Touch this system with a lange Zange and a Gefahrenschutzanzug
pub(super) fn workflow_wait_handling_system(world: &mut World) {}

pub(super) fn workflow_completion_handling_system(world: &mut World) {
    let mut system_state: SystemState<(
        ResMut<WorkflowMap>,
        ResMut<WorkflowTypeModuleRegistry>,
        ConsumableEventReader<StageCompletionEvent>,
        ConsumableEventWriter<StageInitializationEvent>,
    )> = SystemState::new(world);
    let (
        mut workflow_map,
        mut workflow_type_module_registry,
        mut stage_completion_event_reader,
        mut stage_initialization_event_writer,
    ) = system_state.get_mut(world);

    let mut intermediate_stage_completions = Vec::new();
    let mut final_stage_completions = Vec::new();

    for event in stage_completion_event_reader.read() {
        let event = event.consume();
        let module_name = event.module_name;
        let workflow_name = event.workflow_name;
        let current_stage = event.current_stage;
        let stage_output = event.stage_output;
        let stage = event.stage_return;

        if let Some(workflows) = workflow_map.map.get_mut(module_name) {
            if let Some(instance) = workflows.get_mut(workflow_name) {
                let workflow_type = workflow_type_module_registry
                    .get_workflow_type_mut(module_name, workflow_name)
                    .unwrap();
                let stage_count = workflow_type.stages.len();

                let current_stage_type = match instance.state_mut() {
                    WorkflowState::Processing { current_stage: other_current_stage, current_stage_type, stage_completed: completed, .. } => {
                        if current_stage != *other_current_stage {
                            unreachable!("Unexpected workflow state. Completion event is at stage '{}', but the workflow instance is at stage '{}'", current_stage, other_current_stage);
                        }
                        if *completed {
                            unreachable!("Unexpected workflow state. Workflow '{}' in module '{}' is already completed.", workflow_name, module_name);
                        }

                        *completed = true;

                        *current_stage_type
                    },
                    state => unreachable!("Unexpected workflow state. Expected 'WorkflowState::Processing(_)', got '{:?}'", state),
                };

                if current_stage + 1 < stage_count {
                    intermediate_stage_completions.push((
                        module_name,
                        workflow_name,
                        current_stage,
                        current_stage_type,
                        stage_output
                    ));
                } else {
                    final_stage_completions.push((
                        module_name,
                        workflow_name,
                        instance.take_callback(),
                        stage_output
                    ));
                }

                workflow_type.stages[current_stage] = stage;
            }
        }
    }

    // Handle intermediate stage completions
    for (module_name, workflow_name, current_stage, current_stage_type, stage_output) in
        intermediate_stage_completions
    {
        if let Some(workflows) = workflow_map.map.get_mut(module_name) {
            if let Some(instance) = workflows.get_mut(workflow_name) {
                let new_stage_type = workflow_type_module_registry
                    .get_workflow_type(module_name, workflow_name)
                    .unwrap()
                    .stages[current_stage + 1]
                    .get_type();

                *instance.state_mut() = WorkflowState::Processing {
                    current_stage: current_stage + 1,
                    current_stage_type: new_stage_type,
                    stage_initialized: false,
                    stage_completed: false,
                };

                stage_initialization_event_writer.send(StageInitializationEvent {
                    module_name,
                    workflow_name,
                    stage_input: stage_output,
                });
            }
        }
    }

    // Handle final stage completions
    let mut callbacks = Vec::new();
    for (module_name, workflow_name, callback, stage_output) in final_stage_completions {
        if let Some(workflows) = workflow_map.map.get_mut(module_name) {
            workflows.remove(workflow_name);
            callbacks.push((callback, stage_output));
        }
    }
    for (callback, data) in callbacks {
        match callback {
            WorkflowCallback::None(callback) => {
                (callback)()
            },
            WorkflowCallback::E(callback) => {
                let data = match data {
                    Some(data) => data,
                    None => unreachable!("Workflow callback error: Expected data, but got None."),
                };
                
                (callback)(data)
            },
            WorkflowCallback::O(callback) => {
                let data = match data {
                    Some(data) => data,
                    None => unreachable!("Workflow callback error: Expected data, but got None."),
                };
                
                (callback)(data)
            },
            WorkflowCallback::OE(callback) => {
                let data = match data {
                    Some(data) => data,
                    None => unreachable!("Workflow callback error: Expected data, but got None."),
                };
                
                (callback)(data)
            },
            WorkflowCallback::I(callback) => {
                (callback)()
            },
            WorkflowCallback::IE(callback) => {
                let data = match data {
                    Some(data) => data,
                    None => unreachable!("Workflow callback error: Expected data, but got None."),
                };
                
                (callback)(data)
            },
            WorkflowCallback::IO(callback) => {
                let data = match data {
                    Some(data) => data,
                    None => unreachable!("Workflow callback error: Expected data, but got None."),
                };
                
                (callback)(data)
            },
            WorkflowCallback::IOE(callback) => {
                let data = match data {
                    Some(data) => data,
                    None => unreachable!("Workflow callback error: Expected data, but got None."),
                };
                
                (callback)(data)
            },
        }
    }
}

// TODO: MAJOR: Touch this system with a lange Zange and a Gefahrenschutzanzug
pub(super) fn workflow_failure_handling_system(world: &mut World) {}
