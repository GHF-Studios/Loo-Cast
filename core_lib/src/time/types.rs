use bevy::prelude::*;
use pin_project_lite::pin_project;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};
use std::sync::atomic::Ordering;
use std::time::Duration;

use super::errors::TimeoutError;
use super::statics::{ELAPSED_VIRTUAL_NANOS, PENDING_VIRTUAL_SLEEPS};

#[derive(Clone, Default, Debug, PartialEq, Eq, Reflect)]
pub enum PauseState {
    #[default]
    Running,
    Paused,
    Step,
}
impl PauseState {
    pub fn is_paused(&self) -> bool {
        matches!(self, PauseState::Paused | PauseState::Step)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Reflect)]
pub enum StepConfig {
    Cycles(u32),
    Seconds(f32),
}
impl Default for StepConfig {
    fn default() -> Self {
        StepConfig::Cycles(1)
    }
}

#[derive(Debug)]
pub struct PendingSleep {
    pub deadline: u64,
    pub waker: Waker,
}

#[derive(Debug)]
pub struct VirtualSleep {
    deadline: u64,
    registered: bool,
}

impl VirtualSleep {
    pub fn new(duration: Duration) -> Self {
        let now = ELAPSED_VIRTUAL_NANOS.load(Ordering::Relaxed);
        let nanos = duration.as_nanos() as u64;
        VirtualSleep {
            deadline: now + nanos,
            registered: false,
        }
    }
}

impl Future for VirtualSleep {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let now = ELAPSED_VIRTUAL_NANOS.load(Ordering::Relaxed);
        if now >= self.deadline {
            return Poll::Ready(());
        }

        if !self.registered {
            let waker = cx.waker().clone();
            PENDING_VIRTUAL_SLEEPS
                .lock()
                .unwrap()
                .push(PendingSleep {
                    deadline: self.deadline,
                    waker,
                });
            self.registered = true;
        }

        Poll::Pending
    }
}

pin_project! {
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    #[derive(Debug)]
    pub struct VirtualTimeout<F> {
        #[pin]
        pub(super) future: F,

        #[pin]
        pub(super) sleeper: VirtualSleep,
    }
}

impl<T> VirtualTimeout<T> {
    //pub(crate) fn new_with_delay(future: T, delay: Sleep) -> VirtualTimeout<T> {
    //    // TODO
    //}

    /// Gets a reference to the underlying future in this virtual timeout.
    pub fn get_ref(&self) -> &T {
        &self.future
    }

    /// Gets a mutable reference to the underlying future in this virtual timeout.
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.future
    }

    /// Consumes this virtual timeout, returning the underlying future.
    pub fn into_inner(self) -> T {
        self.future
    }
}

impl<F> Future for VirtualTimeout<F>
where
    F: Future,
{
    type Output = Result<F::Output, TimeoutError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let me = self.project();

        // ---- Step 1: Poll inner future first ----
        if let Poll::Ready(output) = me.future.poll(cx) {
            return Poll::Ready(Ok(output));
        }

        // ---- Step 2: Define timeout poller ----
        let poll_sleep = || -> Poll<Self::Output> {
            match me.sleeper.poll(cx) {
                Poll::Ready(()) => Poll::Ready(Err(TimeoutError)),
                Poll::Pending => Poll::Pending,
            }
        };

        // ---- Step 3: (Optional) Coop hook goes here ----
        // if let (true, false) = (had_budget_before, has_budget_now) {
        //     with_unconstrained(poll_sleep)
        // } else {
        //     poll_sleep()
        // }

        // We skip coop for now and just poll the sleep directly
        poll_sleep()
    }
}