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
) {
    if let Some(_) = load_universe_event_reader.iter().last() {
        commands.insert_resource(UniverseManager {
            current_scale_level: 0,
            current_chunk_offset_x: 0,
            current_chunk_offset_y: 0,
        });

        commands.insert_resource(ChunkGenerator::new());
    }
}

pub fn universe_observer_system(
    mut commands: Commands,
    chunk_query: Query<&Chunk>,
    chunk_entity_query: Query<(Entity, &Chunk)>,
    mut query: Query<(&mut UniverseObserver, &Transform)>,
    game_manager: Res<GameManager>,
) {
    for (mut observer, transform) in query.iter_mut() {
        let x = transform.translation.x;
        let y = transform.translation.y;

        let proximal_chunk_coordinates = observer.get_proximal_chunk_coordinates(x, y);

        for coordinate in &proximal_chunk_coordinates {
            spawn_chunk(&mut commands, &chunk_query, *coordinate, &game_manager);
        }

        for old_coordinate in &observer.old_proximal_chunk_coordinates {
            if !proximal_chunk_coordinates.contains(old_coordinate) {
                despawn_chunk(
                    &mut commands,
                    &chunk_entity_query,
                    *old_coordinate,
                    &game_manager,
                );
            }
        }

        observer.old_proximal_chunk_coordinates = proximal_chunk_coordinates;
    }
}

pub fn is_chunk_generated(chunk_pos: LocalChunkPosition, game_manager: &Res<GameManager>) -> bool {
    let dir_path = format!(
        "assets/data/saves/{}/state/scale_0/parent_chunk_0_0/chunk_{}_{}",
        game_manager.current_save_game.name, chunk_pos.x, chunk_pos.y
    );
    Path::new(&dir_path).exists()
}

pub fn generate_chunk(
    commands: &mut Commands,
    chunk_pos: LocalChunkPosition,
    game_manager: &Res<GameManager>,
) {
    // Create the new chunk
    let chunk = Chunk {
        pos: chunk_pos,
        scale_level: 0,
        stored_entities: Vec::new(),
    };

    let serialized_chunk: String = serde_json::to_string(&chunk).unwrap();

    // Save the chunk
    let dir_path = format!(
        "assets/data/saves/{}/state/scale_0/parent_chunk_0_0/chunk_{}_{}",
        game_manager.current_save_game.name, chunk.pos.x, chunk.pos.y
    );

    std::fs::create_dir_all(&dir_path).expect("Failed to create chunk directory");

    let string_path = format!("{}/info.json", dir_path);
    let mut file = File::create(&string_path).unwrap();
    file.write_all(serialized_chunk.as_bytes()).unwrap();

    // Generate a noise image for this chunk with positional offset
    let noise_image = generate_noise_image(chunk_pos);
    let image_path = format!("{}/noise.png", dir_path);
    noise_image.save(image_path).unwrap();

    // Add to the world
    commands.spawn(chunk);
}

pub fn load_chunk(
    commands: &mut Commands,
    chunk_pos: LocalChunkPosition,
    game_manager: &Res<GameManager>,
) {
    let dir_path = format!(
        "assets/data/saves/{}/state/scale_0/parent_chunk_0_0/chunk_{}_{}",
        game_manager.current_save_game.name, chunk_pos.x, chunk_pos.y
    );

    let string_path = format!("{}/info.json", dir_path);
    let file = File::open(&string_path).unwrap();
    let chunk: Chunk = serde_json::from_reader(file).unwrap();

    commands.spawn(chunk);
}

pub fn unload_chunk(commands: &mut Commands, entity: Entity) {
    commands.entity(entity).despawn();
}

// Modified spawn_chunk and despawn_chunk
pub fn spawn_chunk(
    commands: &mut Commands,
    chunk_query: &Query<&Chunk>,
    chunk_pos: LocalChunkPosition,
    game_manager: &Res<GameManager>,
) {
    for chunk in chunk_query.iter() {
        if chunk.pos == chunk_pos {
            return; // Chunk already exists
        }
    }

    // Check if the chunk is generated, if not, generate it
    if !is_chunk_generated(chunk_pos, &game_manager) {
        generate_chunk(commands, chunk_pos, &game_manager);
    } else {
        load_chunk(commands, chunk_pos, &game_manager);
    }

    println!(
        "Spawned chunk with coordinates ({}, {})",
        chunk_pos.x, chunk_pos.y
    );
}

pub fn despawn_chunk(
    commands: &mut Commands,
    chunk_query: &Query<(Entity, &Chunk)>,
    chunk_pos: LocalChunkPosition,
    _game_manager: &Res<GameManager>,
) {
    for (entity, chunk) in chunk_query.iter() {
        if chunk.pos == chunk_pos {
            unload_chunk(commands, entity);
            println!(
                "Despawned chunk with coordinates ({}, {})",
                chunk_pos.x, chunk_pos.y
            );
            return;
        }
    }
    println!(
        "Chunk with coordinates ({}, {}) not found!",
        chunk_pos.x, chunk_pos.y
    );
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
