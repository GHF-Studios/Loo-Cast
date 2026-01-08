use core_mod_macros::export_static;
use once_cell::sync::Lazy;
use std::collections::HashMap;

use crate::script::bindings::ecs::component::types::{ComponentId, ComponentCtorFn};

export_static!(self, crate::core_mod_api::script::bindings::component::statics::COMPONENT_CTOR_REGISTRY: Lazy<HashMap<ComponentId, ComponentCtorFn>> = Lazy::new(Default::default));