use core_mod_macros::export_static;
use once_cell::sync::Lazy;
use rhai::Dynamic;
use std::collections::{HashSet, HashMap};
use std::sync::{Arc, Mutex};

use crate::script::core::internals::types::{TypeId, TypeInfo, CtorSignature, MethodSignature, StaticFunctionSignature};

export_static!(self, crate::core_mod_api::script::core::internals::statics::SCHEDULE_HOOKS: Lazy<Mutex<HashSet<String>>> = Lazy::new(Default::default));
export_static!(self, crate::core_mod_api::script::core::internals::statics::TYPE_REGISTRY: Lazy<HashMap<TypeId, &'static TypeInfo>> = Lazy::new(|| {
    inventory::iter::<TypeInfo>.into_iter().map(|entry| (entry.type_id.clone(), entry)).collect()
}));
export_static!(self, crate::core_mod_api::script::core::internals::statics::CTOR_REGISTRY: Lazy<HashMap<CtorSignature, fn(Vec<Dynamic>) -> Dynamic>> = Lazy::new(|| {
    HashMap::new()
}));
export_static!(self, crate::core_mod_api::script::core::internals::statics::METHOD_REGISTRY: Lazy<HashMap<MethodSignature, fn(Vec<Dynamic>) -> Dynamic>> = Lazy::new(|| {
    HashMap::new()
}));
export_static!(self, crate::core_mod_api::script::core::internals::statics::STATIC_FUNCTION_REGISTRY: Lazy<HashMap<StaticFunctionSignature, fn(Vec<Dynamic>) -> Dynamic>> = Lazy::new(|| {
    HashMap::new()
}));
