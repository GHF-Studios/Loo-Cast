
use core_mod_core::reflection::{
    ids::TypeId,
    function_ids::{CtorId, MethodId, StaticFunctionId},
    registry::{CtorRegistryEntry, MethodRegistryEntry, StaticFunctionRegistryEntry},
    type_info::TypeInfo,
};
use core_mod_macros::export_static;
use once_cell::sync::Lazy;
use rhai::Dynamic;
use std::collections::{HashSet, HashMap};
use std::sync::Mutex;


export_static!(self, crate::core_mod_api::script::core::internals::statics::SCHEDULE_HOOKS: Lazy<Mutex<HashSet<String>>> = Lazy::new(Default::default));
export_static!(self, crate::core_mod_api::script::core::internals::statics::TYPE_REGISTRY: Lazy<HashMap<TypeId, &'static TypeInfo>> = Lazy::new(|| {
    inventory::iter::<TypeInfo>.into_iter().map(|entry| (entry.type_id.clone(), entry)).collect()
}));
export_static!(self, crate::core_mod_api::script::core::internals::statics::CTOR_REGISTRY: Lazy<HashMap<CtorId, fn(Vec<Dynamic>) -> Dynamic>> = Lazy::new(|| {
    inventory::iter::<CtorRegistryEntry>.into_iter().map(|entry| (entry.id.clone(), entry.fn_ptr)).collect()
}));
export_static!(self, crate::core_mod_api::script::core::internals::statics::METHOD_REGISTRY: Lazy<HashMap<MethodId, fn(Vec<Dynamic>) -> Dynamic>> = Lazy::new(|| {
    inventory::iter::<MethodRegistryEntry>.into_iter().map(|entry| (entry.id.clone(), entry.fn_ptr)).collect()
}));
export_static!(self, crate::core_mod_api::script::core::internals::statics::STATIC_FUNCTION_REGISTRY: Lazy<HashMap<StaticFunctionId, fn(Vec<Dynamic>) -> Dynamic>> = Lazy::new(|| {
    inventory::iter::<StaticFunctionRegistryEntry>.into_iter().map(|entry| (entry.id.clone(), entry.fn_ptr)).collect()
}));
