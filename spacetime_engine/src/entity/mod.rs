pub mod id;

pub mod components;
pub mod events;
pub mod functions;
pub mod resources;
pub mod types;
pub mod structs;
pub mod systems;

use resources::*;
use bevy::prelude::*;

pub(in crate) struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(id::IDPlugin)
            .insert_resource(EntityRegistry::default());
    }
}