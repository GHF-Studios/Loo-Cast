use core_mod_macros::export_static;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Arc;

use super::types::{ComponentCtorEntry, ComponentCtorFn, ComponentId};

export_static!(self, crate::rhai_binding::runtime::ecs::component::internals::statics::COMPONENT_CTOR_REGISTRY: Lazy<HashMap<ComponentId, ComponentCtorFn>> = Lazy::new(|| {
    let mut m: HashMap<ComponentId, ComponentCtorFn> = Default::default();
    for entry in inventory::iter::<ComponentCtorEntry> {
        m.insert(Arc::from(entry.name), entry.ctor);
    }
    m
}));
