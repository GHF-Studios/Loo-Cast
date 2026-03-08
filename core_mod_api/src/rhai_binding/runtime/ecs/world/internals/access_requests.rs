use crate::rhai_binding::runtime::ecs::bundle::internals::trait_objects::BundleTraitObject;
use crate::rhai_binding::runtime::ecs::system::query::bindings::types::{QueryData, QueryFilter};

pub const WORLD_ACCESS_METHOD_QUERY: &str = "query";
pub const WORLD_ACCESS_METHOD_SPAWN_SINGLE: &str = "spawn_single";
pub const WORLD_ACCESS_METHOD_WRITE_PROBE_MESSAGE: &str = "write_probe_message";
pub const WORLD_ACCESS_METHOD_DRAIN_PROBE_MESSAGES: &str = "drain_probe_messages";

#[derive(Clone)]
pub struct WorldQueryRequest {
    pub data: QueryData,
    pub filter: QueryFilter,
}

impl WorldQueryRequest {
    pub fn new(data: QueryData, filter: QueryFilter) -> Self {
        Self { data, filter }
    }
}

pub struct WorldSpawnSingleRequest {
    pub bundle: BundleTraitObject,
}

impl WorldSpawnSingleRequest {
    pub fn new(bundle: BundleTraitObject) -> Self {
        Self { bundle }
    }
}

pub struct WriteProbeMessageRequest {
    pub payload: String,
}

impl WriteProbeMessageRequest {
    pub fn new(payload: String) -> Self {
        Self { payload }
    }
}
