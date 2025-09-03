pub mod components;

use bevy::prelude::*;

use components::ChunkActor;

pub(crate) struct ChunkActorPlugin;
impl Plugin for ChunkActorPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ChunkActor>();
    }
}
