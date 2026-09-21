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

const NAVIGATION_TARGET_TRAVERSAL_SECONDS: f64 = 20.0;
const NAVIGATION_LOCAL_STRUCTURE_RADIUS_LIMIT: f64 = 12.0;

/// What kind of semantic structure currently sets coarse manual travel pace.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsfNavigationContextKind {
    Fallback,
    HardBody,
    Medium,
    Region,
}

impl UsfNavigationContextKind {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Fallback => "fallback",
            Self::HardBody => "body",
            Self::Medium => "medium",
            Self::Region => "region",
        }
    }
}

/// Marks a travel influence whose approach should automatically refine the
/// observer/runtime chart. Moon-only for the first proof.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct UsfApproachRefinement {
    minimum_scale: SpatialScale,
}

impl UsfApproachRefinement {
    pub const fn new(minimum_scale: SpatialScale) -> Self { Self { minimum_scale } }
    pub const fn minimum_scale(self) -> SpatialScale { self.minimum_scale }
}

/// Observer-local navigation scale derived from semantic spatial structure.
///
/// This is not a physics state and it is not a presentation LOD. It answers:
/// "what spatial length are we currently navigating?" Coarse manual movement
/// uses that length to choose a useful pace, while S0 character movement keeps
/// its ordinary physical locomotion.
///
/// The first implementation deliberately derives context only from travel
/// influences already available in [`UsfTravelNeighborhood`]. Future terrain,
/// topology and worldgen realizers can contribute finer context without changing
/// the consumer contract.
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct UsfNavigationContext {
    kind: UsfNavigationContextKind,
    interaction_scale: SpatialScale,
    source_scale: Option<SpatialScale>,
    characteristic_length_scale0: f64,
}

impl Default for UsfNavigationContext {
    fn default() -> Self {
        Self::fallback(SpatialScale::MAX)
    }
}

impl UsfNavigationContext {
    pub fn fallback(interaction_scale: SpatialScale) -> Self {
        let scale0_per_native = 10.0_f64.powi(interaction_scale.exponent() as i32);
        Self {
            kind: UsfNavigationContextKind::Fallback,
            interaction_scale,
            source_scale: None,
            characteristic_length_scale0:
                scale0_per_native * NAVIGATION_TARGET_TRAVERSAL_SECONDS,
        }
    }

    pub fn resolve(
        observer_absolute: DVec3,
        observer_scale: SpatialScale,
        frames: &UsfScaleLayerFrames,
        neighborhood: &UsfTravelNeighborhood,
    ) -> Self {
        #[derive(Clone, Copy)]
        struct Candidate {
            kind: UsfNavigationContextKind,
            source_scale: SpatialScale,
            relative_proximity: f64,
            characteristic_length_scale0: f64,
        }

        let mut local = None::<Candidate>;
        let mut region = None::<Candidate>;
        let mut any_structure = None::<Candidate>;

        for influence in neighborhood.influences() {
            let Some(measurement) =
                influence.measure_from(observer_absolute, observer_scale, frames)
            else {
                continue;
            };

            let kind = match influence.kind() {
                UsfTravelInfluenceKind::HardBody => UsfNavigationContextKind::HardBody,
                UsfTravelInfluenceKind::Medium(_) => UsfNavigationContextKind::Medium,
                UsfTravelInfluenceKind::Region => UsfNavigationContextKind::Region,
            };

            let characteristic_length_scale0 =
                navigation_length_scale0(influence.kind(), measurement);
            if !characteristic_length_scale0.is_finite()
                || characteristic_length_scale0 <= 0.0
            {
                continue;
            }

            let candidate = Candidate {
                kind,
                source_scale: influence.scale(),
                relative_proximity: measurement.relative_proximity(),
                characteristic_length_scale0,
            };

            let should_replace = |current: Option<Candidate>| {
                current.is_none_or(|current| {
                    candidate.relative_proximity < current.relative_proximity
                })
            };

            if should_replace(any_structure) {
                any_structure = Some(candidate);
            }

            match kind {
                UsfNavigationContextKind::HardBody | UsfNavigationContextKind::Medium
                    if measurement.inside()
                        || measurement.relative_proximity()
                            <= NAVIGATION_LOCAL_STRUCTURE_RADIUS_LIMIT =>
                {
                    if should_replace(local) {
                        local = Some(candidate);
                    }
                }
                UsfNavigationContextKind::Region => {
                    if should_replace(region) {
                        region = Some(candidate);
                    }
                }
                _ => {}
            }
        }

        let selected = local.or(region).or(any_structure);
        let Some(selected) = selected else {
            return Self::fallback(observer_scale);
        };

        Self {
            kind: selected.kind,
            interaction_scale: observer_scale,
            source_scale: Some(selected.source_scale),
            characteristic_length_scale0: selected.characteristic_length_scale0,
        }
    }

    pub const fn kind(self) -> UsfNavigationContextKind {
        self.kind
    }

    pub const fn interaction_scale(self) -> SpatialScale {
        self.interaction_scale
    }

    pub const fn source_scale(self) -> Option<SpatialScale> {
        self.source_scale
    }

    pub const fn characteristic_length_scale0(self) -> f64 {
        self.characteristic_length_scale0
    }

    pub fn characteristic_length_native(self, scale: SpatialScale) -> f64 {
        self.characteristic_length_scale0
            / 10.0_f64.powi(scale.exponent() as i32)
    }

    /// Baseline coarse manual speed for this navigation context.
    ///
    /// A characteristic length should take roughly twenty seconds to traverse.
    /// User `speed` remains a dimensionless multiplier applied on top.
    pub fn manual_native_units_per_second(self, scale: SpatialScale) -> f32 {
        let native = self.characteristic_length_native(scale)
            / NAVIGATION_TARGET_TRAVERSAL_SECONDS;
        native.clamp(0.0, f32::MAX as f64) as f32
    }
}

fn navigation_length_scale0(
    kind: UsfTravelInfluenceKind,
    measurement: UsfTravelInfluenceMeasure,
) -> f64 {
    match kind {
        UsfTravelInfluenceKind::HardBody => {
            measurement.boundary_clearance_scale0().max(1.0)
        }
        UsfTravelInfluenceKind::Medium(_) => measurement
            .boundary_clearance_scale0()
            .max(measurement.characteristic_scale0()),
        UsfTravelInfluenceKind::Region => measurement
            .boundary_clearance_scale0()
            .max(measurement.extent_radius_scale0() * 2.0),
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
    fn navigation_fallback_is_one_native_unit_per_second() {
        let scale = SpatialScale::new(18).unwrap();
        let context = UsfNavigationContext::fallback(scale);
        assert!((context.manual_native_units_per_second(scale) - 1.0).abs() < 1.0e-6);
    }

    #[test]
    fn navigation_length_uses_approach_distance_until_object_scale_takes_over() {
        let measure = UsfTravelInfluenceMeasure {
            center_distance_scale0: 160.0,
            boundary_clearance_scale0: 143.0,
            penetration_depth_scale0: 0.0,
            extent_radius_scale0: 17.0,
            characteristic_scale0: 17.0,
            inside: false,
        };
        assert_eq!(
            navigation_length_scale0(UsfTravelInfluenceKind::HardBody, measure),
            143.0,
        );

        let close = UsfTravelInfluenceMeasure {
            boundary_clearance_scale0: 2.0,
            ..measure
        };
        assert_eq!(
            navigation_length_scale0(UsfTravelInfluenceKind::HardBody, close),
            34.0,
        );
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
