//! Editable volumetric world primitives.
//!
//! M0 intentionally starts with one dense sampled chunk. Storage hierarchy,
//! streaming, LOD, persistence and physics can grow around these field/edit
//! semantics without being baked into them.

mod chunk;
mod edit;
mod field;
mod mesh;

pub use chunk::{
    CHUNK_SIZE, SAMPLE_COUNT, SAMPLE_PADDING, SAMPLE_SIZE, VoxelChunk, VoxelChunkEditResult,
    VoxelRayHit,
};
pub use edit::{VoxelBrush, VoxelEdit};
pub use field::{SignedDistance, VoxelMaterialId, VoxelSample};

use bevy::prelude::*;

pub struct VoxelPlugin;

impl Plugin for VoxelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, mesh::remesh_dirty_chunks);
    }
}

/// Creates an empty mesh asset suitable for a [`VoxelChunk`] render entity.
/// The voxel plugin will populate it during `PostUpdate`.
pub fn empty_voxel_mesh() -> Mesh {
    mesh::empty_mesh()
}
