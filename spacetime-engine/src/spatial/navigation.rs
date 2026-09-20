//! Spatial influences used by long-distance navigation.
//!
//! These are not colliders and not presentation LODs. They expose only enough
//! semantic geometry for travel systems to reason about safe approach speed.

use bevy::{math::DVec3, prelude::*};

use super::SpatialScale;

#[derive(Component, Debug, Clone, Copy)]
pub struct UsfTravelInfluence {
    absolute: DVec3,
    scale: SpatialScale,
    radius_native: f64,
}

impl UsfTravelInfluence {
    pub fn new(absolute: DVec3, scale: SpatialScale, radius_native: f64) -> Self {
        assert!(radius_native.is_finite() && radius_native > 0.0);
        Self { absolute, scale, radius_native }
    }

    pub const fn absolute(self) -> DVec3 { self.absolute }
    pub const fn scale(self) -> SpatialScale { self.scale }
    pub const fn radius_native(self) -> f64 { self.radius_native }
}
