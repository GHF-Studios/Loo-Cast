use bevy::log::*;
use crate::chunk::structs::ChunkPosition;
use crate::entity::structs::EntityPosition;
use crate::chunk::commands::*;
use crate::chunk_actor::commands::*;
use crate::math::structs::I16Vec2;

pub async fn startup() {
    spawn_start_chunks(2).await;
    spawn_start_chunk_actors(2).await;
}

async fn spawn_start_chunks(range: i16) {
    for x in -range..=range {
        for y in -range..=range {
            let chunk_position = ChunkPosition(I16Vec2(x, y));

            if let Err(e) = spawn_chunk(chunk_position).await {
                error!("Error spawning chunk: {:?}", e);
            } else {
                debug!("Spawned chunk at {:?}", chunk_position);
            }
        }
    }
}

async fn spawn_start_chunk_actors(range: i16) {
    for x in -range..=range {
        for y in -range..=range {
            let chunk_position = ChunkPosition(I16Vec2(x, y));
            let entity_position: EntityPosition = chunk_position.into();

            if let Err(e) = spawn_chunk_actor(entity_position).await {
                error!("Error spawning chunk actor: {:?}", e);
            } else {
                debug!("Spawned chunk actor at {:?}", entity_position);
            }
        }
    }
}