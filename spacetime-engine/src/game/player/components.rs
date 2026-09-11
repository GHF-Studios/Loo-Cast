use bevy::prelude::*;

/// The locally controlled gameplay entity.
#[derive(Component)]
pub struct Player;

/// Configuration for the built-in world-up controller.
#[derive(Component, Debug, Clone, Copy)]
pub struct PlayerController {
    pub move_speed: f32,
    pub look_sensitivity: f32,
}

impl Default for PlayerController {
    fn default() -> Self {
        Self {
            move_speed: 5.0,
            look_sensitivity: 0.002,
        }
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
    pub first_person_offset: Vec3,
    pub third_person_offset: Vec3,
}

impl Default for PlayerCamera {
    fn default() -> Self {
        Self {
            mode: CameraMode::FirstPerson,
            first_person_offset: Vec3::ZERO,

            // +Z is behind the player because forward is local -Z.
            third_person_offset: Vec3::new(0.0, 0.75, 4.0),
        }
    }
}
