use crate::bevy::ecs::world::EntityWorldMut as BevyEntityWorldMut;
use crate::rhai_binding::runtime::ecs::bundle::internals::trait_objects::BundleTraitObject;

pub type BundleSpawnDispatchKey = (String, String);
pub type BundleSpawnDispatchFn = fn(&mut BevyEntityWorldMut, BundleTraitObject);

pub fn bundle_spawn_dispatch_key_from_paths(instance_type_id: &str, trait_id: &str) -> BundleSpawnDispatchKey {
    if instance_type_id.trim().is_empty() {
        panic!("bundle spawn dispatch key requires a non-empty instance_type_id");
    }
    if trait_id.trim().is_empty() {
        panic!("bundle spawn dispatch key requires a non-empty trait_id");
    }

    (instance_type_id.to_string(), trait_id.to_string())
}

inventory::collect!(BundleSpawnDispatchEntry);
pub struct BundleSpawnDispatchEntry {
    pub signature_id: &'static str,
    pub instance_type_id: &'static str,
    pub trait_id: &'static str,
    pub dispatch: BundleSpawnDispatchFn,
}
