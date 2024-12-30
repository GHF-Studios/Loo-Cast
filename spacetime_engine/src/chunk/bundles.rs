use bevy::prelude::*;
use super::components::ChunkComponent;

#[derive(Bundle, Default)]
pub struct ChunkBundle {
    pub chunk: ChunkComponent,
    pub sprite_bundle: SpriteBundle,
}