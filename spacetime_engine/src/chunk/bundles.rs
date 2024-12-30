use bevy::prelude::*;
use super::components::ChunkComponent;

#[derive(Bundle)]
pub struct ChunkBundle {
    pub chunk: ChunkComponent,
    pub sprite_bundle: SpriteBundle,
}
impl Default for ChunkBundle {
    fn default() -> Self {
        Self {
            chunk: Default::default(),
            sprite_bundle: Default::default(),
        }
    }
}