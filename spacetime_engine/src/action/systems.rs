use std::any::Any;
use bevy::prelude::*;
use bevy::ecs::system::SystemState;
use crossbeam_channel::Sender;

use super::{
    events::ActionStageProcessedEvent,
    resources::{ActionMap, ActionTypeModuleRegistry},
    stage::{ActionStage, ActionStageEcs},
    stage_io::{ActionIO, CallbackState},
    types::{ActionState, RawActionData},
    ActionStageProcessedMessageReceiver, ActionStageProcessedMessageSender,
};

pub(in super) fn async_stage_event_relay_system(
    receiver: ResMut<ActionStageProcessedMessageReceiver>,
    mut action_event_writer: EventWriter<ActionStageProcessedEvent>, 
) {
    while let Ok(event) = receiver.0.try_recv() {
        action_event_writer.send(event);
    }
}

pub(in super) fn action_tick_system(world: &mut World) {
    let mut system_state: SystemState<(
        ResMut<ActionMap>, 
        EventReader<ActionStageProcessedEvent>
    )> = SystemState::new(world);
    let (mut action_map, mut stage_event_reader) = system_state.get_mut(world);

    process_active_actions(&mut action_map);
    let completed_actions = process_stage_events(&mut action_map, &mut stage_event_reader);
    finalize_completed_actions(world, completed_actions);
}

fn process_active_actions(action_map: &mut ActionMap) {
    for (module_name, actions) in action_map.map.iter_mut() {
        for (action_name, instance) in actions.iter_mut() {
            if let Some(instance) = instance {
                if instance.timeout_frames == 0 {
                    panic!(
                        "Action timeout error: Action '{}' in module '{}' exceeded execution time.",
                        action_name, module_name
                    );
                }
                instance.timeout_frames -= 1;
            }
        }
    }
}


fn process_stage_events(
    action_map: &mut ActionMap,
    stage_event_reader: &mut EventReader<ActionStageProcessedEvent>,
) -> Vec<(String, String)> {
    let mut completed_actions = Vec::new();

    for event in stage_event_reader.read() {
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
    process_actions(world, actions_to_process, async_sender);
}

fn collect_action_data(world: &mut World) -> (Vec<(String, String, usize, ActionStage, Box<dyn Any + Send + Sync>)>, Sender<ActionStageProcessedEvent>) {
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
                        std::mem::replace(&mut instance.data_buffer, Box::new(())),
                    ));
                }
            }
        }
    }

    (actions_to_process, async_sender.0.clone())
}

fn process_actions(
    world: &mut World,
    mut actions_to_process: Vec<(String, String, usize, ActionStage, Box<dyn Any + Send + Sync>)>,
    async_sender: Sender<ActionStageProcessedEvent>,
) {
    let mut stage_outputs = Vec::new();

    for (module_name, action_name, current_stage, mut stage, data_buffer) in actions_to_process.drain(..) {
        let sender = async_sender.clone();

        match stage {
            // **ECS Stage: Runs in immediate ECS context**
            ActionStage::Ecs(ref mut ecs_stage) => {
                let io = ActionIO::new_input(RawActionData::new(data_buffer));
                let function = &mut ecs_stage.function;
                let output = (function)(io, world).consume();

                stage_outputs.push((
                    module_name,
                    action_name,
                    current_stage,
                    output,
                ));
            }

            // **Async Stage: Runs in a separate task**
            ActionStage::Async(ref mut async_stage) => {
                let io = ActionIO::new_input(RawActionData::new(data_buffer));
                let function = &mut async_stage.function;
                let future = (function)(io);

                tokio::spawn(async move {
                    let output = future.await.consume();
                    sender
                        .send(ActionStageProcessedEvent {
                            module_name: module_name.to_string(),
                            action_name: action_name.to_string(),
                            stage_index: current_stage,
                            stage_output: output,
                        })
                        .unwrap();
                });
            }

            // **EcsWhile Stage: Loops until a condition is met**
            ActionStage::EcsWhile(ref mut ecs_while_stage) => {
                let io = ActionIO::new_input(RawActionData::new(data_buffer));
                let function = &mut ecs_while_stage.function;

                match (function)(io, world) {
                    // **Condition met** → Stage is complete, output result
                    Err(output) => {
                        stage_outputs.push((
                            module_name,
                            action_name,
                            current_stage,
                            output.consume(),
                        ));
                    }

                    // **Condition not met** → Loop again next frame
                    Ok(new_input) => {
                        let mut system_state: SystemState<ResMut<ActionMap>> = SystemState::new(world);
                        let mut action_map = system_state.get_mut(world);

                        if let Some(actions) = action_map.map.get_mut(&module_name) {
                            if let Some(instance) = actions.get_mut(&action_name).and_then(|a| a.as_mut()) {
                                instance.data_buffer = Box::new(new_input);
                                instance.timeout_frames = instance.num_stages * 30; // Reset timeout
                            }
                        }
                    }
                }
            }
        }
    }

    apply_stage_outputs(world, stage_outputs);
}

fn apply_stage_outputs(
    world: &mut World,
    stage_outputs: Vec<(String, String, usize, Box<dyn Any + Send + Sync>)>,
) {
    let mut system_state: SystemState<EventWriter<ActionStageProcessedEvent>> = SystemState::new(world);
    let mut stage_event_writer = system_state.get_mut(world);

    for (module_name, action_name, stage_index, stage_output) in stage_outputs {
        stage_event_writer.send(ActionStageProcessedEvent {
            module_name,
            action_name,
            stage_index,
            stage_output,
        });
    }
}
