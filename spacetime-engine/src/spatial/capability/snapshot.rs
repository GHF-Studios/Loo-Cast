//! Persistent index of realized capability facts.

use super::*;

/// Persistent snapshot of currently realized capability coverage.
///
/// The snapshot is reconciled from live [`UsfCapabilityRealization`] components
/// after capability publication. A realization that disappears automatically
/// disappears from coverage at the next reconciliation.
#[derive(Resource, Debug, Default)]
pub struct UsfScaleCoverageSnapshot {
    revision: u64,
    entries: Vec<UsfScaleCoverage>,
    by_authority_scale: HashMap<(Entity, SpatialScale), Vec<usize>>,
}

impl UsfScaleCoverageSnapshot {
    /// Iterate only facts indexed for one semantic authority and Scale Slice.
    /// The index is derived from `entries` during reconciliation.
    fn authority_scale_entries(
        &self,
        authority: Entity,
        scale: SpatialScale,
    ) -> impl Iterator<Item = UsfScaleCoverage> + '_ {
        self.by_authority_scale
            .get(&(authority, scale))
            .into_iter()
            .flatten()
            .map(|&index| self.entries[index])
    }

    pub const fn revision(&self) -> u64 {
        self.revision
    }

    pub fn iter(&self) -> impl ExactSizeIterator<Item = UsfScaleCoverage> + '_ {
        self.entries.iter().copied()
    }

    pub fn apertures(
        &self,
        required: UsfScaleRoleMask,
    ) -> impl Iterator<Item = UsfRefinementAperture> + '_ {
        self.entries
            .iter()
            .copied()
            .filter(move |coverage| coverage.roles().contains(required))
            .filter_map(UsfRefinementAperture::from_coverage)
    }

    pub fn has_near(
        &self,
        scale: SpatialScale,
        point: &UsfPosition,
        required: UsfScaleRoleMask,
        radius_native: f32,
    ) -> bool {
        self.entries.iter().copied().any(|coverage| {
            coverage.scale() == scale && coverage.is_within(point, required, radius_native)
        })
    }

    pub fn has_near_for_authority(
        &self,
        authority: Entity,
        scale: SpatialScale,
        point: &UsfPosition,
        required: UsfScaleRoleMask,
        radius_native: f32,
    ) -> bool {
        self.authority_scale_entries(authority, scale)
            .any(|coverage| coverage.is_within(point, required, radius_native))
    }

    /// Returns whether one authority has realized `required` capability inside
    /// exactly this canonical context.
    ///
    /// Refinement ancestry is topological, not geometric. A coarse realization
    /// may approximate a fine surface hundreds of native units away while still
    /// being the correct canonical parent branch. Consumers that need
    /// parent-before-child ordering should query context identity rather than
    /// physical proximity between two different-resolution surfaces.
    pub fn has_in_context_for_authority(
        &self,
        authority: Entity,
        context: UsfChunkAddress,
        required: UsfScaleRoleMask,
    ) -> bool {
        let scale = context.scale();

        self.authority_scale_entries(authority, scale)
            .any(|coverage| {
                coverage.roles().contains(required)
                    && UsfChunkAddress::containing(coverage.center(), scale)
                        .is_ok_and(|coverage_context| coverage_context == context)
            })
    }

    pub(super) fn reconcile(&mut self, mut next: Vec<UsfScaleCoverage>) {
        next.sort_by_key(|coverage| coverage.realization().to_bits());

        if self.entries != next {
            let mut by_authority_scale = HashMap::<(Entity, SpatialScale), Vec<usize>>::new();
            for (index, coverage) in next.iter().copied().enumerate() {
                by_authority_scale
                    .entry((coverage.authority(), coverage.scale()))
                    .or_default()
                    .push(index);
            }

            self.entries = next;
            self.by_authority_scale = by_authority_scale;
            self.revision = self.revision.wrapping_add(1).max(1);
        }
    }
}
