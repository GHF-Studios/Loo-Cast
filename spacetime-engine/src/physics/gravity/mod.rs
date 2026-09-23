//! Canonical USF gravity-field queries.
//!
//! Gravity sources are semantic physical facts. Consumers query the field at a
//! canonical [`UsfPosition`] and receive SI acceleration; navigation, cameras,
//! locomotion regimes and runtime Scale Slices do not own gravity.
//!
//! [`GravityFieldQuery`] is deliberately a query boundary rather than a storage
//! contract. Its current exact backend evaluates the sparse set of canonical
//! sources directly. The planned scale-quantized USF field cache can replace
//! that backend without changing character, flight, telemetry or other
//! consumers.

use bevy::{
    app::RunFixedMainLoop,
    ecs::system::SystemParam,
    math::DVec3,
    prelude::*,
};

use crate::spatial::{SpatialScale, UsfPosition, UsfScaleLayer, UsfSpatialFrame};

/// Canonical spherical gravity source.
///
/// `field_scale` is only the numerically appropriate chart in which to measure
/// source-relative displacement. It is not semantic ownership of gravity.
#[derive(Component, Debug, Clone, Copy)]
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

    fn acceleration_at(self, position: &UsfPosition) -> Option<DVec3> {
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

        let factor = if distance_metres >= self.radius_metres {
            (self.radius_metres / distance_metres).powi(2)
        } else {
            // Finite uniform-sphere interior approximation. This keeps the
            // field continuous and prevents invalid/missing collision from
            // becoming a singularity at the semantic body center.
            (distance_metres / self.radius_metres).clamp(0.0, 1.0)
        };
        let magnitude =
            f64::from(self.surface_gravity_metres_per_second2) * factor;
        Some(-relative_metres / distance_metres * magnitude)
    }
}

/// Fixed-tick field sample attached to a runtime subject.
///
/// The vector is expressed in canonical USF axes and SI m/s². It is independent
/// from the subject's active numerical Scale Slice.
#[derive(Component, Debug, Clone, Copy)]
pub struct GravitySample {
    acceleration_metres_per_second2: DVec3,
    strongest_source: Option<Entity>,
}

impl Default for GravitySample {
    fn default() -> Self {
        Self {
            acceleration_metres_per_second2: DVec3::ZERO,
            strongest_source: None,
        }
    }
}

impl GravitySample {
    pub const fn acceleration_metres_per_second2(self) -> DVec3 {
        self.acceleration_metres_per_second2
    }

    pub const fn strongest_source(self) -> Option<Entity> {
        self.strongest_source
    }

    pub fn magnitude_metres_per_second2(self) -> f32 {
        self.acceleration_metres_per_second2
            .length()
            .clamp(0.0, f64::from(f32::MAX)) as f32
    }
}

/// Exact gravity query over canonical sources.
///
/// This is intentionally a [`SystemParam`]: consumers depend on "query gravity
/// here", not on how the field is stored. The forthcoming hierarchical sparse
/// cache will preserve this boundary while replacing direct source iteration.
#[derive(SystemParam)]
pub struct GravityFieldQuery<'w, 's> {
    sources: Query<'w, 's, (Entity, &'static RadialGravitySource)>,
}

impl GravityFieldQuery<'_, '_> {
    pub fn sample(&self, position: &UsfPosition) -> GravitySample {
        let mut acceleration = DVec3::ZERO;
        let mut strongest = None;
        let mut strongest_magnitude2 = 0.0_f64;

        for (entity, source) in self.sources.iter() {
            let Some(contribution) = source.acceleration_at(position) else {
                continue;
            };
            let magnitude2 = contribution.length_squared();
            acceleration += contribution;
            if magnitude2 > strongest_magnitude2 {
                strongest_magnitude2 = magnitude2;
                strongest = Some(entity);
            }
        }

        GravitySample {
            acceleration_metres_per_second2: acceleration,
            strongest_source: strongest,
        }
    }
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GravitySet {
    Sample,
}

fn sample_gravity_receivers(
    frame: Res<UsfSpatialFrame>,
    gravity: GravityFieldQuery,
    mut receivers: Query<(&Transform, &UsfScaleLayer, &mut GravitySample)>,
) {
    for (transform, layer, mut sample) in &mut receivers {
        let Ok(position) = frame
            .origin()
            .translated_at_scale(layer.scale(), transform.translation)
        else {
            *sample = GravitySample::default();
            continue;
        };
        *sample = gravity.sample(&position);
    }
}

pub struct GravityPlugin;

impl Plugin for GravityPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(RunFixedMainLoop, GravitySet::Sample)
            .add_systems(
                RunFixedMainLoop,
                sample_gravity_receivers.in_set(GravitySet::Sample),
            );
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
}
