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

    let operation = T::new(operation_args, sender);

    {
        let mut operation_queue = OPERATION_QUEUE.lock().unwrap();
        operation_queue.add_operation(Box::new(operation));
    }

    poll_fn(move |cx: &mut Context<'_>| {
        let pinned_receiver = Pin::new(&mut receiver);

        match pinned_receiver.try_poll(cx) {
            Poll::Ready(Ok(result)) => {
                Poll::Ready(result)
            },
            Poll::Ready(Err(e)) => {
                panic!("Failed to receive operation result: {}!", e);
            },
            Poll::Pending => {
                Poll::Pending
            }
        }
    }).await
}
