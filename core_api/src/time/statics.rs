use core_api_macros::export_static;
use std::sync::atomic::AtomicU64;
use std::sync::Mutex;

use super::types::PendingSleep;

export_static!(self, crate::time::statics::ELAPSED_VIRTUAL_NANOS: AtomicU64 = AtomicU64::new(0));
export_static!(self, crate::time::statics::PENDING_VIRTUAL_SLEEPS: Mutex<Vec<PendingSleep>> = Mutex::new(vec![]));
