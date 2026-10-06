//! Publishes generic capability readiness from voxel stores and disposable manifestations.

use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    config::EngineConfig,
    ecs::{UsfAuthorityPartitionOf, UsfLogicalRealizationOf},
    spatial::{
        UsfCapabilityCoverageFact, UsfCapabilityCoveragePublication, UsfCapabilityRealization,
        UsfScaleLayer, UsfScaleRoleMask,
    },
};

use super::super::{
    CelestialVoxelRealizationFrame, MATERIALIZATION_CHUNK_SIZE, VoxelCollisionDisabled,
    VoxelEditingDisabled, VoxelMaterializationResidency, VoxelRealizationDemandSnapshot,
    VoxelScaleRealization,
};
use super::{
    VoxelPresentationManifestation,
    collision::{VoxelCollisionAggregateRegistry, collision_requested},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CapabilityRealizationSignature {
    materializations: u64,
    streaming: Option<u64>,
    collision_disabled: bool,
    editing_disabled: bool,
    scale: crate::spatial::SpatialScale,
    logical: Option<Entity>,
    frame_revision: u64,
}

#[derive(Default)]
pub(in crate::voxel) struct CapabilityPublicationCache {
    initialized: bool,
    worlds: HashMap<Entity, CapabilityRealizationSignature>,
}

pub(in crate::voxel) fn publish_voxel_capability_realizations(
    config: Res<EngineConfig>,
    realization_demand: Res<VoxelRealizationDemandSnapshot>,
    mut commands: Commands,
    worlds: Query<(
        Entity,
        &VoxelScaleRealization,
        &UsfScaleLayer,
        Option<&UsfLogicalRealizationOf>,
        Option<&VoxelMaterializationResidency>,
        Option<&VoxelCollisionDisabled>,
        Option<&VoxelEditingDisabled>,
        Option<&CelestialVoxelRealizationFrame>,
    )>,
    authority_partitions: Query<&UsfAuthorityPartitionOf>,
    collision_registry: Res<VoxelCollisionAggregateRegistry>,
    mut runtimes: Query<(
        Entity,
        &VoxelPresentationManifestation,
        Option<&mut UsfCapabilityRealization>,
    )>,
    mut coverage_publications: Query<&mut UsfCapabilityCoveragePublication>,
    mut cache: Local<CapabilityPublicationCache>,
) {
    let mut world_signatures =
        HashMap::<Entity, CapabilityRealizationSignature>::with_capacity(worlds.iter().len());
    for (
        world_entity,
        world,
        layer,
        logical_realization,
        streaming,
        collision_disabled,
        editing_disabled,
        celestial_frame,
    ) in &worlds
    {
        world_signatures.insert(
            world_entity,
            CapabilityRealizationSignature {
                materializations: world.materializations().capability_revision(),
                streaming: streaming.map(VoxelMaterializationResidency::collision_policy_revision),
                collision_disabled: collision_disabled.is_some(),
                editing_disabled: editing_disabled.is_some(),
                scale: layer.scale(),
                logical: logical_realization.map(|logical| logical.0),
                frame_revision: celestial_frame.map_or(0, |frame| frame.revision()),
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

    //
    // No-surface realization facts are store-owned. Publish them as one
    // deterministic batch on the VoxelScaleRealization entity instead of manufacturing a
    // runtime entity for every uniform air/solid materialization.
    for (
        world_entity,
        world,
        layer,
        logical_realization,
        streaming,
        collision_disabled,
        editing_disabled,
        _celestial_frame,
    ) in &worlds
    {
        let authority = logical_realization
            .and_then(|logical| authority_partitions.get(logical.0).ok())
            .map_or(world_entity, |partition| partition.0);

        let mut keys = world.materializations().active_keys().collect::<Vec<_>>();
        keys.sort_by_key(|key| key.components());

        let mut facts = Vec::<UsfCapabilityCoverageFact>::new();
        for key in keys {
            let Some(dense_revision) = world.materializations().active_dense_revision(key) else {
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

            let requested = streaming.map_or_else(
                || {
                    let mut roles =
                        UsfScaleRoleMask::REALIZATION.union(UsfScaleRoleMask::PRESENTATION);
                    if collision_disabled.is_none() {
                        roles = roles.union(UsfScaleRoleMask::COLLISION);
                    }
                    if editing_disabled.is_none() {
                        roles = roles.union(UsfScaleRoleMask::EDITING);
                    }
                    roles
                },
                |streaming| streaming.effective_roles(key),
            );

            let mut roles = UsfScaleRoleMask::REALIZATION;
            let derived_current =
                world.materializations().active_derived_revision(key) == Some(dense_revision);

            // A derived-current no-surface result is a positive fact: there is
            // nothing to draw/collide here, so requested presentation is ready.
            if derived_current && requested.contains(UsfScaleRoleMask::PRESENTATION) {
                roles = roles.union(UsfScaleRoleMask::PRESENTATION);
            }
            if requested.contains(UsfScaleRoleMask::EDITING) && editing_disabled.is_none() {
                // Editing owns dense semantic working data, not a mesh.
                roles = roles.union(UsfScaleRoleMask::EDITING);
            }

            facts.push(UsfCapabilityCoverageFact::new(
                authority,
                layer.scale(),
                center,
                half_extent,
                roles,
                dense_revision,
            ));
        }

        let next = UsfCapabilityCoveragePublication::new(facts);
        match coverage_publications.get_mut(world_entity) {
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
            _celestial_frame,
        )) = worlds.get(runtime.realization())
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

        let requested = streaming.map_or_else(
            || {
                let mut roles = UsfScaleRoleMask::REALIZATION.union(UsfScaleRoleMask::PRESENTATION);
                if collision_disabled.is_none() {
                    roles = roles.union(UsfScaleRoleMask::COLLISION);
                }
                if editing_disabled.is_none() {
                    roles = roles.union(UsfScaleRoleMask::EDITING);
                }
                roles
            },
            |streaming| streaming.effective_roles(runtime.key()),
        );

        let mut roles = UsfScaleRoleMask::NONE;
        if derived_current {
            let collision_current = collision_registry.member_current(
                runtime.realization(),
                runtime.key(),
                runtime.revision(),
            );
            let rigid_current =
                world
                    .materializations()
                    .surface(runtime.key())
                    .is_some_and(|cache| {
                        cache.revision == runtime.revision() && cache.surface.has_rigid_triangles()
                    });

            //
            // PRESENTATION is a readiness claim, not just "a mesh exists".
            // Whenever this exact rigid materialization belongs to current
            // collision demand, physical presentation waits for the independently
            // owned collision aggregate to publish the same revision.
            //
            // Known-empty, translucent/non-rigid, contextual-only and explicitly
            // collision-disabled materializations keep their independent
            // presentation semantics.
            let collision_required = requested.contains(UsfScaleRoleMask::COLLISION)
                && collision_disabled.is_none()
                && rigid_current
                && streaming.is_some_and(|streaming| {
                    collision_requested(
                        runtime.realization(),
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
            if requested.contains(UsfScaleRoleMask::PRESENTATION)
                && (!collision_required || collision_current)
            {
                roles = roles.union(UsfScaleRoleMask::PRESENTATION);
            }
            if requested.contains(UsfScaleRoleMask::COLLISION) && collision_current {
                roles = roles.union(UsfScaleRoleMask::COLLISION);
            }
            if requested.contains(UsfScaleRoleMask::EDITING) && editing_disabled.is_none() {
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
