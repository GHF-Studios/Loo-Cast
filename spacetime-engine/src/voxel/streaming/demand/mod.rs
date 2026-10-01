//! Spatial-demand interpretation and voxel residency reconciliation.

use std::collections::{HashMap, VecDeque};

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
    CelestialVoxelRealization, MATERIALIZATION_CHUNK_SIZE,
    VoxelCollisionDisabled, VoxelEditingDisabled, VoxelMaterializationKey,
    VoxelQueryPosition, VoxelRealizationDemandSnapshot,
    VoxelRegionSpan,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct VoxelMaterializationBox {
    minimum: [i64; 3],
    maximum: [i64; 3],
}

impl VoxelMaterializationBox {
    fn from_plan(plan: VoxelDemandPlanKey) -> Option<Self> {
        let center = plan.center_key.components();
        Some(Self {
            minimum: [
                center[0].checked_add(i64::from(plan.minimum.x))?,
                center[1].checked_add(i64::from(plan.minimum.y))?,
                center[2].checked_add(i64::from(plan.minimum.z))?,
            ],
            maximum: [
                center[0].checked_add(i64::from(plan.maximum.x))?,
                center[1].checked_add(i64::from(plan.maximum.y))?,
                center[2].checked_add(i64::from(plan.maximum.z))?,
            ],
        })
    }

    fn intersection(self, other: Self) -> Option<Self> {
        let minimum = [
            self.minimum[0].max(other.minimum[0]),
            self.minimum[1].max(other.minimum[1]),
            self.minimum[2].max(other.minimum[2]),
        ];
        let maximum = [
            self.maximum[0].min(other.maximum[0]),
            self.maximum[1].min(other.maximum[1]),
            self.maximum[2].min(other.maximum[2]),
        ];
        (minimum[0] <= maximum[0]
            && minimum[1] <= maximum[1]
            && minimum[2] <= maximum[2])
            .then_some(Self { minimum, maximum })
    }

    fn for_each(self, mut visit: impl FnMut(VoxelMaterializationKey)) {
        for z in self.minimum[2]..=self.maximum[2] {
            for y in self.minimum[1]..=self.maximum[1] {
                for x in self.minimum[0]..=self.maximum[0] {
                    visit(VoxelMaterializationKey::new([x, y, z]));
                }
            }
        }
    }

    fn for_each_difference(
        self,
        subtract: Self,
        mut visit: impl FnMut(VoxelMaterializationKey),
    ) {
        let Some(intersection) = self.intersection(subtract) else {
            self.for_each(visit);
            return;
        };

        let slab = |minimum: [i64; 3],
                    maximum: [i64; 3],
                    visit: &mut dyn FnMut(VoxelMaterializationKey)| {
            if minimum[0] > maximum[0]
                || minimum[1] > maximum[1]
                || minimum[2] > maximum[2]
            {
                return;
            }
            VoxelMaterializationBox { minimum, maximum }.for_each(visit);
        };

        slab(
            self.minimum,
            [
                intersection.minimum[0] - 1,
                self.maximum[1],
                self.maximum[2],
            ],
            &mut visit,
        );
        slab(
            [
                intersection.maximum[0] + 1,
                self.minimum[1],
                self.minimum[2],
            ],
            self.maximum,
            &mut visit,
        );

        let middle_x = [intersection.minimum[0], intersection.maximum[0]];
        slab(
            [middle_x[0], self.minimum[1], self.minimum[2]],
            [
                middle_x[1],
                intersection.minimum[1] - 1,
                self.maximum[2],
            ],
            &mut visit,
        );
        slab(
            [
                middle_x[0],
                intersection.maximum[1] + 1,
                self.minimum[2],
            ],
            [middle_x[1], self.maximum[1], self.maximum[2]],
            &mut visit,
        );

        let middle_y = [intersection.minimum[1], intersection.maximum[1]];
        slab(
            [middle_x[0], middle_y[0], self.minimum[2]],
            [
                middle_x[1],
                middle_y[1],
                intersection.minimum[2] - 1,
            ],
            &mut visit,
        );
        slab(
            [
                middle_x[0],
                middle_y[0],
                intersection.maximum[2] + 1,
            ],
            [middle_x[1], middle_y[1], self.maximum[2]],
            &mut visit,
        );
    }
}

fn incremental_plan_compatible(
    previous: VoxelDemandPlanKey,
    next: VoxelDemandPlanKey,
) -> bool {
    previous.source == next.source
        && previous.priority == next.priority
        && previous.roles == next.roles
        && previous.view_revision == 0
        && next.view_revision == 0
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
        Option<&CelestialVoxelRealization>,
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
        celestial_realization,
        pinned,
        collision_disabled,
        editing_disabled,
    ) in &mut worlds
    {
        voxel_demands.clear();
        voxel_demands.extend(realization_demand.requests_for(world_entity));
        let pinned_shell = pinned
            .and_then(|pinned| pinned.surface_radius_native())
            .map(|radius| {
                (
                    celestial_realization
                        .map_or(world_entity, |realization| realization.authority()),
                    radius,
                )
            });

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
            validate_context_residency(demands, residency, context_scale)?;
        }
        streaming.residency_revision = residency.revision();
        return Ok(false);
    }

    // Context responsibility is declared by demand scopes, not by 10-native
    // materialization cells. Validate the handful of intersected USF contexts
    // before enumerating capability-local materializations.
    {
        let _span = bevy::log::info_span!("voxel_residency.validate_context").entered();
        validate_context_residency(demands, residency, context_scale)?;
    }

    // Common hot path: one ordinary cuboid demand moved to a neighboring
    // materialization window. Update only entering/leaving slabs. View-frustum
    // and pinned-shell plans retain the conservative full planner below.
    let incremental = demands.len() == 1
        && pinned_shell.is_none()
        && demands[0].view_source().is_none()
        && streaming.demand_key.len() == 1
        && key.len() == 1
        && incremental_plan_compatible(streaming.demand_key[0], key[0]);

    if incremental
        && let (Some(previous_box), Some(next_box)) = (
            VoxelMaterializationBox::from_plan(streaming.demand_key[0]),
            VoxelMaterializationBox::from_plan(key[0]),
        )
    {
        let _span =
            bevy::log::info_span!("voxel_residency.enumerate_delta").entered();
        let request = demands[0];
        let demand = request.scope();
        let mut leaving = Vec::<VoxelMaterializationKey>::new();
        let mut entering = Vec::<DemandedChunk>::new();

        previous_box.for_each_difference(next_box, |materialization| {
            leaving.push(materialization);
        });

        let mut error = None;
        next_box.for_each_difference(previous_box, |materialization| {
            if error.is_some() {
                return;
            }
            match demanded_chunk_for_key(world, request, materialization) {
                Ok(demanded) => entering.push(demanded),
                Err(value) => error = Some(value),
            }
        });
        if let Some(error) = error {
            return Err(error.into());
        }
        drop(_span);

        {
            let _span =
                bevy::log::info_span!("voxel_residency.stage_delta").entered();
            streaming.stage_incremental_desired(leaving, entering);
        }

        streaming.demand_key = key;
        streaming.residency_revision = residency.revision();
        return Ok(true);
    }

    let desired = {
        let _span = bevy::log::info_span!("voxel_residency.enumerate_desired.full").entered();
        demanded_chunk_addresses(world, demands, pinned_shell, view_demands)?
    };
    {
        let _span = bevy::log::info_span!("voxel_residency.stage_plan.full").entered();
        let mut desired_roles = HashMap::with_capacity(desired.len());
        let mut pending_desired = VecDeque::with_capacity(desired.len());

        for chunk in desired {
            desired_roles.insert(chunk.key, chunk.roles);
            if !world.materializations().is_active(chunk.key) {
                pending_desired.push_back(chunk);
            }
        }

        streaming.stage_desired_roles(desired_roles);
        streaming.pending_desired = pending_desired;
    }
    streaming.demand_key = key;
    streaming.residency_revision = residency.revision();
    Ok(true)
}

fn validate_context_residency(
    demands: &[VoxelRealizationScope],
    residency: &UsfContextResidency,
    context_scale: SpatialScale,
) -> Result<(), VoxelDemandPlanError> {
    for request in demands {
        let demand = request.scope();
        let residency_scope = SpatialDemandScope::at_scale(
            demand.source(),
            context_scale,
            demand.center(),
            request.residency_half_extent_native(),
            demand.priority(),
        );

        if let Some(context) = residency.first_missing_intersecting(residency_scope)? {
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

fn demanded_chunk_for_key(
    world: &VoxelWorld,
    request: VoxelRealizationScope,
    key: VoxelMaterializationKey,
) -> Result<DemandedChunk, UsfPositionError> {
    let demand = request.scope();
    let center = world.materialization_address(key)?.center()?;
    let bound =
        demand.half_extent_native().length() + MATERIALIZATION_CHUNK_SIZE as f32 * 2.0 + 1.0;
    let relative = center.relative_at_scale_bounded(
        &demand.center(),
        demand.scale(),
        bound.max(MATERIALIZATION_CHUNK_SIZE as f32 * 2.0),
    )?;

    Ok(DemandedChunk {
        key,
        priority: demand.priority(),
        distance_squared: relative.length_squared(),
        roles: request.roles(),
    })
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

// region-first-demand-v1
fn collect_all_region_leaves(
    center_key: VoxelMaterializationKey,
    region: VoxelRegionSpan,
    demand: SpatialDemandScope,
    request: VoxelRealizationScope,
    local_center: Vec3,
    size: f32,
    merged: &mut HashMap<VoxelMaterializationKey, DemandedChunk>,
) -> Result<(), crate::spatial::UsfPositionError> {
    let relative_origin = region.relative_origin_chunks(center_key)?;
    let extent = region.extent_chunks();
    for z in 0..extent.z { for y in 0..extent.y { for x in 0..extent.x {
        let local_offset=IVec3::new(x,y,z);
        let key=region.origin().translated_chunks(local_offset)?;
        let chunk_offset=relative_origin+local_offset;
        let chunk_center=chunk_offset.as_vec3()*size+Vec3::splat(size*0.5);
        merge_demanded_chunk(merged,DemandedChunk{
            key, priority:demand.priority(),
            distance_squared:(chunk_center-local_center).length_squared(),
            roles:request.roles(),
        });
    }}}
    Ok(())
}

fn region_may_intersect_surface_shell(
    block_center_from_demand: Vec3,
    block_half_extent: Vec3,
    radius_native: f32,
    leaf_size: f32,
) -> bool {
    let c=block_center_from_demand.abs();
    let nearest=(c-block_half_extent).max(Vec3::ZERO).length();
    let farthest=(c+block_half_extent).length();
    let margin=Vec3::splat(leaf_size*0.5).length()+1.5;
    nearest<=radius_native+margin && farthest>=(radius_native-margin).max(0.0)
}

fn collect_culled_region(
    center_key: VoxelMaterializationKey,
    center_origin: &UsfPosition,
    region: VoxelRegionSpan,
    demand: SpatialDemandScope,
    request: VoxelRealizationScope,
    view: Option<&crate::spatial::UsfViewDemand>,
    pinned_shell: Option<(Entity,f32)>,
    local_center: Vec3,
    size: f32,
    merged: &mut HashMap<VoxelMaterializationKey,DemandedChunk>,
)->Result<(),crate::spatial::UsfPositionError>{
    let relative_origin=region.relative_origin_chunks(center_key)?;
    let block_min=relative_origin.as_vec3()*size;
    let block_max=block_min+region.extent_chunks().as_vec3()*size;
    let block_center_local=(block_min+block_max)*0.5;
    let block_half_extent=(block_max-block_min)*0.5;
    let block_center=center_origin.translated_native(block_center_local)?;

    if let Some(view)=view
        && !view.intersects_native_aabb(demand.scale(),&block_center,block_half_extent)
    { return Ok(()); }

    if let Some((source,radius))=pinned_shell
        && demand.source()==source
        && !region_may_intersect_surface_shell(
            block_center_local-local_center,block_half_extent,radius,size)
    { return Ok(()); }

    if region.is_leaf(){
        let key=region.origin();
        let chunk_center=relative_origin.as_vec3()*size+Vec3::splat(size*0.5);
        let distance_squared=(chunk_center-local_center).length_squared();
        if let Some((source,radius))=pinned_shell && demand.source()==source {
            let margin=Vec3::splat(size*0.5).length()+1.5;
            if (distance_squared.sqrt()-radius).abs()>margin { return Ok(()); }
        }
        merge_demanded_chunk(merged,DemandedChunk{
            key,priority:demand.priority(),distance_squared,roles:request.roles()
        });
        return Ok(());
    }

    let Some((left,right))=region.split_longest()? else { unreachable!() };
    collect_culled_region(center_key,center_origin,left,demand,request,view,pinned_shell,local_center,size,merged)?;
    collect_culled_region(center_key,center_origin,right,demand,request,view,pinned_shell,local_center,size,merged)
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

        let region = VoxelRegionSpan::from_relative_bounds(center_key, minimum, maximum)?;
        let view = match request.view_source() {
            Some(source) => {
                let Some(view) = view_demands.get(source) else { continue; };
                Some(view)
            }
            None => None,
        };
        let shell = pinned_shell.filter(|(source, _)| demand.source() == *source);

        if view.is_some() || shell.is_some() {
            collect_culled_region(
                center_key, center_address.origin(), region, demand, request,
                view, shell, local_center, size, &mut merged,
            )?;
        } else {
            // Exact demand still chooses the dense leaf backend today. The
            // region is now the request boundary, so another backend can later
            // satisfy it without changing materialization identity.
            collect_all_region_leaves(
                center_key, region, demand, request, local_center, size, &mut merged,
            )?;
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
