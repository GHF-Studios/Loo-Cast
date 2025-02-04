use std::{any::{Any, TypeId}, collections::HashMap};
use bevy::prelude::*;
use crossbeam_channel::{Receiver, Sender};

use super::{events::ActionStageProcessedEvent, target::ActionTargetType, types::{ActionInstance, ActionState, ActionType}};

#[derive(Resource, Default)]
pub struct ActionTargetTypeRegistry {
    registry: HashMap<String, HashMap<String, ActionType>>,
}

impl ActionTargetTypeRegistry {
    pub fn register<T: Component>(&mut self, mut action_target_type: ActionTargetType) {
        let component_type_id = TypeId::of::<T>();
        let action_target_type_name = action_target_type.name.clone();
        let action_target_type_id = action_target_type.type_id.clone();

        if component_type_id != action_target_type_id {
            unreachable!("Attempted to register the type '{:?}' as an action target type, but the provided type id '{:?}' did not match the given type.", action_target_type_id, component_type_id)
        }

        let mut registered_actions: HashMap<String, ActionType> = match self.registry.get(&action_target_type_name) {
            Some(_) => {
                unreachable!("Attempted to register action target type '{:?}' with name '{}' that is already in use.", action_target_type_id, action_target_type_name)
            },
            None => default()
        };

        while let Some(action_type) = action_target_type.action_types.pop() {
            let action_type_name = action_type.name.clone();

            if let Some(_) = registered_actions.insert(action_type.name.clone(), action_type) {
                unreachable!("Attempted to register action type with name '{}' that is already in use.", action_type_name)
            }
        }

        self.registry.insert(action_target_type_name.clone(), registered_actions);
    }

    pub fn get_action(&self, target_type: &str, action_name: &str) -> Option<&ActionType> {
        self.registry.get(target_type)?.get(action_name)
    }
}

#[derive(Resource, Default)]
pub struct ActionRequestBuffer {
    pub requests: Vec<ActionInstance>,
}

#[derive(Resource)]
pub(in super) struct ActionStageProcessedMessageSender(pub Sender<ActionStageProcessedEvent>);

#[derive(Resource)]
pub(in super) struct ActionStageProcessedMessageReceiver(pub Receiver<ActionStageProcessedEvent>);

#[derive(Resource, Default, Debug)]
pub(in super) struct ActionMap {
    pub map: HashMap<String, (Vec<ActionInstance>, HashMap<Entity, usize>)>,
}

impl ActionMap {
    pub fn insert_action(&mut self, mut action_instance: ActionInstance) {
        let action_entity = &action_instance.entity;
        let action_target_type = &action_instance.target_type;
        let action_state = &mut action_instance.state;

        if !action_state.is_requested() {
            unreachable!(
                "Action insertion error: Action has an invalid state. Expected state 'ActionState::Requested', found '{}'.",
                action_instance.state
            );
        }

        *action_state = ActionState::Processing { current_stage: 0 };

        let entry = self.map.entry(action_target_type.to_owned()).or_insert_with(|| (Vec::new(), HashMap::new()));
        let (instances, entity_index) = entry;

        if entity_index.contains_key(action_entity) {
            unreachable!(
                "Action insertion error: Entity {:?} already has an active action for target type '{}'.",
                action_entity, action_target_type
            );
        }

        let index = instances.len();
        entity_index.insert(*action_entity, index);
        instances.push(action_instance);
    }

    pub fn advance_stage(&mut self, entity: Entity, target_type: &str) {
        if let Some((instances, entity_index)) = self.map.get_mut(target_type) {
            if let Some(&index) = entity_index.get(&entity) {
                match &mut instances[index].state {
                    ActionState::Processing { current_stage } => *current_stage += 1,
                    _ => unreachable!()
                }
            } else {
                unreachable!(
                    "Action stage advancement error: No active action found for entity {:?} under target type '{}'.",
                    entity, target_type
                );
            }
        } else {
            unreachable!(
                "Action stage advancement error: No actions exist for target type '{}'.",
                target_type
            );
        }
    }

    pub fn has_action(&self, entity: Entity, target_type: &str) -> bool {
        self.map
            .get(target_type)
            .and_then(|(_, entity_index)| entity_index.get(&entity))
            .is_some()
    }
}