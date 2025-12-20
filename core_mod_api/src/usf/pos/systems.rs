use bevy::prelude::*;

use crate::chunk::components::{Chunk, ChunkActor, ChunkLoader};
use crate::config::statics::CONFIG;
use crate::usf::pos::grid::types::GridVec;
use crate::usf::pos::unit::types::UnitVec;

#[tracing::instrument(skip_all)]
pub(crate) fn update_managed_positions(
    mut chunk_loader: Single<(&Transform, &mut ChunkLoader, &mut ChunkActor)>,
    mut chunk_actor_query: Query<(&Transform, &mut ChunkActor), (Changed<Transform>, Without<ChunkLoader>)>,
) {
    let (loader_transform, ref mut chunk_loader, ref mut chunk_actor) = *chunk_loader;

    let native_logical_origin = GridVec::from_native_logical(
        GridVec::default(),
        (loader_transform.translation.truncate(), chunk_loader.scale),
    );
    
    chunk_loader.coord = native_logical_origin.clone();
    chunk_actor.coord = native_logical_origin;

    for (transform, mut actor) in chunk_actor_query.iter_mut() {
        let new_coord = GridVec::from_native_logical(
            GridVec::default(),
            (transform.translation.truncate(), actor.coord.scale)
        );
        actor.coord = new_coord;
    }
}

// Shiny new not-shitass systems ABOVE
// Old depricated shitass systems BELOW
#[tracing::instrument(skip_all)]
pub(crate) fn realign_origin_offset_system(
    mut chunk_loader: Single<(&mut Transform, &mut ChunkLoader), Changed<Transform>>,
) {
    // Re-align origin_offset
    let (ref mut transform, ref mut chunk_loader) = *chunk_loader;
    let unit_pos = crate::usf::pos::unit::types::UnitVec::new(chunk_loader.origin_offset.clone(), transform.translation.truncate()); // `UnitVec::new` internally normalizes the position based on the current origin_offset; does the heavy lifting for us
    /*
        The unitpos we now have, is constructed from the current origin offset, and the updated unit_offset,
        but the resulting unit_pos is wrapped immediately, to a canonical representation, automatically,
        but we want the grid_offset of this to be our new origin_offset, if, and only if the distance between unit_pos's grid_offset and the origin_offset is greater than ore equal to the origin_offset_threshold
    */
    let grid_diff = unit_pos.grid_offset.xy - chunk_loader.origin_offset.xy;
    let threshold = CONFIG().get::<u8>("usf/pos/origin_offset_threshold") as i32;
    
    if grid_diff.x.abs() >= threshold || grid_diff.y.abs() >= threshold {
        chunk_loader.origin_offset = unit_pos.grid_offset;
    }
    transform.translation = unit_pos.unit_offset;
    transform.translation.z = CONFIG().get::<f32>("player/z");
}

#[tracing::instrument(skip_all)]
pub(crate) fn apply_new_origin_offset_system(
    mut chunk_loader_query: Query<&ChunkLoader, (Changed<ChunkLoader>, Without<Chunk>, With<ChunkActor>)>,
    mut chunk_transform_query: Query<(&mut Chunk, &mut Transform), (Without<ChunkActor>, Without<ChunkLoader>)>,
    mut chunk_actor_transform_query: Query<(&mut ChunkActor, &mut Transform), (Without<Chunk>, Without<ChunkLoader>)>,
    mut chunk_loader_transform_query: Query<(&mut ChunkActor, &mut Transform), (Without<Chunk>, With<ChunkLoader>)>,
) {
    let chunk_loader = match chunk_loader_query.single_mut() {
        Ok(data) => data,
        Err(_) => {
            return;
        }
    };
    let origin_offset_threshold = CONFIG().get::<u8>("usf/pos/origin_offset_threshold");

    for (mut chunk, mut transform) in chunk_transform_query.iter_mut() {
        let unit_pos = UnitVec::new(chunk_loader.origin_offset.clone(), transform.translation.truncate());

        if unit_pos.grid_offset != chunk_loader.origin_offset {
            let grid_diff = unit_pos.grid_offset.xy - chunk_loader.origin_offset.clone().xy;

            let mut unit_diff = Vec2::ZERO;

            if grid_diff.x.abs() >= origin_offset_threshold as i32 {
                unit_diff.x = grid_diff.x as f32 * 1000.0;
            }
            if grid_diff.y.abs() >= origin_offset_threshold as i32 {
                unit_diff.y = grid_diff.y as f32 * 1000.0;
            }

            transform.translation.x -= unit_diff.x;
            transform.translation.y -= unit_diff.y;
            chunk.coord -= grid_diff;
        }
    }
    for (mut chunk_actor, mut transform) in chunk_actor_transform_query.iter_mut().chain(chunk_loader_transform_query.iter_mut()) {
        let unit_pos = UnitVec::new(chunk_loader.origin_offset.clone(), transform.translation.truncate());

        if unit_pos.grid_offset != chunk_loader.origin_offset {
            let grid_diff = unit_pos.grid_offset.xy - chunk_loader.origin_offset.clone().xy;

            let mut unit_diff = Vec2::ZERO;

            if grid_diff.x.abs() >= origin_offset_threshold as i32 {
                unit_diff.x = grid_diff.x as f32 * 1000.0;
            }
            if grid_diff.y.abs() >= origin_offset_threshold as i32 {
                unit_diff.y = grid_diff.y as f32 * 1000.0;
            }

            transform.translation.x -= unit_diff.x;
            transform.translation.y -= unit_diff.y;
            chunk_actor.coord -= grid_diff;
        }
    }
}

#[tracing::instrument(skip_all)]
pub(crate) fn sync_logical_from_transform_system(chunk_loader_query: Query<&ChunkLoader>, mut query: Query<(&Transform, &mut ChunkActor)>) {
    let loader = match chunk_loader_query.single() {
        Ok(loader) => loader,
        Err(_) => return,
    };

    for (transform, mut actor) in &mut query {
        let new_coord = GridVec::from_native_logical(loader.origin_offset.clone(), (transform.translation.truncate(), actor.coord.scale));
        actor.coord = new_coord;
    }
}
