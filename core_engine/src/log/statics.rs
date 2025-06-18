use lazy_static::lazy_static;
use std::sync::Arc;

use crate::log::{location_tree::LocationTree, resources::{LocationTreeHandle, LogStorageHandle, SpanTreeHandle}, span_tree::SpanTree, storage::LogStorage};

lazy_static! {
    pub static ref LOG_STORAGE_HANDLE: LogStorageHandle =
        LogStorageHandle(Arc::new(LogStorage::new()));
    pub static ref SPAN_TREE_HANDLE: SpanTreeHandle =
        SpanTreeHandle(Arc::new(SpanTree::default()));
    pub static ref LOCATION_TREE_HANDLE: LocationTreeHandle =
        LocationTreeHandle(Arc::new(LocationTree::default()));
}
