//! Sparse construction-time spatial index over canonical USF space.
//!
//! A domain supplies its *own* typed construction recipes and conservative
//! bounds. The catalog indexes known/derivable phenomena at canonical positions
//! without constructing any descendant chunks or acquiring simulation authority.
//!
//! A declaration may span arbitrarily many chunks and is discoverable from
//! each intersecting region. The catalog records authored/initial construction
//! conditions; mutable live pose, semantic history, and realization readiness
//! remain downstream. It is not a snapshot of a moving body's current position.

use std::collections::BTreeMap;
use crate::usf::{UsfChunkAddress, UsfPosition, USF_CHUNK_NATIVE_SIZE};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstructionCatalogError {
    EmptyIdentity,
    DuplicateIdentity,
    InvalidBound,
}

/// One known canonical phenomenon, with domain-owned construction input T.
/// This is NOT the runtime semantic entity or a simulation/presentation proxy.
#[derive(Debug)]
pub struct AnchoredConstruction<T> {
    key: String,
    initial_center: UsfPosition,
    conservative_radius_metres: f64,
    recipe: T,
}

impl<T> AnchoredConstruction<T> {
    pub fn key(&self) -> &str { &self.key }
    pub const fn initial_center(&self) -> UsfPosition { self.initial_center }
    pub const fn conservative_radius_metres(&self) -> f64 {
        self.conservative_radius_metres
    }
    pub fn recipe(&self) -> &T { &self.recipe }

    /// Broadphase at the *requested* canonical region's scale.
    /// Numerical projection failures yield conservative candidates, not false
    /// claims of nonexistence. Callers can refine broadphase evidence later.
    pub fn may_intersect(&self, region: UsfChunkAddress) -> bool {
        let Ok(delta) = self.initial_center.relative_at_scale_bounded_f64(
            &region.center(), region.scale(), f64::MAX,
        ) else {
            return true;
        };
        let radius = self.conservative_radius_metres * region.scale().native_per_metre();
        if !delta.is_finite() || !radius.is_finite() {
            return true;
        }
        let half = f64::from(USF_CHUNK_NATIVE_SIZE) * 0.5;
        let dx = (delta.x.abs() - half).max(0.0);
        let dy = (delta.y.abs() - half).max(0.0);
        let dz = (delta.z.abs() - half).max(0.0);
        // hypot avoids overflow when evaluating large but finite scales.
        dx.hypot(dy).hypot(dz) <= radius
    }
}

/// Domain-typed known-content overlay. One record per stable identity, never
/// one record per intersected chunk or per Scale Slice.
///
/// This is a sparse source catalog, not a worldgen scheduling manager. A
/// generator may supply entries just as a human-authored scene may; downstream
/// generation can consume them as constraints without changing their identity.
#[derive(Debug)]
pub struct SparseConstructionAtlas<T> {
    entries: BTreeMap<String, AnchoredConstruction<T>>,
}

impl<T> Default for SparseConstructionAtlas<T> {
    fn default() -> Self { Self { entries: BTreeMap::new() } }
}

impl<T> SparseConstructionAtlas<T> {
    pub fn len(&self) -> usize { self.entries.len() }
    pub fn is_empty(&self) -> bool { self.entries.is_empty() }

    pub fn declare(
        &mut self,
        key: impl Into<String>,
        initial_center: UsfPosition,
        conservative_radius_metres: f64,
        recipe: T,
    ) -> Result<(), ConstructionCatalogError> {
        let key = key.into();
        if key.is_empty() { return Err(ConstructionCatalogError::EmptyIdentity); }
        if !conservative_radius_metres.is_finite() || conservative_radius_metres < 0.0 {
            return Err(ConstructionCatalogError::InvalidBound);
        }
        if self.entries.contains_key(&key) {
            return Err(ConstructionCatalogError::DuplicateIdentity);
        }
        self.entries.insert(key.clone(), AnchoredConstruction {
            key,
            initial_center,
            conservative_radius_metres,
            recipe,
        });
        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<&AnchoredConstruction<T>> {
        self.entries.get(key)
    }

    /// A canonical region can discover whole phenomena crossing its boundary.
    /// Candidates are returned in stable identity order, independent of chunk
    /// visit order, viewport distance, and how much local content is resident.
    pub fn intersecting(
        &self,
        region: UsfChunkAddress,
    ) -> impl Iterator<Item = &AnchoredConstruction<T>> + '_ {
        self.entries.values().filter(move |entry| entry.may_intersect(region))
    }

    pub fn entries(&self) -> impl ExactSizeIterator<Item = &AnchoredConstruction<T>> {
        self.entries.values()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::math::Vec3;
    use crate::usf::SpatialScale;

    #[test]
    fn one_large_fact_is_discoverable_from_both_sides_of_chunk_boundary() {
        let mut atlas = SparseConstructionAtlas::default();
        let center = UsfPosition::zero(SpatialScale::ZERO)
            .translated_native(Vec3::new(490.0, 0.0, 0.0)).unwrap();
        atlas.declare("region/bridge", center, 40.0, "bridge recipe").unwrap();
        let left = UsfChunkAddress::containing(center, SpatialScale::ZERO).unwrap();
        let right = UsfChunkAddress::containing(
            center.translated_native(Vec3::X * 40.0).unwrap(), SpatialScale::ZERO,
        ).unwrap();
        assert_ne!(left, right);
        assert_eq!(atlas.intersecting(left).count(), 1);
        assert_eq!(atlas.intersecting(right).count(), 1);
        assert_eq!(atlas.len(), 1); // No per-chunk duplicates or resident ECS.
    }

    #[test]
    fn empty_region_needs_no_phenomena_or_realizations() {
        let mut atlas = SparseConstructionAtlas::default();
        let origin = UsfPosition::zero(SpatialScale::ZERO);
        atlas.declare("here", origin, 10.0, ()).unwrap();
        let remote = UsfChunkAddress::containing(
            origin.translated_native(Vec3::X * 4_000.0).unwrap(), SpatialScale::ZERO,
        ).unwrap();
        assert_eq!(atlas.intersecting(remote).count(), 0);
        assert_eq!(atlas.len(), 1);
        assert!(matches!(atlas.declare("here", origin, 1.0, ()),
            Err(ConstructionCatalogError::DuplicateIdentity)));
    }

    #[test]
    fn coarser_context_can_discover_fine_authored_facts_without_loading_descendants() {
        let mut atlas = SparseConstructionAtlas::default();
        let origin = UsfPosition::zero(SpatialScale::MIN);
        atlas.declare("system/earth", origin, 6_371_000.0, ()).unwrap();
        atlas.declare("system/moon", origin.translated_metres_f64(
            bevy::math::DVec3::X * 384_400_000.0).unwrap(), 1_737_400.0, ()).unwrap();
        let coarse = UsfChunkAddress::containing(origin, SpatialScale::new(8).unwrap()).unwrap();
        assert_eq!(atlas.intersecting(coarse).count(), 2);
        assert_eq!(atlas.len(), 2);
    }
}
