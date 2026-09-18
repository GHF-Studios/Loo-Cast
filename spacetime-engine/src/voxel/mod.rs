//! Editable volumetric world primitives.
//!
//! Procedural base + sparse modifications are authoritative. Dense chunks are
//! only materialized working caches; rendering and physics are disposable
//! representations rebuilt from those chunks as the active window streams.

mod aggregate;
mod async_pipeline;
mod base;
mod chunk;
mod devtools;
mod edit;
mod field;
mod mesh;
mod modification;
mod perf;
mod physics;
mod streaming;
mod world;

pub use base::{ProceduralTerrain, ProceduralVolume, VoxelBase};
pub use chunk::{
    CHUNK_SIZE, MATERIALIZATION_CHUNK_SIZE, VoxelChunk, VoxelChunkEditResult, VoxelRayHit,
};
pub use edit::{EDIT_INFLUENCE_MARGIN, VoxelBounds, VoxelBrush, VoxelEdit, VoxelQueryPosition};
pub use field::{SignedDistance, VoxelMaterialId, VoxelSample};
pub use modification::VoxelModificationLayer;
pub use streaming::{VoxelMaterializationDemand, VoxelStreaming};
pub use world::{
    VoxelChunkAddress, VoxelChunkCoord, VoxelChunkOf, VoxelMaterializationChunkAddress, VoxelWorld,
};

use bevy::prelude::*;

use crate::spatial::SpatialDemandSet;

#[derive(Component, Debug, Clone, Copy)]
pub(crate) struct VoxelChunkPresentation(pub Entity);

/// Tracks completion of the collider-presence request for one dense chunk.
/// Rendering and semantic materialization remain independent.
#[derive(Component, Debug, Clone, Copy, Default)]
pub(crate) struct VoxelChunkPhysicsLod {
    pub(crate) requested: bool,
    pub(crate) built_revision: Option<u64>,
    pub(crate) collider_ready: bool,
}

pub struct VoxelPlugin;

impl Plugin for VoxelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<perf::VoxelPerfStats>()
            .add_systems(
                Update,
                streaming::retire_orphaned_chunks.before(streaming::stream_voxel_chunks),
            )
            .add_systems(
                Update,
                streaming::stream_voxel_chunks.after(SpatialDemandSet::Collect),
            )
            .add_systems(
                PostUpdate,
                (
                    // Finish field generation after ordinary Update gameplay
                    // edits, then publish/queue revision-checked derived caches.
                    streaming::finish_chunk_generation,
                    async_pipeline::publish_completed_chunk_builds,
                    async_pipeline::queue_dirty_chunk_builds,
                )
                    .chain(),
            )
            .add_systems(
                PostUpdate,
                perf::report_voxel_perf.after(async_pipeline::queue_dirty_chunk_builds),
            );

        devtools::configure(app);
    }
}

/// Creates an empty mesh asset suitable for a [`VoxelChunk`] render entity.
/// The voxel plugin will populate it during `PostUpdate`.
pub fn empty_voxel_mesh() -> Mesh {
    mesh::empty_mesh()
}
