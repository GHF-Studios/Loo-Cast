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
    center: UsfPosition,
    radius_metres: f64,
    field_scale: SpatialScale,
    surface_gravity_metres_per_second2: f32,
}

impl RadialGravitySource {
    pub fn new(
        center: UsfPosition,
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
            center,
            radius_metres,
            field_scale,
            surface_gravity_metres_per_second2,
        }
    }

    pub const fn center(self) -> UsfPosition {
        self.center
    }

    pub const fn radius_metres(self) -> f64 {
        self.radius_metres
    }

    pub const fn field_scale(self) -> SpatialScale {
        self.field_scale
    }

    pub const fn surface_gravity_metres_per_second2(self) -> f32 {
        self.surface_gravity_metres_per_second2
    }

    pub fn gravitational_parameter_metres3_per_second2(self) -> f64 {
        f64::from(self.surface_gravity_metres_per_second2) * self.radius_metres.powi(2)
    }

    pub(super) fn acceleration_at(self, position: &UsfPosition) -> Option<DVec3> {
        let relative_native = position
            .relative_at_scale_bounded(&self.center, self.field_scale, f32::MAX)
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
            self.gravitational_parameter_metres3_per_second2()
                / distance_metres.powi(2)
        } else {
            // Finite uniform-sphere interior approximation. This keeps the
            // field continuous and prevents missing collision from becoming a
            // singularity at the semantic body center.
            f64::from(self.surface_gravity_metres_per_second2)
                * (distance_metres / self.radius_metres).clamp(0.0, 1.0)
        };

        Some(-relative_metres / distance_metres * magnitude)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn source(surface_gravity: f32) -> RadialGravitySource {
        RadialGravitySource::new(
            UsfPosition::zero(SpatialScale::ZERO),
            10.0,
            SpatialScale::ZERO,
            surface_gravity,
        )
    }

    #[test]
    fn radial_surface_sample_matches_authored_gravity() {
        let sample_position = UsfPosition::zero(SpatialScale::ZERO)
            .translated_at_scale(SpatialScale::ZERO, Vec3::Y * 10.0)
            .unwrap();
        let acceleration = source(9.0).acceleration_at(&sample_position).unwrap();

        assert!((acceleration - DVec3::NEG_Y * 9.0).length() < 1.0e-6);
    }

    #[test]
    fn radial_exterior_follows_inverse_square_law() {
        let sample_position = UsfPosition::zero(SpatialScale::ZERO)
            .translated_at_scale(SpatialScale::ZERO, Vec3::Y * 20.0)
            .unwrap();
        let acceleration = source(8.0).acceleration_at(&sample_position).unwrap();

        assert!((acceleration.length() - 2.0).abs() < 1.0e-6);
    }

    #[test]
    fn radial_interior_is_finite_and_linear() {
        let sample_position = UsfPosition::zero(SpatialScale::ZERO)
            .translated_at_scale(SpatialScale::ZERO, Vec3::Y * 5.0)
            .unwrap();
        let acceleration = source(8.0).acceleration_at(&sample_position).unwrap();

        assert!((acceleration.length() - 4.0).abs() < 1.0e-6);
    }

    #[test]
    fn gravitational_parameter_matches_surface_authoring() {
        let source = source(8.0);
        assert!((source.gravitational_parameter_metres3_per_second2() - 800.0).abs() < 1.0e-9);
    }
}
