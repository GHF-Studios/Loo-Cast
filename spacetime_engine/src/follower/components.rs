use bevy::{ecs::component::StorageType, prelude::*};

use super::hooks::{hook_on_add_follower_target, hook_on_remove_follower_target};

#[derive(Component)]
pub struct FollowerComponent {
    pub follow_id: String,
    pub offset: Vec2,
    pub smoothness: f32,
    followed_entity: Option<Entity>,
}
impl FollowerComponent {
    pub fn new(follow_id: String, offset: Vec2, smoothness: f32) -> Self {
        Self {
            follow_id,
            offset,
            smoothness,
            followed_entity: None,
        }
    }

    pub fn get_followed_entity(&self) -> &Option<Entity> {
        &self.followed_entity
    }

    pub fn get_followed_entity_mut(&mut self) -> &mut Option<Entity> {
        &mut self.followed_entity
    }
}

pub struct FollowerTargetComponent {
    pub id: String,
}
impl Component for FollowerTargetComponent {
    const STORAGE_TYPE: bevy::ecs::component::StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_add(hook_on_add_follower_target);
        hooks.on_remove(hook_on_remove_follower_target);
    }
}
