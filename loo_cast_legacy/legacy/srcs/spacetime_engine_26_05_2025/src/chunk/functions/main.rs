use bevy::prelude::*;
use crate::chunk::components::*;
use super::hooks;

pub(in crate) fn setup(world: &mut World) {
    world
        .register_component_hooks::<Chunk>()
        .on_add(hooks::on_add_chunk)
        .on_remove(hooks::on_remove_chunk);
}


