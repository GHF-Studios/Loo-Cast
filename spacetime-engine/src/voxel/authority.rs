//! Shared semantic authority for voxel fields with multiple realizations.
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
    seed: u32,
    profile: CelestialBodyProfile,
}

impl CelestialVoxelField {
    pub fn new(
        radius_metres: f64,
        coarsest_detail_scale: SpatialScale,
        seed: u32,
        profile: CelestialBodyProfile,
    ) -> Self {
        assert!(
            radius_metres.is_finite() && radius_metres > 0.0,
            "celestial authority radius must be finite and positive"
        );
        Self { radius_metres, coarsest_detail_scale, seed, profile }
    }

    pub const fn radius_metres(self) -> f64 { self.radius_metres }
    pub const fn coarsest_detail_scale(self) -> SpatialScale { self.coarsest_detail_scale }
    pub const fn profile(self) -> CelestialBodyProfile { self.profile }

    pub fn surface_position(
        self,
        body_origin: &UsfPosition,
        body_frame: UsfSemanticFrame,
        direction: Vec3,
        scale: SpatialScale,
    ) -> Result<UsfPosition, UsfPositionError> {
        self.realization(*body_origin, body_frame, scale).surface_position(direction)
    }

    /// Body-local semantic surface position in SI metres.
    pub(crate) fn surface_local_metres(
        self, direction: Vec3, scale: SpatialScale,
    ) -> Result<DVec3, UsfPositionError> {
        self.realization(
            UsfPosition::zero(SpatialScale::MIN), UsfSemanticFrame::identity(), scale,
        ).surface_local_metres(direction)
    }

    /// Creates one disposable pose-bound procedural sampler for a realization.
    /// The snapshot is never semantic placement authority.
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
            self.seed,
            self.profile,
        )
    }

    pub(crate) fn conservative_outer_radius_metres(self) -> f64 {
        // Placement is irrelevant to this scalar geometry bound. Use a disposable
        // zero/identity snapshot rather than re-introducing a center into authority.
        self.realization(
            UsfPosition::zero(SpatialScale::MIN),
            UsfSemanticFrame::identity(),
            SpatialScale::MIN,
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
        let surface_scale = requested_scale.min(self.coarsest_detail_scale());
        let body = (*self).realization(*body_origin, body_frame, surface_scale);
        let (surface, outward, _) = body.surface_near(observer, f32::MAX)?;
        UsfTravelBoundarySample::new(surface, outward, surface_scale)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            detail_root,
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
