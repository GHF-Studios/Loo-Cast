//! Editable volumetric world primitives.
//!
//! Procedural base + sparse modifications are authoritative. Dense chunks are
//! only materialized working caches; rendering and physics are disposable
//! representations rebuilt from those chunks as the active window streams.

mod base;
mod chunk;
mod edit;
mod field;
mod mesh;
mod modification;
mod physics;
mod streaming;
mod world;

pub use base::{ProceduralTerrain, VoxelBase};
pub use chunk::{
    CHUNK_SIZE, SAMPLE_COUNT, SAMPLE_PADDING, SAMPLE_SIZE, VoxelChunk, VoxelChunkEditResult,
    VoxelRayHit,
};
pub use edit::{EDIT_INFLUENCE_MARGIN, VoxelBounds, VoxelBrush, VoxelEdit};
pub use field::{SignedDistance, VoxelMaterialId, VoxelSample};
pub use modification::VoxelModificationLayer;
pub use streaming::VoxelStreaming;
pub use world::{VoxelChunkCoord, VoxelChunkOf, VoxelWorld};

use bevy::prelude::*;

pub struct VoxelPlugin;

impl Plugin for VoxelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, streaming::stream_voxel_chunks)
            .add_systems(PostUpdate, mesh::rebuild_dirty_chunks);
    }
}

/// Creates an empty mesh asset suitable for a [`VoxelChunk`] render entity.
/// The voxel plugin will populate it during `PostUpdate`.
pub fn empty_voxel_mesh() -> Mesh {
    mesh::empty_mesh()
}
