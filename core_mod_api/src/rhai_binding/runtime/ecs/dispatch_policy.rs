//! Canonical policy for generic-like Rhai dispatch bindings.
//!
//! Rhai cannot request new Rust monomorphizations at runtime. Instead, the
//! dialect exposes "generic-like" behavior through:
//!
//! 1. reflected generic metadata (`reflect_extern_generic_*`),
//! 2. compile-time signature catalogs (`inventory::submit!`),
//! 3. runtime resolver/provider dispatch by normalized keys.
//!
//! This module centralizes contract validation and submission shapes so query,
//! message, and bundle paths follow one policy.

/// Prefix required for query signature IDs.
pub const QUERY_SIGNATURE_PREFIX: &str = "QUERY_SIG__";
/// Prefix required for message signature IDs.
pub const MESSAGE_SIGNATURE_PREFIX: &str = "MESSAGE_SIG__";
/// Prefix required for bundle signature IDs.
pub const BUNDLE_SIGNATURE_PREFIX: &str = "BUNDLE_SIG__";

/// Validate query signature ID format.
pub fn validate_query_signature_id(signature_id: &str) {
    validate_signature_id(signature_id, QUERY_SIGNATURE_PREFIX, "query");
}

/// Validate message signature ID format.
pub fn validate_message_signature_id(signature_id: &str) {
    validate_signature_id(signature_id, MESSAGE_SIGNATURE_PREFIX, "message");
}

/// Validate bundle signature ID format.
pub fn validate_bundle_signature_id(signature_id: &str) {
    validate_signature_id(signature_id, BUNDLE_SIGNATURE_PREFIX, "bundle");
}

/// Validate canonical Rust-style type path IDs used by dispatch contracts.
pub fn validate_type_path_id(context: &str, type_id: &str) {
    validate_path_like_id(context, "type id", type_id);
}

/// Validate canonical Rust-style trait path IDs used by dispatch contracts.
pub fn validate_trait_path_id(context: &str, trait_id: &str) {
    validate_path_like_id(context, "trait id", trait_id);
}

/// Validate a homogeneous list of canonical type-path IDs.
pub fn validate_type_path_list(context: &str, values: &[&str]) {
    for value in values {
        validate_type_path_id(context, value);
    }
}

fn validate_signature_id(signature_id: &str, expected_prefix: &str, domain: &str) {
    if signature_id.trim().is_empty() {
        panic!("Invalid {domain} signature id: value cannot be empty");
    }
    if signature_id.contains(char::is_whitespace) {
        panic!("Invalid {domain} signature id '{signature_id}': whitespace is not allowed");
    }
    if !signature_id.starts_with(expected_prefix) {
        panic!(
            "Invalid {domain} signature id '{signature_id}': expected prefix '{expected_prefix}'"
        );
    }
}

fn validate_path_like_id(context: &str, kind: &str, value: &str) {
    if value.trim().is_empty() {
        panic!("{context} requires a non-empty {kind}");
    }
    if value != value.trim() {
        panic!("{context} contains leading/trailing whitespace for {kind}: '{value}'");
    }
    if value.contains(char::is_whitespace) {
        panic!("{context} contains whitespace inside {kind}: '{value}'");
    }
    if !value.contains("::") {
        panic!("{context} expects canonical Rust path-style {kind}, got '{value}'");
    }
}

macro_rules! submit_query_dispatch_entry {
    (
        signature_id = $signature_id:expr,
        data_terms = $data_terms:expr,
        filter_with = $filter_with:expr,
        filter_without = $filter_without:expr,
        dispatch = $dispatch:path $(,)?
    ) => {
        inventory::submit! {
            crate::rhai_binding::runtime::ecs::system::query::internals::types::QueryDispatchEntry {
                signature_id: $signature_id,
                data_terms: $data_terms,
                filter_with: $filter_with,
                filter_without: $filter_without,
                dispatch: $dispatch,
            }
        }
    };
}
pub(crate) use submit_query_dispatch_entry;

macro_rules! submit_message_write_dispatch_entry {
    (
        signature_id = $signature_id:expr,
        message_type_id = $message_type_id:expr,
        dispatch = $dispatch:path $(,)?
    ) => {
        inventory::submit! {
            crate::rhai_binding::runtime::ecs::message::internals::types::MessageWriteDispatchEntry {
                signature_id: $signature_id,
                message_type_id: $message_type_id,
                dispatch: $dispatch,
            }
        }
    };
}
pub(crate) use submit_message_write_dispatch_entry;

macro_rules! submit_message_drain_dispatch_entry {
    (
        signature_id = $signature_id:expr,
        message_type_id = $message_type_id:expr,
        dispatch = $dispatch:path $(,)?
    ) => {
        inventory::submit! {
            crate::rhai_binding::runtime::ecs::message::internals::types::MessageDrainDispatchEntry {
                signature_id: $signature_id,
                message_type_id: $message_type_id,
                dispatch: $dispatch,
            }
        }
    };
}
pub(crate) use submit_message_drain_dispatch_entry;

macro_rules! submit_bundle_spawn_dispatch_entry {
    (
        signature_id = $signature_id:expr,
        instance_type_id = $instance_type_id:expr,
        trait_id = $trait_id:expr,
        dispatch = $dispatch:path $(,)?
    ) => {
        inventory::submit! {
            crate::rhai_binding::runtime::ecs::bundle::internals::types::BundleSpawnDispatchEntry {
                signature_id: $signature_id,
                instance_type_id: $instance_type_id,
                trait_id: $trait_id,
                dispatch: $dispatch,
            }
        }
    };
}
pub(crate) use submit_bundle_spawn_dispatch_entry;

#[cfg(test)]
mod tests {
    use super::{
        validate_bundle_signature_id,
        validate_message_signature_id,
        validate_query_signature_id,
        validate_trait_path_id,
        validate_type_path_id,
        validate_type_path_list,
    };
    use std::collections::HashSet;

    use crate::rhai_binding::bridges::domains::bevy::ecs::catalog::{
        bundle_signatures::{
            BUNDLE_SIG__PLAYER__SPAWN_SINGLE,
            TYPE_PATH__PLAYER_BUNDLE,
            TYPE_PATH__TRAIT_BUNDLE,
        },
        message_signatures::{
            MESSAGE_SIG__SCRIPT_PROBE__DRAIN,
            MESSAGE_SIG__SCRIPT_PROBE__WRITE,
            TYPE_PATH__SCRIPT_PROBE_MESSAGE,
        },
        query_signatures::{
            QUERY_SIG__ENTITY,
            QUERY_SIG__ENTITY__WITH_PLAYER,
            TYPE_PATH__ENTITY,
            TYPE_PATH__PLAYER,
        },
    };
    use crate::rhai_binding::internals::statics::RUNTIME_BINDING_GRAPH;
    use crate::rhai_binding::runtime::ecs::bundle::internals::statics::bundle_spawn_dispatch_registry;
    use crate::rhai_binding::runtime::ecs::bundle::internals::types::{
        bundle_spawn_dispatch_key_from_paths, BundleSpawnDispatchEntry,
    };
    use crate::rhai_binding::runtime::ecs::message::internals::statics::{
        resolve_message_drain_dispatch, resolve_message_write_dispatch,
    };
    use crate::rhai_binding::runtime::ecs::message::internals::types::{
        MessageDrainDispatchEntry, MessageWriteDispatchEntry,
    };
    use crate::rhai_binding::runtime::ecs::system::query::internals::statics::resolve_query_dispatch;
    use crate::rhai_binding::runtime::ecs::system::query::internals::types::{
        query_data_key, query_filter_key, QueryDispatchAccess, QueryDispatchEntry, QueryDispatchTerm,
    };

    const QUERY_GENERIC_ID: &str = "bevy::ecs::system::Query<TData, TFilter>";
    const QUERY_GENERIC_INSTANCE_ID: &str =
        "bevy::ecs::system::Query<bevy::ecs::entity::Entity, core_mod_api::player::components::Player>";

    #[test]
    fn query_signatures_follow_policy_and_are_registered() {
        let mut ids = HashSet::new();

        for entry in inventory::iter::<QueryDispatchEntry> {
            validate_query_signature_id(entry.signature_id);
            for term in entry.data_terms {
                validate_type_path_id("QueryDispatchEntry::data_terms.type_id", term.type_id);
            }
            validate_type_path_list("QueryDispatchEntry::filter_with", entry.filter_with);
            validate_type_path_list("QueryDispatchEntry::filter_without", entry.filter_without);
            assert!(
                ids.insert(entry.signature_id),
                "Duplicate query signature id '{}'",
                entry.signature_id
            );
        }

        assert!(ids.contains(QUERY_SIG__ENTITY));
        assert!(ids.contains(QUERY_SIG__ENTITY__WITH_PLAYER));
    }

    #[test]
    fn message_signatures_follow_policy_and_are_registered() {
        let mut write_ids = HashSet::new();
        for entry in inventory::iter::<MessageWriteDispatchEntry> {
            validate_message_signature_id(entry.signature_id);
            validate_type_path_id("MessageWriteDispatchEntry::message_type_id", entry.message_type_id);
            assert!(
                write_ids.insert(entry.signature_id),
                "Duplicate message write signature id '{}'",
                entry.signature_id
            );
        }

        let mut drain_ids = HashSet::new();
        for entry in inventory::iter::<MessageDrainDispatchEntry> {
            validate_message_signature_id(entry.signature_id);
            validate_type_path_id("MessageDrainDispatchEntry::message_type_id", entry.message_type_id);
            assert!(
                drain_ids.insert(entry.signature_id),
                "Duplicate message drain signature id '{}'",
                entry.signature_id
            );
        }

        assert!(write_ids.contains(MESSAGE_SIG__SCRIPT_PROBE__WRITE));
        assert!(drain_ids.contains(MESSAGE_SIG__SCRIPT_PROBE__DRAIN));
    }

    #[test]
    fn bundle_signatures_follow_policy_and_are_registered() {
        let mut ids = HashSet::new();
        for entry in inventory::iter::<BundleSpawnDispatchEntry> {
            validate_bundle_signature_id(entry.signature_id);
            validate_type_path_id("BundleSpawnDispatchEntry::instance_type_id", entry.instance_type_id);
            validate_trait_path_id("BundleSpawnDispatchEntry::trait_id", entry.trait_id);
            assert!(
                ids.insert(entry.signature_id),
                "Duplicate bundle signature id '{}'",
                entry.signature_id
            );
        }

        assert!(ids.contains(BUNDLE_SIG__PLAYER__SPAWN_SINGLE));
    }

    #[test]
    fn query_metadata_and_dispatch_catalog_are_coherent() {
        let graph = RUNTIME_BINDING_GRAPH();
        assert!(graph.generic_definitions.contains_key(QUERY_GENERIC_ID));
        let Some(instantiation) = graph.generic_instantiations.get(QUERY_GENERIC_INSTANCE_ID) else {
            panic!("Missing query generic instantiation metadata '{QUERY_GENERIC_INSTANCE_ID}'");
        };
        assert_eq!(instantiation.generic_id.get().as_str(), QUERY_GENERIC_ID);

        let data_key = query_data_key(&[QueryDispatchTerm {
            type_id: TYPE_PATH__ENTITY,
            access: QueryDispatchAccess::Value,
        }]);
        let no_filter_key = query_filter_key(&[], &[]);
        let player_filter_key = query_filter_key(&[TYPE_PATH__PLAYER], &[]);

        let _ = resolve_query_dispatch(data_key.as_str(), no_filter_key.as_str());
        let _ = resolve_query_dispatch(data_key.as_str(), player_filter_key.as_str());
    }

    #[test]
    fn message_catalog_resolves_known_signatures() {
        let _ = resolve_message_write_dispatch(TYPE_PATH__SCRIPT_PROBE_MESSAGE);
        let _ = resolve_message_drain_dispatch(TYPE_PATH__SCRIPT_PROBE_MESSAGE);
    }

    #[test]
    fn bundle_catalog_contains_known_spawn_signature() {
        let key = bundle_spawn_dispatch_key_from_paths(TYPE_PATH__PLAYER_BUNDLE, TYPE_PATH__TRAIT_BUNDLE);
        assert!(
            bundle_spawn_dispatch_registry().contains_key(&key),
            "Missing bundle spawn registry entry for ({}, {})",
            TYPE_PATH__PLAYER_BUNDLE,
            TYPE_PATH__TRAIT_BUNDLE
        );
    }
}
