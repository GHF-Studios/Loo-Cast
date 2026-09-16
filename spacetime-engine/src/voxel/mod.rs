//! Editable volumetric world primitives.
//!
//! M1 connects dense sampled chunks into one addressed world and keeps edits
//! finite/local. Storage hierarchy, streaming, LOD, persistence and physics can
//! now grow around these field/edit semantics without becoming part of them.

mod chunk;
mod edit;
mod field;
mod mesh;
mod world;

pub use chunk::{
    CHUNK_SIZE, SAMPLE_COUNT, SAMPLE_PADDING, SAMPLE_SIZE, VoxelChunk, VoxelChunkEditResult,
    VoxelRayHit,
};
pub use edit::{EDIT_INFLUENCE_MARGIN, VoxelBounds, VoxelBrush, VoxelEdit};
pub use field::{SignedDistance, VoxelMaterialId, VoxelSample};
pub use world::{VoxelChunkCoord, VoxelChunkOf, VoxelWorld};

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
