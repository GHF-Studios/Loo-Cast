pub mod bundles;
pub mod components;
pub mod resources;
pub mod systems;
pub mod types;

pub mod workflows;

use bevy::prelude::*;
use bundles::PlayerBundle;
use components::Player;
use resources::{PlayerLifecycle, PlayerWorkflowQueue};
use systems::update_player_system;
use types::PlayerWorkflow;

pub(crate) struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerLifecycle::default())
            .insert_resource(PlayerWorkflowQueue::default())
            .add_systems(Update, update_player_system)
            .register_type::<PlayerBundle>()
            .register_type::<Player>()
            .register_type::<PlayerLifecycle>()
            .register_type::<PlayerWorkflowQueue>()
            .register_type::<PlayerWorkflow>();
    }
}
