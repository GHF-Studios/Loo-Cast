//! Canonical orientation for movable semantic spatial frames.
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
