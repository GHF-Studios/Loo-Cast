//! Observer-local cache and selection policy for nearby travel influences.

use super::*;
use bevy::{math::DVec3, prelude::*};

const NEIGHBORHOOD_HARD_BY_RELATIVE_PROXIMITY: usize = 6;
const NEIGHBORHOOD_HARD_BY_ABSOLUTE_PROXIMITY: usize = 4;
const NEIGHBORHOOD_MEDIUM_BY_RELATIVE_PROXIMITY: usize = 6;
const NEIGHBORHOOD_MEDIUM_BY_ABSOLUTE_PROXIMITY: usize = 4;
const NEIGHBORHOOD_REGION_BY_RELATIVE_PROXIMITY: usize = 4;
const NEIGHBORHOOD_MAX_AGE_SECONDS: f32 = 0.5;
const NEIGHBORHOOD_PROXIMITY_REFRESH_FRACTION: f64 = 0.10;
const NEIGHBORHOOD_FEATURE_REFRESH_FRACTION: f64 = 0.05;
const NEIGHBORHOOD_MIN_REFRESH_DISTANCE_METRES: f64 = 1.0;

#[derive(Debug, Clone)]
struct CachedTravelInfluence {
    entity: Entity,
    anchor: UsfPosition,
    frame: UsfSemanticFrame,
    influence: UsfTravelInfluence,
    boundary: Option<UsfTravelBoundaryProvider>,
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
            .boundary_clearance_metres()
            .total_cmp(&b.measurement.boundary_clearance_metres()),
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
    refresh_distance_metres: f64,
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

    fn advance(&mut self, dt_seconds: f32) {
        self.age_seconds += dt_seconds.max(0.0);
    }

    fn needs_refresh(
        &self,
        observer: &UsfPosition,
        observer_scale: SpatialScale,
        velocity_metres_per_second: DVec3,
        step_seconds: f32,
    ) -> bool {
        let (Some(sampled), Some(sampled_scale)) = (self.sampled_position, self.sampled_scale)
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

        // The cache's spatial validity also has a temporal deadline. A fast
        // subject can traverse the whole unsampled margin between two ordinary
        // age-based refreshes, even when the previous frame barely moved.
        // Refresh before the next step reaches half the measured margin.
        let speed = velocity_metres_per_second.length();
        if !speed.is_finite() {
            return true;
        }
        if speed > 0.0
            && (f64::from(self.age_seconds) + f64::from(step_seconds.max(0.0))) * speed
                >= self.refresh_distance_metres * 0.5
        {
            return true;
        }

        if !self.refresh_distance_metres.is_finite() {
            return false;
        }

        let bound_native = (self.refresh_distance_metres / observer_scale.metres_per_native())
            .max(1.0)
            .min(f64::from(f32::MAX)) as f32;
        let Ok(delta) = observer.relative_at_scale_bounded(&sampled, observer_scale, bound_native)
        else {
            return true;
        };
        let moved_scale0 = f64::from(delta.length()) * observer_scale.metres_per_native();
        moved_scale0 >= self.refresh_distance_metres
    }

    fn refresh<I>(&mut self, observer: UsfPosition, observer_scale: SpatialScale, influences: I)
    where
        I: IntoIterator<
            Item = (
                Entity,
                UsfPosition,
                UsfSemanticFrame,
                UsfTravelInfluence,
                Option<UsfTravelBoundaryProvider>,
            ),
        >,
    {
        let candidates = measured_candidates(observer, observer_scale, influences);
        self.refresh_distance_metres = refresh_distance(&candidates);
        self.influences = selected_influences(&candidates);
        self.sampled_position = Some(observer);
        self.sampled_scale = Some(observer_scale);
        self.age_seconds = 0.0;
    }

    /// Advance and refresh the observer-local cache when its validity horizon expires.
    ///
    /// `influences` is a closure so callers do not build/clone refresh input
    /// unless a refresh is actually required.
    pub fn refresh_if_needed<I, F>(
        &mut self,
        dt_seconds: f32,
        observer: UsfPosition,
        observer_scale: SpatialScale,
        velocity_metres_per_second: DVec3,
        influences: F,
    ) -> bool
    where
        F: FnOnce() -> I,
        I: IntoIterator<
            Item = (
                Entity,
                UsfPosition,
                UsfSemanticFrame,
                UsfTravelInfluence,
                Option<UsfTravelBoundaryProvider>,
            ),
        >,
    {
        self.advance(dt_seconds);
        if !self.needs_refresh(
            &observer,
            observer_scale,
            velocity_metres_per_second,
            dt_seconds,
        ) {
            return false;
        }

        self.refresh(observer, observer_scale, influences());
        true
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
                .map(|measurement| (cached.entity, cached.anchor, cached.influence, measurement))
        })
    }
}

fn measured_candidates<I>(
    observer: UsfPosition,
    scale: SpatialScale,
    influences: I,
) -> Vec<TravelInfluenceCandidate>
where
    I: IntoIterator<
        Item = (
            Entity,
            UsfPosition,
            UsfSemanticFrame,
            UsfTravelInfluence,
            Option<UsfTravelBoundaryProvider>,
        ),
    >,
{
    influences
        .into_iter()
        .filter_map(|(entity, anchor, frame, influence, boundary)| {
            influence
                .measure_from_at_scale(&anchor, frame, &observer, scale, boundary.as_ref())
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
        .collect()
}

fn refresh_distance(candidates: &[TravelInfluenceCandidate]) -> f64 {
    // Regions provide discovery context, not a physical refresh horizon. Use
    // one only when the neighborhood contains no hard body or medium.
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
                .boundary_clearance_metres()
                .total_cmp(&b.measurement.boundary_clearance_metres())
        })
        .or_else(|| {
            candidates.iter().min_by(|a, b| {
                a.measurement
                    .boundary_clearance_metres()
                    .total_cmp(&b.measurement.boundary_clearance_metres())
            })
        });
    nearest.map_or(f64::INFINITY, |candidate| {
        (candidate.measurement.boundary_clearance_metres()
            * NEIGHBORHOOD_PROXIMITY_REFRESH_FRACTION)
            .max(
                candidate.measurement.characteristic_length_metres()
                    * NEIGHBORHOOD_FEATURE_REFRESH_FRACTION,
            )
            .max(NEIGHBORHOOD_MIN_REFRESH_DISTANCE_METRES)
    })
}

fn selected_influences(candidates: &[TravelInfluenceCandidate]) -> Vec<CachedTravelInfluence> {
    let of_kind = |kind: fn(UsfTravelInfluenceKind) -> bool| {
        candidates
            .iter()
            .cloned()
            .filter(|candidate| kind(candidate.cached.influence.kind()))
            .collect::<Vec<_>>()
    };
    let hard = of_kind(|kind| matches!(kind, UsfTravelInfluenceKind::HardBody));
    let media = of_kind(|kind| matches!(kind, UsfTravelInfluenceKind::Medium(_)));
    let regions = of_kind(|kind| matches!(kind, UsfTravelInfluenceKind::Region));
    let mut selected = Vec::new();
    append_nearest(
        &mut selected,
        &hard,
        NEIGHBORHOOD_HARD_BY_RELATIVE_PROXIMITY,
        CandidateOrder::Relative,
    );
    append_nearest(
        &mut selected,
        &hard,
        NEIGHBORHOOD_HARD_BY_ABSOLUTE_PROXIMITY,
        CandidateOrder::Absolute,
    );
    append_nearest(
        &mut selected,
        &media,
        NEIGHBORHOOD_MEDIUM_BY_RELATIVE_PROXIMITY,
        CandidateOrder::Relative,
    );
    append_nearest(
        &mut selected,
        &media,
        NEIGHBORHOOD_MEDIUM_BY_ABSOLUTE_PROXIMITY,
        CandidateOrder::Absolute,
    );
    append_nearest(
        &mut selected,
        &regions,
        NEIGHBORHOOD_REGION_BY_RELATIVE_PROXIMITY,
        CandidateOrder::Relative,
    );
    selected
}
