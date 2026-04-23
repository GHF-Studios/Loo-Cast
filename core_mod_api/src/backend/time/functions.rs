use std::future::IntoFuture;
use std::time::Duration;

use super::types::{VirtualSleep, VirtualTimeout};

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
