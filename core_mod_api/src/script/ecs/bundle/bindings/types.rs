use rhai::{Dynamic, Map};
use std::collections::HashMap;
use std::sync::Arc;

use crate::script::ecs::component::internals::types::{ComponentCtorParams, ComponentId};

#[derive(Clone)]
#[repr(transparent)]
pub struct Bundle(pub(crate) HashMap<ComponentId, Dynamic>);
impl Bundle {
    pub fn create_batch(components: Map) -> Self {
        let raw_components = components.into_iter().map(|(name, params)| {
            (Arc::from(name.as_str()), params)
        }).collect();
        Self(raw_components)
    }
}
