use bevy::prelude::*;
use crate::chunk::components::*;
use crate::chunk::constants::*;
use crate::chunk::id::structs::ChunkID as ChunkID;
use crate::chunk::coordinate::structs::ChunkCoordinate as ChunkCoordinate;
use crate::chunk::actor::coordinate::structs::ChunkActorCoordinate as ChunkActorCoordinate;

pub(in crate) fn new_chunk_entity(commands: &mut Commands, chunk_id: ChunkID) -> Entity {
    let chunk_coordinate: ChunkCoordinate = chunk_id.into();
    let chunk_chunk_actor_coordinate: ChunkActorCoordinate = chunk_coordinate.into();

    let chunk_color = if (chunk_coordinate.0.0 + chunk_coordinate.0.1) % 2 == 0 {
        Color::rgb(0.25, 0.25, 0.25)
    } else {
        Color::rgb(0.75, 0.75, 0.75)
    };

    let chunk_entity = commands.spawn((
        Chunk { id: chunk_id, chunk_actors: Vec::new()},
        SpriteBundle {
            sprite: Sprite {
                color: chunk_color,
                custom_size: Some(Vec2::new(CHUNK_SIZE as f32, CHUNK_SIZE as f32)),
                ..default()
            },
            transform: Transform::from_translation(chunk_chunk_actor_coordinate.0),
            ..default()
        },
    )).id();

    chunk_entity
}
