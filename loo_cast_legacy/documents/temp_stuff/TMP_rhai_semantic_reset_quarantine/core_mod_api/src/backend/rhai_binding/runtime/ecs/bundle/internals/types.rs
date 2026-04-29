use crate::bevy::ecs::world::EntityWorldMut as BevyEntityWorldMut;
use crate::rhai_binding::runtime::ecs::bundle::internals::trait_objects::BundleTraitObject;
use crate::rhai_binding::runtime::ecs::dispatch_policy::{validate_trait_path_id, validate_type_path_id};

pub type BundleSpawnDispatchKey = (String, String);
pub type BundleSpawnDispatchFn = fn(&mut BevyEntityWorldMut, BundleTraitObject);

pub fn bundle_spawn_dispatch_key_from_paths(instance_type_id: &str, trait_id: &str) -> BundleSpawnDispatchKey {
    validate_type_path_id("bundle_spawn_dispatch_key_from_paths::instance_type_id", instance_type_id);
    validate_trait_path_id("bundle_spawn_dispatch_key_from_paths::trait_id", trait_id);

    (instance_type_id.to_string(), trait_id.to_string())
}

inventory::collect!(BundleSpawnDispatchEntry);
pub struct BundleSpawnDispatchEntry {
    pub signature_id: &'static str,
    pub instance_type_id: &'static str,
    pub trait_id: &'static str,
    pub dispatch: BundleSpawnDispatchFn,
}
