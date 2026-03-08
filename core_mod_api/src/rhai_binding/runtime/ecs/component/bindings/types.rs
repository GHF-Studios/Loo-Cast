use rhai::{Dynamic, Identifier};
use std::sync::Arc;

use super::super::internals::types::{ComponentCtorParams, ComponentId};

#[derive(Clone)]
#[repr(transparent)]
pub struct Component(pub(crate) (ComponentId, ComponentCtorParams));
impl Component {
    pub fn create_single(component_ctor_prepwork: (Identifier, Dynamic)) -> Self {
        Self((Arc::from(component_ctor_prepwork.0.as_str()), component_ctor_prepwork.1))
    }
}
