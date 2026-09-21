//! Sparse authoritative voxel world with store-owned materialization caches.

use bevy::prelude::{Component, IVec3, Vec3};

use crate::spatial::{SpatialScale, UsfPosition, UsfPositionError};

use super::{
    MATERIALIZATION_CHUNK_SIZE, VoxelAuthority, VoxelBase, VoxelBounds, VoxelChunk,
    VoxelEdit,
    VoxelModificationLayer, VoxelQueryPosition, VoxelSample, chunk::SAMPLE_PADDING,
    store::VoxelMaterializationStore,
};

mod address;
mod recipe;

pub use address::{
    VoxelChunkAddress, VoxelChunkCoord, VoxelMaterializationChunkAddress,
};
pub(in crate::voxel) use recipe::VoxelChunkRecipe;

/// Semantic voxel-world root.
///
/// The authoritative state is `base + modifications`. Materialization residency
/// is compact ordinary Rust data owned by this world; a canonical atom does not
/// need an ECS entity, transform, collider, or mesh to be resident.
#[derive(Component, Debug)]
pub struct VoxelWorld {
    origin: UsfPosition,
    base: VoxelBase,
    modifications: VoxelModificationLayer,
    materializations: VoxelMaterializationStore,
}

impl Default for VoxelWorld {
    fn default() -> Self {
        Self::new(VoxelBase::default())
    }
}

impl VoxelWorld {
    pub fn new(base: VoxelBase) -> Self {
        Self::new_at(base, UsfPosition::default())
    }

    pub fn new_at(base: VoxelBase, origin: UsfPosition) -> Self {
        Self {
            origin,
            base,
            modifications: VoxelModificationLayer::default(),
            materializations: VoxelMaterializationStore::default(),
        }
    }

    pub const fn origin(&self) -> &UsfPosition {
        &self.origin
    }

    /// Compatibility adapter from a nearby bounded lattice offset. Core Pass-B
    /// code addresses materializations directly with canonical addresses.
    pub fn chunk_address(
        &self,
        coord: VoxelChunkCoord,
    ) -> Result<VoxelMaterializationChunkAddress, UsfPositionError> {
        self.origin
            .translated_whole_native(coord.native_offset())
            .map(VoxelMaterializationChunkAddress::new)
    }

    /// Returns the decimal base materialization containing a canonical point.
    ///
    /// Materialization alignment is relative to this world's canonical origin.
    /// USF digits represent multiples of 1000 leaf-native units, so for a 10-unit
    /// base extent only the bounded leaf offsets are needed to recover grid phase.
    pub fn materialization_address_containing(
        &self,
        point: VoxelQueryPosition,
    ) -> Result<VoxelMaterializationChunkAddress, UsfPositionError> {
        if point.usf().leaf_scale() != self.origin.leaf_scale() {
            return Err(UsfPositionError::IncompatibleLeafScale);
        }

        let size = MATERIALIZATION_CHUNK_SIZE as f32;
        let delta = point.usf().offset() - self.origin.offset();
        let remainder = Vec3::new(
            delta.x.rem_euclid(size),
            delta.y.rem_euclid(size),
            delta.z.rem_euclid(size),
        );
        point
            .translated(-remainder)
            .map(|origin| VoxelMaterializationChunkAddress::new(origin.usf()))
    }

    pub const fn base(&self) -> VoxelBase {
        self.base
    }

    pub fn modifications(&self) -> &VoxelModificationLayer {
        &self.modifications
    }

    /// Records one authoritative semantic edit and indexes it by the canonical
    /// base materialization scopes whose padded sample domains it can affect.
    pub fn record_edit(&mut self, edit: VoxelEdit) -> Result<(), UsfPositionError> {
        let addresses = self.materialization_addresses_intersecting(edit.influence_bounds())?;
        self.modifications.push(edit, addresses.iter().copied());
        for address in addresses {
            self.materializations.apply_edit(address, edit);
        }
        Ok(())
    }

    /// Captures immutable canonical generation input for one chunk. This is
    /// deliberately cheap relative to dense generation: procedural bases are
    /// compact and only edits indexed for this semantic address are copied.
    pub(in crate::voxel) fn chunk_recipe(
        &self,
        address: VoxelMaterializationChunkAddress,
    ) -> VoxelChunkRecipe {
        VoxelChunkRecipe {
            address,
            world_origin: VoxelQueryPosition::new(self.origin),
            base: self.base,
            edits: self.modifications.for_chunk(address).collect(),
            applied_edit_count: self.modifications.len(),
        }
    }

    /// Captures generation input from one shared semantic authority.
    ///
    /// Canonical edits are currently authored at S0. Fine S0 realizations localize
    /// those edits directly. Coarser realizations intentionally ignore them until
    /// a real coarse edit-aggregation policy exists.
    pub(in crate::voxel) fn chunk_recipe_from_authority(
        &self,
        address: VoxelMaterializationChunkAddress,
        authority: &VoxelAuthority,
    ) -> VoxelChunkRecipe {
        let edits = if self.origin.leaf_scale() == SpatialScale::ZERO {
            let extra_extent = MATERIALIZATION_CHUNK_SIZE as f32 + SAMPLE_PADDING as f32;
            authority
                .edits()
                .iter()
                .copied()
                .filter(|edit| {
                    edit.localized(address.query_origin(), extra_extent)
                        .is_some()
                })
                .collect()
        } else {
            Vec::new()
        };

        VoxelChunkRecipe {
            address,
            world_origin: VoxelQueryPosition::new(self.origin),
            base: self.base,
            edits,
            applied_edit_count: authority.len(),
        }
    }

    /// Applies a newly-recorded shared-authority edit to resident caches without
    /// duplicating it in this realization's inline modification log.
    pub(crate) fn apply_authority_edit(
        &mut self,
        edit: VoxelEdit,
    ) -> Result<(), UsfPositionError> {
        if edit.influence_bounds().anchor().usf().leaf_scale() != self.origin.leaf_scale() {
            return Ok(());
        }

        let addresses = self.materialization_addresses_intersecting(edit.influence_bounds())?;
        for address in addresses {
            self.materializations.apply_edit(address, edit);
        }
        Ok(())
    }

    /// Reconstructs one dense chunk-local working cache from canonical semantic
    /// base + sparse edits.
    pub fn materialize_chunk(&self, address: VoxelMaterializationChunkAddress) -> VoxelChunk {
        self.chunk_recipe(address).materialize()
    }

    /// Resolves one arbitrary canonical sample without requiring a materialized
    /// dense chunk.
    pub fn resolve_sample(
        &self,
        point: VoxelQueryPosition,
    ) -> Result<VoxelSample, UsfPositionError> {
        let mut sample = self
            .base
            .sample_in_world(VoxelQueryPosition::new(self.origin), point);
        let address = self.materialization_address_containing(point)?;
        for edit in self.modifications.for_chunk(address) {
            if edit.influence_bounds().contains(point) {
                sample = edit.apply_to_sample(point, sample);
            }
        }
        Ok(sample)
    }

    pub(in crate::voxel) const fn materializations(&self) -> &VoxelMaterializationStore {
        &self.materializations
    }

    pub(in crate::voxel) fn materializations_mut(&mut self) -> &mut VoxelMaterializationStore {
        &mut self.materializations
    }

    pub fn len(&self) -> usize {
        self.materializations.active_count()
    }

    pub fn is_empty(&self) -> bool {
        self.materializations.active_count() == 0
    }

    /// Active dense materializations exposed for gameplay adapters that need
    /// direct access to currently realized voxel data.
    pub fn active_dense_materializations(
        &self,
    ) -> impl Iterator<Item = (VoxelMaterializationChunkAddress, &VoxelChunk)> + '_ {
        self.materializations.active_dense_entries()
    }

    /// Inserts one already-materialized active chunk.
    ///
    /// This is primarily useful for authored/test worlds that construct a
    /// bounded voxel realization synchronously instead of using streaming.
    pub fn insert_active_materialization(
        &mut self,
        address: VoxelMaterializationChunkAddress,
        chunk: VoxelChunk,
    ) {
        self.materializations.insert_dense_active(address, chunk);
    }

    /// Canonical base materialization addresses whose padded sample domains
    /// intersect a finite semantic voxel scope.
    pub fn materialization_addresses_intersecting(
        &self,
        bounds: VoxelBounds,
    ) -> Result<Vec<VoxelMaterializationChunkAddress>, UsfPositionError> {
        let reference = self.materialization_address_containing(bounds.anchor())?;
        let size = MATERIALIZATION_CHUNK_SIZE as f32;
        let padding = SAMPLE_PADDING as f32;
        let anchor_local = bounds
            .anchor()
            .relative_to(reference.query_origin(), size + 0.01)?;
        let local_min = anchor_local + bounds.min_offset();
        let local_max = anchor_local + bounds.max_offset();

        // A chunk at local offset O stores samples in [O-padding, O+size].
        // Solve that bounded interval intersection without ever constructing a
        // universe-wide chunk coordinate.
        let minimum = checked_ivec3(((local_min - Vec3::splat(size)) / size).ceil())?;
        let maximum = checked_ivec3(((local_max + Vec3::splat(padding)) / size).floor())?;
        if minimum.cmpgt(maximum).any() {
            return Ok(Vec::new());
        }
        let extent_x = i64::from(maximum.x) - i64::from(minimum.x) + 1;
        let extent_y = i64::from(maximum.y) - i64::from(minimum.y) + 1;
        let extent_z = i64::from(maximum.z) - i64::from(minimum.z) + 1;
        let capacity = extent_x
            .checked_mul(extent_y)
            .and_then(|value| value.checked_mul(extent_z))
            .and_then(|value| usize::try_from(value).ok())
            .ok_or(UsfPositionError::TranslationTooLarge)?;
        let mut addresses = Vec::with_capacity(capacity);

        for z in minimum.z..=maximum.z {
            for y in minimum.y..=maximum.y {
                for x in minimum.x..=maximum.x {
                    addresses.push(reference.translated_chunks(IVec3::new(x, y, z))?);
                }
            }
        }

        Ok(addresses)
    }
}

fn checked_ivec3(value: Vec3) -> Result<IVec3, UsfPositionError> {
    fn component(value: f32) -> Result<i32, UsfPositionError> {
        let value64 = f64::from(value);
        if !value.is_finite() || value64 < i32::MIN as f64 || value64 > i32::MAX as f64 {
            Err(UsfPositionError::TranslationTooLarge)
        } else {
            Ok(value as i32)
        }
    }

    Ok(IVec3::new(
        component(value.x)?,
        component(value.y)?,
        component(value.z)?,
    ))
}

#[cfg(test)]
mod tests;
