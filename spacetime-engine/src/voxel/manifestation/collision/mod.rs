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

use avian3d::prelude::{Collider, CollisionMargin, Position, RigidBody};
use bevy::prelude::*;

use crate::{
    config::EngineConfig,
    spatial::{SpatialScale, UsfScaleLayer, UsfScaleRoleMask, UsfSpatialFrame},
};

use super::super::{
    MATERIALIZATION_CHUNK_SIZE, VoxelCollisionDisabled, VoxelMaterializationKey,
    VoxelRealizationDemandSnapshot, VoxelStreaming, VoxelWorld, physics,
};

/// Historical aggregate edge that bounded incremental rebuild amplification
/// while reducing one-to-one collider-tree proxy count by up to 4³ = 64×.
const COLLISION_GROUP_EDGE: i64 = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(in crate::voxel) struct VoxelCollisionAggregateKey {
    world: Entity,
    origin: VoxelMaterializationKey,
}

#[derive(Debug)]
struct VoxelCollisionAggregateState {
    entity: Entity,
    members: Vec<(VoxelMaterializationKey, u64)>,
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct VoxelCollisionWorldRevision {
    materializations: u64,
    streaming: u64,
    collision_disabled: bool,
    scale: SpatialScale,
}

#[derive(Default)]
pub(in crate::voxel) struct VoxelCollisionReconcileCache {
    initialized: bool,
    interaction_padding_bits: u32,
    worlds: HashMap<Entity, VoxelCollisionWorldRevision>,
}

impl VoxelCollisionAggregateRegistry {
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
    frame: &UsfSpatialFrame,
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

/// Reconciles demanded rigid materializations into collision-only aggregates.
///
/// The desired/member scan is still exact and revision-aware, but Avian sees at
/// most one static proxy per aligned 4³ region rather than one proxy per
/// materialization.
pub(in crate::voxel) fn sync_manifestation_collision_residency(
    config: Res<EngineConfig>,
    frame: Res<UsfSpatialFrame>,
    realization_demand: Res<VoxelRealizationDemandSnapshot>,
    mut commands: Commands,
    worlds: Query<(
        Entity,
        &VoxelWorld,
        &UsfScaleLayer,
        &VoxelStreaming,
        Option<&VoxelCollisionDisabled>,
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
    for (world_entity, world, layer, streaming, collision_disabled) in &worlds {
        world_revisions.insert(
            world_entity,
            VoxelCollisionWorldRevision {
                materializations: world.materializations().capability_revision(),
                streaming: streaming.collision_policy_revision(),
                collision_disabled: collision_disabled.is_some(),
                scale: layer.scale(),
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

        // collision-before-presentation-v2
        //
        // Collision consumes store-owned derived surfaces directly. It must not
        // wait for a presentation runtime to exist: renderer manifestation is a
        // downstream disposable consumer of the same derived truth.
        for (world_entity, world, layer, streaming, collision_disabled) in &worlds {
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
            commands.entity(state.entity).despawn();
        }
    }

    {
        let _span = bevy::log::info_span!("voxel_collision.aggregate_build").entered();

        for (&aggregate, members) in desired.iter() {
            let unchanged = registry
                .groups
                .get(&aggregate)
                .is_some_and(|state| state.members == *members);
            if unchanged {
                continue;
            }

            let Ok((_, world, layer, _, collision_disabled)) =
                worlds.get(aggregate.world)
            else {
                continue;
            };
            if collision_disabled.is_some() {
                continue;
            }

            let Some(collider) =
                build_aggregate_collider(aggregate.origin, members, world)
            else {
                if let Some(state) = registry.groups.remove(&aggregate) {
                    commands.entity(state.entity).despawn();
                }
                continue;
            };

            if let Some(state) = registry.groups.get_mut(&aggregate) {
                commands.entity(state.entity).insert(collider);
                state.members.clone_from(members);
                continue;
            }

            let Some(translation) =
                aggregate_runtime_translation(&frame, layer, world, aggregate.origin)
            else {
                continue;
            };

            let entity = commands
                .spawn((
                    Name::new("Voxel Collision Aggregate"),
                    *layer,
                    RigidBody::Static,
                    Position::new(translation),
                    Transform::from_translation(translation),
                    collider,
                    CollisionMargin(physics::VOXEL_COLLISION_MARGIN),
                ))
                .id();

            registry.groups.insert(
                aggregate,
                VoxelCollisionAggregateState {
                    entity,
                    members: members.clone(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn four_cubed_materializations_share_one_collision_group() {
        let base = VoxelMaterializationKey::new([0, 0, 0]);
        for z in 0..4 {
            for y in 0..4 {
                for x in 0..4 {
                    let key = VoxelMaterializationKey::new([x, y, z]);
                    assert_eq!(aligned_group_origin(key), base);
                }
            }
        }
    }

    #[test]
    fn negative_keys_align_with_euclidean_groups() {
        assert_eq!(
            aligned_group_origin(VoxelMaterializationKey::new([-1, -4, -5])),
            VoxelMaterializationKey::new([-4, -4, -8]),
        );
    }

    #[test]
    fn member_offsets_are_group_local_native_units() {
        let origin = VoxelMaterializationKey::new([8, -4, 12]);
        let member = VoxelMaterializationKey::new([11, -2, 15]);
        assert_eq!(
            member_offset(origin, member),
            Some(Vec3::new(30.0, 20.0, 30.0)),
        );
    }
}
