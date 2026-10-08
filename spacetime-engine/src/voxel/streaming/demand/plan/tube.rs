//! Sparse predictive centerline and prioritized chunk selection.

use super::*;

#[derive(Debug, Clone, Copy)]
struct PredictiveTubeCandidate {
    score: f32,
    distance_squared: f32,
    delta: IVec3,
}

impl PartialEq for PredictiveTubeCandidate {
    fn eq(&self, other: &Self) -> bool {
        self.score.total_cmp(&other.score) == std::cmp::Ordering::Equal
            && self.distance_squared.total_cmp(&other.distance_squared) == std::cmp::Ordering::Equal
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
            .then_with(|| other.distance_squared.total_cmp(&self.distance_squared))
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
    let relative = delta.as_vec3() * size + Vec3::splat(size * 0.5) - local_center;
    PredictiveTubeCandidate {
        score: motion.trajectory_distance_squared(relative),
        distance_squared: relative.length_squared(),
        delta,
    }
}

pub(super) fn collect_predictive_tube(
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
    let budget = maximum_total_chunks
        .max(1)
        .min(MAXIMUM_PREDICTIVE_TUBE_WORKING_SET);
    if merged.len() >= budget {
        return Ok(());
    }

    //
    // Deep prediction is useless if the controlled subject's immediate
    // collision/landing neighborhood is still missing. Seed the complete local
    // bounded demand box first. The adaptive working-set budget explicitly
    // accounts for this footprint.
    let local_half = demand.half_extent_native();
    let local_minimum = checked_ivec3(((local_center - local_half) / size).floor())?;
    let local_maximum = checked_ivec3(((local_center + local_half) / size).floor())?;
    let local_region =
        VoxelRegionSpan::from_relative_bounds(center_key, local_minimum, local_maximum)?;
    collect_all_region_leaves(
        center_key,
        local_region,
        demand,
        request,
        local_center,
        size,
        motion,
        merged,
    )?;
    if merged.len() >= budget {
        return Ok(());
    }

    let (minimum_offset, maximum_offset) = motion.demand_offsets(demand.half_extent_native());
    let minimum = checked_ivec3(((local_center + minimum_offset) / size).floor())?;
    let maximum = checked_ivec3(((local_center + maximum_offset) / size).floor())?;

    let in_bounds = |delta: IVec3| {
        delta.x >= minimum.x
            && delta.x <= maximum.x
            && delta.y >= minimum.y
            && delta.y <= maximum.y
            && delta.z >= minimum.z
            && delta.z <= maximum.z
    };
    let relative_for =
        |delta: IVec3| delta.as_vec3() * size + Vec3::splat(size * 0.5) - local_center;

    let remaining = budget.saturating_sub(merged.len());
    let maximum_horizon = remaining.saturating_sub(1) as f32 * size;
    let mut target_offset = motion.predicted_offset_native;
    let target_length = target_offset.length();
    if target_length > maximum_horizon && target_length > f32::EPSILON {
        target_offset *= maximum_horizon / target_length;
    }

    let target_delta = checked_ivec3(((local_center + target_offset) / size).floor())?;
    let total_steps = target_delta
        .x
        .unsigned_abs()
        .max(target_delta.y.unsigned_abs())
        .max(target_delta.z.unsigned_abs()) as usize;

    let mut visited = HashSet::<IVec3>::with_capacity(
        budget
            .saturating_sub(merged.len())
            .min(4_096)
            .saturating_mul(2)
            .max(32),
    );
    let mut centerline = Vec::<IVec3>::with_capacity(total_steps.saturating_add(1).min(remaining));
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
        budget
            .saturating_sub(merged.len())
            .min(4_096)
            .saturating_mul(2),
    );

    let push_neighbors =
        |origin: IVec3,
         visited: &mut HashSet<IVec3>,
         frontier: &mut BinaryHeap<PredictiveTubeCandidate>| {
            for offset in PREDICTIVE_TUBE_NEIGHBORS {
                let Some(x) = origin.x.checked_add(offset.x) else {
                    continue;
                };
                let Some(y) = origin.y.checked_add(offset.y) else {
                    continue;
                };
                let Some(z) = origin.z.checked_add(offset.z) else {
                    continue;
                };
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
            make_demanded_chunk(demand, request, key, relative_for(candidate.delta), motion),
        );
        push_neighbors(candidate.delta, &mut visited, &mut frontier);
    }

    Ok(())
}

pub(super) fn demanded_chunk_addresses_with_motion<T>(
    world: &VoxelScaleRealization,
    demands: &[T],
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
        let local_center = center.relative_to(center_address.query_origin(), size + 0.01)?;
        let half = demand.half_extent_native();
        let motion = VoxelDemandMotion::with_expected_latency(
            demand,
            motions.velocity_metres_per_second(demand.source()),
            expected_build_seconds,
        );
        let (minimum_offset, maximum_offset) = motion.demand_offsets(half);
        let minimum = checked_ivec3(((local_center + minimum_offset) / size).floor())?;
        let maximum = checked_ivec3(((local_center + maximum_offset) / size).floor())?;

        let view = match request.view_source() {
            Some(source) => {
                let Some(view) = view_demands.get(source) else {
                    continue;
                };
                Some(view)
            }
            None => None,
        };
        let surface_shell = if view.is_some() {
            match world.base() {
                crate::voxel::VoxelBase::CelestialBody(field) => {
                    let body_center = world.origin().relative_at_scale_bounded(
                        center_address.origin(),
                        demand.scale(),
                        f32::MAX,
                    )?;
                    Some((body_center, field.radius_native_f64() as f32))
                }
                _ => None,
            }
        } else {
            None
        };
        if view.is_none() && motion.direction_native != Vec3::ZERO {
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
            let region = VoxelRegionSpan::from_relative_bounds(center_key, minimum, maximum)?;
            if view.is_some() {
                collect_culled_region(
                    center_key,
                    center_address.origin(),
                    region,
                    demand,
                    request,
                    view,
                    surface_shell,
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

pub(super) fn checked_ivec3(value: Vec3) -> Result<IVec3, crate::spatial::UsfPositionError> {
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
