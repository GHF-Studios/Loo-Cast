use bevy::prelude::*;

/// Physics-facing movement intent.
///
/// The motor deliberately does not know about keyboards, cameras or players.
/// Adapters write world-space intent before the fixed loop and keep tuning in
/// [`super::CharacterMovementConfig`] immutable.
#[derive(Component, Reflect, Clone, Debug)]
#[reflect(Component)]
pub struct CharacterMovementInput {
    /// Desired world-space movement direction.
    pub wish_direction: Vec3,
    /// Analog magnitude in `[0, 1]`.
    pub wish_speed_fraction: f32,
    /// Dimensionless multiplier applied to the configured movement speed.
    ///
    /// Sprinting, crouch-speed changes and similar adapter-level intent belong
    /// here rather than mutating [`super::CharacterMovementConfig`] every frame.
    pub speed_multiplier: f32,
    /// Current held state, used for auto-bhop when enabled.
    pub jump_held: bool,
    /// Latched one-shot request consumed by the next fixed movement tick.
    pub jump_pressed: bool,
}

impl Default for CharacterMovementInput {
    fn default() -> Self {
        Self {
            wish_direction: Vec3::ZERO,
            wish_speed_fraction: 0.0,
            speed_multiplier: 1.0,
            jump_held: false,
            jump_pressed: false,
        }
    }
}

impl CharacterMovementInput {
    pub fn set_wish(&mut self, direction: Vec3, speed_fraction: f32) {
        self.wish_direction = direction;
        self.wish_speed_fraction = speed_fraction.clamp(0.0, 1.0);
    }

    pub fn set_speed_multiplier(&mut self, multiplier: f32) {
        self.speed_multiplier = multiplier.max(0.0);
    }

    pub fn clear(&mut self) {
        self.wish_direction = Vec3::ZERO;
        self.wish_speed_fraction = 0.0;
        self.speed_multiplier = 1.0;
        self.jump_held = false;
        self.jump_pressed = false;
    }
}
