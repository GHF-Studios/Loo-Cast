use bevy::prelude::*;
use bevy::ecs::system::SystemState;
use bevy::render::MainWorld;
use bevy_consumable_event::{ConsumableEventReader, ConsumableEventWriter};

use super::{
    events::ActionStageCompletionEvent, resources::{ActionMap, ActionTypeModuleRegistry}, stage::{ActionStage, ActionStageAsync, ActionStageEcs, ActionStageEcsWhile, ActionStageRender, ActionStageRenderWhile}, stage_io::{ActionIO, InputState}, types::{ActionState, RawActionData}, ActionStageCompletionEventReceiverAsync, ActionStageCompletionEventReceiverEcs, ActionStageCompletionEventReceiverEcsWhile, ActionStageCompletionEventReceiverRender, ActionStageCompletionEventReceiverRenderWhile, ActionStageCompletionEventSenderAsync, ActionStageCompletionEventSenderEcs, ActionStageCompletionEventSenderEcsWhile, ActionStageCompletionEventSenderRender, ActionStageCompletionEventSenderRenderWhile, AsyncStageCompletionEventQueue, EcsStageCompletionEventQueue, EcsWhileStageCompletionEventQueue, RenderStageCompletionEventQueue, RenderWhileStageCompletionEventQueue, DEBUG_ACTION_MODULE, DEBUG_ACTION_NAME, DEBUG_LOGGING_ENABLED
};

pub(in super) fn handle_ecs_stage_completion_event_system(
    stage_event_receiver: Res<ActionStageCompletionEventReceiverEcs>,
    mut stage_event_writer: ConsumableEventWriter<ActionStageCompletionEvent>,
) {
    while let Ok((module_name, action_name, current_stage, stage, stage_output)) = stage_event_receiver.0.try_recv() {
        stage_event_writer.send(ActionStageCompletionEvent {
            module_name: module_name.clone(),
            action_name: action_name.clone(),
            current_stage,
            stage_output,
            stage_return: Some(ActionStage::Ecs(stage)),
        });
    }
}

pub(in super) fn handle_ecs_while_stage_completion_event_system(
    stage_event_receiver: Res<ActionStageCompletionEventReceiverEcsWhile>,
    mut stage_event_writer: ConsumableEventWriter<ActionStageCompletionEvent>,
) {
    while let Ok((module_name, action_name, current_stage, stage, stage_output)) = stage_event_receiver.0.try_recv() {
        stage_event_writer.send(ActionStageCompletionEvent {
            module_name: module_name.clone(),
            action_name: action_name.clone(),
            current_stage,
            stage_output,
            stage_return: Some(ActionStage::EcsWhile(stage)),
        });
    }
}

pub(in super) fn handle_render_stage_completion_event_system(
    stage_event_receiver: Res<ActionStageCompletionEventReceiverRender>,
    mut stage_event_writer: ConsumableEventWriter<ActionStageCompletionEvent>, 
) {
    while let Ok((module_name, action_name, current_stage, stage, stage_output)) = stage_event_receiver.0.try_recv() {
        stage_event_writer.send(ActionStageCompletionEvent {
            module_name: module_name.clone(),
            action_name: action_name.clone(),
            current_stage,
            stage_output,
            stage_return: Some(ActionStage::Render(stage)),
        });
    }
}

pub(in super) fn handle_render_while_stage_completion_event_system(
    stage_event_receiver: Res<ActionStageCompletionEventReceiverRenderWhile>,
    mut stage_event_writer: ConsumableEventWriter<ActionStageCompletionEvent>, 
) {
    while let Ok((module_name, action_name, current_stage, stage, stage_output)) = stage_event_receiver.0.try_recv() {
        stage_event_writer.send(ActionStageCompletionEvent {
            module_name: module_name.clone(),
            action_name: action_name.clone(),
            current_stage,
            stage_output,
            stage_return: Some(ActionStage::RenderWhile(stage)),
        });
    }
}

pub(in super) fn handle_async_stage_completion_event_system(
    stage_event_receiver: ResMut<ActionStageCompletionEventReceiverAsync>,
    mut stage_event_writer: ConsumableEventWriter<ActionStageCompletionEvent>, 
) {
    while let Ok((module_name, action_name, current_stage, stage, stage_output)) = stage_event_receiver.0.try_recv() {
        stage_event_writer.send(ActionStageCompletionEvent {
            module_name: module_name.clone(),
            action_name: action_name.clone(),
            current_stage,
            stage_output,
            stage_return: Some(ActionStage::Async(stage)),
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

    for (module_name, action_name, current_stage, mut stage, data_buffer) in drained_queue {
        let io = ActionIO::new_input(data_buffer);
        let function = &mut stage.function;
        let output = (function)(io, world).consume_raw();

        completion_event.push(ActionStageCompletionEvent {
            module_name,
            action_name,
            current_stage,
            stage_output: output,
            stage_return: Some(ActionStage::Ecs(stage))
        });
    }

    for event in completion_event {
        match event.stage_return {
            Some(ActionStage::Ecs(stage)) => {
                let sender = world.get_resource::<ActionStageCompletionEventSenderEcs>().unwrap();
                let _ = sender.0.send((event.module_name, event.action_name, event.current_stage, stage, event.stage_output));
            },
            Some(ActionStage::EcsWhile(_)) => unreachable!("Expected Ecs stage return, got EcsWhile"),
            Some(ActionStage::Render(_)) => unreachable!("Expected Ecs stage return, got Render"),
            Some(ActionStage::RenderWhile(_)) => unreachable!("Expected Ecs stage return, got RenderWhile"),
            Some(ActionStage::Async(_)) => unreachable!("Expected Ecs stage return, got Async"),
            None => unreachable!("Expected Ecs stage return, got None"),
        }
    }
}

pub(in super) fn execute_ecs_while_stages_system(world: &mut World) {
    // TODO: 1. Implement
}

pub(in super) fn execute_render_stages_system(world: &mut World) {
    let drained_queue = {
        let mut queue = world.resource_mut::<RenderStageCompletionEventQueue>();
        std::mem::take(&mut queue.0)
    };

    let mut result_events = Vec::with_capacity(drained_queue.len());

    for (module_name, action_name, current_stage, mut stage, data_buffer) in drained_queue {
        let io = ActionIO::new_input(data_buffer);
        let function = &mut stage.function;
        let output = (function)(io, world).consume_raw();

        result_events.push(ActionStageCompletionEvent {
            module_name,
            action_name,
            current_stage,
            stage_output: output,
            stage_return: Some(ActionStage::Render(stage))
        });
    }

    for event in result_events {
        match event.stage_return {
            Some(ActionStage::Ecs(_)) => unreachable!("Expected Render stage return, got Ecs"),
            Some(ActionStage::EcsWhile(_)) => unreachable!("Expected Render stage return, got EcsWhile"),
            Some(ActionStage::Render(stage)) => {
                let sender = world.get_resource::<ActionStageCompletionEventSenderRender>().unwrap();
                let _ = sender.0.send((event.module_name, event.action_name, event.current_stage, stage, event.stage_output));
            },
            Some(ActionStage::RenderWhile(_)) => unreachable!("Expected Render stage return, got RenderWhile"),
            Some(ActionStage::Async(_)) => unreachable!("Expected Render stage return, got Async"),
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

    for (module_name, action_name, current_stage, mut stage, data_buffer) in drained_queue {
        let io = ActionIO::new_input(data_buffer);
        let function = &mut stage.function;
        let future = (function)(io);

        let sender = world
            .get_resource::<ActionStageCompletionEventSenderAsync>()
            .unwrap()
            .0
            .clone();

        let cloned_module_name = module_name.clone();
        let cloned_action_name = action_name.clone();

        tokio::spawn(async move {
            let output = future.await.consume_raw();
            
            let _ = sender.send((
                cloned_module_name,
                cloned_action_name,
                current_stage,
                stage,
                output
            ));
        });
    }
}

// TODO: 3. Maybe: While resources are stolen, validation can not access those resources! Maybe we just literally remove and reinsert them?
pub(in super) fn action_request_system(world: &mut World) {
    let mut system_state: SystemState<(
        ResMut<ActionMap>, 
        ResMut<ActionTypeModuleRegistry>,
    )> = SystemState::new(world);
    let (
        mut action_map, 
        mut action_type_module_registry
    ) = system_state.get_mut(world);

    // TODO: 2. Duplicate to other relevant places: Rely less on std::mem::take/replace and more on optional resource queries
    let mut stolen_action_map = std::mem::take(&mut *action_map);
    let mut stolen_action_type_module_registry = std::mem::take(&mut *action_type_module_registry);

    for (module_name, actions) in stolen_action_map.map.iter_mut() {
        for (action_name, instance) in actions.iter_mut() {
            let action_type = stolen_action_type_module_registry.get_action_type_mut(module_name, action_name).unwrap();
            
            if let Some(instance) = instance {
                match instance.state {
                    ActionState::Requested => {
                        if DEBUG_LOGGING_ENABLED && module_name == DEBUG_ACTION_MODULE && action_name == DEBUG_ACTION_NAME {
                            debug!("Lifecycle Stage 2.1 @ {}: Action `{}` enters Secondary Validation.", instance.state.current_stage(), action_name);
                        }

                        let input = std::mem::replace(&mut instance.data_buffer, RawActionData::new(()));
                        match (action_type.secondary_validation)(ActionIO::<InputState>::new_input(input), world) {
                            Ok(input) => {
                                instance.data_buffer = input.consume_raw();
                            },
                            Err(err) => {
                                panic!(
                                    "Action validation error: Action '{}' in module '{}' failed secondary validation: {}",
                                    action_name, module_name, err
                                );
                            }
                        }

                        if DEBUG_LOGGING_ENABLED && module_name == DEBUG_ACTION_MODULE && action_name == DEBUG_ACTION_NAME {
                            debug!("Lifecycle Stage 2.2 @ {}: Secondary Validation Passed.", instance.state.current_stage());
                        }

                        instance.state = ActionState::Processing { current_stage: 0 };
                    },
                    ActionState::Processing { current_stage } => {
                        if instance.timeout_frames == 0 {
                            panic!(
                                "Action processing error: Action '{}' in module '{}' timed out at stage {}",
                                action_name, module_name, current_stage
                            );
                        }
                        instance.timeout_frames -= 1;
                    },
                    ActionState::Processed { .. } => {}
                }
            }
        }
    }

    let mut system_state: SystemState<(
        ResMut<ActionMap>, 
        ResMut<ActionTypeModuleRegistry>,
    )> = SystemState::new(world);
    let (
        mut action_map, 
        mut action_type_module_registry
    ) = system_state.get_mut(world);
    
    *action_map = stolen_action_map;
    *action_type_module_registry = stolen_action_type_module_registry;
}

pub(in super) fn action_stage_execution_cleanup_system(world: &mut World) {
    let completed_executions = find_completed_action_stages(world);
    cleanup_completed_action_stages(world, completed_executions);
}

fn find_completed_action_stages(world: &mut World) -> Vec<(String, String, usize, ActionStage, RawActionData)> {
    let mut system_state: SystemState<(
        ResMut<ActionTypeModuleRegistry>, 
        ResMut<ActionMap>, 
    )> = SystemState::new(world);
    let (
        mut module_name_registry, 
        mut action_map, 
    ) = system_state.get_mut(world);

    let mut actions_to_process = Vec::new();

    for (module_name, actions) in action_map.map.iter_mut() {
        for (action_name, instance) in actions.iter_mut() {
            if let Some(instance) = instance {
                /* TODO: 4. Maybe: For all Non-Ecs Stages: This can not distinguish between a fresh while stage and a while stage that has already been "polled".
                *           We could add a "is_looping" to alongside "current_stage" to distinguish between the two,
                *           and non-while actions would just have "is_looping" set to false from the beginning 
                *           and also have "is_looping" completely ignored in non-while stages' execution.
                *           This inability to distinguish results in repeated polling of while stage, but not normal polling but polling as if the stage was fresh.
                */
                if let ActionState::Processing { current_stage } = &instance.state {
                    let action_type = module_name_registry
                        .get_action_type_mut(module_name, action_name)
                        .unwrap();

                    let stage = std::mem::take(&mut action_type.stages[*current_stage]).unwrap();

                    actions_to_process.push((
                        module_name.clone(),
                        action_name.clone(),
                        *current_stage,
                        stage,
                        std::mem::replace(&mut instance.data_buffer, RawActionData::new(())),
                    ));
                }
            }
        }
    }

    actions_to_process
}

fn cleanup_completed_action_stages(
    world: &mut World,
    mut actions_to_process: Vec<(String, String, usize, ActionStage, RawActionData)>,
) {
    let mut system_state: SystemState<(
        ResMut<ActionTypeModuleRegistry>, 
        ResMut<ActionMap>, 
        Res<ActionStageCompletionEventSenderEcs>,
        Res<ActionStageCompletionEventSenderEcsWhile>,
        Res<ActionStageCompletionEventSenderRender>,
        Res<ActionStageCompletionEventSenderRenderWhile>,
        Res<ActionStageCompletionEventSenderAsync>,
    )> = SystemState::new(world);
    let (
        mut module_name_registry, 
        mut action_map, 
        ecs_sender,
        ecs_while_sender,
        render_sender,
        render_while_sender,
        async_sender
    ) = system_state.get_mut(world);

    let async_sender = (*async_sender).0.clone();

    for (module_name, action_name, current_stage, mut stage, data_buffer) in actions_to_process.drain(..) {
        if DEBUG_LOGGING_ENABLED && module_name == DEBUG_ACTION_MODULE && action_name == DEBUG_ACTION_NAME {
            debug!(
                "Lifecycle Stage 3.1 @ {}: Starting execution of Stage for `{}` in module `{}`.",
                current_stage, action_name, module_name
            );
        }

        match stage {
            // **ECS Stage: Runs in immediate ECS context**
            ActionStage::Ecs(ref mut ecs_stage) => {
                if DEBUG_LOGGING_ENABLED && module_name == DEBUG_ACTION_MODULE && action_name == DEBUG_ACTION_NAME {
                    debug!(
                        "Lifecycle Stage 3.2 @ {}: Running ECS stage `{}` for `{}` in module `{}`.",
                        current_stage, ecs_stage.name, action_name, module_name
                    );
                }

                let ecs_stage = std::mem::replace(ecs_stage, ActionStageEcs {
                    name: "placeholder".to_string(),
                    function: Box::new(|_, _| unreachable!()),
                });

                let mut ecs_stage_queue = SystemState::<ResMut<EcsStageCompletionEventQueue>>::new(world).get_mut(world);
                ecs_stage_queue.0.push((module_name.clone(), action_name.clone(), current_stage, ecs_stage, data_buffer));

                // TODO: Legcay code for stage 'Ecs'
                //let io = ActionIO::new_input(data_buffer);
                //let function = &mut ecs_stage.function;
                //let output = (function)(io, world).consume_raw();

                //let cloned_module_name = module_name.clone();
                //let cloned_action_name = action_name.clone();

                //ecs_stage_completions.push((
                //    cloned_module_name,
                //    cloned_action_name,
                //    current_stage,
                //    output,
                //));
            }

            // **EcsWhile Stage: Loops in immediate ECS context until a condition is met**
            ActionStage::EcsWhile(ref mut ecs_while_stage) => {
                if DEBUG_LOGGING_ENABLED && module_name == DEBUG_ACTION_MODULE && action_name == DEBUG_ACTION_NAME {
                    debug!(
                        "Lifecycle Stage 3.2 @ {}: Running ECS While stage `{}` for `{}` in module `{}`.",
                        current_stage, ecs_while_stage.name, action_name, module_name
                    );
                }

                let ecs_while_stage = std::mem::replace(ecs_while_stage, ActionStageEcsWhile {
                    name: "placeholder".to_string(),
                    function: Box::new(|_, _| unreachable!()),
                });

                let mut ecs_while_stage_queue = SystemState::<ResMut<EcsWhileStageCompletionEventQueue>>::new(world).get_mut(world);
                ecs_while_stage_queue.0.push((module_name.clone(), action_name.clone(), current_stage, ecs_while_stage, data_buffer));

                // TODO: Legcay code for stage 'EcsWhile'
                //let io = ActionIO::new_input(data_buffer);
                //let function = &mut ecs_while_stage.function;
                //
                //let cloned_module_name = module_name.clone();
                //let cloned_action_name = action_name.clone();
                //
                //match (function)(io, world) {
                //    ActionStageEcsWhileOutcome::Waiting(input) => {
                //        let mut action_map = SystemState::<ResMut<ActionMap>>::new(world).get_mut(world);
                //        if let Some(actions) = action_map.map.get_mut(&module_name) {
                //            if let Some(instance) = actions.get_mut(&action_name).and_then(|a| a.as_mut()) {
                //                instance.data_buffer = input.consume_raw();
                //                instance.timeout_frames = instance.num_stages * 30; // Reset timeout
                //            }
                //        }
                //    },
                //    ActionStageEcsWhileOutcome::Completed(output) => {
                //        ecs_stage_completions.push((
                //            cloned_module_name,
                //            cloned_action_name,
                //            current_stage,
                //            output.consume_raw(),
                //        ));
                //    }
                //}
            }

            // TODO: Make these stages outcome-based too, so we can immediately return some sort of outcome, even if it is 'ask me later', and POSSIBLY add an error variant to the outcome
            // **ECS Render Stage → Queue for RenderApp**
            ActionStage::Render(ref mut render_stage) => {
                if DEBUG_LOGGING_ENABLED && module_name == DEBUG_ACTION_MODULE && action_name == DEBUG_ACTION_NAME {
                    debug!(
                        "Lifecycle Stage 3.2 @ {}: Running ECS Render stage `{}` for `{}` in module `{}`.",
                        current_stage, render_stage.name, action_name, module_name
                    );
                }

                let render_stage = std::mem::replace(render_stage, ActionStageRender {
                    name: "placeholder".to_string(),
                    function: Box::new(|_, _| unreachable!()),
                });
                
                let mut render_stage_queue = SystemState::<ResMut<RenderStageCompletionEventQueue>>::new(world).get_mut(world);
                render_stage_queue.0.push((module_name.clone(), action_name.clone(), current_stage, render_stage, data_buffer));
            }

            // TODO: Make these stages outcome-based too, so we can immediately return some sort of outcome, even if it is 'ask me later', and POSSIBLY add an error variant to the outcome
            // **ECS RenderWhile Stage → Queue for RenderApp (Retries Until Completion)**
            ActionStage::RenderWhile(ref mut render_while_stage) => {
                if DEBUG_LOGGING_ENABLED && module_name == DEBUG_ACTION_MODULE && action_name == DEBUG_ACTION_NAME {
                    debug!(
                        "Lifecycle Stage 3.2 @ {}: Running ECS Render While stage `{}` for `{}` in module `{}`.",
                        current_stage, render_while_stage.name, action_name, module_name
                    );
                }

                let render_while_stage = std::mem::replace(render_while_stage, ActionStageRenderWhile {
                    name: "placeholder".to_string(),
                    function: Box::new(|_, _| unreachable!()),
                });

                let mut render_while_stage_queue = SystemState::<ResMut<RenderWhileStageCompletionEventQueue>>::new(world).get_mut(world);
                render_while_stage_queue.0.push((module_name.clone(), action_name.clone(), current_stage, render_while_stage, data_buffer));
            }

            // TODO: Make these stages outcome-based too, so we can immediately return some sort of outcome, even if it is 'ask me later', and POSSIBLY add an error variant to the outcome
            // **Async Stage: Runs non-blocking in a separate task**
            ActionStage::Async(ref mut async_stage) => {
                if DEBUG_LOGGING_ENABLED && module_name == DEBUG_ACTION_MODULE && action_name == DEBUG_ACTION_NAME {
                    debug!(
                        "Lifecycle Stage 3.2 @ {}: Running Async stage `{}` for `{}` in module `{}`.",
                        current_stage, async_stage.name, action_name, module_name
                    );
                }

                let async_stage = std::mem::replace(async_stage, ActionStageAsync {
                    name: "placeholder".to_string(),
                    function: Box::new(|_| unreachable!()),
                });

                let mut async_stage_queue = SystemState::<ResMut<AsyncStageCompletionEventQueue>>::new(world).get_mut(world);
                async_stage_queue.0.push((module_name.clone(), action_name.clone(), current_stage, async_stage, data_buffer));

                // TODO: Legcay code for stage 'Async'
                //let io = ActionIO::new_input(data_buffer);
                //let function = &mut async_stage.function;
                //let future = (function)(io);
                //
                //let cloned_module_name = module_name.clone();
                //let cloned_action_name = action_name.clone();
                //
                //tokio::spawn(async move {
                //    let output = future.await.consume_raw();
                //    
                //    async_sender
                //        .send(ActionStageProcessedEvent {
                //            module_name: cloned_module_name,
                //            action_name: cloned_action_name,
                //            current_stage,
                //            stage_output: output,
                //            stage_return: None,
                //        })
                //        .unwrap();
                //});
            }
        }
    }
}

pub(in super) fn action_progression_system(world: &mut World) {
    let mut system_state: SystemState<(
        ResMut<ActionMap>, 
        ConsumableEventReader<ActionStageCompletionEvent>,
        ResMut<ActionTypeModuleRegistry>
    )> = SystemState::new(world);
    let (
        mut action_map, 
        mut stage_event_reader,
        mut action_type_module_registry,
    ) = system_state.get_mut(world);

    let mut completed_stages = Vec::new();
    let mut completed_actions = Vec::new();
    let mut callbacks = Vec::new();

    for event in stage_event_reader.read() {
        let event = event.consume();

        if DEBUG_LOGGING_ENABLED && event.module_name == DEBUG_ACTION_MODULE && event.action_name == DEBUG_ACTION_NAME {
            debug!(
                "Lifecycle Stage 3.3 @ {}: Stage completion event received for `{}` in module `{}`",
                event.current_stage, event.action_name, event.module_name
            );
        }

        if let Some(actions) = action_map.map.get_mut(&event.module_name) {
            if let Some(instance) = actions.get_mut(&event.action_name).and_then(|a| a.as_mut()) {
                match &mut instance.state {
                    ActionState::Processing { current_stage } => {
                        instance.data_buffer = event.stage_output;
                        instance.timeout_frames = instance.num_stages * 30;
                        completed_stages.push((event.module_name.clone(), event.action_name.clone(), *current_stage));

                        if *current_stage + 1 >= instance.num_stages {
                            completed_actions.push((event.module_name.clone(), event.action_name.clone()));
                        }
                    }
                    state => unreachable!("Unexpected state transition to {:?}", state),
                };

                if event.stage_return.is_some() {
                    let action_type = action_type_module_registry
                        .get_action_type_mut(&event.module_name, &event.action_name)
                        .unwrap();

                    action_type.stages[event.current_stage] = event.stage_return;
                }
            }
        }
    }

    // Finalize completed stages
    for (module_name, action_name, current_stage) in completed_stages {
        if let Some(actions) = action_map.map.get_mut(&module_name) {
            if let Some(instance) = actions.get_mut(&action_name).and_then(|a| a.as_mut()) {
                instance.state = ActionState::Processing { current_stage: current_stage + 1 };
            }

            if DEBUG_LOGGING_ENABLED && module_name == DEBUG_ACTION_MODULE && action_name == DEBUG_ACTION_NAME {
                debug!(
                    "Lifecycle Stage 3.4 @ {}: Stage completion handled for `{}` in module `{}`",
                    current_stage, action_name, module_name
                );
            }
        }
    }

    // Finalize completed actions
    for (module_name, action_name) in completed_actions {
        if let Some(actions) = action_map.map.get_mut(&module_name) {
            if let Some(instance) = actions.remove(&action_name).flatten() {
                callbacks.push((instance.callback, instance.data_buffer));
            }
        }

        if DEBUG_LOGGING_ENABLED && module_name == DEBUG_ACTION_MODULE && action_name == DEBUG_ACTION_NAME {
            debug!(
                "Lifecycle Stage 3.5 @ x: Action `{}` in module `{}` completed.",
                action_name, module_name
            );
        }
    }

    // Execute callbacks
    for (callback, data) in callbacks {
        if let Some(callback) = callback {
            let io = ActionIO::new_callback_data(data);
            callback(world, io);
        }
    }
}

