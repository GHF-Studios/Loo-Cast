use bevy::prelude::*;

use crate::physics::character::CharacterDimensions;

/// The locally controlled gameplay entity.
#[derive(Component)]
pub struct Player;

/// Configuration for local-player control mapping.
#[derive(Component, Debug, Clone, Copy)]
pub struct PlayerController {
    pub look_sensitivity: f32,
}

impl Default for PlayerController {
    fn default() -> Self {
        Self {
            look_sensitivity: 0.002,
        }
    }
}

/// View orientation relative to the physical player body frame.
///
/// Body orientation is reserved for physical/topological orientation. Looking
/// around therefore never tilts or yaws the collision hull.
#[derive(Component, Debug, Clone, Copy)]
pub struct PlayerAim {
    pub yaw: f32,
    pub pitch: f32,
    pub min_pitch: f32,
    pub max_pitch: f32,
}

impl Default for PlayerAim {
    fn default() -> Self {
        Self {
            yaw: 0.0,
            pitch: 0.0,
            min_pitch: -1.5,
            max_pitch: 1.5,
        }
    }
}

impl PlayerAim {
    pub fn yaw_rotation(&self) -> Quat {
        Quat::from_rotation_y(self.yaw)
    }

    pub fn local_rotation(&self) -> Quat {
        self.yaw_rotation() * Quat::from_rotation_x(self.pitch)
    }
}

/// Available local-player camera presentations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CameraMode {
    #[default]
    FirstPerson,
    ThirdPerson,
}

/// Presentation state for the local player's camera.
#[derive(Component, Debug)]
pub struct PlayerCamera {
    pub mode: CameraMode,
    /// Body-local offset to the eye point.
    pub first_person_offset: Vec3,
    /// View-local offset from the eye point in third person.
    pub third_person_offset: Vec3,
}

impl PlayerCamera {
    /// Resolves the camera without reading its presentation transform.
    ///
    /// Input/actions can use this before presentation runs, avoiding a stale
    /// one-frame camera transform.
    pub fn resolve_transform(
        &self,
        body: &Transform,
        aim: &PlayerAim,
    ) -> Transform {
        let view_rotation = body.rotation * aim.local_rotation();
        let eye = body.translation + body.rotation * self.first_person_offset;

        let translation = match self.mode {
            CameraMode::FirstPerson => eye,
            CameraMode::ThirdPerson => {
                eye + view_rotation * self.third_person_offset
            }
        };

        Transform {
            translation,
            rotation: view_rotation,
            ..default()
        }
    }
}

impl Default for PlayerCamera {
    fn default() -> Self {
        Self {
            mode: CameraMode::FirstPerson,
            first_person_offset: Vec3::Y * CharacterDimensions::CENTER_TO_EYE,

            // +Z is behind because forward is local -Z.
            third_person_offset: Vec3::new(0.0, 0.75, 4.0),
        }
    }
}
