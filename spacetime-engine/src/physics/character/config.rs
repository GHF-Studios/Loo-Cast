use avian3d::prelude::Collider;
use bevy::prelude::*;

/// Dimensions of the default standing character body, in metres.
pub struct CharacterDimensions;

impl CharacterDimensions {
    pub const HULL_WIDTH: f32 = 0.8128;
    pub const HULL_HEIGHT: f32 = 1.8288;
    pub const HALF_HEIGHT: f32 = 0.9144;
    pub const EYE_HEIGHT: f32 = 1.6256;
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
    /// Maximum commanded ground speed, in m/s.
    pub max_ground_speed: f32,
    /// Ground acceleration coefficient.
    pub ground_acceleration: f32,
    /// Air acceleration coefficient.
    pub air_acceleration: f32,
    /// Ground friction coefficient.
    pub friction: f32,
    /// Speed below which friction uses this value as its control speed, in m/s.
    pub stop_speed: f32,
    /// Per-surface friction multiplier. Material integration can replace this.
    pub surface_friction: f32,

    /// Downward gravitational acceleration magnitude, in m/s².
    pub gravity: f32,
    /// Instantaneous jump speed along locomotion-up, in m/s.
    pub jump_speed: f32,

    /// Air wish-speed cap in m/s. The cap affects remaining speed while the
    /// acceleration magnitude still uses uncapped wish speed.
    pub air_wish_speed_cap: Option<f32>,

    /// Maximum automatic stair height, in metres.
    pub step_height: f32,
    /// Small downward probe used to maintain grounded classification, in metres.
    pub ground_snap_distance: f32,
    /// Minimum `dot(surface_normal, up)` for walkable ground.
    pub min_ground_dot: f32,

    /// Holding jump repeats a jump on the first grounded fixed tick.
    pub auto_bhop: bool,
}

impl CharacterMovementConfig {
    /// Movement preset derived from Source SDK 2013 values, expressed directly
    /// in SI units.
    ///
    /// The name records the reference lineage rather than claiming byte-identical
    /// defaults: this project's adopted ground friction is intentionally 8.0.
    pub fn source_2013() -> Self {
        Self {
            max_ground_speed: 8.128,
            ground_acceleration: 10.0,
            air_acceleration: 10.0,
            friction: 8.0,
            stop_speed: 2.54,
            surface_friction: 1.0,
            gravity: 20.32,
            jump_speed: 6.815_535_5,
            air_wish_speed_cap: Some(0.762),
            step_height: 0.4572,
            ground_snap_distance: 0.0508,
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
