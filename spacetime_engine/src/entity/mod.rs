pub mod id;

pub mod events;
pub mod resources;
pub mod systems;

use events::*;
use resources::*;
use systems::*;
use bevy::prelude::*;

pub(in crate) struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<RegisterEntity>()
            .add_event::<UnregisterEntity>()
            .add_plugins(id::IDPlugin)
            .add_systems(Update, handle_register_events)
            .add_systems(Update, handle_unregister_events)
            .insert_resource(EntityRegistry::default());
    }
}