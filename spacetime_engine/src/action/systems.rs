use std::any::Any;
use bevy::prelude::*;
use bevy::ecs::system::SystemState;
use bevy_consumable_event::{ConsumableEventReader, ConsumableEventWriter};
use crossbeam_channel::Sender;

use super::{
    events::ActionStageProcessedEvent,
    resources::{ActionMap, ActionTypeModuleRegistry},
    stage::{ActionStage, ActionStageEcs},
    stage_io::{ActionIO, CallbackState, InputState},
    types::{ActionState, ActionType, RawActionData},
    ActionStageProcessedMessageReceiver, ActionStageProcessedMessageSender,
};

pub(in super) fn async_stage_event_relay_system(
    receiver: ResMut<ActionStageProcessedMessageReceiver>,
    mut action_event_writer: ConsumableEventWriter<ActionStageProcessedEvent>, 
) {
    while let Ok(event) = receiver.0.try_recv() {
        action_event_writer.send(event);
    }
}

pub(in super) fn action_processing_system(world: &mut World) {
    process_active_actions(world);
    let completed_actions = process_stage_events(world);
    finalize_completed_actions(world, completed_actions);
}

// TODO: While resources are stolen, validation can not access those resources! Maybe we just literally remove and reinsert them?
fn process_active_actions(world: &mut World) {
    let mut system_state: SystemState<(
        ResMut<ActionMap>, 
        ResMut<ActionTypeModuleRegistry>,
    )> = SystemState::new(world);
    let (
        mut action_map, 
        mut action_type_module_registry
    ) = system_state.get_mut(world);

    let mut stolen_action_map = std::mem::take(&mut *action_map);
    let mut stolen_action_type_module_registry = std::mem::take(&mut *action_type_module_registry);

    for (module_name, actions) in stolen_action_map.map.iter_mut() {
        for (action_name, instance) in actions.iter_mut() {
            let action_type = stolen_action_type_module_registry.get_action_type_mut(module_name, action_name).unwrap();
            if let Some(instance) = instance {
                match instance.state {
                    ActionState::Requested => {
                        let input = std::mem::replace(&mut instance.data_buffer, RawActionData::new(()));
                        match (action_type.secondary_validation)(ActionIO::<InputState>::new_input(input), world) {
                            Ok(input) => {
                                let _ = std::mem::replace(&mut instance.data_buffer, input.consume_raw());
                            },
                            Err(err) => {
                                panic!(
                                    "Action validation error: Action '{}' in module '{}' failed secondary validation: {}",
                                    action_name, module_name, err
                                );
                            }
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

fn process_stage_events(world: &mut World) -> Vec<(String, String)> {
    let mut system_state: SystemState<(
        ResMut<ActionMap>, 
        ConsumableEventReader<ActionStageProcessedEvent>,
    )> = SystemState::new(world);
    let (
        mut action_map, 
        mut stage_event_reader,
    ) = system_state.get_mut(world);

    let mut completed_actions = Vec::new();

    for event in stage_event_reader.read() {
        let event = event.consume();
        debug!(
            "Action '{}' in module '{}' completed stage {}: Phase 2",
            event.action_name, event.module_name, event.stage_index
        );
        if let Some(actions) = action_map.map.get_mut(&event.module_name) {
            if let Some(instance) = actions.get_mut(&event.action_name).and_then(|a| a.as_mut()) {
                match &mut instance.state {
                    ActionState::Processing { current_stage } => {
                        *current_stage += 1;

                        if *current_stage < instance.num_stages {
                            instance.timeout_frames = instance.num_stages * 30;
                        } else {
                            completed_actions.push((event.module_name.clone(), event.action_name.clone()));
                        }

                        instance.data_buffer = event.stage_output;
                    }
                    _ => unreachable!("Unexpected state transition"),
                };
            }
        }
    }

    completed_actions
}

fn finalize_completed_actions(
    world: &mut World,
    completed_actions: Vec<(String, String)>,
) {
    let mut system_state: SystemState<ResMut<ActionMap>> = SystemState::new(world);
    let mut action_map = system_state.get_mut(world);

    let mut callbacks = Vec::new();

    for (module_name, action_name) in completed_actions {
        if let Some(actions) = action_map.map.get_mut(&module_name) {
            if let Some(instance) = actions.remove(&action_name).flatten() {
                callbacks.push((instance.callback, instance.data_buffer));
            }
        }
    }

    for (callback, data) in callbacks {
        if let Some(callback) = callback {
            let io = ActionIO::new_callback_data(RawActionData::new(data));
            callback(world, io);
        }
    }
}

pub(in super) fn action_execution_system(world: &mut World) {
    let (actions_to_process, async_sender) = collect_action_data(world);
    progress_actions(world, actions_to_process, async_sender);
}

fn collect_action_data(world: &mut World) -> (Vec<(String, String, usize, ActionStage, RawActionData)>, Sender<ActionStageProcessedEvent>) {
    let mut system_state: SystemState<(ResMut<ActionMap>, ResMut<ActionTypeModuleRegistry>, Res<ActionStageProcessedMessageSender>)> = SystemState::new(world);
    let (mut action_map, mut module_name_registry, async_sender) = system_state.get_mut(world);

    let mut actions_to_process = Vec::new();

    for (module_name, actions) in action_map.map.iter_mut() {
        for (action_name, instance) in actions.iter_mut() {
            if let Some(instance) = instance {
                if let ActionState::Processing { current_stage } = &instance.state {
                    let action_type = module_name_registry
                        .get_action_type_mut(module_name, action_name)
                        .unwrap();

                    let stage = std::mem::replace(
                        &mut action_type.stages[*current_stage],
                        ActionStage::Ecs(ActionStageEcs {
                            name: "placeholder".to_string(),
                            function: Box::new(|_, _| unreachable!()),
                        }),
                    );

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

    (actions_to_process, async_sender.0.clone())
}

fn progress_actions(
    world: &mut World,
    mut actions_to_process: Vec<(String, String, usize, ActionStage, RawActionData)>,
    async_sender: Sender<ActionStageProcessedEvent>,
) {
    let mut stage_outputs = Vec::new();

    for (module_name, action_name, current_stage, mut stage, data_buffer) in actions_to_process.drain(..) {
        let sender = async_sender.clone();

        match stage {
            // **ECS Stage: Runs in immediate ECS context**
            ActionStage::Ecs(ref mut ecs_stage) => {
                let io = ActionIO::new_input(data_buffer);
                let function = &mut ecs_stage.function;
                let output = (function)(io, world).consume_raw();

                let cloned_module_name = module_name.clone();
                let cloned_action_name = action_name.clone();

                stage_outputs.push((
                    cloned_module_name,
                    cloned_action_name,
                    current_stage,
                    output,
                ));
            }

            // **Async Stage: Runs in a separate task**
            ActionStage::Async(ref mut async_stage) => {
                let io = ActionIO::new_input(data_buffer);
                let function = &mut async_stage.function;
                let future = (function)(io);

                let cloned_module_name = module_name.clone();
                let cloned_action_name = action_name.clone();

                tokio::spawn(async move {
                    let output = future.await.consume_raw();
                    sender
                        .send(ActionStageProcessedEvent {
                            module_name: cloned_module_name,
                            action_name: cloned_action_name,
                            stage_index: current_stage,
                            stage_output: output,
                        })
                        .unwrap();
                });
            }

            // **EcsWhile Stage: Loops until a condition is met**
            ActionStage::EcsWhile(ref mut ecs_while_stage) => {
                let io = ActionIO::new_input(data_buffer);
                let function = &mut ecs_while_stage.function;

                let cloned_module_name = module_name.clone();
                let cloned_action_name = action_name.clone();

                match (function)(io, world) {
                    // **Condition met** → Stage is complete, output result
                    Err(output) => {
                        stage_outputs.push((
                            cloned_module_name,
                            cloned_action_name,
                            current_stage,
                            output.consume_raw(),
                        ));
                    }

                    // **Condition not met** → Loop again next frame
                    Ok(input) => {
                        let mut system_state: SystemState<ResMut<ActionMap>> = SystemState::new(world);
                        let mut action_map = system_state.get_mut(world);

                        if let Some(actions) = action_map.map.get_mut(&module_name) {
                            if let Some(instance) = actions.get_mut(&action_name).and_then(|a| a.as_mut()) {
                                instance.data_buffer = input.consume_raw();
                                instance.timeout_frames = instance.num_stages * 30; // Reset timeout
                            }
                        }
                    }
                }
            }
        }

        // Give back the stolen stage:
        let mut module_name_registry = SystemState::<ResMut<ActionTypeModuleRegistry>>::new(world).get_mut(world);

        let action_type = module_name_registry
            .get_action_type_mut(&module_name, &action_name)
            .unwrap();

        let _ = std::mem::replace(&mut action_type.stages[current_stage], stage);
    }

    apply_stage_outputs(world, stage_outputs);
}

fn apply_stage_outputs(
    world: &mut World,
    stage_outputs: Vec<(String, String, usize, RawActionData)>,
) {
    let mut system_state: SystemState<(
        ConsumableEventWriter<ActionStageProcessedEvent>, 
        ResMut<ActionMap>
    )> = SystemState::new(world);
    let (
        mut stage_event_writer, 
        mut action_map
    ) = system_state.get_mut(world);

    for (module_name, action_name, stage_index, stage_output) in stage_outputs {
        debug!(
            "Action '{}' in module '{}' completed stage {}: Phase 1",
            action_name, module_name, stage_index
        );
        stage_event_writer.send(ActionStageProcessedEvent {
            module_name: module_name.clone(),
            action_name: action_name.clone(),
            stage_index,
            stage_output,
        });

        if let Some(actions) = action_map.map.get_mut(&module_name) {
            if let Some(instance) = actions.get_mut(&action_name).and_then(|a| a.as_mut()) {
                if let ActionState::Processing { current_stage } = &instance.state {
                    if *current_stage == stage_index {
                        instance.state = ActionState::Processed { current_stage: *current_stage };
                    }
                }
            }
        }
    }
}
