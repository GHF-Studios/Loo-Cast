//! Sparse authoritative voxel world with dense materialized chunk caches.

use std::collections::HashMap;

use bevy::prelude::{Component, Entity, IVec3, Vec3};

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

/// Semantic voxel-world root.
///
/// The authoritative state is `base + modifications`. `chunks` only indexes
/// currently materialized dense working caches used by rendering/query code.
/// Destroying every chunk therefore does not destroy the world.
#[derive(Component, Debug)]
pub struct VoxelWorld {
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
        Self {
            base,
            modifications: VoxelModificationLayer::default(),
            chunks: HashMap::new(),
        }
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

    /// Reconstructs one dense working chunk from procedural base + sparse edits.
    pub fn materialize_chunk(&self, coord: VoxelChunkCoord) -> VoxelChunk {
        let base = self.base;
        let edits = self
            .modifications
            .intersecting(coord.sample_bounds())
            .collect::<Vec<_>>();

        VoxelChunk::generate(coord.origin(), move |point| {
            let mut sample = base.sample(point);
            for edit in &edits {
                sample = edit.apply_to_sample(point, sample);
            }
            sample
        })
    }

    /// Resolves one arbitrary sample without requiring a materialized chunk.
    pub fn resolve_sample(&self, point: Vec3) -> VoxelSample {
        let mut sample = self.base.sample(point);
        for edit in self.modifications.edits() {
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

    /// Returns only materialized chunks whose stored sample domains intersect
    /// the finite influence bounds of an edit.
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

fn chunk_coord_range(bounds: VoxelBounds) -> (IVec3, IVec3) {
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

/// Identifies a materialized chunk as belonging to a particular voxel world.
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
    use crate::voxel::{VoxelBrush, VoxelMaterialId, VoxelSample};

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
