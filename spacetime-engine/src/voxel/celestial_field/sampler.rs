//! Prepared presentation adapter over one canonical celestial field.

use bevy::{math::DVec3, prelude::*};

use crate::spatial::UsfPositionError;
use crate::voxel::base::PreparedCelestialPresentationBody;

// Presentation resolution is a sampling/aggregation choice. Every binary LOD
// samples the same canonical volumetric field.
#[derive(Debug)]
pub(crate) struct CelestialPresentationFieldSampler {
    pub(super) body: PreparedCelestialPresentationBody,
}

impl CelestialPresentationFieldSampler {
    #[inline]
    pub(crate) fn signed_distance_local_metres(&self, local_point_metres: DVec3) -> Option<f64> {
        self.body.signed_distance_local_metres(local_point_metres)
    }

    #[inline]
    pub(crate) fn surface_local_metres(&self, direction: Vec3) -> Result<DVec3, UsfPositionError> {
        self.body.surface_local_metres(direction)
    }
}
