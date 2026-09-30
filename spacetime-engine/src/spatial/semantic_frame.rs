// Installer tranche: usf-semantic-celestial-frames-body-local-edits-v1//! Canonical orientation for movable semantic spatial frames.
//!
//! Position authority remains the ordinary [`UsfPosition`] component on the
//! semantic entity. This component owns orientation only; consumers receive the
//! entity position explicitly so no capability can silently acquire a second
//! mutable world-space anchor.

use bevy::{math::{DQuat, DVec3}, prelude::*};

use super::{SpatialScale, UsfPosition, UsfPositionError};

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct UsfSemanticFrame {
    orientation: DQuat,
}

impl Default for UsfSemanticFrame {
    fn default() -> Self {
        Self::identity()
    }
}

impl UsfSemanticFrame {
    pub const fn identity() -> Self {
        Self { orientation: DQuat::IDENTITY }
    }

    pub fn from_orientation(orientation: DQuat) -> Self {
        assert!(orientation.is_finite(), "semantic-frame orientation must be finite");
        assert!(orientation.length_squared() > f64::EPSILON, "semantic-frame orientation must be non-zero");
        Self { orientation: orientation.normalize() }
    }

    pub const fn orientation(self) -> DQuat {
        self.orientation
    }

    /// Rotates a body-local direction/vector into canonical USF axes.
    pub fn local_direction_to_world(self, local: Vec3) -> Vec3 {
        let world = self.orientation * DVec3::new(
            f64::from(local.x),
            f64::from(local.y),
            f64::from(local.z),
        );
        Vec3::new(world.x as f32, world.y as f32, world.z as f32)
    }

    /// Rotates a canonical-USF direction/vector into body-local axes.
    pub fn world_direction_to_local(self, world: Vec3) -> Vec3 {
        let local = self.orientation.conjugate() * DVec3::new(
            f64::from(world.x),
            f64::from(world.y),
            f64::from(world.z),
        );
        Vec3::new(local.x as f32, local.y as f32, local.z as f32)
    }

    pub fn local_metres_to_world(
        self,
        origin: UsfPosition,
        local_metres: DVec3,
    ) -> Result<UsfPosition, UsfPositionError> {
        origin.translated_metres_f64(self.orientation * local_metres)
    }

    pub fn world_to_local_metres(
        self,
        origin: &UsfPosition,
        world: &UsfPosition,
        measurement_scale: SpatialScale,
        max_abs_native: f64,
    ) -> Result<DVec3, UsfPositionError> {
        let world_native = world.relative_at_scale_bounded_f64(
            origin,
            measurement_scale,
            max_abs_native,
        )?;
        let world_metres = world_native * measurement_scale.metres_per_native();
        Ok(self.orientation.conjugate() * world_metres)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_near(actual: DVec3, expected: DVec3) {
        assert!((actual - expected).length() < 1.0e-6, "actual={actual:?}, expected={expected:?}");
    }

    #[test]
    fn identity_frame_round_trips_offsets_at_fine_and_coarse_measurement_scales() {
        let frame = UsfSemanticFrame::identity();
        let origin = UsfPosition::zero(SpatialScale::MIN);
        let local = DVec3::new(12.25, -3.5, 27.75);
        let world = frame.local_metres_to_world(origin, local).unwrap();

        assert_near(
            frame.world_to_local_metres(&origin, &world, SpatialScale::ZERO, f64::MAX).unwrap(),
            local,
        );
        let coarse = SpatialScale::new(6).unwrap();
        assert_near(
            frame.world_to_local_metres(&origin, &world, coarse, f64::MAX).unwrap(),
            local,
        );
    }

    #[test]
    fn rotated_frame_rotates_directions_without_moving_origin() {
        let frame = UsfSemanticFrame::from_orientation(
            DQuat::from_rotation_z(std::f64::consts::FRAC_PI_2),
        );
        let world = frame.local_direction_to_world(Vec3::X);
        assert!((world - Vec3::Y).length() < 1.0e-6);
        assert!((frame.world_direction_to_local(world) - Vec3::X).length() < 1.0e-6);
    }

    #[test]
    fn translating_semantic_origin_moves_projection_without_changing_local_coordinate() {
        let frame = UsfSemanticFrame::identity();
        let origin_a = UsfPosition::zero(SpatialScale::ZERO);
        let origin_b = origin_a.translated_at_scale(SpatialScale::ZERO, Vec3::X * 100.0).unwrap();
        let local = DVec3::new(7.0, 2.0, -1.0);
        let world_a = frame.local_metres_to_world(origin_a, local).unwrap();
        let world_b = frame.local_metres_to_world(origin_b, local).unwrap();

        assert_near(
            frame.world_to_local_metres(&origin_a, &world_a, SpatialScale::ZERO, 1_000.0).unwrap(),
            local,
        );
        assert_near(
            frame.world_to_local_metres(&origin_b, &world_b, SpatialScale::ZERO, 1_000.0).unwrap(),
            local,
        );
        let moved = world_b.relative_at_scale_bounded_f64(&world_a, SpatialScale::ZERO, 1_000.0).unwrap();
        assert_near(moved, DVec3::X * 100.0);
    }
}
