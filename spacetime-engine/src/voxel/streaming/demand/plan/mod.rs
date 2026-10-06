//! Bounded chunk plan, region culling and predictive tube selection.

use super::*;

mod regions;
mod tube;

use regions::{collect_all_region_leaves, collect_culled_region};
use tube::{checked_ivec3, demanded_chunk_addresses_with_motion};

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
    let focus_distance_squared = request
        .priority_focus()
        .and_then(|focus| {
            let bound = demand.half_extent_native().length()
                + motion.predicted_offset_native.length()
                + MATERIALIZATION_CHUNK_SIZE as f32 * 4.0
                + 1.0;
            focus
                .relative_at_scale_bounded(&demand.center(), demand.scale(), bound)
                .ok()
        })
        .map_or(f32::INFINITY, |focus_relative| {
            (relative - focus_relative).length_squared()
        });

    DemandedChunk {
        key,
        priority: demand.priority(),
        distance_squared: relative.length_squared(),
        trajectory_distance_squared: motion.trajectory_distance_squared(relative),
        focus_distance_squared,
        role_priority: demand_role_priority(request.roles()),
        roles: request.roles(),
    }
}

fn estimated_local_chunk_count(half_extent_native: Vec3) -> usize {
    const MAXIMUM_WORKING_SET: usize = 65_536;
    let size = MATERIALIZATION_CHUNK_SIZE as f32;
    let axis = |half: f32| {
        if !half.is_finite() {
            return MAXIMUM_WORKING_SET;
        }
        // Two guard cells cover chunk-center phase near each side.
        ((half.abs() * 2.0 / size).ceil() as usize)
            .saturating_add(2)
            .max(1)
    };
    axis(half_extent_native.x)
        .saturating_mul(axis(half_extent_native.y))
        .saturating_mul(axis(half_extent_native.z))
        .min(MAXIMUM_WORKING_SET)
}

fn desired_chunk_budget(
    load_budget_per_frame: usize,
    demands: &[VoxelRealizationScope],
    motions: &SpatialDemandMotionSnapshot,
    expected_build_seconds: f64,
) -> usize {
    const UNBOUNDED_THROUGHPUT_HINT: usize = 256;
    const MAXIMUM_WORKING_SET: usize = 65_536;

    let throughput = if load_budget_per_frame == usize::MAX {
        UNBOUNDED_THROUGHPUT_HINT
    } else {
        load_budget_per_frame.max(1).min(MAXIMUM_WORKING_SET)
    };

    // Reserve enough queue depth to absorb scheduler/worker latency without
    // manufacturing thousands of irrelevant chunks for a tiny local demand.
    let throughput_reserve = throughput.saturating_mul(16).max(128);

    let mut local_required = 0usize;
    let mut predictive_centerline = 0usize;
    for request in demands {
        let demand = request.scope();
        local_required =
            local_required.saturating_add(estimated_local_chunk_count(demand.half_extent_native()));

        let motion = VoxelDemandMotion::with_expected_latency(
            demand,
            motions.velocity_metres_per_second(demand.source()),
            expected_build_seconds,
        );
        let chunks = motion.predicted_offset_native.length() / MATERIALIZATION_CHUNK_SIZE as f32;
        if chunks.is_finite() && chunks > 0.0 {
            predictive_centerline = predictive_centerline
                .max(chunks.ceil().clamp(0.0, MAXIMUM_WORKING_SET as f32) as usize);
        }
    }

    // The local physical box and forward centerline are useful work. Additional
    // lateral predictive breadth is bounded by near-term throughput rather than
    // by the volume of the swept AABB.
    local_required
        .saturating_add(predictive_centerline)
        .saturating_add(throughput_reserve)
        .max(throughput_reserve)
        .min(MAXIMUM_WORKING_SET)
}

fn demand_plan_still_valid(previous: &[VoxelDemandPlanKey], next: &[VoxelDemandPlanKey]) -> bool {
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

    if previous.motion != VoxelMotionPriorityKey::STATIONARY {
        return displacement <= MOVING_PLAN_CENTER_HOLD_CHUNKS;
    }

    let guard_chunks = previous.validity_chunks.max(2) / 2;
    displacement <= u64::from(guard_chunks.max(1))
}

fn incremental_plan_compatible(previous: VoxelDemandPlanKey, next: VoxelDemandPlanKey) -> bool {
    previous.source == next.source
        && previous.priority == next.priority
        && previous.roles == next.roles
        && previous.view_revision == 0
        && next.view_revision == 0
        && previous.motion == next.motion
        //
        // Moving demand is a sparse predictive tube INSIDE the swept AABB.
        // AABB slab differences are not an exact delta for that sparse set:
        // previously selected cells can remain inside the overlap while no
        // longer belonging to the new tube. Only stationary full cuboids may
        // use the cheap slab-delta path.
        && previous.motion == VoxelMotionPriorityKey::STATIONARY
}

#[derive(Debug)]
pub(super) enum VoxelDemandPlanError {
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
            Self::MissingResidentContext(context) => {
                write!(formatter, "missing resident USF context: {context:?}")
            }
        }
    }
}

pub(super) fn refresh_demand_plan(
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
            let _span = bevy::log::info_span!("voxel_residency.revalidate_context").entered();
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
        let _span = bevy::log::info_span!("voxel_residency.enumerate_desired.full").entered();
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
        let mut desired_ranks = HashMap::with_capacity(desired.len());
        let mut pending_desired = VecDeque::with_capacity(desired.len());
        for chunk in desired {
            desired_roles.insert(chunk.key, chunk.roles);
            desired_ranks.insert(chunk.key, chunk.work_rank());
            if !world.materializations().is_active(chunk.key) {
                pending_desired.push_back(chunk);
            }
        }
        streaming.stage_desired_roles(desired_roles);
        streaming.replace_desired_work_ranks(desired_ranks);
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
        let motion =
            VoxelDemandMotion::with_expected_latency(demand, velocity, expected_build_seconds);
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
                let raw = (motion.predicted_offset_native.length()
                    / MATERIALIZATION_CHUNK_SIZE as f32)
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

    Ok(make_demanded_chunk(demand, request, key, relative, motion))
}

fn merge_demanded_chunk(
    merged: &mut HashMap<VoxelMaterializationKey, DemandedChunk>,
    candidate: DemandedChunk,
) {
    merged
        .entry(candidate.key)
        .and_modify(|current| {
            let merged_roles = current.roles.union(candidate.roles);
            let candidate_better = candidate.priority > current.priority
                || (candidate.priority == current.priority
                    && candidate.focus_distance_squared < current.focus_distance_squared)
                || (candidate.priority == current.priority
                    && candidate.focus_distance_squared == current.focus_distance_squared
                    && candidate.trajectory_distance_squared < current.trajectory_distance_squared);

            if candidate_better {
                current.priority = candidate.priority;
                current.distance_squared = candidate.distance_squared;
                current.trajectory_distance_squared = candidate.trajectory_distance_squared;
                current.focus_distance_squared = candidate.focus_distance_squared;
            }
            current.roles = merged_roles;
            current.role_priority = demand_role_priority(merged_roles);
        })
        .or_insert(candidate);
}
