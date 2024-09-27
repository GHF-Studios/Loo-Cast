use tokio::sync::oneshot;
use super::singletons::*;
use super::traits::*;

pub async fn run_op<T: Operation>(operation_args: T::Args) -> T::Result {
    let (sender, receiver) = oneshot::channel::<T::Result>();

    let operation = T::new(operation_args, sender);

    {
        let mut operation_queue = OPERATION_QUEUE.lock().unwrap();
        operation_queue.add_operation(Box::new(operation));
    }

    match receiver.await {
        Ok(result) => result,
        Err(_) => {
            panic!("Failed to receive operation result!");
        }
    }
}