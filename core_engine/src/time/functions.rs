use std::future::IntoFuture;
use std::time::Duration;
use std::sync::atomic::Ordering;

use super::types::{VirtualSleep, VirtualTimeout};
use super::statics::ELAPSED_VIRTUAL_NANOS;

#[track_caller]
pub fn virtual_timeout<F>(duration: Duration, future: F) -> VirtualTimeout<F::IntoFuture>
where
    F: IntoFuture,
{
    VirtualTimeout {
        future: future.into_future(),
        sleeper: VirtualSleep::new(duration),
    }
}
