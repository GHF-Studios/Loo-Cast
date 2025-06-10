pub mod bundles;
pub mod components;
pub mod resources;
pub mod systems;
pub mod types;

pub mod workflows;

use crate::player::resources::PlayerWorkflowQueue;
use bevy::prelude::*;
use systems::update_player_system;
use types::PlayerLifecycle;

pub(crate) struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerLifecycle::default())
            .insert_resource(PlayerWorkflowQueue::default())
            .add_systems(Update, update_player_system);
    }
}
