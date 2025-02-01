use std::{any::{Any, TypeId}, collections::HashMap};

use bevy::prelude::*;

use super::structs::{ActionTargetType, ActionType};

#[derive(Resource, Default)]
pub struct ActionTargetTypeRegistry {
    registry: HashMap<String, HashMap<String, ActionType>>, // Maps target type -> (action name -> ActionType)
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

pub struct ActionInstance {
    pub target_entity: Entity,
    pub action_name: String,
    pub current_stage: usize,
    pub callback: Option<Box<dyn FnOnce(&mut World) + Send + Sync>>,
}
impl std::fmt::Debug for ActionInstance{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ActionInstance(target_entity: {:?}, action_name: {:?}, current_stage: {:?})", self.target_entity, self.action_name, self.current_stage)
    }
}

#[derive(Resource, Default, Debug)]
pub struct ActionMap {
    map: HashMap<String, (Vec<ActionInstance>, HashMap<Entity, usize>)>,
}

impl ActionMap {
    pub fn insert_action(
        &mut self,
        target_type: &str,
        target_entity: Entity,
        action_name: impl Into<String>,
        callback: Option<Box<dyn FnOnce(&mut World) + Send + Sync>>,
    ) {
        let entry = self.map.entry(target_type.to_owned()).or_insert_with(|| (Vec::new(), HashMap::new()));

        let (instances, entity_index) = entry;

        if entity_index.contains_key(&target_entity) {
            panic!(
                "Action insertion error: Entity {:?} already has an active action for target type '{}'.",
                target_entity, target_type
            );
        }

        let index = instances.len();
        instances.push(ActionInstance {
            target_entity,
            action_name: action_name.into(),
            current_stage: 0,
            callback,
        });

        entity_index.insert(target_entity, index);
    }

    pub fn advance_stage(&mut self, target_type: &str, target_entity: Entity) {
        if let Some((instances, entity_index)) = self.map.get_mut(target_type) {
            if let Some(&index) = entity_index.get(&target_entity) {
                instances[index].current_stage += 1;
            } else {
                panic!(
                    "Action stage advancement error: No active action found for entity {:?} under target type '{}'.",
                    target_entity, target_type
                );
            }
        } else {
            panic!(
                "Action stage advancement error: No actions exist for target type '{}'.",
                target_type
            );
        }
    }

    pub fn finalize_action(&mut self, target_type: &str, target_entity: Entity, world: &mut World) {
        if let Some((instances, entity_index)) = self.map.get_mut(target_type) {
            if let Some(index) = entity_index.remove(&target_entity) {
                let action = instances.swap_remove(index);

                if let Some(callback) = action.callback {
                    callback(world);
                }

                if index < instances.len() {
                    let swapped_entity = instances[index].target_entity;
                    entity_index.insert(swapped_entity, index);
                }
            } else {
                panic!(
                    "Action finalization error: No active action found for entity {:?} under target type '{}'.",
                    target_entity, target_type
                );
            }
        } else {
            panic!(
                "Action finalization error: No actions exist for target type '{}'.",
                target_type
            );
        }
    }

    pub fn has_action(&self, target_type: &str, entity: Entity) -> bool {
        self.map
            .get(target_type)
            .and_then(|(_, entity_index)| entity_index.get(&entity))
            .is_some()
    }
}

pub struct ActionManager {}
impl ActionManager {
    pub fn request(&mut self, target_type: String, action_type: String, params: Box<dyn Any + Send + Sync>) {
    }
}