use bevy::prelude::*;
use crate::entity::{components::*, EntityRegistry};
use crate::operations::{Registry, MAIN_TYPE_REGISTRY};
use super::hooks;

pub(in crate) fn setup(world: &mut World) {
    world
        .register_component_hooks::<SpacetimeEntity>()
        .on_add(hooks::on_add_entity)
        .on_remove(hooks::on_remove_entity);
}