

use core_mod_macros::export_static;
use once_cell::sync::Lazy;
use rhai::Dynamic;
use std::collections::{HashSet, HashMap};
use std::sync::Mutex;

use crate::reflection::{
    function_ids::{CtorId, MethodId, StaticFunctionId},
    ids::TypeId,
    registry::{CtorRegistryEntry, MethodRegistryEntry, StaticFunctionRegistryEntry},
    traits::{TraitTypeKey, TraitTypeVTables, TraitTypeEntry},
    type_info::TypeInfo
};

use super::types::ReflectionRegistry;

export_static!(self, crate::reflection::internals::statics::SCHEDULE_HOOKS: Lazy<Mutex<HashSet<String>>> = Lazy::new(Default::default));

export_static!(self, crate::reflection::internals::statics::TYPE_REGISTRY: Lazy<HashMap<TypeId, &'static TypeInfo>> = Lazy::new(|| {
    inventory::iter::<TypeInfo>.into_iter().map(|entry| (entry.type_id.clone(), entry)).collect()
}));

export_static!(self, crate::reflection::internals::statics::CTOR_REGISTRY: Lazy<HashMap<CtorId, fn(Vec<Dynamic>) -> Dynamic>> = Lazy::new(|| {
    inventory::iter::<CtorRegistryEntry>.into_iter().map(|entry| (entry.id.clone(), entry.fn_ptr)).collect()
}));
export_static!(self, crate::reflection::internals::statics::METHOD_REGISTRY: Lazy<HashMap<MethodId, fn(Vec<Dynamic>) -> Dynamic>> = Lazy::new(|| {
    inventory::iter::<MethodRegistryEntry>.into_iter().map(|entry| (entry.id.clone(), entry.fn_ptr)).collect()
}));
export_static!(self, crate::reflection::internals::statics::STATIC_FUNCTION_REGISTRY: Lazy<HashMap<StaticFunctionId, fn(Vec<Dynamic>) -> Dynamic>> = Lazy::new(|| {
    inventory::iter::<StaticFunctionRegistryEntry>.into_iter().map(|entry| (entry.id.clone(), entry.fn_ptr)).collect()
}));

export_static!(self, crate::reflection::internals::statics::TRAIT_OBJECT_VTABLE_REGISTRY: Lazy<HashMap<TraitTypeKey, TraitTypeVTables>> = Lazy::new(|| {
    let mut m: HashMap<TraitTypeKey, TraitTypeVTables> = Default::default();
    for entry in inventory::iter::<TraitTypeEntry> {
        m.insert(entry.key.clone(), entry.value.clone());
    }
    m
}));

export_static!(self, crate::reflection::internals::statics::REFLECTION_REGISTRY: Lazy<ReflectionRegistry> = Lazy::new(|| ReflectionRegistry::build()));
