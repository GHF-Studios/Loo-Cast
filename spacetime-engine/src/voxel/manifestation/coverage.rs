//! Reconciles voxel runtime manifestations into the generic capability lifecycle.

use bevy::prelude::*;

use crate::{
    ecs::{UsfAuthorityPartitionOf, UsfLogicalRealizationOf},
    spatial::{
        UsfCapabilityRealization, UsfScaleLayer, UsfScaleRoleMask,
    },
};

use super::{
    VoxelMaterializationRuntime,
    collision::VoxelCollisionAggregateRegistry,
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
    collision_registry: Res<VoxelCollisionAggregateRegistry>,
    mut runtimes: Query<(
        Entity,
        &VoxelMaterializationRuntime,
        Option<&mut UsfCapabilityRealization>,
    )>,
) {
    let half_extent = Vec3::splat(MATERIALIZATION_CHUNK_SIZE as f32 * 0.5);

    for (entity, runtime, existing) in &mut runtimes {
        let Ok((world_entity, world, layer, logical_realization, editing_disabled)) =
            worlds.get(runtime.world())
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
            // A current empty derived result is still realized presentation
            // truth. Mesh existence is not capability existence: known-empty
            // space must erase a coarse approximation after
            // excavation/caves/void generation.
            roles = UsfScaleRoleMask::REALIZATION
                .union(UsfScaleRoleMask::PRESENTATION);

            let collision_current = collision_registry.member_current(
                runtime.world(),
                runtime.key(),
                runtime.revision(),
            );
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
