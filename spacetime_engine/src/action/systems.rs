use std::{any::{Any, TypeId}, collections::HashSet};
use bevy::prelude::*;
use bevy::ecs::system::SystemState;
use crossbeam_channel::{Receiver, Sender};

use super::{events::ActionStageProcessedEvent, resources::{ActionMap, ActionRequestBuffer, ActionTargetTypeRegistry}, stage::ActionStage, stage_io::ActionStageIO, target::{ActionTargetRef, ActionTargetType}, types::{Action, ActionState, ActionType}, ActionStageProcessedMessageReceiver, ActionStageProcessedMessageSender};

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
        EventReader<ActionStageProcessedEvent>,
    )> = SystemState::new(world);

    let (
        mut action_map, 
        mut stage_event_reader, 
    ) = system_state.get_mut(world);

    let mut completed_actions = Vec::new();

    // Process all active actions
    for (target_type, (instances, _)) in action_map.map.iter_mut() {
        for instance in instances.iter_mut() {
            // Decrement timeout counter
            if instance.timeout_frames == 0 {
                panic!(
                    "Action timeout error: Entity {:?} running '{}' for target '{}' has exceeded its allowed execution time.",
                    instance.entity, instance.action_name, target_type
                );
            }
            instance.timeout_frames -= 1;
        }
    }

    // Process completion events and move to the next stage
    for event in stage_event_reader.read() {
        if let Some((instances, _)) = action_map.map.get_mut(&event.target_type) {
            if let Some(instance) = instances.iter_mut().find(|a| a.entity == event.target_entity) {
                match &mut instance.state {
                    ActionState::Processing { current_stage } => {
                        *current_stage += 1;

                        if *current_stage < instance.num_stages {
                            instance.timeout_frames = instance.num_stages * 30;
                        } else {
                            completed_actions.push((event.target_type.clone(), event.target_entity));
                        }
                    }
                    _ => unreachable!("Unexpected state transition"),
                };
            }
        }
    }

    // Prepare finalization of completed actions
    let mut callbacks = Vec::new();
    for (target_type, entity) in completed_actions {
        if let Some((ref mut instances, ref mut entity_index)) = action_map.map.get_mut(&target_type) {
            if let Some(index) = entity_index.remove(&entity) {
                let action = instances.swap_remove(index);
                callbacks.push((action.callback, action.data_buffer));

                if index < instances.len() {
                    let swapped_entity = instances[index].entity;
                    entity_index.insert(swapped_entity, index);
                }
            } else {
                unreachable!(
                    "Action finalization error: No active action found for entity {:?} under target type '{}'.",
                    entity, target_type
                );
            }
        } else {
            unreachable!(
                "Action finalization error: No actions exist for target type '{}'.",
                target_type
            );
        }
    }

    for (callback, data) in callbacks {
        if let Some(callback) = callback {
            callback(world, data);
        }
    }


}

pub(in super) fn action_execution_system(world: &mut World) {
    // Phase 1: Data Collection
    let mut system_state: SystemState<(
        ResMut<ActionTargetTypeRegistry>,
        EventWriter<ActionStageProcessedEvent>,
        Res<ActionStageProcessedMessageSender>,
    )> = SystemState::new(world);

    let (
        mut target_type_registry, 
        mut stage_event_writer, 
        async_sender
    ) = system_state.get_mut(world);

    let mut system_state_read: SystemState<Res<ActionMap>> = SystemState::new(world);
    let action_map = system_state_read.get(world);

    let mut actions_to_process: Vec<(Entity, String, String, usize, ActionStage, Box<&(dyn Any + Send + Sync)>)> = Vec::new();

    for (target_type, (instances, _)) in action_map.map.iter_mut() {
        for instance in instances.iter() {
            if let ActionState::Processing { current_stage } = &instance.state {
                let action_type = target_type_registry
                    .get(target_type, &instance.action_name)
                    .expect("David");

                /* TODO: Somehow temporarily acquire the stage and give it back after we are done with processing,
                *        so we don't have to clone or keep a reference, but just move it (temporarily)
                */
                let stage = &action_type.stages[*current_stage];

                actions_to_process.push((
                    instance.entity,
                    target_type.clone(),
                    instance.action_name.clone(),
                    *current_stage,
                    stage.clone(),
                    Box::new(instance.data_buffer.as_ref().clone()),
                ));
            }
        }
    }

    drop(system_state_read); // Explicitly drop to release immutable borrow on action_map

    // Phase 2: Processing
    let mut stage_outputs = Vec::new();

    for (entity, target_type, action_name, current_stage, ref mut stage, ref mut data_buffer) in actions_to_process.iter_mut() {
        match stage {
            ActionStage::Ecs(ref mut ecs_stage) => {
                let io = ActionStageIO::new(Box::new(data_buffer));
                let function = &mut ecs_stage.function;
                let output = (function)(io, world).consume();

                stage_outputs.push((
                    entity,
                    target_type,
                    action_name,
                    current_stage,
                    output,
                ));
            }
            ActionStage::Async(ref mut async_stage) => {
                let io = ActionStageIO::new(Box::new(data_buffer));
                let function = &mut async_stage.function;
                let future = (function)(io);

                let sender = async_sender.0.clone();
                tokio::spawn(async move {
                    let output = future.await.consume();
                    sender
                        .send(ActionStageProcessedEvent {
                            target_entity: *entity,
                            target_type: target_type.to_string(),
                            action_name: action_name.to_string(),
                            stage_index: *current_stage,
                            stage_output: output,
                        })
                        .unwrap();
                });
            }
        }
    }

    // Phase 3: Applying Changes
    let mut system_state_write: SystemState<ResMut<ActionMap>> = SystemState::new(world);
    let mut action_map = system_state_write.get_mut(world);

    for (entity, target_type, action_name, stage_index, stage_output) in stage_outputs {
        stage_event_writer.send(ActionStageProcessedEvent {
            target_entity: *entity,
            target_type: target_type.to_string(),
            action_name: action_name.to_string(),
            stage_index: *stage_index,
            stage_output,
        });
    }
}
