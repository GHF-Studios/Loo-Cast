use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::script::ecs::query::internals::types::{
    query_data_key, query_dispatch_key, query_filter_key, QueryDispatchEntry, QueryDispatchFn, QueryDispatchKey,
};

static QUERY_DISPATCH_REGISTRY: Lazy<HashMap<QueryDispatchKey, QueryDispatchFn>> = Lazy::new(|| {
    let mut registry = HashMap::new();
    let mut signature_by_key: HashMap<QueryDispatchKey, &'static str> = HashMap::new();
    for entry in inventory::iter::<QueryDispatchEntry> {
        let data_key = query_data_key(entry.data_terms);
        let filter_key = query_filter_key(entry.filter_with, entry.filter_without);
        let dispatch_key = query_dispatch_key(data_key.as_str(), filter_key.as_str());
        if let Some(existing_signature) = signature_by_key.insert(dispatch_key.clone(), entry.signature_id) {
            panic!(
                "Duplicate query dispatcher registration for data_key='{}', filter_key='{}': '{}' conflicts with '{}'",
                dispatch_key.0, dispatch_key.1, existing_signature, entry.signature_id
            );
        }
        registry.insert(dispatch_key, entry.dispatch);
    }
    registry
});

pub fn query_dispatch_registry() -> &'static HashMap<QueryDispatchKey, QueryDispatchFn> {
    &QUERY_DISPATCH_REGISTRY
}

pub fn resolve_query_dispatch(data_key: &str, filter_key: &str) -> QueryDispatchFn {
    let key = query_dispatch_key(data_key, filter_key);
    query_dispatch_registry().get(&key).copied().unwrap_or_else(|| {
        let available = query_dispatch_registry()
            .keys()
            .map(|(existing_data_key, existing_filter_key)| format!("({existing_data_key}, {existing_filter_key})"))
            .collect::<Vec<_>>()
            .join(", ");

        panic!(
            "No query dispatcher registered for data_key='{}', filter_key='{}'. Available dispatchers: [{}]",
            data_key, filter_key, available
        )
    })
}
