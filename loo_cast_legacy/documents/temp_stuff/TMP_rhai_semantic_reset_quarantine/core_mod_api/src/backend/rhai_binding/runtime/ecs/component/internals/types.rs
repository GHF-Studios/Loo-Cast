use crate::bevy::ecs::world::EntityWorldMut;
use rhai::Dynamic;
use std::sync::Arc;

pub type ComponentId = Arc<str>;
pub type ComponentCtorParams = Dynamic;
pub type ComponentCtorFn = fn(&mut EntityWorldMut, Dynamic);
pub type ComponentRemoveFn = fn(&mut EntityWorldMut);

inventory::collect!(ComponentCtorEntry);
pub struct ComponentCtorEntry {
    pub name: &'static str,
    pub ctor: ComponentCtorFn,
    pub remove: ComponentRemoveFn,
}
