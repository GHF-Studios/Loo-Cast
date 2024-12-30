use bevy::prelude::*;

#[derive(Component)]
pub struct ChunkLoaderComponent {
    pub radius: u32,
}
impl Default for ChunkLoaderComponent {
    fn default() -> Self {
        ChunkLoaderComponent {
            radius: 3
        }
    }
}