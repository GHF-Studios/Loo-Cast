use crate::bevy::prelude::*;

use crate::chunk::components::{Chunk, ChunkActor, ChunkLoader};
use crate::config::statics::CONFIG;
use crate::usf::pos::grid::types::GridVec;
use crate::usf::pos::unit::types::UnitVec;

// TODO: Fix
#[tracing::instrument(skip_all)]
pub(crate) fn update_managed_positions(
    mut chunk_loader: Single<(&mut Transform, &mut ChunkLoader, &mut ChunkActor)>,
    mut chunk_query: Query<(&mut Transform, &Chunk), (Without<ChunkLoader>, Without<ChunkActor>)>,
    mut chunk_actor_query: Query<(&mut Transform, &mut ChunkActor), Without<ChunkLoader>>,
) {
    const NATIVE_LOGICAL_LAYER_SIZE_MIN: f32 = -5500.0;
    const NATIVE_LOGICAL_LAYER_SIZE_MAX: f32 = 4500.0;
    const NATIVE_LOGICAL_LAYER_WRAPPING_SIZE: f32 = 10000.0;

    let (ref mut loader_transform, ref mut chunk_loader, ref mut chunk_actor) = *chunk_loader;

    let grid_origin = GridVec::from_native_logical(
        GridVec::default_n((chunk_loader.scale.index_from_top() + 1) as usize),
        (loader_transform.translation, chunk_loader.scale),
    );

    // DEPRECATED (I think?)
    // let mut native_logical_origin_offset = UnitVec::native_logical_offset(
    //     &UnitVec::default(),
    //     &UnitVec::new_grid(grid_origin.clone()),
    // ).unwrap();
    // DEPRECATED (I think?)

    if loader_transform.translation.x >= NATIVE_LOGICAL_LAYER_SIZE_MAX {
        loader_transform.translation.x -= NATIVE_LOGICAL_LAYER_WRAPPING_SIZE;
    } else if loader_transform.translation.x < NATIVE_LOGICAL_LAYER_SIZE_MIN {
        loader_transform.translation.x += NATIVE_LOGICAL_LAYER_WRAPPING_SIZE;
    }

    if loader_transform.translation.y >= NATIVE_LOGICAL_LAYER_SIZE_MAX {
        loader_transform.translation.y -= NATIVE_LOGICAL_LAYER_WRAPPING_SIZE;
    } else if loader_transform.translation.y < NATIVE_LOGICAL_LAYER_SIZE_MIN {
        loader_transform.translation.y += NATIVE_LOGICAL_LAYER_WRAPPING_SIZE;
    }

    chunk_loader.coord = grid_origin.clone();
    chunk_actor.coord = grid_origin;

    for (transform, mut actor) in chunk_actor_query.iter_mut() {
        let new_coord = GridVec::from_native_logical(GridVec::default(), (transform.translation, actor.coord.scale));
        actor.coord = new_coord;
    }
}

// Shiny new not-shitass systems ABOVE
// Old depricated shitass systems BELOW
#[tracing::instrument(skip_all)]
pub(crate) fn realign_origin_offset_system(mut chunk_loader: Single<(&mut Transform, &mut ChunkLoader), Changed<Transform>>) {
    // Re-align origin_offset
    let (ref mut transform, ref mut chunk_loader) = *chunk_loader;
    let unit_pos = crate::usf::pos::unit::types::UnitVec::new(chunk_loader.origin_offset.clone(), transform.translation); // `UnitVec::new` internally normalizes the position based on the current origin_offset; does the heavy lifting for us
    let grid_diff = IVec3::new(
        unit_pos.grid_offset.xyz.x - chunk_loader.origin_offset.xyz.x,
        unit_pos.grid_offset.xyz.y - chunk_loader.origin_offset.xyz.y,
        unit_pos.grid_offset.xyz.z - chunk_loader.origin_offset.xyz.z,
    );
    let threshold = CONFIG().get::<u8>("usf/pos/origin_offset_threshold") as i32;

    if grid_diff.x.abs() >= threshold || grid_diff.y.abs() >= threshold || grid_diff.z.abs() >= threshold {
        chunk_loader.origin_offset = unit_pos.grid_offset;
    }
    transform.translation = unit_pos.unit_offset;
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
        let unit_pos = UnitVec::new(chunk_loader.origin_offset.clone(), transform.translation);

        if unit_pos.grid_offset != chunk_loader.origin_offset {
            let grid_diff = IVec3::new(
                unit_pos.grid_offset.xyz.x - chunk_loader.origin_offset.xyz.x,
                unit_pos.grid_offset.xyz.y - chunk_loader.origin_offset.xyz.y,
                unit_pos.grid_offset.xyz.z - chunk_loader.origin_offset.xyz.z,
            );

            let mut unit_diff = Vec3::ZERO;

            if grid_diff.x.abs() >= origin_offset_threshold as i32 {
                unit_diff.x = grid_diff.x as f32 * 1000.0;
            }
            if grid_diff.y.abs() >= origin_offset_threshold as i32 {
                unit_diff.y = grid_diff.y as f32 * 1000.0;
            }
            if grid_diff.z.abs() >= origin_offset_threshold as i32 {
                unit_diff.z = grid_diff.z as f32 * 1000.0;
            }

            transform.translation.x -= unit_diff.x;
            transform.translation.y -= unit_diff.y;
            transform.translation.z -= unit_diff.z;
            chunk.coord -= grid_diff;
        }
    }
    for (mut chunk_actor, mut transform) in chunk_actor_transform_query.iter_mut().chain(chunk_loader_transform_query.iter_mut()) {
        let unit_pos = UnitVec::new(chunk_loader.origin_offset.clone(), transform.translation);

        if unit_pos.grid_offset != chunk_loader.origin_offset {
            let grid_diff = IVec3::new(
                unit_pos.grid_offset.xyz.x - chunk_loader.origin_offset.xyz.x,
                unit_pos.grid_offset.xyz.y - chunk_loader.origin_offset.xyz.y,
                unit_pos.grid_offset.xyz.z - chunk_loader.origin_offset.xyz.z,
            );

            let mut unit_diff = Vec3::ZERO;

            if grid_diff.x.abs() >= origin_offset_threshold as i32 {
                unit_diff.x = grid_diff.x as f32 * 1000.0;
            }
            if grid_diff.y.abs() >= origin_offset_threshold as i32 {
                unit_diff.y = grid_diff.y as f32 * 1000.0;
            }
            if grid_diff.z.abs() >= origin_offset_threshold as i32 {
                unit_diff.z = grid_diff.z as f32 * 1000.0;
            }

            transform.translation.x -= unit_diff.x;
            transform.translation.y -= unit_diff.y;
            transform.translation.z -= unit_diff.z;
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
        let new_coord = GridVec::from_native_logical(loader.origin_offset.clone(), (transform.translation, actor.coord.scale));
        actor.coord = new_coord;
    }
}
