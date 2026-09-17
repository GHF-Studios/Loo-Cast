//! Sparse authoritative modifications layered over a reconstructible base.

use std::collections::HashMap;

use super::{VoxelEdit, VoxelMaterializationChunkAddress};

/// Ordered authoritative edit log plus a canonical materialization-address index.
///
/// `edits` is the canonical persistence/replay ordering. `by_chunk` stores only
/// indices into that log, so rebuilding/discarding the acceleration index never
/// changes edit semantics.
#[derive(Debug, Clone, Default)]
pub struct VoxelModificationLayer {
    edits: Vec<VoxelEdit>,
    by_chunk: HashMap<VoxelMaterializationChunkAddress, Vec<usize>>,
}

impl VoxelModificationLayer {
    pub(crate) fn push(
        &mut self,
        edit: VoxelEdit,
        addresses: impl IntoIterator<Item = VoxelMaterializationChunkAddress>,
    ) {
        let edit_index = self.edits.len();
        self.edits.push(edit);

        for address in addresses {
            self.by_chunk.entry(address).or_default().push(edit_index);
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

    /// Replays only edits indexed for this canonical materialization address,
    /// preserving the original global edit order.
    pub fn for_chunk(
        &self,
        address: VoxelMaterializationChunkAddress,
    ) -> impl Iterator<Item = VoxelEdit> + '_ {
        self.for_chunk_since(address, 0)
    }

    /// Like [`Self::for_chunk`], but ignores edits with global indices before
    /// `first_edit_index`. This lets asynchronous generation catch up without
    /// scanning unrelated edits recorded elsewhere in semantic space.
    pub fn for_chunk_since(
        &self,
        address: VoxelMaterializationChunkAddress,
        first_edit_index: usize,
    ) -> impl Iterator<Item = VoxelEdit> + '_ {
        self.by_chunk
            .get(&address)
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
    use crate::voxel::{
        MATERIALIZATION_CHUNK_SIZE, VoxelBase, VoxelBrush, VoxelChunkCoord, VoxelMaterialId,
        VoxelQueryPosition, VoxelWorld,
    };

    fn query(local: Vec3) -> VoxelQueryPosition {
        VoxelQueryPosition::from_scale0_local(local).unwrap()
    }

    #[test]
    fn canonical_chunk_index_returns_only_local_edits_in_global_order() {
        let mut world = VoxelWorld::new(VoxelBase::Empty);
        let local_address = world.chunk_address(VoxelChunkCoord::new(IVec3::ZERO)).unwrap();
        let first = VoxelEdit::Add {
            brush: VoxelBrush::sphere(query(Vec3::splat(8.0)), 2.0),
            material: VoxelMaterialId::ROCK,
        };
        let distant = VoxelEdit::Add {
            brush: VoxelBrush::sphere(query(Vec3::splat(1000.0)), 2.0),
            material: VoxelMaterialId::ROCK,
        };
        let second = VoxelEdit::Remove {
            brush: VoxelBrush::sphere(query(Vec3::splat(12.0)), 1.0),
        };

        world.record_edit(first).unwrap();
        world.record_edit(distant).unwrap();
        world.record_edit(second).unwrap();

        assert_eq!(
            world.modifications().for_chunk(local_address).collect::<Vec<_>>(),
            vec![first, second]
        );
        assert_eq!(world.modifications().edits(), &[first, distant, second]);
    }

    #[test]
    fn canonical_chunk_index_can_replay_only_edits_after_a_snapshot() {
        let mut world = VoxelWorld::new(VoxelBase::Empty);
        let address = world.chunk_address(VoxelChunkCoord::new(IVec3::ZERO)).unwrap();

        world
            .record_edit(VoxelEdit::Add {
                brush: VoxelBrush::sphere(query(Vec3::splat(8.0)), 2.0),
                material: VoxelMaterialId::ROCK,
            })
            .unwrap();
        let snapshot_count = world.modifications().len();

        world
            .record_edit(VoxelEdit::Add {
                brush: VoxelBrush::sphere(query(Vec3::splat(1000.0)), 2.0),
                material: VoxelMaterialId::ROCK,
            })
            .unwrap();
        let local_after_snapshot = VoxelEdit::Remove {
            brush: VoxelBrush::sphere(query(Vec3::splat(8.0)), 1.0),
        };
        world.record_edit(local_after_snapshot).unwrap();

        assert_eq!(
            world
                .modifications()
                .for_chunk_since(address, snapshot_count)
                .collect::<Vec<_>>(),
            vec![local_after_snapshot]
        );
    }

    #[test]
    fn seam_edit_is_indexed_for_both_canonical_padded_chunk_domains() {
        let mut world = VoxelWorld::new(VoxelBase::Empty);
        let edit = VoxelEdit::Add {
            brush: VoxelBrush::sphere(
                query(Vec3::new(MATERIALIZATION_CHUNK_SIZE as f32, 8.0, 8.0)),
                1.0,
            ),
            material: VoxelMaterialId::ROCK,
        };
        world.record_edit(edit).unwrap();

        let left = world.chunk_address(VoxelChunkCoord::new(IVec3::ZERO)).unwrap();
        let right = world.chunk_address(VoxelChunkCoord::new(IVec3::X)).unwrap();
        assert_eq!(
            world.modifications().for_chunk(left).collect::<Vec<_>>(),
            vec![edit]
        );
        assert_eq!(
            world.modifications().for_chunk(right).collect::<Vec<_>>(),
            vec![edit]
        );
    }
}
