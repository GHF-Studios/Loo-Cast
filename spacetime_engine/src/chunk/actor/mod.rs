pub mod coordinate;
pub mod id;

pub mod components;
pub mod constants;
pub mod functions;
pub mod resources;
pub mod structs;
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