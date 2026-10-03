//! Spatial-demand interpretation and voxel residency reconciliation.

// aggressive-demand-and-clipmap-local-balance-v1

use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

use bevy::math::DVec3;

use bevy::prelude::*;

use crate::{
    config::EngineConfig,
    reconstructible::{ReconstructibleFrameBudget, ReconstructibleWorkClass},
    spatial::{
        SpatialDemandMotionSnapshot, SpatialDemandScope,
        SpatialRealizationGranularityRequest, SpatialScale,
        UsfCapabilityRealization, UsfChunkAddress, UsfContextResidency, UsfPosition,
        UsfPositionError, UsfScaleLayer, UsfScaleRoleMask, UsfViewDemandSnapshot,
    },
};

use super::{VoxelPinnedDemand, VoxelStreaming};

use super::super::{
    CelestialVoxelRealization, MATERIALIZATION_CHUNK_SIZE,
    VoxelCollisionDisabled, VoxelEditingDisabled, VoxelMaterializationKey,
    VoxelQueryPosition, VoxelRealizationDemandSnapshot,
    VoxelRegionSpan,
    VoxelRealizationScope, VoxelWorld, manifestation::VoxelMaterializationRuntime,
    worker::{VoxelWorkerLane, VoxelWorkerPool},
};

#[derive(Debug, Clone, Copy)]
pub(super) struct DemandedChunk {
    pub(super) key: VoxelMaterializationKey,
    pub(super) priority: i32,
    pub(super) distance_squared: f32,
    pub(super) trajectory_distance_squared: f32,
    pub(super) role_priority: u8,
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
    motion: VoxelMotionPriorityKey,
    validity_chunks: u32,
}

const MIN_PREDICTIVE_VALIDITY_SECONDS: f64 = 1.0;
const MAX_PREDICTIVE_VALIDITY_SECONDS: f64 = 4.0;
const PREDICTIVE_LATENCY_MULTIPLIER: f64 = 4.0;
const MOTION_DIRECTION_QUANTIZATION: f64 = 8.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct VoxelMotionPriorityKey {
    direction: [i8; 3],
    speed_bucket: u8,
    horizon_bucket: u8,
}

impl VoxelMotionPriorityKey {
    const STATIONARY: Self = Self {
        direction: [0, 0, 0],
        speed_bucket: 0,
        horizon_bucket: 0,
    };
}

#[derive(Debug, Clone, Copy)]
struct VoxelDemandMotion {
    predicted_offset_native: Vec3,
    lookahead_seconds: f32,
    bias: f32,
    direction_native: Vec3,
}

impl VoxelDemandMotion {
    fn new(demand: SpatialDemandScope, velocity_metres_per_second: DVec3) -> Self {
        Self::with_expected_latency(demand, velocity_metres_per_second, 0.0)
    }

    fn with_expected_latency(
        demand: SpatialDemandScope,
        velocity_metres_per_second: DVec3,
        expected_build_seconds: f64,
    ) -> Self {
        if !velocity_metres_per_second.is_finite() {
            return Self::stationary();
        }
        let factor = demand.scale().scale0_to_native_f64(1.0);
        let native = velocity_metres_per_second * factor;
        let velocity_native = Vec3::new(
            saturating_motion_f32(native.x),
            saturating_motion_f32(native.y),
            saturating_motion_f32(native.z),
        );
        let speed_native = velocity_native.length();
        if !speed_native.is_finite() || speed_native <= f32::EPSILON {
            return Self::stationary();
        }

        let speed_chunks = speed_native / MATERIALIZATION_CHUNK_SIZE as f32;
        let bias = (speed_chunks / (speed_chunks + 1.0)).clamp(0.0, 1.0);
        let chunk_extent_metres =
            f64::from(MATERIALIZATION_CHUNK_SIZE) * demand.scale().metres_per_native();
        let granularity = SpatialRealizationGranularityRequest::new(
            chunk_extent_metres,
            chunk_extent_metres,
            chunk_extent_metres,
            1,
            1,
            velocity_metres_per_second.length(),
            expected_build_seconds,
            MIN_PREDICTIVE_VALIDITY_SECONDS,
            MAX_PREDICTIVE_VALIDITY_SECONDS,
            PREDICTIVE_LATENCY_MULTIPLIER,
        ).solve();
        let lookahead_seconds = granularity.validity_seconds() as f32;
        let predicted_offset_native = velocity_native * lookahead_seconds;

        Self {
            predicted_offset_native,
            lookahead_seconds,
            bias,
            direction_native: velocity_native.normalize_or_zero(),
        }
    }

    const fn stationary() -> Self {
        Self {
            predicted_offset_native: Vec3::ZERO,
            lookahead_seconds: 0.0,
            bias: 0.0,
            direction_native: Vec3::ZERO,
        }
    }

    fn trajectory_distance_squared(self, relative: Vec3) -> f32 {
        let current = relative.length_squared();
        if self.bias <= f32::EPSILON {
            return current;
        }
        let segment = self.predicted_offset_native;
        let segment_length_squared = segment.length_squared();
        if segment_length_squared <= f32::EPSILON {
            return current;
        }
        let t = (relative.dot(segment) / segment_length_squared).clamp(0.0, 1.0);
        let nearest = segment * t;
        let lateral_squared = (relative - nearest).length_squared();
        let corridor_score = lateral_squared * 8.0 + current * 0.05;
        (current + (corridor_score - current) * self.bias).max(0.0)
    }

    fn demand_offsets(self, half_extent: Vec3) -> (Vec3, Vec3) {
        let minimum = -half_extent;
        let maximum = half_extent;
        if self.bias <= f32::EPSILON || self.direction_native == Vec3::ZERO {
            return (minimum, maximum);
        }
        (
            minimum.min(self.predicted_offset_native - half_extent),
            maximum.max(self.predicted_offset_native + half_extent),
        )
    }
}


fn quantized_motion_key(
    velocity_metres_per_second: DVec3,
    lookahead_seconds: f32,
) -> VoxelMotionPriorityKey {
    let speed = velocity_metres_per_second.length();
    if !speed.is_finite() || speed < 0.5 {
        return VoxelMotionPriorityKey::STATIONARY;
    }
    let direction = velocity_metres_per_second / speed;
    let quantize = |value: f64| {
        (value * MOTION_DIRECTION_QUANTIZATION)
            .round()
            .clamp(-MOTION_DIRECTION_QUANTIZATION, MOTION_DIRECTION_QUANTIZATION)
            as i8
    };
    let speed_bucket = (speed.log2().floor() + 16.0).clamp(1.0, 63.0) as u8;
    let horizon_bucket = if lookahead_seconds > 0.0 {
        (f64::from(lookahead_seconds).log2().floor() + 16.0).clamp(1.0, 63.0) as u8
    } else {
        0
    };
    VoxelMotionPriorityKey {
        direction: [quantize(direction.x), quantize(direction.y), quantize(direction.z)],
        speed_bucket,
        horizon_bucket,
    }
}


fn saturating_motion_f32(value: f64) -> f32 {
    if value.is_nan() {
        0.0
    } else {
        value.clamp(-f64::from(f32::MAX), f64::from(f32::MAX)) as f32
    }
}

fn demand_role_priority(roles: UsfScaleRoleMask) -> u8 {
    if roles.contains(UsfScaleRoleMask::COLLISION) {
        4
    } else if roles.contains(UsfScaleRoleMask::EDITING) {
        3
    } else if roles.contains(UsfScaleRoleMask::PRESENTATION) {
        2
    } else if roles.contains(UsfScaleRoleMask::REALIZATION) {
        1
    } else {
        0
    }
}

fn make_demanded_chunk(
    demand: SpatialDemandScope,
    request: VoxelRealizationScope,
    key: VoxelMaterializationKey,
    relative: Vec3,
    motion: VoxelDemandMotion,
) -> DemandedChunk {
    DemandedChunk {
        key,
        priority: demand.priority(),
        distance_squared: relative.length_squared(),
        trajectory_distance_squared:
            motion.trajectory_distance_squared(relative),
        role_priority: demand_role_priority(request.roles()),
        roles: request.roles(),
    }
}



fn desired_chunk_budget(
    load_budget_per_frame: usize,
    demands: &[VoxelRealizationScope],
    motions: &SpatialDemandMotionSnapshot,
    expected_build_seconds: f64,
) -> usize {
    const UNBOUNDED_THROUGHPUT_HINT: usize = 256;
    const MINIMUM_STRESS_WORKING_SET: usize = 4_096;
    const MAXIMUM_STRESS_WORKING_SET: usize = 65_536;

    let throughput = if load_budget_per_frame == usize::MAX {
        UNBOUNDED_THROUGHPUT_HINT
    } else {
        load_budget_per_frame.max(1).min(MAXIMUM_STRESS_WORKING_SET)
    };
    let throughput_reserve = throughput.saturating_mul(32).max(256);

    let horizon_steps = demands
        .iter()
        .map(|request| {
            let demand = request.scope();
            let motion = VoxelDemandMotion::with_expected_latency(
                demand,
                motions.velocity_metres_per_second(demand.source()),
                expected_build_seconds,
            );
            let chunks = motion.predicted_offset_native.length()
                / MATERIALIZATION_CHUNK_SIZE as f32;
            if !chunks.is_finite() || chunks <= 0.0 {
                0
            } else {
                chunks.ceil().clamp(0.0, MAXIMUM_STRESS_WORKING_SET as f32) as usize
            }
        })
        .max()
        .unwrap_or(0);

    let lateral_reserve = throughput.saturating_mul(8);
    let requested = throughput_reserve.max(
        horizon_steps.saturating_add(1).saturating_add(lateral_reserve),
    );
    let throughput_ceiling =
        throughput.saturating_mul(256).max(MINIMUM_STRESS_WORKING_SET);

    requested.min(throughput_ceiling).min(MAXIMUM_STRESS_WORKING_SET)
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

fn demand_plan_still_valid(
    previous: &[VoxelDemandPlanKey],
    next: &[VoxelDemandPlanKey],
) -> bool {
    if previous == next {
        return true;
    }
    if previous.len() != 1 || next.len() != 1 {
        return false;
    }

    let previous = previous[0];
    let next = next[0];

    if previous.source != next.source
        || previous.priority != next.priority
        || previous.roles != next.roles
        || previous.view_revision != 0
        || next.view_revision != 0
        || previous.motion != next.motion
        || previous.validity_chunks == 0
        || previous.validity_chunks != next.validity_chunks
    {
        return false;
    }

    let before = previous.center_key.components();
    let after = next.center_key.components();
    let dx = after[0].abs_diff(before[0]);
    let dy = after[1].abs_diff(before[1]);
    let dz = after[2].abs_diff(before[2]);
    let displacement = dx.max(dy).max(dz);
    let guard = u64::from(previous.validity_chunks.max(2) / 2);

    displacement <= guard
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
        && previous.motion == next.motion
        && previous.motion == VoxelMotionPriorityKey::STATIONARY
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
    motions: Res<SpatialDemandMotionSnapshot>,
    workers: Res<VoxelWorkerPool>,
    mut frame_budget: ResMut<ReconstructibleFrameBudget>,
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
    let expected_dense_build_seconds =
        workers.estimated_latency_seconds(VoxelWorkerLane::Generation)
            + workers.estimated_latency_seconds(VoxelWorkerLane::Derivation);

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

        let Some(work_token) =
            frame_budget.begin(ReconstructibleWorkClass::Planning)
        else {
            break;
        };
        let plan_result = refresh_demand_plan(
            &world,
            &voxel_demands,
            &mut streaming,
            pinned_shell,
            &residency,
            &view_demands,
            &motions,
            layer.scale(),
            expected_dense_build_seconds,
        );
        frame_budget.finish(work_token);

        let plan_changed = match plan_result {
            Ok(changed) => changed,
            Err(error) => {
                error!(
                    error = %error,
                    world = ?world_entity,
                    scale = %layer.scale(),
                    world_leaf = %world.origin().leaf_scale(),
                    "voxel spatial demand could not be represented canonically; retiring stale voxel residency and retrying next frame"
                );
                if streaming.retire_all_desired() {
                    reconcile_materialization_residency(&mut world, &mut streaming, 0);
                }
                continue;
            }
        };

        let candidate_committed = if streaming.migration_active() {
            if !runtime_roles_ready {
                let _span = bevy::log::info_span!("voxel_residency.runtime_roles").entered();
                for (runtime, realization) in &runtimes {
                    if realization.revision() == runtime.revision() {
                        runtime_roles.insert(
                            (runtime.world(), runtime.key()),
                            realization.roles(),
                        );
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
            if let Some(radius) =
                pinned.and_then(|pinned| pinned.surface_radius_native())
            {
                prioritize_pending_work(&mut streaming, Some(radius));
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
    streaming.migration_candidate_addresses().all(|(key, requested_roles)| {
        let store = world.materializations();
        // dense-readiness-without-surface-v1
        // REALIZATION-only context is ready when dense truth is current. A
        // derived surface is required only for roles that actually consume one.
        if super::roles_require_surface(requested_roles) {
            if !store.is_derived_current(key) {
                return false;
            }
        } else if store.active_dense_revision(key).is_none() {
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

fn compare_demanded_chunks(
    a: &DemandedChunk,
    b: &DemandedChunk,
) -> std::cmp::Ordering {
    b.role_priority
        .cmp(&a.role_priority)
        .then_with(|| b.priority.cmp(&a.priority))
        .then_with(|| {
            a.trajectory_distance_squared
                .total_cmp(&b.trajectory_distance_squared)
        })
        .then_with(|| a.distance_squared.total_cmp(&b.distance_squared))
}

fn prioritize_pending_work(
    streaming: &mut VoxelStreaming,
    surface_radius_native: Option<f32>,
) {
    let _span = bevy::log::info_span!("voxel_residency.priority_sort").entered();
    let mut pending = streaming.pending_desired.drain(..).collect::<Vec<_>>();
    pending.sort_by(|a, b| {
        let shell_order = surface_radius_native.map_or(std::cmp::Ordering::Equal, |radius| {
            let a_error = (a.distance_squared.sqrt() - radius).abs();
            let b_error = (b.distance_squared.sqrt() - radius).abs();
            a_error.total_cmp(&b_error)
        });

        b.role_priority
            .cmp(&a.role_priority)
            .then_with(|| b.priority.cmp(&a.priority))
            .then(shell_order)
            .then_with(|| {
                a.trajectory_distance_squared
                    .total_cmp(&b.trajectory_distance_squared)
            })
            .then_with(|| a.distance_squared.total_cmp(&b.distance_squared))
    });
    streaming.pending_desired = pending.into();
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
    let activated = activate.clone();
    let role_refresh = streaming.take_role_refresh();

    {
        let _span = bevy::log::info_span!("voxel_residency.store_delta").entered();
        world.materializations_mut().apply_residency_delta(
            activate,
            deactivate,
            warm_inactive_materialization_limit,
        );
    }

    // Role changes are capability-pipeline changes, not regeneration events.
    // Reuse resident dense truth and wake only the products now requested.
    for key in activated.into_iter().chain(role_refresh) {
        if streaming.surface_required(key) {
            world.materializations_mut().ensure_derived_dirty(key);
        }
        world.materializations_mut().refresh_render_membership(key);
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
    motions: &SpatialDemandMotionSnapshot,
    context_scale: SpatialScale,
    expected_build_seconds: f64,
) -> Result<bool, VoxelDemandPlanError> {
    let desired_budget = desired_chunk_budget(
        streaming.load_budget_per_frame(),
        demands,
        motions,
        expected_build_seconds,
    );

    let key = {
        let _span = bevy::log::info_span!("voxel_residency.plan_key").entered();
        demand_plan_key(
            world,
            demands,
            view_demands,
            motions,
            expected_build_seconds,
        )?
    };

    if demand_plan_still_valid(&streaming.demand_key, &key) {
        if streaming.residency_revision == residency.revision() {
            return Ok(false);
        }
        {
            let _span =
                bevy::log::info_span!("voxel_residency.revalidate_context").entered();
            validate_context_residency(demands, residency, context_scale)?;
        }
        streaming.residency_revision = residency.revision();
        return Ok(false);
    }

    {
        let _span = bevy::log::info_span!("voxel_residency.validate_context").entered();
        validate_context_residency(demands, residency, context_scale)?;
    }

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
        && previous_box.intersection(next_box).is_some()
    {
        let _span = bevy::log::info_span!("voxel_residency.enumerate_delta").entered();
        let request = demands[0];
        let request_scope = request.scope();
        let motion = VoxelDemandMotion::with_expected_latency(
            request_scope,
            motions.velocity_metres_per_second(request_scope.source()),
            expected_build_seconds,
        );
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
            match demanded_chunk_for_key(world, request, materialization, motion) {
                Ok(demanded) => entering.push(demanded),
                Err(value) => error = Some(value),
            }
        });
        if let Some(error) = error {
            return Err(error.into());
        }
        entering.sort_by(compare_demanded_chunks);
        drop(_span);

        {
            let _span = bevy::log::info_span!("voxel_residency.stage_delta").entered();
            streaming.stage_incremental_desired(leaving, entering);
        }
        streaming.demand_key = key;
        streaming.residency_revision = residency.revision();
        return Ok(true);
    }

    let desired = {
        let _span =
            bevy::log::info_span!("voxel_residency.enumerate_desired.full").entered();
        demanded_chunk_addresses_with_motion(
            world,
            demands,
            pinned_shell,
            view_demands,
            motions,
            expected_build_seconds,
            desired_budget,
        )?
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
    motions: &SpatialDemandMotionSnapshot,
    expected_build_seconds: f64,
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
        let velocity = motions.velocity_metres_per_second(demand.source());
        let motion = VoxelDemandMotion::with_expected_latency(
            demand,
            velocity,
            expected_build_seconds,
        );
        let (minimum_offset, maximum_offset) = motion.demand_offsets(half);

        result.push(VoxelDemandPlanKey {
            source: demand.source(),
            center_key,
            minimum: checked_ivec3(((local + minimum_offset) / size).floor())?,
            maximum: checked_ivec3(((local + maximum_offset) / size).floor())?,
            priority: demand.priority(),
            roles: request.roles().bits(),
            view_revision: request.view_source().map_or(0, |_| view_demands.revision()),
            motion: quantized_motion_key(velocity, motion.lookahead_seconds),
            validity_chunks: {
                let raw = (
                    motion.predicted_offset_native.length()
                        / MATERIALIZATION_CHUNK_SIZE as f32
                )
                .ceil()
                .clamp(0.0, u32::MAX as f32) as u32;
                if raw == 0 {
                    0
                } else {
                    raw.checked_next_power_of_two().unwrap_or(u32::MAX)
                }
            },
        });
    }
    Ok(result)
}


fn demanded_chunk_for_key(
    world: &VoxelWorld,
    request: VoxelRealizationScope,
    key: VoxelMaterializationKey,
    motion: VoxelDemandMotion,
) -> Result<DemandedChunk, UsfPositionError> {
    let demand = request.scope();
    let center = world.materialization_address(key)?.center()?;
    let bound = demand.half_extent_native().length()
        + motion.predicted_offset_native.length()
        + MATERIALIZATION_CHUNK_SIZE as f32 * 2.0
        + 1.0;
    let relative = center.relative_at_scale_bounded(
        &demand.center(),
        demand.scale(),
        bound.max(MATERIALIZATION_CHUNK_SIZE as f32 * 2.0),
    )?;

    Ok(make_demanded_chunk(
        demand,
        request,
        key,
        relative,
        motion,
    ))
}

fn merge_demanded_chunk(
    merged: &mut HashMap<VoxelMaterializationKey, DemandedChunk>,
    candidate: DemandedChunk,
) {
    merged
        .entry(candidate.key)
        .and_modify(|current| {
            current.roles = current.roles.union(candidate.roles);
            current.role_priority = demand_role_priority(current.roles);
            if candidate.priority > current.priority
                || (candidate.priority == current.priority
                    && candidate.trajectory_distance_squared
                        < current.trajectory_distance_squared)
            {
                current.priority = candidate.priority;
                current.distance_squared = candidate.distance_squared;
                current.trajectory_distance_squared =
                    candidate.trajectory_distance_squared;
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
    motion: VoxelDemandMotion,
    merged: &mut HashMap<VoxelMaterializationKey, DemandedChunk>,
) -> Result<(), crate::spatial::UsfPositionError> {
    let relative_origin = region.relative_origin_chunks(center_key)?;
    let extent = region.extent_chunks();
    for z in 0..extent.z { for y in 0..extent.y { for x in 0..extent.x {
        let local_offset=IVec3::new(x,y,z);
        let key=region.origin().translated_chunks(local_offset)?;
        let chunk_offset=relative_origin+local_offset;
        let chunk_center=chunk_offset.as_vec3()*size+Vec3::splat(size*0.5);
        merge_demanded_chunk(
            merged,
            make_demanded_chunk(
                demand,
                request,
                key,
                chunk_center - local_center,
                motion,
            ),
        );
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
    motion: VoxelDemandMotion,
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
        merge_demanded_chunk(
            merged,
            make_demanded_chunk(
                demand,
                request,
                key,
                chunk_center - local_center,
                motion,
            ),
        );
        return Ok(());
    }

    let Some((left,right))=region.split_longest()? else { unreachable!() };
    collect_culled_region(center_key,center_origin,left,demand,request,view,pinned_shell,local_center,size,motion,merged)?;
    collect_culled_region(center_key,center_origin,right,demand,request,view,pinned_shell,local_center,size,motion,merged)
}


#[derive(Debug, Clone, Copy)]
struct PredictiveTubeCandidate {
    score: f32,
    distance_squared: f32,
    delta: IVec3,
}

impl PartialEq for PredictiveTubeCandidate {
    fn eq(&self, other: &Self) -> bool {
        self.score.total_cmp(&other.score) == std::cmp::Ordering::Equal
            && self.distance_squared.total_cmp(&other.distance_squared)
                == std::cmp::Ordering::Equal
            && self.delta == other.delta
    }
}

impl Eq for PredictiveTubeCandidate {}

impl PartialOrd for PredictiveTubeCandidate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PredictiveTubeCandidate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .score
            .total_cmp(&self.score)
            .then_with(|| {
                other
                    .distance_squared
                    .total_cmp(&self.distance_squared)
            })
            .then_with(|| other.delta.x.cmp(&self.delta.x))
            .then_with(|| other.delta.y.cmp(&self.delta.y))
            .then_with(|| other.delta.z.cmp(&self.delta.z))
    }
}

const PREDICTIVE_TUBE_NEIGHBORS: [IVec3; 6] = [
    IVec3::new(-1, 0, 0),
    IVec3::new(1, 0, 0),
    IVec3::new(0, -1, 0),
    IVec3::new(0, 1, 0),
    IVec3::new(0, 0, -1),
    IVec3::new(0, 0, 1),
];

fn predictive_tube_candidate(
    delta: IVec3,
    local_center: Vec3,
    size: f32,
    motion: VoxelDemandMotion,
) -> PredictiveTubeCandidate {
    let relative =
        delta.as_vec3() * size
            + Vec3::splat(size * 0.5)
            - local_center;
    PredictiveTubeCandidate {
        score: motion.trajectory_distance_squared(relative),
        distance_squared: relative.length_squared(),
        delta,
    }
}

fn collect_predictive_tube(
    center_key: VoxelMaterializationKey,
    demand: SpatialDemandScope,
    request: VoxelRealizationScope,
    local_center: Vec3,
    motion: VoxelDemandMotion,
    maximum_total_chunks: usize,
    merged: &mut HashMap<VoxelMaterializationKey, DemandedChunk>,
) -> Result<(), crate::spatial::UsfPositionError> {
    let size = MATERIALIZATION_CHUNK_SIZE as f32;
    const MAXIMUM_PREDICTIVE_TUBE_WORKING_SET: usize = 65_536;
    let budget = maximum_total_chunks.max(1).min(MAXIMUM_PREDICTIVE_TUBE_WORKING_SET);
    if merged.len() >= budget {
        return Ok(());
    }

    let (minimum_offset, maximum_offset) =
        motion.demand_offsets(demand.half_extent_native());
    let minimum = checked_ivec3(((local_center + minimum_offset) / size).floor())?;
    let maximum = checked_ivec3(((local_center + maximum_offset) / size).floor())?;

    let in_bounds = |delta: IVec3| {
        delta.x >= minimum.x && delta.x <= maximum.x
            && delta.y >= minimum.y && delta.y <= maximum.y
            && delta.z >= minimum.z && delta.z <= maximum.z
    };
    let relative_for = |delta: IVec3| {
        delta.as_vec3() * size + Vec3::splat(size * 0.5) - local_center
    };

    let remaining = budget.saturating_sub(merged.len());
    let maximum_horizon = remaining.saturating_sub(1) as f32 * size;
    let mut target_offset = motion.predicted_offset_native;
    let target_length = target_offset.length();
    if target_length > maximum_horizon && target_length > f32::EPSILON {
        target_offset *= maximum_horizon / target_length;
    }

    let target_delta =
        checked_ivec3(((local_center + target_offset) / size).floor())?;
    let total_steps = target_delta.x.unsigned_abs()
        .max(target_delta.y.unsigned_abs())
        .max(target_delta.z.unsigned_abs()) as usize;

    let mut visited = HashSet::<IVec3>::with_capacity(
        budget.saturating_sub(merged.len()).min(4_096).saturating_mul(2).max(32),
    );
    let mut centerline =
        Vec::<IVec3>::with_capacity(total_steps.saturating_add(1).min(remaining));
    let mut previous = None::<IVec3>;

    let steps = total_steps.min(remaining.saturating_sub(1));
    for step in 0..=steps {
        let delta = if total_steps == 0 {
            IVec3::ZERO
        } else {
            let t = step as f32 / total_steps as f32;
            (target_delta.as_vec3() * t).round().as_ivec3()
        };
        if previous == Some(delta) || !in_bounds(delta) {
            continue;
        }
        previous = Some(delta);
        visited.insert(delta);
        centerline.push(delta);

        let key = center_key.translated_chunks(delta)?;
        merge_demanded_chunk(
            merged,
            make_demanded_chunk(demand, request, key, relative_for(delta), motion),
        );
        if merged.len() >= budget {
            return Ok(());
        }
    }

    let mut frontier = BinaryHeap::<PredictiveTubeCandidate>::with_capacity(
        budget.saturating_sub(merged.len()).min(4_096).saturating_mul(2),
    );

    let push_neighbors =
        |origin: IVec3,
         visited: &mut HashSet<IVec3>,
         frontier: &mut BinaryHeap<PredictiveTubeCandidate>| {
            for offset in PREDICTIVE_TUBE_NEIGHBORS {
                let Some(x) = origin.x.checked_add(offset.x) else { continue; };
                let Some(y) = origin.y.checked_add(offset.y) else { continue; };
                let Some(z) = origin.z.checked_add(offset.z) else { continue; };
                let delta = IVec3::new(x, y, z);
                if !in_bounds(delta) || !visited.insert(delta) {
                    continue;
                }
                frontier.push(predictive_tube_candidate(delta, local_center, size, motion));
            }
        };

    for &delta in &centerline {
        push_neighbors(delta, &mut visited, &mut frontier);
    }

    while merged.len() < budget {
        let Some(candidate) = frontier.pop() else {
            break;
        };
        let key = center_key.translated_chunks(candidate.delta)?;
        merge_demanded_chunk(
            merged,
            make_demanded_chunk(
                demand,
                request,
                key,
                relative_for(candidate.delta),
                motion,
            ),
        );
        push_neighbors(candidate.delta, &mut visited, &mut frontier);
    }

    Ok(())
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
    demanded_chunk_addresses_with_motion(
        world,
        demands,
        pinned_shell,
        view_demands,
        &SpatialDemandMotionSnapshot::default(),
        0.0,
        usize::MAX,
    )
}

fn demanded_chunk_addresses_with_motion<T>(
    world: &VoxelWorld,
    demands: &[T],
    pinned_shell: Option<(Entity, f32)>,
    view_demands: &UsfViewDemandSnapshot,
    motions: &SpatialDemandMotionSnapshot,
    expected_build_seconds: f64,
    maximum_desired_chunks: usize,
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
        let local_center =
            center.relative_to(center_address.query_origin(), size + 0.01)?;
        let half = demand.half_extent_native();
        let motion = VoxelDemandMotion::with_expected_latency(
            demand,
            motions.velocity_metres_per_second(demand.source()),
            expected_build_seconds,
        );
        let (minimum_offset, maximum_offset) = motion.demand_offsets(half);
        let minimum =
            checked_ivec3(((local_center + minimum_offset) / size).floor())?;
        let maximum =
            checked_ivec3(((local_center + maximum_offset) / size).floor())?;

        let view = match request.view_source() {
            Some(source) => {
                let Some(view) = view_demands.get(source) else { continue; };
                Some(view)
            }
            None => None,
        };
        let shell = pinned_shell.filter(|(source, _)| demand.source() == *source);

        if view.is_none() && shell.is_none() && motion.direction_native != Vec3::ZERO {
            collect_predictive_tube(
                center_key,
                demand,
                request,
                local_center,
                motion,
                maximum_desired_chunks,
                &mut merged,
            )?;
        } else {
            let region =
                VoxelRegionSpan::from_relative_bounds(center_key, minimum, maximum)?;
            if view.is_some() || shell.is_some() {
                collect_culled_region(
                    center_key,
                    center_address.origin(),
                    region,
                    demand,
                    request,
                    view,
                    shell,
                    local_center,
                    size,
                    motion,
                    &mut merged,
                )?;
            } else {
                collect_all_region_leaves(
                    center_key,
                    region,
                    demand,
                    request,
                    local_center,
                    size,
                    motion,
                    &mut merged,
                )?;
            }
        }
    }

    let mut desired = merged.into_values().collect::<Vec<_>>();
    desired.sort_by(compare_demanded_chunks);
    desired.truncate(maximum_desired_chunks);
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

#[cfg(test)]
mod motion_priority_tests {
    use super::*;

    #[test]
    fn high_speed_motion_prefers_forward_work() {
        let mut ecs = World::new();
        let source = ecs.spawn_empty().id();
        let demand = SpatialDemandScope::at_scale(
            source,
            SpatialScale::ZERO,
            UsfPosition::zero(SpatialScale::ZERO),
            Vec3::splat(100.0),
            0,
        );
        let motion = VoxelDemandMotion::new(
            demand,
            DVec3::new(100.0, 0.0, 0.0),
        );

        let forward =
            motion.trajectory_distance_squared(Vec3::new(50.0, 0.0, 0.0));
        let behind =
            motion.trajectory_distance_squared(Vec3::new(-50.0, 0.0, 0.0));

        assert!(
            forward < behind,
            "forward work should outrank equidistant trailing work: {forward} vs {behind}"
        );
    }

    #[test]
    fn stationary_motion_reduces_to_distance_order() {
        let motion = VoxelDemandMotion::stationary();
        let relative = Vec3::new(12.0, -3.0, 4.0);
        assert_eq!(
            motion.trajectory_distance_squared(relative),
            relative.length_squared()
        );
    }
}



#[cfg(test)]
mod adaptive_streaming_pressure_tests {
    use super::*;

    #[test]
    fn high_speed_keeps_full_cross_section_and_extends_forward() {
        let mut ecs = World::new();
        let source = ecs.spawn_empty().id();
        let half = Vec3::splat(100.0);
        let demand = SpatialDemandScope::at_scale(
            source,
            SpatialScale::ZERO,
            UsfPosition::zero(SpatialScale::ZERO),
            half,
            0,
        );
        let motion =
            VoxelDemandMotion::new(demand, DVec3::new(1_000.0, 0.0, 0.0));

        let (minimum, maximum) = motion.demand_offsets(half);
        assert_eq!(minimum, -half);
        assert!(maximum.x > half.x);
        assert_eq!(maximum.y, half.y);
        assert_eq!(maximum.z, half.z);
    }

    #[test]
    fn stationary_motion_keeps_full_geometry_at_every_load_tier() {
        let half = Vec3::new(100.0, 60.0, 80.0);
        for tier in 0..=4 {
            let (minimum, maximum) =
                VoxelDemandMotion::stationary().demand_offsets(half);
            assert_eq!(minimum, -half);
            assert_eq!(maximum, half);
        }
    }
}

#[cfg(test)]
mod high_speed_predictive_tube_tests {
    use super::*;

    #[test]
    fn predictive_horizon_is_not_clamped_to_the_local_load_radius() {
        let mut ecs = World::new();
        let source = ecs.spawn_empty().id();
        let demand = SpatialDemandScope::at_scale(
            source,
            SpatialScale::ZERO,
            UsfPosition::zero(SpatialScale::ZERO),
            Vec3::splat(32.0),
            0,
        );
        let motion =
            VoxelDemandMotion::new(demand, DVec3::new(4_000.0, 0.0, 0.0));

        assert!(
            motion.predicted_offset_native.x > 32.0,
            "prediction must reach beyond the ordinary load radius at high speed"
        );
    }

    #[test]
    fn disjoint_boxes_are_detected_as_a_full_jump() {
        let a = VoxelMaterializationBox {
            minimum: [0, 0, 0],
            maximum: [3, 3, 3],
        };
        let b = VoxelMaterializationBox {
            minimum: [20, 0, 0],
            maximum: [23, 3, 3],
        };
        assert!(a.intersection(b).is_none());
    }
}
