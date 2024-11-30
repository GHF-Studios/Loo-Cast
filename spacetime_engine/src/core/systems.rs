use bevy::prelude::*;
use crate::{core::components::Serialized, operation::singletons::OPERATION_QUEUE, singletons::{LOCKING_HIERARCHY, UNLOCK_QUEUE}};
use super::hooks::*;

pub(in super) fn startup(world: &mut World) {
    world
        .register_component_hooks::<Serialized>()
        .on_add(on_add_serialized)
        .on_remove(on_remove_serialized);
}

pub(in super) fn post_update(world: &mut World) {
    {
        let mut operations = OPERATION_QUEUE.lock().unwrap().remove_operations();

        while let Some(mut operation_box) = operations.pop() {
            operation_box.execute(world);
        }
    }

    {
        let mut unlock_queue = UNLOCK_QUEUE.lock().unwrap();
        let mut locking_hierarchy = LOCKING_HIERARCHY.lock().unwrap();
        
        for unlock_request in unlock_queue.iter() {
            let node = locking_hierarchy.get_node_mut(&unlock_request.path).unwrap();
            node.unlock();
        }
        unlock_queue.clear();
    }
}