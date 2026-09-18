//! Collision manifestation residency and collider construction.

use std::collections::HashMap;

use avian3d::prelude::{Collider, CollisionMargin, RigidBody};
use bevy::prelude::*;

use crate::{
    config::EngineConfig,
    spatial::{UsfScaleLayer, UsfViewFrame},
};

use super::{VoxelRenderAggregate, VoxelRenderAggregateRegistry};
use super::super::{
    MATERIALIZATION_CHUNK_SIZE, VoxelMaterializationChunkAddress, VoxelWorld,
    aggregate::VoxelMaterializationAggregateScope,
    physics,
};

/// Keeps collision manifestations resident only around the active interaction
/// region.
///
/// This stage is deliberately separate from mesh rebuilds. An unchanged visual
/// manifestation can gain or lose its Avian collider without rebuilding its
/// render mesh.
pub(crate) fn sync_manifestation_collision_residency(
    config: Res<EngineConfig>,
    mut commands: Commands,
    view: Res<UsfViewFrame>,
    worlds: Query<(&VoxelWorld, &UsfScaleLayer)>,
    aggregate_roots: Query<Option<&Collider>, With<VoxelRenderAggregate>>,
    registry: Res<VoxelRenderAggregateRegistry>,
) {
    for (&key, &entity) in &registry.aggregate_entities {
        let Some(members) = registry.groups.get(&key) else {
            continue;
        };
        let Ok((world, layer)) = worlds.get(key.world) else {
            continue;
        };

        let wants_collider = aggregate_collider_proximity_squared(
            &view,
            key.scope,
            layer,
            config.voxel.manifestation.physics_interaction_radius_native,
        )
        .is_some();
        let has_collider = aggregate_roots
            .get(entity)
            .ok()
            .flatten()
            .is_some();

        match (wants_collider, has_collider) {
            (true, false) => {
                let collider = build_aggregate_collider(key.scope, members, world);
                publish_collider_manifestation(&mut commands, entity, true, collider);
            }
            (false, true) => {
                publish_collider_manifestation(&mut commands, entity, false, None);
            }
            _ => {}
        }
    }
}

pub(super) fn publish_collider_manifestation(
    commands: &mut Commands,
    entity: Entity,
    requested: bool,
    collider: Option<Collider>,
) {
    let mut entity_commands = commands.entity(entity);
    if requested {
        if let Some(collider) = collider {
            entity_commands.insert((
                RigidBody::Static,
                collider,
                CollisionMargin(physics::VOXEL_COLLISION_MARGIN),
            ));
        } else {
            entity_commands.remove::<RigidBody>();
            entity_commands.remove::<Collider>();
            entity_commands.remove::<CollisionMargin>();
        }
    } else {
        entity_commands.remove::<RigidBody>();
        entity_commands.remove::<Collider>();
        entity_commands.remove::<CollisionMargin>();
    }
}

pub(crate) fn aggregate_collider_proximity_squared(
    view: &UsfViewFrame,
    scope: VoxelMaterializationAggregateScope,
    layer: &UsfScaleLayer,
    interaction_radius_native: f32,
) -> Option<f32> {
    if layer.scale() != view.dominant_scale() {
        return None;
    }

    let extent = scope.extent().native_units_per_axis() as f32;
    let minimum = scope
        .origin()
        .origin()
        .relative_native_bounded(view.anchor(), interaction_radius_native + extent * 2.0)
        .ok()?;
    let maximum = minimum + Vec3::splat(extent);
    let nearest = Vec3::new(
        0.0_f32.clamp(minimum.x, maximum.x),
        0.0_f32.clamp(minimum.y, maximum.y),
        0.0_f32.clamp(minimum.z, maximum.z),
    );
    let distance_squared = nearest.length_squared();
    (distance_squared <= interaction_radius_native * interaction_radius_native)
        .then_some(distance_squared)
}

fn build_aggregate_collider(
    scope: VoxelMaterializationAggregateScope,
    members: &HashMap<VoxelMaterializationChunkAddress, u64>,
    world: &VoxelWorld,
) -> Option<Collider> {
    let mut vertices = Vec::<Vec3>::new();
    let mut triangles = Vec::<[u32; 3]>::new();
    let bound =
        scope.extent().native_units_per_axis() as f32 + MATERIALIZATION_CHUNK_SIZE as f32 * 2.0;

    for (&address, &expected_revision) in members {
        let cache = world.materializations().surface(address)?;
        if cache.revision != expected_revision {
            return None;
        }

        let offset = address
            .origin()
            .relative_native_bounded(scope.origin().origin(), bound)
            .ok()?;
        let vertex_base = u32::try_from(vertices.len()).ok()?;
        vertices.extend(
            cache
                .surface
                .positions
                .iter()
                .map(|position| Vec3::from_array(*position) + offset),
        );

        for triangle in physics::owned_triangles(&cache.surface) {
            triangles.push([
                vertex_base.checked_add(triangle[0])?,
                vertex_base.checked_add(triangle[1])?,
                vertex_base.checked_add(triangle[2])?,
            ]);
        }
    }

    physics::build_trimesh_collider(vertices, triangles, "voxel aggregate")
}
