//! Editable volumetric world primitives.
//!
//! Procedural base + sparse modifications are authoritative. Dense chunks are
//! only materialized working caches; rendering and physics are disposable
//! representations rebuilt from those chunks as the active window streams.

mod generation_scope;
mod async_pipeline;
mod authority;
mod celestial_realization;
mod base;
mod chunk;
mod collision_query;
mod devtools;
mod edit;
mod field;
mod frame;
mod mesh;
mod medium;
mod modification;
mod presentation_palette;
mod worker;
mod physics;
mod realization;
mod region;
mod resolution;
mod manifestation;
mod store;
mod streaming;
mod world;

pub use authority::{CelestialVoxelField, VoxelAuthority};
pub(in crate::voxel) use authority::CelestialPresentationFieldSampler;
pub use celestial_realization::CelestialVoxelRealizationPolicy;
pub(in crate::voxel) use celestial_realization::{
    CelestialVoxelRealization, CelestialVoxelRealizationRegistry,
};
pub use base::{
    CelestialBodyProfile, ProceduralCelestialBody, ProceduralTerrain, ProceduralVolume, VoxelBase,
};
pub use chunk::{
    CHUNK_SIZE, MATERIALIZATION_CHUNK_SIZE, VoxelChunk, VoxelChunkEditResult, VoxelRayHit,
};
pub use collision_query::VoxelCollisionQuery;
pub use edit::{EDIT_INFLUENCE_MARGIN, VoxelBounds, VoxelBrush, VoxelEdit, VoxelQueryPosition};
pub use field::{
    SignedDistance, VoxelCollisionMode, VoxelMaterialBehavior, VoxelMaterialId, VoxelSample,
};
pub use frame::{VoxelFrameBrush, VoxelFrameEdit, VoxelFramePosition, VoxelFrameSnapshot};
pub use modification::VoxelModificationLayer;
pub use realization::VoxelScaleDomain;
pub(in crate::voxel) use realization::{
    VoxelRealizationDemandSnapshot, VoxelRealizationIntentSnapshot,
    VoxelRealizationScope,
};
pub(in crate::voxel) use region::VoxelRegionSpan;
pub use manifestation::VoxelPresentationMaterial;
pub use streaming::{
    VoxelMaterializationDemand, VoxelPinnedDemand, VoxelStreaming, VoxelStreamingTelemetry,
};
pub use world::{VoxelChunkAddress, VoxelChunkCoord, VoxelMaterializationChunkAddress, VoxelWorld};
pub(in crate::voxel) use world::VoxelMaterializationKey;

use bevy::prelude::*;

use crate::{
    physics::{
        character::CharacterMovementSet,
        collision_query::UsfCollisionQuerySet,
    },
    spatial::{
        SpatialDemandSet, UsfCapabilitySet, UsfResidencySet, UsfSpatialSet,
    },
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
    RealizationIntent,
    RealizationLifecycle,
    RealizationDemand,
    Residency,
    RetireStaleWork,
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
    Capability,
}

impl Plugin for VoxelPlugin {
    fn build(&self, app: &mut App) {
        manifestation::configure(app);
        resolution::configure(app);

        app.init_resource::<manifestation::VoxelMaterializationRuntimeRegistry>()
            .init_resource::<VoxelRealizationIntentSnapshot>()
            .init_resource::<VoxelRealizationDemandSnapshot>()
            .init_resource::<CelestialVoxelRealizationRegistry>()
            .init_resource::<worker::VoxelWorkerPool>()
            .init_resource::<VoxelStreamingTelemetry>()
            .configure_sets(
                Update,
                (
                    VoxelUpdateSet::RealizationIntent.after(SpatialDemandSet::Collect),
                    VoxelUpdateSet::RealizationLifecycle
                        .after(VoxelUpdateSet::RealizationIntent),
                    VoxelUpdateSet::RealizationDemand
                        .after(VoxelUpdateSet::RealizationLifecycle),
                    VoxelUpdateSet::Residency
                        .after(VoxelUpdateSet::RealizationDemand)
                        .after(UsfResidencySet::Reconcile),
                    VoxelUpdateSet::RetireStaleWork.after(VoxelUpdateSet::Residency),
                    VoxelUpdateSet::Generation.after(VoxelUpdateSet::RetireStaleWork),
                ),
            )
            .add_systems(
                Update,
                (
                    realization::collect_voxel_realization_intent
                        .in_set(VoxelUpdateSet::RealizationIntent)
                        .in_set(UsfResidencySet::Collect),
                    celestial_realization::sync_celestial_voxel_realizations
                        .in_set(VoxelUpdateSet::RealizationLifecycle)
                        .in_set(UsfResidencySet::Collect),
                    realization::resolve_voxel_realization_demand
                        .in_set(VoxelUpdateSet::RealizationDemand)
                        .in_set(UsfResidencySet::Collect),
                )
                    .chain(),
            )
            .add_systems(
                Update,
                //
                // Celestial presentation is a Cartesian volumetric hierarchy
                // owned by voxel::resolution.
                streaming::refresh_voxel_residency.in_set(VoxelUpdateSet::Residency),
            )
            .add_systems(
                Update,
                (
                    streaming::retire_stale_generation_tasks,
                    async_pipeline::retire_stale_chunk_builds,
                )
                    .in_set(VoxelUpdateSet::RetireStaleWork),
            )
            .add_systems(
                Update,
                streaming::schedule_voxel_generation.in_set(VoxelUpdateSet::Generation),
            )
            .add_systems(
                FixedUpdate,
                medium::apply_voxel_medium_drag.after(CharacterMovementSet::Simulate),
            )
            .configure_sets(
                PostUpdate,
                (
                    VoxelPostUpdateSet::SurfaceScheduling
                        .after(VoxelPostUpdateSet::DensePublication)
                        .after(VoxelPostUpdateSet::SurfacePublication),
                    VoxelPostUpdateSet::Membership
                        .after(VoxelPostUpdateSet::SurfacePublication)
                        .after(VoxelPostUpdateSet::ManifestationCleanup),
                    // Collision is a direct consumer of store-owned derived
                    // surfaces. Publish physics before renderer manifestation so
                    // physical terrain can never win the readiness race.
                    VoxelPostUpdateSet::Collision
                        .after(VoxelPostUpdateSet::SurfacePublication),
                    VoxelPostUpdateSet::Rebuild
                        .after(VoxelPostUpdateSet::Membership)
                        .after(VoxelPostUpdateSet::Collision),
                    VoxelPostUpdateSet::Capability
                        .after(VoxelPostUpdateSet::Rebuild)
                        .before(UsfSpatialSet::SyncSemantic),
                ),
            )
            .configure_sets(
                PostUpdate,
                (
                    VoxelPostUpdateSet::Collision,
                    VoxelPostUpdateSet::Rebuild,
                    VoxelPostUpdateSet::Capability,
                )
                    .chain(),
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
                (
                    manifestation::sync_manifestation_runtime_transforms
                        .after(VoxelPostUpdateSet::Rebuild),
                    manifestation::sync_collision_aggregate_runtime_transforms,
                )
                    .in_set(UsfSpatialSet::RuntimeProjection),
            )
            .add_systems(
                PostUpdate,
                manifestation::sync_manifestation_collision_residency
                    .in_set(VoxelPostUpdateSet::Collision),
            )
            .add_systems(
                PostUpdate,
                manifestation::sync_capability_realizations
                    .in_set(VoxelPostUpdateSet::Capability)
                    .in_set(UsfCapabilitySet::Publish),
            )
            .add_systems(
                PostUpdate,
                collision_query::publish_collision_query_candidates
                    .in_set(UsfCollisionQuerySet::Providers),
            );

        #[cfg(feature = "profiling-tracy")]
        app.add_systems(
            PostUpdate,
            worker::emit_worker_pressure.after(VoxelPostUpdateSet::SurfaceScheduling),
        );

        devtools::configure(app);
    }
}
