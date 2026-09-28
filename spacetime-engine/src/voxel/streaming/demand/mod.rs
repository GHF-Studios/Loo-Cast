//! Spatial-demand interpretation and voxel residency reconciliation.

use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    config::EngineConfig,
    spatial::{
        SpatialDemandScope, SpatialScale, UsfCapabilityRealization, UsfChunkAddress,
        UsfContextResidency, UsfPositionError, UsfScaleLayer, UsfScaleRoleMask,
    },
};

use super::{VoxelPinnedDemand, VoxelStreaming};

use super::super::{
    MATERIALIZATION_CHUNK_SIZE, VoxelCollisionDisabled, VoxelEditingDisabled,
    VoxelMaterializationChunkAddress, VoxelQueryPosition, VoxelRealizationDemandSnapshot,
    VoxelRealizationScope, VoxelWorld, manifestation::VoxelMaterializationRuntime,
};

#[derive(Debug, Clone, Copy)]
pub(super) struct DemandedChunk {
    pub(super) address: VoxelMaterializationChunkAddress,
    pub(super) priority: i32,
    pub(super) distance_squared: f32,
    pub(super) roles: UsfScaleRoleMask,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct VoxelDemandPlanKey {
    source: Entity,
    center_address: VoxelMaterializationChunkAddress,
    minimum: IVec3,
    maximum: IVec3,
    priority: i32,
    roles: u16,
}

/// Reconciles active voxel materialization residency with the latest spatial
/// demand snapshot.
///
/// This stage owns demand interpretation and hot/warm residency transitions. It
/// does not spawn asynchronous generation work.
pub(in crate::voxel) fn refresh_voxel_residency(
    config: Res<EngineConfig>,
    residency: Res<UsfContextResidency>,
    realization_demand: Res<VoxelRealizationDemandSnapshot>,
    runtimes: Query<(&VoxelMaterializationRuntime, &UsfCapabilityRealization)>,
    mut worlds: Query<(
        Entity,
        &mut VoxelWorld,
        &mut VoxelStreaming,
        &UsfScaleLayer,
        Option<&VoxelPinnedDemand>,
        Option<&VoxelCollisionDisabled>,
        Option<&VoxelEditingDisabled>,
    )>,
    mut voxel_demands: Local<Vec<VoxelRealizationScope>>,
    mut runtime_roles: Local<
        HashMap<
            (Entity, VoxelMaterializationChunkAddress),
            UsfScaleRoleMask,
        >,
    >,
) {
    let warm_limit = config.voxel.streaming.warm_inactive_materialization_limit;

    runtime_roles.clear();
    for (runtime, realization) in &runtimes {
        if realization.revision() == runtime.revision() {
            runtime_roles.insert(
                (runtime.world(), runtime.address()),
                realization.roles(),
            );
        }
    }

    for (
        world_entity,
        mut world,
        mut streaming,
        layer,
        pinned,
        collision_disabled,
        editing_disabled,
    ) in &mut worlds
    {
        voxel_demands.clear();
        voxel_demands.extend(realization_demand.requests_for(world_entity));
        let pinned_shell = pinned
            .and_then(|pinned| pinned.surface_radius_native())
            .map(|radius| (world_entity, radius));

        let plan_changed = match refresh_demand_plan(
            &world,
            &voxel_demands,
            &mut streaming,
            pinned_shell,
            &residency,
            layer.scale(),
        ) {
            Ok(changed) => changed,
            Err(error) => {
                error!(
                    error = %error,
                    world = ?world_entity,
                    scale = %layer.scale(),
                    world_leaf = %world.origin().leaf_scale(),
                    "voxel spatial demand could not be represented canonically; retaining previous residency"
                );
                continue;
            }
        };

        let candidate_committed = if streaming.migration_active()
            && candidate_plan_ready(
                world_entity,
                &world,
                &streaming,
                collision_disabled.is_none(),
                editing_disabled.is_none(),
                &runtime_roles,
            )
        {
            streaming.commit_candidate()
        } else {
            false
        };

        if plan_changed || candidate_committed {
            if let Some(surface_radius_native) =
                pinned.and_then(|pinned| pinned.surface_radius_native())
            {
                prioritize_surface_shell(&mut streaming, surface_radius_native);
            }
            reconcile_materialization_residency(
                &mut world,
                &mut streaming,
                warm_limit,
            );
        }
    }
}



fn candidate_plan_ready(
    world_entity: Entity,
    world: &VoxelWorld,
    streaming: &VoxelStreaming,
    collision_enabled: bool,
    editing_enabled: bool,
    runtime_roles: &HashMap<
        (Entity, VoxelMaterializationChunkAddress),
        UsfScaleRoleMask,
    >,
) -> bool {
    streaming.candidate_addresses().all(|(address, requested_roles)| {
        let store = world.materializations();
        if !store.is_derived_current(address) {
            return false;
        }

        let Some(cache) = store.surface(address) else {
            return true;
        };
        if !cache.surface.has_triangles() {
            return true;
        }

        let mut required = UsfScaleRoleMask::REALIZATION;
        if requested_roles.contains(UsfScaleRoleMask::PRESENTATION) {
            required = required.union(UsfScaleRoleMask::PRESENTATION);
        }
        if collision_enabled
            && requested_roles.contains(UsfScaleRoleMask::COLLISION)
            && cache.surface.has_rigid_triangles()
        {
            required = required.union(UsfScaleRoleMask::COLLISION);
        }
        if editing_enabled && requested_roles.contains(UsfScaleRoleMask::EDITING) {
            required = required.union(UsfScaleRoleMask::EDITING);
        }

        runtime_roles
            .get(&(world_entity, address))
            .is_some_and(|roles| roles.contains(required))
    })
}

fn prioritize_surface_shell(streaming: &mut VoxelStreaming, radius_native: f32) {
    let mut pending = streaming.pending_desired.drain(..).collect::<Vec<_>>();
    pending.sort_by(|a, b| {
        b.priority.cmp(&a.priority).then_with(|| {
            let a_error = (a.distance_squared.sqrt() - radius_native).abs();
            let b_error = (b.distance_squared.sqrt() - radius_native).abs();
            a_error.total_cmp(&b_error)
        })
    });
    streaming.pending_desired = pending.into();
}

fn reconcile_materialization_residency(
    world: &mut VoxelWorld,
    streaming: &mut VoxelStreaming,
    warm_inactive_materialization_limit: usize,
) {
    let effective_desired = streaming.effective_desired_set();
    world.materializations_mut().reconcile_residency(
        &effective_desired,
        warm_inactive_materialization_limit,
    );

    streaming
        .pending_desired
        .retain(|demanded| !world.materializations().is_active(demanded.address));
}

#[derive(Debug)]
enum VoxelDemandPlanError {
    Position(UsfPositionError),
    MissingResidentContext(UsfChunkAddress),
}

impl From<UsfPositionError> for VoxelDemandPlanError {
    fn from(value: UsfPositionError) -> Self {
        Self::Position(value)
    }
}

impl std::fmt::Display for VoxelDemandPlanError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Position(error) => write!(formatter, "canonical position error: {error:?}"),
            Self::MissingResidentContext(context) => write!(formatter, "missing resident USF context: {context:?}"),
        }
    }
}

fn refresh_demand_plan(
    world: &VoxelWorld,
    demands: &[SpatialDemandScope],
    streaming: &mut VoxelStreaming,
    pinned_shell: Option<(Entity, f32)>,
    residency: &UsfContextResidency,
    context_scale: SpatialScale,
) -> Result<bool, VoxelDemandPlanError> {
    let key = demand_plan_key(world, demands)?;
    if key == streaming.demand_key && streaming.residency_revision == residency.revision() {
        return Ok(false);
    }

    let desired = demanded_chunk_addresses(world, demands, pinned_shell)?;
    validate_context_residency(&desired, residency, context_scale)?;
    let desired_roles = desired
        .iter()
        .map(|chunk| (chunk.address, chunk.roles))
        .collect::<HashMap<_, _>>();
    streaming.stage_desired_roles(desired_roles);
    streaming.pending_desired = desired
        .iter()
        .copied()
        .filter(|chunk| !world.materializations().is_active(chunk.address))
        .collect();
    streaming.demand_key = key;
    streaming.residency_revision = residency.revision();
    Ok(true)
}

fn validate_context_residency(
    desired: &[DemandedChunk],
    residency: &UsfContextResidency,
    context_scale: SpatialScale,
) -> Result<(), VoxelDemandPlanError> {
    for demanded in desired {
        let center = demanded.address.center()?;
        let context = UsfChunkAddress::containing(center, context_scale)?;
        if !residency.contains(context) {
            return Err(VoxelDemandPlanError::MissingResidentContext(context));
        }
    }
    Ok(())
}

fn demand_plan_key(
    world: &VoxelWorld,
    demands: &[VoxelRealizationScope],
) -> Result<Vec<VoxelDemandPlanKey>, crate::spatial::UsfPositionError> {
    let mut result = Vec::with_capacity(demands.len());
    let size = MATERIALIZATION_CHUNK_SIZE as f32;
    for request in demands {
        let demand = request.scope();
        let center = VoxelQueryPosition::new(demand.center());
        let center_address = world.materialization_address_containing(center)?;
        let local = center.relative_to(center_address.query_origin(), size + 0.01)?;
        let half = demand.half_extent_native();
        result.push(VoxelDemandPlanKey {
            source: demand.source(),
            center_address,
            minimum: checked_ivec3(((local - half) / size).floor())?,
            maximum: checked_ivec3(((local + half) / size).floor())?,
            priority: demand.priority(),
            roles: request.roles().bits(),
        });
    }
    Ok(result)
}

pub(super) fn demanded_chunk_addresses<T>(
    world: &VoxelWorld,
    demands: &[T],
    pinned_shell: Option<(Entity, f32)>,
) -> Result<Vec<DemandedChunk>, crate::spatial::UsfPositionError>
where
    T: Copy + Into<VoxelRealizationScope>,
{
    let mut merged = HashMap::<VoxelMaterializationChunkAddress, DemandedChunk>::new();

    for request in demands.iter().copied().map(Into::into) {
        let demand = request.scope();
        let center = VoxelQueryPosition::new(demand.center());
        let center_address = world.materialization_address_containing(center)?;
        let size = MATERIALIZATION_CHUNK_SIZE as f32;
        let local_center = center.relative_to(center_address.query_origin(), size + 0.01)?;
        let half = demand.half_extent_native();
        let minimum = checked_ivec3(((local_center - half) / size).floor())?;
        let maximum = checked_ivec3(((local_center + half) / size).floor())?;

        for z in minimum.z..=maximum.z {
            for y in minimum.y..=maximum.y {
                for x in minimum.x..=maximum.x {
                    let offset = IVec3::new(x, y, z);
                    let address = center_address.translated_chunks(offset)?;
                    let chunk_center = offset.as_vec3() * size + Vec3::splat(size * 0.5);
                    let from_demand_center = chunk_center - local_center;
                    let distance_squared = from_demand_center.length_squared();

                    // A pinned celestial body's far realization is a sparse
                    // surface shell, not a solid enclosing cube.
                    if let Some((pinned_source, radius_native)) = pinned_shell {
                        if demand.source() == pinned_source {
                            let chunk_half_diagonal = Vec3::splat(size * 0.5).length();
                            let surface_margin = chunk_half_diagonal + 1.5;
                            let distance = distance_squared.sqrt();
                            if (distance - radius_native).abs() > surface_margin {
                                continue;
                            }
                        }
                    }

                    let candidate = DemandedChunk {
                        address,
                        priority: demand.priority(),
                        distance_squared,
                        roles: request.roles(),
                    };

                    merged
                        .entry(address)
                        .and_modify(|current| {
                            current.roles = current.roles.union(candidate.roles);
                            if candidate.priority > current.priority
                                || (candidate.priority == current.priority
                                    && candidate.distance_squared < current.distance_squared)
                            {
                                current.priority = candidate.priority;
                                current.distance_squared = candidate.distance_squared;
                            }
                        })
                        .or_insert(candidate);
                }
            }
        }
    }

    let mut desired = merged.into_values().collect::<Vec<_>>();
    desired.sort_by(|a, b| {
        b.priority
            .cmp(&a.priority)
            .then_with(|| a.distance_squared.total_cmp(&b.distance_squared))
    });
    Ok(desired)
}

fn checked_ivec3(value: Vec3) -> Result<IVec3, crate::spatial::UsfPositionError> {
    fn component(value: f32) -> Result<i32, crate::spatial::UsfPositionError> {
        let value64 = f64::from(value);
        if !value.is_finite() || value64 < i32::MIN as f64 || value64 > i32::MAX as f64 {
            Err(crate::spatial::UsfPositionError::TranslationTooLarge)
        } else {
            Ok(value as i32)
        }
    }

    Ok(IVec3::new(
        component(value.x)?,
        component(value.y)?,
        component(value.z)?,
    ))
}
