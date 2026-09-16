//! Sparse authoritative voxel world with dense materialized chunk caches.

use std::collections::HashMap;

use bevy::prelude::{Component, Entity, IVec3, Vec3};

use crate::spatial::{UsfPosition, UsfPositionError};

use super::{
    CHUNK_SIZE, SAMPLE_PADDING, VoxelBase, VoxelBounds, VoxelChunk, VoxelEdit,
    VoxelModificationLayer, VoxelSample,
};

/// Integer coordinate of a logical chunk in the voxel-world grid.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VoxelChunkCoord(pub IVec3);

impl VoxelChunkCoord {
    pub const fn new(value: IVec3) -> Self {
        Self(value)
    }

    pub fn origin(self) -> IVec3 {
        self.0 * CHUNK_SIZE as i32
    }

    /// Chunk containing one world-space position. Uses floor semantics so
    /// negative coordinates map to the expected Euclidean grid cell.
    pub fn containing(point: Vec3) -> Self {
        Self((point / CHUNK_SIZE as f32).floor().as_ivec3())
    }

    /// Bounds of the physical sample allocation belonging to this chunk,
    /// including the one-sample neighbor border.
    pub fn sample_bounds(self) -> VoxelBounds {
        let origin = self.origin().as_vec3();
        VoxelBounds::new(
            origin - Vec3::splat(SAMPLE_PADDING as f32),
            origin + Vec3::splat(CHUNK_SIZE as f32),
        )
    }
}


/// Canonical semantic origin of one dense voxel brick.
///
/// The current [`VoxelChunkCoord`] remains a small world-local cache key. This
/// component gives the materialized brick an origin in USF semantic space so
/// projection can evolve independently from the storage lattice.
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct VoxelChunkAddress {
    origin: UsfPosition,
}

impl VoxelChunkAddress {
    pub const fn new(origin: UsfPosition) -> Self {
        Self { origin }
    }

    pub const fn origin(&self) -> &UsfPosition {
        &self.origin
    }
}

/// Immutable background-generation recipe for one dense chunk.
///
/// The recipe owns exactly the edits that existed when it was created. The
/// completion path can then replay edits appended after `applied_edit_count`
/// before publishing the generated chunk.
#[derive(Debug, Clone)]
pub(crate) struct VoxelChunkRecipe {
    coord: VoxelChunkCoord,
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
            coord,
            base,
            edits,
            ..
        } = self;

        VoxelChunk::generate(coord.origin(), move |point| {
            let mut sample = base.sample(point);
            for edit in &edits {
                sample = edit.apply_to_sample(point, sample);
            }
            sample
        })
    }
}

/// Semantic voxel-world root.
///
/// The authoritative state is `base + modifications`. `chunks` only indexes
/// currently reserved chunk-coordinate entities: some may still be generating,
/// while others hold dense working caches for rendering/query code. Destroying
/// every one of them therefore does not destroy the world.
#[derive(Component, Debug)]
pub struct VoxelWorld {
    origin: UsfPosition,
    base: VoxelBase,
    modifications: VoxelModificationLayer,
    chunks: HashMap<VoxelChunkCoord, Entity>,
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

    pub fn chunk_address(
        &self,
        coord: VoxelChunkCoord,
    ) -> Result<VoxelChunkAddress, UsfPositionError> {
        self.origin
            .translated_native(coord.origin().as_vec3())
            .map(VoxelChunkAddress::new)
    }

    pub const fn base(&self) -> VoxelBase {
        self.base
    }

    pub fn modifications(&self) -> &VoxelModificationLayer {
        &self.modifications
    }

    /// Records one authoritative world edit.
    ///
    /// Callers should also apply it to intersecting materialized chunks so the
    /// active cache reflects the new state immediately. Future rematerialization
    /// will replay this record automatically.
    pub fn record_edit(&mut self, edit: VoxelEdit) {
        self.modifications.push(edit);
    }

    /// Captures immutable generation input for one chunk. This is deliberately
    /// cheap relative to dense generation: procedural bases are compact and only
    /// edits intersecting the chunk's padded sample domain are copied.
    pub(crate) fn chunk_recipe(&self, coord: VoxelChunkCoord) -> VoxelChunkRecipe {
        VoxelChunkRecipe {
            coord,
            base: self.base,
            edits: self.modifications.for_chunk(coord).collect(),
            applied_edit_count: self.modifications.len(),
        }
    }

    /// Reconstructs one dense working chunk from procedural base + sparse edits.
    /// Synchronous callers (currently the small authored playground fixture and
    /// tests) share exactly the same recipe used by streaming background tasks.
    pub fn materialize_chunk(&self, coord: VoxelChunkCoord) -> VoxelChunk {
        self.chunk_recipe(coord).materialize()
    }

    /// Resolves one arbitrary sample without requiring a materialized chunk.
    pub fn resolve_sample(&self, point: Vec3) -> VoxelSample {
        let mut sample = self.base.sample(point);
        let coord = VoxelChunkCoord::containing(point);
        for edit in self.modifications.for_chunk(coord) {
            if edit.influence_bounds().contains(point) {
                sample = edit.apply_to_sample(point, sample);
            }
        }
        sample
    }

    pub fn insert_chunk(&mut self, coord: VoxelChunkCoord, entity: Entity) -> Option<Entity> {
        self.chunks.insert(coord, entity)
    }

    pub fn remove_chunk(&mut self, coord: VoxelChunkCoord) -> Option<Entity> {
        self.chunks.remove(&coord)
    }

    pub fn chunk_entity(&self, coord: VoxelChunkCoord) -> Option<Entity> {
        self.chunks.get(&coord).copied()
    }

    pub fn chunk_entities(&self) -> impl Iterator<Item = Entity> + '_ {
        self.chunks.values().copied()
    }

    pub fn chunk_entries(&self) -> impl Iterator<Item = (VoxelChunkCoord, Entity)> + '_ {
        self.chunks.iter().map(|(&coord, &entity)| (coord, entity))
    }

    /// Returns reserved chunk entities whose stored sample domains would
    /// intersect the finite influence bounds of an edit. Callers that require a
    /// dense cache can simply query for [`VoxelChunk`] and skip pending entities.
    pub fn chunks_intersecting(&self, bounds: VoxelBounds) -> Vec<Entity> {
        let (minimum, maximum) = chunk_coord_range(bounds);
        let mut entities = Vec::new();

        for z in minimum.z..=maximum.z {
            for y in minimum.y..=maximum.y {
                for x in minimum.x..=maximum.x {
                    let coord = VoxelChunkCoord::new(IVec3::new(x, y, z));
                    if let Some(entity) = self.chunk_entity(coord) {
                        entities.push(entity);
                    }
                }
            }
        }

        entities
    }

    pub fn len(&self) -> usize {
        self.chunks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.chunks.is_empty()
    }
}

pub(crate) fn chunk_coord_range(bounds: VoxelBounds) -> (IVec3, IVec3) {
    let size = CHUNK_SIZE as f32;
    let padding = SAMPLE_PADDING as f32;

    // A chunk at origin O physically stores lattice samples in
    // [O - padding, O + CHUNK_SIZE]. Solve that interval intersection against
    // the edit bounds to obtain the inclusive coordinate range.
    let minimum = ((bounds.min - Vec3::splat(size)) / size)
        .ceil()
        .as_ivec3();
    let maximum = ((bounds.max + Vec3::splat(padding)) / size)
        .floor()
        .as_ivec3();
    (minimum, maximum)
}

/// Identifies a reserved/materialized chunk coordinate belonging to a voxel world.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct VoxelChunkOf {
    pub world: Entity,
    pub coord: VoxelChunkCoord,
}

impl VoxelChunkOf {
    pub const fn new(world: Entity, coord: VoxelChunkCoord) -> Self {
        Self { world, coord }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        spatial::SpatialScale,
        voxel::{VoxelBrush, VoxelMaterialId, VoxelSample},
    };

    #[test]
    fn chunk_coordinates_are_euclidean_grid_coordinates() {
        assert_eq!(
            VoxelChunkCoord::new(IVec3::new(-1, 2, 0)).origin(),
            IVec3::new(-(CHUNK_SIZE as i32), 2 * CHUNK_SIZE as i32, 0)
        );
        assert_eq!(
            VoxelChunkCoord::containing(Vec3::new(-0.01, 0.0, 31.99)),
            VoxelChunkCoord::new(IVec3::new(-1, 0, 0))
        );
        assert_eq!(
            VoxelChunkCoord::containing(Vec3::new(32.0, 0.0, 32.0)),
            VoxelChunkCoord::new(IVec3::new(1, 0, 1))
        );
    }

    #[test]
    fn brick_address_is_canonical_across_semantic_region_boundaries() {
        let world = VoxelWorld::new(VoxelBase::Empty);
        let address = world
            .chunk_address(VoxelChunkCoord::new(IVec3::new(40, 0, 0)))
            .unwrap();

        // 40 * 32 m = 1280 m, represented canonically as one S0 chunk digit
        // plus a bounded +280 m leaf offset rather than a giant runtime Vec3.
        assert_eq!(address.origin().digit(SpatialScale::ZERO).x, 1);
        assert_eq!(address.origin().offset().x, 280.0);
    }

    #[test]
    fn edit_bounds_address_both_chunks_at_a_seam() {
        let bounds = VoxelBounds::new(
            Vec3::new(CHUNK_SIZE as f32 - 0.5, 8.0, 8.0),
            Vec3::new(CHUNK_SIZE as f32 + 0.5, 9.0, 9.0),
        );
        let (minimum, maximum) = chunk_coord_range(bounds);

        assert_eq!(minimum.x, 0);
        assert_eq!(maximum.x, 1);
    }

    #[test]
    fn neighbor_chunks_store_identical_overlap_after_cross_boundary_edit() {
        let mut left = VoxelChunk::generate(IVec3::ZERO, |_| VoxelSample::empty(100.0));
        let mut right = VoxelChunk::generate(IVec3::X * CHUNK_SIZE as i32, |_| {
            VoxelSample::empty(100.0)
        });
        let edit = VoxelEdit::Add {
            brush: VoxelBrush::sphere(Vec3::new(CHUNK_SIZE as f32, 16.0, 16.0), 3.0),
            material: VoxelMaterialId::ROCK,
        };

        assert!(left.sample_bounds().intersects(edit.influence_bounds()));
        assert!(right.sample_bounds().intersects(edit.influence_bounds()));
        left.apply_edit(edit);
        right.apply_edit(edit);

        for x in [CHUNK_SIZE as i32 - 1, CHUNK_SIZE as i32] {
            for y in 13..=19 {
                for z in 13..=19 {
                    let point = IVec3::new(x, y, z);
                    assert_eq!(left.sample(point), right.sample(point));
                }
            }
        }
    }

    #[test]
    fn sparse_edits_survive_chunk_rematerialization() {
        let coord = VoxelChunkCoord::new(IVec3::ZERO);
        let mut world = VoxelWorld::new(VoxelBase::sphere(
            Vec3::splat(8.0),
            6.0,
            VoxelMaterialId::ROCK,
        ));
        let point = IVec3::splat(8);

        assert!(world.materialize_chunk(coord).sample(point).unwrap().distance.is_solid());

        world.record_edit(VoxelEdit::Remove {
            brush: VoxelBrush::sphere(point.as_vec3(), 2.0),
        });

        // No old chunk state participates in this reconstruction. The edit is
        // authoritative independently from materialized dense storage.
        let rebuilt = world.materialize_chunk(coord);
        assert!(rebuilt.sample(point).unwrap().distance.is_empty());
        assert_eq!(world.modifications().len(), 1);
    }
}
