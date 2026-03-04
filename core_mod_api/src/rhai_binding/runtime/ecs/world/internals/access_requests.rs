use crate::rhai_binding::runtime::ecs::query::bindings::types::{QueryData, QueryFilter};

pub const WORLD_ACCESS_METHOD_QUERY: &str = "query";
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

pub struct WriteProbeMessageRequest {
    pub payload: String,
}

impl WriteProbeMessageRequest {
    pub fn new(payload: String) -> Self {
        Self { payload }
    }
}
