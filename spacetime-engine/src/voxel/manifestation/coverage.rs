//! Publishes actual voxel realization coverage into the generic Scale Stack.

use avian3d::prelude::Collider;
use bevy::prelude::*;

use crate::{
    ecs::UsfManifestationOf,
    spatial::{
        UsfScaleCoverage, UsfScaleCoverageSnapshot, UsfScaleLayer, UsfScaleRoleMask,
    },
};

use super::{VoxelManifestation, VoxelManifestationRegistry};
use super::super::{MATERIALIZATION_CHUNK_SIZE, VoxelEditingDisabled, VoxelWorld};

pub(in crate::voxel) fn publish_scale_coverage(
    worlds: Query<(
        Entity,
        &VoxelWorld,
        &UsfScaleLayer,
        Option<&UsfManifestationOf>,
        Option<&VoxelEditingDisabled>,
    )>,
    roots: Query<Option<&Collider>, With<VoxelManifestation>>,
    registry: Res<VoxelManifestationRegistry>,
    mut coverage: ResMut<UsfScaleCoverageSnapshot>,
) {
    let extent = MATERIALIZATION_CHUNK_SIZE as f32;
    let half_extent = Vec3::splat(extent * 0.5);

    for (&key, &root) in &registry.entities {
        let Some(&expected_revision) = registry.revisions.get(&key) else { continue; };
        let Ok((world_entity, world, layer, manifestation, editing_disabled)) = worlds.get(key.world) else {
            continue;
        };
        let Some(surface) = world.materializations().surface(key.address) else { continue; };
        if surface.revision != expected_revision { continue; }

        let Ok(center) = key.address.query_origin().translated(Vec3::splat(extent * 0.5)) else {
            continue;
        };

        let Ok(collider) = roots.get(root) else { continue; };
        let mut roles = UsfScaleRoleMask::REALIZATION.union(UsfScaleRoleMask::PRESENTATION);
        if collider.is_some() {
            roles = roles.union(UsfScaleRoleMask::COLLISION);
        }
        if editing_disabled.is_none() {
            roles = roles.union(UsfScaleRoleMask::EDITING);
        }

        let authority = manifestation.map_or(world_entity, |manifestation| manifestation.0);
        coverage.publish(UsfScaleCoverage::new(
            authority,
            layer.scale(),
            center.usf(),
            half_extent,
            roles,
        ));
    }
}
