use std::sync::atomic::AtomicU64;

pub static ELAPSED_VIRTUAL_NANOS: AtomicU64 = AtomicU64::new(0);