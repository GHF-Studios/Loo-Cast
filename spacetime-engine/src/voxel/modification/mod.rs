//! Sparse inline modifications layered over a reconstructible base. Semantic authority-backed worlds use VoxelAuthority instead.

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
mod tests;
