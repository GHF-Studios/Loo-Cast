//! Reconciles store-backed voxel facts and surface manifestations into generic capability coverage.

use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    config::EngineConfig,
    ecs::{UsfAuthorityPartitionOf, UsfLogicalRealizationOf},
    spatial::{
        UsfCapabilityCoverageBatch, UsfCapabilityCoverageRecord,
        UsfCapabilityRealization, UsfScaleLayer, UsfScaleRoleMask,
    },
};

use super::{
    VoxelMaterializationRuntime,
    collision::{collision_requested, VoxelCollisionAggregateRegistry},
};
use super::super::{
    VoxelCollisionDisabled,
    VoxelRealizationDemandSnapshot,
    VoxelStreaming,
    MATERIALIZATION_CHUNK_SIZE, VoxelEditingDisabled, VoxelWorld,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CapabilityWorldSignature {
    materializations: u64,
    streaming: Option<u64>,
    collision_disabled: bool,
    editing_disabled: bool,
    scale: crate::spatial::SpatialScale,
    logical: Option<Entity>,
}

#[derive(Default)]
pub(in crate::voxel) struct CapabilitySyncCache {
    initialized: bool,
    worlds: HashMap<Entity, CapabilityWorldSignature>,
}

pub(in crate::voxel) fn sync_capability_realizations(
    config: Res<EngineConfig>,
    realization_demand: Res<VoxelRealizationDemandSnapshot>,
    mut commands: Commands,
    worlds: Query<(
        Entity,
        &VoxelWorld,
        &UsfScaleLayer,
        Option<&UsfLogicalRealizationOf>,
        Option<&VoxelStreaming>,
        Option<&VoxelCollisionDisabled>,
        Option<&VoxelEditingDisabled>,
    )>,
    authority_partitions: Query<&UsfAuthorityPartitionOf>,
    collision_registry: Res<VoxelCollisionAggregateRegistry>,
    mut runtimes: Query<(
        Entity,
        &VoxelMaterializationRuntime,
        Option<&mut UsfCapabilityRealization>,
    )>,
    mut coverage_batches: Query<&mut UsfCapabilityCoverageBatch>,
    mut cache: Local<CapabilitySyncCache>,
) {
    let mut world_signatures =
        HashMap::<Entity, CapabilityWorldSignature>::with_capacity(worlds.iter().len());
    for (
        world_entity,
        world,
        layer,
        logical_realization,
        streaming,
        collision_disabled,
        editing_disabled,
    ) in &worlds
    {
        world_signatures.insert(
            world_entity,
            CapabilityWorldSignature {
                materializations: world.materializations().capability_revision(),
                streaming: streaming.map(VoxelStreaming::collision_policy_revision),
                collision_disabled: collision_disabled.is_some(),
                editing_disabled: editing_disabled.is_some(),
                scale: layer.scale(),
                logical: logical_realization.map(|logical| logical.0),
            },
        );
    }

    if cache.initialized
        && !config.is_changed()
        && !realization_demand.is_changed()
        && !collision_registry.is_changed()
        && cache.worlds == world_signatures
    {
        return;
    }

    cache.initialized = true;
    cache.worlds = world_signatures;

    let _span = bevy::log::info_span!("voxel_capability.reconcile_changed").entered();
    let half_extent = Vec3::splat(MATERIALIZATION_CHUNK_SIZE as f32 * 0.5);
    let interaction_padding = config
        .voxel
        .manifestation
        .physics_interaction_radius_native
        .max(0.0);

    // segmented-materialization-coverage-v1
    //
    // No-surface realization facts are store-owned. Publish them as one
    // deterministic batch on the VoxelWorld entity instead of manufacturing a
    // runtime entity for every uniform air/solid materialization.
    for (
        world_entity,
        world,
        layer,
        logical_realization,
        _streaming,
        _collision_disabled,
        editing_disabled,
    ) in &worlds
    {
        let authority = logical_realization
            .and_then(|logical| authority_partitions.get(logical.0).ok())
            .map_or(world_entity, |partition| partition.0);

        let mut keys = world.materializations().active_keys().collect::<Vec<_>>();
        keys.sort_by_key(|key| key.components());

        let mut records = Vec::<UsfCapabilityCoverageRecord>::new();
        for key in keys {
            let Some(revision) =
                world.materializations().active_derived_revision(key)
            else {
                continue;
            };
            if world.materializations().surface(key).is_some() {
                continue;
            }

            let Ok(center) = world
                .materialization_address(key)
                .and_then(|address| address.center())
            else {
                continue;
            };

            let mut roles =
                UsfScaleRoleMask::REALIZATION.union(UsfScaleRoleMask::PRESENTATION);
            if editing_disabled.is_none() {
                roles = roles.union(UsfScaleRoleMask::EDITING);
            }

            records.push(UsfCapabilityCoverageRecord::new(
                authority,
                layer.scale(),
                center,
                half_extent,
                roles,
                revision,
            ));
        }

        let next = UsfCapabilityCoverageBatch::new(records);
        match coverage_batches.get_mut(world_entity) {
            Ok(mut current) => {
                if *current != next {
                    *current = next;
                }
            }
            Err(_) => {
                commands.entity(world_entity).insert(next);
            }
        }
    }

    for (entity, runtime, existing) in &mut runtimes {
        if !runtime.active() {
            if let Some(mut realization) = existing {
                realization.set_roles(UsfScaleRoleMask::NONE);
            }
            continue;
        }

        let Ok((
            world_entity,
            world,
            layer,
            logical_realization,
            streaming,
            collision_disabled,
            editing_disabled,
        )) = worlds.get(runtime.world())
        else {
            if let Some(mut realization) = existing {
                realization.set_roles(UsfScaleRoleMask::NONE);
            }
            continue;
        };

        let Ok(center) = world
            .materialization_address(runtime.key())
            .and_then(|address| address.center())
        else {
            if let Some(mut realization) = existing {
                realization.set_roles(UsfScaleRoleMask::NONE);
            }
            continue;
        };

        let derived_current = world
            .materializations()
            .active_derived_revision(runtime.key())
            == Some(runtime.revision());

        let mut roles = UsfScaleRoleMask::NONE;
        if derived_current {
            let collision_current = collision_registry.member_current(
                runtime.world(),
                runtime.key(),
                runtime.revision(),
            );
            let rigid_current = world
                .materializations()
                .surface(runtime.key())
                .is_some_and(|cache| {
                    cache.revision == runtime.revision()
                        && cache.surface.has_rigid_triangles()
                });

            // collision-before-presentation-v2
            //
            // PRESENTATION is a readiness claim, not just "a mesh exists".
            // Whenever this exact rigid materialization belongs to current
            // collision demand, physical presentation waits for the independently
            // owned collision aggregate to publish the same revision.
            //
            // Known-empty, translucent/non-rigid, contextual-only and explicitly
            // collision-disabled materializations keep their independent
            // presentation semantics.
            let collision_required = collision_disabled.is_none()
                && rigid_current
                && streaming.is_some_and(|streaming| {
                    collision_requested(
                        runtime.world(),
                        runtime.key(),
                        world,
                        layer,
                        streaming,
                        &realization_demand,
                        interaction_padding,
                    )
                });

            // Derived-current data is immediately realized truth. Physical
            // presentation becomes publishable only after its collision
            // readiness contract is satisfied.
            roles = UsfScaleRoleMask::REALIZATION;
            if !collision_required || collision_current {
                roles = roles.union(UsfScaleRoleMask::PRESENTATION);
            }
            if collision_current {
                roles = roles.union(UsfScaleRoleMask::COLLISION);
            }
            if editing_disabled.is_none() {
                roles = roles.union(UsfScaleRoleMask::EDITING);
            }
        }

        // Semantic worlds publish capability facts against the semantic
        // authority reached through the generic ownership graph. Standalone
        // voxel worlds remain valid capability-local authorities.
        let authority = logical_realization
            .and_then(|logical| authority_partitions.get(logical.0).ok())
            .map_or(world_entity, |partition| partition.0);
        let next = UsfCapabilityRealization::new(
            authority,
            layer.scale(),
            center,
            half_extent,
            roles,
            runtime.revision(),
        );

        if let Some(mut realization) = existing {
            if *realization != next {
                *realization = next;
            }
        } else {
            commands.entity(entity).insert(next);
        }
    }
}
