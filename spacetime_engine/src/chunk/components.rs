use bevy::prelude::*;

#[derive(Component)]
pub struct ChunkComponent {
    pub coordinates: (i32, i32),
    pub owner: Option<Entity>,
}
impl Default for ChunkComponent {
    fn default() -> Self {
        ChunkComponent {
            coordinates: Default::default(),
            owner: None
        }
    }
}