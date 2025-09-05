use std::sync::atomic::AtomicU64;
use once_cell::sync::Lazy;
use std::sync::Mutex;

use super::types::PendingSleep;

pub static ELAPSED_VIRTUAL_NANOS: AtomicU64 = AtomicU64::new(0);
pub static PENDING_VIRTUAL_SLEEPS: Lazy<Mutex<Vec<PendingSleep>>> = Lazy::new(|| Mutex::new(vec![]));