use super::*;
use super::components::*;
use super::events::*;
use super::resources::UniverseManager;

use bevy::prelude::*;

pub fn handle_load_universe(
    mut commands: Commands,
    mut load_universe_event_reader: EventReader<LoadUniverse>,
) {
    if let Some(_) = load_universe_event_reader.iter().last() {
        commands.insert_resource(UniverseManager {
            current_scale_level: 0,
            current_chunk_offset_x: 0,
            current_chunk_offset_y: 0,
        })
    }
}

pub fn universe_observer_system(
    mut commands: Commands,
    chunk_query: Query<&Chunk>,
    chunk_entity_query: Query<(Entity, &Chunk)>,
    mut query: Query<(&mut UniverseObserver, &Transform)>,
) {
    for (mut observer, transform) in query.iter_mut() {
        let x = transform.translation.x;
        let y = transform.translation.y;

        let proximal_chunk_coordinates = observer.get_proximal_chunk_coordinates(x, y);

        for coordinate in &proximal_chunk_coordinates {
            spawn_chunk(&mut commands, &chunk_query, coordinate.x, coordinate.y);
        }

        for old_coordinate in &observer.old_proximal_chunk_coordinates {
            if !proximal_chunk_coordinates.contains(old_coordinate) {
                
                despawn_chunk(&mut commands, &chunk_entity_query, old_coordinate.x, old_coordinate.y);
            }
        }

        observer.old_proximal_chunk_coordinates = proximal_chunk_coordinates;
    }
}

pub fn spawn_chunk(
    commands: &mut Commands,
    chunk_query: &Query<&Chunk>,
    chunk_pos_x: i16,
    chunk_pos_y: i16,
) {
    // Check if the chunk already exists
    for chunk in chunk_query.iter() {
        if chunk.chunk_x == chunk_pos_x && chunk.chunk_y == chunk_pos_y {
            return;
        }
    }

    commands.spawn((
        Chunk {
            chunk_x: chunk_pos_x,
            chunk_y: chunk_pos_y,
            scale_level: 0,
            stored_entities: Vec::new(),
        },
    ));
    //println!("Spawned chunk with coordinates ({}, {})", chunk_pos_x, chunk_pos_y);
}

pub fn despawn_chunk(
    commands: &mut Commands,
    chunk_query: &Query<(Entity, &Chunk)>,
    chunk_pos_x: i16,
    chunk_pos_y: i16,
) {
    for (entity, chunk) in chunk_query.iter() {
        if chunk.chunk_x == chunk_pos_x && chunk.chunk_y == chunk_pos_y {
            commands.entity(entity).despawn();
            //println!("Despawned chunk with coordinates ({}, {})", chunk_pos_x, chunk_pos_y);
            return;
        }
    }
    //println!("Chunk with coordinates ({}, {}) not found!", chunk_pos_x, chunk_pos_y);
}

pub fn debug_chunks_update(
    chunk_query: Query<&Chunk>,
    mut gizmos: Gizmos
) {
    for chunk in chunk_query.iter() {
        let chunk_pos_x = ((chunk.chunk_x as f32) * (CHUNK_SIZE as f32)) + (CHUNK_SIZE as f32 / 2.0);
        let chunk_pos_y = ((chunk.chunk_y as f32) * (CHUNK_SIZE as f32)) + (CHUNK_SIZE as f32 / 2.0);
        gizmos.rect_2d(
            Vec2::new(chunk_pos_x, chunk_pos_y),
            0.0,
            Vec2::splat(CHUNK_SIZE.into()),
            Color::RED,
        );
    }
}