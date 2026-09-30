//! Semantic-scale descriptors for planetary terrain morphology.
//!
//! A band is introduced at one ordinary USF Scale and remains part of terrain
//! truth at every finer Scale. This is semantic refinement, not mesh LOD:
//! asking for a finer Scale adds residual terrain instead of replacing the
//! coarser surface with an unrelated noise field.

use crate::spatial::SpatialScale;

/// One deterministic planetary terrain band.
///
/// `introduced_at` is the coarsest Scale at which this band belongs to semantic
/// terrain truth. `amplitude_metres` is deliberately SI-valued so representation
/// chart size cannot silently change morphology.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct PlanetaryTerrainBand {
    introduced_at: SpatialScale,
    amplitude_metres: f64,
    angular_frequency: f32,
    seed_salt: u32,
}

impl PlanetaryTerrainBand {
    pub(super) fn new(
        introduced_at: SpatialScale,
        amplitude_metres: f64,
        angular_frequency: f32,
        seed_salt: u32,
    ) -> Self {
        assert!(
            amplitude_metres.is_finite() && amplitude_metres >= 0.0,
            "planetary terrain-band amplitude must be finite and non-negative"
        );
        assert!(
            angular_frequency.is_finite() && angular_frequency > 0.0,
            "planetary terrain-band frequency must be finite and positive"
        );
        Self {
            introduced_at,
            amplitude_metres,
            angular_frequency,
            seed_salt,
        }
    }

    /// True when semantic terrain resolved through `through_scale` owns this
    /// band. Lower/finer Scale exponents accumulate every coarser band.
    pub(super) fn contributes_through(self, through_scale: SpatialScale) -> bool {
        through_scale <= self.introduced_at
    }

    pub(super) const fn introduced_at(self) -> SpatialScale {
        self.introduced_at
    }

    pub(super) const fn amplitude_metres(self) -> f64 {
        self.amplitude_metres
    }

    pub(super) const fn angular_frequency(self) -> f32 {
        self.angular_frequency
    }

    pub(super) const fn seed_salt(self) -> u32 {
        self.seed_salt
    }
}
