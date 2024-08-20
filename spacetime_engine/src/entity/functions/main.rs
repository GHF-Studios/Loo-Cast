use bevy::prelude::*;
use crate::entity::{components::*, EntityRegistry};
use crate::operations::{Registry, TYPE_REGISTRY};
use super::hooks;

pub(in crate) fn setup(world: &mut World) {
    let mut type_reg = TYPE_REGISTRY.lock().unwrap();

    // Initialize the entity registry
    type_reg.set_data::<Entity, Registry<Entity>>(Registry::new());

    world
        .register_component_hooks::<SpacetimeEntity>()
        .on_add(hooks::on_add_entity)
        .on_remove(hooks::on_remove_entity);
}