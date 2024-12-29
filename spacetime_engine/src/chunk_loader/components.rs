use bevy::{ecs::component::StorageType, prelude::*};

use super::hooks::{on_add_chunk_loader, on_remove_chunk_loader};

pub struct ChunkLoaderComponent {
    pub radius: u32,
}
impl Default for ChunkLoaderComponent {
    fn default() -> Self {
        ChunkLoaderComponent {
            radius: 2
        }
    }
}
impl Component for ChunkLoaderComponent {
    const STORAGE_TYPE: bevy::ecs::component::StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_add(on_add_chunk_loader);
        hooks.on_remove(on_remove_chunk_loader);
    }
}