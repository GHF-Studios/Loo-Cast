use bevy::prelude::*;

#[derive(Component, Default)]
pub struct ChunkComponent {
    pub coord: (i32, i32),
    pub owner: Option<Entity>,
}