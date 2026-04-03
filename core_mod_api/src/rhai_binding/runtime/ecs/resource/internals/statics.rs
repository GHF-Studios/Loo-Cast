use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::rhai_binding::runtime::ecs::dispatch_policy::{validate_resource_signature_id, validate_type_path_id};
use crate::rhai_binding::runtime::ecs::resource::internals::types::{
    ResourceGetDispatchEntry, ResourceGetDispatchFn, ResourceGetMutDispatchEntry, ResourceGetMutDispatchFn, ResourceInitDispatchEntry,
    ResourceInitDispatchFn, ResourceInsertDispatchEntry, ResourceInsertDispatchFn, ResourceRemoveDispatchEntry, ResourceRemoveDispatchFn,
};

pub type ResourceDispatchKey = String;

fn resource_dispatch_key(resource_type_id: &str) -> ResourceDispatchKey {
    validate_type_path_id("resource_dispatch_key::resource_type_id", resource_type_id);
    resource_type_id.to_string()
}

static RESOURCE_INSERT_DISPATCH_REGISTRY: Lazy<HashMap<ResourceDispatchKey, ResourceInsertDispatchFn>> = Lazy::new(|| {
    let mut registry = HashMap::new();
    let mut signature_by_key: HashMap<ResourceDispatchKey, &'static str> = HashMap::new();
    for entry in inventory::iter::<ResourceInsertDispatchEntry> {
        validate_resource_signature_id(entry.signature_id);
        validate_type_path_id("ResourceInsertDispatchEntry::resource_type_id", entry.resource_type_id);

        let dispatch_key = resource_dispatch_key(entry.resource_type_id);
        if let Some(existing_signature) = signature_by_key.insert(dispatch_key.clone(), entry.signature_id) {
            panic!(
                "Duplicate resource insert dispatcher registration for resource_type_id='{}': '{}' conflicts with '{}'",
                dispatch_key, existing_signature, entry.signature_id
            );
        }
        registry.insert(dispatch_key, entry.dispatch);
    }
    registry
});

static RESOURCE_INIT_DISPATCH_REGISTRY: Lazy<HashMap<ResourceDispatchKey, ResourceInitDispatchFn>> = Lazy::new(|| {
    let mut registry = HashMap::new();
    let mut signature_by_key: HashMap<ResourceDispatchKey, &'static str> = HashMap::new();
    for entry in inventory::iter::<ResourceInitDispatchEntry> {
        validate_resource_signature_id(entry.signature_id);
        validate_type_path_id("ResourceInitDispatchEntry::resource_type_id", entry.resource_type_id);

        let dispatch_key = resource_dispatch_key(entry.resource_type_id);
        if let Some(existing_signature) = signature_by_key.insert(dispatch_key.clone(), entry.signature_id) {
            panic!(
                "Duplicate resource init dispatcher registration for resource_type_id='{}': '{}' conflicts with '{}'",
                dispatch_key, existing_signature, entry.signature_id
            );
        }
        registry.insert(dispatch_key, entry.dispatch);
    }
    registry
});

static RESOURCE_GET_DISPATCH_REGISTRY: Lazy<HashMap<ResourceDispatchKey, ResourceGetDispatchFn>> = Lazy::new(|| {
    let mut registry = HashMap::new();
    let mut signature_by_key: HashMap<ResourceDispatchKey, &'static str> = HashMap::new();
    for entry in inventory::iter::<ResourceGetDispatchEntry> {
        validate_resource_signature_id(entry.signature_id);
        validate_type_path_id("ResourceGetDispatchEntry::resource_type_id", entry.resource_type_id);

        let dispatch_key = resource_dispatch_key(entry.resource_type_id);
        if let Some(existing_signature) = signature_by_key.insert(dispatch_key.clone(), entry.signature_id) {
            panic!(
                "Duplicate resource get dispatcher registration for resource_type_id='{}': '{}' conflicts with '{}'",
                dispatch_key, existing_signature, entry.signature_id
            );
        }
        registry.insert(dispatch_key, entry.dispatch);
    }
    registry
});

static RESOURCE_GET_MUT_DISPATCH_REGISTRY: Lazy<HashMap<ResourceDispatchKey, ResourceGetMutDispatchFn>> = Lazy::new(|| {
    let mut registry = HashMap::new();
    let mut signature_by_key: HashMap<ResourceDispatchKey, &'static str> = HashMap::new();
    for entry in inventory::iter::<ResourceGetMutDispatchEntry> {
        validate_resource_signature_id(entry.signature_id);
        validate_type_path_id("ResourceGetMutDispatchEntry::resource_type_id", entry.resource_type_id);

        let dispatch_key = resource_dispatch_key(entry.resource_type_id);
        if let Some(existing_signature) = signature_by_key.insert(dispatch_key.clone(), entry.signature_id) {
            panic!(
                "Duplicate resource get_mut dispatcher registration for resource_type_id='{}': '{}' conflicts with '{}'",
                dispatch_key, existing_signature, entry.signature_id
            );
        }
        registry.insert(dispatch_key, entry.dispatch);
    }
    registry
});

static RESOURCE_REMOVE_DISPATCH_REGISTRY: Lazy<HashMap<ResourceDispatchKey, ResourceRemoveDispatchFn>> = Lazy::new(|| {
    let mut registry = HashMap::new();
    let mut signature_by_key: HashMap<ResourceDispatchKey, &'static str> = HashMap::new();
    for entry in inventory::iter::<ResourceRemoveDispatchEntry> {
        validate_resource_signature_id(entry.signature_id);
        validate_type_path_id("ResourceRemoveDispatchEntry::resource_type_id", entry.resource_type_id);

        let dispatch_key = resource_dispatch_key(entry.resource_type_id);
        if let Some(existing_signature) = signature_by_key.insert(dispatch_key.clone(), entry.signature_id) {
            panic!(
                "Duplicate resource remove dispatcher registration for resource_type_id='{}': '{}' conflicts with '{}'",
                dispatch_key, existing_signature, entry.signature_id
            );
        }
        registry.insert(dispatch_key, entry.dispatch);
    }
    registry
});

pub fn resolve_resource_insert_dispatch(resource_type_id: &str) -> ResourceInsertDispatchFn {
    let key = resource_dispatch_key(resource_type_id);
    RESOURCE_INSERT_DISPATCH_REGISTRY.get(&key).copied().unwrap_or_else(|| {
        let available = RESOURCE_INSERT_DISPATCH_REGISTRY
            .keys()
            .map(String::as_str)
            .collect::<Vec<_>>()
            .join(", ");
        panic!(
            "No resource insert dispatcher registered for resource_type_id='{}'. Available dispatchers: [{}]",
            resource_type_id, available
        )
    })
}

pub fn resolve_resource_init_dispatch(resource_type_id: &str) -> ResourceInitDispatchFn {
    let key = resource_dispatch_key(resource_type_id);
    RESOURCE_INIT_DISPATCH_REGISTRY.get(&key).copied().unwrap_or_else(|| {
        let available = RESOURCE_INIT_DISPATCH_REGISTRY.keys().map(String::as_str).collect::<Vec<_>>().join(", ");
        panic!(
            "No resource init dispatcher registered for resource_type_id='{}'. Available dispatchers: [{}]",
            resource_type_id, available
        )
    })
}

pub fn resolve_resource_get_dispatch(resource_type_id: &str) -> ResourceGetDispatchFn {
    let key = resource_dispatch_key(resource_type_id);
    RESOURCE_GET_DISPATCH_REGISTRY.get(&key).copied().unwrap_or_else(|| {
        let available = RESOURCE_GET_DISPATCH_REGISTRY.keys().map(String::as_str).collect::<Vec<_>>().join(", ");
        panic!(
            "No resource get dispatcher registered for resource_type_id='{}'. Available dispatchers: [{}]",
            resource_type_id, available
        )
    })
}

pub fn resolve_resource_get_mut_dispatch(resource_type_id: &str) -> ResourceGetMutDispatchFn {
    let key = resource_dispatch_key(resource_type_id);
    RESOURCE_GET_MUT_DISPATCH_REGISTRY.get(&key).copied().unwrap_or_else(|| {
        let available = RESOURCE_GET_MUT_DISPATCH_REGISTRY.keys().map(String::as_str).collect::<Vec<_>>().join(", ");
        panic!(
            "No resource get_mut dispatcher registered for resource_type_id='{}'. Available dispatchers: [{}]",
            resource_type_id, available
        )
    })
}

pub fn resolve_resource_remove_dispatch(resource_type_id: &str) -> ResourceRemoveDispatchFn {
    let key = resource_dispatch_key(resource_type_id);
    RESOURCE_REMOVE_DISPATCH_REGISTRY.get(&key).copied().unwrap_or_else(|| {
        let available = RESOURCE_REMOVE_DISPATCH_REGISTRY.keys().map(String::as_str).collect::<Vec<_>>().join(", ");
        panic!(
            "No resource remove dispatcher registered for resource_type_id='{}'. Available dispatchers: [{}]",
            resource_type_id, available
        )
    })
}

pub fn registered_resource_type_ids() -> Vec<String> {
    RESOURCE_INSERT_DISPATCH_REGISTRY.keys().cloned().collect::<Vec<_>>()
}

pub fn is_registered_resource_type(resource_type_id: &str) -> bool {
    let key = resource_dispatch_key(resource_type_id);
    RESOURCE_INSERT_DISPATCH_REGISTRY.contains_key(&key)
        || RESOURCE_INIT_DISPATCH_REGISTRY.contains_key(&key)
        || RESOURCE_GET_DISPATCH_REGISTRY.contains_key(&key)
        || RESOURCE_GET_MUT_DISPATCH_REGISTRY.contains_key(&key)
        || RESOURCE_REMOVE_DISPATCH_REGISTRY.contains_key(&key)
}
