use std::any::Any;
use bevy::prelude::*;
use bevy::ecs::system::SystemState;
use crossbeam_channel::Sender;

use super::{
    events::ActionStageProcessedEvent,
    resources::{ActionMap, ActionTypeModuleRegistry},
    stage::{ActionStage, ActionStageEcs},
    stage_io::ActionIO,
    types::ActionState,
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
    for (module_type, (instances, _)) in action_map.map.iter_mut() {
        for instance in instances.iter_mut() {
            if instance.timeout_frames == 0 {
                panic!(
                    "Action timeout error: Entity {:?} running '{}' for target '{}' exceeded execution time.",
                    instance.entity, instance.action_name, module_type
                );
            }
            instance.timeout_frames -= 1;
        }
    }
}

fn process_stage_events(
    action_map: &mut ActionMap,
    stage_event_reader: &mut EventReader<ActionStageProcessedEvent>,
) -> Vec<(String, Entity)> {
    let mut completed_actions = Vec::new();

    for event in stage_event_reader.read() {
        if let Some((instances, _)) = action_map.map.get_mut(&event.module_type) {
            if let Some(instance) = instances.iter_mut().find(|a| a.entity == event.target_entity) {
                match &mut instance.state {
                    ActionState::Processing { current_stage } => {
                        *current_stage += 1;

                        if *current_stage < instance.num_stages {
                            instance.timeout_frames = instance.num_stages * 30;
                        } else {
                            completed_actions.push((event.module_type.clone(), event.target_entity));
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
    completed_actions: Vec<(String, Entity)>,
) {
    let mut system_state: SystemState<ResMut<ActionMap>> = SystemState::new(world);
    let mut action_map = system_state.get_mut(world);

    let mut callbacks = Vec::new();

    for (module_type, entity) in completed_actions {
        if let Some((instances, entity_index)) = action_map.map.get_mut(&module_type) {
            if let Some(index) = entity_index.remove(&entity) {
                let action = instances.swap_remove(index);
                callbacks.push((action.callback, action.data_buffer));

                if index < instances.len() {
                    let swapped_entity = instances[index].entity;
                    entity_index.insert(swapped_entity, index);
                }
            }
        }
    }

    for (callback, data) in callbacks {
        if let Some(callback) = callback {
            callback(world, data);
        }
    }
}

pub(in super) fn action_execution_system(world: &mut World) {
    let (actions_to_process, async_sender) = collect_action_data(world);
    process_actions(world, actions_to_process, async_sender);
}

fn collect_action_data(world: &mut World) -> (Vec<(Entity, String, String, usize, ActionStage, Box<dyn Any + Send + Sync>)>, Sender<ActionStageProcessedEvent>) {
    let mut system_state: SystemState<(ResMut<ActionMap>, ResMut<ActionTypeModuleRegistry>, Res<ActionStageProcessedMessageSender>)> = SystemState::new(world);
    let (mut action_map, mut module_type_registry, async_sender) = system_state.get_mut(world);

    let mut actions_to_process = Vec::new();

    for (module_type, (instances, _)) in action_map.map.iter_mut() {
        for instance in instances.iter_mut() {
            if let ActionState::Processing { current_stage } = &instance.state {
                let action_type = module_type_registry
                    .get_action_type_mut(module_type, &instance.action_name)
                    .unwrap_or_else(|| panic!("Action type `{}` not found in registry for `{}`",
                        instance.action_name, module_type));

                let stage = std::mem::replace(
                    &mut action_type.stages[*current_stage],
                    ActionStage::Ecs(ActionStageEcs {
                        name: "placeholder".to_string(),
                        function: Box::new(|_, _| unreachable!()),
                    }),
                );

                actions_to_process.push((
                    instance.entity,
                    module_type.clone(),
                    instance.action_name.clone(),
                    *current_stage,
                    stage,
                    std::mem::replace(&mut instance.data_buffer, Box::new(())), // Move data out safely
                ));
            }
        }
    }

    (actions_to_process, async_sender.0.clone())
}

fn process_actions(
    world: &mut World,
    mut actions_to_process: Vec<(Entity, String, String, usize, ActionStage, Box<dyn Any + Send + Sync>)>,
    async_sender: Sender<ActionStageProcessedEvent>,
) {
    let mut stage_outputs = Vec::new();

    for (entity, module_type, action_name, current_stage, mut stage, data_buffer) in actions_to_process.drain(..) {
        let sender = async_sender.clone();
        match stage {
            ActionStage::Ecs(ref mut ecs_stage) => {
                let io = ActionIO::new(Box::new(data_buffer));
                let function = &mut ecs_stage.function;
                let output = (function)(io, world).consume();

                stage_outputs.push((
                    entity,
                    module_type,
                    action_name,
                    current_stage,
                    output,
                ));
            }
            ActionStage::Async(ref mut async_stage) => {
                let io = ActionIO::new(Box::new(data_buffer));
                let function = &mut async_stage.function;
                let future = (function)(io);

                tokio::spawn(async move {
                    let output = future.await.consume();
                    sender
                        .send(ActionStageProcessedEvent {
                            target_entity: entity,
                            module_type: module_type.to_string(),
                            action_name: action_name.to_string(),
                            stage_index: current_stage,
                            stage_output: output,
                        })
                        .unwrap();
                });
            }
        }
    }

    apply_stage_outputs(world, stage_outputs);
}

fn apply_stage_outputs(
    world: &mut World,
    stage_outputs: Vec<(Entity, String, String, usize, Box<dyn Any + Send + Sync>)>,
) {
    let mut system_state: SystemState<EventWriter<ActionStageProcessedEvent>> = SystemState::new(world);
    let mut stage_event_writer = system_state.get_mut(world);

    for (entity, module_type, action_name, stage_index, stage_output) in stage_outputs {
        stage_event_writer.send(ActionStageProcessedEvent {
            target_entity: entity,
            module_type,
            action_name,
            stage_index,
            stage_output,
        });
    }
}
