pub mod bundles;
pub mod components;
pub mod resources;
pub mod systems;

pub mod workflows;

pub mod workflows_MACROINPUT;
pub mod workflows_MACROOUTPUT;

use bevy::prelude::*;
use resources::PlayerWorkflowQueue;
use systems::{process_player_workflow_queue, update_player_system};

pub(crate) struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerWorkflowQueue::default())
            .add_systems(Update, update_player_system)
            .add_systems(
                Update,
                process_player_workflow_queue.after(update_player_system),
            );
    }
}
