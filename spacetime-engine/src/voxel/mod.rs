//! Semantic voxel authority, scale realizations, and disposable materialization backends.
//!
//! Procedural base + sparse modifications are authoritative. Dense chunks are
//! only materialized working caches; rendering and physics are disposable
//! representations rebuilt from those chunks as the active window streams.
//!
//! ## Integration
//!
//! Semantic authority and sparse edits feed scale realizations; demand selects bounded
//! materializations, worker execution derives their caches, and manifestation and collision
//! facilities publish disposable outputs.
//!
//! ## Module map
//!
//! - `async_pipeline`: Surface-derivation pipeline from dense materializations to disposable CPU
//!   geometry.
//! - `authority`: Canonical semantic edit authority shared by voxel scale realizations.
//! - `base`: Procedural backing fields for voxel worlds.
//! - `celestial_field`: Semantic celestial field definition and canonical surface queries.
//! - `celestial_realization`: Lazy lifecycle for disposable celestial voxel scale realizations.
//! - `chunk`: Dense bounded working materialization of a voxel field.
//! - `collision_query`: Conservative swept-collision candidates from semantic voxel authority.
//! - `devtools`: Voxel realization developer visualization.
//! - `edit`: Constructive edits over the authoritative voxel field.
//! - `field`: Scalar field samples used by the voxel world.
//! - `frame`: Body-local semantic coordinates for movable voxel authority.
//! - `generation_scope`: Decimal processing scopes for voxel generation batching.
//! - `manifestation`: Disposable presentation manifestations derived from store-owned voxel
//!   surfaces.
//! - `medium`: Non-rigid interaction with volumetric voxel materials.
//! - `mesh`: Disposable render-mesh and collision-surface extraction from voxel fields.
//! - `modification`: Sparse inline edits for standalone realizations. Shared semantic
//!   realizations use VoxelSemanticAuthority.
//! - `physics`: Disposable collision representation derived from authoritative voxel chunks.
//! - `presentation_palette`: Shared diagnostic colors for independent scale and binary LOD views.
//! - `realization`: Multiscale voxel realization policy.
//! - `region`: Sparse capability-local region addressing over voxel materialization leaves.
//! - `store`: Compact residency and cache storage for voxel materializations.
//! - `streaming`: Demand-driven residency policy for voxel materializations.
//! - `worker`: Bounded execution facility for reconstructible voxel work.
//! - `world`: One scale-local voxel realization over canonical semantic voxel authority.
//!
//! The plugin or configure entrypoint here wires the child systems and resources.
//!

mod async_pipeline;
mod authority;
mod base;
mod celestial_field;
mod celestial_realization;
mod chunk;
mod collision_query;
mod devtools;
mod edit;
mod field;
mod frame;
mod generation_scope;
mod manifestation;
mod medium;
mod mesh;
mod modification;
mod physics;
mod presentation_palette;
mod realization;
mod region;
mod store;
mod streaming;
mod worker;
mod world;

pub use authority::VoxelSemanticAuthority;
pub use base::{CelestialFieldRealization, ProceduralTerrain, ProceduralVolume, VoxelBase};
pub use celestial_field::CelestialVoxelField;
pub use celestial_realization::CelestialVoxelRealizationPolicy;
pub(in crate::voxel) use celestial_realization::{
    CelestialVoxelRealizationFrame, CelestialVoxelRealizations, CelestialVoxelScaleRealization,
};
pub use chunk::{
    DenseVoxelMaterialization, MATERIALIZATION_CHUNK_SIZE, VoxelMaterializationEditResult,
    VoxelRayHit,
};
pub use collision_query::VoxelCollisionQuery;
pub use edit::{EDIT_INFLUENCE_MARGIN, VoxelBounds, VoxelBrush, VoxelEdit, VoxelQueryPosition};
pub use field::{
    SignedDistance, VoxelCollisionMode, VoxelMaterialBehavior, VoxelMaterialId, VoxelSample,
};
pub use frame::{VoxelFrameBrush, VoxelFrameEdit, VoxelFramePosition, VoxelFrameSnapshot};
pub use manifestation::VoxelPresentationMaterial;
pub use modification::VoxelModificationLayer;
pub use realization::VoxelScaleDomain;
pub(in crate::voxel) use realization::{
    VoxelRealizationDemandSnapshot, VoxelRealizationIntentSnapshot, VoxelRealizationScope,
};
pub(in crate::voxel) use region::VoxelRegionSpan;
pub use streaming::{
    VoxelMaterializationDemand, VoxelMaterializationResidency, VoxelMaterializationTelemetry,
    VoxelPinnedMaterializationDemand,
};
pub(in crate::voxel) use world::VoxelMaterializationKey;
pub use world::{VoxelChunkCoord, VoxelMaterializationChunkAddress, VoxelScaleRealization};

use bevy::prelude::*;

use crate::{
    physics::{character::CharacterMovementSet, collision_query::UsfCollisionQuerySet},
    spatial::{SpatialDemandSet, UsfCapabilitySet, UsfResidencySet, UsfSpatialSet},
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

        app.init_resource::<manifestation::VoxelPresentationManifestationRegistry>()
            .init_resource::<VoxelRealizationIntentSnapshot>()
            .init_resource::<VoxelRealizationDemandSnapshot>()
            .init_resource::<CelestialVoxelRealizations>()
            .init_resource::<worker::VoxelWorkExecutor>()
            .init_resource::<VoxelMaterializationTelemetry>()
            .configure_sets(
                Update,
                (
                    VoxelUpdateSet::RealizationIntent.after(SpatialDemandSet::Collect),
                    VoxelUpdateSet::RealizationLifecycle.after(VoxelUpdateSet::RealizationIntent),
                    VoxelUpdateSet::RealizationDemand.after(VoxelUpdateSet::RealizationLifecycle),
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
                    realization::collect_voxel_realization_intents
                        .in_set(VoxelUpdateSet::RealizationIntent)
                        .in_set(UsfResidencySet::Collect),
                    celestial_realization::reconcile_celestial_voxel_realizations
                        .in_set(VoxelUpdateSet::RealizationLifecycle)
                        .in_set(UsfResidencySet::Collect),
                    realization::resolve_voxel_realization_demands
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
                streaming::reconcile_voxel_materialization_residency
                    .in_set(VoxelUpdateSet::Residency),
            )
            .add_systems(
                Update,
                (
                    streaming::retire_stale_materialization_generation,
                    async_pipeline::retire_stale_surface_derivations,
                )
                    .in_set(VoxelUpdateSet::RetireStaleWork),
            )
            .add_systems(
                Update,
                streaming::schedule_dense_materialization_generation
                    .in_set(VoxelUpdateSet::Generation),
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
                    VoxelPostUpdateSet::Collision.after(VoxelPostUpdateSet::SurfacePublication),
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
                streaming::publish_generated_materializations
                    .in_set(VoxelPostUpdateSet::DensePublication),
            )
            .add_systems(
                PostUpdate,
                async_pipeline::publish_completed_surface_derivations
                    .in_set(VoxelPostUpdateSet::SurfacePublication),
            )
            .add_systems(
                PostUpdate,
                async_pipeline::schedule_surface_derivations
                    .in_set(VoxelPostUpdateSet::SurfaceScheduling),
            )
            .add_systems(
                PostUpdate,
                manifestation::retire_orphaned_presentation_manifestations
                    .in_set(VoxelPostUpdateSet::ManifestationCleanup),
            )
            .add_systems(
                PostUpdate,
                manifestation::reconcile_presentation_manifestations
                    .in_set(VoxelPostUpdateSet::Membership),
            )
            .add_systems(
                PostUpdate,
                manifestation::rebuild_dirty_presentation_manifestations
                    .in_set(VoxelPostUpdateSet::Rebuild),
            )
            .add_systems(
                PostUpdate,
                (
                    manifestation::sync_presentation_manifestation_transforms
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
                manifestation::publish_voxel_capability_realizations
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
