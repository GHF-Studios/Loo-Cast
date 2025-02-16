use bevy::prelude::*;
use bevy::ecs::system::SystemState;
use bevy::render::MainWorld;
use bevy_consumable_event::{ConsumableEventReader, ConsumableEventWriter};

use crate::{config::statics::CONFIG, statics::TOKIO_RUNTIME};

use super::{
    events::WorkflowStageCompletionEvent, resources::{WorkflowMap, WorkflowTypeModuleRegistry}, stage::{WorkflowStage, WorkflowStageAsync, WorkflowStageEcs, WorkflowStageEcsWhile, WorkflowStageRender, WorkflowStageRenderWhile}, stage_io::{WorkflowIO, InputState}, types::{WorkflowState, RawWorkflowData}, WorkflowStageCompletionEventReceiverAsync, EcsStageCompletionEventReceiver, EcsWhileStageCompletionEventReceiver, WorkflowStageCompletionEventReceiverRender, WorkflowStageCompletionEventReceiverRenderWhile, WorkflowStageCompletionEventSenderAsync, EcsStageCompletionEventSender, EcsWhileStageCompletionEventSender, RenderStageCompletionEventSender, WorkflowStageCompletionEventSenderRenderWhile, AsyncStageCompletionEventQueue, EcsStageCompletionEventQueue, EcsWhileStageCompletionEventQueue, RenderStageCompletionEventQueue, RenderWhileStageCompletionEventQueue, DEBUG_ACTION_MODULE, DEBUG_ACTION_NAME, DEBUG_LOGGING_ENABLED
};

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
            stage_return: Some(WorkflowStage::Ecs(stage)),
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
            stage_return: Some(WorkflowStage::EcsWhile(stage)),
        });
    }
}

pub(in super) fn handle_render_stage_completion_event_system(
    stage_event_receiver: Res<WorkflowStageCompletionEventReceiverRender>,
    mut stage_event_writer: ConsumableEventWriter<WorkflowStageCompletionEvent>, 
) {
    while let Ok((module_name, workflow_name, current_stage, stage, stage_output)) = stage_event_receiver.0.try_recv() {
        stage_event_writer.send(WorkflowStageCompletionEvent {
            module_name: module_name.clone(),
            workflow_name: workflow_name.clone(),
            current_stage,
            stage_output,
            stage_return: Some(WorkflowStage::Render(stage)),
        });
    }
}

pub(in super) fn handle_render_while_stage_completion_event_system(
    stage_event_receiver: Res<WorkflowStageCompletionEventReceiverRenderWhile>,
    mut stage_event_writer: ConsumableEventWriter<WorkflowStageCompletionEvent>, 
) {
    while let Ok((module_name, workflow_name, current_stage, stage, stage_output)) = stage_event_receiver.0.try_recv() {
        stage_event_writer.send(WorkflowStageCompletionEvent {
            module_name: module_name.clone(),
            workflow_name: workflow_name.clone(),
            current_stage,
            stage_output,
            stage_return: Some(WorkflowStage::RenderWhile(stage)),
        });
    }
}

pub(in super) fn handle_async_stage_completion_event_system(
    stage_event_receiver: ResMut<WorkflowStageCompletionEventReceiverAsync>,
    mut stage_event_writer: ConsumableEventWriter<WorkflowStageCompletionEvent>, 
) {
    while let Ok((module_name, workflow_name, current_stage, stage, stage_output)) = stage_event_receiver.0.try_recv() {
        stage_event_writer.send(WorkflowStageCompletionEvent {
            module_name: module_name.clone(),
            workflow_name: workflow_name.clone(),
            current_stage,
            stage_output,
            stage_return: Some(WorkflowStage::Async(stage)),
        });
    }
}

pub(in super) fn extract_render_stage_queue_system(world: &mut World) {
    let mut main_world = SystemState::<ResMut<MainWorld>>::new(world).get_mut(world);
    let extracted_queue = match main_world.get_resource_mut::<RenderStageCompletionEventQueue>() {
        Some(mut queue) => {
            std::mem::take(&mut *queue)
        },
        None => unreachable!("Render stage queue resource not found"),
    };

    match world.get_resource_mut::<RenderStageCompletionEventQueue>() {
        Some(mut queue) => {
            *queue = extracted_queue;
        },
        None => {
            world.insert_resource(extracted_queue);
        }
    }
}

pub(in super) fn extract_render_while_stage_queue_system(world: &mut World) {
    let mut main_world = SystemState::<ResMut<MainWorld>>::new(world).get_mut(world);
    let extracted_queue = match main_world.get_resource_mut::<RenderWhileStageCompletionEventQueue>() {
        Some(mut queue) => {
            std::mem::take(&mut *queue)
        },
        None => unreachable!("Render while stage queue resource not found"),
    };

    match world.get_resource_mut::<RenderWhileStageCompletionEventQueue>() {
        Some(mut queue) => {
            *queue = extracted_queue;
        },
        None => {
            world.insert_resource(extracted_queue);
        }
    }
}

pub(in super) fn execute_ecs_stages_system(world: &mut World) {
    let drained_queue = {
        let mut queue = world.resource_mut::<EcsStageCompletionEventQueue>();
        std::mem::take(&mut queue.0)
    };

    let mut completion_event = Vec::with_capacity(drained_queue.len());

    for (module_name, workflow_name, current_stage, mut stage, data_buffer) in drained_queue {
        let io = WorkflowIO::new_input(data_buffer);
        let function = &mut stage.function;
        let output = (function)(io, world).consume_raw();

        completion_event.push(WorkflowStageCompletionEvent {
            module_name,
            workflow_name,
            current_stage,
            stage_output: output,
            stage_return: Some(WorkflowStage::Ecs(stage))
        });
    }

    for event in completion_event {
        match event.stage_return {
            Some(WorkflowStage::Ecs(stage)) => {
                let sender = world.get_resource::<EcsStageCompletionEventSender>().unwrap();
                let _ = sender.0.send((event.module_name, event.workflow_name, event.current_stage, stage, event.stage_output));
            },
            Some(WorkflowStage::EcsWhile(_)) => unreachable!("Expected Ecs stage return, got EcsWhile"),
            Some(WorkflowStage::Render(_)) => unreachable!("Expected Ecs stage return, got Render"),
            Some(WorkflowStage::RenderWhile(_)) => unreachable!("Expected Ecs stage return, got RenderWhile"),
            Some(WorkflowStage::Async(_)) => unreachable!("Expected Ecs stage return, got Async"),
            None => unreachable!("Expected Ecs stage return, got None"),
        }
    }
}

pub(in super) fn execute_ecs_while_stages_system(world: &mut World) {
    // TODO: 1. Implement
    // TODO: Legcay code for stage 'EcsWhile':
    //
    //match (function)(io, world) {
    //    WorkflowStageEcsWhileOutcome::Waiting(input) => {
    //        let mut workflow_map = SystemState::<ResMut<WorkflowMap>>::new(world).get_mut(world);
    //        if let Some(workflows) = workflow_map.map.get_mut(&module_name) {
    //            if let Some(instance) = workflows.get_mut(&workflow_name).and_then(|a| a.as_mut()) {
    //                instance.data_buffer = input.consume_raw();
    //                instance.timeout_frames = instance.num_stages * CONFIG.get::<usize>("workflow/timeout_frames_per_stage"); // Reset timeout
    //            }
    //        }
    //    },
    //    WorkflowStageEcsWhileOutcome::Completed(output) => {
    //        ecs_stage_completions.push((
    //            cloned_module_name,
    //            cloned_workflow_name,
    //            current_stage,
    //            output.consume_raw(),
    //        ));
    //    }
    //}
}

pub(in super) fn execute_render_stages_system(world: &mut World) {
    let drained_queue = {
        let mut queue = world.resource_mut::<RenderStageCompletionEventQueue>();
        std::mem::take(&mut queue.0)
    };

    let mut result_events = Vec::with_capacity(drained_queue.len());

    for (module_name, workflow_name, current_stage, mut stage, data_buffer) in drained_queue {
        let io = WorkflowIO::new_input(data_buffer);
        let function = &mut stage.function;
        let output = (function)(io, world).consume_raw();

        result_events.push(WorkflowStageCompletionEvent {
            module_name,
            workflow_name,
            current_stage,
            stage_output: output,
            stage_return: Some(WorkflowStage::Render(stage))
        });
    }

    for event in result_events {
        match event.stage_return {
            Some(WorkflowStage::Ecs(_)) => unreachable!("Expected Render stage return, got Ecs"),
            Some(WorkflowStage::EcsWhile(_)) => unreachable!("Expected Render stage return, got EcsWhile"),
            Some(WorkflowStage::Render(stage)) => {
                let sender = world.get_resource::<RenderStageCompletionEventSender>().unwrap();
                let _ = sender.0.send((event.module_name, event.workflow_name, event.current_stage, stage, event.stage_output));
            },
            Some(WorkflowStage::RenderWhile(_)) => unreachable!("Expected Render stage return, got RenderWhile"),
            Some(WorkflowStage::Async(_)) => unreachable!("Expected Render stage return, got Async"),
            None => unreachable!("Expected Render stage return, got None"),
        }
    }
}

pub(in super) fn execute_render_while_stages_system(world: &mut World) {
    // TODO: 1. Implement
}

pub(in super) fn execute_async_stages_system(world: &mut World) {
    let drained_queue = {
        let mut queue = world.resource_mut::<AsyncStageCompletionEventQueue>();
        std::mem::take(&mut queue.0)
    };

    for (module_name, workflow_name, current_stage, mut stage, data_buffer) in drained_queue {
        let io = WorkflowIO::new_input(data_buffer);
        let function = &mut stage.function;
        let future = (function)(io);

        let sender = world
            .get_resource::<WorkflowStageCompletionEventSenderAsync>()
            .unwrap()
            .0
            .clone();

        let cloned_module_name = module_name.clone();
        let cloned_workflow_name = workflow_name.clone();

        let task_spawn_result = TOKIO_RUNTIME.lock().unwrap().block_on(async {
            tokio::spawn(async move {
                let output = future.await.consume_raw();
                
                let output_send_result = sender.send((
                    cloned_module_name,
                    cloned_workflow_name,
                    current_stage,
                    stage,
                    output
                ));

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

// TODO: 3. Maybe: While resources are stolen, validation can not access those resources! Maybe we just literally remove and reinsert them?
pub(in super) fn workflow_request_system(world: &mut World) {
    let mut system_state: SystemState<(
        ResMut<WorkflowMap>, 
        ResMut<WorkflowTypeModuleRegistry>,
    )> = SystemState::new(world);
    let (
        mut workflow_map, 
        mut workflow_type_module_registry
    ) = system_state.get_mut(world);

    // TODO: 2. Duplicate to other relevant places: Rely less on std::mem::take/replace and more on optional resource queries
    let mut stolen_workflow_map = std::mem::take(&mut *workflow_map);
    let mut stolen_workflow_type_module_registry = std::mem::take(&mut *workflow_type_module_registry);

    for (module_name, workflows) in stolen_workflow_map.map.iter_mut() {
        for (workflow_name, instance) in workflows.iter_mut() {
            let workflow_type = stolen_workflow_type_module_registry.get_workflow_type_mut(module_name, workflow_name).unwrap();
            
            if let Some(instance) = instance {
                match instance.state {
                    WorkflowState::Requested => {
                        if DEBUG_LOGGING_ENABLED && module_name == DEBUG_ACTION_MODULE && workflow_name == DEBUG_ACTION_NAME {
                            debug!("Lifecycle Stage 2.1 @ {}: Workflow `{}` enters Secondary Validation.", instance.state.current_stage(), workflow_name);
                        }

                        let input = std::mem::replace(&mut instance.data_buffer, RawWorkflowData::new(()));
                        match (workflow_type.secondary_validation)(WorkflowIO::<InputState>::new_input(input), world) {
                            Ok(input) => {
                                instance.data_buffer = input.consume_raw();
                            },
                            Err(err) => {
                                panic!(
                                    "Workflow validation error: Workflow '{}' in module '{}' failed secondary validation: {}",
                                    workflow_name, module_name, err
                                );
                            }
                        }

                        if DEBUG_LOGGING_ENABLED && module_name == DEBUG_ACTION_MODULE && workflow_name == DEBUG_ACTION_NAME {
                            debug!("Lifecycle Stage 2.2 @ {}: Secondary Validation Passed.", instance.state.current_stage());
                        }

                        // TODO: This is not enough. We need to properly send some sort of workflow entrypoint event
                        instance.state = WorkflowState::Processing { current_stage: 0 };
                    },
                    WorkflowState::Processing { current_stage } => {
                        if instance.timeout_frames == 0 {
                            panic!(
                                "Workflow processing error: Workflow '{}' in module '{}' timed out at stage {}",
                                workflow_name, module_name, current_stage
                            );
                        }
                        instance.timeout_frames -= 1;
                    },
                    WorkflowState::Processed { .. } => {
                        warn!("This message appearing may be bad, but it does not necessarily have to be, if nothing else seems to indicate any sort of failure!")
                    }
                }
            }
        }
    }

    let mut system_state: SystemState<(
        ResMut<WorkflowMap>, 
        ResMut<WorkflowTypeModuleRegistry>,
    )> = SystemState::new(world);
    let (
        mut workflow_map, 
        mut workflow_type_module_registry
    ) = system_state.get_mut(world);
    
    *workflow_map = stolen_workflow_map;
    *workflow_type_module_registry = stolen_workflow_type_module_registry;
}

pub(in super) fn stage_execution_cleanup_system(world: &mut World) {
    let completed_executions = find_completed_workflow_stages(world);
    cleanup_completed_workflow_stages(world, completed_executions);
}

fn find_completed_workflow_stages(world: &mut World) -> Vec<(String, String, usize, WorkflowStage, RawWorkflowData)> {
    let mut system_state: SystemState<(
        ConsumableEventReader<WorkflowStageCompletionEvent>,
        ResMut<WorkflowMap>, 
    )> = SystemState::new(world);
    let (
        mut stage_completion_event_reader,
        mut workflow_map, 
    ) = system_state.get_mut(world);

    let mut completed_workflow_stages = Vec::new();

    for event in stage_completion_event_reader.read() {
        let event = event.consume();
        let module_name = event.module_name;
        let workflow_name = event.workflow_name;
        let stage_output = event.stage_output;
        let stage_return = event.stage_return;

        let workflow_instance = workflow_map
            .map
            .get_mut(&module_name)
            .and_then(|workflows| workflows.get_mut(&workflow_name))
            .and_then(|a| a.as_mut());

        if let Some(workflow_instance) = workflow_instance {
            /* TODO: 4. Maybe: For all Non-Ecs Stages: This can not distinguish between a fresh while stage and a while stage that has already been "polled".
            *           We could add a "is_looping" to alongside "current_stage" to distinguish between the two,
            *           and non-while workflows would just have "is_looping" set to false from the beginning 
            *           and also have "is_looping" completely ignored in non-while stages' execution.
            *           This inability to distinguish results in repeated polling of while stage, but not normal polling but polling as if the stage was fresh.
            */
            if let WorkflowState::Processed { current_stage } = &workflow_instance.state {
                if event.current_stage != *current_stage {
                    unreachable!("Unexpected workflow state. Completion event is at stage '{}', but the workflow instance is at stage '{}'", event.current_stage, current_stage);
                }
                completed_workflow_stages.push((
                    module_name.clone(),
                    workflow_name.clone(),
                    *current_stage,
                    stage_return.unwrap(),
                    stage_output,
                ));
            } else {
                unreachable!("Unexpected workflow state. Expected 'WorkflowState::Processed(_)', got '{:?}'", workflow_instance.state);
            }
        } else {
            unreachable!("Workflow instance not found for module '{}' and workflow '{}'", module_name, workflow_name);
        }
    }

    completed_workflow_stages
}

fn cleanup_completed_workflow_stages(
    world: &mut World,
    mut completed_workflow_stages: Vec<(String, String, usize, WorkflowStage, RawWorkflowData)>,
) {
    let mut system_state: SystemState<(
        ResMut<EcsStageCompletionEventQueue>,
        ResMut<EcsWhileStageCompletionEventQueue>,
        ResMut<RenderStageCompletionEventQueue>,
        ResMut<RenderWhileStageCompletionEventQueue>,
        ResMut<AsyncStageCompletionEventQueue>,
    )> = SystemState::new(world);
    let (
        mut ecs_stage_completion_event_queue,
        mut ecs_while_stage_completion_event_queue,
        mut render_stage_completion_event_queue,
        mut render_while_stage_completion_event_queue,
        mut async_stage_completion_event_queue,
    ) = system_state.get_mut(world);

    for (module_name, workflow_name, current_stage, mut stage, data_buffer) in completed_workflow_stages.drain(..) {
        if DEBUG_LOGGING_ENABLED && module_name == DEBUG_ACTION_MODULE && workflow_name == DEBUG_ACTION_NAME {
            debug!(
                "Lifecycle Stage 3.1 @ {}: Starting execution of Stage for `{}` in module `{}`.",
                current_stage, workflow_name, module_name
            );
        }

        match stage {
            // **ECS Stage: Runs in immediate ECS context**
            WorkflowStage::Ecs(ref mut ecs_stage) => {
                if DEBUG_LOGGING_ENABLED && module_name == DEBUG_ACTION_MODULE && workflow_name == DEBUG_ACTION_NAME {
                    debug!(
                        "Lifecycle Stage 3.2 @ {}: Running ECS stage `{}` for `{}` in module `{}`.",
                        current_stage, ecs_stage.name, workflow_name, module_name
                    );
                }

                let ecs_stage = std::mem::replace(ecs_stage, WorkflowStageEcs {
                    name: "placeholder".to_string(),
                    function: Box::new(|_, _| unreachable!()),
                });

                ecs_stage_completion_event_queue.0.push((module_name.clone(), workflow_name.clone(), current_stage, ecs_stage, data_buffer));
            }

            // **EcsWhile Stage: Loops in immediate ECS context until a condition is met**
            WorkflowStage::EcsWhile(ref mut ecs_while_stage) => {
                if DEBUG_LOGGING_ENABLED && module_name == DEBUG_ACTION_MODULE && workflow_name == DEBUG_ACTION_NAME {
                    debug!(
                        "Lifecycle Stage 3.2 @ {}: Running ECS While stage `{}` for `{}` in module `{}`.",
                        current_stage, ecs_while_stage.name, workflow_name, module_name
                    );
                }

                let ecs_while_stage = std::mem::replace(ecs_while_stage, WorkflowStageEcsWhile {
                    name: "placeholder".to_string(),
                    function: Box::new(|_, _| unreachable!()),
                });

                ecs_while_stage_completion_event_queue.0.push((module_name.clone(), workflow_name.clone(), current_stage, ecs_while_stage, data_buffer));
            }

            // TODO: Make these stages outcome-based too, so we can immediately return some sort of outcome, even if it is 'ask me later', and POSSIBLY add an error variant to the outcome
            // **ECS Render Stage → Queue for RenderApp**
            WorkflowStage::Render(ref mut render_stage) => {
                if DEBUG_LOGGING_ENABLED && module_name == DEBUG_ACTION_MODULE && workflow_name == DEBUG_ACTION_NAME {
                    debug!(
                        "Lifecycle Stage 3.2 @ {}: Running ECS Render stage `{}` for `{}` in module `{}`.",
                        current_stage, render_stage.name, workflow_name, module_name
                    );
                }

                let render_stage = std::mem::replace(render_stage, WorkflowStageRender {
                    name: "placeholder".to_string(),
                    function: Box::new(|_, _| unreachable!()),
                });
                
                render_stage_completion_event_queue.0.push((module_name.clone(), workflow_name.clone(), current_stage, render_stage, data_buffer));
            }

            // TODO: Make these stages outcome-based too, so we can immediately return some sort of outcome, even if it is 'ask me later', and POSSIBLY add an error variant to the outcome
            // **ECS RenderWhile Stage → Queue for RenderApp (Retries Until Completion)**
            WorkflowStage::RenderWhile(ref mut render_while_stage) => {
                if DEBUG_LOGGING_ENABLED && module_name == DEBUG_ACTION_MODULE && workflow_name == DEBUG_ACTION_NAME {
                    debug!(
                        "Lifecycle Stage 3.2 @ {}: Running ECS Render While stage `{}` for `{}` in module `{}`.",
                        current_stage, render_while_stage.name, workflow_name, module_name
                    );
                }

                let render_while_stage = std::mem::replace(render_while_stage, WorkflowStageRenderWhile {
                    name: "placeholder".to_string(),
                    function: Box::new(|_, _| unreachable!()),
                });

                render_while_stage_completion_event_queue.0.push((module_name.clone(), workflow_name.clone(), current_stage, render_while_stage, data_buffer));
            }

            // TODO: Make these stages outcome-based too, so we can immediately return some sort of outcome, even if it is 'ask me later', and POSSIBLY add an error variant to the outcome
            // **Async Stage: Runs non-blocking in a separate task**
            WorkflowStage::Async(ref mut async_stage) => {
                if DEBUG_LOGGING_ENABLED && module_name == DEBUG_ACTION_MODULE && workflow_name == DEBUG_ACTION_NAME {
                    debug!(
                        "Lifecycle Stage 3.2 @ {}: Running Async stage `{}` for `{}` in module `{}`.",
                        current_stage, async_stage.name, workflow_name, module_name
                    );
                }

                let async_stage = std::mem::replace(async_stage, WorkflowStageAsync {
                    name: "placeholder".to_string(),
                    function: Box::new(|_| unreachable!()),
                });

                async_stage_completion_event_queue.0.push((module_name.clone(), workflow_name.clone(), current_stage, async_stage, data_buffer));
            }
        }
    }
}

pub(in super) fn workflow_progression_system(world: &mut World) {
    let mut system_state: SystemState<(
        ResMut<WorkflowMap>, 
        ConsumableEventReader<WorkflowStageCompletionEvent>,
        ResMut<WorkflowTypeModuleRegistry>
    )> = SystemState::new(world);
    let (
        mut workflow_map, 
        mut stage_event_reader,
        mut workflow_type_module_registry,
    ) = system_state.get_mut(world);

    let mut completed_stages = Vec::new();
    let mut completed_workflows = Vec::new();
    let mut callbacks = Vec::new();

    for event in stage_event_reader.read() {
        let event = event.consume();

        if DEBUG_LOGGING_ENABLED && event.module_name == DEBUG_ACTION_MODULE && event.workflow_name == DEBUG_ACTION_NAME {
            debug!(
                "Lifecycle Stage 3.3 @ {}: Stage completion event received for `{}` in module `{}`",
                event.current_stage, event.workflow_name, event.module_name
            );
        }

        if let Some(workflows) = workflow_map.map.get_mut(&event.module_name) {
            if let Some(instance) = workflows.get_mut(&event.workflow_name).and_then(|a| a.as_mut()) {
                match &mut instance.state {
                    WorkflowState::Processed { current_stage } => {
                        instance.data_buffer = event.stage_output;
                        instance.timeout_frames = instance.num_stages * CONFIG.get::<usize>("workflow/timeout_frames_per_stage");
                        completed_stages.push((event.module_name.clone(), event.workflow_name.clone(), *current_stage));

                        if *current_stage + 1 >= instance.num_stages {
                            completed_workflows.push((event.module_name.clone(), event.workflow_name.clone()));
                        }
                    }
                    state => unreachable!("Unexpected state transition to {:?}", state),
                };

                if event.stage_return.is_some() {
                    let workflow_type = workflow_type_module_registry
                        .get_workflow_type_mut(&event.module_name, &event.workflow_name)
                        .unwrap();

                    workflow_type.stages[event.current_stage] = event.stage_return;
                }
            }
        }
    }

    // Finalize completed stages
    for (module_name, workflow_name, current_stage) in completed_stages {
        if let Some(workflows) = workflow_map.map.get_mut(&module_name) {
            if let Some(instance) = workflows.get_mut(&workflow_name).and_then(|a| a.as_mut()) {
                instance.state = WorkflowState::Processing { current_stage: current_stage + 1 };
            }

            if DEBUG_LOGGING_ENABLED && module_name == DEBUG_ACTION_MODULE && workflow_name == DEBUG_ACTION_NAME {
                debug!(
                    "Lifecycle Stage 3.4 @ {}: Stage completion handled for `{}` in module `{}`",
                    current_stage, workflow_name, module_name
                );
            }
        }
    }

    // Finalize completed workflows
    for (module_name, workflow_name) in completed_workflows {
        if let Some(workflows) = workflow_map.map.get_mut(&module_name) {
            if let Some(instance) = workflows.remove(&workflow_name).flatten() {
                callbacks.push((instance.callback, instance.data_buffer));
            }
        }

        if DEBUG_LOGGING_ENABLED && module_name == DEBUG_ACTION_MODULE && workflow_name == DEBUG_ACTION_NAME {
            debug!(
                "Lifecycle Stage 3.5 @ x: Workflow `{}` in module `{}` completed.",
                workflow_name, module_name
            );
        }
    }

    // Execute callbacks
    for (callback, data) in callbacks {
        if let Some(callback) = callback {
            let io = WorkflowIO::new_callback_data(data);
            callback(world, io);
        }
    }
}
