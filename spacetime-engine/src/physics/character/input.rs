use bevy::prelude::*;

/// Physics-facing movement intent.
///
/// The motor deliberately does not know about keyboards, cameras or players.
/// Adapters should write a world-space wish direction before the fixed loop.
#[derive(Component, Reflect, Clone, Debug)]
#[reflect(Component)]
pub struct CharacterMovementInput {
    /// Desired world-space movement direction.
    pub wish_direction: Vec3,
    /// Analog magnitude in `[0, 1]`.
    pub wish_speed_fraction: f32,
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

    pub fn clear(&mut self) {
        self.wish_direction = Vec3::ZERO;
        self.wish_speed_fraction = 0.0;
        self.jump_held = false;
        self.jump_pressed = false;
    }
}
