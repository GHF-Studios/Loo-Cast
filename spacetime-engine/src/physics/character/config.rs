use bevy::{math::DVec3, prelude::*};

use crate::{physics::PhysicalBoxHull, spatial::SpatialScale};

/// Dimensions of the default character body, in metres.
pub struct CharacterDimensions;

impl CharacterDimensions {
    pub const HULL_WIDTH: f32 = 0.8128;
    pub const HULL_HEIGHT: f32 = 1.9;
    pub const HALF_HEIGHT: f32 = Self::HULL_HEIGHT / 2.0;
    pub const EYE_HEIGHT: f32 = 1.73;
    pub const CENTER_TO_EYE: f32 = Self::EYE_HEIGHT - Self::HALF_HEIGHT;

    pub const CROUCH_HEIGHT: f32 = 1.2;
    pub const CROUCH_HALF_HEIGHT: f32 = Self::CROUCH_HEIGHT / 2.0;
    pub const CROUCH_EYE_HEIGHT: f32 = 1.05;
    pub const CROUCH_CENTER_TO_EYE: f32 = Self::CROUCH_EYE_HEIGHT - Self::CROUCH_HALF_HEIGHT;

    pub fn standing_hull() -> PhysicalBoxHull {
        PhysicalBoxHull::from_size_metres(Vec3::new(
            Self::HULL_WIDTH,
            Self::HULL_HEIGHT,
            Self::HULL_WIDTH,
        ))
    }

    pub fn crouching_hull() -> PhysicalBoxHull {
        PhysicalBoxHull::from_size_metres(Vec3::new(
            Self::HULL_WIDTH,
            Self::CROUCH_HEIGHT,
            Self::HULL_WIDTH,
        ))
    }
}

/// Per-character movement tuning.
///
/// [`super::CharacterLocomotionFrame`] defines locomotion/gravity up separately
/// from the body's temporary physical/topological orientation.
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

    /// Dimensionless response multiplier applied to physical gravity.
    ///
    /// This is locomotion/game-feel policy, not environmental gravity. The
    /// Source-derived default maps Earth gravity to the controller's historical
    /// 20.32 m/s² fall response while preserving relative differences between
    /// environments.
    pub gravity_response_multiplier: f32,
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
    /// defaults. This project's canonical movement intentionally adopts stronger
    /// ground friction and a higher jump for the portal-heavy game feel.
    pub fn source_2013() -> Self {
        Self {
            max_ground_speed: 8.128,
            ground_acceleration: 10.0,
            air_acceleration: 10.0,
            friction: 8.0,
            stop_speed: 2.54,
            surface_friction: 1.0,
            gravity_response_multiplier: 2.072_063_4,
            jump_speed: 9.144,
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


/// Chart-native parameters resolved for one character fixed tick.
///
/// This is intentionally not an ECS component. [`CharacterMovementConfig`]
/// remains canonical SI policy; conversion happens exactly at the numerical
/// motor boundary.
#[derive(Clone, Copy, Debug)]
pub(super) struct ResolvedCharacterMovementConfig {
    pub max_ground_speed: f32,
    pub ground_acceleration: f32,
    pub air_acceleration: f32,
    pub friction: f32,
    pub stop_speed: f32,
    pub surface_friction: f32,
    pub gravity_acceleration_native: Vec3,
    pub jump_speed: f32,
    pub air_wish_speed_cap: Option<f32>,
    pub step_height: f32,
    pub ground_snap_distance: f32,
    pub min_ground_dot: f32,
    pub auto_bhop: bool,
}

impl CharacterMovementConfig {
    pub(super) fn resolve_for_chart(
        &self,
        scale: SpatialScale,
        gravity_metres_per_second2: DVec3,
    ) -> ResolvedCharacterMovementConfig {
        let gravity = gravity_metres_per_second2
            * f64::from(self.gravity_response_multiplier.max(0.0));
        let to_native = |value: f32| scale.metres_to_native_f32(value);
        let component = |value: f64| {
            scale
                .metres_to_native_f64(value)
                .clamp(-(f32::MAX as f64), f32::MAX as f64) as f32
        };

        ResolvedCharacterMovementConfig {
            max_ground_speed: to_native(self.max_ground_speed),
            ground_acceleration: self.ground_acceleration,
            air_acceleration: self.air_acceleration,
            friction: self.friction,
            stop_speed: to_native(self.stop_speed),
            surface_friction: self.surface_friction,
            gravity_acceleration_native: Vec3::new(
                component(gravity.x),
                component(gravity.y),
                component(gravity.z),
            ),
            jump_speed: to_native(self.jump_speed),
            air_wish_speed_cap: self.air_wish_speed_cap.map(to_native),
            step_height: to_native(self.step_height),
            ground_snap_distance: to_native(self.ground_snap_distance),
            min_ground_dot: self.min_ground_dot,
            auto_bhop: self.auto_bhop,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_profile_preserves_historical_earth_fall_response() {
        let resolved = CharacterMovementConfig::source_2013().resolve_for_chart(
            SpatialScale::ZERO,
            DVec3::NEG_Y * 9.80665,
        );
        assert!((resolved.gravity_acceleration_native.length() - 20.32).abs() < 1.0e-3);
    }

    #[test]
    fn si_policy_resolves_equivalently_across_runtime_charts() {
        let config = CharacterMovementConfig::source_2013();
        let scale = SpatialScale::new(3).unwrap();
        let resolved = config.resolve_for_chart(scale, DVec3::NEG_Y * 9.80665);

        assert!(
            (f64::from(resolved.max_ground_speed) * scale.metres_per_native()
                - f64::from(config.max_ground_speed))
                .abs()
                < 1.0e-5
        );
        assert!(
            (f64::from(resolved.jump_speed) * scale.metres_per_native()
                - f64::from(config.jump_speed))
                .abs()
                < 1.0e-5
        );
    }
}
