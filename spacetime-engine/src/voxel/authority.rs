//! Shared semantic authority for voxel fields with multiple realizations.
//!
//! A realization owns residency, dense materializations, meshes and colliders.
//! The authority owns semantic identity and the canonical ordered edit history.
//! Scale-specific spatial indexes may be derived later, but they must never
//! become edit identity.

use bevy::prelude::*;

use super::VoxelFrameEdit;

/// Canonical edit authority shared by one or more voxel realizations.
///
/// The log is intentionally address-agnostic. A materialization address belongs
/// to one scale-local realization and therefore cannot be the global edit key.
#[derive(Component, Debug, Default)]
pub struct VoxelAuthority {
    edits: Vec<VoxelFrameEdit>,
}

impl VoxelAuthority {
    pub fn record_edit(&mut self, edit: VoxelFrameEdit) {
        self.edits.push(edit);
    }

    pub fn edits(&self) -> &[VoxelFrameEdit] {
        &self.edits
    }

    pub fn edits_since(
        &self,
        first_edit_index: usize,
    ) -> impl Iterator<Item = VoxelFrameEdit> + '_ {
        self.edits
            .iter()
            .copied()
            .skip(first_edit_index.min(self.edits.len()))
    }

    pub fn len(&self) -> usize {
        self.edits.len()
    }
    pub fn is_empty(&self) -> bool {
        self.edits.is_empty()
    }
}
