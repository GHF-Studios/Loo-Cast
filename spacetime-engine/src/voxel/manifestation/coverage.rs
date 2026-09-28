//! Reconciles voxel runtime manifestations into the generic capability lifecycle.

use avian3d::prelude::Collider;
use bevy::prelude::*;

use crate::{
    ecs::{UsfAuthorityPartitionOf, UsfLogicalRealizationOf},
    spatial::{
        UsfCapabilityRealization, UsfScaleLayer, UsfScaleRoleMask,
    },
};

use super::{
    VoxelMaterializationRuntime,
    collision::VoxelMaterializationColliderRevision,
};
use super::super::{
    MATERIALIZATION_CHUNK_SIZE, VoxelEditingDisabled, VoxelWorld,
};

pub(in crate::voxel) fn sync_capability_realizations(
    mut commands: Commands,
    worlds: Query<(
        Entity,
        &VoxelWorld,
        &UsfScaleLayer,
        Option<&UsfLogicalRealizationOf>,
        Option<&VoxelEditingDisabled>,
    )>,
    authority_partitions: Query<&UsfAuthorityPartitionOf>,
    mut runtimes: Query<(
        Entity,
        &VoxelMaterializationRuntime,
        Option<&Collider>,
        Option<&VoxelMaterializationColliderRevision>,
        Option<&mut UsfCapabilityRealization>,
    )>,
) {
    let half_extent = Vec3::splat(MATERIALIZATION_CHUNK_SIZE as f32 * 0.5);

    for (entity, runtime, collider, collider_revision, existing) in &mut runtimes {
        let Ok((world_entity, world, layer, logical_realization, editing_disabled)) =
            worlds.get(runtime.world())
        else {
            if let Some(mut realization) = existing {
                realization.set_roles(UsfScaleRoleMask::NONE);
            }
            continue;
        };

        let Ok(center) = runtime.address().center() else {
            if let Some(mut realization) = existing {
                realization.set_roles(UsfScaleRoleMask::NONE);
            }
            continue;
        };

        let surface_current = world
            .materializations()
            .surface(runtime.address())
            .is_some_and(|surface| surface.revision == runtime.revision());

        let mut roles = UsfScaleRoleMask::NONE;
        if surface_current {
            // A current empty derived surface is still realized presentation
            // truth. It must be able to erase a coarse approximation after
            // excavation/caves/void generation even though it owns no mesh.
            roles = UsfScaleRoleMask::REALIZATION
                .union(UsfScaleRoleMask::PRESENTATION);

            let collision_current = collider.is_some()
                && collider_revision.is_some_and(|revision| {
                    revision.revision() == runtime.revision()
                });
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
