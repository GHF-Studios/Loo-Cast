//! Spatial influences used by long-distance navigation.
//!
//! These are neither colliders nor presentation LODs. They expose semantic
//! spatial structure to travel systems without pretending that every large
//! thing is an impenetrable sphere.

use bevy::{math::DVec3, prelude::*};

use super::{SpatialScale, UsfScaleLayerFrames};

const NEIGHBORHOOD_HARD_BY_RELATIVE_PROXIMITY: usize = 6;
const NEIGHBORHOOD_HARD_BY_ABSOLUTE_PROXIMITY: usize = 4;
const NEIGHBORHOOD_MEDIUM_BY_RELATIVE_PROXIMITY: usize = 6;
const NEIGHBORHOOD_MEDIUM_BY_ABSOLUTE_PROXIMITY: usize = 4;
const NEIGHBORHOOD_REGION_BY_RELATIVE_PROXIMITY: usize = 4;
const NEIGHBORHOOD_MAX_AGE_SECONDS: f32 = 0.5;
const NEIGHBORHOOD_PROXIMITY_REFRESH_FRACTION: f64 = 0.10;
const NEIGHBORHOOD_FEATURE_REFRESH_FRACTION: f64 = 0.05;
const NEIGHBORHOOD_MIN_REFRESH_DISTANCE_SCALE0: f64 = 1.0;

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
        f64::from(
            self.density * 0.45 + self.turbulence * 0.35 + self.hazard * 0.20,
        )
        .clamp(0.0, 1.0)
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct UsfTravelInfluence {
    absolute: DVec3,
    scale: SpatialScale,
    extent_radius_native: f64,
    kind: UsfTravelInfluenceKind,
}

impl UsfTravelInfluence {
    pub fn hard_body(absolute: DVec3, scale: SpatialScale, radius_native: f64) -> Self {
        Self::with_kind(
            absolute,
            scale,
            radius_native,
            UsfTravelInfluenceKind::HardBody,
        )
    }

    pub fn medium(
        absolute: DVec3,
        scale: SpatialScale,
        extent_radius_native: f64,
        characteristic_feature_size_native: f64,
        density: f32,
        turbulence: f32,
        hazard: f32,
    ) -> Self {
        Self::with_kind(
            absolute,
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

    pub fn region(absolute: DVec3, scale: SpatialScale, extent_radius_native: f64) -> Self {
        Self::with_kind(
            absolute,
            scale,
            extent_radius_native,
            UsfTravelInfluenceKind::Region,
        )
    }

    fn with_kind(
        absolute: DVec3,
        scale: SpatialScale,
        extent_radius_native: f64,
        kind: UsfTravelInfluenceKind,
    ) -> Self {
        assert!(extent_radius_native.is_finite() && extent_radius_native > 0.0);
        Self {
            absolute,
            scale,
            extent_radius_native,
            kind,
        }
    }

    pub const fn absolute(self) -> DVec3 {
        self.absolute
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
            UsfTravelInfluenceKind::Medium(medium) => {
                medium.characteristic_feature_size_native()
            }
        }
    }

    /// Measures this influence from an observer without flattening either point
    /// into one universe-wide float chart first.
    pub fn measure_from(
        self,
        observer_absolute: DVec3,
        observer_scale: SpatialScale,
        frames: &UsfScaleLayerFrames,
    ) -> Option<UsfTravelInfluenceMeasure> {
        let observer_in_scale =
            frames.convert_absolute(observer_absolute, observer_scale, self.scale);
        let center_distance_native = (self.absolute - observer_in_scale).length();
        if !center_distance_native.is_finite() {
            return None;
        }

        let to_scale0 = 10.0_f64.powi(self.scale.exponent() as i32);
        let center_distance_scale0 = center_distance_native * to_scale0;
        let extent_radius_scale0 = self.extent_radius_native * to_scale0;
        let characteristic_scale0 = self.characteristic_scale_native() * to_scale0;
        let signed_boundary_native = center_distance_native - self.extent_radius_native;
        let boundary_clearance_scale0 = signed_boundary_native.max(0.0) * to_scale0;
        let penetration_depth_scale0 = (-signed_boundary_native).max(0.0) * to_scale0;

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
            inside: signed_boundary_native <= 0.0,
        })
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

#[derive(Debug, Clone, Copy)]
struct CachedTravelInfluence {
    entity: Entity,
    influence: UsfTravelInfluence,
}

#[derive(Debug, Clone, Copy)]
struct TravelInfluenceCandidate {
    cached: CachedTravelInfluence,
    measurement: UsfTravelInfluenceMeasure,
}

#[derive(Debug, Clone, Copy)]
enum CandidateOrder {
    Absolute,
    Relative,
}

fn append_nearest(
    destination: &mut Vec<CachedTravelInfluence>,
    candidates: &[TravelInfluenceCandidate],
    limit: usize,
    order: CandidateOrder,
) {
    let mut sorted = candidates.to_vec();
    sorted.sort_by(|a, b| match order {
        CandidateOrder::Absolute => a
            .measurement
            .boundary_clearance_scale0()
            .total_cmp(&b.measurement.boundary_clearance_scale0()),
        CandidateOrder::Relative => a
            .measurement
            .relative_proximity()
            .total_cmp(&b.measurement.relative_proximity()),
    });

    for candidate in sorted.into_iter().take(limit) {
        if destination
            .iter()
            .any(|cached| cached.entity == candidate.cached.entity)
        {
            continue;
        }
        destination.push(candidate.cached);
    }
}

/// Small observer-local cache of the travel influences most likely to matter.
///
/// It deliberately keeps two notions of "near": absolute boundary proximity
/// catches small nearby bodies/volumes, while proximity in characteristic
/// feature scales keeps enormous structures relevant without making their
/// enclosing radius itself a speed limit.
///
/// Refresh currently scans the available influence components only when the
/// cache expires or the observer moves materially. A future spatial index can
/// replace that refresh source without changing consumers of this component.
#[derive(Component, Debug, Default)]
pub struct UsfTravelNeighborhood {
    sampled_absolute: Option<DVec3>,
    sampled_scale: Option<SpatialScale>,
    refresh_distance_scale0: f64,
    age_seconds: f32,
    influences: Vec<CachedTravelInfluence>,
}

impl UsfTravelNeighborhood {
    pub fn len(&self) -> usize {
        self.influences.len()
    }

    pub fn is_empty(&self) -> bool {
        self.influences.is_empty()
    }

    pub fn advance(&mut self, dt_seconds: f32) {
        self.age_seconds += dt_seconds.max(0.0);
    }

    pub fn needs_refresh(
        &self,
        observer_absolute: DVec3,
        observer_scale: SpatialScale,
    ) -> bool {
        let (Some(sampled_absolute), Some(sampled_scale)) =
            (self.sampled_absolute, self.sampled_scale)
        else {
            return true;
        };

        if sampled_scale != observer_scale || self.age_seconds >= NEIGHBORHOOD_MAX_AGE_SECONDS {
            return true;
        }

        let moved_native = (observer_absolute - sampled_absolute).length();
        if !moved_native.is_finite() {
            return true;
        }
        let moved_scale0 = moved_native * 10.0_f64.powi(observer_scale.exponent() as i32);
        moved_scale0 >= self.refresh_distance_scale0
    }

    pub fn refresh<I>(
        &mut self,
        observer_absolute: DVec3,
        observer_scale: SpatialScale,
        frames: &UsfScaleLayerFrames,
        influences: I,
    ) where
        I: IntoIterator<Item = (Entity, UsfTravelInfluence)>,
    {
        let mut candidates = influences
            .into_iter()
            .filter_map(|(entity, influence)| {
                influence
                    .measure_from(observer_absolute, observer_scale, frames)
                    .map(|measurement| TravelInfluenceCandidate {
                        cached: CachedTravelInfluence { entity, influence },
                        measurement,
                    })
            })
            .collect::<Vec<_>>();

        let nearest = candidates
            .iter()
            .filter(|candidate| {
                !matches!(
                    candidate.cached.influence.kind(),
                    UsfTravelInfluenceKind::Region
                )
            })
            .min_by(|a, b| {
                a.measurement
                    .boundary_clearance_scale0()
                    .total_cmp(&b.measurement.boundary_clearance_scale0())
            })
            .copied()
            .or_else(|| {
                candidates
                    .iter()
                    .min_by(|a, b| {
                        a.measurement
                            .boundary_clearance_scale0()
                            .total_cmp(&b.measurement.boundary_clearance_scale0())
                    })
                    .copied()
            });

        let hard = candidates
            .iter()
            .copied()
            .filter(|candidate| {
                matches!(
                    candidate.cached.influence.kind(),
                    UsfTravelInfluenceKind::HardBody
                )
            })
            .collect::<Vec<_>>();
        let media = candidates
            .iter()
            .copied()
            .filter(|candidate| {
                matches!(
                    candidate.cached.influence.kind(),
                    UsfTravelInfluenceKind::Medium(_)
                )
            })
            .collect::<Vec<_>>();
        let regions = candidates
            .drain(..)
            .filter(|candidate| {
                matches!(
                    candidate.cached.influence.kind(),
                    UsfTravelInfluenceKind::Region
                )
            })
            .collect::<Vec<_>>();

        self.influences.clear();
        append_nearest(
            &mut self.influences,
            &hard,
            NEIGHBORHOOD_HARD_BY_RELATIVE_PROXIMITY,
            CandidateOrder::Relative,
        );
        append_nearest(
            &mut self.influences,
            &hard,
            NEIGHBORHOOD_HARD_BY_ABSOLUTE_PROXIMITY,
            CandidateOrder::Absolute,
        );
        append_nearest(
            &mut self.influences,
            &media,
            NEIGHBORHOOD_MEDIUM_BY_RELATIVE_PROXIMITY,
            CandidateOrder::Relative,
        );
        append_nearest(
            &mut self.influences,
            &media,
            NEIGHBORHOOD_MEDIUM_BY_ABSOLUTE_PROXIMITY,
            CandidateOrder::Absolute,
        );
        append_nearest(
            &mut self.influences,
            &regions,
            NEIGHBORHOOD_REGION_BY_RELATIVE_PROXIMITY,
            CandidateOrder::Relative,
        );

        self.refresh_distance_scale0 = nearest.map_or(f64::INFINITY, |nearest| {
            (nearest.measurement.boundary_clearance_scale0()
                * NEIGHBORHOOD_PROXIMITY_REFRESH_FRACTION)
                .max(
                    nearest.measurement.characteristic_scale0()
                        * NEIGHBORHOOD_FEATURE_REFRESH_FRACTION,
                )
                .max(NEIGHBORHOOD_MIN_REFRESH_DISTANCE_SCALE0)
        });
        self.sampled_absolute = Some(observer_absolute);
        self.sampled_scale = Some(observer_scale);
        self.age_seconds = 0.0;
    }

    pub fn influences(&self) -> impl Iterator<Item = UsfTravelInfluence> + '_ {
        self.influences.iter().map(|cached| cached.influence)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn medium_resistance_is_bounded() {
        let medium = UsfTravelMedium::new(4.0, 1.5, -2.0, 0.5);
        assert_eq!(medium.density(), 1.0);
        assert_eq!(medium.turbulence(), 0.0);
        assert_eq!(medium.hazard(), 0.5);
        assert!((0.0..=1.0).contains(&medium.traversal_resistance()));
    }

    #[test]
    fn constructors_make_semantics_explicit() {
        let scale = SpatialScale::ZERO;
        let hard = UsfTravelInfluence::hard_body(DVec3::ZERO, scale, 2.0);
        let medium = UsfTravelInfluence::medium(DVec3::ZERO, scale, 8.0, 1.0, 0.4, 0.2, 0.1);
        let region = UsfTravelInfluence::region(DVec3::ZERO, scale, 20.0);

        assert!(matches!(hard.kind(), UsfTravelInfluenceKind::HardBody));
        assert!(matches!(medium.kind(), UsfTravelInfluenceKind::Medium(_)));
        assert!(matches!(region.kind(), UsfTravelInfluenceKind::Region));
    }
}
