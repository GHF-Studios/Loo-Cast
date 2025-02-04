use std::{any::{Any, TypeId}, collections::HashSet};

use bevy::prelude::*;
use crossbeam_channel::{Receiver, Sender};

use super::{events::ActionStageProcessedEvent, resources::{ActionMap, ActionRequestBuffer, ActionTargetTypeRegistry}, stage::ActionStage, stage_io::ActionStageIO, target::{ActionTargetRef, ActionTargetType}, types::{ActionState, ActionType}, ActionStageProcessedMessageReceiver, ActionStageProcessedMessageSender};

pub(in super) fn async_stage_event_relay_system(
    receiver: ResMut<ActionStageProcessedMessageReceiver>,
    mut action_event_writer: EventWriter<ActionStageProcessedEvent>, 
) {
    while let Ok(event) = receiver.0.try_recv() {
        action_event_writer.send(event);
    }
}

pub(in super) fn action_tick_system(world: &mut World) {
    let mut completed_actions = Vec::new();

    // Process all active actions
    let mut action_map = world.resource_mut::<ActionMap>();
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
    let mut stage_processed_events = world.resource_mut::<Events<ActionStageProcessedEvent>>();
    let mut events = stage_processed_events.drain().collect::<Vec<_>>();
    let mut action_map = world.resource_mut::<ActionMap>();
    while let Some(event) = events.pop() {
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

    // Finalize completed actions
    for (target_type, entity) in completed_actions {
        let mut action_map = world.resource_mut::<ActionMap>();
        if let Some((mut instances, mut entity_index)) = action_map.map.remove(&target_type) {
            if let Some(index) = entity_index.remove(&entity) {
                let action = instances.swap_remove(index);

                if let Some(callback) = action.callback {
                    callback(world, action.params_buffer);
                }

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
}

pub(in super) fn action_execution_system(
    mut action_map: ResMut<ActionMap>,
    mut stage_event_writer: EventWriter<ActionStageProcessedEvent>,
    mut world: &mut World,
    async_sender: Res<ActionStageProcessedMessageSender>,
) {
    for (target_type, (instances, _)) in action_map.map.iter_mut() {
        // TODO: Resolve action target type
        let action_target_type: ActionTargetType = ();

        for instance in instances.iter_mut() {
            // TODO: Resolve action type
            let action_type: ActionType = ();

            if let ActionState::Processing { current_stage } = &instance.state {
                let stage = instance.get_current_stage();

                match stage {
                    Some(ActionStage::Ecs(mut ecs_stage)) => {
                        // Execute the ECS stage
                        let io = ActionStageIO::new(instance.params_buffer.take());
                        let output = (ecs_stage.function)(io, world);

                        // Emit completion event
                        stage_event_writer.send(ActionStageProcessedEvent {
                            target_entity: instance.entity,
                            target_type: target_type.clone(),
                            action_name: instance.action_name.clone(),
                            stage_name: ecs_stage.name.clone(),
                            stage_output: Some(output.state.output),
                        });
                    }
                    Some(ActionStage::Async(async_stage)) => {
                        let entity = instance.entity;
                        let target_type = target_type.clone();
                        let action_name = instance.action_name.clone();

                        // Execute async task
                        let io = ActionStageIO::new(instance.params_buffer.take());
                        let future = (async_stage.function)(io);

                        let sender = async_sender.0.clone();
                        tokio::spawn(async move {
                            let output = future.await;
                            sender.send(ActionStageProcessedEvent {
                                target_entity: entity,
                                target_type,
                                action_name,
                                stage_name: async_stage.name.clone(),
                                stage_output: Some(output.state.output),
                            }).unwrap();
                        });
                    }
                    None => continue, // Action is finished or invalid
                }
            }
        }
    }
}