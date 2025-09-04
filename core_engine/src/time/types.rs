use bevy::prelude::*;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::sync::atomic::Ordering;

use super::errors::TimeoutError;
use super::statics::ELAPSED_VIRTUAL_NANOS;

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
pub struct VirtualTimeout<F> {
    pub(super) future: F,
    pub(super) start_nanos: u64,
    pub(super) timeout_nanos: u64,
}

impl<F> Future for VirtualTimeout<F>
where
    F: Future + Unpin,
{
    type Output = Result<F::Output, TimeoutError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let now = ELAPSED_VIRTUAL_NANOS.load(Ordering::Relaxed);
        if now - self.start_nanos >= self.timeout_nanos {
            return Poll::Ready(Err(TimeoutError));
        }

        match Pin::new(&mut self.future).poll(cx) {
            Poll::Ready(output) => Poll::Ready(Ok(output)),
            Poll::Pending => Poll::Pending,
        }
    }
}