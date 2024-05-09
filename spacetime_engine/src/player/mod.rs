pub mod creative;
pub mod movement;

pub mod components;
pub mod events;
pub mod systems;

use creative::*;
use movement::*;
use systems::*;
use bevy::prelude::*;

pub(in crate) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<events::Startup>()
            .add_plugins(CreativePlugin)
            .add_plugins(MovementPlugin)
            .add_systems(Startup, startup)
        ;
    }
}