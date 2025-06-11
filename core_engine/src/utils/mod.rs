use bevy::prelude::*;

#[derive(Component)]
pub struct InitHook<T: Component> {
    has_fired: bool,
    _phantom: std::marker::PhantomData<T>,
}
impl<T: Component> InitHook<T> {
    pub fn fire(&mut self) {
        self.has_fired = true;
    }

    pub fn has_fired(&self) -> bool {
        self.has_fired
    }
}
impl<T: Component> Default for InitHook<T> {
    fn default() -> Self {
        debug!("Spawning InitHook");

        InitHook {
            has_fired: false,
            _phantom: std::marker::PhantomData,
        }
    }
}

#[derive(Component)]
pub struct DropHook<T: Component> {
    has_fired: bool,
    _phantom: std::marker::PhantomData<T>,
}
impl<T: Component> DropHook<T> {
    pub fn fire(&mut self) {
        self.has_fired = true;
    }

    pub fn has_fired(&self) -> bool {
        self.has_fired
    }
}
impl<T: Component> Default for DropHook<T> {
    fn default() -> Self {
        debug!("Spawning DropHook");

        DropHook {
            has_fired: false,
            _phantom: std::marker::PhantomData,
        }
    }
}

pub fn cleanup_init_hooks_system<T: Component>(mut commands: Commands, mut init_hook_query: Query<(Entity, &mut InitHook<T>)>) {
    for (entity, init_hook) in init_hook_query.iter_mut() {
        if init_hook.has_fired() {
            commands.entity(entity).remove::<InitHook<T>>();
        }
    }
}

pub fn cleanup_drop_hooks_system<T: Component>(mut commands: Commands, mut drop_hook_query: Query<(Entity, &mut DropHook<T>)>) {
    for (entity, drop_hook) in drop_hook_query.iter_mut() {
        if drop_hook.has_fired() {
            commands.entity(entity).remove::<DropHook<T>>();
        }
    }
}

pub fn observe_on_remove_init_hook<T: Component>(_trigger: Trigger<OnRemove, InitHook<T>>) {
    debug!("Despawning InitHook");
}

pub fn observe_on_remove_drop_hook<T: Component>(_trigger: Trigger<OnRemove, DropHook<T>>) {
    debug!("Despawning DropHook");
}
