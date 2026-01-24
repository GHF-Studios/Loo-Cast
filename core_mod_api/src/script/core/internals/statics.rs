use core_mod_macros::export_static;
use once_cell::sync::Lazy;
use std::collections::{HashSet, HashMap};
use std::sync::{Arc, Mutex};

use crate::script::core::internals::types::{TypeId, TypeInfo};

export_static!(self, crate::core_mod_api::script::core::internals::statics::SCHEDULE_HOOKS: Lazy<Mutex<HashSet<String>>> = Lazy::new(Default::default));
export_static!(self, crate::core_mod_api::script::core::internals::statics::TYPE_REGISTRY: Lazy<HashMap<TypeId, &'static TypeInfo>> = Lazy::new(|| {
    inventory::iter::<TypeInfo>.into_iter().map(|entry| (entry.type_id.clone(), entry)).collect()
}));