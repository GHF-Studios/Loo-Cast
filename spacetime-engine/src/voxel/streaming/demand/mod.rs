//! Spatial-demand interpretation and voxel residency reconciliation.

use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    config::EngineConfig,
    spatial::{
        SpatialDemandScope, SpatialScale, UsfCapabilityRealization, UsfChunkAddress,
        UsfContextResidency, UsfPosition, UsfPositionError, UsfScaleLayer,
        UsfScaleRoleMask, UsfViewDemandSnapshot,
    },
};

use super::{VoxelPinnedDemand, VoxelStreaming};

use super::super::{
    MATERIALIZATION_CHUNK_SIZE, VoxelCollisionDisabled, VoxelEditingDisabled,
    VoxelMaterializationKey, VoxelQueryPosition, VoxelRealizationDemandSnapshot,
    VoxelRealizationScope, VoxelWorld, manifestation::VoxelMaterializationRuntime,
};

#[derive(Debug, Clone, Copy)]
pub(super) struct DemandedChunk {
    pub(super) key: VoxelMaterializationKey,
    pub(super) priority: i32,
    pub(super) distance_squared: f32,
    pub(super) roles: UsfScaleRoleMask,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct VoxelDemandPlanKey {
    source: Entity,
    center_key: VoxelMaterializationKey,
    minimum: IVec3,
    maximum: IVec3,
    priority: i32,
    roles: u16,
    view_revision: u64,
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
    view_demands: Res<UsfViewDemandSnapshot>,
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
    mut runtime_roles: Local<HashMap<(Entity, VoxelMaterializationKey), UsfScaleRoleMask>>,
) {
    let warm_limit = config.voxel.streaming.warm_inactive_materialization_limit;

    runtime_roles.clear();
    let mut runtime_roles_ready = false;

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
            &view_demands,
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

        let candidate_committed = if streaming.migration_active() {
            if !runtime_roles_ready {
                let _span = bevy::log::info_span!("voxel_residency.runtime_roles").entered();
                for (runtime, realization) in &runtimes {
                    if realization.revision() == runtime.revision() {
                        runtime_roles.insert((runtime.world(), runtime.key()), realization.roles());
                    }
                }
                runtime_roles_ready = true;
            }

            if candidate_plan_ready(
                world_entity,
                &world,
                &streaming,
                collision_disabled.is_none(),
                editing_disabled.is_none(),
                &runtime_roles,
            ) {
                streaming.commit_candidate()
            } else {
                false
            }
        } else {
            false
        };

        if plan_changed || candidate_committed {
            if let Some(surface_radius_native) = pinned.and_then(|pinned| pinned.surface_radius_native()) {
                prioritize_surface_shell(&mut streaming, surface_radius_native);
            }
            reconcile_materialization_residency(&mut world, &mut streaming, warm_limit);
        }
    }
}




fn candidate_plan_ready(
    world_entity: Entity,
    world: &VoxelWorld,
    streaming: &VoxelStreaming,
    collision_enabled: bool,
    editing_enabled: bool,
    runtime_roles: &HashMap<(Entity, VoxelMaterializationKey), UsfScaleRoleMask>,
) -> bool {
    let _span = bevy::log::info_span!("voxel_residency.candidate_ready").entered();
    streaming.candidate_addresses().all(|(key, requested_roles)| {
        let store = world.materializations();
        if !store.is_derived_current(key) {
            return false;
        }

        let Some(cache) = store.surface(key) else {
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
            .get(&(world_entity, key))
            .is_some_and(|roles| roles.contains(required))
    })
}

fn prioritize_surface_shell(streaming: &mut VoxelStreaming, radius_native: f32) {
    let _span = bevy::log::info_span!("voxel_residency.surface_sort").entered();
    let mut pending = streaming
        .pending_desired
        .drain(..)
        .map(|demanded| {
            let shell_error = (demanded.distance_squared.sqrt() - radius_native).abs();
            (demanded, shell_error)
        })
        .collect::<Vec<_>>();
    pending.sort_by(|(a, a_error), (b, b_error)| {
        b.priority.cmp(&a.priority).then_with(|| a_error.total_cmp(b_error))
    });
    streaming.pending_desired = pending.into_iter().map(|(demanded, _)| demanded).collect();
}

fn reconcile_materialization_residency(
    world: &mut VoxelWorld,
    streaming: &mut VoxelStreaming,
    warm_inactive_materialization_limit: usize,
) {
    let (activate, deactivate) = {
        let _span = bevy::log::info_span!("voxel_residency.delta_collect").entered();
        streaming.take_residency_delta()
    };
    {
        let _span = bevy::log::info_span!("voxel_residency.store_delta").entered();
        world.materializations_mut().apply_residency_delta(
            activate,
            deactivate,
            warm_inactive_materialization_limit,
        );
    }
    {
        let _span = bevy::log::info_span!("voxel_residency.pending_retain").entered();
        streaming.pending_desired.retain(|demanded| !world.materializations().is_active(demanded.key));
    }
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
    demands: &[VoxelRealizationScope],
    streaming: &mut VoxelStreaming,
    pinned_shell: Option<(Entity, f32)>,
    residency: &UsfContextResidency,
    view_demands: &UsfViewDemandSnapshot,
    context_scale: SpatialScale,
) -> Result<bool, VoxelDemandPlanError> {
    let key = {
        let _span = bevy::log::info_span!("voxel_residency.plan_key").entered();
        demand_plan_key(world, demands, view_demands)?
    };

    if key == streaming.demand_key {
        if streaming.residency_revision == residency.revision() {
            return Ok(false);
        }
        {
            let _span = bevy::log::info_span!("voxel_residency.revalidate_context").entered();
            validate_context_residency_keys(
                world,
                streaming.cached_desired_roles.keys().copied(),
                residency,
                context_scale,
            )?;
        }
        streaming.residency_revision = residency.revision();
        return Ok(false);
    }

    let desired = {
        let _span = bevy::log::info_span!("voxel_residency.enumerate_desired").entered();
        demanded_chunk_addresses(world, demands, pinned_shell, view_demands)?
    };
    {
        let _span = bevy::log::info_span!("voxel_residency.validate_context").entered();
        validate_context_residency(world, &desired, residency, context_scale)?;
    }
    {
        let _span = bevy::log::info_span!("voxel_residency.stage_plan").entered();
        let desired_roles = desired.iter().map(|chunk| (chunk.key, chunk.roles)).collect::<HashMap<_, _>>();
        streaming.stage_desired_roles(desired_roles);
        streaming.pending_desired = desired
            .iter()
            .copied()
            .filter(|chunk| !world.materializations().is_active(chunk.key))
            .collect();
    }
    streaming.demand_key = key;
    streaming.residency_revision = residency.revision();
    Ok(true)
}

fn validate_context_residency(
    world: &VoxelWorld,
    desired: &[DemandedChunk],
    residency: &UsfContextResidency,
    context_scale: SpatialScale,
) -> Result<(), VoxelDemandPlanError> {
    validate_context_residency_keys(
        world,
        desired.iter().map(|demanded| demanded.key),
        residency,
        context_scale,
    )
}

fn validate_context_residency_keys(
    world: &VoxelWorld,
    keys: impl IntoIterator<Item = VoxelMaterializationKey>,
    residency: &UsfContextResidency,
    context_scale: SpatialScale,
) -> Result<(), VoxelDemandPlanError> {
    for key in keys {
        let center = world.materialization_address(key)?.center()?;
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
    view_demands: &UsfViewDemandSnapshot,
) -> Result<Vec<VoxelDemandPlanKey>, crate::spatial::UsfPositionError> {
    let mut result = Vec::with_capacity(demands.len());
    let size = MATERIALIZATION_CHUNK_SIZE as f32;
    for request in demands {
        let demand = request.scope();
        let center = VoxelQueryPosition::new(demand.center());
        let center_key = world.materialization_key_containing(center)?;
        let center_address = world.materialization_address(center_key)?;
        let local = center.relative_to(center_address.query_origin(), size + 0.01)?;
        let half = demand.half_extent_native();
        result.push(VoxelDemandPlanKey {
            source: demand.source(),
            center_key,
            minimum: checked_ivec3(((local - half) / size).floor())?,
            maximum: checked_ivec3(((local + half) / size).floor())?,
            priority: demand.priority(),
            roles: request.roles().bits(),
            view_revision: request.view_source().map_or(0, |_| view_demands.revision()),
        });
    }
    Ok(result)
}

fn merge_demanded_chunk(
    merged: &mut HashMap<VoxelMaterializationKey, DemandedChunk>,
    candidate: DemandedChunk,
) {
    merged
        .entry(candidate.key)
        .and_modify(|current| {
            current.roles = current.roles.union(candidate.roles);
            if candidate.priority > current.priority
                || (candidate.priority == current.priority && candidate.distance_squared < current.distance_squared)
            {
                current.priority = candidate.priority;
                current.distance_squared = candidate.distance_squared;
            }
        })
        .or_insert(candidate);
}

fn collect_visible_chunk_block(
    center_key: VoxelMaterializationKey,
    center_origin: &UsfPosition,
    minimum: IVec3,
    maximum: IVec3,
    demand: SpatialDemandScope,
    request: VoxelRealizationScope,
    view: &crate::spatial::UsfViewDemand,
    local_center: Vec3,
    size: f32,
    merged: &mut HashMap<VoxelMaterializationKey, DemandedChunk>,
) -> Result<(), crate::spatial::UsfPositionError> {
    if minimum.cmpgt(maximum).any() {
        return Ok(());
    }

    let block_min = minimum.as_vec3() * size;
    let block_max = (maximum + IVec3::ONE).as_vec3() * size;
    let block_center_local = (block_min + block_max) * 0.5;
    let block_half_extent = (block_max - block_min) * 0.5;
    let block_center = center_origin.translated_native(block_center_local)?;

    if !view.intersects_native_aabb(demand.scale(), &block_center, block_half_extent) {
        return Ok(());
    }

    if minimum == maximum {
        let key = center_key.translated_chunks(minimum)?;
        let chunk_center = minimum.as_vec3() * size + Vec3::splat(size * 0.5);
        let distance_squared = (chunk_center - local_center).length_squared();
        merge_demanded_chunk(
            merged,
            DemandedChunk { key, priority: demand.priority(), distance_squared, roles: request.roles() },
        );
        return Ok(());
    }

    let span = maximum - minimum;
    let axis = if span.x >= span.y && span.x >= span.z { 0 } else if span.y >= span.z { 1 } else { 2 };
    let mut left_max = maximum;
    let mut right_min = minimum;
    match axis {
        0 => { let middle = minimum.x + span.x / 2; left_max.x = middle; right_min.x = middle + 1; }
        1 => { let middle = minimum.y + span.y / 2; left_max.y = middle; right_min.y = middle + 1; }
        2 => { let middle = minimum.z + span.z / 2; left_max.z = middle; right_min.z = middle + 1; }
        _ => unreachable!(),
    }

    collect_visible_chunk_block(center_key, center_origin, minimum, left_max, demand, request, view, local_center, size, merged)?;
    collect_visible_chunk_block(center_key, center_origin, right_min, maximum, demand, request, view, local_center, size, merged)
}

pub(super) fn demanded_chunk_addresses<T>(
    world: &VoxelWorld,
    demands: &[T],
    pinned_shell: Option<(Entity, f32)>,
    view_demands: &UsfViewDemandSnapshot,
) -> Result<Vec<DemandedChunk>, crate::spatial::UsfPositionError>
where
    T: Copy + Into<VoxelRealizationScope>,
{
    let mut merged = HashMap::<VoxelMaterializationKey, DemandedChunk>::new();

    for request in demands.iter().copied().map(Into::into) {
        let demand = request.scope();
        let center = VoxelQueryPosition::new(demand.center());
        let center_key = world.materialization_key_containing(center)?;
        let center_address = world.materialization_address(center_key)?;
        let size = MATERIALIZATION_CHUNK_SIZE as f32;
        let local_center = center.relative_to(center_address.query_origin(), size + 0.01)?;
        let half = demand.half_extent_native();
        let minimum = checked_ivec3(((local_center - half) / size).floor())?;
        let maximum = checked_ivec3(((local_center + half) / size).floor())?;

        if let Some(view_source) = request.view_source() {
            let Some(view) = view_demands.get(view_source) else { continue; };
            collect_visible_chunk_block(
                center_key,
                center_address.origin(),
                minimum,
                maximum,
                demand,
                request,
                view,
                local_center,
                size,
                &mut merged,
            )?;
            continue;
        }

        for z in minimum.z..=maximum.z {
            for y in minimum.y..=maximum.y {
                for x in minimum.x..=maximum.x {
                    let offset = IVec3::new(x, y, z);
                    let key = center_key.translated_chunks(offset)?;
                    let chunk_center = offset.as_vec3() * size + Vec3::splat(size * 0.5);
                    let distance_squared = (chunk_center - local_center).length_squared();

                    if let Some((pinned_source, radius_native)) = pinned_shell {
                        if demand.source() == pinned_source {
                            let surface_margin = Vec3::splat(size * 0.5).length() + 1.5;
                            let distance = distance_squared.sqrt();
                            if (distance - radius_native).abs() > surface_margin {
                                continue;
                            }
                        }
                    }

                    merge_demanded_chunk(
                        &mut merged,
                        DemandedChunk { key, priority: demand.priority(), distance_squared, roles: request.roles() },
                    );
                }
            }
        }
    }

    let mut desired = merged.into_values().collect::<Vec<_>>();
    desired.sort_by(|a, b| {
        b.priority.cmp(&a.priority).then_with(|| a.distance_squared.total_cmp(&b.distance_squared))
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
