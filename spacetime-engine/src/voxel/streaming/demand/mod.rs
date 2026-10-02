//! Spatial-demand interpretation and voxel residency reconciliation.

use std::collections::{HashMap, VecDeque};

use bevy::math::DVec3;

use bevy::prelude::*;

use crate::{
    config::EngineConfig,
    spatial::{
        SpatialDemandMotionSnapshot, SpatialDemandScope, SpatialScale,
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
    load_tier: u8,
}

const MOTION_LOOKAHEAD_SECONDS: f32 = 1.0;
const MOTION_DIRECTION_QUANTIZATION: f64 = 8.0;
/// At extreme traversal speed the demand tail is allowed to collapse to one
/// materialization cell along the dominant movement axes.
const MOTION_MAX_TAIL_SHRINK: f32 = 0.90;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct VoxelMotionPriorityKey {
    direction: [i8; 3],
    speed_bucket: u8,
}

impl VoxelMotionPriorityKey {
    const STATIONARY: Self = Self {
        direction: [0, 0, 0],
        speed_bucket: 0,
    };
}

#[derive(Debug, Clone, Copy)]
struct VoxelDemandMotion {
    predicted_offset_native: Vec3,
    bias: f32,
    direction_native: Vec3,
}

impl VoxelDemandMotion {
    fn new(
        demand: SpatialDemandScope,
        velocity_metres_per_second: DVec3,
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

        let speed_chunks =
            speed_native / MATERIALIZATION_CHUNK_SIZE as f32;
        let bias = (speed_chunks / (speed_chunks + 1.0)).clamp(0.0, 1.0);

        let mut predicted_offset_native =
            velocity_native * MOTION_LOOKAHEAD_SECONDS;
        let max_lookahead = demand
            .half_extent_native()
            .length()
            .max(MATERIALIZATION_CHUNK_SIZE as f32);
        let predicted_length = predicted_offset_native.length();
        if predicted_length > max_lookahead && predicted_length > f32::EPSILON {
            predicted_offset_native *= max_lookahead / predicted_length;
        }

        Self {
            predicted_offset_native,
            bias,
            direction_native: velocity_native.normalize_or_zero(),
        }
    }

    const fn stationary() -> Self {
        Self {
            predicted_offset_native: Vec3::ZERO,
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

        let t =
            (relative.dot(segment) / segment_length_squared).clamp(0.0, 1.0);
        let nearest = segment * t;
        let lateral_squared = (relative - nearest).length_squared();
        let corridor_score =
            lateral_squared * 8.0 + current * 0.05;

        (current + (corridor_score - current) * self.bias).max(0.0)
    }

    /// Actual asymmetric materialization demand offsets.
    ///
    /// Velocity removes trailing volume. Forward radius intentionally remains
    /// unchanged in this tranche so faster travel reduces total demand instead
    /// of merely moving/expanding it.
    fn demand_offsets(self, half_extent: Vec3, load_tier: u8) -> (Vec3, Vec3) {
        let load_tier = load_tier.min(4);
        let cross_section_scale = match load_tier {
            0 => 1.0,
            1 => 0.80,
            2 => 0.55,
            3 => 0.35,
            _ => 0.20,
        };

        let one_cell = Vec3::splat(MATERIALIZATION_CHUNK_SIZE as f32);
        let directional_weight = self.direction_native.abs();
        let axis_scale = if self.direction_native == Vec3::ZERO {
            Vec3::splat(cross_section_scale)
        } else {
            Vec3::splat(cross_section_scale)
                + directional_weight * (1.0 - cross_section_scale)
        };
        let effective_half =
            (half_extent * axis_scale).max(half_extent.min(one_cell));

        if self.bias <= f32::EPSILON
            || self.direction_native == Vec3::ZERO
        {
            return (-effective_half, effective_half);
        }

        let shrink =
            directional_weight * (self.bias * MOTION_MAX_TAIL_SHRINK);
        let tail_floor = effective_half.min(one_cell);
        let trailing_extent =
            (effective_half * (Vec3::ONE - shrink)).max(tail_floor);

        let mut minimum = -effective_half;
        let mut maximum = effective_half;

        if self.direction_native.x > 0.0 {
            minimum.x = -trailing_extent.x;
        } else if self.direction_native.x < 0.0 {
            maximum.x = trailing_extent.x;
        }

        if self.direction_native.y > 0.0 {
            minimum.y = -trailing_extent.y;
        } else if self.direction_native.y < 0.0 {
            maximum.y = trailing_extent.y;
        }

        if self.direction_native.z > 0.0 {
            minimum.z = -trailing_extent.z;
        } else if self.direction_native.z < 0.0 {
            maximum.z = trailing_extent.z;
        }

        (minimum, maximum)
    }
}

fn quantized_motion_key(
    velocity_metres_per_second: DVec3,
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

    VoxelMotionPriorityKey {
        direction: [
            quantize(direction.x),
            quantize(direction.y),
            quantize(direction.z),
        ],
        speed_bucket,
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

fn raw_streaming_load_tier(
    streaming: &VoxelStreaming,
    demands: &[VoxelRealizationScope],
    motions: &SpatialDemandMotionSnapshot,
) -> u8 {
    let budget = streaming.load_budget_per_frame().max(1);
    let backlog_frames = streaming.pending_desired_len().div_ceil(budget);

    let backlog_tier = if backlog_frames <= 2 {
        0
    } else if backlog_frames <= 4 {
        1
    } else if backlog_frames <= 8 {
        2
    } else if backlog_frames <= 16 {
        3
    } else {
        4
    };

    let maximum_speed_chunks_per_second = demands
        .iter()
        .map(|request| {
            let demand = request.scope();
            let velocity =
                motions.velocity_metres_per_second(demand.source());
            if !velocity.is_finite() {
                return 0.0;
            }
            let factor = demand.scale().scale0_to_native_f64(1.0);
            velocity.length() * factor / f64::from(MATERIALIZATION_CHUNK_SIZE)
        })
        .filter(|speed| speed.is_finite())
        .fold(0.0_f64, f64::max);

    let speed_tier = if maximum_speed_chunks_per_second < 1.0 {
        0
    } else if maximum_speed_chunks_per_second < 4.0 {
        1
    } else if maximum_speed_chunks_per_second < 12.0 {
        2
    } else if maximum_speed_chunks_per_second < 32.0 {
        3
    } else {
        4
    };

    backlog_tier.max(speed_tier)
}

fn desired_chunk_budget(
    load_budget_per_frame: usize,
    load_tier: u8,
) -> usize {
    let load_budget_per_frame = load_budget_per_frame.max(1);
    match load_tier.min(4) {
        0 => usize::MAX,
        1 => load_budget_per_frame.saturating_mul(16).max(64),
        2 => load_budget_per_frame.saturating_mul(8).max(48),
        3 => load_budget_per_frame.saturating_mul(4).max(32),
        _ => load_budget_per_frame.saturating_mul(2).max(16),
    }
}

fn adaptive_warm_limit(
    configured_limit: usize,
    demands: &[VoxelRealizationScope],
    motions: &SpatialDemandMotionSnapshot,
) -> usize {
    let speed = demands
        .iter()
        .map(|request| {
            motions
                .velocity_metres_per_second(request.scope().source())
                .length()
        })
        .filter(|speed| speed.is_finite())
        .fold(0.0_f64, f64::max);

    let velocity_cap = if speed >= 1_000.0 {
        32
    } else if speed >= 250.0 {
        64
    } else if speed >= 50.0 {
        128
    } else if speed >= 5.0 {
        256
    } else {
        512
    };

    configured_limit.min(velocity_cap)
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
        && previous.motion == next.motion
        && previous.load_tier == 0
        && next.load_tier == 0
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
    let configured_warm_limit =
        config.voxel.streaming.warm_inactive_materialization_limit;

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
        let warm_limit =
            adaptive_warm_limit(configured_warm_limit, &voxel_demands, &motions);
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
            &motions,
            layer.scale(),
        ) {
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
                    reconcile_materialization_residency(
                        &mut world,
                        &mut streaming,
                        0,
                    );
                }
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
            prioritize_pending_work(
                &mut streaming,
                pinned.and_then(|pinned| pinned.surface_radius_native()),
            );
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
    motions: &SpatialDemandMotionSnapshot,
    context_scale: SpatialScale,
) -> Result<bool, VoxelDemandPlanError> {
    let raw_load_tier =
        raw_streaming_load_tier(streaming, demands, motions);
    let load_tier =
        streaming.update_adaptive_load_tier(raw_load_tier);
    let desired_budget =
        desired_chunk_budget(streaming.load_budget_per_frame(), load_tier);

    let key = {
        let _span = bevy::log::info_span!("voxel_residency.plan_key").entered();
        demand_plan_key(
            world,
            demands,
            view_demands,
            motions,
            load_tier,
        )?
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
        let request_scope = request.scope();
        let motion = VoxelDemandMotion::new(
            request_scope,
            motions.velocity_metres_per_second(request_scope.source()),
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
        demanded_chunk_addresses_with_motion(
            world,
            demands,
            pinned_shell,
            view_demands,
            motions,
            load_tier,
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
    load_tier: u8,
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
        let velocity =
            motions.velocity_metres_per_second(demand.source());
        let motion = VoxelDemandMotion::new(demand, velocity);
        let (minimum_offset, maximum_offset) =
            motion.demand_offsets(half, load_tier);

        result.push(VoxelDemandPlanKey {
            source: demand.source(),
            center_key,
            minimum: checked_ivec3(
                ((local + minimum_offset) / size).floor(),
            )?,
            maximum: checked_ivec3(
                ((local + maximum_offset) / size).floor(),
            )?,
            priority: demand.priority(),
            roles: request.roles().bits(),
            view_revision: request.view_source().map_or(0, |_| view_demands.revision()),
            motion: quantized_motion_key(velocity),
            load_tier,
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
    let bound =
        demand.half_extent_native().length() + MATERIALIZATION_CHUNK_SIZE as f32 * 2.0 + 1.0;
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
        0,
        usize::MAX,
    )
}

fn demanded_chunk_addresses_with_motion<T>(
    world: &VoxelWorld,
    demands: &[T],
    pinned_shell: Option<(Entity, f32)>,
    view_demands: &UsfViewDemandSnapshot,
    motions: &SpatialDemandMotionSnapshot,
    load_tier: u8,
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
        let local_center = center.relative_to(center_address.query_origin(), size + 0.01)?;
        let half = demand.half_extent_native();
        let motion = VoxelDemandMotion::new(
            demand,
            motions.velocity_metres_per_second(demand.source()),
        );
        let (minimum_offset, maximum_offset) =
            motion.demand_offsets(half, load_tier);
        let minimum = checked_ivec3(
            ((local_center + minimum_offset) / size).floor(),
        )?;
        let maximum = checked_ivec3(
            ((local_center + maximum_offset) / size).floor(),
        )?;

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
                view, shell, local_center, size, motion, &mut merged,
            )?;
        } else {
            // Exact demand still chooses the dense leaf backend today. The
            // region is now the request boundary, so another backend can later
            // satisfy it without changing materialization identity.
            collect_all_region_leaves(
                center_key, region, demand, request, local_center, size, motion, &mut merged,
            )?;
        }
    }

    let mut desired = merged.into_values().collect::<Vec<_>>();
    desired.sort_by(|a, b| {
        b.role_priority
            .cmp(&a.role_priority)
            .then_with(|| b.priority.cmp(&a.priority))
            .then_with(|| {
                a.trajectory_distance_squared
                    .total_cmp(&b.trajectory_distance_squared)
            })
            .then_with(|| a.distance_squared.total_cmp(&b.distance_squared))
    });
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
mod motion_demand_geometry_tests {
    use super::*;

    #[test]
    fn fast_motion_physically_shrinks_the_trailing_demand_radius() {
        let mut ecs = World::new();
        let source = ecs.spawn_empty().id();
        let half = Vec3::new(100.0, 60.0, 80.0);
        let demand = SpatialDemandScope::at_scale(
            source,
            SpatialScale::ZERO,
            UsfPosition::zero(SpatialScale::ZERO),
            half,
            0,
        );
        let motion = VoxelDemandMotion::new(
            demand,
            DVec3::new(1_000.0, 0.0, 0.0),
        );

        let (minimum, maximum) = motion.demand_offsets(half, load_tier);

        assert_eq!(maximum.x, half.x);
        assert!(
            minimum.x > -half.x * 0.25,
            "1 km/s should leave only a tight trailing tail, got minimum.x={}",
            minimum.x,
        );
        assert_eq!(minimum.y, -half.y);
        assert_eq!(maximum.y, half.y);
        assert_eq!(minimum.z, -half.z);
        assert_eq!(maximum.z, half.z);
    }

    #[test]
    fn stationary_motion_keeps_symmetric_demand_geometry() {
        let half = Vec3::new(100.0, 60.0, 80.0);
        let (minimum, maximum) =
            VoxelDemandMotion::stationary().demand_offsets(half, 0);
        assert_eq!(minimum, -half);
        assert_eq!(maximum, half);
    }

    #[test]
    fn negative_velocity_shrinks_the_positive_tail() {
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
        let motion = VoxelDemandMotion::new(
            demand,
            DVec3::new(-1_000.0, 0.0, 0.0),
        );

        let (minimum, maximum) = motion.demand_offsets(half, load_tier);
        assert_eq!(minimum.x, -half.x);
        assert!(maximum.x < half.x * 0.25);
    }
}

#[cfg(test)]
mod adaptive_streaming_pressure_tests {
    use super::*;

    #[test]
    fn overload_tier_collapses_cross_section_and_caps_working_set() {
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

        let (minimum, maximum) = motion.demand_offsets(half, 4);
        assert_eq!(maximum.x, half.x);
        assert!(maximum.y <= 20.01);
        assert!(maximum.z <= 20.01);
        assert!(minimum.x >= -10.01);

        assert_eq!(desired_chunk_budget(24, 4), 48);
        assert_eq!(desired_chunk_budget(24, 3), 96);
    }

    #[test]
    fn healthy_stationary_streaming_keeps_full_geometry() {
        let half = Vec3::new(100.0, 60.0, 80.0);
        let (minimum, maximum) =
            VoxelDemandMotion::stationary().demand_offsets(half, 0);
        assert_eq!(minimum, -half);
        assert_eq!(maximum, half);
        assert_eq!(desired_chunk_budget(24, 0), usize::MAX);
    }
}
