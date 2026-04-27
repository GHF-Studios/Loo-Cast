pub mod bundles;
pub mod components;
pub mod resources;
pub mod systems;

use crate::bevy::prelude::*;
use crate::bevy_rapier3d::prelude::PhysicsSet;
use bundles::PlayerBundle;
use components::{Player, PlayerSpawnRecovery, PlayerVisual3dLink};
use resources::{PlayerCameraMode, PlayerCameraRigSettings, PlayerControlSettings, PlayerLookState};
use systems::{
    apply_player_camera_mode_system, apply_player_camera_orientation_system, apply_player_visual_orientation_system, ensure_player_physics_controller_system,
    ensure_player_visual_3d_system, ensure_single_player_exists_system, resolve_player_spawn_overlap_system, sync_mouse_capture_system,
    sync_pause_menu_state_system, toggle_pause_menu_system, toggle_player_camera_mode_system, update_player_system,
};

use crate::chunk::run_conditions::run_if_chunk_load_gate_open;
use crate::core::{orchestration::AppSet, run_conditions::run_after_startup_finished};
use crate::follower::systems::update_follower_system;
use crate::time::run_conditions::run_if_not_paused;

pub(crate) struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                toggle_player_camera_mode_system.in_set(AppSet::InputGather).run_if(run_after_startup_finished),
                toggle_pause_menu_system.in_set(AppSet::InputGather).run_if(run_after_startup_finished),
                sync_pause_menu_state_system.in_set(AppSet::Intent).run_if(run_after_startup_finished),
                update_player_system
                    .in_set(AppSet::Intent)
                    .after(sync_pause_menu_state_system)
                    .run_if(run_after_startup_finished.and(run_if_not_paused).and(run_if_chunk_load_gate_open)),
                ensure_player_visual_3d_system.in_set(AppSet::Presentation).run_if(run_after_startup_finished),
                ensure_player_physics_controller_system
                    .in_set(AppSet::Presentation)
                    .run_if(run_after_startup_finished),
                sync_mouse_capture_system.in_set(AppSet::Presentation).run_if(run_after_startup_finished),
            ),
        )
        .add_systems(PostStartup, ensure_single_player_exists_system)
        .add_systems(
            PostUpdate,
            (
                apply_player_camera_mode_system
                    .in_set(AppSet::Camera)
                    .before(update_follower_system)
                    .run_if(run_after_startup_finished),
                resolve_player_spawn_overlap_system
                    .in_set(AppSet::Simulation)
                    .after(PhysicsSet::Writeback)
                    .run_if(run_after_startup_finished),
                apply_player_camera_orientation_system
                    .in_set(AppSet::Presentation)
                    .run_if(run_after_startup_finished),
                apply_player_visual_orientation_system
                    .in_set(AppSet::Presentation)
                    .run_if(run_after_startup_finished),
            ),
        )
        .init_resource::<PlayerCameraMode>()
        .init_resource::<PlayerControlSettings>()
        .init_resource::<PlayerLookState>()
        .init_resource::<PlayerCameraRigSettings>()
        .register_type::<PlayerBundle>()
        .register_type::<Player>()
        .register_type::<PlayerSpawnRecovery>()
        .register_type::<PlayerVisual3dLink>()
        .register_type::<PlayerCameraMode>()
        .register_type::<PlayerControlSettings>()
        .register_type::<PlayerLookState>()
        .register_type::<PlayerCameraRigSettings>();
    }
}
