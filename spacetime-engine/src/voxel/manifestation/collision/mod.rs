//! Collision residency for realized voxel materialization runtimes.
//!
//! Collision follows physical/materialization demand. Presentation/view state is
//! deliberately absent: moving a camera must never create or retire physics.

use avian3d::prelude::{Collider, CollisionMargin};
use bevy::prelude::*;

use crate::{
    config::EngineConfig,
    spatial::UsfScaleLayer,
};

use super::VoxelMaterializationRuntime;

/// Surface-cache revision currently encoded by this runtime's physics collider.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::voxel) struct VoxelMaterializationColliderRevision(u64);
use super::super::{
    VoxelCollisionDisabled, VoxelMaterializationChunkAddress,
    VoxelRealizationDemandSnapshot, VoxelWorld, physics,
};

pub(in crate::voxel) fn sync_manifestation_collision_residency(
    config: Res<EngineConfig>,
    realization_demand: Res<VoxelRealizationDemandSnapshot>,
    mut commands: Commands,
    worlds: Query<(&VoxelWorld, &UsfScaleLayer, Option<&VoxelCollisionDisabled>)>,
    runtimes: Query<(
        Entity,
        Ref<VoxelMaterializationRuntime>,
        Option<&Collider>,
        Option<&VoxelMaterializationColliderRevision>,
    )>,
) {
    let runtime_changed = runtimes
        .iter()
        .any(|(_, runtime, _, _)| runtime.is_changed());
    if !config.is_changed() && !realization_demand.is_changed() && !runtime_changed {
        return;
    }

    let interaction_padding =
        config.voxel.manifestation.physics_interaction_radius_native.max(0.0);

    for (entity, runtime, collider, collider_revision) in &runtimes {
        let Ok((world, layer, collision_disabled)) = worlds.get(runtime.world()) else {
            continue;
        };

        let has_rigid_surface = world
            .materializations()
            .surface(runtime.address())
            .is_some_and(|cache| {
                cache.revision == runtime.revision() && cache.surface.has_rigid_triangles()
            });

        let has_collision_demand = realization_demand
            .scopes_for(runtime.world())
            .filter(|scope| scope.scale() == layer.scale())
            .any(|scope| {
                runtime
                    .address()
                    .distance_squared_to_region(
                        &scope.center(),
                        scope.half_extent_native(),
                        interaction_padding,
                    )
                    .is_some_and(|distance_squared| {
                        distance_squared <= interaction_padding * interaction_padding
                    })
            });

        let wants_collider =
            collision_disabled.is_none() && has_rigid_surface && has_collision_demand;
        let collider_current = collider.is_some()
            && collider_revision
                .is_some_and(|revision| revision.0 == runtime.revision());

        if wants_collider && !collider_current {
            let collider = build_materialization_collider(
                runtime.address(),
                runtime.revision(),
                world,
            );
            publish_collider_manifestation(
                &mut commands,
                entity,
                true,
                collider,
                Some(runtime.revision()),
            );
        } else if !wants_collider && (collider.is_some() || collider_revision.is_some()) {
            publish_collider_manifestation(&mut commands, entity, false, None, None);
        }
    }
}

pub(super) fn publish_collider_manifestation(
    commands: &mut Commands,
    entity: Entity,
    requested: bool,
    collider: Option<Collider>,
    revision: Option<u64>,
) {
    let mut entity_commands = commands.entity(entity);

    if requested {
        if let (Some(collider), Some(revision)) = (collider, revision) {
            entity_commands.insert((
                collider,
                CollisionMargin(physics::VOXEL_COLLISION_MARGIN),
                VoxelMaterializationColliderRevision(revision),
            ));
        } else {
            entity_commands.remove::<Collider>();
            entity_commands.remove::<CollisionMargin>();
            entity_commands.remove::<VoxelMaterializationColliderRevision>();
        }
    } else {
        entity_commands.remove::<Collider>();
        entity_commands.remove::<CollisionMargin>();
        entity_commands.remove::<VoxelMaterializationColliderRevision>();
    }
}

fn build_materialization_collider(
    address: VoxelMaterializationChunkAddress,
    expected_revision: u64,
    world: &VoxelWorld,
) -> Option<Collider> {
    let cache = world.materializations().surface(address)?;
    if cache.revision != expected_revision {
        return None;
    }

    let vertices = cache
        .surface
        .positions
        .iter()
        .copied()
        .map(Vec3::from_array)
        .collect();
    let triangles = physics::owned_triangles(&cache.surface);

    physics::build_trimesh_collider(vertices, triangles, "voxel materialization runtime")
}
