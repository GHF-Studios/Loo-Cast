//! Spatial influences used by long-distance navigation.
//!
//! These are neither colliders nor presentation LODs. They expose semantic
//! spatial structure to travel systems without pretending that every large
//! thing is an impenetrable sphere.

use std::{fmt, sync::Arc};

use bevy::{math::DVec3, prelude::*};

use super::{SpatialScale, UsfPosition, UsfSemanticFrame};

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

const NAVIGATION_FALLBACK_CHARACTERISTIC_METRES: f64 = 10_000.0;
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

/// Marks a travel influence whose realized structure can take responsibility
/// at finer Scale Slices during approach.
///
/// This is a realization-capability contract, not body-specific gameplay data.
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
        Self {
            kind: UsfNavigationContextKind::Fallback,
            interaction_scale,
            source_scale: None,
            characteristic_length_scale0: NAVIGATION_FALLBACK_CHARACTERISTIC_METRES,
        }
    }

    pub fn resolve(
        observer: &UsfPosition,
        observer_scale: SpatialScale,
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

        for (_, _, influence, measurement) in
            neighborhood.measurements_from(observer, observer_scale)
        {
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

/// One scale-local sample of a semantic hard-body boundary.
///
/// This is navigation geometry only. It owns neither collision, presentation,
/// residency nor canonical body identity.
#[derive(Debug, Clone, Copy)]
pub struct UsfTravelBoundarySample {
    surface: UsfPosition,
    outward: Vec3,
    scale: SpatialScale,
}

impl UsfTravelBoundarySample {
    pub fn new(surface: UsfPosition, outward: Vec3, scale: SpatialScale) -> Option<Self> {
        let outward = outward.normalize_or_zero();
        (outward != Vec3::ZERO).then_some(Self {
            surface,
            outward,
            scale,
        })
    }

    pub const fn surface(self) -> UsfPosition { self.surface }
    pub const fn outward(self) -> Vec3 { self.outward }
    pub const fn scale(self) -> SpatialScale { self.scale }
}

/// Capability adapter for refinable semantic hard-body boundaries.
///
/// The coarse `UsfTravelInfluence` sphere remains the far-field fallback.
/// Capability-specific code can provide an actual semantic surface without
/// making the generic spatial/navigation layer depend on that capability.
pub trait UsfTravelBoundary: fmt::Debug + Send + Sync + 'static {
    fn sample_near(
        &self,
        body_origin: &UsfPosition,
        body_frame: UsfSemanticFrame,
        observer: &UsfPosition,
        scale: SpatialScale,
    ) -> Option<UsfTravelBoundarySample>;
}

#[derive(Component, Clone)]
pub struct UsfTravelBoundaryResolver {
    provider: Arc<dyn UsfTravelBoundary>,
}

impl fmt::Debug for UsfTravelBoundaryResolver {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("UsfTravelBoundaryResolver")
            .finish_non_exhaustive()
    }
}

impl UsfTravelBoundaryResolver {
    pub fn new<T: UsfTravelBoundary>(provider: T) -> Self {
        Self { provider: Arc::new(provider) }
    }

    pub fn sample_near(
        &self,
        body_origin: &UsfPosition,
        body_frame: UsfSemanticFrame,
        observer: &UsfPosition,
        scale: SpatialScale,
    ) -> Option<UsfTravelBoundarySample> {
        self.provider.sample_near(body_origin, body_frame, observer, scale)
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
        Self { scale, extent_radius_native, kind }
    }

    pub const fn scale(self) -> SpatialScale { self.scale }
    pub const fn extent_radius_native(self) -> f64 { self.extent_radius_native }
    pub const fn kind(self) -> UsfTravelInfluenceKind { self.kind }

    fn characteristic_scale_native(self) -> f64 {
        match self.kind {
            UsfTravelInfluenceKind::HardBody | UsfTravelInfluenceKind::Region => self.extent_radius_native,
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
                        .relative_at_scale_bounded_f64(
                            &sample.surface(),
                            sample.scale(),
                            f64::MAX,
                        )
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

#[derive(Debug, Clone)]
struct CachedTravelInfluence {
    entity: Entity,
    anchor: UsfPosition,
    frame: UsfSemanticFrame,
    influence: UsfTravelInfluence,
    boundary: Option<UsfTravelBoundaryResolver>,
}

#[derive(Debug, Clone)]
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
    sampled_position: Option<UsfPosition>,
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
        observer: &UsfPosition,
        observer_scale: SpatialScale,
    ) -> bool {
        let (Some(sampled), Some(sampled_scale)) =
            (self.sampled_position, self.sampled_scale)
        else {
            return true;
        };

        if sampled_scale != observer_scale || self.age_seconds >= NEIGHBORHOOD_MAX_AGE_SECONDS {
            return true;
        }

        // "No structure was present when sampled" is not stable world knowledge:
        // world/bootstrap streaming may publish influences after this cache was
        // first evaluated. Keep probing until at least one influence exists.
        if self.influences.is_empty() {
            return true;
        }

        if !self.refresh_distance_scale0.is_finite() {
            return false;
        }

        let bound_native = (self.refresh_distance_scale0
            / observer_scale.scale0_units_per_native())
            .max(1.0)
            .min(f64::from(f32::MAX)) as f32;
        let Ok(delta) =
            observer.relative_at_scale_bounded(&sampled, observer_scale, bound_native)
        else {
            return true;
        };
        let moved_scale0 =
            f64::from(delta.length()) * observer_scale.scale0_units_per_native();
        moved_scale0 >= self.refresh_distance_scale0
    }

    pub fn refresh<I>(
        &mut self,
        observer: UsfPosition,
        observer_scale: SpatialScale,
        influences: I,
    ) where
        I: IntoIterator<
            Item = (
                Entity,
                UsfPosition,
                UsfSemanticFrame,
                UsfTravelInfluence,
                Option<UsfTravelBoundaryResolver>,
            ),
        >,
    {
        let mut candidates = influences
            .into_iter()
            .filter_map(|(entity, anchor, frame, influence, boundary)| {
                influence
                    .measure_from_at_scale(&anchor, frame, &observer, observer_scale, boundary.as_ref())
                    .map(|measurement| TravelInfluenceCandidate {
                        cached: CachedTravelInfluence {
                            entity,
                            anchor,
                            frame,
                            influence,
                            boundary,
                        },
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
            .cloned()
            .or_else(|| {
                candidates
                    .iter()
                    .min_by(|a, b| {
                        a.measurement
                            .boundary_clearance_scale0()
                            .total_cmp(&b.measurement.boundary_clearance_scale0())
                    })
                    .cloned()
            });

        let hard = candidates
            .iter()
            .cloned()
            .filter(|candidate| {
                matches!(
                    candidate.cached.influence.kind(),
                    UsfTravelInfluenceKind::HardBody
                )
            })
            .collect::<Vec<_>>();
        let media = candidates
            .iter()
            .cloned()
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
        self.sampled_position = Some(observer);
        self.sampled_scale = Some(observer_scale);
        self.age_seconds = 0.0;
    }

    pub fn influences(&self) -> impl Iterator<Item = UsfTravelInfluence> + '_ {
        self.influences.iter().map(|cached| cached.influence)
    }

    pub fn influences_with_entities(
        &self,
    ) -> impl Iterator<Item = (Entity, UsfTravelInfluence)> + '_ {
        self.influences
            .iter()
            .map(|cached| (cached.entity, cached.influence))
    }

    pub fn measurements_from<'a>(
    &'a self,
    observer: &'a UsfPosition,
    scale: SpatialScale,
) -> impl Iterator<
    Item = (
        Entity,
        UsfPosition,
        UsfTravelInfluence,
        UsfTravelInfluenceMeasure,
    ),
> + 'a {
    self.influences.iter().filter_map(move |cached| {
        cached
            .influence
            .measure_from_at_scale(
                &cached.anchor,
                cached.frame,
                observer,
                scale,
                cached.boundary.as_ref(),
            )
            .map(|measurement| (
                cached.entity,
                cached.anchor,
                cached.influence,
                measurement,
            ))
    })
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
    fn navigation_fallback_is_canonical_and_chart_independent() {
        let coarse = UsfNavigationContext::fallback(SpatialScale::new(18).unwrap());
        let fine = UsfNavigationContext::fallback(SpatialScale::new(-6).unwrap());

        assert_eq!(coarse.characteristic_length_scale0(), 10_000.0);
        assert_eq!(
            coarse.characteristic_length_scale0(),
            fine.characteristic_length_scale0(),
        );
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

    #[derive(Debug)]
    struct FixedTravelBoundary {
        surface: UsfPosition,
        outward: Vec3,
        scale: SpatialScale,
    }

    impl UsfTravelBoundary for FixedTravelBoundary {
        fn sample_near(
            &self,
            _: &UsfPosition,
            _: UsfSemanticFrame,
            _: &UsfPosition,
            _: SpatialScale,
        ) -> Option<UsfTravelBoundarySample> {
            UsfTravelBoundarySample::new(self.surface, self.outward, self.scale)
        }
    }

    #[test]
    fn hard_body_boundary_resolver_overrides_spherical_clearance() {
        let scale = SpatialScale::ZERO;
        let center = UsfPosition::zero(scale);
        let observer = UsfPosition::from_scale_native_f64(
            DVec3::new(0.0, 12.0, 0.0),
            scale,
            scale,
        )
        .unwrap();
        let surface = UsfPosition::from_scale_native_f64(
            DVec3::new(0.0, 11.5, 0.0),
            scale,
            scale,
        )
        .unwrap();

        let influence = UsfTravelInfluence::hard_body(scale, 10.0);
        let resolver = UsfTravelBoundaryResolver::new(FixedTravelBoundary {
            surface,
            outward: Vec3::Y,
            scale,
        });

        let spherical = influence.measure_from(&center, &observer).unwrap();
        let resolved = influence
            .measure_from_at_scale(&center, UsfSemanticFrame::identity(), &observer, scale, Some(&resolver))
            .unwrap();

        assert!((spherical.boundary_clearance_scale0() - 2.0).abs() < 1.0e-6);
        assert!((resolved.boundary_clearance_scale0() - 0.5).abs() < 1.0e-6);
        assert!(!resolved.inside());
    }

    #[test]
    fn constructors_make_semantics_explicit() {
        let scale = SpatialScale::ZERO;
        let hard = UsfTravelInfluence::hard_body(scale, 2.0);
        let medium = UsfTravelInfluence::medium(scale, 8.0, 1.0, 0.4, 0.2, 0.1);
        let region = UsfTravelInfluence::region(scale, 20.0);

        assert_eq!(hard.scale(), scale);
        assert!(matches!(hard.kind(), UsfTravelInfluenceKind::HardBody));
        assert!(matches!(medium.kind(), UsfTravelInfluenceKind::Medium(_)));
        assert!(matches!(region.kind(), UsfTravelInfluenceKind::Region));
    }

    #[test]
    fn travel_influence_follows_external_semantic_anchor() {
        let scale = SpatialScale::ZERO;
        let influence = UsfTravelInfluence::hard_body(scale, 10.0);
        let anchor_a = UsfPosition::zero(scale);
        let anchor_b = anchor_a.translated_at_scale(scale, Vec3::X * 100.0).unwrap();
        let observer = anchor_b.translated_at_scale(scale, Vec3::X * 12.0).unwrap();

        let from_a = influence.measure_from(&anchor_a, &observer).unwrap();
        let from_b = influence.measure_from(&anchor_b, &observer).unwrap();

        assert!(from_a.boundary_clearance_scale0() > 90.0);
        assert!((from_b.boundary_clearance_scale0() - 2.0).abs() < 1.0e-6);
    }
}
