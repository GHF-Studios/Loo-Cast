//! Local human-player identity and controller-owned state.
//!
//! Generic controlled-subject locomotion/navigation state lives in
//! `game::locomotion` and `game::navigation`, not here.

use bevy::prelude::*;

/// The local human player's runtime manifestation.
#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct Player;

/// Marks the local human player after its semantic entity dies.
///
/// Death is controller/gameplay state. Whichever subject the player currently
/// controls consumes this through the control adapter rather than owning a
/// duplicate death component.
#[derive(Component, Reflect, Debug, Default, Clone, Copy)]
#[reflect(Component)]
pub struct PlayerDead;

/// Local human input tuning. Physical movement tuning belongs to the controlled
/// subject's locomotion implementation.
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct PlayerController {
    pub look_sensitivity: f32,
    pub sprint_multiplier: f32,
    pub crouch_speed_multiplier: f32,
}

impl Default for PlayerController {
    fn default() -> Self {
        Self {
            look_sensitivity: 0.002,
            sprint_multiplier: 2.0,
            crouch_speed_multiplier: 0.45,
        }
    }
}

/// Live yaw/pitch intent owned by the human controller.
///
/// The controlled subject supplies the physical/control frame; this component
/// supplies only the local controller-relative orientation intent.
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
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
