use bevy::prelude::*;

use super::components::{InitHook, DropHook};

pub fn cleanup_init_hooks_system<T: Component>(mut commands: Commands, mut init_hook_query: Query<(Entity, &mut InitHook<T>)>) {
    for (entity, init_hook) in init_hook_query.iter_mut() {
        if init_hook.has_fired() {
            debug!("Despawning InitHook");
            commands.entity(entity).remove::<InitHook<T>>();
        }
    }
}

pub fn cleanup_drop_hooks_system<T: Component>(mut commands: Commands, mut drop_hook_query: Query<(Entity, &mut DropHook<T>)>) {
    for (entity, drop_hook) in drop_hook_query.iter_mut() {
        if drop_hook.has_fired() {
            debug!("Despawning DropHook");
            commands.entity(entity).remove::<DropHook<T>>();
        }
    }
}

pub fn observe_on_remove_init_hook<T: Component>(_trigger: Trigger<OnRemove, InitHook<T>>) {
    debug!("Despawned InitHook");
}

pub fn observe_on_remove_drop_hook<T: Component>(_trigger: Trigger<OnRemove, DropHook<T>>) {
    debug!("Despawned DropHook");
}