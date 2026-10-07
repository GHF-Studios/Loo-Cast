//! Semantic celestial field definition and canonical surface queries.
//!
//! This field owns the body's volumetric terrain identity. Numerical Scale is
//! chosen only when adapting it to a bounded voxel sampler or presentation.
//!
//! ## Module map
//!
//! - `boundary`: Volumetric zero-crossing projection and travel-boundary queries.
//! - `sampler`: Prepared presentation adapter over one canonical celestial field.
//!
//! Reexports here define the supported surface; child modules hold its implementation.
//!

use bevy::{math::DVec3, prelude::*};

use crate::spatial::{
    SpatialScale, UsfPosition, UsfPositionError, UsfSemanticFrame, UsfTravelBoundary,
    UsfTravelBoundarySample,
};

use super::CelestialFieldRealization;
use super::base::CelestialFieldSample;

/// One semantic celestial field definition.
///
/// This is deliberately independent of realization scale. `realization()` is
/// the adapter that produces a bounded scale-local procedural sampler.
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct CelestialVoxelField {
    radius_metres: f64,
    coarsest_detail_scale: SpatialScale,
}

mod boundary;
mod sampler;

pub(crate) use sampler::CelestialPresentationFieldSampler;

impl CelestialVoxelField {
    /// Plain volumetric sphere. Detail and cave algorithms are not authored here.
    pub fn sphere(radius_metres: f64, coarsest_detail_scale: SpatialScale) -> Self {
        assert!(
            radius_metres.is_finite() && radius_metres > 0.0,
            "celestial authority radius must be finite and positive"
        );
        Self {
            radius_metres,
            coarsest_detail_scale,
        }
    }

    pub const fn radius_metres(self) -> f64 {
        self.radius_metres
    }
    pub const fn coarsest_detail_scale(self) -> SpatialScale {
        self.coarsest_detail_scale
    }
    /// One canonical semantic surface. No realization Scale parameter exists
    /// here by design: callers cannot request a different planet by choosing a
    /// different numerical chart.
    pub fn surface_position(
        self,
        body_origin: &UsfPosition,
        body_frame: UsfSemanticFrame,
        direction: Vec3,
    ) -> Result<UsfPosition, UsfPositionError> {
        self.realization(*body_origin, body_frame, SpatialScale::ZERO)
            .surface_position(direction)
    }

    pub(crate) fn surface_local_metres(self, direction: Vec3) -> Result<DVec3, UsfPositionError> {
        self.realization(
            UsfPosition::zero(SpatialScale::MIN),
            UsfSemanticFrame::identity(),
            SpatialScale::ZERO,
        )
        .surface_local_metres(direction)
    }

    /// `scale` selects numerical units/cache addressing only.
    pub fn realization(
        self,
        body_origin: UsfPosition,
        body_frame: UsfSemanticFrame,
        scale: SpatialScale,
    ) -> CelestialFieldRealization {
        CelestialFieldRealization::new(body_origin, body_frame, self.radius_metres, scale)
    }

    pub(crate) fn presentation_sampler(
        self,
        sample_spacing_metres: f64,
    ) -> Option<CelestialPresentationFieldSampler> {
        // Spacing validates the representation request only. It MUST NOT select
        // a different semantic terrain function.
        if !sample_spacing_metres.is_finite() || sample_spacing_metres <= 0.0 {
            return None;
        }

        Some(CelestialPresentationFieldSampler {
            body: self
                .realization(
                    UsfPosition::zero(SpatialScale::MIN),
                    UsfSemanticFrame::identity(),
                    SpatialScale::ZERO,
                )
                .prepare_presentation_sampler(),
        })
    }

    pub(crate) fn volumetric_surface_inward_support_metres(self) -> f64 {
        self.realization(
            UsfPosition::zero(SpatialScale::MIN),
            UsfSemanticFrame::identity(),
            SpatialScale::ZERO,
        )
        .volumetric_surface_inward_support_metres()
    }

    /// Global radial interval for this canonical field's outer surface.
    pub(crate) fn conservative_surface_radius_bounds_metres(self) -> (f64, f64) {
        self.realization(
            UsfPosition::zero(SpatialScale::MIN),
            UsfSemanticFrame::identity(),
            SpatialScale::ZERO,
        )
        .conservative_surface_radius_bounds_metres()
    }

    /// Sample the canonical body-local volumetric field in SI metres.
    ///
    /// This is the common terrain truth for dense voxel caches, local clipmap
    /// presentation and any future collision/query acceleration structure.
    pub(crate) fn sample_local_metres(
        self,
        local_point_metres: DVec3,
    ) -> Option<CelestialFieldSample> {
        self.realization(
            UsfPosition::zero(SpatialScale::MIN),
            UsfSemanticFrame::identity(),
            SpatialScale::ZERO,
        )
        .field_sample_local_metres(local_point_metres)
    }

    /// Stable outer-surface approximation of the canonical field.
    pub(crate) fn outer_signed_distance_local_metres(
        self,
        local_point_metres: DVec3,
    ) -> Option<f64> {
        self.realization(
            UsfPosition::zero(SpatialScale::MIN),
            UsfSemanticFrame::identity(),
            SpatialScale::ZERO,
        )
        .outer_signed_distance_local_metres(local_point_metres)
    }

    pub(crate) fn signed_distance_local_metres(self, local_point_metres: DVec3) -> Option<f64> {
        self.sample_local_metres(local_point_metres)
            .map(CelestialFieldSample::signed_distance_metres)
    }

    pub(crate) fn conservative_outer_radius_metres(self) -> f64 {
        self.realization(
            UsfPosition::zero(SpatialScale::MIN),
            UsfSemanticFrame::identity(),
            SpatialScale::ZERO,
        )
        .conservative_outer_radius_metres()
    }
}

impl UsfTravelBoundary for CelestialVoxelField {
    fn sample_near(
        &self,
        body_origin: &UsfPosition,
        body_frame: UsfSemanticFrame,
        observer: &UsfPosition,
        requested_scale: SpatialScale,
    ) -> Option<UsfTravelBoundarySample> {
        // Scale selects the bounded measurement chart, never surface semantics.
        let measurement_scale = requested_scale.min(self.coarsest_detail_scale());
        let body = (*self).realization(*body_origin, body_frame, measurement_scale);
        let (surface, outward, _) = body.surface_near(observer, f32::MAX)?;
        UsfTravelBoundarySample::new(surface, outward, measurement_scale)
    }
}
