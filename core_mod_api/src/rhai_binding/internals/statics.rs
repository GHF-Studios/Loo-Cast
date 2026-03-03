
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
use crate::rhai_binding::meta::registry::RuntimeBindingGraph;

export_static!(self, crate::rhai_binding::internals::statics::RUNTIME_BINDING_GRAPH: Lazy<RuntimeBindingGraph> = Lazy::new(|| RuntimeBindingGraph::build()));