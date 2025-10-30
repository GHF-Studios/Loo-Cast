use bevy::prelude::*;
use core_mod_macros::{composite_workflow, composite_workflow_return};
use tokio::task::JoinHandle;

use crate::chunk::components::Chunk;
use crate::chunk_actor::components::ChunkActor;
use crate::chunk_loader::components::ChunkLoader;
use crate::chunk_loader::enums::ZoomState;
use crate::chunk_loader::resources::RemovedChunkLoaders;
use crate::config::statics::CONFIG;
use crate::usf::pos::unit::types::UnitVec;
use crate::workflow::composite_workflow_context::ScopedCompositeWorkflowContext;
use crate::workflow::functions::handle_composite_workflow_return_now;

use super::constants::ORIGIN_OFFSET_THRESHOLD;

pub(crate) fn zoom_cooldown_system(
    time: Res<Time<Virtual>>,
    mut timer: Local<f32>,
    mut query: Query<&mut ChunkLoader>,
) {
    if *timer > 0.0 {
        *timer -= time.delta_secs();
        if *timer < 0.0 {
            *timer = 0.0;
        }
    }

    for mut chunk_loader in query.iter_mut() {
        if chunk_loader.zoom_state != ZoomState::None && *timer == 0.0 {
            chunk_loader.zoom_state = ZoomState::None;
            *timer = CONFIG().get::<f32>("chunk_loader/zoom_cooldown_secs");
        }
    }
}

#[tracing::instrument(skip_all)]
pub(crate) fn update_chunk_loader_system(mut composite_workflow_handle: Local<Option<JoinHandle<ScopedCompositeWorkflowContext>>>) {
    let handle_is_some = (*composite_workflow_handle).is_some();
    let handle_is_finished = match *composite_workflow_handle {
        Some(ref handle) => handle.is_finished(),
        None => false,
    };

    if !handle_is_some {
        let handle = composite_workflow!(UpdateChunkLoaders, {
            let categorize_chunks_output = workflow!(O, ChunkLoader::CategorizeChunks);

            let load_chunk_inputs = categorize_chunks_output.inner.load_chunk_inputs;
            let unload_chunk_inputs = categorize_chunks_output.inner.unload_chunk_inputs;

            workflow!(I, ChunkLoader::LoadChunks, Input {
                inner: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs },
            });

            workflow!(I, ChunkLoader::UnloadChunks, Input {
                inner: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs },
            });
        });

        *composite_workflow_handle = Some(handle);
    }
    if handle_is_some && !handle_is_finished {
        return;
    }

    if handle_is_some && handle_is_finished {
        let handle = composite_workflow_handle.take().unwrap();
        handle_composite_workflow_return_now(handle, |_ctx| {
            composite_workflow_return!();
        });
    }
}

#[tracing::instrument(skip_all)]
pub(crate) fn post_update_chunk_loader_system(mut removed_chunk_loaders: ResMut<RemovedChunkLoaders>) {
    removed_chunk_loaders.0.clear();
}

#[tracing::instrument(skip_all)]
pub(crate) fn origin_offset_system(
    mut chunk_loader_query: Single<(&mut ChunkLoader, &Transform), (Changed<Transform>, Without<Chunk>, Without<ChunkActor>)>,
    mut chunk_transform_query: Query<&mut Transform, (With<Chunk>, Without<ChunkActor>, Without<ChunkLoader>)>,
    mut chunk_actor_transform_query: Query<&mut Transform, (With<ChunkActor>, Without<Chunk>, Without<ChunkLoader>)>,
) {
    let (chunk_loader, transform) = &mut *chunk_loader_query;
    let unit_pos = UnitVec::new(chunk_loader.origin_offset.clone(), transform.translation.truncate());

    if unit_pos.grid_offset.scale != chunk_loader.origin_offset.scale {
        unreachable!()
    }

    if unit_pos.grid_offset != chunk_loader.origin_offset {
        let grid_diff = unit_pos.grid_offset.xy - chunk_loader.origin_offset.clone().xy;

        let mut unit_diff = Vec2::ZERO;

        if grid_diff.x.abs() >= ORIGIN_OFFSET_THRESHOLD as i32 {
            unit_diff.x = grid_diff.x as f32 * 1000.0;
        }
        if grid_diff.y.abs() >= ORIGIN_OFFSET_THRESHOLD as i32 {
            unit_diff.y = grid_diff.y as f32 * 1000.0;
        }

        for mut transform in chunk_transform_query.iter_mut().chain(chunk_actor_transform_query.iter_mut()) {
            transform.translation.x -= unit_diff.x;
            transform.translation.y -= unit_diff.y;
        }
    }
}