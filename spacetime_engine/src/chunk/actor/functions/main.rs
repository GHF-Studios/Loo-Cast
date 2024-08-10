use bevy::prelude::World;
use crate::chunk::actor::components::ChunkActor;
use super::hooks;

pub(in crate) fn setup(world: &mut World) {
    world
        .register_component_hooks::<ChunkActor>()
        .on_add(hooks::on_add_chunk_actor)
        .on_remove(hooks::on_remove_chunk_actor);
}
