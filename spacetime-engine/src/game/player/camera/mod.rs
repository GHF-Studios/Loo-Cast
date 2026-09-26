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

use avian3d::prelude::{Collider, ShapeCastConfig, SpatialQuery};
use bevy::prelude::*;

use crate::{
    ecs::{UsfLogicalProjection, UsfManifestationOf, UsfManifestations},
    portal::{
        Portal, PortalActive, crossed_aperture_fraction, map_through_portal,
    },
    physics::{
        slice::UsfPhysicsSlices,
        character::{CharacterControlFrame, CharacterDimensions},
    },
    spatial::UsfScaleLayer,
};

use crate::game::{
    control::LocalViewTarget,
    locomotion::CharacterStance,
};

use super::{
    Player, PlayerAim,
    input::{PlayerAction, PlayerInputFrame},
};

/// Available local-player camera presentations.
#[derive(Reflect, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CameraMode {
    #[default]
    FirstPerson,
    ThirdPerson,
}

/// Which orientation authority a target-owned camera rig follows.
#[derive(Reflect, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(super) enum ViewOrientationPolicy {
    /// Character-style accumulated controller look relative to the control frame.
    #[default]
    ControllerLook,
    /// Vehicle-style camera locked to the resolved physical subject attitude.
    SubjectAttitude,
}

/// Persistent intent and transient collision result for a third-person boom.
#[derive(Reflect, Debug, Clone, Copy)]
pub struct ThirdPersonCamera {
    /// Normal authored distance behind the pivot, in physical metres.
    pub base_distance_metres: f32,
    /// Persistent user adjustment relative to [`Self::base_distance_metres`].
    pub zoom_offset_metres: f32,
    /// Minimum distance the user can request through zoom input.
    pub minimum_distance_metres: f32,
    /// Maximum distance the user can request through zoom input.
    pub maximum_distance_metres: f32,
    /// Physical metres added or removed for one scroll step.
    pub zoom_step_metres: f32,
    /// Control-frame-up offset from the eye to the third-person orbit pivot.
    pub pivot_height_metres: f32,
    /// Radius of the sphere swept from the pivot toward the desired camera.
    pub collision_radius_metres: f32,
    /// Extra clearance kept in front of a camera obstruction.
    pub collision_padding_metres: f32,
    /// Collision-constrained path distance from the most recent presentation pass.
    /// This is output/diagnostic state; input never uses it as its zoom base.
    pub resolved_distance_metres: f32,
}

impl ThirdPersonCamera {
    fn distance_limits(&self) -> (f32, f32) {
        let minimum = self.minimum_distance_metres.min(self.maximum_distance_metres).max(0.0);
        let maximum = self
            .minimum_distance_metres
            .max(self.maximum_distance_metres)
            .max(minimum);
        (minimum, maximum)
    }

    pub fn desired_distance_metres(&self) -> f32 {
        let (minimum, maximum) = self.distance_limits();
        (self.base_distance_metres + self.zoom_offset_metres).clamp(minimum, maximum)
    }

    pub fn desired_distance_native(&self, scale: crate::spatial::SpatialScale) -> f32 {
        scale.metres_to_native_f32(self.desired_distance_metres())
    }

    pub fn collision_radius_native(&self, scale: crate::spatial::SpatialScale) -> f32 {
        scale.metres_to_native_f32(self.collision_radius_metres.max(0.0))
    }

    pub fn collision_padding_native(&self, scale: crate::spatial::SpatialScale) -> f32 {
        scale.metres_to_native_f32(self.collision_padding_metres.max(0.0))
    }

    pub fn add_zoom_steps(&mut self, steps: f32) {
        let (minimum, maximum) = self.distance_limits();
        let desired =
            (self.desired_distance_metres() + steps * self.zoom_step_metres.abs()).clamp(minimum, maximum);
        self.zoom_offset_metres = desired - self.base_distance_metres;
    }


}

impl Default for ThirdPersonCamera {
    fn default() -> Self {
        Self {
            base_distance_metres: 4.0,
            zoom_offset_metres: 0.0,
            minimum_distance_metres: 1.0,
            maximum_distance_metres: 10.0,
            zoom_step_metres: 0.5,
            pivot_height_metres: 0.75,
            collision_radius_metres: 0.2,
            collision_padding_metres: 0.05,
            resolved_distance_metres: 4.0,
        }
    }
}

/// Target-owned camera presentation profile.
///
/// Eye geometry and boom tuning belong to the viewed subject, not to the one
/// global camera. Changing viewed subject cannot leak vehicle camera state into
/// character third person (or vice versa).
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct ViewCameraProfile {
    pub preferred_mode: CameraMode,
    orientation_policy: ViewOrientationPolicy,
    pub standing_eye_offset_metres: Vec3,
    pub crouched_eye_offset_metres: Vec3,
    pub third_person: ThirdPersonCamera,
}

impl ViewCameraProfile {
    pub fn character() -> Self {
        Self {
            preferred_mode: CameraMode::FirstPerson,
            orientation_policy: ViewOrientationPolicy::ControllerLook,
            standing_eye_offset_metres: Vec3::Y * CharacterDimensions::CENTER_TO_EYE,
            crouched_eye_offset_metres: Vec3::Y * CharacterDimensions::CROUCH_CENTER_TO_EYE,
            third_person: ThirdPersonCamera::default(),
        }
    }

    /// Reference spacecraft camera policy.
    ///
    /// The placeholder ship has no authored cockpit socket yet, so first person
    /// lives at the semantic hull origin. Third person orbits that SAME origin:
    /// there is no permanent hidden "up boom" beneath the actual boom.
    ///
    /// All authored distances are physical metres and are converted only at the
    /// active Scale-Slice numerical boundary.
    pub fn spacecraft(boom_distance_metres: f32) -> Self {
        let mut third_person = ThirdPersonCamera::default();
        third_person.base_distance_metres = boom_distance_metres.max(1.0);
        third_person.maximum_distance_metres =
            (boom_distance_metres * 3.0).max(third_person.base_distance_metres);
        third_person.pivot_height_metres = 0.0;
        third_person.resolved_distance_metres = third_person.base_distance_metres;

        Self {
            preferred_mode: CameraMode::ThirdPerson,
            orientation_policy: ViewOrientationPolicy::SubjectAttitude,
            standing_eye_offset_metres: Vec3::ZERO,
            crouched_eye_offset_metres: Vec3::ZERO,
            third_person,
        }
    }

    pub const fn uses_controller_look(&self) -> bool {
        matches!(self.orientation_policy, ViewOrientationPolicy::ControllerLook)
    }

    pub fn rig_rotation(
        &self,
        body: &Transform,
        control: &CharacterControlFrame,
    ) -> Quat {
        match self.orientation_policy {
            ViewOrientationPolicy::ControllerLook => control.rotation(),
            ViewOrientationPolicy::SubjectAttitude => body.rotation.normalize(),
        }
    }

    pub fn view_rotation(
        &self,
        body: &Transform,
        control: &CharacterControlFrame,
        aim: &PlayerAim,
    ) -> Quat {
        match self.orientation_policy {
            ViewOrientationPolicy::ControllerLook => control.rotation() * aim.local_rotation(),
            ViewOrientationPolicy::SubjectAttitude => body.rotation.normalize(),
        }
    }

    pub fn eye_offset_metres(&self, stance: Option<&CharacterStance>) -> Vec3 {
        if stance.is_some_and(|stance| stance.crouched) {
            self.crouched_eye_offset_metres
        } else {
            self.standing_eye_offset_metres
        }
    }

    /// Resolves authored physical eye geometry into the current bounded
    /// Scale-Slice chart. Camera/profile data remains in metres everywhere else.
    pub fn eye_offset_native(
        &self,
        stance: Option<&CharacterStance>,
        scale: crate::spatial::SpatialScale,
    ) -> Vec3 {
        self.eye_offset_metres(stance) * scale.metres_to_native_f32(1.0)
    }
}

/// Camera used only for the observer-relative multiscale USF presentation.
///
/// Translation is pinned to the semantic view anchor. Rotation, projection,
/// viewport and render target mirror the local gameplay camera. Local camera
/// rig offsets therefore cannot expose the compressed multiscale scene.
#[derive(Component, Debug, Default)]
pub struct UsfProjectionCamera;

/// State of the one primary local camera.
///
/// Subject-specific geometry/tuning lives on ViewCameraProfile.
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct PlayerCamera {
    pub mode: CameraMode,
    /// Desired horizontal field of view in degrees.
    pub horizontal_fov_degrees: f32,
}

impl Default for PlayerCamera {
    fn default() -> Self {
        Self {
            mode: CameraMode::FirstPerson,
            horizontal_fov_degrees: 110.0,
        }
    }
}

mod input;
mod presentation;
mod third_person;

pub(super) use input::{toggle_camera_mode, zoom_third_person};
pub(super) use presentation::{
    sync_player_camera, sync_player_fov, sync_usf_projection_camera,
    sync_view_camera_profile, sync_view_subject_presentations,
};

use third_person::resolve_third_person_boom;

#[cfg(test)]
mod tests;
