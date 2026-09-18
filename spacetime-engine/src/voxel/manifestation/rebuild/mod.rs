//! Mesh manifestation construction and publication.

use std::{collections::HashMap, time::Instant};

use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};

use crate::{
    config::EngineConfig,
    spatial::{UsfLocalScalePresentation, UsfScaleLayer, UsfScaleLayerFrames},
};

use super::{
    VoxelRenderAggregate, VoxelRenderAggregatePresentation, VoxelRenderAggregateRegistry,
    collision::publish_collider_manifestation,
};
use super::super::{
    MATERIALIZATION_CHUNK_SIZE, VoxelMaterializationChunkAddress, VoxelQueryPosition,
    VoxelWorld,
    aggregate::VoxelMaterializationAggregateScope,
    perf::VoxelPerfStats,
    streaming::VoxelPresentationMaterial,
};

/// Rebuilds dirty physical manifestations within the configured frame budget.
///
/// Mesh construction and publication live here. Collision data for a rebuilt
/// manifestation is produced only when that manifestation is currently inside
/// the physics interaction region.
pub(crate) fn rebuild_dirty_manifestations(
    config: Res<EngineConfig>,
    mut commands: Commands,
    layer_frames: Res<UsfScaleLayerFrames>,
    mut meshes: ResMut<Assets<Mesh>>,
    worlds: Query<(
        Entity,
        &VoxelWorld,
        &VoxelPresentationMaterial,
        &UsfScaleLayer,
    )>,
    mut aggregates: Query<&mut VoxelRenderAggregate>,
    aggregate_presentations: Query<Option<&Mesh3d>, With<VoxelRenderAggregatePresentation>>,
    mut registry: ResMut<VoxelRenderAggregateRegistry>,
    mut perf: ResMut<VoxelPerfStats>,
) {
    for _ in 0..config.voxel.manifestation.rebuild_budget_per_frame {
        let Some(key) = registry.dirty.iter().next().copied() else {
            break;
        };
        registry.dirty.remove(&key);

        let Some(members) = registry.groups.get(&key) else {
            continue;
        };

        if members.is_empty() {
            registry.groups.remove(&key);
            if let Some(entity) = registry.aggregate_entities.remove(&key) {
                commands.entity(entity).despawn();
            }
            continue;
        }

        let Ok((_, world, material, layer)) = worlds.get(key.world) else {
            registry.groups.remove(&key);
            registry
                .address_keys
                .retain(|(world, _), member_key| *world != key.world && *member_key != key);
            if let Some(entity) = registry.aggregate_entities.remove(&key) {
                commands.entity(entity).despawn();
            }
            continue;
        };

        let started = Instant::now();
        let Some(mesh) = build_aggregate_mesh(key.scope, members, world) else {
            registry.dirty.insert(key);
            continue;
        };

        let member_count = members.len();
        let mut mesh = Some(mesh);
        let mut root_entity = registry.aggregate_entities.get(&key).copied();

        if let Some(aggregate_entity) = root_entity
            && let Ok(mut aggregate) = aggregates.get_mut(aggregate_entity)
        {
            match aggregate_presentations.get(aggregate.presentation) {
                Ok(Some(mesh3d)) => {
                    let replacement = mesh.take().expect("aggregate mesh is published once");
                    if let Some(mut existing) = meshes.get_mut(&mesh3d.0) {
                        *existing = replacement;
                    } else {
                        commands
                            .entity(aggregate.presentation)
                            .insert(Mesh3d(meshes.add(replacement)));
                    }
                    aggregate.member_count = member_count;
                }
                Ok(None) => {
                    let replacement = mesh.take().expect("aggregate mesh is published once");
                    commands
                        .entity(aggregate.presentation)
                        .insert(Mesh3d(meshes.add(replacement)));
                    aggregate.member_count = member_count;
                }
                Err(_) => {
                    commands.entity(aggregate_entity).despawn();
                    registry.aggregate_entities.remove(&key);
                    root_entity = None;
                }
            }
        } else {
            root_entity = None;
        }

        let root = if let Some(root) = root_entity {
            root
        } else {
            let Some(local_translation) =
                aggregate_runtime_translation(world, layer, &layer_frames, key.scope)
            else {
                registry.dirty.insert(key);
                continue;
            };

            let mesh_handle = meshes.add(mesh.expect("aggregate mesh is still available"));
            let root = commands
                .spawn((
                    Name::new("Voxel Render Aggregate"),
                    *layer,
                    Transform::from_translation(local_translation),
                    Visibility::Inherited,
                ))
                .id();
            let presentation = commands
                .spawn((
                    Name::new("Voxel Render Aggregate Presentation"),
                    ChildOf(root),
                    VoxelRenderAggregatePresentation,
                    UsfLocalScalePresentation::new(layer.scale()),
                    Mesh3d(mesh_handle),
                    MeshMaterial3d(material.handle().clone()),
                    Transform::IDENTITY,
                    Visibility::Inherited,
                ))
                .id();
            commands.entity(root).insert(VoxelRenderAggregate {
                presentation,
                member_count,
                scope: key.scope,
            });
            registry.aggregate_entities.insert(key, root);
            root
        };

        // Any previous collider was derived from the old surface. Invalidate it
        // here; the following collision-residency stage independently decides
        // whether this manifestation currently needs a fresh collider.
        publish_collider_manifestation(&mut commands, root, false, None);
        perf.record_render_aggregate_rebuild(started.elapsed().as_micros() as u64);
    }
}

fn aggregate_runtime_translation(
    world: &VoxelWorld,
    layer: &UsfScaleLayer,
    frames: &UsfScaleLayerFrames,
    scope: VoxelMaterializationAggregateScope,
) -> Option<Vec3> {
    let world_origin = VoxelQueryPosition::new(*world.origin());
    let relative = scope
        .origin()
        .query_origin()
        .relative_to(world_origin, 1_000_000.0)
        .ok()?;
    let absolute = bevy::math::DVec3::new(relative.x as f64, relative.y as f64, relative.z as f64);
    Some(frames.runtime_from_absolute(layer.scale(), absolute))
}

fn build_aggregate_mesh(
    scope: VoxelMaterializationAggregateScope,
    members: &HashMap<VoxelMaterializationChunkAddress, u64>,
    world: &VoxelWorld,
) -> Option<Mesh> {
    let mut positions = Vec::<[f32; 3]>::new();
    let mut normals = Vec::<[f32; 3]>::new();
    let mut colors = Vec::<[f32; 4]>::new();
    let mut uvs = Vec::<[f32; 2]>::new();
    let mut tangents = Vec::<[f32; 4]>::new();
    let mut indices = Vec::<u32>::new();

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
        let vertex_base = u32::try_from(positions.len()).ok()?;
        let vertex_count = cache.surface.positions.len();

        positions.extend(
            cache
                .surface
                .positions
                .iter()
                .map(|position| (Vec3::from_array(*position) + offset).to_array()),
        );
        normals.extend_from_slice(&cache.surface.normals);
        colors.extend((0..vertex_count).map(|_| cache.debug_color));
        uvs.extend_from_slice(&cache.surface.uvs);
        tangents.extend_from_slice(&cache.surface.tangents);
        for &index in &cache.surface.indices {
            indices.push(vertex_base.checked_add(index)?);
        }
    }

    if positions.is_empty() || indices.is_empty() {
        return None;
    }

    Some(
        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
        .with_inserted_attribute(Mesh::ATTRIBUTE_TANGENT, tangents)
        .with_inserted_indices(Indices::U32(indices)),
    )
}
