//! Typed exact gravity-field query.
//!
//! The query boundary is semantic and stable: consumers ask for physical
//! acceleration at a canonical USF position. The current backend evaluates the
//! authored sources directly. Representation/approximation machinery will be
//! introduced only when concrete scale/performance pressure defines its error
//! contract.

use bevy::{
    ecs::system::SystemParam,
    math::DVec3,
    prelude::*,
};

use crate::spatial::UsfPosition;

use super::RadialGravitySource;

/// Fixed-tick physical gravity sample attached to a runtime subject.
///
/// Acceleration is expressed in canonical USF axes and SI m/s². `strongest_source`
/// is diagnostic/reference-frame information only; the acceleration is the sum
/// of every contributing source.
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

/// Typed gravity query. Consumers do not know whether gravity is eventually
/// evaluated analytically, from cached context summaries, grids or another
/// representation.
#[derive(SystemParam)]
pub struct GravityFieldQuery<'w, 's> {
    sources: Query<'w, 's, (Entity, &'static RadialGravitySource)>,
}

impl GravityFieldQuery<'_, '_> {
    pub fn sample(&self, position: &UsfPosition) -> GravitySample {
        let mut acceleration = DVec3::ZERO;
        let mut strongest_source = None;
        let mut strongest_magnitude2 = 0.0_f64;

        for (entity, source) in &self.sources {
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
        }
    }
}
