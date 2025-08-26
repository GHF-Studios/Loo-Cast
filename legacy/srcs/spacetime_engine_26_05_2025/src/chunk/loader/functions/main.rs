use bevy::prelude::World;
use crate::chunk::loader::components::ChunkLoader;
use super::hooks;

pub(in crate) fn setup(world: &mut World) {
    world
        .register_component_hooks::<ChunkLoader>()
        .on_add(hooks::on_add_chunk_loader)
        .on_remove(hooks::on_remove_chunk_loader);
}