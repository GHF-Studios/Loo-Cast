use bevy::prelude::*;

use crate::config::statics::CONFIG;

#[derive(Component)]
pub struct ChunkLoaderComponent {
    pub radius: u32,
}
impl Default for ChunkLoaderComponent {
    fn default() -> Self {
        ChunkLoaderComponent {
            radius: CONFIG.get::<u32>("chunk_loader/default_radius")
        }
    }
}