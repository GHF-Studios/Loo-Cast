use std::future::IntoFuture;
use std::time::Duration;
use std::sync::atomic::Ordering;

use super::types::VirtualTimeout;
use super::statics::ELAPSED_VIRTUAL_NANOS;

#[track_caller]
pub fn virtual_timeout<F>(duration: Duration, future: F) -> VirtualTimeout<F>
where
    F: IntoFuture
{
    let start = ELAPSED_VIRTUAL_NANOS.load(Ordering::Relaxed);
    let timeout_nanos = duration.as_nanos() as u64;

    VirtualTimeout {
        future,
        start_nanos: start,
        timeout_nanos,
    }
}
