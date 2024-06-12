use crate::chunk::actor::id::structs::ChunkActorID;
use crate::chunk::actor::resources::ChunkActorRegistry;
use crate::chunk::components::Chunk;
use crate::chunk::id::structs::ChunkID;
use crate::chunk::loader::components::ChunkLoader;
use crate::chunk::resources::ChunkRegistry;
use crate::entity::id::structs::EntityID;
use crate::entity::resources::*;
use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use super::components::Player;

pub(in crate) fn start(
) {
    david
}

pub(in crate) fn handle_start(
) {
    jackson
}

pub(in crate) fn change_player_chunk_load_radius(
    mut chunk_loader_query: Query<(&mut ChunkLoader, &Player)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for (mut chunk_loader, _) in chunk_loader_query.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::KeyQ) {
            *chunk_loader.load_radius_mut() = (chunk_loader.load_radius() as i16 - 1).max(0) as u16;
        }
        if keyboard_input.just_pressed(KeyCode::KeyE) {
            *chunk_loader.load_radius_mut() = (chunk_loader.load_radius() as i16 + 1) as u16;
        }
    }
}