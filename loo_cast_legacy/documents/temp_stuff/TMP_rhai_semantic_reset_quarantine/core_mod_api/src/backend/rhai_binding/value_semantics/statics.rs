use std::collections::HashMap;

use core_engine_macros::export_static;
use once_cell::sync::Lazy;
use rhai::Dynamic;

use crate::reflection::{
    function_ids::{CtorId, MethodId, StaticFunctionId},
    registry::{CtorRegistryEntry, MethodRegistryEntry, StaticFunctionRegistryEntry},
    type_info::TypeInfo,
};
use crate::rhai_binding::value_semantics::{
    ids::TypeId,
    trait_object::{TraitTypeEntry, TraitTypeKey, TraitTypeVTables},
};

export_static!(
    self,
    crate::rhai_binding::value_semantics::statics::TYPE_REGISTRY: Lazy<HashMap<TypeId, &'static TypeInfo>> = Lazy::new(|| {
        inventory::iter::<TypeInfo>.into_iter().map(|entry| (entry.type_id.clone(), entry)).collect()
    })
);

export_static!(
    self,
    crate::rhai_binding::value_semantics::statics::CTOR_REGISTRY: Lazy<HashMap<CtorId, fn(Vec<Dynamic>) -> Dynamic>> = Lazy::new(|| {
        inventory::iter::<CtorRegistryEntry>.into_iter().map(|entry| (entry.id.clone(), entry.fn_ptr)).collect()
    })
);

export_static!(
    self,
    crate::rhai_binding::value_semantics::statics::METHOD_REGISTRY: Lazy<HashMap<MethodId, fn(Vec<Dynamic>) -> Dynamic>> = Lazy::new(|| {
        inventory::iter::<MethodRegistryEntry>.into_iter().map(|entry| (entry.id.clone(), entry.fn_ptr)).collect()
    })
);

export_static!(
    self,
    crate::rhai_binding::value_semantics::statics::STATIC_FUNCTION_REGISTRY: Lazy<HashMap<StaticFunctionId, fn(Vec<Dynamic>) -> Dynamic>> = Lazy::new(|| {
        inventory::iter::<StaticFunctionRegistryEntry>.into_iter().map(|entry| (entry.id.clone(), entry.fn_ptr)).collect()
    })
);

export_static!(
    self,
    crate::rhai_binding::value_semantics::statics::TRAIT_OBJECT_VTABLE_REGISTRY: Lazy<HashMap<TraitTypeKey, TraitTypeVTables>> = Lazy::new(|| {
        let mut m: HashMap<TraitTypeKey, TraitTypeVTables> = Default::default();
        for entry in inventory::iter::<TraitTypeEntry> {
            m.insert(entry.key.clone(), entry.value.clone());
        }
        m
    })
);
