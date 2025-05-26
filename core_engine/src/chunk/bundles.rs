use super::components::ChunkComponent;
use bevy::prelude::*;

#[derive(Bundle, Default)]
pub struct ChunkBundle {
    pub chunk: ChunkComponent,
    pub sprite_bundle: SpriteBundle,
}
