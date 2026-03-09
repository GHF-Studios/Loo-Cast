pub mod bundles;
pub mod components;
pub mod systems;

use crate::bevy::prelude::*;
use bundles::PlayerBundle;
use components::{Player, PlayerVisual3dLink};
use systems::{ensure_player_visual_3d_system, update_player_system};

use crate::chunk::run_conditions::run_if_chunk_load_gate_open;
use crate::core::{orchestration::AppSet, run_conditions::run_after_startup_finished};
use crate::time::run_conditions::run_if_not_paused;

pub(crate) struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_player_system
                    .in_set(AppSet::Intent)
                    .run_if(run_after_startup_finished.and(run_if_not_paused).and(run_if_chunk_load_gate_open)),
                ensure_player_visual_3d_system
                    .in_set(AppSet::Presentation)
                    .run_if(run_after_startup_finished),
            ),
        )
        .register_type::<PlayerBundle>()
        .register_type::<Player>()
        .register_type::<PlayerVisual3dLink>();
    }
}
