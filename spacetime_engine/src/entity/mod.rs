pub mod id;

pub mod resources;

use resources::*;
use bevy::prelude::*;

pub(in crate) struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(id::IDPlugin)
            .insert_resource(EntityManager::default());
    }
}