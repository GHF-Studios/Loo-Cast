//! Publishes actual voxel realization coverage into the generic Scale Stack.

use avian3d::prelude::Collider;
use bevy::prelude::*;

use crate::{
    ecs::UsfManifestationOf,
    spatial::{
        UsfScaleCoverage, UsfScaleCoverageSnapshot, UsfScaleLayer, UsfScaleRoleMask,
    },
};

use super::{
    VoxelMaterializationRuntime,
    collision::VoxelMaterializationColliderRevision,
};
use super::super::{MATERIALIZATION_CHUNK_SIZE, VoxelEditingDisabled, VoxelWorld};

pub(in crate::voxel) fn publish_scale_coverage(
    worlds: Query<(
        Entity,
        &VoxelWorld,
        &UsfScaleLayer,
        Option<&UsfManifestationOf>,
        Option<&VoxelEditingDisabled>,
    )>,
    runtimes: Query<(
        &VoxelMaterializationRuntime,
        Option<&Collider>,
        Option<&VoxelMaterializationColliderRevision>,
    )>,
    mut coverage: ResMut<UsfScaleCoverageSnapshot>,
) {
    let extent = MATERIALIZATION_CHUNK_SIZE as f32;
    let half_extent = Vec3::splat(extent * 0.5);

    for (runtime, collider, collider_revision) in &runtimes {
        let Ok((world_entity, world, layer, manifestation, editing_disabled)) =
            worlds.get(runtime.world())
        else {
            continue;
        };
        let Some(surface) = world.materializations().surface(runtime.address()) else {
            continue;
        };
        if surface.revision != runtime.revision() {
            continue;
        }

        let Ok(center) = runtime.address().center() else {
            continue;
        };

        let mut roles =
            UsfScaleRoleMask::REALIZATION.union(UsfScaleRoleMask::PRESENTATION);
        let collision_current = collider.is_some()
            && collider_revision
                .is_some_and(|revision| revision.revision() == runtime.revision());
        if collision_current {
            roles = roles.union(UsfScaleRoleMask::COLLISION);
        }
        if editing_disabled.is_none() {
            roles = roles.union(UsfScaleRoleMask::EDITING);
        }

        let authority = manifestation.map_or(world_entity, |manifestation| manifestation.0);
        coverage.publish(UsfScaleCoverage::new(
            authority,
            layer.scale(),
            center,
            half_extent,
            roles,
        ));
    }
}
