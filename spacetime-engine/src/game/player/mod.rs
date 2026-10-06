//! Local-player gameplay and presentation.
//!
//! The player body is simulation state. Aim is control-frame-local view intent.
//! Camera placement is presentation. Device input is an adapter that writes
//! those components. Keeping those layers explicit makes them independently
//! replaceable by mods, AI, replay/network input or a different camera rig.
//!
//! ## Module map
//!
//! - `camera`: Local-player camera presentation.
//! - `components`: Local human-player identity and controller-owned state.
//! - `controls`: Human-player controller adapters.
//! - `cursor`: Focus-aware mouse capture.
//! - `hud`: Center-relative flight instrumentation.
//! - `input`: Local-human device bindings and per-frame semantic input snapshot.
//! - `lifecycle`: Adaptation of semantic player death into local control/runtime state.
//! - `model`: Minimal visible representation of the player.
//! - `spawn`: Construction of the local player's semantic entity and runtime manifestations.
//! - `stance`: Physical crouch/stand transitions.
//!
//! The plugin or configure entrypoint here wires the child systems and resources.
//!

mod camera;
mod components;
mod controls;
pub mod cursor;
mod hud;
mod input;
mod lifecycle;
mod model;
mod spawn;
mod stance;

pub use camera::{CameraMode, PlayerCamera, ThirdPersonCamera, ViewCameraProfile};
pub(crate) use camera::{DebugFreecam, FreecamControlPolicy, FreecamProjectionPolicy};
pub use components::{Player, PlayerAim, PlayerController, PlayerDead};

pub(crate) use input::{
    BINDABLE_INPUT_NAMES, PLAYER_BIND_TARGETS, PlayerAction, PlayerInputBindings, PlayerInputFrame,
};

use avian3d::prelude::{
    ActiveCollisionHooks, CollisionLayers, CustomPositionIntegration, CustomVelocityIntegration,
    LinearVelocity, RigidBody,
};
use bevy::{app::RunFixedMainLoop, camera::visibility::RenderLayers, prelude::*};

use crate::{
    ecs::{
        UsfAuthorityPartitionOf, UsfEntity, UsfLogicalRealizationOf, UsfOwnershipQuery,
        UsfPresentationProjectionOf, UsfPresentationView, UsfPresentationViewOf,
    },
    input_focus::{InputFocus, InputFocusSet},
    physics::{
        character::{
            CharacterControlFrame, CharacterDimensions, CharacterGroundState,
            CharacterLocomotionFrame, CharacterMotor, CharacterMovementConfig,
            CharacterMovementIntent, GravityAlignedLocomotionFrame,
        },
        topology::{KinematicQueryExclusions, SpatialSplitPeer},
    },
    portal::{MAIN_PORTAL_LAYER, PortalSplitTraveler, PortalTraveler, PortalView},
    spatial::{
        SpatialDemandSource, SpatialRefinementDemand, SpatialScale, UsfInteractionProjection,
        UsfPosition, UsfScaleLayer, UsfSpatialAnchor, UsfTravelNeighborhood, UsfViewAnchor,
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
            .init_resource::<DebugFreecam>()
            .init_resource::<InputFocus>()
            .init_resource::<input::PlayerInputBindings>()
            .init_resource::<input::PlayerInputFrame>()
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
            .add_systems(Startup, (spawn_player, hud::spawn_flight_hud_presentation))
            .configure_sets(
                PreUpdate,
                (
                    InputFocusSet::Resolve,
                    input::PlayerInputSet::Cursor,
                    input::PlayerInputSet::Sample,
                )
                    .chain(),
            )
            .add_systems(
                PreUpdate,
                cursor::apply_input_focus.in_set(InputFocusSet::Resolve),
            )
            .add_systems(
                PreUpdate,
                cursor::update_cursor_capture.in_set(input::PlayerInputSet::Cursor),
            )
            .add_systems(
                PreUpdate,
                input::dispatch_bound_console_commands
                    .after(InputFocusSet::Resolve)
                    .before(input::PlayerInputSet::Sample),
            )
            .add_systems(
                PreUpdate,
                input::sample_player_input.in_set(input::PlayerInputSet::Sample),
            )
            .add_systems(
                RunFixedMainLoop,
                controls::sample_flight_control_intent.in_set(ControlSet::Sample),
            )
            .add_systems(
                RunFixedMainLoop,
                (
                    controls::toggle_attitude_law,
                    controls::toggle_thrusters,
                    controls::toggle_rcs,
                    controls::toggle_adaptive_cruise,
                )
                    .chain()
                    .in_set(ControlSet::Request),
            )
            .add_systems(
                RunFixedMainLoop,
                (
                    stance::update_stance,
                    controls::write_character_movement_intent,
                )
                    .chain()
                    .in_set(ControlSet::CharacterIntent),
            )
            .add_systems(
                Update,
                (
                    // Mouse motion is accumulated once per rendered frame.
                    // Consume it exactly once here; fixed-step simulation may
                    // run zero or multiple ticks for the same render frame.
                    camera::update_freecam,
                    controls::write_player_view_intent,
                    controls::toggle_spatial_demand,
                    controls::adjust_view_scale_bias,
                    controls::adjust_manual_travel_pace,
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
                    camera::sync_freecam_observer_policy,
                    camera::sync_view_camera_profile,
                    camera::sync_player_camera,
                    camera::sync_player_fov,
                    camera::sync_usf_projection_camera,
                    camera::sync_view_subject_presentations,
                    hud::project_flight_hud,
                )
                    .chain()
                    .in_set(PresentationSet::PrimaryView),
            );
    }
}
