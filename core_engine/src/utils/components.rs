use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct InitHook<T: Component> {
    has_fired: bool,
    #[reflect(ignore)]
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

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct DropHook<T: Component> {
    has_fired: bool,
    #[reflect(ignore)]
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
