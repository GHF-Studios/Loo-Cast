use bevy::prelude::*;
use bevy_rapier2d::plugin::RapierConfiguration;
use crate::components::Serialized;
use crate::singletons::*;
use crate::hooks::*;

pub(in super) fn pre_startup(world: &mut World) {
    let mut rapier_configuration = world.get_resource_mut::<RapierConfiguration>().unwrap();
    rapier_configuration.gravity = Vec2::new(0.0, 0.0);
    drop(rapier_configuration);
}

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
            let node = locking_hierarchy.try_get_node_mut(unlock_request.node_path.clone()).unwrap();
            node.unlock();
        }
        unlock_queue.clear();
    }
}