use bevy::prelude::*;
use super::singletons::*;

pub(in super) fn post_update(world: &mut World) {
    let mut operations = OPERATION_QUEUE.lock().unwrap().remove_operations();

    while let Some(mut operation_box) = operations.pop() {
        operation_box.execute(world);
    }
}