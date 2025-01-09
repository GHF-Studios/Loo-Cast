use bevy::prelude::*;

use crate::chunk_loader::components::ChunkLoaderComponent;

use super::{components::ChunkComponent, enums::ChunkAction, functions::{chunk_pos_to_world, world_pos_to_chunk}, ChunkActionBuffer, ChunkManager};

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

pub(in crate) fn process_chunk_actions(
    mut commands: Commands,
    chunk_loader_query: Query<(Entity, &Transform, &ChunkLoaderComponent)>,
    chunk_query: Query<(Entity, &ChunkComponent)>,
    chunk_manager: Res<ChunkManager>,
    mut chunk_action_buffer: ResMut<ChunkActionBuffer>,
) {
    for (chunk_coord, chunk_action) in chunk_action_buffer.0.iter() {
        match chunk_action {
            ChunkAction::Spawn { coord: chunk_coord } => {

            },
            ChunkAction::Despawn { coord } => {

            },
            ChunkAction::TransferOwnership { coord, new_owner } => {

            }
        }
    }
}