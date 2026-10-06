//! Local-player camera presentation.
//!
//! Third-person zoom has three deliberately separate concepts:
//!
//! 1. `base_distance_metres`: the normal authored boom length;
//! 2. `zoom_offset_metres`: the player's persistent scroll adjustment;
//! 3. `resolved_distance_metres`: the temporary collision-constrained result.
//!
//! Collision may therefore push the camera inward without ever overwriting the
//! user's intended zoom. When the obstruction disappears, the camera returns
//! to `base_distance_metres + zoom_offset_metres`.
//!
//! ## Module map
//!
//! - `model`: Target-owned camera profile and primary-camera state.
//! - `freecam`: Detached local debug camera and observer-policy adapter.
//! - `input`: Local camera-mode and third-person zoom intent.
//! - `presentation`: Runtime camera transform, FOV and self-presentation policy.
//! - `third_person`: Collision- and portal-aware third-person boom resolution.
//!
//! Reexports here define the supported surface; child modules hold its implementation.
//!

mod model;

pub(in crate::game::player) use model::ViewOrientationPolicy;
pub use model::{
    CameraMode, PlayerCamera, ThirdPersonCamera, UsfProjectionCamera, ViewCameraProfile,
};

use avian3d::prelude::{Collider, ShapeCastConfig, SpatialQuery};
use bevy::prelude::*;

use crate::{
    physics::{character::CharacterControlFrame, slice::UsfPhysicsSliceQuery},
    portal::{Portal, PortalActive, crossed_aperture_fraction, map_through_portal},
    spatial::UsfScaleLayer,
};

use crate::game::{control::LocalViewTarget, locomotion::CharacterStance};

use super::{
    Player, PlayerAim,
    input::{PlayerAction, PlayerInputFrame},
};

mod freecam;
mod input;
mod presentation;
mod third_person;

pub(crate) use freecam::{DebugFreecam, FreecamControlPolicy, FreecamProjectionPolicy};
pub(super) use freecam::{sync_freecam_observer_policy, update_freecam};
pub(super) use input::{toggle_camera_mode, zoom_third_person};
pub(super) use presentation::{
    sync_player_camera, sync_player_fov, sync_usf_projection_camera, sync_view_camera_profile,
    sync_view_subject_presentations,
};

use third_person::resolve_third_person_boom;
