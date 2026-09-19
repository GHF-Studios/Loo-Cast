//! Player-owned simulation and input state.
//!
//! Presentation-only camera state lives in [`super::camera`]. Keeping those
//! concerns separate means gameplay/mod code can reason about the player body
//! without depending on one particular camera implementation.

use bevy::prelude::*;

/// The locally controlled gameplay entity.
#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct Player;

/// Marks the local controlled manifestation after its semantic entity dies.
///
/// Death is gameplay state, not despawn policy. The camera and corpse may remain
/// while control adapters refuse to resurrect movement implicitly.
#[derive(Component, Reflect, Debug, Default, Clone, Copy)]
#[reflect(Component)]
pub struct PlayerDead;

/// Local input tuning. Physical movement tuning remains in
/// [`crate::physics::character::CharacterMovementConfig`].
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct PlayerController {
    /// Mouse radians per logical mouse unit.
    pub look_sensitivity: f32,
    /// Multiplier applied while sprint is held.
    pub sprint_multiplier: f32,
    /// Multiplier applied while crouched.
    pub crouch_speed_multiplier: f32,
    /// Free-flight speed used by developer noclip, in active scale-native units/s.
    pub noclip_speed: f32,
}

impl Default for PlayerController {
    fn default() -> Self {
        Self {
            look_sensitivity: 0.002,
            sprint_multiplier: 2.0,
            crouch_speed_multiplier: 0.45,
            noclip_speed: 20.0,
        }
    }
}

/// Current physical stance of the local player.
#[derive(Component, Reflect, Debug, Default, Clone, Copy)]
#[reflect(Component)]
pub struct PlayerStance {
    pub crouched: bool,
}

/// Developer free-flight state.
#[derive(Component, Reflect, Debug, Default, Clone, Copy)]
#[reflect(Component)]
pub struct PlayerNoclip {
    pub active: bool,
}

/// Free-flight navigation for USF charts outside the local character domain.
///
/// This is ordinary coarse-scale navigation, not developer noclip. Speed is in
/// units native to the active scale and is adjustable over many orders of magnitude.
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct PlayerScaleNavigation {
    pub speed_native: f32,
}

impl Default for PlayerScaleNavigation {
    fn default() -> Self {
        Self { speed_native: 0.05 }
    }
}

impl PlayerScaleNavigation {
    const MIN_SPEED_NATIVE: f32 = 1.0e-7;
    const MAX_SPEED_NATIVE: f32 = 1.0e4;
    const SPEED_DECADES_PER_SCROLL_STEP: f32 = 0.25;

    pub fn adjust_speed(&mut self, scroll_steps: f32) {
        if scroll_steps == 0.0 {
            return;
        }

        let factor =
            10.0_f32.powf(scroll_steps.signum() * Self::SPEED_DECADES_PER_SCROLL_STEP);
        self.speed_native =
            (self.speed_native * factor).clamp(Self::MIN_SPEED_NATIVE, Self::MAX_SPEED_NATIVE);
    }
}

/// Live yaw/pitch offset relative to the character control frame.
///
/// Portal/topology transitions may rotate that base control frame underneath
/// this offset. Mouse look therefore remains responsive without rewriting the
/// player's local aim while the world/view basis settles.
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
