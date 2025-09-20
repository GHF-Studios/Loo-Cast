use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ChunkActor;
impl Default for ChunkActor {
    fn default() -> Self {
        ChunkActor
    }
}
