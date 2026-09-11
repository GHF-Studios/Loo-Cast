use avian3d::prelude::Collider;
use bevy::prelude::*;

/// Dimensions from the default Source SDK 2013 standing player hull.
///
/// Source units are inches; Spacetime Engine's current local gameplay space is
/// treated as metres here.
pub struct SourceCharacterDimensions;

impl SourceCharacterDimensions {
    pub const SOURCE_UNIT_METRES: f32 = 0.0254;
    pub const HULL_WIDTH: f32 = 32.0 * Self::SOURCE_UNIT_METRES;
    pub const HULL_HEIGHT: f32 = 72.0 * Self::SOURCE_UNIT_METRES;
    pub const HALF_HEIGHT: f32 = Self::HULL_HEIGHT * 0.5;
    pub const EYE_HEIGHT: f32 = 64.0 * Self::SOURCE_UNIT_METRES;
    pub const CENTER_TO_EYE: f32 = Self::EYE_HEIGHT - Self::HALF_HEIGHT;

    pub fn standing_collider() -> Collider {
        Collider::cuboid(
            Self::HULL_WIDTH,
            Self::HULL_HEIGHT,
            Self::HULL_WIDTH,
        )
    }
}

/// Per-character movement tuning.
///
/// The physical body's local `+Y` axis is locomotion-up. This keeps arbitrary
/// portal/gravity orientation independent from view yaw and pitch.
#[derive(Component, Reflect, Clone, Debug)]
#[reflect(Component)]
pub struct CharacterMovementConfig {
    /// Maximum commanded ground speed.
    pub max_ground_speed: f32,
    /// Source-style ground acceleration coefficient (`sv_accelerate`).
    pub ground_acceleration: f32,
    /// Source-style air acceleration coefficient (`sv_airaccelerate`).
    pub air_acceleration: f32,
    /// Source-style friction coefficient (`sv_friction`).
    pub friction: f32,
    /// Source-style stop speed (`sv_stopspeed`).
    pub stop_speed: f32,
    /// Per-surface friction multiplier. Material integration can replace this.
    pub surface_friction: f32,

    /// Downward gravitational acceleration magnitude.
    pub gravity: f32,
    /// Instantaneous jump speed along locomotion-up.
    pub jump_speed: f32,

    /// Source air wish-speed cap. The cap affects `add_speed`, while the
    /// acceleration term still uses uncapped wish speed.
    pub air_wish_speed_cap: Option<f32>,

    /// Maximum automatic stair height.
    pub step_height: f32,
    /// Small downward probe used to maintain grounded classification.
    pub ground_snap_distance: f32,
    /// Minimum `dot(surface_normal, up)` for walkable ground.
    pub min_ground_dot: f32,

    /// Holding jump repeats a jump on the first grounded fixed tick.
    pub auto_bhop: bool,
}

impl CharacterMovementConfig {
    pub const SOURCE_UNIT_METRES: f32 = SourceCharacterDimensions::SOURCE_UNIT_METRES;

    /// Source SDK 2013-style baseline values converted to metres.
    ///
    /// Individual Source games override several movement constants; this is a
    /// deliberate baseline profile, not a claim that every Source title shares
    /// byte-identical movement tuning.
    pub fn source_2013() -> Self {
        Self {
            max_ground_speed: 320.0 * Self::SOURCE_UNIT_METRES,
            ground_acceleration: 10.0,
            air_acceleration: 10.0,
            friction: 4.0,
            stop_speed: 100.0 * Self::SOURCE_UNIT_METRES,
            surface_friction: 1.0,
            gravity: 800.0 * Self::SOURCE_UNIT_METRES,
            jump_speed: 268.328_16 * Self::SOURCE_UNIT_METRES,
            air_wish_speed_cap: Some(30.0 * Self::SOURCE_UNIT_METRES),
            step_height: 18.0 * Self::SOURCE_UNIT_METRES,
            ground_snap_distance: 2.0 * Self::SOURCE_UNIT_METRES,
            min_ground_dot: 0.7,
            auto_bhop: true,
        }
    }
}

impl Default for CharacterMovementConfig {
    fn default() -> Self {
        Self::source_2013()
    }
}
