//! Compile-time resource signature catalog.
//!
//! Reserve this module for resource operations registered by canonical
//! Rust type-path IDs.

use crate::bevy::prelude::World as BevyWorld;
use crate::rhai_binding::runtime::ecs::dispatch_policy::{
    submit_resource_get_dispatch_entry, submit_resource_get_mut_dispatch_entry, submit_resource_init_dispatch_entry, submit_resource_insert_dispatch_entry,
    submit_resource_remove_dispatch_entry,
};
use crate::rhai_binding::runtime::ecs::resource::bindings::types::ScriptProbeResource;

pub const RESOURCE_SIG__SCRIPT_PROBE__INSERT: &str = "RESOURCE_SIG__SCRIPT_PROBE__INSERT";
pub const RESOURCE_SIG__SCRIPT_PROBE__INIT: &str = "RESOURCE_SIG__SCRIPT_PROBE__INIT";
pub const RESOURCE_SIG__SCRIPT_PROBE__GET: &str = "RESOURCE_SIG__SCRIPT_PROBE__GET";
pub const RESOURCE_SIG__SCRIPT_PROBE__GET_MUT: &str = "RESOURCE_SIG__SCRIPT_PROBE__GET_MUT";
pub const RESOURCE_SIG__SCRIPT_PROBE__REMOVE: &str = "RESOURCE_SIG__SCRIPT_PROBE__REMOVE";

pub const TYPE_PATH__SCRIPT_PROBE_RESOURCE: &str = "core_mod_api::rhai_binding::runtime::ecs::resource::bindings::types::ScriptProbeResource";

fn dispatch_resource_sig_script_probe_insert(world: &mut BevyWorld, payload: String) {
    world.insert_resource(ScriptProbeResource { payload });
}

fn dispatch_resource_sig_script_probe_init(world: &mut BevyWorld) {
    world.init_resource::<ScriptProbeResource>();
}

fn dispatch_resource_sig_script_probe_get(world: &mut BevyWorld) -> Option<String> {
    world.get_resource::<ScriptProbeResource>().map(|resource| resource.payload.clone())
}

fn dispatch_resource_sig_script_probe_get_mut(world: &mut BevyWorld) -> Option<String> {
    world.get_resource_mut::<ScriptProbeResource>().map(|resource| resource.payload.clone())
}

fn dispatch_resource_sig_script_probe_remove(world: &mut BevyWorld) -> Option<String> {
    world.remove_resource::<ScriptProbeResource>().map(|resource| resource.payload)
}

submit_resource_insert_dispatch_entry!(
    signature_id = RESOURCE_SIG__SCRIPT_PROBE__INSERT,
    resource_type_id = TYPE_PATH__SCRIPT_PROBE_RESOURCE,
    dispatch = dispatch_resource_sig_script_probe_insert,
);

submit_resource_init_dispatch_entry!(
    signature_id = RESOURCE_SIG__SCRIPT_PROBE__INIT,
    resource_type_id = TYPE_PATH__SCRIPT_PROBE_RESOURCE,
    dispatch = dispatch_resource_sig_script_probe_init,
);

submit_resource_get_dispatch_entry!(
    signature_id = RESOURCE_SIG__SCRIPT_PROBE__GET,
    resource_type_id = TYPE_PATH__SCRIPT_PROBE_RESOURCE,
    dispatch = dispatch_resource_sig_script_probe_get,
);

submit_resource_get_mut_dispatch_entry!(
    signature_id = RESOURCE_SIG__SCRIPT_PROBE__GET_MUT,
    resource_type_id = TYPE_PATH__SCRIPT_PROBE_RESOURCE,
    dispatch = dispatch_resource_sig_script_probe_get_mut,
);

submit_resource_remove_dispatch_entry!(
    signature_id = RESOURCE_SIG__SCRIPT_PROBE__REMOVE,
    resource_type_id = TYPE_PATH__SCRIPT_PROBE_RESOURCE,
    dispatch = dispatch_resource_sig_script_probe_remove,
);
