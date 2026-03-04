use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::script::ecs::query::internals::types::{query_dispatch_key, QueryDispatchEntry, QueryDispatchFn, QueryDispatchKey};

static QUERY_DISPATCH_REGISTRY: Lazy<HashMap<QueryDispatchKey, QueryDispatchFn>> = Lazy::new(|| {
    let mut registry = HashMap::new();
    for entry in inventory::iter::<QueryDispatchEntry> {
        registry.insert(query_dispatch_key(entry.data_id, entry.filter_id), entry.dispatch);
    }
    registry
});

pub fn query_dispatch_registry() -> &'static HashMap<QueryDispatchKey, QueryDispatchFn> {
    &QUERY_DISPATCH_REGISTRY
}
