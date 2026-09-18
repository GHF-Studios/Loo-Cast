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
mod render_aggregate;
mod store;
mod streaming;
mod world;

pub use base::{ProceduralTerrain, ProceduralVolume, VoxelBase};
pub use chunk::{
    CHUNK_SIZE, MATERIALIZATION_CHUNK_SIZE, VoxelChunk, VoxelChunkEditResult, VoxelRayHit,
};
pub use edit::{EDIT_INFLUENCE_MARGIN, VoxelBounds, VoxelBrush, VoxelEdit, VoxelQueryPosition};
pub use field::{SignedDistance, VoxelMaterialId, VoxelSample};
pub use modification::VoxelModificationLayer;
pub use streaming::{VoxelMaterializationDemand, VoxelPresentationMaterial, VoxelStreaming};
pub use world::{VoxelChunkAddress, VoxelChunkCoord, VoxelMaterializationChunkAddress, VoxelWorld};

use bevy::prelude::*;

use crate::spatial::SpatialDemandSet;

pub struct VoxelPlugin;

impl Plugin for VoxelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<perf::VoxelPerfStats>()
            .init_resource::<render_aggregate::VoxelRenderAggregateRegistry>()
            .add_systems(
                Update,
                streaming::retire_orphaned_tasks.before(streaming::stream_voxel_chunks),
            )
            .add_systems(
                Update,
                streaming::stream_voxel_chunks.after(SpatialDemandSet::Collect),
            )
            .add_systems(
                PostUpdate,
                (
                    // Finish field generation, publish store-owned derived caches,
                    // then rebuild only dirty aggregate runtime manifestations.
                    streaming::finish_chunk_generation,
                    async_pipeline::publish_completed_chunk_builds,
                    async_pipeline::queue_dirty_chunk_builds,
                    render_aggregate::sync_render_aggregates,
                )
                    .chain(),
            );

        devtools::configure(app);
    }
}

/// Creates an empty mesh asset suitable for a [`VoxelChunk`] render entity.
/// The voxel plugin will populate it during `PostUpdate`.
pub fn empty_voxel_mesh() -> Mesh {
    mesh::empty_mesh()
}
