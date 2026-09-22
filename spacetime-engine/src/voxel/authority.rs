//! Shared semantic authority for voxel fields with multiple realizations.
//!
//! A realization owns residency, dense materializations, meshes and colliders.
//! The authority owns semantic identity and the canonical ordered edit history.
//! Scale-specific spatial indexes may be derived later, but they must never
//! become edit identity.

use bevy::prelude::*;

use crate::spatial::{SpatialScale, UsfPosition};

use super::{CelestialBodyProfile, ProceduralCelestialBody, VoxelEdit};

/// Canonical edit authority shared by one or more voxel realizations.
///
/// The log is intentionally address-agnostic. A materialization address belongs
/// to one scale-local realization and therefore cannot be the global edit key.
#[derive(Component, Debug, Default)]
pub struct VoxelAuthority {
    edits: Vec<VoxelEdit>,
}

impl VoxelAuthority {
    pub fn record_edit(&mut self, edit: VoxelEdit) {
        self.edits.push(edit);
    }

    pub fn edits(&self) -> &[VoxelEdit] {
        &self.edits
    }

    pub fn edits_since(&self, first_edit_index: usize) -> impl Iterator<Item = VoxelEdit> + '_ {
        self.edits
            .iter()
            .copied()
            .skip(first_edit_index.min(self.edits.len()))
    }

    pub fn len(&self) -> usize {
        self.edits.len()
    }

    pub fn is_empty(&self) -> bool {
        self.edits.is_empty()
    }
}


/// One semantic celestial field definition.
///
/// This is deliberately independent of realization scale. `realization()` is
/// the adapter that produces a bounded scale-local procedural sampler.
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct CelestialVoxelField {
    center: UsfPosition,
    radius_metres: f64,
    coarsest_detail_scale: SpatialScale,
    seed: u32,
    profile: CelestialBodyProfile,
}

impl CelestialVoxelField {
    pub fn new(
        center: UsfPosition,
        radius_metres: f64,
        coarsest_detail_scale: SpatialScale,
        seed: u32,
        profile: CelestialBodyProfile,
    ) -> Self {
        assert!(
            radius_metres.is_finite() && radius_metres > 0.0,
            "celestial authority radius must be finite and positive"
        );
        Self {
            center,
            radius_metres,
            coarsest_detail_scale,
            seed,
            profile,
        }
    }

    pub const fn center(self) -> UsfPosition {
        self.center
    }

    pub const fn radius_metres(self) -> f64 {
        self.radius_metres
    }

    pub const fn coarsest_detail_scale(self) -> SpatialScale {
        self.coarsest_detail_scale
    }

    pub const fn profile(self) -> CelestialBodyProfile {
        self.profile
    }

    pub fn realization(self, scale: SpatialScale) -> ProceduralCelestialBody {
        ProceduralCelestialBody::new(
            self.center,
            self.radius_metres,
            scale,
            self.coarsest_detail_scale,
            self.seed,
            self.profile,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authority_preserves_one_global_edit_order() {
        use crate::voxel::{VoxelBrush, VoxelMaterialId, VoxelQueryPosition};
        use bevy::prelude::Vec3;

        let center = VoxelQueryPosition::from_scale0_local(Vec3::ZERO).unwrap();
        let first = VoxelEdit::Add {
            brush: VoxelBrush::sphere(center, 2.0),
            material: VoxelMaterialId::ROCK,
        };
        let second = VoxelEdit::Remove {
            brush: VoxelBrush::sphere(center, 1.0),
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
        let detail_root = SpatialScale::new(5).unwrap();
        let field = CelestialVoxelField::new(
            center,
            1_737_000.0,
            detail_root,
            0x4D4F_4F4E,
            CelestialBodyProfile::Lunar,
        );

        for raw in 0..=5 {
            let scale = SpatialScale::new(raw).unwrap();
            let realization = field.realization(scale);
            let reconstructed =
                f64::from(realization.radius_native()) * scale.metres_per_native();
            assert!((reconstructed - field.radius_metres()).abs() < 1.0);
        }
    }
}
