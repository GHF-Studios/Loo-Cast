//! Semantic celestial field definition and canonical surface queries.
//!
//! This field owns terrain identity and bandwidth. Numerical Scale is chosen
//! only when adapting it to a bounded voxel sampler or presentation cache.
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

use super::base::CelestialFieldSample;
use super::{CelestialBodyProfile, CelestialFieldRealization};

/// One semantic celestial field definition.
///
/// This is deliberately independent of realization scale. `realization()` is
/// the adapter that produces a bounded scale-local procedural sampler.
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct CelestialVoxelField {
    radius_metres: f64,
    coarsest_detail_scale: SpatialScale,
    /// Finest semantic terrain band owned by this body definition.
    /// Realization Scale Slices never change this value.
    surface_detail_scale: SpatialScale,
    seed: u32,
    profile: CelestialBodyProfile,
}

mod boundary;
mod sampler;

pub(crate) use sampler::CelestialPresentationFieldSampler;

impl CelestialVoxelField {
    pub fn new(
        radius_metres: f64,
        coarsest_detail_scale: SpatialScale,
        surface_detail_scale: SpatialScale,
        seed: u32,
        profile: CelestialBodyProfile,
    ) -> Self {
        assert!(
            radius_metres.is_finite() && radius_metres > 0.0,
            "celestial authority radius must be finite and positive"
        );
        assert!(
            surface_detail_scale <= coarsest_detail_scale,
            "celestial surface detail floor must not be coarser than its detail root"
        );
        Self {
            radius_metres,
            coarsest_detail_scale,
            surface_detail_scale,
            seed,
            profile,
        }
    }

    pub const fn radius_metres(self) -> f64 {
        self.radius_metres
    }
    pub const fn coarsest_detail_scale(self) -> SpatialScale {
        self.coarsest_detail_scale
    }
    pub const fn surface_detail_scale(self) -> SpatialScale {
        self.surface_detail_scale
    }
    pub const fn profile(self) -> CelestialBodyProfile {
        self.profile
    }
    pub(crate) const fn seed(self) -> u32 {
        self.seed
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
        self.realization(*body_origin, body_frame, self.surface_detail_scale)
            .surface_position(direction)
    }

    pub(crate) fn surface_local_metres(self, direction: Vec3) -> Result<DVec3, UsfPositionError> {
        self.realization(
            UsfPosition::zero(SpatialScale::MIN),
            UsfSemanticFrame::identity(),
            self.surface_detail_scale,
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
        CelestialFieldRealization::new(
            body_origin,
            body_frame,
            self.radius_metres,
            scale,
            self.coarsest_detail_scale,
            self.surface_detail_scale,
            self.seed,
            self.profile,
        )
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

    // These are ONE-SHOT adapters. Constructing PreparedCelestialPresentationBody
    // here allocated a 4096-slot semantic-corner cache for every planner sample.
    // Reuse belongs to explicit `presentation_sampler()` owners instead.
    pub(crate) fn presentation_surface_local_metres(
        self,
        direction: Vec3,
        sample_spacing_metres: f64,
    ) -> Result<DVec3, UsfPositionError> {
        if !sample_spacing_metres.is_finite() || sample_spacing_metres <= 0.0 {
            return Err(UsfPositionError::NonFiniteTranslation);
        }
        self.realization(
            UsfPosition::zero(SpatialScale::MIN),
            UsfSemanticFrame::identity(),
            SpatialScale::ZERO,
        )
        .surface_local_metres(direction)
    }

    pub(crate) fn presentation_signed_distance_local_metres(
        self,
        local_point_metres: DVec3,
        sample_spacing_metres: f64,
    ) -> Option<f64> {
        if !sample_spacing_metres.is_finite() || sample_spacing_metres <= 0.0 {
            return None;
        }
        self.realization(
            UsfPosition::zero(SpatialScale::MIN),
            UsfSemanticFrame::identity(),
            SpatialScale::ZERO,
        )
        .signed_distance_local_metres(local_point_metres)
    }

    pub(crate) fn presentation_caves_may_intersect_aabb(
        self,
        center_local_metres: DVec3,
        half_extent_metres: DVec3,
    ) -> bool {
        self.realization(
            UsfPosition::zero(SpatialScale::MIN),
            UsfSemanticFrame::identity(),
            SpatialScale::ZERO,
        )
        .cave_void_may_intersect_local_aabb(center_local_metres, half_extent_metres)
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

    /// Optional void SDF used to subtract caves from a presentation shell.
    pub(crate) fn volumetric_void_signed_distance_local_metres(
        self,
        local_point_metres: DVec3,
    ) -> Option<f64> {
        self.realization(
            UsfPosition::zero(SpatialScale::MIN),
            UsfSemanticFrame::identity(),
            SpatialScale::ZERO,
        )
        .volumetric_void_signed_distance_local_metres(local_point_metres)
    }

    pub(crate) fn conservative_outer_radius_metres(self) -> f64 {
        self.realization(
            UsfPosition::zero(SpatialScale::MIN),
            UsfSemanticFrame::identity(),
            self.surface_detail_scale,
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
