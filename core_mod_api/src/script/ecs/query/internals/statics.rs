use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::script::ecs::query::internals::types::{
    query_data_key, query_dispatch_key, query_filter_key, QueryDispatchEntry, QueryDispatchFn, QueryDispatchKey,
};

static QUERY_DISPATCH_REGISTRY: Lazy<HashMap<QueryDispatchKey, QueryDispatchFn>> = Lazy::new(|| {
    let mut registry = HashMap::new();
    for entry in inventory::iter::<QueryDispatchEntry> {
        let data_key = query_data_key(entry.data_terms);
        let filter_key = query_filter_key(entry.filter_with, entry.filter_without);
        let dispatch_key = query_dispatch_key(data_key.as_str(), filter_key.as_str());
        if registry.insert(dispatch_key.clone(), entry.dispatch).is_some() {
            panic!(
                "Duplicate query dispatcher registration for data_key='{}', filter_key='{}'",
                dispatch_key.0, dispatch_key.1
            );
        }
    }
    registry
});

pub fn query_dispatch_registry() -> &'static HashMap<QueryDispatchKey, QueryDispatchFn> {
    &QUERY_DISPATCH_REGISTRY
}
