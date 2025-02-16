pub mod bundles;
pub mod components;
pub mod resources;
pub mod systems;

use bevy::prelude::*;
use resources::PlayerActionQueue;
use systems::{process_player_action_queue, update_player_system};

pub(in crate) struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(PlayerActionQueue::default())
            .add_systems(Update, update_player_system)
            .add_systems(Update, process_player_action_queue.after(update_player_system));
    }
}