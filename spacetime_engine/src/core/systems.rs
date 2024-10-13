use bevy::prelude::*;
use crate::core::components::Serialized;
use super::hooks::*;

pub(in super) fn startup(world: &mut World) {
    world
        .register_component_hooks::<Serialized>()
        .on_add(on_add_serialized)
        .on_remove(on_remove_serialized);
}