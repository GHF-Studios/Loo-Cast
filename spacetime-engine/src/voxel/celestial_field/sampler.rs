//! Prepared presentation adapter over one canonical celestial field.

use bevy::{math::DVec3, prelude::*};

use crate::spatial::UsfPositionError;
use crate::voxel::base::PreparedCelestialPresentationBody;

//
// Presentation resolution is a sampling/aggregation choice, not semantic
// terrain bandwidth. Every binary LOD samples the same final canonical
// volumetric field. A future filtered/aggregated representation may reduce
// aliasing, but it must approximate this same field rather than deleting
// semantic terrain bands or caves at arbitrary sample-spacing thresholds.
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
    pub(crate) fn outer_signed_distance_local_metres(
        &self,
        local_point_metres: DVec3,
    ) -> Option<f64> {
        self.body
            .outer_signed_distance_local_metres(local_point_metres)
    }

    #[inline]
    pub(crate) fn surface_local_metres(&self, direction: Vec3) -> Result<DVec3, UsfPositionError> {
        self.body.surface_local_metres(direction)
    }

    #[inline]
    pub(crate) fn pre_fine_surface_local_metres(
        &self,
        direction: Vec3,
    ) -> Result<DVec3, UsfPositionError> {
        self.body.pre_fine_surface_local_metres(direction)
    }

    pub(crate) fn semantic_noise_cache_stats(&self) -> (u64, u64) {
        self.body.noise_cache_stats()
    }

    pub(crate) fn semantic_noise_cell_cache_stats(&self) -> (u64, u64) {
        self.body.noise_cell_cache_stats()
    }
}
