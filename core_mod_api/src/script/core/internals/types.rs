use bevy::ecs::world::EntityWorldMut;
use rhai::Dynamic;
use std::sync::Arc;

pub type ComponentId = Arc<str>;
pub type ComponentCtorParams = Dynamic;
pub type TraitObjectUseRefFn = fn(&mut EntityWorldMut, Dynamic);

inventory::collect!(ComponentCtorEntry);
pub struct ComponentCtorEntry {
    pub name: &'static str,
    pub ctor: ComponentCtorFn,
}
