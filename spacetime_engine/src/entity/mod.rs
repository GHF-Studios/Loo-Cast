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
            .add_event::<CreateEntity>()
            .add_event::<DestroyEntity>()
            .add_event::<LoadEntity>()
            .add_event::<UnloadEntity>()
            .add_plugins(id::IDPlugin)
            .add_systems(Update, handle_create_events)
            .add_systems(Update, handle_destroy_events)
            .add_systems(Update, handle_load_events)
            .add_systems(Update, handle_unload_events)
            .insert_resource(EntityRegistry::default());
    }
}