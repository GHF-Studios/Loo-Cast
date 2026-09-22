//! Editable volumetric world primitives.
//!
//! Procedural base + sparse modifications are authoritative. Dense chunks are
//! only materialized working caches; rendering and physics are disposable
//! representations rebuilt from those chunks as the active window streams.

mod generation_scope;
mod async_pipeline;
mod authority;
mod base;
mod chunk;
mod devtools;
mod edit;
mod field;
mod mesh;
mod medium;
mod modification;
mod worker;
mod physics;
mod realization;
mod manifestation;
mod store;
mod streaming;
mod world;

pub use authority::{CelestialVoxelField, VoxelAuthority};
pub use base::{
    CelestialBodyProfile, ProceduralCelestialBody, ProceduralTerrain, ProceduralVolume, VoxelBase,
};
pub use chunk::{
    CHUNK_SIZE, MATERIALIZATION_CHUNK_SIZE, VoxelChunk, VoxelChunkEditResult, VoxelRayHit,
};
pub use edit::{EDIT_INFLUENCE_MARGIN, VoxelBounds, VoxelBrush, VoxelEdit, VoxelQueryPosition};
pub use field::{
    SignedDistance, VoxelCollisionMode, VoxelMaterialBehavior, VoxelMaterialId, VoxelSample,
};
pub use modification::VoxelModificationLayer;
pub use realization::VoxelScaleDomain;
pub(in crate::voxel) use realization::VoxelRealizationDemandSnapshot;
pub use streaming::{
    VoxelMaterializationDemand, VoxelPinnedDemand, VoxelPresentationMaterial, VoxelStreaming,
};
pub use world::{VoxelChunkAddress, VoxelChunkCoord, VoxelMaterializationChunkAddress, VoxelWorld};

use bevy::prelude::*;

use crate::{
    physics::character::CharacterMovementSet,
    spatial::{SpatialDemandSet, UsfSpatialSet},
};

/// Suppresses derived physics colliders for a voxel world.
#[derive(Component, Debug, Default, Clone, Copy)]
pub struct VoxelCollisionDisabled;

/// Suppresses gameplay edits for a voxel world while retaining the normal
/// reconstructible-base/materialization pipeline.
#[derive(Component, Debug, Default, Clone, Copy)]
pub struct VoxelEditingDisabled;

pub struct VoxelPlugin;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum VoxelUpdateSet {
    RealizationDemand,
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
            .init_resource::<VoxelRealizationDemandSnapshot>()
            .add_systems(Startup, manifestation::initialize_translucent_voxel_material)
            .configure_sets(
                Update,
                (
                    VoxelUpdateSet::RealizationDemand.after(SpatialDemandSet::Collect),
                    VoxelUpdateSet::Residency.after(VoxelUpdateSet::RealizationDemand),
                    VoxelUpdateSet::Generation.after(VoxelUpdateSet::Residency),
                ),
            )
            .add_systems(
                Update,
                realization::collect_voxel_realization_demand
                    .in_set(VoxelUpdateSet::RealizationDemand),
            )
            .add_systems(
                Update,
                streaming::refresh_voxel_residency.in_set(VoxelUpdateSet::Residency),
            )
            .add_systems(
                Update,
                streaming::schedule_voxel_generation.in_set(VoxelUpdateSet::Generation),
            )
            .add_systems(
                FixedUpdate,
                medium::apply_voxel_medium_drag.after(CharacterMovementSet::Simulate),
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
                manifestation::sync_manifestation_runtime_transforms
                    .after(VoxelPostUpdateSet::Rebuild)
                    .after(UsfSpatialSet::Rebase)
                    .before(UsfSpatialSet::ViewProjection),
            )
            .add_systems(
                PostUpdate,
                manifestation::sync_manifestation_collision_residency
                    .in_set(VoxelPostUpdateSet::Collision),
            );

        devtools::configure(app);
    }
}

