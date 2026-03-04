use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::rhai_binding::runtime::ecs::dispatch_policy::{
    validate_bundle_signature_id, validate_trait_path_id, validate_type_path_id,
};
use crate::rhai_binding::runtime::ecs::bundle::internals::trait_objects::BundleTraitObject;
use crate::rhai_binding::runtime::ecs::bundle::internals::types::{
    bundle_spawn_dispatch_key_from_paths, BundleSpawnDispatchEntry, BundleSpawnDispatchFn, BundleSpawnDispatchKey,
};

static BUNDLE_SPAWN_DISPATCH_REGISTRY: Lazy<HashMap<BundleSpawnDispatchKey, BundleSpawnDispatchFn>> = Lazy::new(|| {
    let mut registry = HashMap::new();
    let mut signature_by_key: HashMap<BundleSpawnDispatchKey, &'static str> = HashMap::new();

    for entry in inventory::iter::<BundleSpawnDispatchEntry> {
        validate_bundle_signature_id(entry.signature_id);
        validate_type_path_id("BundleSpawnDispatchEntry::instance_type_id", entry.instance_type_id);
        validate_trait_path_id("BundleSpawnDispatchEntry::trait_id", entry.trait_id);

        let dispatch_key = bundle_spawn_dispatch_key_from_paths(entry.instance_type_id, entry.trait_id);
        if let Some(existing_signature) = signature_by_key.insert(dispatch_key.clone(), entry.signature_id) {
            panic!(
                "Duplicate bundle spawn dispatcher registration for instance_type_id='{}', trait_id='{}': '{}' conflicts with '{}'",
                dispatch_key.0, dispatch_key.1, existing_signature, entry.signature_id
            );
        }
        registry.insert(dispatch_key, entry.dispatch);
    }

    registry
});

pub fn bundle_spawn_dispatch_registry() -> &'static HashMap<BundleSpawnDispatchKey, BundleSpawnDispatchFn> {
    &BUNDLE_SPAWN_DISPATCH_REGISTRY
}

pub fn resolve_bundle_spawn_dispatch(bundle: &BundleTraitObject) -> BundleSpawnDispatchFn {
    let instance_type_id = bundle.0.instance_type_id.to_string();
    let trait_id = bundle.0.trait_id.id.to_string();
    let key = bundle_spawn_dispatch_key_from_paths(instance_type_id.as_str(), trait_id.as_str());

    bundle_spawn_dispatch_registry().get(&key).copied().unwrap_or_else(|| {
        let available = bundle_spawn_dispatch_registry()
            .keys()
            .map(|(existing_type_id, existing_trait_id)| format!("({existing_type_id}, {existing_trait_id})"))
            .collect::<Vec<_>>()
            .join(", ");

        panic!(
            "No bundle spawn dispatcher registered for instance_type_id='{}', trait_id='{}'. Available dispatchers: [{}]",
            instance_type_id, trait_id, available
        )
    })
}
