pub mod creative;
pub mod movement;
pub mod teleportation;

pub mod components;
pub mod constants;
pub mod events;
pub mod functions;
pub(in crate) mod systems;

use creative::*;
use movement::*;
use teleportation::*;
use systems::*;
use bevy::prelude::*;

pub(in crate) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<events::Startup>()
            .add_plugins(CreativePlugin)
            .add_plugins(MovementPlugin)
            .add_plugins(TeleportationPlugin)
            .add_systems(Startup, startup)
            .register_type::<components::Player>();
    }
}