use bevy::prelude::*;
use crate::entity::components::*;
use super::hooks;

pub(in crate) fn startup(world: &mut World) {
    world
        .register_component_hooks::<SpacetimeEntity>()
        .on_add(hooks::on_add_entity)
        .on_remove(hooks::on_remove_entity);
}