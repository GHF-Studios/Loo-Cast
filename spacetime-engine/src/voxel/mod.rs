//! Editable volumetric world primitives.
//!
//! M2 makes procedural base + sparse modifications authoritative. Dense chunks
//! are only materialized working caches for rendering/queries, which means an
//! untouched world can remain almost entirely implicit.

mod base;
mod chunk;
mod edit;
mod field;
mod mesh;
mod modification;
mod world;

pub use base::{ProceduralTerrain, VoxelBase};
pub use chunk::{
    CHUNK_SIZE, SAMPLE_COUNT, SAMPLE_PADDING, SAMPLE_SIZE, VoxelChunk, VoxelChunkEditResult,
    VoxelRayHit,
};
pub use edit::{EDIT_INFLUENCE_MARGIN, VoxelBounds, VoxelBrush, VoxelEdit};
pub use field::{SignedDistance, VoxelMaterialId, VoxelSample};
pub use modification::VoxelModificationLayer;
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
