use std::{any::{Any, TypeId}, collections::HashMap};

use bevy::prelude::*;

use super::structs::ActionTargetType;

#[derive(Resource, Default)]
pub struct ActionTargetTypeRegistry {
    registered: HashMap<TypeId, ActionTargetType>
}
impl ActionTargetTypeRegistry {
    // TODO: Validate target type to avoid duplicate names 
    //          for all target types,
    //          for all action types given a specific target type,
    //          and for all action stage types given a specific action type
    // TODO: Make key String-based
    pub fn register<T: Component>(&mut self, target_type: ActionTargetType) {
        let target_type_id = TypeId::of::<T>();

        if let Some(_) = self.registered.insert(target_type_id, target_type) {
            unreachable!("Attempted to register type {:?}, which is already registered.", target_type_id)
        }
    }
}

pub struct ActionManager {}
impl ActionManager {
    pub fn request(&mut self, target_type: String, action_type: String, params: Box<dyn Any + Send + Sync>) {
    }
}