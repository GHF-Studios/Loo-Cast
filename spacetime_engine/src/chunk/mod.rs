pub mod bundles;
pub mod components;
pub mod constants;
pub mod events;
pub mod functions;
pub mod hooks;
pub mod statics;

use bevy::prelude::*;

pub(in crate) struct ChunkPlugin;
impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
    }
}