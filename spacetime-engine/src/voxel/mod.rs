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
mod worker;
mod physics;
mod manifestation;
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

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum VoxelUpdateSet {
    Residency,
    Generation,
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum VoxelPostUpdateSet {
    DensePublication,
    SurfacePublication,
    SurfaceScheduling,
    ManifestationCleanup,
    Membership,
    Rebuild,
    Collision,
}

impl Plugin for VoxelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<manifestation::VoxelManifestationRegistry>()
            .configure_sets(
                Update,
                (
                    VoxelUpdateSet::Residency.after(SpatialDemandSet::Collect),
                    VoxelUpdateSet::Generation.after(VoxelUpdateSet::Residency),
                ),
            )
            .add_systems(
                Update,
                streaming::refresh_voxel_residency.in_set(VoxelUpdateSet::Residency),
            )
            .add_systems(
                Update,
                streaming::schedule_voxel_generation.in_set(VoxelUpdateSet::Generation),
            )
            // Orphan retirement is independent of demand planning and should not
            // serialize the normal residency -> generation path.
            .add_systems(Update, streaming::retire_orphaned_tasks)
            .configure_sets(
                PostUpdate,
                (
                    VoxelPostUpdateSet::SurfaceScheduling
                        .after(VoxelPostUpdateSet::DensePublication)
                        .after(VoxelPostUpdateSet::SurfacePublication),
                    VoxelPostUpdateSet::Membership
                        .after(VoxelPostUpdateSet::SurfacePublication)
                        .after(VoxelPostUpdateSet::ManifestationCleanup),
                    VoxelPostUpdateSet::Rebuild.after(VoxelPostUpdateSet::Membership),
                    VoxelPostUpdateSet::Collision.after(VoxelPostUpdateSet::Rebuild),
                ),
            )
            .add_systems(
                PostUpdate,
                streaming::finish_chunk_generation
                    .in_set(VoxelPostUpdateSet::DensePublication),
            )
            .add_systems(
                PostUpdate,
                async_pipeline::publish_completed_chunk_builds
                    .in_set(VoxelPostUpdateSet::SurfacePublication),
            )
            .add_systems(
                PostUpdate,
                async_pipeline::queue_dirty_chunk_builds
                    .in_set(VoxelPostUpdateSet::SurfaceScheduling),
            )
            .add_systems(
                PostUpdate,
                manifestation::retire_removed_world_manifestations
                    .in_set(VoxelPostUpdateSet::ManifestationCleanup),
            )
            .add_systems(
                PostUpdate,
                manifestation::sync_manifestation_membership
                    .in_set(VoxelPostUpdateSet::Membership),
            )
            .add_systems(
                PostUpdate,
                manifestation::rebuild_dirty_manifestations
                    .in_set(VoxelPostUpdateSet::Rebuild),
            )
            .add_systems(
                PostUpdate,
                manifestation::sync_manifestation_collision_residency
                    .in_set(VoxelPostUpdateSet::Collision),
            );

        devtools::configure(app);
    }
}

/// Creates an empty mesh asset suitable for a [`VoxelChunk`] render entity.
pub fn empty_voxel_mesh() -> Mesh {
    mesh::empty_mesh()
}
