use once_cell::sync::OnceCell;
use std::sync::atomic::AtomicU64;
use std::sync::Mutex;

use crate::statics::get_ref;

use super::types::PendingSleep;

pub fn init_elapsed_virtual_nanos() -> AtomicU64 {
    AtomicU64::new(0)
}

pub fn init_pending_virtual_sleeps() -> Mutex<Vec<PendingSleep>> {
    Mutex::new(vec![])
}

pub fn elapsed_virtual_nanos() -> &'static AtomicU64 {
    get_ref("elapsed_virtual_nanos")
}

pub fn pending_virtual_sleeps() -> &'static Mutex<Vec<PendingSleep>> {
    get_ref("pending_virtual_sleeps")
}
