use bevy::prelude::*;

use crate::chunk::components::Chunk;
use crate::chunk_actor::components::ChunkActor;
use crate::chunk_loader::components::ChunkLoader;
use crate::usf::pos::unit::types::UnitVec;

use super::constants::ORIGIN_OFFSET_THRESHOLD;

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