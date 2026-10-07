//! Sparse staged frontier construction and publication specs.

use super::super::*;
use super::balance::*;
use super::input::*;

pub(super) fn local_frontier_spacing<'a>(
    specs: impl IntoIterator<Item = &'a CelestialClipmapBlockSpec>,
    planning_anchor_local: DVec3,
    probe_radius_metres: f64,
) -> Option<f64> {
    specs
        .into_iter()
        .filter(|spec| {
            block_distance_to_point(spec.key, planning_anchor_local) <= probe_radius_metres
        })
        .map(|spec| spec.key.spacing_metres())
        .min_by(f64::total_cmp)
}

pub(in crate::voxel::resolution::live) fn initial_stage_for_plan(
    stages: &[Vec<CelestialClipmapBlockSpec>],
    committed_specs: &HashSet<CelestialClipmapBlockSpec>,
    input: CelestialClipmapPlanInput,
) -> Option<usize> {
    if stages.is_empty() {
        return None;
    }
    if committed_specs.is_empty() {
        return Some(0);
    }

    //
    // Preserve whatever local quality is already visible at the new focus. Do
    // not regress to roots, but also do not wait for the entire final frontier
    // when an intermediate replacement is already at least as good.
    let fine_extent = input.finest.sample_spacing_metres() * BLOCK_SUBDIVISIONS as f64;
    let probe_radius = input.validity_radius_metres.max(fine_extent * 4.0);

    let existing = local_frontier_spacing(
        committed_specs.iter(),
        input.planning_anchor_local,
        probe_radius,
    );

    // If the previous frontier does not cover the new focus, require a useful
    // bootstrap (within 8x target) before swapping. This is still much smaller
    // than a final-frontier global barrier.
    let maximum_acceptable = existing.unwrap_or(input.finest.sample_spacing_metres() * 8.0);

    stages
        .iter()
        .position(|stage| {
            local_frontier_spacing(stage.iter(), input.planning_anchor_local, probe_radius)
                .is_some_and(|spacing| spacing <= maximum_acceptable * 1.001)
        })
        .or(Some(stages.len() - 1))
}

pub(super) fn body_root_coordinate_bounds(
    field: CelestialVoxelField,
    coarsest: VoxelPresentationResolution,
) -> Option<(i32, i32)> {
    let extent = coarsest.sample_spacing_metres() * BLOCK_SUBDIVISIONS as f64;
    if !extent.is_finite() || extent <= 0.0 {
        return None;
    }
    let radius = field.conservative_outer_radius_metres() * WHOLE_BODY_ROOT_MARGIN;
    let low = (-radius / extent).floor();
    let high = (radius / extent).floor();
    if low < f64::from(i32::MIN) || high > f64::from(i32::MAX) || low > high {
        return None;
    }
    Some((low as i32, high as i32))
}

/// Converts one balanced leaf frontier into deterministic build specs.
///
/// Priority ordering only needs monotonic distance, so avoid square roots here.
/// More importantly, this is no longer called from the inner refinement loop.
pub(super) fn specs_for_frontier(
    leaves: &HashSet<CelestialClipmapBlockKey>,
    planning_anchor_local: DVec3,
) -> Vec<CelestialClipmapBlockSpec> {
    let mut transitions = transition_faces_for_frontier(leaves);
    let mut ordered = leaves
        .iter()
        .copied()
        .map(|key| {
            let distance2 = block_distance_squared_to_point(key, planning_anchor_local);
            (distance2.to_bits(), key)
        })
        .collect::<Vec<_>>();

    // Distances are finite/non-negative, so IEEE positive-f64 bits preserve
    // numeric order while avoiding total_cmp in the comparator hot loop.
    ordered.sort_unstable_by_key(|(distance_bits, key)| {
        (
            *distance_bits,
            -i32::from(key.resolution.binary_exponent()),
            key.coord.x,
            key.coord.y,
            key.coord.z,
        )
    });

    ordered
        .into_iter()
        .map(|(_, key)| CelestialClipmapBlockSpec {
            key,
            transition_faces: transitions.remove(&key).unwrap_or_default(),
        })
        .collect()
}

/// Coarsen an already-balanced final frontier to one publication cap.
///
/// No field queries occur here. Every final leaf maps to its exact dyadic
/// ancestor at `cap` if it is finer than the cap; duplicates collapse. Because
/// the final frontier is 2:1 balanced, applying one common minimum-resolution
/// cap preserves a complete balanced frontier while exposing progressively
/// finer transactions.
pub(super) fn frontier_capped_at_resolution(
    final_leaves: &HashSet<CelestialClipmapBlockKey>,
    cap: VoxelPresentationResolution,
) -> HashSet<CelestialClipmapBlockKey> {
    let mut staged = HashSet::<CelestialClipmapBlockKey>::with_capacity(final_leaves.len());

    for &key in final_leaves {
        let staged_key = if key.resolution < cap {
            key.ancestor_at(cap).unwrap_or(key)
        } else {
            key
        };
        staged.insert(staged_key);
    }

    staged
}

pub(super) fn append_progressive_frontier_stages(
    stages: &mut Vec<Vec<CelestialClipmapBlockSpec>>,
    final_leaves: &HashSet<CelestialClipmapBlockKey>,
    final_stage: &[CelestialClipmapBlockSpec],
    coarsest: VoxelPresentationResolution,
    finest: VoxelPresentationResolution,
    ordering_anchor_local: DVec3,
) {
    let stride = RECORDED_FRONTIER_BINARY_LEVEL_STRIDE.max(1);
    let mut exponent = coarsest.binary_exponent().saturating_sub(stride);

    while exponent > finest.binary_exponent()
        && stages.len().saturating_add(1) < MAX_RECORDED_FRONTIER_STAGES
    {
        let cap = VoxelPresentationResolution::new(exponent);
        let staged_leaves = frontier_capped_at_resolution(final_leaves, cap);
        let staged_specs = {
            let _span =
                bevy::log::info_span!("voxel.worker.presentation_planning.progressive_specs")
                    .entered();
            specs_for_frontier(&staged_leaves, ordering_anchor_local)
        };

        if !staged_specs.is_empty()
            && stages.last() != Some(&staged_specs)
            && staged_specs.as_slice() != final_stage
        {
            stages.push(staged_specs);
        }

        exponent = exponent.saturating_sub(stride);
    }
}

pub(in crate::voxel::resolution::live) fn sparse_frontier_leaf_budget(
    input: CelestialClipmapPlanInput,
) -> usize {
    let requested_levels = i32::from(input.coarsest.binary_exponent())
        .saturating_sub(i32::from(input.finest.binary_exponent()))
        .max(0) as usize;

    let fine_extent = input.finest.sample_spacing_metres() * BLOCK_SUBDIVISIONS as f64;
    let validity_blocks = if fine_extent.is_finite() && fine_extent > 0.0 {
        (input.validity_radius_metres / fine_extent)
            .ceil()
            .clamp(1.0, 64.0) as usize
    } else {
        1
    };

    MIN_SPARSE_FRONTIER_LEAVES
        .saturating_add(requested_levels.saturating_mul(LEAVES_PER_REQUESTED_LEVEL))
        .saturating_add(validity_blocks.saturating_mul(64))
        .clamp(MIN_SPARSE_FRONTIER_LEAVES, MAX_SPARSE_FRONTIER_LEAVES)
}

const RECORDED_FRONTIER_BINARY_LEVEL_STRIDE: i16 = 4;

pub(super) fn push_refinement_candidates(
    candidates: &mut BinaryHeap<ClipmapRefinementCandidate>,
    keys: impl IntoIterator<Item = CelestialClipmapBlockKey>,
    finest: VoxelPresentationResolution,
    observer_anchor_local: DVec3,
    validity_radius_metres: f64,
) {
    for key in keys {
        if let Some(candidate) =
            refinement_candidate(key, finest, observer_anchor_local, validity_radius_metres)
        {
            candidates.push(candidate);
        }
    }
}

pub(super) fn should_record_frontier_checkpoint(
    recorded: VoxelPresentationResolution,
    current: VoxelPresentationResolution,
    finest: VoxelPresentationResolution,
) -> bool {
    if current >= recorded {
        return false;
    }
    let gained = recorded
        .binary_exponent()
        .saturating_sub(current.binary_exponent());
    gained >= RECORDED_FRONTIER_BINARY_LEVEL_STRIDE || current <= finest
}

/// Expensive sparse staged-frontier construction.
///
/// The coarse whole-body ancestry is conservative. Fine detail is a sparse
/// boundary aperture: the planner refines only blocks with actual local SDF
/// boundary evidence, balances the local transition ring, and records a
/// transaction stage only when local LOD depth improves.

pub(in crate::voxel::resolution::live) fn build_plan(
    field: CelestialVoxelField,
    input: CelestialClipmapPlanInput,
    surface_cache: &mut CelestialClipmapSurfaceCache,
    warm_replan: bool,
) -> Option<Vec<Vec<CelestialClipmapBlockSpec>>> {
    assert!(
        input.finest <= input.coarsest,
        "clipmap planner finest resolution must not be coarser than root"
    );
    assert!(
        input.observer_anchor_local.is_finite() && input.planning_anchor_local.is_finite(),
        "clipmap planner anchors must be finite"
    );

    surface_cache.begin_plan(field);
    let result = build_plan_inner(field, input, surface_cache, warm_replan);
    surface_cache.finish_plan();
    result
}

pub(super) fn build_plan_inner(
    field: CelestialVoxelField,
    input: CelestialClipmapPlanInput,
    surface_cache: &mut CelestialClipmapSurfaceCache,
    warm_replan: bool,
) -> Option<Vec<Vec<CelestialClipmapBlockSpec>>> {
    let sampler = field.presentation_sampler(1.0)?;
    let classifier = ClipmapBoundaryClassifier::new(field, &sampler);
    let maximum_leaves = sparse_frontier_leaf_budget(input);
    let mut leaves = planner_root_leaves(
        field,
        input.coarsest,
        maximum_leaves,
        surface_cache,
        &classifier,
        input.visibility,
    )?;
    // Cold start is latency-sensitive. Publish the cheapest complete
    // whole-body frontier immediately instead of first solving the eventual
    // fine frontier and synthesizing a coarse stage afterwards.
    if !warm_replan {
        let bootstrap = specs_for_frontier(&leaves, input.observer_anchor_local);
        return (!bootstrap.is_empty()).then_some(vec![bootstrap]);
    }

    refine_plan_frontier(
        &mut leaves,
        input,
        maximum_leaves,
        surface_cache,
        &classifier,
    );

    let final_stage = specs_for_frontier(&leaves, input.observer_anchor_local);
    if final_stage.is_empty() {
        return None;
    }

    Some(build_publication_stages(
        None,
        &leaves,
        final_stage,
        input,
    ))
}

pub(super) fn planner_root_leaves(
    field: CelestialVoxelField,
    coarsest: VoxelPresentationResolution,
    maximum_leaves: usize,
    surface_cache: &mut CelestialClipmapSurfaceCache,
    classifier: &ClipmapBoundaryClassifier<'_>,
    visibility: ClipmapVisibilityDemand,
) -> Option<HashSet<CelestialClipmapBlockKey>> {
    let _span = bevy::log::info_span!("voxel.worker.presentation_planning.roots").entered();
    let (root_low, root_high) = body_root_coordinate_bounds(field, coarsest)?;
    let mut leaves = HashSet::<CelestialClipmapBlockKey>::with_capacity(maximum_leaves.min(8_192));

    for z in root_low..=root_high {
        for y in root_low..=root_high {
            for x in root_low..=root_high {
                let key = CelestialClipmapBlockKey {
                    resolution: coarsest,
                    coord: IVec3::new(x, y, z),
                };
                if !visibility.demands_block(key) {
                    continue;
                }
                if surface_cache.intersects(key, classifier) {
                    leaves.insert(key);
                }
            }
        }
    }

    (!leaves.is_empty()).then_some(leaves)
}

pub(super) fn surface_focus_resolution(
    input: CelestialClipmapPlanInput,
) -> VoxelPresentationResolution {
    let distance = (input.observer_anchor_local - input.planning_anchor_local).length();
    target_resolution_at_distance(
        input.finest,
        effective_observer_lod_distance_metres(distance, input.validity_radius_metres),
    )
    .min(input.coarsest)
}

pub(super) fn refine_plan_frontier(
    leaves: &mut HashSet<CelestialClipmapBlockKey>,
    input: CelestialClipmapPlanInput,
    maximum_leaves: usize,
    surface_cache: &mut CelestialClipmapSurfaceCache,
    classifier: &ClipmapBoundaryClassifier<'_>,
) {
    let _span = bevy::log::info_span!("voxel.worker.presentation_planning.refine").entered();
    let focus_resolution = surface_focus_resolution(input);
    let mut candidates =
        BinaryHeap::<ClipmapRefinementCandidate>::with_capacity(leaves.len().min(8_192));
    push_refinement_candidates(
        &mut candidates,
        leaves.iter().copied(),
        input.finest,
        input.observer_anchor_local,
        input.validity_radius_metres,
    );

    let mut journal =
        Vec::<LeafRefinementMutation>::with_capacity(MAX_PRIMARY_REFINEMENTS_PER_WAVE * 2);
    let mut inserted = Vec::<CelestialClipmapBlockKey>::with_capacity(8);
    let mut balance_seeds =
        Vec::<CelestialClipmapBlockKey>::with_capacity(MAX_PRIMARY_REFINEMENTS_PER_WAVE * 8);
    let mut balanced_inserted =
        Vec::<CelestialClipmapBlockKey>::with_capacity(MAX_PRIMARY_REFINEMENTS_PER_WAVE * 8);

    loop {
        journal.clear();
        inserted.clear();
        balance_seeds.clear();
        balanced_inserted.clear();

        let focus = leaf_containing_point(
            leaves,
            input.planning_anchor_local,
            focus_resolution,
            input.coarsest,
        )
        .filter(|key| key.resolution > focus_resolution);

        if focus.is_none() && candidates.is_empty() {
            break;
        }
        if !refine_plan_wave(
            leaves,
            focus,
            &mut candidates,
            input,
            maximum_leaves,
            surface_cache,
            classifier,
            &mut inserted,
            &mut journal,
            &mut balance_seeds,
            &mut balanced_inserted,
        ) {
            break;
        }
    }
}

fn refine_plan_wave(
    leaves: &mut HashSet<CelestialClipmapBlockKey>,
    focus: Option<CelestialClipmapBlockKey>,
    candidates: &mut BinaryHeap<ClipmapRefinementCandidate>,
    input: CelestialClipmapPlanInput,
    maximum_leaves: usize,
    surface_cache: &mut CelestialClipmapSurfaceCache,
    classifier: &ClipmapBoundaryClassifier<'_>,
    inserted: &mut Vec<CelestialClipmapBlockKey>,
    journal: &mut Vec<LeafRefinementMutation>,
    balance_seeds: &mut Vec<CelestialClipmapBlockKey>,
    balanced_inserted: &mut Vec<CelestialClipmapBlockKey>,
) -> bool {
    let mut primary = 0usize;
    if let Some(focus_key) = focus {
        if leaves.len().saturating_add(7) > maximum_leaves {
            return false;
        }
        let outcome = refine_leaf_transactional(
            input,
            leaves,
            focus_key,
            surface_cache,
            classifier,
            inserted,
            journal,
        );
        if outcome == LeafRefinementOutcome::Refined {
            primary = 1;
            balance_seeds.extend(inserted.iter().copied());
        }
    }

    while primary < MAX_PRIMARY_REFINEMENTS_PER_WAVE {
        let Some(candidate) = candidates.pop() else {
            break;
        };
        if !leaves.contains(&candidate.key) {
            continue;
        }
        if leaves.len().saturating_add(7) > maximum_leaves {
            break;
        }

        let outcome = refine_leaf_transactional(
            input,
            leaves,
            candidate.key,
            surface_cache,
            classifier,
            inserted,
            journal,
        );
        if outcome != LeafRefinementOutcome::Refined {
            continue;
        }
        primary += 1;
        balance_seeds.extend(inserted.iter().copied());
    }

    if primary == 0 {
        return false;
    }
    if !balance_leaves_2_to_1_from_seeds(
        input,
        leaves,
        surface_cache,
        classifier,
        input.coarsest.binary_exponent(),
        maximum_leaves,
        balance_seeds,
        journal,
        balanced_inserted,
    ) {
        rollback_leaf_refinements(leaves, journal);
        return false;
    }

    push_refinement_candidates(
        candidates,
        balance_seeds
            .iter()
            .copied()
            .chain(balanced_inserted.iter().copied()),
        input.finest,
        input.observer_anchor_local,
        input.validity_radius_metres,
    );

    leaves.len().saturating_add(7) <= maximum_leaves
}

pub(super) fn build_publication_stages(
    bootstrap: Option<Vec<CelestialClipmapBlockSpec>>,
    leaves: &HashSet<CelestialClipmapBlockKey>,
    final_stage: Vec<CelestialClipmapBlockSpec>,
    input: CelestialClipmapPlanInput,
) -> Vec<Vec<CelestialClipmapBlockSpec>> {
    let mut stages = Vec::with_capacity(MAX_RECORDED_FRONTIER_STAGES);
    if let Some(bootstrap) = bootstrap
        && bootstrap != final_stage
    {
        stages.push(bootstrap);
    }

    append_progressive_frontier_stages(
        &mut stages,
        leaves,
        &final_stage,
        input.coarsest,
        input.finest,
        input.observer_anchor_local,
    );
    if stages.last() != Some(&final_stage) {
        stages.push(final_stage);
    }

    assert!(
        !stages.is_empty(),
        "planner must publish at least one stage"
    );
    assert!(
        stages.len() <= MAX_RECORDED_FRONTIER_STAGES,
        "planner publication stage cap must be respected"
    );
    stages
}
