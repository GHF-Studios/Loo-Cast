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
        InitHook {
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
