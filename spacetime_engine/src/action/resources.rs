use bevy::prelude::*;

use super::structs::ActionTargetType;

#[derive(Resource, Default)]
pub struct ActionTargetTypeRegistry {

}
impl ActionTargetTypeRegistry {
    pub fn register<T: Component>(&mut self, target_type: ActionTargetType) {

    }
}