pub mod creative;
pub mod movement;

pub mod components;
pub(crate) mod constants;
pub(crate) mod systems;

use creative::CreativePlugin;
use movement::MovementPlugin;
use bevy::prelude::*;

pub(in crate) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(CreativePlugin)
            .add_plugins(MovementPlugin);
    
    }
}