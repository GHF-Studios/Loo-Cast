//! Typed gravity-field query and exact reference evaluator.
//!
//! Consumers ask for physical acceleration at a canonical [`UsfPosition`].
//! They do not depend on whether the production backend is eventually direct,
//! hierarchical, multipole-based, grid-backed or hybrid.
//!
//! [`GravityEvaluation::ExactDirect`] is deliberately retained as the reference
//! oracle. Any future approximate backend must be validated against this path
//! before it is allowed to replace it for a query class.

use bevy::{
    ecs::system::SystemParam,
    math::DVec3,
    prelude::*,
};

use crate::spatial::UsfPosition;

use super::RadialGravitySource;

/// Numerical representation used to produce one gravity sample.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GravityEvaluation {
    /// Every authored source was evaluated analytically and summed directly.
    ExactDirect,
}

/// Fixed-tick physical gravity sample attached to a runtime subject.
///
/// Acceleration is expressed in canonical USF axes and SI m/s².
/// `strongest_source` is diagnostic/reference-frame information only; physical
/// acceleration is the vector sum of every evaluated source.
#[derive(Component, Debug, Clone, Copy)]
pub struct GravitySample {
    acceleration_metres_per_second2: DVec3,
    strongest_source: Option<Entity>,
    evaluation: GravityEvaluation,
    evaluated_source_count: usize,
}

impl Default for GravitySample {
    fn default() -> Self {
        Self {
            acceleration_metres_per_second2: DVec3::ZERO,
            strongest_source: None,
            evaluation: GravityEvaluation::ExactDirect,
            evaluated_source_count: 0,
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

    pub const fn evaluation(self) -> GravityEvaluation {
        self.evaluation
    }

    pub const fn evaluated_source_count(self) -> usize {
        self.evaluated_source_count
    }

    pub fn magnitude_metres_per_second2(self) -> f32 {
        self.acceleration_metres_per_second2
            .length()
            .clamp(0.0, f64::from(f32::MAX)) as f32
    }
}

/// Typed gravity query.
///
/// `sample()` intentionally uses the exact reference evaluator today. A future
/// optimized backend may change that dispatch, but `sample_exact()` remains the
/// correctness oracle and does not become approximation-aware.
#[derive(SystemParam)]
pub struct GravityFieldQuery<'w, 's> {
    sources: Query<'w, 's, (Entity, &'static RadialGravitySource)>,
}

impl GravityFieldQuery<'_, '_> {
    pub fn sample(&self, position: &UsfPosition) -> GravitySample {
        self.sample_exact(position)
    }

    pub fn sample_exact(&self, position: &UsfPosition) -> GravitySample {
        exact_direct_sample(
            position,
            self.sources.iter().map(|(entity, source)| (entity, *source)),
        )
    }
}

fn exact_direct_sample(
    position: &UsfPosition,
    sources: impl IntoIterator<Item = (Entity, RadialGravitySource)>,
) -> GravitySample {
    let mut acceleration = DVec3::ZERO;
    let mut strongest_source = None;
    let mut strongest_magnitude2 = 0.0_f64;
    let mut evaluated_source_count = 0usize;

    for (entity, source) in sources {
        evaluated_source_count += 1;

        let Some(contribution) = source.acceleration_at(position) else {
            continue;
        };

        acceleration += contribution;
        let magnitude2 = contribution.length_squared();
        if magnitude2 > strongest_magnitude2 {
            strongest_magnitude2 = magnitude2;
            strongest_source = Some(entity);
        }
    }

    GravitySample {
        acceleration_metres_per_second2: acceleration,
        strongest_source,
        evaluation: GravityEvaluation::ExactDirect,
        evaluated_source_count,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spatial::SpatialScale;

    fn source_at(x_metres: f32, surface_gravity: f32) -> RadialGravitySource {
        let center = UsfPosition::zero(SpatialScale::ZERO)
            .translated_at_scale(SpatialScale::ZERO, Vec3::X * x_metres)
            .unwrap();
        RadialGravitySource::new(
            center,
            1.0,
            SpatialScale::ZERO,
            surface_gravity,
        )
    }

    #[test]
    fn exact_reference_superposes_sources_without_hidden_dominance() {
        let mut world = World::new();
        let left = world.spawn_empty().id();
        let right = world.spawn_empty().id();
        let sample = exact_direct_sample(
            &UsfPosition::zero(SpatialScale::ZERO),
            [
                (left, source_at(-10.0, 4.0)),
                (right, source_at(10.0, 4.0)),
            ],
        );

        assert!(sample.acceleration_metres_per_second2().length() < 1.0e-12);
        assert_eq!(sample.evaluation(), GravityEvaluation::ExactDirect);
        assert_eq!(sample.evaluated_source_count(), 2);
    }
}
