use bevy::prelude::*;

use crate::chunk::components::Chunk;
use crate::chunk_actor::components::ChunkActor;
use crate::chunk_loader::components::ChunkLoader;
use crate::usf::pos::grid::types::GridVec;
use crate::usf::pos::unit::types::UnitVec;

use super::constants::ORIGIN_OFFSET_THRESHOLD;

#[tracing::instrument(skip_all)]
pub(crate) fn apply_new_origin_offset_system(
    mut chunk_loader_query: Query<&ChunkLoader, (Changed<ChunkActor>, Without<Chunk>, With<ChunkActor>)>,
    mut chunk_transform_query: Query<(&mut Chunk, &mut Transform), Without<ChunkActor>>,
    mut chunk_actor_transform_query: Query<(&mut ChunkActor, &mut Transform), Without<Chunk>>,
) {
    let chunk_loader = match chunk_loader_query.single_mut() {
        Ok(data) => data,
        Err(_) => {
            return;
        }
    };

    for (mut chunk, mut transform) in chunk_transform_query.iter_mut() {
        let unit_pos = UnitVec::new(chunk_loader.origin_offset.clone(), transform.translation.truncate());

        if unit_pos.grid_offset != chunk_loader.origin_offset {
            let grid_diff = unit_pos.grid_offset.xy - chunk_loader.origin_offset.clone().xy;

            let mut unit_diff = Vec2::ZERO;

            if grid_diff.x.abs() >= ORIGIN_OFFSET_THRESHOLD as i32 {
                unit_diff.x = grid_diff.x as f32 * 1000.0;
            }
            if grid_diff.y.abs() >= ORIGIN_OFFSET_THRESHOLD as i32 {
                unit_diff.y = grid_diff.y as f32 * 1000.0;
            }

            transform.translation.x -= unit_diff.x;
            transform.translation.y -= unit_diff.y;
            chunk.coord -= grid_diff;
        }
    }
    for (mut chunk_actor, mut transform) in chunk_actor_transform_query.iter_mut() {
        let unit_pos = UnitVec::new(chunk_loader.origin_offset.clone(), transform.translation.truncate());

        if unit_pos.grid_offset != chunk_loader.origin_offset {
            let grid_diff = unit_pos.grid_offset.xy - chunk_loader.origin_offset.clone().xy;

            let mut unit_diff = Vec2::ZERO;

            if grid_diff.x.abs() >= ORIGIN_OFFSET_THRESHOLD as i32 {
                unit_diff.x = grid_diff.x as f32 * 1000.0;
            }
            if grid_diff.y.abs() >= ORIGIN_OFFSET_THRESHOLD as i32 {
                unit_diff.y = grid_diff.y as f32 * 1000.0;
            }

            transform.translation.x -= unit_diff.x;
            transform.translation.y -= unit_diff.y;
            chunk_actor.coord -= grid_diff;
        }
    }
}

#[tracing::instrument(skip_all)]
pub(crate) fn sync_logical_from_transform_system(
    chunk_loader_query: Query<&ChunkLoader>,
    mut query: Query<(&Transform, &mut ChunkActor)>,
) {
    let loader = match chunk_loader_query.single() {
        Ok(loader) => loader,
        Err(_) => return,
    };

    for (transform, mut actor) in &mut query {
        let new_coord = GridVec::from_native_logical(
            loader.origin_offset.clone(),
            (transform.translation.truncate(), actor.coord.scale)
        );
        actor.coord = new_coord;
    }
}
