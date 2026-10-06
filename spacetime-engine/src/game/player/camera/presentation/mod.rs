//! Runtime camera transform, FOV and self-presentation policy.

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
