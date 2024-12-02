use std::any::Any;
use bevy::prelude::*;
use futures::future::join_all;
use futures::TryFuture;
use tokio::sync::oneshot;
use futures::future::poll_fn;
use std::task::{Context, Poll};
use std::pin::Pin;
use crate::singletons::*;
use crate::chunk::structs::ChunkPosition;
use crate::entity::structs::EntityPosition;
use crate::chunk::commands::*;
use crate::chunk_actor::commands::*;
use crate::camera::commands::*;
use crate::math::structs::I16Vec2;
use crate::traits::Operation;
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
            Poll::Ready(Err(_)) => {
                panic!("An operation has panicked!");
            },
            Poll::Pending => {
                Poll::Pending
            }
        }
    }).await
}

pub async fn spawn_main_camera(_params: Box<dyn Any>) -> Box<dyn Any> {
    let entity_position = EntityPosition(Vec2::new(0.0, 0.0));
    if let Err(e) = spawn_camera(entity_position).await {
        error!("Error spawning camera: {:?}", e);
    } else {
        debug!("Spawned camera at {:?}", entity_position);
    }

    Box::new(())
}

pub async fn spawn_start_chunks(params: Box<dyn Any>) -> Box<dyn Any> {
    let range = *params.downcast_ref::<i16>().unwrap();

    let mut tasks = Vec::new();

    for x in -range..=range {
        for y in -range..=range {
            let chunk_position = ChunkPosition(I16Vec2(x, y));

            let task = async move {
                if let Err(e) = spawn_chunk(chunk_position).await {
                    error!("Error spawning chunk: {:?}", e);
                } else {
                    debug!("Spawned chunk at {:?}", chunk_position);
                }
            };

            tasks.push(task);
        }
    }

    join_all(tasks).await;

    Box::new(())
}

pub async fn spawn_start_chunk_actors(params: Box<dyn Any>) -> Box<dyn Any> {
    let range = *params.downcast_ref::<i16>().unwrap();

    let mut tasks = Vec::new();

    for x in -range..=range {
        for y in -range..=range {
            let chunk_position = ChunkPosition(I16Vec2(x, y));
            let entity_position: EntityPosition = chunk_position.into();

            let task = async move {
                if let Err(e) = spawn_chunk_actor(entity_position).await {
                    error!("Error spawning chunk actor: {:?}", e);
                } else {
                    debug!("Spawned chunk actor at {:?}", entity_position);
                }
            };

            tasks.push(task);
        }
    }

    join_all(tasks).await;

    Box::new(())
}