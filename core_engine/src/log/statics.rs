use lazy_static::lazy_static;
use std::sync::Arc;

use crate::log::{arena::Arena, resources::LogTreeHandle};

lazy_static! {
    pub static ref LOG_TREE_HANDLE: LogTreeHandle =
        LogTreeHandle(Arc::new(Arena::new()));
}
