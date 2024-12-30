use bevy::prelude::*;

use super::{components::ChunkComponent, functions::{chunk_pos_to_world, world_pos_to_chunk}};

pub(in crate) fn update_chunk_system(
    chunk_query: Query<(Entity, &Transform, &ChunkComponent)>,
) {
    for (_, transform, chunk) in chunk_query.iter() {
        let world_pos = transform.translation.truncate();
        let chunk_pos = world_pos_to_chunk(world_pos);

        if chunk.coord != chunk_pos {
            panic!("Attempted to move chunk entity");
        }

        if chunk_pos_to_world(chunk.coord) != world_pos {
            panic!("Attempted to move chunk entity");
        }
    }
}