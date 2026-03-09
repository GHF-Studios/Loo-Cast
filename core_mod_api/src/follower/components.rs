use crate::bevy::{
    ecs::{
        component::{Mutable, StorageType},
        lifecycle::ComponentHook,
    },
    prelude::*,
};

use super::hooks::{hook_on_add_follower_target, hook_on_remove_follower_target};

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Follower {
    pub follow_id: String,
    pub offset: Vec2,
    pub smoothness: f32,
    followed_entity: Option<Entity>,
}
impl Follower {
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

#[derive(Reflect)]
#[reflect(Component)]
pub struct FollowerTarget {
    /// Who this target is followed by
    pub id: String,
}
impl Component for FollowerTarget {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    type Mutability = Mutable;

    fn on_add() -> Option<ComponentHook> {
        Some(hook_on_add_follower_target)
    }

    fn on_remove() -> Option<ComponentHook> {
        Some(hook_on_remove_follower_target)
    }
}
