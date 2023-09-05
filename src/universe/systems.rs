use super::chunk::*;
use super::components::*;
use super::events::*;
use super::resources::*;
use super::*;

use crate::game::resources::*;
use crate::noise::*;

use bevy::prelude::*;
use std::fs::*;
use std::io::Write;
use std::path::Path;

pub fn handle_load_universe(
    mut commands: Commands,
    mut load_universe_event_reader: EventReader<LoadUniverse>,
    game_manager: Res<GameManager>,
) {
    if let Some(_) = load_universe_event_reader.iter().last() {
        commands.insert_resource(UniverseManager {
            current_scale_level: 0,
            current_chunk_offset_x: 0,
            current_chunk_offset_y: 0,
        });

        commands.insert_resource(ChunkManager::new(format!(
            "assets/data/saves/{}/state/scale_0/parent_chunk_0_0",
            game_manager.current_save_game.name
        )));
    }
}

pub fn universe_observer_system(
    mut commands: Commands,
    chunk_query: Query<&Chunk>,
    chunk_entity_query: Query<(Entity, &Chunk)>,
    mut query: Query<(&mut UniverseObserver, &Transform)>,
    chunk_manager: Res<ChunkManager>,
) {
    for (mut observer, transform) in query.iter_mut() {
        let x = transform.translation.x;
        let y = transform.translation.y;

        let proximal_chunk_coordinates = observer.get_proximal_chunk_coordinates(x, y);

        for coordinate in &proximal_chunk_coordinates {
            // Chunk should be spawned
            if let Some(chunk_state) = chunk_manager.get_chunk_state(*coordinate)
            {
                match chunk_state {
                    QueuedForGeneration => {},
                    InProgressForGeneration => {},
                    Generated => {},
                    QueuedForLoading => {},
                    InProgressForLoading => {},
                    Loaded => {},
                    QueuedForSpawning => {},
                    InProgressForSpawning => {},
                    Spawned => {},
                    QueuedForDespawning => {},
                    InProgressForDespawning => {},
                    Despawned => {},
                    QueuedForUnloading => {},
                    InProgressForUnloading => {},
                    Unloaded => {},
                }
            }
        }

        for old_coordinate in &observer.old_proximal_chunk_coordinates {
            if !proximal_chunk_coordinates.contains(old_coordinate) {
                // Chunk should not be spawned
                if let Some(chunk_state) = chunk_manager.get_chunk_state(*old_coordinate)
                {
                    match chunk_state {
                        QueuedForGeneration => {},
                        InProgressForGeneration => {},
                        Generated => {},
                        QueuedForLoading => {},
                        InProgressForLoading => {},
                        Loaded => {},
                        QueuedForSpawning => {},
                        InProgressForSpawning => {},
                        Spawned => {},
                        QueuedForDespawning => {},
                        InProgressForDespawning => {},
                        Despawned => {},
                        QueuedForUnloading => {},
                        InProgressForUnloading => {},
                        Unloaded => {},
                    }
                }
            }
        }

        observer.old_proximal_chunk_coordinates = proximal_chunk_coordinates;
    }
}

pub fn debug_chunks_update(chunk_query: Query<&Chunk>, mut gizmos: Gizmos) {
    for chunk in chunk_query.iter() {
        let chunk_pos_x = ((chunk.pos.x as f32) * (CHUNK_SIZE as f32)) + (CHUNK_SIZE as f32 / 2.0);
        let chunk_pos_y = ((chunk.pos.y as f32) * (CHUNK_SIZE as f32)) + (CHUNK_SIZE as f32 / 2.0);
        gizmos.rect_2d(
            Vec2::new(chunk_pos_x, chunk_pos_y),
            0.0,
            Vec2::splat(CHUNK_SIZE.into()),
            Color::RED,
        );
    }
}
