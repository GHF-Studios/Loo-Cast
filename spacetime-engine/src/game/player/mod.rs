//! Local-player gameplay and presentation.
//!
//! The player body is simulation state. Aim is control-frame-local view intent.
//! Camera placement is presentation. Device input is an adapter that writes
//! those components. Keeping those layers explicit makes them independently
//! replaceable by mods, AI, replay/network input or a different camera rig.

mod camera;
mod components;
mod controls;
mod hud;
mod lifecycle;
mod spawn;
pub mod cursor;
mod model;
mod stance;

pub use camera::{CameraMode, PlayerCamera, ThirdPersonCamera, ViewCameraProfile};
pub use components::{Player, PlayerAim, PlayerController, PlayerDead};


use avian3d::prelude::{
    ActiveCollisionHooks, Collider, CollisionLayers, CustomPositionIntegration, CustomVelocityIntegration,
    LinearVelocity, RigidBody,
};
use bevy::{
    app::RunFixedMainLoop,
    camera::visibility::RenderLayers,
    prelude::*,
};

use crate::{
    ecs::{
        UsfEntity, UsfLogicalProjection, UsfManifestationAuthority, UsfManifestationOf,
        UsfPresentationProjectionOf,
    },
    input_focus::{InputFocus, InputFocusSet},
    portal::{MAIN_PORTAL_LAYER, PortalSplitTraveler, PortalTraveler, PortalView},
    physics::{
        character::{
            CharacterControlFrame, CharacterDimensions, CharacterGroundState,
            CharacterLocomotionFrame, CharacterMotor, CharacterMovementConfig,
            CharacterMovementInput,
        },
        topology::{KinematicQueryExclusions, SpatialSplitBox, SpatialSplitPeer},
    },
    spatial::{
        SpatialDemandSource, SpatialRefinementDemand, SpatialScale, UsfInteractionProjection,
        UsfPosition, UsfScaleLayer,
        UsfSpatialAnchor, UsfTravelNeighborhood, UsfViewAnchor,
        UsfViewContext, UsfViewRenderAnchor,
    },
    thermal::{ThermalBody, ThermalInjury, ThermalSpatialSample},
    view::{PrimaryGameView, PrimaryViewPresentation},
    voxel::VoxelMaterializationDemand,
};

use crate::game::control::ControlSet;

use super::{
    GameSet, InputSet, PresentationSet,
    combat::Weapon,
    health::{Died, Health},
};

// Keep a generous horizontal terrain window without materializing a
// 192-metre vertical cube around the player. Semantic voxel resolution stays
// unchanged; this controls only the hot realization working set.
const PLAYER_SPATIAL_DEMAND_HALF_EXTENT: Vec3 = Vec3::new(64.0, 32.0, 64.0);
const PLAYER_SPATIAL_DEMAND_PRIORITY: i32 = 100;

use lifecycle::handle_player_death;
use spawn::spawn_player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<cursor::CursorCapture>()
            .init_resource::<InputFocus>()
            .init_resource::<PrimaryViewPresentation>()
            .register_type::<Player>()
            .register_type::<PlayerController>()
            .register_type::<PlayerDead>()
            .register_type::<PlayerAim>()
            .register_type::<PlayerCamera>()
            .register_type::<ViewCameraProfile>()
            .register_type::<camera::ViewOrientationPolicy>()
            .register_type::<ThirdPersonCamera>()
            .register_type::<CameraMode>()
            .add_systems(Startup, (spawn_player, hud::spawn_flight_hud))
            .add_systems(
                PreUpdate,
                cursor::apply_input_focus.in_set(InputFocusSet::Resolve),
            )
            .add_systems(
                RunFixedMainLoop,
                (controls::look, controls::sample_flight_control_intent)
                    .chain()
                    .in_set(ControlSet::Sample),
            )
            .add_systems(
                RunFixedMainLoop,
                (
                    controls::toggle_local_flight,
                    controls::toggle_local_flight_thrusters,
                    controls::toggle_adaptive_cruise,
                )
                    .chain()
                    .in_set(ControlSet::Request),
            )
            .add_systems(
                RunFixedMainLoop,
                (stance::update_stance, controls::movement)
                    .chain()
                    .in_set(ControlSet::CharacterIntent),
            )
            .add_systems(
                Update,
                cursor::update_cursor_capture.in_set(InputSet::Cursor),
            )
            .add_systems(
                Update,
                (
                    controls::toggle_spatial_demand,
                    controls::zoom_spatial_view,
                    camera::toggle_camera_mode,
                    camera::zoom_third_person,
                )
                    .chain()
                    .in_set(InputSet::Gameplay),
            )
            .add_systems(Update, handle_player_death.in_set(GameSet::Cleanup))
            .add_systems(
                Update,
                (
                    camera::sync_view_camera_profile,
                    camera::sync_player_camera,
                    camera::sync_player_fov,
                    camera::sync_usf_projection_camera,
                    camera::sync_view_subject_presentations,
                    hud::update_flight_hud,
                )
                    .chain()
                    .in_set(PresentationSet::PrimaryView),
            );
    }
}
