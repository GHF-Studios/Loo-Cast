//! Typed gravity-field query and exact reference evaluator.
//!
//! Consumers ask for physical acceleration at a canonical [`UsfPosition`].
//! They do not depend on whether the production backend is eventually direct,
//! hierarchical, multipole-based, grid-backed or hybrid.
//!
//! The exact evaluator is the reference oracle for any future approximation.

use bevy::{ecs::system::SystemParam, math::DVec3, prelude::*};

use crate::spatial::UsfPosition;

use super::RadialGravitySource;

/// Fixed-tick physical gravity sample attached to a runtime subject.
///
/// Acceleration is expressed in canonical USF axes and SI m/s².
/// `strongest_source` is diagnostic/reference-frame information only; physical
/// acceleration is the vector sum of every evaluated source.
#[derive(Component, Debug, Clone, Copy)]
pub struct GravitySample {
    acceleration_metres_per_second2: DVec3,
    strongest_source: Option<Entity>,
    evaluated_source_count: usize,
}

impl Default for GravitySample {
    fn default() -> Self {
        Self {
            acceleration_metres_per_second2: DVec3::ZERO,
            strongest_source: None,
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
/// Samples every authored source analytically and sums the contributions.
#[derive(SystemParam)]
pub struct GravityFieldQuery<'w, 's> {
    sources: Query<'w, 's, (Entity, &'static UsfPosition, &'static RadialGravitySource)>,
}

impl GravityFieldQuery<'_, '_> {
    pub fn sample(&self, position: &UsfPosition) -> GravitySample {
        exact_direct_sample(
            position,
            self.sources
                .iter()
                .map(|(entity, center, source)| (entity, *center, *source)),
        )
    }
}

fn exact_direct_sample(
    position: &UsfPosition,
    sources: impl IntoIterator<Item = (Entity, UsfPosition, RadialGravitySource)>,
) -> GravitySample {
    let mut acceleration = DVec3::ZERO;
    let mut strongest_source = None;
    let mut strongest_magnitude2 = 0.0_f64;
    let mut evaluated_source_count = 0usize;

    for (entity, center, source) in sources {
        evaluated_source_count += 1;

        let Some(contribution) = source.acceleration_at(&center, position) else {
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
        evaluated_source_count,
    }
}
