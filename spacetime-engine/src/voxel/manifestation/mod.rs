//! Runtime manifestations derived from store-owned voxel surface caches.
//!
//! The 10-native-unit materialization atom remains independently addressable in
//! the [`VoxelWorld`] store. This module groups cached atom surfaces into a much
//! smaller number of Bevy/Avian manifestations. Rendering and collision are
//! consumers of the same grouping membership, but remain separate lifecycle
//! stages. No semantic LOD is introduced.

use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

use avian3d::prelude::{Collider, CollisionMargin, RigidBody};
use bevy::{
    asset::RenderAssetUsages,
    ecs::lifecycle::RemovedComponents,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};

use crate::{
    config::{EngineConfig, VoxelGroupingStrategy, VoxelManifestationGroupingConfig},
    spatial::{UsfLocalScalePresentation, UsfScaleLayer, UsfScaleLayerFrames, UsfViewFrame},
};

use super::{
    MATERIALIZATION_CHUNK_SIZE, VoxelMaterializationChunkAddress, VoxelQueryPosition, VoxelWorld,
    aggregate::{VoxelMaterializationAggregateExtent, VoxelMaterializationAggregateScope},
    perf::VoxelPerfStats,
    physics,
    streaming::VoxelPresentationMaterial,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct AggregateKey {
    world: Entity,
    scope: VoxelMaterializationAggregateScope,
}

/// Maps virtual atom surfaces to physical mesh manifestations.
///
/// Semantic voxel identity never depends on this policy. Future adaptive/cost
/// partitioners can replace `AlignedRegions` behind this boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct VoxelManifestationGroupingPolicy {
    extent: VoxelMaterializationAggregateExtent,
}

impl VoxelManifestationGroupingPolicy {
    fn from_config(config: VoxelManifestationGroupingConfig) -> Option<Self> {
        match config.strategy {
            VoxelGroupingStrategy::AlignedRegions => {
                VoxelMaterializationAggregateExtent::from_base_chunks_per_axis(
                    config.base_chunks_per_axis,
                )
                .map(|extent| Self { extent })
            }
        }
    }
}

/// Root entity for one disposable same-resolution render/collision manifestation.
#[derive(Component)]
pub(crate) struct VoxelRenderAggregate {
    presentation: Entity,
    member_count: usize,
    scope: VoxelMaterializationAggregateScope,
}

impl VoxelRenderAggregate {
    pub(crate) const fn member_count(&self) -> usize {
        self.member_count
    }

    pub(crate) const fn scope(&self) -> VoxelMaterializationAggregateScope {
        self.scope
    }
}

/// Marks the only `Mesh3d` entity created for one render aggregate.
#[derive(Component, Debug, Default, Clone, Copy)]
pub(crate) struct VoxelRenderAggregatePresentation;

/// Incremental aggregate membership registry keyed by canonical atom addresses.
///
/// Quiet resident atoms never participate in this system. Store-side dirty
/// queues update only aggregates whose membership or cached surface changed.
#[derive(Resource, Default)]
pub(crate) struct VoxelRenderAggregateRegistry {
    grouping_policy: Option<VoxelManifestationGroupingPolicy>,
    groups: HashMap<AggregateKey, HashMap<VoxelMaterializationChunkAddress, u64>>,
    address_keys: HashMap<(Entity, VoxelMaterializationChunkAddress), AggregateKey>,
    dirty: HashSet<AggregateKey>,
    aggregate_entities: HashMap<AggregateKey, Entity>,
}


/// Retires all disposable manifestations owned by voxel worlds that disappeared.
///
/// This stage owns world-lifetime cleanup only. It does not derive membership,
/// build meshes, or make collision-residency decisions.
pub(crate) fn retire_removed_world_manifestations(
    mut commands: Commands,
    mut removed_worlds: RemovedComponents<VoxelWorld>,
    mut registry: ResMut<VoxelRenderAggregateRegistry>,
) {
    let removed = removed_worlds.read().collect::<HashSet<_>>();
    if removed.is_empty() {
        return;
    }

    let dead_keys = registry
        .aggregate_entities
        .keys()
        .copied()
        .filter(|key| removed.contains(&key.world))
        .collect::<Vec<_>>();

    for key in dead_keys {
        if let Some(entity) = registry.aggregate_entities.remove(&key) {
            commands.entity(entity).despawn();
        }
        registry.groups.remove(&key);
        registry.dirty.remove(&key);
    }

    registry
        .address_keys
        .retain(|(world, _), _| !removed.contains(world));
}

/// Applies manifestation-grouping policy changes.
///
/// Regrouping invalidates only disposable manifestation state. Store-owned dense
/// materializations and semantic voxel state remain untouched.
pub(crate) fn sync_manifestation_grouping_policy(
    config: Res<EngineConfig>,
    mut commands: Commands,
    mut worlds: Query<&mut VoxelWorld>,
    mut registry: ResMut<VoxelRenderAggregateRegistry>,
) {
    let grouping_policy =
        VoxelManifestationGroupingPolicy::from_config(config.voxel.manifestation.grouping)
            .expect("validated engine config must produce a manifestation grouping policy");

    if registry.grouping_policy == Some(grouping_policy) {
        return;
    }

    for (_, entity) in registry.aggregate_entities.drain() {
        commands.entity(entity).despawn();
    }
    registry.groups.clear();
    registry.address_keys.clear();
    registry.dirty.clear();
    registry.grouping_policy = Some(grouping_policy);

    for mut world in &mut worlds {
        world.materializations_mut().mark_all_active_render_dirty();
    }

    info!(
        base_chunks_per_axis = grouping_policy.extent.base_chunks_per_axis(),
        native_units_per_axis = grouping_policy.extent.native_units_per_axis(),
        "reconfigured voxel physical manifestation grouping"
    );
}

/// Consumes store-side surface changes into manifestation membership.
///
/// This stage is proportional to changed surface caches rather than resident
/// world size.
pub(crate) fn sync_manifestation_membership(
    mut worlds: Query<(Entity, &mut VoxelWorld)>,
    mut registry: ResMut<VoxelRenderAggregateRegistry>,
) {
    let Some(grouping_policy) = registry.grouping_policy else {
        return;
    };

    for (world_entity, mut world) in &mut worlds {
        while let Some(address) = world.materializations_mut().pop_dirty_render() {
            let current = world
                .materializations()
                .active_surface(address)
                .map(|surface| surface.revision);
            let previous = registry.address_keys.get(&(world_entity, address)).copied();

            match current {
                Some(revision) => {
                    let Ok(scope) = VoxelMaterializationAggregateScope::containing(
                        &world,
                        address,
                        grouping_policy.extent,
                    ) else {
                        warn!(
                            ?address,
                            "voxel manifestation scope could not be derived canonically"
                        );
                        continue;
                    };
                    let key = AggregateKey {
                        world: world_entity,
                        scope,
                    };

                    if let Some(previous) = previous
                        && previous != key
                    {
                        remove_member(&mut registry, previous, address);
                    }

                    registry.address_keys.insert((world_entity, address), key);
                    registry
                        .groups
                        .entry(key)
                        .or_default()
                        .insert(address, revision);
                    registry.dirty.insert(key);
                }
                None => {
                    if let Some(previous) = registry.address_keys.remove(&(world_entity, address)) {
                        remove_member(&mut registry, previous, address);
                    }
                }
            }
        }
    }
}

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

fn publish_collider_manifestation(
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

fn remove_member(
    registry: &mut VoxelRenderAggregateRegistry,
    key: AggregateKey,
    address: VoxelMaterializationChunkAddress,
) {
    if let Some(group) = registry.groups.get_mut(&key) {
        group.remove(&address);
    }
    registry.dirty.insert(key);
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
