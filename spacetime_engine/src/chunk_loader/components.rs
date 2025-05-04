use std::sync::Mutex;

use crate::config::statics::CONFIG;
use bevy::{ecs::component::StorageType, prelude::*};
use lazy_static::lazy_static;

lazy_static! {
    static ref ID_COUNTER: Mutex<u32> = Mutex::new(0);
}

#[derive(Component)]
pub struct ChunkLoaderComponent {
    pub radius: u32,
    pub id: u32,
}
impl Default for ChunkLoaderComponent {
    fn default() -> Self {
        let mut id_counter = ID_COUNTER.lock().unwrap();
        let id = *id_counter;
        *id_counter += 1;

        ChunkLoaderComponent {
            radius: CONFIG.get::<u32>("chunk_loader/default_radius"),
            id,
        }
    }
}
