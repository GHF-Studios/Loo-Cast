use std::{any::TypeId, collections::HashMap};

use bevy::prelude::*;

use super::structs::ActionTargetType;

#[derive(Resource, Default)]
pub struct ActionTargetTypeRegistry {
    registered: HashMap<TypeId, ActionTargetType>
}
impl ActionTargetTypeRegistry {
    pub fn register<T: Component>(&mut self, target_type: ActionTargetType) {
        let target_type_id = TypeId::of::<T>();

        if let Some(_) = self.registered.insert(target_type_id, target_type) {
            unreachable!("Attempted to register type {:?}, which is already registered.", target_type_id)
        }
    }
}