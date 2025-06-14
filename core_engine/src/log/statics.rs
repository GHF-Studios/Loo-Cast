use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

use crate::log::{resources::LogTreeHandle, types::LogTree};

lazy_static! {
    pub static ref LOG_TREE_HANDLE: LogTreeHandle = LogTreeHandle(Arc::new(Mutex::new(LogTree::default())));
}
