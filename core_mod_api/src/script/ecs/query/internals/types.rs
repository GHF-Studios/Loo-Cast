use crate::bevy::prelude::World as BevyWorld;

use crate::script::ecs::query::bindings::types::Query;

pub type QueryDispatchKey = (String, String);
pub type QueryDispatchFn = fn(&mut BevyWorld) -> Query;

pub fn query_dispatch_key(data_id: &str, filter_id: &str) -> QueryDispatchKey {
    (data_id.to_string(), filter_id.to_string())
}

inventory::collect!(QueryDispatchEntry);
pub struct QueryDispatchEntry {
    pub data_id: &'static str,
    pub filter_id: &'static str,
    pub dispatch: QueryDispatchFn,
}
