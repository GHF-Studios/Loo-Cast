use super::components::Chunk;
use bevy::prelude::*;

#[derive(Bundle, Default)]
pub struct ChunkBundle {
    pub chunk: Chunk,
    pub sprite_bundle: SpriteBundle,
}
