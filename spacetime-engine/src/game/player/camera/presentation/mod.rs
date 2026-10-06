//! Runtime camera transform, FOV and self-presentation policy.
//!
//! ## Module map
//!
//! - `fov`: Physical-camera field of view and active Scale-Slice near plane.
//! - `pose`: Subject profile handoff and resolved local camera pose.
//! - `projection`: Contextual USF camera mirrors the local view from a bounded semantic anchor.
//! - `visibility`: Primary-view self-presentation policy.
//!
//! Reexports here define the supported surface; child modules hold its implementation.
//!

use super::*;
use crate::{
    ecs::{UsfLogicalRealizationOf, UsfPresentationProjectionOf},
    physics::topology::UsfRuntimeOwnershipQuery,
    portal::DERIVED_VIEW_LAYER,
    view::ViewSubjectPresentation,
};
use bevy::{camera::visibility::RenderLayers, math::DVec3};

mod fov;
mod pose;
mod projection;
mod visibility;

pub(in crate::game::player) use fov::sync_player_fov;
pub(in crate::game::player) use pose::{sync_player_camera, sync_view_camera_profile};
pub(in crate::game::player) use projection::sync_usf_projection_camera;
pub(in crate::game::player) use visibility::sync_view_subject_presentations;
