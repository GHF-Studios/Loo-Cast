use crate::{config::statics::CONFIG, entity::functions::reserve_entity_id};
use bevy::prelude::*;

#[derive(Component)]
pub struct ChunkLoaderComponent {
    pub radius: u32,
    pub entity: Entity,
}
impl Default for ChunkLoaderComponent {
    fn default() -> Self {
        ChunkLoaderComponent {
            radius: CONFIG.get::<u32>("chunk_loader/default_radius"),
            entity: reserve_entity_id(),
        }
    }
}
