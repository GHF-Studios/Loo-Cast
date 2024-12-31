use bevy::prelude::*;

use super::constants::DEFAULT_CHUNK_LOADER_RADIUS;

#[derive(Component)]
pub struct ChunkLoaderComponent {
    pub radius: u32,
}
impl Default for ChunkLoaderComponent {
    fn default() -> Self {
        ChunkLoaderComponent {
            radius: DEFAULT_CHUNK_LOADER_RADIUS
        }
    }
}