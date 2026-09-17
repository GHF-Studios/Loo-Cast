//! Sparse authoritative voxel world with dense materialized chunk caches.

use std::collections::HashMap;

use bevy::prelude::{Component, Entity, IVec3, Vec3};

use crate::spatial::{UsfPosition, UsfPositionError};

use super::{
    MATERIALIZATION_CHUNK_SIZE, VoxelBase, VoxelBounds, VoxelChunk, VoxelEdit,
    VoxelModificationLayer, VoxelQueryPosition, VoxelSample, chunk::SAMPLE_PADDING,
};

/// Transitional coordinate in the original `VoxelWorld`-local sampling lattice.
///
/// Pass B no longer uses this as semantic identity, edit/query authority,
/// streaming identity, or generation input. It remains only as a compact
/// compatibility adapter for authored/tests that still describe a nearby grid
/// offset from [`VoxelWorld::origin`].
#[doc(hidden)]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VoxelChunkCoord(pub IVec3);

impl VoxelChunkCoord {
    pub const fn new(value: IVec3) -> Self {
        Self(value)
    }

    pub fn origin(self) -> IVec3 {
        self.0 * MATERIALIZATION_CHUNK_SIZE as i32
    }

    /// Exact displacement from this voxel world's canonical origin in whole
    /// leaf-native units. This compatibility path never squeezes the semantic
    /// address through one large floating-point vector.
    pub fn native_offset(self) -> [i64; 3] {
        let size = i64::from(MATERIALIZATION_CHUNK_SIZE);
        [
            i64::from(self.0.x) * size,
            i64::from(self.0.y) * size,
            i64::from(self.0.z) * size,
        ]
    }

    /// Local compatibility chunk containing one nearby scale-0 point.
    pub fn containing(point: Vec3) -> Self {
        Self(
            (point / MATERIALIZATION_CHUNK_SIZE as f32)
                .floor()
                .as_ivec3(),
        )
    }
}

/// Canonical semantic address of one decimal `10³` voxel materialization chunk.
///
/// The address is sparse/virtual: constructing it allocates no dense voxel data,
/// entity, mesh, collider, or hierarchy node. It is representation identity, not
/// a USF Chunk and not a runtime Bevy/Avian coordinate.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VoxelMaterializationChunkAddress {
    origin: UsfPosition,
}

impl VoxelMaterializationChunkAddress {
    pub const fn new(origin: UsfPosition) -> Self {
        Self { origin }
    }

    pub const fn origin(&self) -> &UsfPosition {
        &self.origin
    }

    pub const fn query_origin(self) -> VoxelQueryPosition {
        VoxelQueryPosition::new(self.origin)
    }

    pub fn translated_chunks(self, delta: IVec3) -> Result<Self, UsfPositionError> {
        let size = i64::from(MATERIALIZATION_CHUNK_SIZE);
        self.origin
            .translated_whole_native([
                i64::from(delta.x) * size,
                i64::from(delta.y) * size,
                i64::from(delta.z) * size,
            ])
            .map(Self::new)
    }

    /// Canonical scope of every lattice sample physically stored by this base
    /// materialization, including private Surface Nets neighbor padding.
    pub fn sample_bounds(self) -> VoxelBounds {
        VoxelBounds::new(
            self.query_origin(),
            Vec3::splat(-(SAMPLE_PADDING as f32)),
            Vec3::splat(MATERIALIZATION_CHUNK_SIZE as f32),
        )
    }
}

/// Compatibility alias for the M7.1a name.
#[doc(hidden)]
pub type VoxelChunkAddress = VoxelMaterializationChunkAddress;

/// Immutable background-generation recipe for one dense local chunk.
///
/// The recipe contains canonical semantic address + semantic edits only. No
/// runtime frame coordinate or giant flat voxel lattice survives into worker
/// generation.
#[derive(Debug, Clone)]
pub(crate) struct VoxelChunkRecipe {
    address: VoxelMaterializationChunkAddress,
    world_origin: VoxelQueryPosition,
    base: VoxelBase,
    edits: Vec<VoxelEdit>,
    applied_edit_count: usize,
}

impl VoxelChunkRecipe {
    pub(crate) const fn applied_edit_count(&self) -> usize {
        self.applied_edit_count
    }

    pub(crate) fn materialize(self) -> VoxelChunk {
        let Self {
            address,
            world_origin,
            base,
            edits,
            ..
        } = self;
        let anchor = address.query_origin();
        let sampler = base.prepare_chunk_sampler(world_origin, anchor);
        let extra_extent = MATERIALIZATION_CHUNK_SIZE as f32 + SAMPLE_PADDING as f32;
        let local_edits = edits
            .into_iter()
            .filter_map(|edit| edit.localized(anchor, extra_extent))
            .collect::<Vec<_>>();

        VoxelChunk::generate(move |local_point| {
            let mut sample = sampler.sample(local_point);
            for edit in &local_edits {
                sample = edit.apply_to_sample(local_point, sample);
            }
            sample
        })
    }
}

/// Semantic voxel-world root.
///
/// The authoritative state is `base + modifications`. `chunks` only indexes
/// currently reserved canonical materialization addresses: some may still be
/// generating, while others hold dense disposable working caches. Destroying
/// every dense cache therefore does not destroy the world.
#[derive(Component, Debug)]
pub struct VoxelWorld {
    origin: UsfPosition,
    base: VoxelBase,
    modifications: VoxelModificationLayer,
    chunks: HashMap<VoxelMaterializationChunkAddress, Entity>,
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
            chunks: HashMap::new(),
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
        self.modifications.push(edit, addresses);
        Ok(())
    }

    /// Captures immutable canonical generation input for one chunk. This is
    /// deliberately cheap relative to dense generation: procedural bases are
    /// compact and only edits indexed for this semantic address are copied.
    pub(crate) fn chunk_recipe(
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

    pub fn insert_chunk(
        &mut self,
        address: VoxelMaterializationChunkAddress,
        entity: Entity,
    ) -> Option<Entity> {
        self.chunks.insert(address, entity)
    }

    pub fn remove_chunk(&mut self, address: VoxelMaterializationChunkAddress) -> Option<Entity> {
        self.chunks.remove(&address)
    }

    pub fn chunk_entity(&self, address: VoxelMaterializationChunkAddress) -> Option<Entity> {
        self.chunks.get(&address).copied()
    }

    pub fn chunk_entities(&self) -> impl Iterator<Item = Entity> + '_ {
        self.chunks.values().copied()
    }

    pub fn chunk_entries(
        &self,
    ) -> impl Iterator<Item = (VoxelMaterializationChunkAddress, Entity)> + '_ {
        self.chunks
            .iter()
            .map(|(&address, &entity)| (address, entity))
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

    /// Reserved dense-cache entities intersecting a canonical semantic scope.
    pub fn chunks_intersecting(
        &self,
        bounds: VoxelBounds,
    ) -> Result<Vec<Entity>, UsfPositionError> {
        Ok(self
            .materialization_addresses_intersecting(bounds)?
            .into_iter()
            .filter_map(|address| self.chunk_entity(address))
            .collect())
    }

    pub fn len(&self) -> usize {
        self.chunks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.chunks.is_empty()
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

/// Identifies the semantic voxel world owning one root-level materialization
/// entity. Canonical location lives in the separate address component; dense
/// voxel samples and render/physics geometry remain chunk-local.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct VoxelChunkOf {
    pub world: Entity,
}

impl VoxelChunkOf {
    pub const fn new(world: Entity) -> Self {
        Self { world }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        spatial::SpatialScale,
        voxel::{VoxelBrush, VoxelMaterialId, VoxelSample},
    };

    fn query(local: Vec3) -> VoxelQueryPosition {
        VoxelQueryPosition::from_scale0_local(local).unwrap()
    }

    #[test]
    fn compatibility_chunk_coordinates_are_euclidean_grid_offsets() {
        assert_eq!(
            VoxelChunkCoord::new(IVec3::new(-1, 2, 0)).origin(),
            IVec3::new(
                -(MATERIALIZATION_CHUNK_SIZE as i32),
                2 * MATERIALIZATION_CHUNK_SIZE as i32,
                0,
            )
        );
        assert_eq!(
            VoxelChunkCoord::containing(Vec3::new(-0.01, 0.0, 9.99)),
            VoxelChunkCoord::new(IVec3::new(-1, 0, 0))
        );
    }

    #[test]
    fn materialization_address_is_canonical_across_semantic_region_boundaries() {
        let world = VoxelWorld::new(VoxelBase::Empty);
        let address = world
            .chunk_address(VoxelChunkCoord::new(IVec3::new(128, 0, 0)))
            .unwrap();

        assert_eq!(address.origin().digit(SpatialScale::ZERO).x, 1);
        assert_eq!(address.origin().offset().x, 280.0);
    }

    #[test]
    fn containing_address_crosses_usf_carry_without_flat_grid_identity() {
        let origin = UsfPosition::from_scale0_local(Vec3::new(499.0, 0.0, 0.0)).unwrap();
        let world = VoxelWorld::new_at(VoxelBase::Empty, origin);
        let point =
            VoxelQueryPosition::new(origin.translated_native(Vec3::new(12.0, 0.0, 0.0)).unwrap());
        let address = world.materialization_address_containing(point).unwrap();

        assert_eq!(
            point.relative_to(address.query_origin(), 10.0).unwrap(),
            Vec3::new(2.0, 0.0, 0.0)
        );
    }

    #[test]
    fn canonical_addresses_do_not_collapse_through_large_f32_coordinates() {
        let world = VoxelWorld::new(VoxelBase::Empty);
        let left = world
            .chunk_address(VoxelChunkCoord::new(IVec3::new(2_000_000_000, 0, 0)))
            .unwrap();
        let right = world
            .chunk_address(VoxelChunkCoord::new(IVec3::new(2_000_000_001, 0, 0)))
            .unwrap();

        assert_eq!(20_000_000_000_i64 as f32, 20_000_000_010_i64 as f32);
        assert_ne!(left, right);
    }

    #[test]
    fn edit_bounds_address_both_padded_chunks_at_a_seam() {
        let world = VoxelWorld::new(VoxelBase::Empty);
        let center = query(Vec3::new(MATERIALIZATION_CHUNK_SIZE as f32, 5.0, 5.0));
        let edit = VoxelEdit::Add {
            brush: VoxelBrush::sphere(center, 0.5),
            material: VoxelMaterialId::ROCK,
        };
        let addresses = world
            .materialization_addresses_intersecting(edit.influence_bounds())
            .unwrap();
        let left = world
            .chunk_address(VoxelChunkCoord::new(IVec3::ZERO))
            .unwrap();
        let right = world.chunk_address(VoxelChunkCoord::new(IVec3::X)).unwrap();

        assert!(addresses.contains(&left));
        assert!(addresses.contains(&right));
    }

    #[test]
    fn neighbor_chunks_store_identical_overlap_after_cross_boundary_edit() {
        let world = VoxelWorld::new(VoxelBase::Empty);
        let left_address = world
            .chunk_address(VoxelChunkCoord::new(IVec3::ZERO))
            .unwrap();
        let right_address = world.chunk_address(VoxelChunkCoord::new(IVec3::X)).unwrap();
        let mut left = VoxelChunk::generate(|_| VoxelSample::empty(100.0));
        let mut right = VoxelChunk::generate(|_| VoxelSample::empty(100.0));
        let edit = VoxelEdit::Add {
            brush: VoxelBrush::sphere(
                query(Vec3::new(MATERIALIZATION_CHUNK_SIZE as f32, 5.0, 5.0)),
                3.0,
            ),
            material: VoxelMaterialId::ROCK,
        };

        assert!(
            left_address
                .sample_bounds()
                .intersects(edit.influence_bounds())
        );
        assert!(
            right_address
                .sample_bounds()
                .intersects(edit.influence_bounds())
        );
        left.apply_edit(left_address, edit);
        right.apply_edit(right_address, edit);

        for world_x in [
            MATERIALIZATION_CHUNK_SIZE as i32 - 1,
            MATERIALIZATION_CHUNK_SIZE as i32,
        ] {
            for y in 2..=8 {
                for z in 2..=8 {
                    let left_point = IVec3::new(world_x, y, z);
                    let right_point = IVec3::new(world_x - MATERIALIZATION_CHUNK_SIZE as i32, y, z);
                    assert_eq!(left.sample(left_point), right.sample(right_point));
                }
            }
        }
    }

    #[test]
    fn sparse_edits_survive_chunk_rematerialization() {
        let world_origin = UsfPosition::default();
        let center = query(Vec3::splat(8.0));
        let mut world = VoxelWorld::new_at(
            VoxelBase::sphere(center, 6.0, VoxelMaterialId::ROCK),
            world_origin,
        );
        let address = world
            .chunk_address(VoxelChunkCoord::new(IVec3::ZERO))
            .unwrap();
        let point = IVec3::splat(8);

        assert!(
            world
                .materialize_chunk(address)
                .sample(point)
                .unwrap()
                .distance
                .is_solid()
        );

        world
            .record_edit(VoxelEdit::Remove {
                brush: VoxelBrush::sphere(center, 2.0),
            })
            .unwrap();

        let rebuilt = world.materialize_chunk(address);
        assert!(rebuilt.sample(point).unwrap().distance.is_empty());
        assert_eq!(world.modifications().len(), 1);
    }
}
