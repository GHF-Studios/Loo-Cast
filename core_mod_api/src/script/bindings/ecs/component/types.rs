use rhai::{Dynamic, Identifier};
use std::sync::Arc;

use crate::script::bindings::ecs::component::statics::COMPONENT_CTOR_REGISTRY;

pub type ComponentId = Arc<str>;
pub type ComponentCtorParams = Arc<Dynamic>;
pub type ComponentCtorFn = fn(Dynamic) -> Component;

#[derive(Clone)]
#[repr(transparent)]
pub struct Component(pub(crate) (ComponentId, ComponentCtorParams));
impl Component {
    pub fn create_single(component: (Identifier, Dynamic)) -> Self {
        Self((Arc::from(component.0.as_str()), Arc::new(component.1)))
    }
}