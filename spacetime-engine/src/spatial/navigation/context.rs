//! Observer-local navigation context and refinement capability.

use super::{
    SpatialScale, UsfPosition, UsfTravelInfluenceKind, UsfTravelInfluenceMeasure,
    UsfTravelNeighborhood,
};
use bevy::prelude::*;

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
    pub const fn new(minimum_scale: SpatialScale) -> Self {
        Self { minimum_scale }
    }
    pub const fn minimum_scale(self) -> SpatialScale {
        self.minimum_scale
    }
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
    characteristic_length_metres: f64,
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
            characteristic_length_metres: NAVIGATION_FALLBACK_CHARACTERISTIC_METRES,
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
            characteristic_length_metres: f64,
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

            let characteristic_length_metres =
                navigation_length_metres(influence.kind(), measurement);
            if !characteristic_length_metres.is_finite() || characteristic_length_metres <= 0.0 {
                continue;
            }

            let candidate = Candidate {
                kind,
                source_scale: influence.scale(),
                relative_proximity: measurement.relative_proximity(),
                characteristic_length_metres,
            };

            let should_replace = |current: Option<Candidate>| {
                current
                    .is_none_or(|current| candidate.relative_proximity < current.relative_proximity)
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
            characteristic_length_metres: selected.characteristic_length_metres,
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

    pub const fn characteristic_length_metres(self) -> f64 {
        self.characteristic_length_metres
    }
}

fn navigation_length_metres(
    kind: UsfTravelInfluenceKind,
    measurement: UsfTravelInfluenceMeasure,
) -> f64 {
    match kind {
        UsfTravelInfluenceKind::HardBody => measurement.boundary_clearance_metres().max(1.0),
        UsfTravelInfluenceKind::Medium(_) => measurement
            .boundary_clearance_metres()
            .max(measurement.characteristic_length_metres()),
        UsfTravelInfluenceKind::Region => measurement
            .boundary_clearance_metres()
            .max(measurement.extent_radius_metres() * 2.0),
    }
}
