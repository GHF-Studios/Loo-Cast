pub mod coordinate;
pub(in crate) mod constants;
pub mod id;

pub mod components;
pub mod functions;
pub mod resources;
pub(in crate) mod systems;

use id::*;
use resources::*;
use systems::*;
use bevy::prelude::*;

pub(in crate) struct ActorPlugin;

impl Plugin for ActorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(IDPlugin)
            .insert_resource(ChunkActorRegistry::default())
            .add_systems(Update, update)
            .register_type::<components::ChunkActor>();
    }
}