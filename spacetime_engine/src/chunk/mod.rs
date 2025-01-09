pub mod bundles;
pub mod components;
pub mod constants;
pub mod enums;
pub mod errors;
pub mod functions;
pub mod hooks;
pub mod resources;
pub mod statics;
pub mod systems;

use bevy::prelude::*;
use resources::{ChunkActionBuffer, ChunkManager};
use systems::update_chunk_system;

pub(in crate) struct ChunkPlugin;
impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ChunkActionBuffer::default())
            .insert_resource(ChunkManager::default())
            .add_systems(Update, update_chunk_system);
    }
}