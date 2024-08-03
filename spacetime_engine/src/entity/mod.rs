pub mod id;

pub mod components;
pub mod events;
pub mod functions;
pub mod resources;
pub mod types;
pub mod structs;
pub mod systems;

use events::*;
use resources::*;
use bevy::prelude::*;

pub(in crate) struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<CreateEntity>()
            .add_event::<DestroyEntity>()
            .add_event::<CreatedEntity>()
            .add_event::<DestroyedEntity>()
            .add_plugins(id::IDPlugin)
            .insert_resource(EntityRegistry::default())
            .add_systems(Startup, functions::main::setup)
            .add_systems(Update, (
                systems::internal_handlers::handle_create_entity, 
                systems::internal_handlers::handle_destroy_entity
            ));
    }
}