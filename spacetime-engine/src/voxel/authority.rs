//! Shared semantic authority for voxel fields with multiple realizations.
// canonical-celestial-surface-coherence-v4
//!
//! A realization owns residency, dense materializations, meshes and colliders.
//! The authority owns semantic identity and the canonical ordered edit history.
//! Scale-specific spatial indexes may be derived later, but they must never
//! become edit identity.

use bevy::{math::DVec3, prelude::*};

use crate::spatial::{
    SpatialScale, UsfPosition, UsfPositionError, UsfSemanticFrame, UsfTravelBoundary,
    UsfTravelBoundarySample,
};

use super::{CelestialBodyProfile, ProceduralCelestialBody, VoxelFrameEdit};
use super::base::CelestialFieldSample;

/// Canonical edit authority shared by one or more voxel realizations.
///
/// The log is intentionally address-agnostic. A materialization address belongs
/// to one scale-local realization and therefore cannot be the global edit key.
#[derive(Component, Debug, Default)]
pub struct VoxelAuthority {
    edits: Vec<VoxelFrameEdit>,
}

impl VoxelAuthority {
    pub fn record_edit(&mut self, edit: VoxelFrameEdit) {
        self.edits.push(edit);
    }

    pub fn edits(&self) -> &[VoxelFrameEdit] { &self.edits }

    pub fn edits_since(&self, first_edit_index: usize) -> impl Iterator<Item = VoxelFrameEdit> + '_ {
        self.edits.iter().copied().skip(first_edit_index.min(self.edits.len()))
    }

    pub fn len(&self) -> usize { self.edits.len() }
    pub fn is_empty(&self) -> bool { self.edits.is_empty() }
}


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

// prepared-presentation-field-sampler-v1
//
// Binary presentation evaluates hundreds/thousands of samples at one fixed
// spacing. Resolve the authored detail band, cave policy and body adapter once
// per mesh build instead of once per density sample.
#[derive(Debug, Clone, Copy)]
pub(crate) struct CelestialPresentationFieldSampler {
    body: ProceduralCelestialBody,
    through_scale: SpatialScale,
    include_caves: bool,
}

impl CelestialPresentationFieldSampler {
    #[inline]
    pub(crate) fn signed_distance_local_metres(
        self,
        local_point_metres: DVec3,
    ) -> Option<f64> {
        self.body.signed_distance_local_metres_through(
            local_point_metres,
            self.through_scale,
            self.include_caves,
        )
    }

    #[inline]
    pub(crate) fn surface_local_metres(
        self,
        direction: Vec3,
    ) -> Result<DVec3, UsfPositionError> {
        self.body
            .surface_local_metres_through(direction, self.through_scale)
    }
}

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

    pub const fn radius_metres(self) -> f64 { self.radius_metres }
    pub const fn coarsest_detail_scale(self) -> SpatialScale { self.coarsest_detail_scale }
    pub const fn surface_detail_scale(self) -> SpatialScale { self.surface_detail_scale }
    pub const fn profile(self) -> CelestialBodyProfile { self.profile }
    pub(crate) const fn seed(self) -> u32 { self.seed }

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

    pub(crate) fn surface_local_metres(
        self,
        direction: Vec3,
    ) -> Result<DVec3, UsfPositionError> {
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
    ) -> ProceduralCelestialBody {
        ProceduralCelestialBody::new(
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

    // presentation-field-bandlimit-v1
    fn presentation_detail_scale_for_spacing(
        self,
        sample_spacing_metres: f64,
    ) -> Option<SpatialScale> {
        if !sample_spacing_metres.is_finite() || sample_spacing_metres <= 0.0 {
            return None;
        }

        let exponent = sample_spacing_metres
            .log10()
            .floor()
            .clamp(
                f64::from(self.surface_detail_scale.exponent()),
                f64::from(self.coarsest_detail_scale.exponent()),
            ) as i8;
        SpatialScale::new(exponent)
    }

    pub(crate) fn presentation_sampler(
        self,
        sample_spacing_metres: f64,
    ) -> Option<CelestialPresentationFieldSampler> {
        const CAVE_MAX_PRESENTATION_SAMPLE_SPACING_METRES: f64 = 64.0;

        let through_scale =
            self.presentation_detail_scale_for_spacing(sample_spacing_metres)?;
        Some(CelestialPresentationFieldSampler {
            body: self.realization(
                UsfPosition::zero(SpatialScale::MIN),
                UsfSemanticFrame::identity(),
                SpatialScale::ZERO,
            ),
            through_scale,
            include_caves:
                sample_spacing_metres <= CAVE_MAX_PRESENTATION_SAMPLE_SPACING_METRES,
        })
    }

    pub(crate) fn presentation_surface_local_metres(
        self,
        direction: Vec3,
        sample_spacing_metres: f64,
    ) -> Result<DVec3, UsfPositionError> {
        self.presentation_sampler(sample_spacing_metres)
            .ok_or(UsfPositionError::NonFiniteTranslation)?
            .surface_local_metres(direction)
    }

    pub(crate) fn presentation_signed_distance_local_metres(
        self,
        local_point_metres: DVec3,
        sample_spacing_metres: f64,
    ) -> Option<f64> {
        self.presentation_sampler(sample_spacing_metres)?
            .signed_distance_local_metres(local_point_metres)
    }

    pub(crate) fn volumetric_surface_inward_support_metres(self) -> f64 {
        self.realization(
            UsfPosition::zero(SpatialScale::MIN),
            UsfSemanticFrame::identity(),
            SpatialScale::ZERO,
        )
        .volumetric_surface_inward_support_metres()
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

pub(crate) fn signed_distance_local_metres(
        self,
        local_point_metres: DVec3,
    ) -> Option<f64> {
        self.sample_local_metres(local_point_metres)
            .map(CelestialFieldSample::signed_distance_metres)
    }

    // semantic-volumetric-boundary-projection-v1
    fn normalized_field_gradient_local_metres(
        self,
        point: DVec3,
        epsilon_metres: f64,
    ) -> Option<DVec3> {
        if !point.is_finite()
            || !epsilon_metres.is_finite()
            || epsilon_metres <= 0.0
        {
            return None;
        }

        let e = epsilon_metres;
        let dx = self.signed_distance_local_metres(point + DVec3::X * e)?
            - self.signed_distance_local_metres(point - DVec3::X * e)?;
        let dy = self.signed_distance_local_metres(point + DVec3::Y * e)?
            - self.signed_distance_local_metres(point - DVec3::Y * e)?;
        let dz = self.signed_distance_local_metres(point + DVec3::Z * e)?
            - self.signed_distance_local_metres(point - DVec3::Z * e)?;

        let gradient = DVec3::new(dx, dy, dz) / (2.0 * e);
        let length = gradient.length();
        if !length.is_finite() || length <= 1.0e-9 {
            return None;
        }
        Some(gradient / length)
    }

    /// Projects a body-local point onto the nearest reachable zero crossing of
    /// the canonical volumetric SDF.
    ///
    /// Unlike `surface_position` / `surface_near`, this follows cave walls and
    /// other non-radial volumetric boundaries because it samples the complete
    /// semantic field.
    ///
    /// The returned signed distance is the INITIAL clearance at `point`, not
    /// the tiny final projection residual.
    pub(crate) fn nearest_boundary_local_metres(
        self,
        point: DVec3,
        max_abs_distance_metres: f64,
    ) -> Option<(DVec3, f64)> {
        const MAX_ITERATIONS: usize = 16;
        const TARGET_RESIDUAL_METRES: f64 = 0.05;
        const ACCEPTABLE_RESIDUAL_METRES: f64 = 2.0;
        const MIN_GRADIENT_EPSILON_METRES: f64 = 0.25;
        const MAX_GRADIENT_EPSILON_METRES: f64 = 16.0;
        const MAX_PROJECTION_STEP_METRES: f64 = 250_000.0;

        if !point.is_finite()
            || !max_abs_distance_metres.is_finite()
            || max_abs_distance_metres < 0.0
        {
            return None;
        }

        let initial_distance = self.signed_distance_local_metres(point)?;
        if !initial_distance.is_finite()
            || initial_distance.abs() > max_abs_distance_metres
        {
            return None;
        }

        let mut projected = point;
        let mut residual = initial_distance;

        for _ in 0..MAX_ITERATIONS {
            if residual.abs() <= TARGET_RESIDUAL_METRES {
                return Some((projected, initial_distance));
            }

            let epsilon = (residual.abs() * 0.05)
                .clamp(
                    MIN_GRADIENT_EPSILON_METRES,
                    MAX_GRADIENT_EPSILON_METRES,
                );

            let normal = self
                .normalized_field_gradient_local_metres(projected, epsilon)
                .or_else(|| {
                    let radius = projected.length();
                    (radius.is_finite() && radius > f64::EPSILON)
                        .then_some(projected / radius)
                })?;

            let step = residual.clamp(
                -MAX_PROJECTION_STEP_METRES,
                MAX_PROJECTION_STEP_METRES,
            );
            projected -= normal * step;
            residual = self.signed_distance_local_metres(projected)?;
        }

        if residual.abs() <= ACCEPTABLE_RESIDUAL_METRES {
            return Some((projected, initial_distance));
        }

        // Robust fallback for pathological/noisy gradients far from caves:
        // recover the authored outer surface rather than dropping all terrain
        // presentation/demand.
        let radius = point.length();
        if !radius.is_finite() || radius <= f64::EPSILON {
            return None;
        }
        let direction = Vec3::new(
            (point.x / radius) as f32,
            (point.y / radius) as f32,
            (point.z / radius) as f32,
        )
        .normalize_or_zero();
        if direction == Vec3::ZERO {
            return None;
        }
        let outer = self.surface_local_metres(direction).ok()?;
        Some((outer, initial_distance))
    }

    /// Canonical/world-space adapter around nearest volumetric boundary
    /// projection.
    pub(crate) fn boundary_near(
        self,
        body_origin: &UsfPosition,
        body_frame: UsfSemanticFrame,
        point: &UsfPosition,
        max_abs_distance_metres: f64,
    ) -> Option<(UsfPosition, f64)> {
        let local = body_frame
            .world_to_local_metres(
                body_origin,
                point,
                SpatialScale::ZERO,
                f64::MAX,
            )
            .ok()?;
        let (boundary_local, signed_clearance_metres) =
            self.nearest_boundary_local_metres(
                local,
                max_abs_distance_metres,
            )?;
        let boundary = body_frame
            .local_metres_to_world(*body_origin, boundary_local)
            .ok()?;
        Some((boundary, signed_clearance_metres))
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nearest_boundary_projection_converges_from_above_surface() {
        let radius = 6_371_000.0;
        let field = CelestialVoxelField::new(
            radius,
            SpatialScale::new(6).unwrap(),
            SpatialScale::ZERO,
            0x4541_5254,
            CelestialBodyProfile::Rocky,
        );
        let direction = DVec3::Y;
        let authored_surface = field
            .surface_local_metres(Vec3::Y)
            .unwrap();
        let observer = authored_surface + direction * 2_000.0;

        let (projected, clearance) = field
            .nearest_boundary_local_metres(observer, 10_000.0)
            .expect("outer boundary should be projectable");

        assert!(clearance > 1_000.0);
        assert!(
            (projected - authored_surface).length() < 5.0,
            "projected={projected:?}, authored={authored_surface:?}",
        );
    }

    #[test]
fn authority_preserves_one_body_local_edit_order() {
        use crate::voxel::{VoxelFrameBrush, VoxelFramePosition, VoxelMaterialId};
        use bevy::math::DVec3;

        let center = VoxelFramePosition::from_scale_native(DVec3::ZERO, SpatialScale::ZERO).unwrap();
        let first = VoxelFrameEdit::Add {
            brush: VoxelFrameBrush::sphere(center, 2.0),
            material: VoxelMaterialId::ROCK,
        };
        let second = VoxelFrameEdit::Remove {
            brush: VoxelFrameBrush::sphere(center, 1.0),
        };

        let mut authority = VoxelAuthority::default();
        authority.record_edit(first);
        authority.record_edit(second);

        assert_eq!(authority.edits(), &[first, second]);
        assert_eq!(authority.edits_since(1).collect::<Vec<_>>(), vec![second]);
    }

    #[test]
    fn one_celestial_field_derives_consistent_scale_realizations() {
        let center = UsfPosition::zero(SpatialScale::ZERO);
        let frame = UsfSemanticFrame::identity();
        let detail_root = SpatialScale::new(5).unwrap();
        let field = CelestialVoxelField::new(
            1_737_000.0,
            detail_root, SpatialScale::ZERO,
            0x4D4F_4F4E,
            CelestialBodyProfile::Lunar,
        );

        for raw in SpatialScale::MIN.exponent()..=5 {
            let scale = SpatialScale::new(raw).unwrap();
            let realization = field.realization(center, frame, scale);
            let reconstructed =
                realization.radius_native_f64() * scale.metres_per_native();
            assert!((reconstructed - field.radius_metres()).abs() < 1.0e-6);
        }
    }
}
