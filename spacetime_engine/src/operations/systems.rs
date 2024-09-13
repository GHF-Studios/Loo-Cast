use bevy::prelude::*;
use super::components::Serialized;
use super::hooks::*;
use super::singletons::OPERATION_QUEUE;


pub(in super) fn startup(world: &mut World) {
    world
        .register_component_hooks::<Serialized>()
        .on_add(on_add_serialized)
        .on_remove(on_remove_serialized);
}

pub(in super) fn post_update(world: &mut World) {
    let mut operations = OPERATION_QUEUE.lock().unwrap().remove_operations();

    while let Some(operation_box) = operations.pop() {
        operation_box.execute(world);
    }
}