pub mod bundles;
pub mod components;
pub mod constants;
pub mod functions;
pub mod hooks;
pub mod statics;
pub mod systems;

use bevy::prelude::*;
use systems::update_chunk_system;

pub(in crate) struct ChunkPlugin;
impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update_chunk_system);
    }
}