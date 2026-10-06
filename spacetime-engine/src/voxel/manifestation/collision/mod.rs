//! Collision-specific aggregation over realized voxel materializations.
//!
//! Rendering remains one-to-one with materializations. Avian does not: nearby
//! materializations are merged into bounded 4³ collision aggregates so broad-
//! phase tree cardinality scales with collision regions rather than render/cache
//! granularity.
//!
//! This deliberately restores the useful collision part of the pre-7966f1c3
//! aggregate model without re-coupling presentation and collision lifecycles.

use std::collections::HashMap;

use avian3d::prelude::{
    Collider, ColliderDisabled, CollisionMargin, Position, RigidBody,
};
use bevy::prelude::*;

use crate::{
    config::EngineConfig,
    spatial::{
        SpatialScale, UsfScaleLayer, UsfScaleRoleMask, UsfRuntimeChartState,
        UsfSpatialTransitionApplied,
    },
};

use super::super::{
    CelestialVoxelFrameBinding, MATERIALIZATION_CHUNK_SIZE, VoxelCollisionDisabled, VoxelMaterializationKey,
    VoxelRealizationDemandSnapshot, VoxelStreaming, VoxelWorld, physics,
};

/// Historical aggregate edge that bounded incremental rebuild amplification
/// while reducing one-to-one collider-tree proxy count by up to 4³ = 64×.
const COLLISION_GROUP_EDGE: i64 = 4;
const MAX_POOLED_COLLISION_AGGREGATES: usize = 512;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(in crate::voxel) struct VoxelCollisionAggregateKey {
    world: Entity,
    origin: VoxelMaterializationKey,
}

#[derive(Debug)]
struct VoxelCollisionAggregateState {
    entity: Entity,
    members: Vec<(VoxelMaterializationKey, u64)>,
    frame_revision: u64,
}

/// Backend-only collision representation registry.
///
/// `published_members` is the capability bridge: it records which exact
/// materialization revisions are currently represented by a live aggregate.
/// The aggregate entity itself is disposable Avian state, never semantic
/// authority.
#[derive(Resource, Default)]
pub(in crate::voxel) struct VoxelCollisionAggregateRegistry {
    groups: HashMap<VoxelCollisionAggregateKey, VoxelCollisionAggregateState>,
    published_members: HashMap<(Entity, VoxelMaterializationKey), u64>,
    pooled_entities: Vec<Entity>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct VoxelCollisionWorldRevision {
    materializations: u64,
    streaming: u64,
    collision_disabled: bool,
    scale: SpatialScale,
    frame_revision: u64,
}

#[derive(Default)]
pub(in crate::voxel) struct VoxelCollisionReconcileCache {
    initialized: bool,
    interaction_padding_bits: u32,
    worlds: HashMap<Entity, VoxelCollisionWorldRevision>,
}

impl VoxelCollisionAggregateRegistry {
    fn recycle(&mut self, commands: &mut Commands, entity: Entity) {
        if self.pooled_entities.len() < MAX_POOLED_COLLISION_AGGREGATES {
            commands.entity(entity).insert(ColliderDisabled);
            self.pooled_entities.push(entity);
        } else {
            commands.entity(entity).despawn();
        }
    }

    fn take_pooled(&mut self) -> Option<Entity> {
        self.pooled_entities.pop()
    }

    pub(super) fn member_current(
        &self,
        world: Entity,
        key: VoxelMaterializationKey,
        revision: u64,
    ) -> bool {
        self.published_members
            .get(&(world, key))
            .is_some_and(|current| *current == revision)
    }
}

fn aligned_group_origin(key: VoxelMaterializationKey) -> VoxelMaterializationKey {
    let [x, y, z] = key.components();
    let edge = COLLISION_GROUP_EDGE;
    VoxelMaterializationKey::new([
        x.div_euclid(edge) * edge,
        y.div_euclid(edge) * edge,
        z.div_euclid(edge) * edge,
    ])
}

fn member_offset(
    group_origin: VoxelMaterializationKey,
    member: VoxelMaterializationKey,
) -> Option<Vec3> {
    let origin = group_origin.components();
    let member = member.components();
    let size = MATERIALIZATION_CHUNK_SIZE as f32;

    let dx = member[0].checked_sub(origin[0])?;
    let dy = member[1].checked_sub(origin[1])?;
    let dz = member[2].checked_sub(origin[2])?;

    Some(Vec3::new(
        dx as f32 * size,
        dy as f32 * size,
        dz as f32 * size,
    ))
}

fn sort_members(members: &mut [(VoxelMaterializationKey, u64)]) {
    members.sort_unstable_by_key(|(key, revision)| {
        let [x, y, z] = key.components();
        (x, y, z, *revision)
    });
}

/// Whether one current materialization belongs to the collision capability's
/// effective demand. Shared with capability publication so presentation and
/// collision use one readiness policy rather than parallel approximations.
pub(super) fn collision_requested(
    world_entity: Entity,
    key: VoxelMaterializationKey,
    world: &VoxelWorld,
    layer: &UsfScaleLayer,
    streaming: &VoxelStreaming,
    realization_demand: &VoxelRealizationDemandSnapshot,
    interaction_padding: f32,
) -> bool {
    let Ok(address) = world.materialization_address(key) else {
        return false;
    };

    realization_demand
        .requests_for(world_entity)
        .filter(|request| request.roles().contains(UsfScaleRoleMask::COLLISION))
        .map(|request| request.scope())
        .filter(|scope| scope.scale() == layer.scale())
        .any(|scope| {
            address
                .distance_squared_to_region(
                    &scope.center(),
                    scope.half_extent_native(),
                    interaction_padding,
                )
                .is_some_and(|distance_squared| {
                    distance_squared <= interaction_padding * interaction_padding
                })
        })
        || streaming.retains_committed_role_during_migration(
            key,
            UsfScaleRoleMask::COLLISION,
        )
}

fn build_aggregate_collider(
    group_origin: VoxelMaterializationKey,
    members: &[(VoxelMaterializationKey, u64)],
    world: &VoxelWorld,
) -> Option<Collider> {
    let mut vertices = Vec::<Vec3>::new();
    let mut triangles = Vec::<[u32; 3]>::new();

    for &(key, expected_revision) in members {
        let cache = world.materializations().surface(key)?;
        if cache.revision != expected_revision {
            return None;
        }

        let offset = member_offset(group_origin, key)?;
        let vertex_base = u32::try_from(vertices.len()).ok()?;

        vertices.extend(
            cache
                .surface
                .positions
                .iter()
                .copied()
                .map(Vec3::from_array)
                .map(|position| position + offset),
        );

        for triangle in physics::triangles(&cache.surface) {
            triangles.push([
                vertex_base.checked_add(triangle[0])?,
                vertex_base.checked_add(triangle[1])?,
                vertex_base.checked_add(triangle[2])?,
            ]);
        }
    }

    physics::build_trimesh_collider(
        vertices,
        triangles,
        "voxel collision aggregate",
    )
}

fn aggregate_runtime_translation(
    frame: &UsfRuntimeChartState,
    layer: &UsfScaleLayer,
    world: &VoxelWorld,
    origin: VoxelMaterializationKey,
) -> Option<Vec3> {
    let address = world.materialization_address(origin).ok()?;
    address
        .query_origin()
        .usf()
        .relative_at_scale_bounded(frame.origin(), layer.scale(), 16_384.0)
        .ok()
}

/// Reprojects all published voxel collision aggregates after a canonical chart
/// transition.
///
/// Ordinary floating-origin rebases already flow through `UsfOriginRebased` and
/// the Avian backend refresh path. `UsfRuntimeChartState::reanchor`, however, is a
/// discontinuous chart transaction: aggregate membership can remain identical
/// while every backend-local position changes. Readiness must never outlive the
/// pose it claims to represent.
pub(in crate::voxel) fn sync_collision_aggregate_runtime_transforms(
    mut transitions: MessageReader<UsfSpatialTransitionApplied>,
    frame: Res<UsfRuntimeChartState>,
    worlds: Query<(&VoxelWorld, &UsfScaleLayer)>,
    registry: Res<VoxelCollisionAggregateRegistry>,
    mut aggregates: Query<(&mut Transform, &mut Position)>,
) {
    let mut transitioned = false;
    for _ in transitions.read() {
        transitioned = true;
    }
    if !transitioned {
        return;
    }

    for (&key, state) in &registry.groups {
        let Ok((world, layer)) = worlds.get(key.world) else {
            continue;
        };
        let Some(translation) =
            aggregate_runtime_translation(&frame, layer, world, key.origin)
        else {
            continue;
        };
        let Ok((mut transform, mut position)) = aggregates.get_mut(state.entity) else {
            continue;
        };

        transform.translation = translation;
        position.0 = translation;
    }
}

/// Reconciles demanded rigid materializations into collision-only aggregates.
///
/// The desired/member scan is still exact and revision-aware, but Avian sees at
/// most one static proxy per aligned 4³ region rather than one proxy per
/// materialization.
pub(in crate::voxel) fn sync_manifestation_collision_residency(
    config: Res<EngineConfig>,
    frame: Res<UsfRuntimeChartState>,
    realization_demand: Res<VoxelRealizationDemandSnapshot>,
    mut commands: Commands,
    worlds: Query<(
        Entity,
        &VoxelWorld,
        &UsfScaleLayer,
        &VoxelStreaming,
        Option<&VoxelCollisionDisabled>,
        Option<&CelestialVoxelFrameBinding>,
    )>,
    mut registry: ResMut<VoxelCollisionAggregateRegistry>,
    mut desired: Local<
        HashMap<
            VoxelCollisionAggregateKey,
            Vec<(VoxelMaterializationKey, u64)>,
        >,
    >,
    mut cache: Local<VoxelCollisionReconcileCache>,
) {
    let _span = bevy::log::info_span!("voxel_collision.aggregate_reconcile").entered();

    let interaction_padding =
        config.voxel.manifestation.physics_interaction_radius_native.max(0.0);
    let interaction_padding_bits = interaction_padding.to_bits();

    let mut world_revisions =
        HashMap::<Entity, VoxelCollisionWorldRevision>::with_capacity(worlds.iter().len());
    for (world_entity, world, layer, streaming, collision_disabled, celestial_frame) in &worlds {
        world_revisions.insert(
            world_entity,
            VoxelCollisionWorldRevision {
                materializations: world.materializations().capability_revision(),
                streaming: streaming.collision_policy_revision(),
                collision_disabled: collision_disabled.is_some(),
                scale: layer.scale(),
                frame_revision: celestial_frame.map_or(0, |frame| frame.revision()),
            },
        );
    }

    if cache.initialized
        && cache.interaction_padding_bits == interaction_padding_bits
        && !realization_demand.is_changed()
        && cache.worlds == world_revisions
    {
        return;
    }

    cache.initialized = true;
    cache.interaction_padding_bits = interaction_padding_bits;
    cache.worlds = world_revisions;

    desired.clear();

    {
        let _span = bevy::log::info_span!("voxel_collision.aggregate_collect").entered();

        //
        // Collision consumes store-owned derived surfaces directly. It must not
        // wait for a presentation runtime to exist: renderer manifestation is a
        // downstream disposable consumer of the same derived truth.
        for (world_entity, world, layer, streaming, collision_disabled, _celestial_frame) in &worlds {
            if collision_disabled.is_some() {
                continue;
            }

            for key in world.materializations().active_keys() {
                let Some(revision) =
                    world.materializations().active_derived_revision(key)
                else {
                    continue;
                };
                let rigid_current = world
                    .materializations()
                    .surface(key)
                    .is_some_and(|cache| {
                        cache.revision == revision
                            && cache.surface.has_rigid_triangles()
                    });
                if !rigid_current
                    || !collision_requested(
                        world_entity,
                        key,
                        world,
                        layer,
                        streaming,
                        &realization_demand,
                        interaction_padding,
                    )
                {
                    continue;
                }

                let aggregate = VoxelCollisionAggregateKey {
                    world: world_entity,
                    origin: aligned_group_origin(key),
                };
                desired.entry(aggregate).or_default().push((key, revision));
            }
        }
    }

    for members in desired.values_mut() {
        sort_members(members);
    }

    let stale = registry
        .groups
        .keys()
        .copied()
        .filter(|key| !desired.contains_key(key))
        .collect::<Vec<_>>();
    for key in stale {
        if let Some(state) = registry.groups.remove(&key) {
            registry.recycle(&mut commands, state.entity);
        }
    }

    {
        let _span = bevy::log::info_span!("voxel_collision.aggregate_build").entered();

        for (&aggregate, members) in desired.iter() {
            let Ok((_, world, layer, _, collision_disabled, celestial_frame)) = worlds.get(aggregate.world) else { continue; };
            if collision_disabled.is_some(){continue;}
            let frame_revision=celestial_frame.map_or(0,|frame|frame.revision());
            if let Some(state)=registry.groups.get_mut(&aggregate) && state.members==*members {
                if state.frame_revision!=frame_revision {
                    if let Some(translation)=aggregate_runtime_translation(&frame,layer,world,aggregate.origin) {
                        commands.entity(state.entity).insert((Position::new(translation),Transform::from_translation(translation)));
                        state.frame_revision=frame_revision;
                    }
                }
                continue;
            }
            let Some(collider)=build_aggregate_collider(aggregate.origin,members,world) else {
                if let Some(state)=registry.groups.remove(&aggregate){registry.recycle(&mut commands,state.entity);} continue;
            };
            if let Some(state)=registry.groups.get_mut(&aggregate) {
                let Some(translation)=aggregate_runtime_translation(&frame,layer,world,aggregate.origin) else {continue;};
                commands.entity(state.entity).insert((collider,Position::new(translation),Transform::from_translation(translation)));
                state.members.clone_from(members); state.frame_revision=frame_revision; continue;
            }

            let Some(translation) =
                aggregate_runtime_translation(&frame, layer, world, aggregate.origin)
            else {
                continue;
            };

            let collision_margin = CollisionMargin(
                layer
                    .scale()
                    .metres_to_native_f32(
                        physics::VOXEL_COLLISION_MARGIN_METRES,
                    )
                    .max(f32::MIN_POSITIVE),
            );
            let entity = if let Some(entity) = registry.take_pooled() {
                commands
                    .entity(entity)
                    .insert((
                        *layer,
                        RigidBody::Static,
                        Position::new(translation),
                        Transform::from_translation(translation),
                        collider,
                        collision_margin,
                    ))
                    .remove::<ColliderDisabled>();
                entity
            } else {
                commands
                    .spawn((
                        Name::new("Voxel Collision Aggregate"),
                        *layer,
                        RigidBody::Static,
                        Position::new(translation),
                        Transform::from_translation(translation),
                        collider,
                        collision_margin,
                    ))
                    .id()
            };

            registry.groups.insert(
                aggregate,
                VoxelCollisionAggregateState {
                    entity,
                    members: members.clone(),
                    frame_revision,
                },
            );
        }
    }

    registry.published_members.clear();
    let published = registry
        .groups
        .iter()
        .flat_map(|(aggregate, state)| {
            state
                .members
                .iter()
                .copied()
                .map(move |(key, revision)| ((aggregate.world, key), revision))
        })
        .collect::<Vec<_>>();
    registry.published_members.extend(published);
}
