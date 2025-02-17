use bevy::prelude::*;
use bevy::ecs::system::SystemState;
use bevy::render::MainWorld;
use bevy_consumable_event::{ConsumableEventReader, ConsumableEventWriter};

use crate::{config::statics::CONFIG, gpu::workflows::setup_texture_generator, statics::TOKIO_RUNTIME};

use super::{
    events::{WorkflowStageCompletionEvent, WorkflowStageInitializationEvent}, resources::{WorkflowMap, WorkflowTypeModuleRegistry}, stage::{WorkflowStage, WorkflowStageEcs, WorkflowStageWhileOutcome}, stage_io::{InputState, WorkflowIO}, types::{RawWorkflowData, WorkflowState}, AsyncStageBuffer, AsyncStageCompletionEventReceiver, AsyncStageCompletionEventSender, EcsStageBuffer, EcsStageCompletionEventReceiver, EcsStageCompletionEventSender, EcsWhileStageBuffer, EcsWhileStageCompletionEventReceiver, EcsWhileStageCompletionEventSender, RenderStageBuffer, RenderStageCompletionEventReceiver, RenderStageCompletionEventSender, RenderWhileStageBuffer, RenderWhileStageCompletionEventReceiver, RenderWhileStageCompletionEventSender, DEBUG_ACTION_MODULE, DEBUG_ACTION_NAME, DEBUG_LOGGING_ENABLED
};

pub(in super) fn extract_render_stage_buffer_system(world: &mut World) {
    let mut main_world = SystemState::<ResMut<MainWorld>>::new(world).get_mut(world);
    let extracted_buffer = match main_world.get_resource_mut::<RenderStageBuffer>() {
        Some(mut buffer) => {
            std::mem::take(&mut *buffer)
        },
        None => unreachable!("Render stage buffer resource not found"),
    };

    match world.get_resource_mut::<RenderStageBuffer>() {
        Some(mut buffer) => {
            buffer.0.extend(extracted_buffer.0);
        },
        None => {
            world.insert_resource(extracted_buffer);
        }
    }
}

pub(in super) fn extract_render_while_stage_buffer_system(world: &mut World) {
    let mut main_world = SystemState::<ResMut<MainWorld>>::new(world).get_mut(world);
    let extracted_buffer = match main_world.get_resource_mut::<RenderWhileStageBuffer>() {
        Some(mut buffer) => {
            std::mem::take(&mut *buffer)
        },
        None => unreachable!("Render while stage buffer resource not found"),
    };

    match world.get_resource_mut::<RenderWhileStageBuffer>() {
        Some(mut buffer) => {
            buffer.0.extend(extracted_buffer.0);
        },
        None => {
            world.insert_resource(extracted_buffer);
        }
    }
}

pub(in super) fn poll_ecs_stage_buffer_system(world: &mut World) {
    let drained_buffer = {
        let mut buffer = world.resource_mut::<EcsStageBuffer>();
        std::mem::take(&mut buffer.0)
    };

    for (module_name, workflow_name, current_stage, mut stage, data_buffer) in drained_buffer {
        let io = WorkflowIO::new_input(data_buffer);
        let function = &mut stage.function;
        let output = (function)(io, world).consume_raw();

        let sender = world
            .get_resource::<EcsStageCompletionEventSender>()
            .unwrap()
            .0
            .clone();

        let cloned_module_name = module_name.clone();
        let cloned_workflow_name = workflow_name.clone();

        let output_send_result = sender.send((cloned_module_name, cloned_workflow_name, current_stage, stage, output));

        if let Err(err) = output_send_result {
            unreachable!("Ecs stage completion error: Output send error: {}", err);
        }
    }
}

pub(in super) fn poll_ecs_while_stage_buffer_system(world: &mut World) {
    let drained_buffer = {
        let mut buffer = world.resource_mut::<EcsWhileStageBuffer>();
        std::mem::take(&mut buffer.0)
    };

    let mut waiting_buffer = Vec::new();

    for (module_name, workflow_name, current_stage, mut stage, data_buffer) in drained_buffer {
        let io = WorkflowIO::new_input(data_buffer);
        let function = &mut stage.function;
        let outcome = (function)(io, world);

        match outcome {
            WorkflowStageWhileOutcome::Waiting(io) => {
                waiting_buffer.push((module_name, workflow_name, current_stage, stage, io.consume_raw()));
            },
            WorkflowStageWhileOutcome::Completed(io) => {
                let output = io.consume_raw();

                let sender = world
                    .get_resource::<EcsWhileStageCompletionEventSender>()
                    .unwrap()
                    .0
                    .clone();

                let cloned_module_name = module_name.clone();
                let cloned_workflow_name = workflow_name.clone();

                let output_send_result = sender.send((cloned_module_name, cloned_workflow_name, current_stage, stage, output));

                if let Err(err) = output_send_result {
                    unreachable!("Ecs while stage completion error: Output send error: {}", err);
                }
            }
        }
    }

    let mut buffer = world.resource_mut::<EcsWhileStageBuffer>();
    std::mem::replace(&mut buffer.0, waiting_buffer);
}

pub(in super) fn poll_render_stage_buffer_system(world: &mut World) {
    let drained_buffer = {
        let mut buffer = world.resource_mut::<RenderStageBuffer>();
        std::mem::take(&mut buffer.0)
    };

    for (module_name, workflow_name, current_stage, mut stage, data_buffer) in drained_buffer {
        let io = WorkflowIO::new_input(data_buffer);
        let function = &mut stage.function;
        let output = (function)(io, world).consume_raw();

        let sender = world
            .get_resource::<RenderStageCompletionEventSender>()
            .unwrap()
            .0
            .clone();

        let cloned_module_name = module_name.clone();
        let cloned_workflow_name = workflow_name.clone();

        let output_send_result = sender.send((cloned_module_name, cloned_workflow_name, current_stage, stage, output));

        if let Err(err) = output_send_result {
            unreachable!("Render stage completion error: Output send error: {}", err);
        }
    }
}

pub(in super) fn poll_render_while_stage_buffer_system(world: &mut World) {
    let drained_buffer = {
        let mut buffer = world.resource_mut::<RenderWhileStageBuffer>();
        std::mem::take(&mut buffer.0)
    };

    let mut waiting_buffer = Vec::new();

    for (module_name, workflow_name, current_stage, mut stage, data_buffer) in drained_buffer {
        let io = WorkflowIO::new_input(data_buffer);
        let function = &mut stage.function;
        let outcome = (function)(io, world);

        match outcome {
            WorkflowStageWhileOutcome::Waiting(io) => {
                waiting_buffer.push((module_name, workflow_name, current_stage, stage, io.consume_raw()));
            },
            WorkflowStageWhileOutcome::Completed(io) => {
                let output = io.consume_raw();

                let sender = world
                    .get_resource::<RenderWhileStageCompletionEventSender>()
                    .unwrap()
                    .0
                    .clone();

                let cloned_module_name = module_name.clone();
                let cloned_workflow_name = workflow_name.clone();

                let output_send_result = sender.send((cloned_module_name, cloned_workflow_name, current_stage, stage, output));

                if let Err(err) = output_send_result {
                    unreachable!("Render while stage completion error: Output send error: {}", err);
                }
            }
        }
    }
}

pub(in super) fn poll_async_stage_buffer_system(world: &mut World) {
    let drained_buffer = {
        let mut buffer = world.resource_mut::<AsyncStageBuffer>();
        std::mem::take(&mut buffer.0)
    };

    for (module_name, workflow_name, current_stage, mut stage, data_buffer) in drained_buffer {
        let io = WorkflowIO::new_input(data_buffer);
        let function = &mut stage.function;
        let future = (function)(io);

        let sender = world
            .get_resource::<AsyncStageCompletionEventSender>()
            .unwrap()
            .0
            .clone();

        let cloned_module_name = module_name.clone();
        let cloned_workflow_name = workflow_name.clone();

        let task_spawn_result = TOKIO_RUNTIME.lock().unwrap().block_on(async {
            tokio::spawn(async move {
                let output = future.await.consume_raw();
                
                let output_send_result = sender.send((cloned_module_name, cloned_workflow_name, current_stage, stage, output));

                if let Err(err) = output_send_result {
                    unreachable!("Async stage completion error: Output send error: {}", err);
                }
            }).await
        });

        if let Err(err) = task_spawn_result {
            unreachable!("Async stage execution error: Task spawn error: {}", err);
        }
    }
}

pub(in super) fn handle_ecs_stage_completion_event_system(
    stage_event_receiver: Res<EcsStageCompletionEventReceiver>,
    mut stage_event_writer: ConsumableEventWriter<WorkflowStageCompletionEvent>,
) {
    while let Ok((module_name, workflow_name, current_stage, stage, stage_output)) = stage_event_receiver.0.try_recv() {
        stage_event_writer.send(WorkflowStageCompletionEvent {
            module_name: module_name.clone(),
            workflow_name: workflow_name.clone(),
            current_stage,
            stage_output,
            stage_return: WorkflowStage::Ecs(stage),
        });
    }
}

pub(in super) fn handle_ecs_while_stage_completion_event_system(
    stage_event_receiver: Res<EcsWhileStageCompletionEventReceiver>,
    mut stage_event_writer: ConsumableEventWriter<WorkflowStageCompletionEvent>,
) {
    while let Ok((module_name, workflow_name, current_stage, stage, stage_output)) = stage_event_receiver.0.try_recv() {
        stage_event_writer.send(WorkflowStageCompletionEvent {
            module_name: module_name.clone(),
            workflow_name: workflow_name.clone(),
            current_stage,
            stage_output,
            stage_return: WorkflowStage::EcsWhile(stage),
        });
    }
}

pub(in super) fn handle_render_stage_completion_event_system(
    stage_event_receiver: Res<RenderStageCompletionEventReceiver>,
    mut stage_event_writer: ConsumableEventWriter<WorkflowStageCompletionEvent>, 
) {
    while let Ok((module_name, workflow_name, current_stage, stage, stage_output)) = stage_event_receiver.0.try_recv() {
        stage_event_writer.send(WorkflowStageCompletionEvent {
            module_name: module_name.clone(),
            workflow_name: workflow_name.clone(),
            current_stage,
            stage_output,
            stage_return: WorkflowStage::Render(stage),
        });
    }
}

pub(in super) fn handle_render_while_stage_completion_event_system(
    stage_event_receiver: Res<RenderWhileStageCompletionEventReceiver>,
    mut stage_event_writer: ConsumableEventWriter<WorkflowStageCompletionEvent>, 
) {
    while let Ok((module_name, workflow_name, current_stage, stage, stage_output)) = stage_event_receiver.0.try_recv() {
        stage_event_writer.send(WorkflowStageCompletionEvent {
            module_name: module_name.clone(),
            workflow_name: workflow_name.clone(),
            current_stage,
            stage_output,
            stage_return: WorkflowStage::RenderWhile(stage),
        });
    }
}

pub(in super) fn handle_async_stage_completion_event_system(
    stage_event_receiver: ResMut<AsyncStageCompletionEventReceiver>,
    mut stage_event_writer: ConsumableEventWriter<WorkflowStageCompletionEvent>, 
) {
    while let Ok((module_name, workflow_name, current_stage, stage, stage_output)) = stage_event_receiver.0.try_recv() {
        stage_event_writer.send(WorkflowStageCompletionEvent {
            module_name: module_name.clone(),
            workflow_name: workflow_name.clone(),
            current_stage,
            stage_output,
            stage_return: WorkflowStage::Async(stage),
        });
    }
}

// TODO: Maybe: While resources are stolen, validation can not access those resources! Maybe we just literally remove and reinsert them? Or it's just the case that they are not available at secondary validation due to this exact reasons, but then we need to explicitly document that somewhere/somehow.
pub(in super) fn workflow_request_system(world: &mut World) {
    let mut system_state: SystemState<(
        ResMut<WorkflowMap>, 
        ResMut<WorkflowTypeModuleRegistry>,
        ConsumableEventWriter<WorkflowStageInitializationEvent>,
    )> = SystemState::new(world);
    let (
        mut workflow_map, 
        mut workflow_type_module_registry,
        _
    ) = system_state.get_mut(world);

    // TODO: Duplicate to other relevant places: Rely less on std::mem::take/replace and more on optional resource queries
    let mut stolen_workflow_map = std::mem::take(&mut *workflow_map);
    let mut stolen_workflow_type_module_registry = std::mem::take(&mut *workflow_type_module_registry);

    let mut stage_initialization_events = Vec::new();

    for (module_name, workflows) in stolen_workflow_map.map.iter_mut() {
        for (workflow_name, instance) in workflows.iter_mut() {
            let workflow_type = stolen_workflow_type_module_registry.get_workflow_type_mut(module_name, workflow_name).unwrap();
            
            if let Some(instance) = instance {
                match instance.state {
                    WorkflowState::Requested => {
                        let input = std::mem::replace(&mut instance.data_buffer, RawWorkflowData::new(()));
                        let input = match (workflow_type.secondary_validation)(WorkflowIO::<InputState>::new_input(input), world) {
                            Ok(input) => {
                                input.consume_raw()
                            },
                            Err(err) => {
                                unreachable!(
                                    "Workflow validation error: Workflow '{}' in module '{}' failed secondary validation: {}",
                                    workflow_name, module_name, err
                                );
                            }
                        };

                        instance.state = WorkflowState::Processing { current_stage: 0, stage_completed: false };
                        stage_initialization_events.push(WorkflowStageInitializationEvent {
                            module_name: module_name.clone(),
                            workflow_name: workflow_name.clone(),
                            stage_input: input,
                        });
                    },
                    WorkflowState::Processing { .. } => {}
                }
            }
        }
    }

    let (
        mut workflow_map, 
        mut workflow_type_module_registry,
        mut stage_initialization_event_writer
    ) = system_state.get_mut(world);
    
    *workflow_map = stolen_workflow_map;
    *workflow_type_module_registry = stolen_workflow_type_module_registry;

    for event in stage_initialization_events {
        stage_initialization_event_writer.send(event);
    }
}

pub(in super) fn workflow_execution_system(world: &mut World) {
    let mut system_state: SystemState<(
        ResMut<WorkflowMap>, 
        ResMut<WorkflowTypeModuleRegistry>,
        ResMut<EcsStageBuffer>,
        ResMut<EcsWhileStageBuffer>,
        ResMut<RenderStageBuffer>,
        ResMut<RenderWhileStageBuffer>,
        ResMut<AsyncStageBuffer>,
        ConsumableEventReader<WorkflowStageInitializationEvent>,
    )> = SystemState::new(world);
    let (
        mut workflow_map, 
        mut workflow_type_module_registry,
        mut ecs_stage_buffer,
        mut ecs_while_stage_buffer,
        mut render_stage_buffer,
        mut render_while_stage_buffer,
        mut async_stage_buffer,
        mut stage_initialization_event_reader 
    ) = system_state.get_mut(world);

    for event in stage_initialization_event_reader.read() {
        let event = event.consume();

        let module_name = event.module_name;
        let workflow_name = event.workflow_name;
        let stage_input = event.stage_input;

        if let Some(workflows) = workflow_map.map.get_mut(&module_name) {
            if let Some(instance) = workflows.get_mut(&workflow_name).and_then(|a| a.as_mut()) {
                let current_stage = instance.state.current_stage();

                let workflow_type = workflow_type_module_registry
                    .get_workflow_type_mut(&module_name, &workflow_name)
                    .unwrap();

                let stage = std::mem::replace(&mut workflow_type.stages[current_stage], WorkflowStage::Ecs(WorkflowStageEcs {
                    name: "placeholder".to_string(),
                    function: Box::new(|_, _| unreachable!()),
                }));

                match stage {
                    WorkflowStage::Ecs(stage) => {
                        ecs_stage_buffer.0.push((module_name.clone(), workflow_name.clone(), current_stage, stage, stage_input));
                    },
                    WorkflowStage::EcsWhile(stage) => {
                        ecs_while_stage_buffer.0.push((module_name.clone(), workflow_name.clone(), current_stage, stage, stage_input));
                    },
                    WorkflowStage::Render(stage) => {
                        render_stage_buffer.0.push((module_name.clone(), workflow_name.clone(), current_stage, stage, stage_input));
                    },
                    WorkflowStage::RenderWhile(stage) => {
                        render_while_stage_buffer.0.push((module_name.clone(), workflow_name.clone(), current_stage, stage, stage_input));
                    },
                    WorkflowStage::Async(stage) => {
                        async_stage_buffer.0.push((module_name.clone(), workflow_name.clone(), current_stage, stage, stage_input));
                    },
                }
            } else {
                unreachable!("Workflow instance not found for module '{}' and name '{}'", module_name, workflow_name);
            }
        } else {
            unreachable!("Workflow instance module not found for name '{}'", module_name);
        }
    }
}

pub(in super) fn workflow_completion_handling_system(world: &mut World) {
    let mut system_state: SystemState<(
        ResMut<WorkflowMap>, 
        ResMut<WorkflowTypeModuleRegistry>,
        ConsumableEventReader<WorkflowStageCompletionEvent>,
        ConsumableEventWriter<WorkflowStageInitializationEvent>,
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
        let stage_return = event.stage_return;

        if let Some(workflows) = workflow_map.map.get_mut(&module_name) {
            if let Some(instance) = workflows.get_mut(&workflow_name).and_then(|a| a.as_mut()) {
                match &mut instance.state {
                    WorkflowState::Processing { current_stage: other_current_stage, stage_completed: completed } => {
                        if current_stage != *other_current_stage {
                            unreachable!("Unexpected workflow state. Completion event is at stage '{}', but the workflow instance is at stage '{}'", current_stage, other_current_stage);
                        }
                        if *completed {
                            unreachable!("Unexpected workflow state. Workflow '{}' in module '{}' is already completed.", workflow_name, module_name);
                        }
                        
                        *completed = true;
                    },
                    state => unreachable!("Unexpected workflow state. Expected 'WorkflowState::Processing(_)', got '{:?}'", state),
                };

                let workflow_type = workflow_type_module_registry
                    .get_workflow_type_mut(&module_name, &workflow_name)
                    .unwrap();

                workflow_type.stages[current_stage] = stage_return;

                if current_stage + 1 < workflow_type.stages.len() {
                    intermediate_stage_completions.push((module_name.clone(), workflow_name.clone(), current_stage, stage_output));
                } else {
                    final_stage_completions.push((module_name.clone(), workflow_name.clone(), std::mem::take(&mut instance.callback), stage_output));
                }
            }
        }
    }

    // Handle intermediate stage completions
    for (module_name, workflow_name, current_stage, stage_output) in intermediate_stage_completions {
        if let Some(workflows) = workflow_map.map.get_mut(&module_name) {
            if let Some(instance) = workflows.get_mut(&workflow_name).and_then(|a| a.as_mut()) {
                instance.state = WorkflowState::Processing { current_stage: current_stage + 1, stage_completed: false };

                stage_initialization_event_writer.send(WorkflowStageInitializationEvent {
                    module_name: module_name.clone(),
                    workflow_name: workflow_name.clone(),
                    stage_input: stage_output,
                });
            }
        }
    }

    // Handle final stage completions
    let mut callbacks = Vec::new();
    for (module_name, workflow_name, callback, stage_output) in final_stage_completions {
        if let Some(workflows) = workflow_map.map.get_mut(&module_name) {
            workflows.remove(&workflow_name);
            callbacks.push((callback, stage_output));
        }
    }
    for (callback, data) in callbacks {
        if let Some(callback) = callback {
            let io = WorkflowIO::new_callback_data(data);
            callback(world, io);
        }
    }
}
