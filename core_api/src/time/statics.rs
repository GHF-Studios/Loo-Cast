use crate::statics::get_ref;
use std::sync::atomic::AtomicU64;
use std::sync::Mutex;

use super::types::PendingSleep;

/// Registry key: "elapsed_virtual_nanos"
pub fn elapsed_virtual_nanos() -> &'static AtomicU64 {
    get_ref("elapsed_virtual_nanos")
}

/// Registry key: "pending_virtual_sleeps"
pub fn pending_virtual_sleeps() -> &'static Mutex<Vec<PendingSleep>> {
    get_ref("pending_virtual_sleeps")
}
