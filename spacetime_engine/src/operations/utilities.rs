use bevy::log::*;
use futures::TryFuture;
use tokio::sync::oneshot;
use futures::future::poll_fn;
use std::task::{Context, Poll};
use std::pin::Pin;
use super::singletons::*;
use super::traits::*;

pub async fn run_op<T: Operation>(operation_args: T::Args) -> T::Result {
    let (sender, mut receiver) = oneshot::channel::<T::Result>();

    debug!("Created channel");

    let operation = T::new(operation_args, sender);

    debug!("Created operation");

    {
        let mut operation_queue = OPERATION_QUEUE.lock().unwrap();

        debug!("Locked operation queue");

        operation_queue.add_operation(Box::new(operation));

        debug!("Added operation to queue");
    }

    // Use `poll_fn` to repeatedly poll the receiver until it is ready
    poll_fn(move |cx: &mut Context<'_>| {
        // Pin the receiver for polling
        let mut pinned_receiver = Pin::new(&mut receiver);

        // Try to poll the receiver for a result
        match pinned_receiver.try_poll(cx) {
            Poll::Ready(Ok(result)) => {
                debug!("Received operation result");
                Poll::Ready(result) // When the result is ready, return it
            },
            Poll::Ready(Err(e)) => {
                panic!("Failed to receive operation result: {}!", e);
            },
            Poll::Pending => {
                debug!("Operation result not ready yet, yielding");
                Poll::Pending // Yield the task, come back and poll again later
            }
        }
    }).await // This awaits until `Poll::Ready` is returned
}
