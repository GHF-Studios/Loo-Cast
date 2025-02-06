use std::collections::HashMap;
use bevy::prelude::*;
use crossbeam_channel::{Receiver, Sender};

use super::{events::ActionStageProcessedEvent, target::ActionTypeModule, types::{ActionInstance, ActionState, ActionType}};

#[derive(Resource, Default)]
pub struct ActionTypeModuleRegistry {
    registry: HashMap<String, HashMap<String, ActionType>>,
}

impl ActionTypeModuleRegistry {
    pub fn register(&mut self, mut action_type_module: ActionTypeModule) {
        let action_type_module_name = action_type_module.name.clone();


        let mut registered_actions: HashMap<String, ActionType> = match self.registry.get(&action_type_module_name) {
            Some(_) => {
                unreachable!("Attempted to register action type module '{}' that is already in use.", action_type_module_name)
            },
            None => default()
        };

        while let Some(action_type) = action_type_module.action_types.pop() {
            let action_type_name = action_type.name.clone();

            if registered_actions.insert(action_type.name.clone(), action_type).is_some() {
                unreachable!("Attempted to register action type with name '{}' that is already in use.", action_type_name)
            }
        }

        self.registry.insert(action_type_module_name.clone(), registered_actions);
    }

    pub fn get_action_type(&self, module_name: &str, action_name: &str) -> Option<&ActionType> {
        self.registry.get(module_name)?.get(action_name)
    }

    pub fn get_action_type_mut(&mut self, module_name: &str, action_name: &str) -> Option<&mut ActionType> {
        self.registry.get_mut(module_name)?.get_mut(action_name)
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
pub struct ActionMap {
    pub map: HashMap<String, (Vec<ActionInstance>, HashMap<Entity, usize>)>,
}

impl ActionMap {
    pub fn insert_action(&mut self, mut action_instance: ActionInstance) {
        let action_entity = &action_instance.entity;
        let action_type_module = &action_instance.module_type;
        let action_state = &mut action_instance.state;

        if !action_state.is_requested() {
            unreachable!(
                "Action insertion error: Action has an invalid state. Expected state 'ActionState::Requested', found '{}'.",
                action_instance.state
            );
        }

        *action_state = ActionState::Processing { current_stage: 0 };

        let entry = self.map.entry(action_type_module.to_owned()).or_insert_with(|| (Vec::new(), HashMap::new()));
        let (instances, entity_index) = entry;

        if entity_index.contains_key(action_entity) {
            unreachable!(
                "Action insertion error: Entity {:?} already has an active action for target type '{}'.",
                action_entity, action_type_module
            );
        }

        let index = instances.len();
        entity_index.insert(*action_entity, index);
        instances.push(action_instance);
    }

    pub fn advance_stage(&mut self, entity: Entity, module_type: &str) {
        if let Some((instances, entity_index)) = self.map.get_mut(module_type) {
            if let Some(&index) = entity_index.get(&entity) {
                match &mut instances[index].state {
                    ActionState::Processing { current_stage } => *current_stage += 1,
                    _ => unreachable!()
                }
            } else {
                unreachable!(
                    "Action stage advancement error: No active action found for entity {:?} under target type '{}'.",
                    entity, module_type
                );
            }
        } else {
            unreachable!(
                "Action stage advancement error: No actions exist for target type '{}'.",
                module_type
            );
        }
    }

    pub fn has_action(&self, entity: Entity, module_type: &str) -> bool {
        self.map
            .get(module_type)
            .and_then(|(_, entity_index)| entity_index.get(&entity))
            .is_some()
    }
}