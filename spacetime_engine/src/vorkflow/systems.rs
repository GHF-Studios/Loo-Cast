use bevy::prelude::*;
use bevy::ecs::system::SystemState;
use bevy::render::MainWorld;
use bevy_consumable_event::{ConsumableEventReader, ConsumableEventWriter};

use crate::{config::statics::CONFIG, gpu::vorkflows::setup_texture_generator, statics::TOKIO_RUNTIME};

use super::{
    events::{VorkflowStageCompletionEvent, VorkflowStageInitializationEvent}, resources::{VorkflowMap, VorkflowTypeModuleRegistry}, stage::{VorkflowStage, VorkflowStageEcs, VorkflowStageWhileOutcome}, io::{InputState, VorkflowIO}, types::{RawVorkflowData, VorkflowState}, AsyncStageBuffer, AsyncStageCompletionEventReceiver, AsyncStageCompletionEventSender, EcsStageBuffer, EcsStageCompletionEventReceiver, EcsStageCompletionEventSender, EcsWhileStageBuffer, EcsWhileStageCompletionEventReceiver, EcsWhileStageCompletionEventSender, RenderStageBuffer, RenderStageCompletionEventReceiver, RenderStageCompletionEventSender, RenderWhileStageBuffer, RenderWhileStageCompletionEventReceiver, RenderWhileStageCompletionEventSender, DEBUG_ACTION_MODULE, DEBUG_ACTION_NAME, DEBUG_LOGGING_ENABLED
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

    for (module_name, vorkflow_name, current_stage, mut stage, data_buffer) in drained_buffer {
        let io = VorkflowIO::new_input(data_buffer);
        let function = &mut stage.function;
        let output = (function)(io, world).consume_raw();

        let sender = world
            .get_resource::<EcsStageCompletionEventSender>()
            .unwrap()
            .0
            .clone();

        let cloned_module_name = module_name.clone();
        let cloned_vorkflow_name = vorkflow_name.clone();

        let output_send_result = sender.send((cloned_module_name, cloned_vorkflow_name, current_stage, stage, output));

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

    for (module_name, vorkflow_name, current_stage, mut stage, data_buffer) in drained_buffer {
        let io = VorkflowIO::new_input(data_buffer);
        let function = &mut stage.function;
        let outcome = (function)(io, world);

        match outcome {
            VorkflowStageWhileOutcome::Waiting(io) => {
                waiting_buffer.push((module_name, vorkflow_name, current_stage, stage, io.consume_raw()));
            },
            VorkflowStageWhileOutcome::Completed(io) => {
                let output = io.consume_raw();

                let sender = world
                    .get_resource::<EcsWhileStageCompletionEventSender>()
                    .unwrap()
                    .0
                    .clone();

                let cloned_module_name = module_name.clone();
                let cloned_vorkflow_name = vorkflow_name.clone();

                let output_send_result = sender.send((cloned_module_name, cloned_vorkflow_name, current_stage, stage, output));

                if let Err(err) = output_send_result {
                    unreachable!("Ecs while stage completion error: Output send error: {}", err);
                }
            }
        }
    }

    let mut buffer = world.resource_mut::<EcsWhileStageBuffer>();
    buffer.0 = waiting_buffer;
}

pub(in super) fn poll_render_stage_buffer_system(world: &mut World) {
    let drained_buffer = {
        let mut buffer = world.resource_mut::<RenderStageBuffer>();
        std::mem::take(&mut buffer.0)
    };

    for (module_name, vorkflow_name, current_stage, mut stage, data_buffer) in drained_buffer {
        let io = VorkflowIO::new_input(data_buffer);
        let function = &mut stage.function;
        let output = (function)(io, world).consume_raw();

        let sender = world
            .get_resource::<RenderStageCompletionEventSender>()
            .unwrap()
            .0
            .clone();

        let cloned_module_name = module_name.clone();
        let cloned_vorkflow_name = vorkflow_name.clone();

        let output_send_result = sender.send((cloned_module_name, cloned_vorkflow_name, current_stage, stage, output));

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

    for (module_name, vorkflow_name, current_stage, mut stage, data_buffer) in drained_buffer {
        let io = VorkflowIO::new_input(data_buffer);
        let function = &mut stage.function;
        let outcome = (function)(io, world);

        match outcome {
            VorkflowStageWhileOutcome::Waiting(io) => {
                waiting_buffer.push((module_name, vorkflow_name, current_stage, stage, io.consume_raw()));
            },
            VorkflowStageWhileOutcome::Completed(io) => {
                let output = io.consume_raw();

                let sender = world
                    .get_resource::<RenderWhileStageCompletionEventSender>()
                    .unwrap()
                    .0
                    .clone();

                let cloned_module_name = module_name.clone();
                let cloned_vorkflow_name = vorkflow_name.clone();

                let output_send_result = sender.send((cloned_module_name, cloned_vorkflow_name, current_stage, stage, output));

                if let Err(err) = output_send_result {
                    unreachable!("Render while stage completion error: Output send error: {}", err);
                }
            }
        }
    }

    let mut buffer = world.resource_mut::<RenderWhileStageBuffer>();
    buffer.0 = waiting_buffer;
}

pub(in super) fn poll_async_stage_buffer_system(world: &mut World) {
    let drained_buffer = {
        let mut buffer = world.resource_mut::<AsyncStageBuffer>();
        std::mem::take(&mut buffer.0)
    };

    for (module_name, vorkflow_name, current_stage, mut stage, data_buffer) in drained_buffer {
        let io = VorkflowIO::new_input(data_buffer);
        let function = &mut stage.function;
        let future = (function)(io);

        let sender = world
            .get_resource::<AsyncStageCompletionEventSender>()
            .unwrap()
            .0
            .clone();

        let cloned_module_name = module_name.clone();
        let cloned_vorkflow_name = vorkflow_name.clone();

        let task_spawn_result = TOKIO_RUNTIME.lock().unwrap().block_on(async {
            tokio::spawn(async move {
                let output = future.await.consume_raw();
                
                let output_send_result = sender.send((cloned_module_name, cloned_vorkflow_name, current_stage, stage, output));

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
    mut stage_event_writer: ConsumableEventWriter<VorkflowStageCompletionEvent>,
) {
    while let Ok((module_name, vorkflow_name, current_stage, stage, stage_output)) = stage_event_receiver.0.try_recv() {
        stage_event_writer.send(VorkflowStageCompletionEvent {
            module_name: module_name.clone(),
            vorkflow_name: vorkflow_name.clone(),
            current_stage,
            stage_output,
            stage_return: VorkflowStage::Ecs(stage),
        });
    }
}

pub(in super) fn handle_ecs_while_stage_completion_event_system(
    stage_event_receiver: Res<EcsWhileStageCompletionEventReceiver>,
    mut stage_event_writer: ConsumableEventWriter<VorkflowStageCompletionEvent>,
) {
    while let Ok((module_name, vorkflow_name, current_stage, stage, stage_output)) = stage_event_receiver.0.try_recv() {
        stage_event_writer.send(VorkflowStageCompletionEvent {
            module_name: module_name.clone(),
            vorkflow_name: vorkflow_name.clone(),
            current_stage,
            stage_output,
            stage_return: VorkflowStage::EcsWhile(stage),
        });
    }
}

pub(in super) fn handle_render_stage_completion_event_system(
    stage_event_receiver: Res<RenderStageCompletionEventReceiver>,
    mut stage_event_writer: ConsumableEventWriter<VorkflowStageCompletionEvent>, 
) {
    while let Ok((module_name, vorkflow_name, current_stage, stage, stage_output)) = stage_event_receiver.0.try_recv() {
        stage_event_writer.send(VorkflowStageCompletionEvent {
            module_name: module_name.clone(),
            vorkflow_name: vorkflow_name.clone(),
            current_stage,
            stage_output,
            stage_return: VorkflowStage::Render(stage),
        });
    }
}

pub(in super) fn handle_render_while_stage_completion_event_system(
    stage_event_receiver: Res<RenderWhileStageCompletionEventReceiver>,
    mut stage_event_writer: ConsumableEventWriter<VorkflowStageCompletionEvent>, 
) {
    while let Ok((module_name, vorkflow_name, current_stage, stage, stage_output)) = stage_event_receiver.0.try_recv() {
        stage_event_writer.send(VorkflowStageCompletionEvent {
            module_name: module_name.clone(),
            vorkflow_name: vorkflow_name.clone(),
            current_stage,
            stage_output,
            stage_return: VorkflowStage::RenderWhile(stage),
        });
    }
}

pub(in super) fn handle_async_stage_completion_event_system(
    stage_event_receiver: ResMut<AsyncStageCompletionEventReceiver>,
    mut stage_event_writer: ConsumableEventWriter<VorkflowStageCompletionEvent>, 
) {
    while let Ok((module_name, vorkflow_name, current_stage, stage, stage_output)) = stage_event_receiver.0.try_recv() {
        stage_event_writer.send(VorkflowStageCompletionEvent {
            module_name: module_name.clone(),
            vorkflow_name: vorkflow_name.clone(),
            current_stage,
            stage_output,
            stage_return: VorkflowStage::Async(stage),
        });
    }
}

// TODO: Maybe: While resources are stolen, validation can not access those resources! Maybe we just literally remove and reinsert them? Or it's just the case that they are not available at secondary validation due to this exact reasons, but then we need to explicitly document that somewhere/somehow.
pub(in super) fn vorkflow_request_system(world: &mut World) {
    let mut system_state: SystemState<(
        ResMut<VorkflowMap>, 
        ResMut<VorkflowTypeModuleRegistry>,
        ConsumableEventWriter<VorkflowStageInitializationEvent>,
    )> = SystemState::new(world);
    let (
        mut vorkflow_map, 
        mut vorkflow_type_module_registry,
        _
    ) = system_state.get_mut(world);

    // TODO: Duplicate to other relevant places: Rely less on std::mem::take/replace and more on optional resource queries
    let mut stolen_vorkflow_map = std::mem::take(&mut *vorkflow_map);
    let mut stolen_vorkflow_type_module_registry = std::mem::take(&mut *vorkflow_type_module_registry);

    let mut stage_initialization_events = Vec::new();

    for (module_name, vorkflows) in stolen_vorkflow_map.map.iter_mut() {
        for (vorkflow_name, instance) in vorkflows.iter_mut() {
            let vorkflow_type = stolen_vorkflow_type_module_registry.get_vorkflow_type_mut(module_name, vorkflow_name).unwrap();
            
            if let Some(instance) = instance {
                match instance.state {
                    VorkflowState::Requested => {
                        let input = std::mem::replace(&mut instance.data_buffer, RawVorkflowData::new(()));
                        let input = match (vorkflow_type.secondary_validation)(VorkflowIO::<InputState>::new_input(input), world) {
                            Ok(input) => {
                                input.consume_raw()
                            },
                            Err(err) => {
                                unreachable!(
                                    "Vorkflow validation error: Vorkflow '{}' in module '{}' failed secondary validation: {}",
                                    vorkflow_name, module_name, err
                                );
                            }
                        };

                        instance.state = VorkflowState::Processing { current_stage: 0, stage_completed: false };
                        stage_initialization_events.push(VorkflowStageInitializationEvent {
                            module_name: module_name.clone(),
                            vorkflow_name: vorkflow_name.clone(),
                            stage_input: input,
                        });
                    },
                    VorkflowState::Processing { .. } => {}
                }
            }
        }
    }

    let (
        mut vorkflow_map, 
        mut vorkflow_type_module_registry,
        mut stage_initialization_event_writer
    ) = system_state.get_mut(world);
    
    *vorkflow_map = stolen_vorkflow_map;
    *vorkflow_type_module_registry = stolen_vorkflow_type_module_registry;

    for event in stage_initialization_events {
        stage_initialization_event_writer.send(event);
    }
}

pub(in super) fn vorkflow_execution_system(world: &mut World) {
    let mut system_state: SystemState<(
        ResMut<VorkflowMap>, 
        ResMut<VorkflowTypeModuleRegistry>,
        ResMut<EcsStageBuffer>,
        ResMut<EcsWhileStageBuffer>,
        ResMut<RenderStageBuffer>,
        ResMut<RenderWhileStageBuffer>,
        ResMut<AsyncStageBuffer>,
        ConsumableEventReader<VorkflowStageInitializationEvent>,
    )> = SystemState::new(world);
    let (
        mut vorkflow_map, 
        mut vorkflow_type_module_registry,
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
        let vorkflow_name = event.vorkflow_name;
        let stage_input = event.stage_input;

        if let Some(vorkflows) = vorkflow_map.map.get_mut(&module_name) {
            if let Some(instance) = vorkflows.get_mut(&vorkflow_name).and_then(|a| a.as_mut()) {
                let current_stage = instance.state.current_stage();

                let vorkflow_type = vorkflow_type_module_registry
                    .get_vorkflow_type_mut(&module_name, &vorkflow_name)
                    .unwrap();

                let stage = std::mem::replace(&mut vorkflow_type.stages[current_stage], VorkflowStage::Ecs(VorkflowStageEcs {
                    name: "placeholder".to_string(),
                    function: Box::new(|_, _| unreachable!()),
                }));

                match stage {
                    VorkflowStage::Ecs(stage) => {
                        ecs_stage_buffer.0.push((module_name.clone(), vorkflow_name.clone(), current_stage, stage, stage_input));
                    },
                    VorkflowStage::EcsWhile(stage) => {
                        ecs_while_stage_buffer.0.push((module_name.clone(), vorkflow_name.clone(), current_stage, stage, stage_input));
                    },
                    VorkflowStage::Render(stage) => {
                        render_stage_buffer.0.push((module_name.clone(), vorkflow_name.clone(), current_stage, stage, stage_input));
                    },
                    VorkflowStage::RenderWhile(stage) => {
                        render_while_stage_buffer.0.push((module_name.clone(), vorkflow_name.clone(), current_stage, stage, stage_input));
                    },
                    VorkflowStage::Async(stage) => {
                        async_stage_buffer.0.push((module_name.clone(), vorkflow_name.clone(), current_stage, stage, stage_input));
                    },
                }
            } else {
                unreachable!("Vorkflow instance not found for module '{}' and name '{}'", module_name, vorkflow_name);
            }
        } else {
            unreachable!("Vorkflow instance module not found for name '{}'", module_name);
        }
    }
}

pub(in super) fn vorkflow_completion_handling_system(world: &mut World) {
    let mut system_state: SystemState<(
        ResMut<VorkflowMap>, 
        ResMut<VorkflowTypeModuleRegistry>,
        ConsumableEventReader<VorkflowStageCompletionEvent>,
        ConsumableEventWriter<VorkflowStageInitializationEvent>,
    )> = SystemState::new(world);
    let (
        mut vorkflow_map, 
        mut vorkflow_type_module_registry,
        mut stage_completion_event_reader,
        mut stage_initialization_event_writer,
    ) = system_state.get_mut(world);

    let mut intermediate_stage_completions = Vec::new();
    let mut final_stage_completions = Vec::new();

    for event in stage_completion_event_reader.read() {
        let event = event.consume();
        let module_name = event.module_name;
        let vorkflow_name = event.vorkflow_name;
        let current_stage = event.current_stage;
        let stage_output = event.stage_output;
        let stage_return = event.stage_return;

        if let Some(vorkflows) = vorkflow_map.map.get_mut(&module_name) {
            if let Some(instance) = vorkflows.get_mut(&vorkflow_name).and_then(|a| a.as_mut()) {
                match &mut instance.state {
                    VorkflowState::Processing { current_stage: other_current_stage, stage_completed: completed } => {
                        if current_stage != *other_current_stage {
                            unreachable!("Unexpected vorkflow state. Completion event is at stage '{}', but the vorkflow instance is at stage '{}'", current_stage, other_current_stage);
                        }
                        if *completed {
                            unreachable!("Unexpected vorkflow state. Vorkflow '{}' in module '{}' is already completed.", vorkflow_name, module_name);
                        }
                        
                        *completed = true;
                    },
                    state => unreachable!("Unexpected vorkflow state. Expected 'VorkflowState::Processing(_)', got '{:?}'", state),
                };

                let vorkflow_type = vorkflow_type_module_registry
                    .get_vorkflow_type_mut(&module_name, &vorkflow_name)
                    .unwrap();

                vorkflow_type.stages[current_stage] = stage_return;

                if current_stage + 1 < vorkflow_type.stages.len() {
                    intermediate_stage_completions.push((module_name.clone(), vorkflow_name.clone(), current_stage, stage_output));
                } else {
                    final_stage_completions.push((module_name.clone(), vorkflow_name.clone(), std::mem::take(&mut instance.callback), stage_output));
                }
            }
        }
    }

    // Handle intermediate stage completions
    for (module_name, vorkflow_name, current_stage, stage_output) in intermediate_stage_completions {
        if let Some(vorkflows) = vorkflow_map.map.get_mut(&module_name) {
            if let Some(instance) = vorkflows.get_mut(&vorkflow_name).and_then(|a| a.as_mut()) {
                instance.state = VorkflowState::Processing { current_stage: current_stage + 1, stage_completed: false };

                stage_initialization_event_writer.send(VorkflowStageInitializationEvent {
                    module_name: module_name.clone(),
                    vorkflow_name: vorkflow_name.clone(),
                    stage_input: stage_output,
                });
            }
        }
    }

    // Handle final stage completions
    let mut callbacks = Vec::new();
    for (module_name, vorkflow_name, callback, stage_output) in final_stage_completions {
        if let Some(vorkflows) = vorkflow_map.map.get_mut(&module_name) {
            vorkflows.remove(&vorkflow_name);
            callbacks.push((callback, stage_output));
        }
    }
    for (callback, data) in callbacks {
        if let Some(callback) = callback {
            let io = VorkflowIO::new_callback_data(data);
            callback(world, io);
        }
    }
}
