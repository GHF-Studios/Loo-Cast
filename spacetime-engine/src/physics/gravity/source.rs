//! Canonical gravity source primitives.

use bevy::{math::DVec3, prelude::*};

use crate::spatial::{SpatialScale, UsfPosition};

/// Canonical spherical gravity source.
///
/// `field_scale` is only the numerically appropriate chart in which to measure
/// source-relative displacement. It is not semantic ownership of gravity.
///
/// Outside the authored spherical body the field is fully characterized by the
/// standard gravitational parameter `μ = g_surface * radius²`. That quantity is
/// intentionally exposed because it is also the physically meaningful weight a
/// future far-field aggregate representation would compose.
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct RadialGravitySource {
    radius_metres: f64,
    field_scale: SpatialScale,
    surface_gravity_metres_per_second2: f32,
}

impl RadialGravitySource {
    pub fn new(
        radius_metres: f64,
        field_scale: SpatialScale,
        surface_gravity_metres_per_second2: f32,
    ) -> Self {
        assert!(radius_metres.is_finite() && radius_metres > 0.0);
        assert!(
            surface_gravity_metres_per_second2.is_finite()
                && surface_gravity_metres_per_second2 >= 0.0
        );
        Self {
            radius_metres,
            field_scale,
            surface_gravity_metres_per_second2,
        }
    }

    pub const fn radius_metres(self) -> f64 { self.radius_metres }
    pub const fn field_scale(self) -> SpatialScale { self.field_scale }
    pub const fn surface_gravity_metres_per_second2(self) -> f32 {
        self.surface_gravity_metres_per_second2
    }

    pub fn gravitational_parameter_metres3_per_second2(self) -> f64 {
        f64::from(self.surface_gravity_metres_per_second2) * self.radius_metres.powi(2)
    }

    pub(super) fn acceleration_at(
        self,
        source_center: &UsfPosition,
        position: &UsfPosition,
    ) -> Option<DVec3> {
        let relative_native = position
            .relative_at_scale_bounded(source_center, self.field_scale, f32::MAX)
            .ok()?;
        let relative_metres = DVec3::new(
            f64::from(relative_native.x),
            f64::from(relative_native.y),
            f64::from(relative_native.z),
        ) * self.field_scale.metres_per_native();

        let distance_metres = relative_metres.length();
        if distance_metres <= f64::EPSILON
            || self.surface_gravity_metres_per_second2 <= 0.0
        {
            return Some(DVec3::ZERO);
        }

        let magnitude = if distance_metres >= self.radius_metres {
            self.gravitational_parameter_metres3_per_second2() / distance_metres.powi(2)
        } else {
            f64::from(self.surface_gravity_metres_per_second2)
                * (distance_metres / self.radius_metres).clamp(0.0, 1.0)
        };

        Some(-relative_metres / distance_metres * magnitude)
    }
}
