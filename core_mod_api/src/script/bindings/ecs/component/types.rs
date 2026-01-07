use bevy::ecs::world::EntityWorldMut;
use rhai::{Dynamic, Identifier};
use std::sync::Arc;

pub type ComponentId = Arc<str>;
pub type ComponentCtorParams = Dynamic;
pub type ComponentCtorFn = fn(&mut EntityWorldMut, Dynamic);

#[derive(Clone)]
#[repr(transparent)]
pub struct Component(pub(crate) (ComponentId, ComponentCtorParams));
impl Component {
    pub fn create_single(component_ctor_prepwork: (Identifier, Dynamic)) -> Self {
        Self((Arc::from(component_ctor_prepwork.0.as_str()), component_ctor_prepwork.1))
    }
}
pub struct ComponentCtorEntry {
    pub name: &'static str,
    pub ctor: fn(rhai::Dynamic) -> Component,
}
