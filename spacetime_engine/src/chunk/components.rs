use bevy::{ecs::component::StorageType, prelude::*};

use super::hooks::{hook_on_add_chunk, hook_on_remove_chunk};

#[derive(Default)]
pub struct ChunkComponent {
    pub coord: (i32, i32),
    pub owner: Option<Entity>,
}
impl Component for ChunkComponent {
    const STORAGE_TYPE: bevy::ecs::component::StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_add(hook_on_add_chunk);
        hooks.on_remove(hook_on_remove_chunk);
    }
}