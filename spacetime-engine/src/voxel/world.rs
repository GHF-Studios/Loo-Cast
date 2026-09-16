//! Spatial index connecting dense working chunks into one editable voxel world.

use std::collections::HashMap;

use bevy::prelude::{Component, Entity, IVec3, Vec3};

use super::{CHUNK_SIZE, SAMPLE_PADDING, VoxelBounds};

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
/// M1 only indexes already-materialized dense chunks. Sparse allocation,
/// procedural backing storage, streaming and LOD can replace this index without
/// changing edit semantics or chunk-local meshing.
#[derive(Component, Debug, Default)]
pub struct VoxelWorld {
    chunks: HashMap<VoxelChunkCoord, Entity>,
}

impl VoxelWorld {
    pub fn insert_chunk(&mut self, coord: VoxelChunkCoord, entity: Entity) -> Option<Entity> {
        self.chunks.insert(coord, entity)
    }

    pub fn chunk_entity(&self, coord: VoxelChunkCoord) -> Option<Entity> {
        self.chunks.get(&coord).copied()
    }

    pub fn chunk_entities(&self) -> impl Iterator<Item = Entity> + '_ {
        self.chunks.values().copied()
    }

    /// Returns only materialized chunks whose stored sample domains intersect
    /// the finite influence bounds of an edit.
    ///
    /// This is addressed directly through chunk coordinates rather than by
    /// scanning every materialized chunk in the world.
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
    use crate::voxel::{VoxelBrush, VoxelChunk, VoxelEdit, VoxelMaterialId, VoxelSample};

    #[test]
    fn chunk_coordinates_are_euclidean_grid_coordinates() {
        assert_eq!(
            VoxelChunkCoord::new(IVec3::new(-1, 2, 0)).origin(),
            IVec3::new(-(CHUNK_SIZE as i32), 2 * CHUNK_SIZE as i32, 0)
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
            brush: VoxelBrush::sphere(
                Vec3::new(CHUNK_SIZE as f32, 16.0, 16.0),
                3.0,
            ),
            material: VoxelMaterialId::ROCK,
        };

        assert!(left.sample_bounds().intersects(edit.influence_bounds()));
        assert!(right.sample_bounds().intersects(edit.influence_bounds()));
        left.apply_edit(edit);
        right.apply_edit(edit);

        // The two allocations overlap at x = 31 and x = 32 because each keeps
        // one sample of neighbor padding. Those copies must stay bit-identical.
        for x in [CHUNK_SIZE as i32 - 1, CHUNK_SIZE as i32] {
            for y in 13..=19 {
                for z in 13..=19 {
                    let point = IVec3::new(x, y, z);
                    assert_eq!(left.sample(point), right.sample(point));
                }
            }
        }
    }
}
