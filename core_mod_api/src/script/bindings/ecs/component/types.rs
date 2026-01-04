use bevy::ecs::component::{Component as BevyComponent, Mutable};
use rhai::{Dynamic, Identifier};
use std::sync::Arc;

pub type ComponentId = Arc<str>;
pub type ComponentCtorParams = Arc<Dynamic>;
pub type ComponentCtorFn = fn(Dynamic) -> Box<dyn BevyComponent<Mutability = Mutable>>;

#[derive(Clone)]
#[repr(transparent)]
pub struct Component(pub(crate) (ComponentId, ComponentCtorParams));
impl Component {
    pub fn create_single(component: (Identifier, Dynamic)) -> Self {
        Self((Arc::from(component.0.as_str()), Arc::new(component.1)))
    }
}