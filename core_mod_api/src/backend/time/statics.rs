use core_engine_macros::export_static;
use std::sync::Mutex;
use std::sync::atomic::AtomicU64;

use super::types::PendingSleep;

export_static!(self, crate::core_mod_api::time::statics::ELAPSED_VIRTUAL_NANOS: AtomicU64 = AtomicU64::new(0));
export_static!(self, crate::core_mod_api::time::statics::PENDING_VIRTUAL_SLEEPS: Mutex<Vec<PendingSleep>> = Mutex::new(vec![]));
