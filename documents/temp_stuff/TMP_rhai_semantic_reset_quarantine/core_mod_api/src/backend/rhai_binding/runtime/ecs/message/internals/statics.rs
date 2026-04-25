use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::rhai_binding::runtime::ecs::dispatch_policy::{validate_message_signature_id, validate_type_path_id};
use crate::rhai_binding::runtime::ecs::message::internals::types::{
    MessageDrainDispatchEntry, MessageDrainDispatchFn, MessageWriteDispatchEntry, MessageWriteDispatchFn,
};

pub type MessageDispatchKey = String;

fn message_dispatch_key(message_type_id: &str) -> MessageDispatchKey {
    validate_type_path_id("message_dispatch_key::message_type_id", message_type_id);
    message_type_id.to_string()
}

static MESSAGE_WRITE_DISPATCH_REGISTRY: Lazy<HashMap<MessageDispatchKey, MessageWriteDispatchFn>> = Lazy::new(|| {
    let mut registry = HashMap::new();
    let mut signature_by_key: HashMap<MessageDispatchKey, &'static str> = HashMap::new();
    for entry in inventory::iter::<MessageWriteDispatchEntry> {
        validate_message_signature_id(entry.signature_id);
        validate_type_path_id("MessageWriteDispatchEntry::message_type_id", entry.message_type_id);

        let dispatch_key = message_dispatch_key(entry.message_type_id);
        if let Some(existing_signature) = signature_by_key.insert(dispatch_key.clone(), entry.signature_id) {
            panic!(
                "Duplicate message write dispatcher registration for message_type_id='{}': '{}' conflicts with '{}'",
                dispatch_key, existing_signature, entry.signature_id
            );
        }
        registry.insert(dispatch_key, entry.dispatch);
    }
    registry
});

static MESSAGE_DRAIN_DISPATCH_REGISTRY: Lazy<HashMap<MessageDispatchKey, MessageDrainDispatchFn>> = Lazy::new(|| {
    let mut registry = HashMap::new();
    let mut signature_by_key: HashMap<MessageDispatchKey, &'static str> = HashMap::new();
    for entry in inventory::iter::<MessageDrainDispatchEntry> {
        validate_message_signature_id(entry.signature_id);
        validate_type_path_id("MessageDrainDispatchEntry::message_type_id", entry.message_type_id);

        let dispatch_key = message_dispatch_key(entry.message_type_id);
        if let Some(existing_signature) = signature_by_key.insert(dispatch_key.clone(), entry.signature_id) {
            panic!(
                "Duplicate message drain dispatcher registration for message_type_id='{}': '{}' conflicts with '{}'",
                dispatch_key, existing_signature, entry.signature_id
            );
        }
        registry.insert(dispatch_key, entry.dispatch);
    }
    registry
});

pub fn resolve_message_write_dispatch(message_type_id: &str) -> MessageWriteDispatchFn {
    let key = message_dispatch_key(message_type_id);

    MESSAGE_WRITE_DISPATCH_REGISTRY.get(&key).copied().unwrap_or_else(|| {
        let available = MESSAGE_WRITE_DISPATCH_REGISTRY.keys().map(String::as_str).collect::<Vec<_>>().join(", ");
        panic!(
            "No message write dispatcher registered for message_type_id='{}'. Available dispatchers: [{}]",
            message_type_id, available
        )
    })
}

pub fn resolve_message_drain_dispatch(message_type_id: &str) -> MessageDrainDispatchFn {
    let key = message_dispatch_key(message_type_id);

    MESSAGE_DRAIN_DISPATCH_REGISTRY.get(&key).copied().unwrap_or_else(|| {
        let available = MESSAGE_DRAIN_DISPATCH_REGISTRY.keys().map(String::as_str).collect::<Vec<_>>().join(", ");
        panic!(
            "No message drain dispatcher registered for message_type_id='{}'. Available dispatchers: [{}]",
            message_type_id, available
        )
    })
}

pub fn registered_message_type_ids() -> Vec<String> {
    MESSAGE_WRITE_DISPATCH_REGISTRY.keys().cloned().collect::<Vec<_>>()
}

pub fn is_registered_message_type(message_type_id: &str) -> bool {
    let key = message_dispatch_key(message_type_id);
    MESSAGE_WRITE_DISPATCH_REGISTRY.contains_key(&key) || MESSAGE_DRAIN_DISPATCH_REGISTRY.contains_key(&key)
}
