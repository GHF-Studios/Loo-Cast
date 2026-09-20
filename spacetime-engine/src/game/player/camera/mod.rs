//! Local-player camera presentation.
//!
//! Third-person zoom has three deliberately separate concepts:
//!
//! 1. `base_distance`: the normal authored boom length;
//! 2. `zoom_offset`: the player's persistent scroll adjustment;
//! 3. `resolved_distance`: the temporary collision-constrained result.
//!
//! Collision may therefore push the camera inward without ever overwriting the
//! user's intended zoom. When the obstruction disappears, the camera returns
//! to `base_distance + zoom_offset`.

use avian3d::prelude::{Collider, ShapeCastConfig, SpatialQuery, SpatialQueryFilter};
use bevy::{camera::visibility::RenderLayers, input::mouse::AccumulatedMouseScroll, prelude::*};

use crate::{
    ecs::{UsfLogicalProjection, UsfManifestationOf, UsfManifestations},
    portal::{
        DERIVED_VIEW_LAYER, Portal, PortalActive, crossed_aperture_fraction, map_through_portal,
    },
    physics::{
        chart::UsfPhysicsCharts,
        character::{CharacterControlFrame, CharacterDimensions},
    },
    spatial::{SpatialScale, UsfScaleLayer},
};

use super::{Player, PlayerAim, PlayerStance, cursor::CursorCapture, model::PlayerModel};

/// Available local-player camera presentations.
#[derive(Reflect, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CameraMode {
    #[default]
    FirstPerson,
    ThirdPerson,
}

/// Persistent intent and transient collision result for a third-person boom.
#[derive(Reflect, Debug, Clone, Copy)]
pub struct ThirdPersonCamera {
    /// Normal authored distance behind the pivot, in active scale-native units.
    pub base_distance: f32,
    /// Persistent user adjustment relative to [`Self::base_distance`].
    pub zoom_offset: f32,
    /// Minimum distance the user can request through zoom input.
    pub minimum_distance: f32,
    /// Maximum distance the user can request through zoom input.
    pub maximum_distance: f32,
    /// Native units added or removed for one scroll step.
    pub zoom_step: f32,
    /// Control-frame-up offset from the eye to the third-person orbit pivot.
    pub pivot_height: f32,
    /// Radius of the sphere swept from the pivot toward the desired camera.
    pub collision_radius: f32,
    /// Extra clearance kept in front of a camera obstruction.
    pub collision_padding: f32,
    /// Collision-constrained path distance from the most recent presentation pass.
    /// This is output/diagnostic state; input never uses it as its zoom base.
    pub resolved_distance: f32,
}

impl ThirdPersonCamera {
    fn distance_limits(&self) -> (f32, f32) {
        let minimum = self.minimum_distance.min(self.maximum_distance).max(0.0);
        let maximum = self
            .minimum_distance
            .max(self.maximum_distance)
            .max(minimum);
        (minimum, maximum)
    }

    pub fn desired_distance(&self) -> f32 {
        let (minimum, maximum) = self.distance_limits();
        (self.base_distance + self.zoom_offset).clamp(minimum, maximum)
    }

    pub fn add_zoom_steps(&mut self, steps: f32) {
        let (minimum, maximum) = self.distance_limits();
        let desired =
            (self.desired_distance() + steps * self.zoom_step.abs()).clamp(minimum, maximum);
        self.zoom_offset = desired - self.base_distance;
    }
}

impl Default for ThirdPersonCamera {
    fn default() -> Self {
        Self {
            base_distance: 4.0,
            zoom_offset: 0.0,
            minimum_distance: 1.0,
            maximum_distance: 10.0,
            zoom_step: 0.5,
            pivot_height: 0.75,
            collision_radius: 0.2,
            collision_padding: 0.05,
            resolved_distance: 4.0,
        }
    }
}

/// Presentation state for the local player's camera.
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct PlayerCamera {
    pub mode: CameraMode,
    /// Control-frame-local standing eye offset.
    pub standing_eye_offset: Vec3,
    /// Control-frame-local crouched eye offset.
    pub crouched_eye_offset: Vec3,
    pub third_person: ThirdPersonCamera,
    /// Desired horizontal field of view in degrees.
    pub horizontal_fov_degrees: f32,
}

impl PlayerCamera {
    pub fn eye_offset(&self, stance: &PlayerStance) -> Vec3 {
        if stance.crouched {
            self.crouched_eye_offset
        } else {
            self.standing_eye_offset
        }
    }

    pub fn view_rotation(&self, control: &CharacterControlFrame, aim: &PlayerAim) -> Quat {
        control.rotation() * aim.local_rotation()
    }

    pub fn eye_position(
        &self,
        body: &Transform,
        control: &CharacterControlFrame,
        stance: &PlayerStance,
    ) -> Vec3 {
        body.translation + control.rotation() * self.eye_offset(stance)
    }
}

impl Default for PlayerCamera {
    fn default() -> Self {
        Self {
            mode: CameraMode::FirstPerson,
            standing_eye_offset: Vec3::Y * CharacterDimensions::CENTER_TO_EYE,
            crouched_eye_offset: Vec3::Y * CharacterDimensions::CROUCH_CENTER_TO_EYE,
            third_person: ThirdPersonCamera::default(),
            horizontal_fov_degrees: 110.0,
        }
    }
}

mod input;
mod presentation;
mod third_person;

pub(super) use input::{toggle_camera_mode, zoom_third_person};
pub(super) use presentation::{sync_player_camera, sync_player_fov, sync_player_model};

use third_person::resolve_third_person_boom;

#[cfg(test)]
mod tests;
