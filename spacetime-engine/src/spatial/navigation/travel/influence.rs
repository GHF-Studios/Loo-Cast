//! Coarse semantic extent and bounded observer measurement.

use super::*;
use bevy::{math::DVec3, prelude::*};

/// How a spatial extent should affect long-distance travel.
#[derive(Debug, Clone, Copy)]
pub enum UsfTravelInfluenceKind {
    /// A body whose boundary is physically important: planet, star, asteroid,
    /// station, megastructure, etc. Approach speed is constrained by clearance
    /// to its surface and by the body's characteristic size.
    HardBody,
    /// A traversable volume such as a nebula, atmosphere, gas cloud or dust
    /// lane. Its outer boundary is not an obstacle; local structure inside the
    /// medium determines the useful reaction scale.
    Medium(UsfTravelMedium),
    /// A broad semantic/discovery region. Merely being inside a galaxy, cluster
    /// or belt must not directly clamp travel speed. Regions remain in the
    /// neighbourhood so later spatial-index/worldgen queries can refine them.
    Region,
}

/// Coarse local properties needed to navigate a traversable medium.
///
/// This is deliberately a tiny navigation projection of richer simulation
/// state. It can later be replaced by sampled fields without changing Cruise's
/// distinction between hard boundaries and traversable structure.
#[derive(Debug, Clone, Copy)]
pub struct UsfTravelMedium {
    characteristic_feature_size_native: f64,
    density: f32,
    turbulence: f32,
    hazard: f32,
}

impl UsfTravelMedium {
    pub fn new(
        characteristic_feature_size_native: f64,
        density: f32,
        turbulence: f32,
        hazard: f32,
    ) -> Self {
        assert!(
            characteristic_feature_size_native.is_finite()
                && characteristic_feature_size_native > 0.0
        );
        Self {
            characteristic_feature_size_native,
            density: density.clamp(0.0, 1.0),
            turbulence: turbulence.clamp(0.0, 1.0),
            hazard: hazard.clamp(0.0, 1.0),
        }
    }

    pub const fn characteristic_feature_size_native(self) -> f64 {
        self.characteristic_feature_size_native
    }

    pub const fn density(self) -> f32 {
        self.density
    }

    pub const fn turbulence(self) -> f32 {
        self.turbulence
    }

    pub const fn hazard(self) -> f32 {
        self.hazard
    }

    /// Dimensionless indication of how strongly the medium should reduce the
    /// speed allowed by its characteristic feature size.
    pub fn traversal_resistance(self) -> f64 {
        f64::from(self.density * 0.45 + self.turbulence * 0.35 + self.hazard * 0.20).clamp(0.0, 1.0)
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct UsfTravelInfluence {
    scale: SpatialScale,
    extent_radius_native: f64,
    kind: UsfTravelInfluenceKind,
}

impl UsfTravelInfluence {
    pub fn hard_body(scale: SpatialScale, radius_native: f64) -> Self {
        Self::with_kind(scale, radius_native, UsfTravelInfluenceKind::HardBody)
    }

    pub fn medium(
        scale: SpatialScale,
        extent_radius_native: f64,
        characteristic_feature_size_native: f64,
        density: f32,
        turbulence: f32,
        hazard: f32,
    ) -> Self {
        Self::with_kind(
            scale,
            extent_radius_native,
            UsfTravelInfluenceKind::Medium(UsfTravelMedium::new(
                characteristic_feature_size_native,
                density,
                turbulence,
                hazard,
            )),
        )
    }

    pub fn region(scale: SpatialScale, extent_radius_native: f64) -> Self {
        Self::with_kind(scale, extent_radius_native, UsfTravelInfluenceKind::Region)
    }

    fn with_kind(
        scale: SpatialScale,
        extent_radius_native: f64,
        kind: UsfTravelInfluenceKind,
    ) -> Self {
        assert!(extent_radius_native.is_finite() && extent_radius_native > 0.0);
        Self {
            scale,
            extent_radius_native,
            kind,
        }
    }

    pub const fn scale(self) -> SpatialScale {
        self.scale
    }
    pub const fn extent_radius_native(self) -> f64 {
        self.extent_radius_native
    }
    pub const fn kind(self) -> UsfTravelInfluenceKind {
        self.kind
    }

    fn characteristic_scale_native(self) -> f64 {
        match self.kind {
            UsfTravelInfluenceKind::HardBody | UsfTravelInfluenceKind::Region => {
                self.extent_radius_native
            }
            UsfTravelInfluenceKind::Medium(medium) => medium.characteristic_feature_size_native(),
        }
    }

    pub fn measure_from_at_scale(
        self,
        anchor: &UsfPosition,
        frame: UsfSemanticFrame,
        observer: &UsfPosition,
        measurement_scale: SpatialScale,
        boundary: Option<&UsfTravelBoundaryResolver>,
    ) -> Option<UsfTravelInfluenceMeasure> {
        const RELATIVE_BOUND_NATIVE: f32 = 1_000_000.0;

        let relative = observer
            .relative_at_scale_bounded(anchor, self.scale, RELATIVE_BOUND_NATIVE)
            .ok()?;
        let center_distance_native = (f64::from(relative.x).powi(2)
            + f64::from(relative.y).powi(2)
            + f64::from(relative.z).powi(2))
        .sqrt();

        let to_scale0 = self.scale.scale0_units_per_native();
        let center_distance_scale0 = center_distance_native * to_scale0;
        let extent_radius_scale0 = self.extent_radius_native * to_scale0;
        let characteristic_scale0 = self.characteristic_scale_native() * to_scale0;
        let spherical_signed_boundary_scale0 =
            (center_distance_native - self.extent_radius_native) * to_scale0;

        let signed_boundary_scale0 = if matches!(self.kind, UsfTravelInfluenceKind::HardBody) {
            boundary
                .and_then(|resolver| {
                    resolver.sample_near(anchor, frame, observer, measurement_scale)
                })
                .and_then(|sample| {
                    let relative = observer
                        .relative_at_scale_bounded_f64(&sample.surface(), sample.scale(), f64::MAX)
                        .ok()?;
                    let outward = sample.outward();
                    let outward = DVec3::new(
                        f64::from(outward.x),
                        f64::from(outward.y),
                        f64::from(outward.z),
                    );
                    let signed_native = relative.dot(outward);
                    let signed_scale0 = signed_native * sample.scale().scale0_units_per_native();
                    signed_scale0.is_finite().then_some(signed_scale0)
                })
                .unwrap_or(spherical_signed_boundary_scale0)
        } else {
            spherical_signed_boundary_scale0
        };

        let boundary_clearance_scale0 = signed_boundary_scale0.max(0.0);
        let penetration_depth_scale0 = (-signed_boundary_scale0).max(0.0);

        if !center_distance_scale0.is_finite()
            || !extent_radius_scale0.is_finite()
            || !characteristic_scale0.is_finite()
            || !boundary_clearance_scale0.is_finite()
            || !penetration_depth_scale0.is_finite()
            || extent_radius_scale0 <= 0.0
            || characteristic_scale0 <= 0.0
        {
            return None;
        }

        Some(UsfTravelInfluenceMeasure {
            center_distance_scale0,
            boundary_clearance_scale0,
            penetration_depth_scale0,
            extent_radius_scale0,
            characteristic_scale0,
            inside: signed_boundary_scale0 <= 0.0,
        })
    }

    pub fn measure_from(
        self,
        anchor: &UsfPosition,
        observer: &UsfPosition,
    ) -> Option<UsfTravelInfluenceMeasure> {
        self.measure_from_at_scale(
            anchor,
            UsfSemanticFrame::identity(),
            observer,
            self.scale,
            None,
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UsfTravelInfluenceMeasure {
    center_distance_scale0: f64,
    boundary_clearance_scale0: f64,
    penetration_depth_scale0: f64,
    extent_radius_scale0: f64,
    characteristic_scale0: f64,
    inside: bool,
}

impl UsfTravelInfluenceMeasure {
    pub const fn center_distance_scale0(self) -> f64 {
        self.center_distance_scale0
    }

    pub const fn boundary_clearance_scale0(self) -> f64 {
        self.boundary_clearance_scale0
    }

    pub const fn penetration_depth_scale0(self) -> f64 {
        self.penetration_depth_scale0
    }

    pub const fn extent_radius_scale0(self) -> f64 {
        self.extent_radius_scale0
    }

    pub const fn characteristic_scale0(self) -> f64 {
        self.characteristic_scale0
    }

    pub const fn inside(self) -> bool {
        self.inside
    }

    /// Distance to the influence boundary measured in units of the local
    /// feature scale that actually matters for navigation.
    pub fn relative_proximity(self) -> f64 {
        self.boundary_clearance_scale0 / self.characteristic_scale0
    }
}
