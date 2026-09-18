//! Same-resolution aggregate voxel presentation.
//!
//! Authoritative materialization remains the 10-native-unit [`VoxelChunk`].
//! This module only groups already-extracted chunk surfaces into fewer disposable
//! render meshes. It does not change sample density, semantic identity, USF scale,
//! or physics ownership.

use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

use bevy::{
    asset::RenderAssetUsages,
    ecs::lifecycle::RemovedComponents,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};

use crate::spatial::{UsfLocalScalePresentation, UsfScaleLayer, UsfScaleLayerFrames};

use super::{
    MATERIALIZATION_CHUNK_SIZE, VoxelChunkOf, VoxelMaterializationChunkAddress, VoxelQueryPosition,
    VoxelWorld,
    aggregate::{VoxelMaterializationAggregateExtent, VoxelMaterializationAggregateScope},
    mesh::VoxelSurface,
    perf::VoxelPerfStats,
    streaming::VoxelStreaming,
};

const RENDER_AGGREGATE_EXTENT: VoxelMaterializationAggregateExtent =
    VoxelMaterializationAggregateExtent::FORTY;
const RENDER_AGGREGATE_REBUILD_BUDGET_PER_FRAME: usize = 8;

#[derive(Component)]
pub(crate) struct VoxelChunkRenderSurface {
    pub(crate) revision: u64,
    surface: VoxelSurface,
    debug_color: [f32; 4],
}

impl VoxelChunkRenderSurface {
    pub(crate) fn new(revision: u64, surface: VoxelSurface, debug_color: [f32; 4]) -> Self {
        Self {
            revision,
            surface,
            debug_color,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct AggregateKey {
    world: Entity,
    scope: VoxelMaterializationAggregateScope,
}

/// Root entity for one disposable same-resolution render cache.
#[derive(Component)]
pub(crate) struct VoxelRenderAggregate {
    presentation: Entity,
    member_count: usize,
}

impl VoxelRenderAggregate {
    pub(crate) const fn member_count(&self) -> usize {
        self.member_count
    }
}

/// Marks the only `Mesh3d` entity created for one render aggregate.
#[derive(Component, Debug, Default, Clone, Copy)]
pub(crate) struct VoxelRenderAggregatePresentation;

/// Incremental membership/dirty registry.
///
/// This deliberately avoids scanning every resident chunk just to discover that
/// almost nothing changed. Surface insert/replace/remove events update the
/// relevant aggregate only.
#[derive(Resource, Default)]
pub(crate) struct VoxelRenderAggregateRegistry {
    groups: HashMap<AggregateKey, HashMap<Entity, u64>>,
    entity_keys: HashMap<Entity, AggregateKey>,
    dirty: HashSet<AggregateKey>,
    aggregate_entities: HashMap<AggregateKey, Entity>,
}

pub(crate) fn sync_render_aggregates(
    mut commands: Commands,
    layer_frames: Res<UsfScaleLayerFrames>,
    mut meshes: ResMut<Assets<Mesh>>,
    worlds: Query<(&VoxelWorld, &VoxelStreaming, &UsfScaleLayer)>,
    changed_surfaces: Query<
        (
            Entity,
            &VoxelChunkOf,
            &VoxelMaterializationChunkAddress,
            &VoxelChunkRenderSurface,
        ),
        Changed<VoxelChunkRenderSurface>,
    >,
    surface_caches: Query<(&VoxelMaterializationChunkAddress, &VoxelChunkRenderSurface)>,
    mut removed_surfaces: RemovedComponents<VoxelChunkRenderSurface>,
    mut aggregates: Query<&mut VoxelRenderAggregate>,
    aggregate_presentations: Query<Option<&Mesh3d>, With<VoxelRenderAggregatePresentation>>,
    mut registry: ResMut<VoxelRenderAggregateRegistry>,
    mut perf: ResMut<VoxelPerfStats>,
) {
    for (entity, chunk_of, address, surface) in &changed_surfaces {
        let Ok((world, _, _)) = worlds.get(chunk_of.world) else {
            continue;
        };
        let Ok(scope) = VoxelMaterializationAggregateScope::containing(
            world,
            *address,
            RENDER_AGGREGATE_EXTENT,
        ) else {
            warn!(
                ?address,
                "voxel render aggregate scope could not be derived canonically"
            );
            continue;
        };
        let key = AggregateKey {
            world: chunk_of.world,
            scope,
        };

        if let Some(previous) = registry.entity_keys.insert(entity, key)
            && previous != key
        {
            remove_member(&mut registry, previous, entity);
        }

        registry
            .groups
            .entry(key)
            .or_default()
            .insert(entity, surface.revision);
        registry.dirty.insert(key);
    }

    for entity in removed_surfaces.read() {
        let Some(key) = registry.entity_keys.remove(&entity) else {
            continue;
        };
        remove_member(&mut registry, key, entity);
    }

    let dirty = registry
        .dirty
        .iter()
        .copied()
        .take(RENDER_AGGREGATE_REBUILD_BUDGET_PER_FRAME)
        .collect::<Vec<_>>();

    for key in dirty {
        registry.dirty.remove(&key);

        let members = registry
            .groups
            .get(&key)
            .map(|members| {
                members
                    .iter()
                    .map(|(&entity, &revision)| (entity, revision))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        if members.is_empty() {
            registry.groups.remove(&key);
            if let Some(entity) = registry.aggregate_entities.remove(&key) {
                commands.entity(entity).despawn();
            }
            continue;
        }

        let Ok((world, streaming, layer)) = worlds.get(key.world) else {
            registry.groups.remove(&key);
            registry
                .entity_keys
                .retain(|_, member_key| *member_key != key);
            if let Some(entity) = registry.aggregate_entities.remove(&key) {
                commands.entity(entity).despawn();
            }
            continue;
        };

        let started = Instant::now();
        let Some(mesh) = build_aggregate_mesh(key.scope, &members, &surface_caches) else {
            // A removal/change may have raced this dirty key. Keep it dirty;
            // the membership event will settle before the next attempt.
            registry.dirty.insert(key);
            continue;
        };

        let member_count = members.len();
        let mut mesh = Some(mesh);
        let mut existing_published = false;
        if let Some(aggregate_entity) = registry.aggregate_entities.get(&key).copied()
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
                    existing_published = true;
                }
                Ok(None) => {
                    let replacement = mesh.take().expect("aggregate mesh is published once");
                    commands
                        .entity(aggregate.presentation)
                        .insert(Mesh3d(meshes.add(replacement)));
                    aggregate.member_count = member_count;
                    existing_published = true;
                }
                Err(_) => {
                    commands.entity(aggregate_entity).despawn();
                    registry.aggregate_entities.remove(&key);
                }
            }
        }

        if !existing_published {
            let Some(local_translation) =
                aggregate_runtime_translation(world, layer, &layer_frames, key.scope)
            else {
                registry.dirty.insert(key);
                continue;
            };

            let mesh_handle = meshes.add(mesh.expect("aggregate mesh is still available"));
            let root = commands
                .spawn((
                    Name::new(format!(
                        "Voxel Render Aggregate {}³",
                        key.scope.extent().native_units_per_axis()
                    )),
                    *layer,
                    Transform::from_translation(local_translation),
                    Visibility::Inherited,
                ))
                .id();
            let presentation = commands
                .spawn((
                    Name::new(format!(
                        "Voxel Render Aggregate {} Presentation",
                        layer.scale()
                    )),
                    ChildOf(root),
                    VoxelRenderAggregatePresentation,
                    UsfLocalScalePresentation::new(layer.scale()),
                    Mesh3d(mesh_handle),
                    MeshMaterial3d(streaming.material().clone()),
                    Transform::IDENTITY,
                    Visibility::Inherited,
                ))
                .id();
            commands.entity(root).insert(VoxelRenderAggregate {
                presentation,
                member_count,
            });
            registry.aggregate_entities.insert(key, root);
        }

        perf.record_render_aggregate_rebuild(started.elapsed().as_micros() as u64);
    }
}

fn remove_member(registry: &mut VoxelRenderAggregateRegistry, key: AggregateKey, entity: Entity) {
    if let Some(group) = registry.groups.get_mut(&key) {
        group.remove(&entity);
    }
    registry.dirty.insert(key);
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
    members: &[(Entity, u64)],
    surfaces: &Query<(&VoxelMaterializationChunkAddress, &VoxelChunkRenderSurface)>,
) -> Option<Mesh> {
    let mut positions = Vec::<[f32; 3]>::new();
    let mut normals = Vec::<[f32; 3]>::new();
    let mut colors = Vec::<[f32; 4]>::new();
    let mut uvs = Vec::<[f32; 2]>::new();
    let mut tangents = Vec::<[f32; 4]>::new();
    let mut indices = Vec::<u32>::new();

    let bound =
        scope.extent().native_units_per_axis() as f32 + MATERIALIZATION_CHUNK_SIZE as f32 * 2.0;

    for &(entity, expected_revision) in members {
        let Ok((address, cache)) = surfaces.get(entity) else {
            return None;
        };
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
