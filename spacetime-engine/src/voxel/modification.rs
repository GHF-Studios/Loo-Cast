//! Sparse authoritative modifications layered over a reconstructible base.

use std::collections::HashMap;

use bevy::prelude::IVec3;

use super::{
    VoxelEdit,
    world::{VoxelChunkCoord, chunk_coord_range},
};

/// Ordered authoritative edit log plus a transitional local-chunk acceleration index.
///
/// `edits` remains the canonical ordering for persistence and edit semantics.
/// `by_chunk` stores only indices into that log and is explicitly Pass-B debt:
/// it can be rebuilt around canonical materialization/scope keys without changing
/// authoritative edit ordering or the dense materialization machinery.
#[derive(Debug, Clone, Default)]
pub struct VoxelModificationLayer {
    edits: Vec<VoxelEdit>,
    by_chunk: HashMap<VoxelChunkCoord, Vec<usize>>,
}

impl VoxelModificationLayer {
    pub fn push(&mut self, edit: VoxelEdit) {
        let edit_index = self.edits.len();
        let (minimum, maximum) = chunk_coord_range(edit.influence_bounds());
        self.edits.push(edit);

        // One edit usually touches only a handful of padded chunk domains. Keep
        // their edit-index lists in global insertion order so replay semantics
        // are identical to iterating the canonical log.
        for z in minimum.z..=maximum.z {
            for y in minimum.y..=maximum.y {
                for x in minimum.x..=maximum.x {
                    let coord = VoxelChunkCoord::new(IVec3::new(x, y, z));
                    self.by_chunk.entry(coord).or_default().push(edit_index);
                }
            }
        }
    }

    pub fn len(&self) -> usize {
        self.edits.len()
    }

    pub fn is_empty(&self) -> bool {
        self.edits.is_empty()
    }

    /// Canonical globally ordered edit log. Spatial hot paths should prefer
    /// [`Self::for_chunk`] or [`Self::for_chunk_since`].
    pub fn edits(&self) -> &[VoxelEdit] {
        &self.edits
    }

    /// Replays only edits whose finite influence bounds intersect this chunk's
    /// padded sample domain, preserving the original global edit order.
    pub fn for_chunk(&self, coord: VoxelChunkCoord) -> impl Iterator<Item = VoxelEdit> + '_ {
        self.for_chunk_since(coord, 0)
    }

    /// Like [`Self::for_chunk`], but ignores edits with global indices before
    /// `first_edit_index`. This lets asynchronous generation catch up without
    /// scanning unrelated edits recorded elsewhere in the world.
    pub fn for_chunk_since(
        &self,
        coord: VoxelChunkCoord,
        first_edit_index: usize,
    ) -> impl Iterator<Item = VoxelEdit> + '_ {
        self.by_chunk
            .get(&coord)
            .into_iter()
            .flatten()
            .copied()
            .filter(move |&index| index >= first_edit_index)
            .map(|index| self.edits[index])
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::{IVec3, Vec3};

    use super::*;
    use crate::voxel::{MATERIALIZATION_CHUNK_SIZE, VoxelBrush, VoxelMaterialId};

    #[test]
    fn chunk_index_returns_only_local_edits_in_global_order() {
        let local_coord = VoxelChunkCoord::new(IVec3::ZERO);
        let first = VoxelEdit::Add {
            brush: VoxelBrush::sphere(Vec3::splat(8.0), 2.0),
            material: VoxelMaterialId::ROCK,
        };
        let distant = VoxelEdit::Add {
            brush: VoxelBrush::sphere(Vec3::splat(1000.0), 2.0),
            material: VoxelMaterialId::ROCK,
        };
        let second = VoxelEdit::Remove {
            brush: VoxelBrush::sphere(Vec3::splat(12.0), 1.0),
        };

        let mut layer = VoxelModificationLayer::default();
        layer.push(first);
        layer.push(distant);
        layer.push(second);

        assert_eq!(
            layer.for_chunk(local_coord).collect::<Vec<_>>(),
            vec![first, second]
        );
        assert_eq!(layer.edits(), &[first, distant, second]);
    }

    #[test]
    fn chunk_index_can_replay_only_edits_appended_after_a_snapshot() {
        let local_coord = VoxelChunkCoord::new(IVec3::ZERO);
        let mut layer = VoxelModificationLayer::default();

        layer.push(VoxelEdit::Add {
            brush: VoxelBrush::sphere(Vec3::splat(8.0), 2.0),
            material: VoxelMaterialId::ROCK,
        });
        let snapshot_count = layer.len();

        // This consumes a global edit index but must not appear in the local
        // chunk's catch-up stream.
        layer.push(VoxelEdit::Add {
            brush: VoxelBrush::sphere(Vec3::splat(1000.0), 2.0),
            material: VoxelMaterialId::ROCK,
        });
        let local_after_snapshot = VoxelEdit::Remove {
            brush: VoxelBrush::sphere(Vec3::splat(8.0), 1.0),
        };
        layer.push(local_after_snapshot);

        assert_eq!(
            layer
                .for_chunk_since(local_coord, snapshot_count)
                .collect::<Vec<_>>(),
            vec![local_after_snapshot]
        );
    }

    #[test]
    fn seam_edit_is_indexed_for_both_padded_chunk_domains() {
        let mut layer = VoxelModificationLayer::default();
        let edit = VoxelEdit::Add {
            brush: VoxelBrush::sphere(
                Vec3::new(MATERIALIZATION_CHUNK_SIZE as f32, 8.0, 8.0),
                1.0,
            ),
            material: VoxelMaterialId::ROCK,
        };
        layer.push(edit);

        assert_eq!(
            layer
                .for_chunk(VoxelChunkCoord::new(IVec3::ZERO))
                .collect::<Vec<_>>(),
            vec![edit]
        );
        assert_eq!(
            layer
                .for_chunk(VoxelChunkCoord::new(IVec3::X))
                .collect::<Vec<_>>(),
            vec![edit]
        );
    }
}
