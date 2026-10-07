//! Prepared sampling of the canonical volumetric celestial field.

use super::*;

#[derive(Debug)]
pub(crate) struct PreparedCelestialPresentationBody {
    body: CelestialFieldRealization,
}

impl PreparedCelestialPresentationBody {
    pub(crate) fn new(body: CelestialFieldRealization) -> Self {
        Self { body }
    }

    pub(crate) fn signed_distance_local_metres(&self, point: DVec3) -> Option<f64> {
        self.body.signed_distance_local_metres(point)
    }

    pub(crate) fn surface_local_metres(&self, direction: Vec3) -> Result<DVec3, UsfPositionError> {
        Ok(dvec(normalized_direction(direction)) * self.body.radius_metres)
    }
}
