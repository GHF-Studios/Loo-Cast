//! Refinable hard-body surface provider; navigation samples but does not own it.

use super::*;
use bevy::prelude::*;
use std::{fmt, sync::Arc};

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

    pub const fn surface(self) -> UsfPosition {
        self.surface
    }
    pub const fn outward(self) -> Vec3 {
        self.outward
    }
    pub const fn scale(self) -> SpatialScale {
        self.scale
    }
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
        Self {
            provider: Arc::new(provider),
        }
    }

    pub fn sample_near(
        &self,
        body_origin: &UsfPosition,
        body_frame: UsfSemanticFrame,
        observer: &UsfPosition,
        scale: SpatialScale,
    ) -> Option<UsfTravelBoundarySample> {
        self.provider
            .sample_near(body_origin, body_frame, observer, scale)
    }
}
