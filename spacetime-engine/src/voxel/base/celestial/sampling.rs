//! Prepared sampling of the canonical volumetric celestial field.

use super::*;

#[derive(Debug)]
pub(crate) struct PreparedCelestialBodySampler {
    body: CelestialFieldRealization,
}

impl PreparedCelestialBodySampler {
    pub(crate) fn new(body: CelestialFieldRealization) -> Self {
        Self { body }
    }

    pub(crate) fn signed_distance_local_metres(&self, point: DVec3) -> Option<f64> {
        self.body.signed_distance_local_metres(point)
    }
}
