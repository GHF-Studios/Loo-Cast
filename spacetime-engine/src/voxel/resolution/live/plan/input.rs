//! Observer demand, plan identity and refresh relevance.

use super::super::*;

pub(super) fn visual_target_spacing_metres(
    clearance_metres: f64,
    pixels_per_radian: Option<f32>,
) -> f64 {
    let clearance = clearance_metres.abs().max(MIN_SAMPLE_SPACING_METRES);

    let clearance_target = clearance / TARGET_CELLS_PER_CLEARANCE;

    let screen_target = pixels_per_radian
        .map(f64::from)
        .filter(|value| value.is_finite() && *value > 0.0)
        .map(|pixels_per_radian| clearance * TARGET_PIXELS_PER_BINARY_SAMPLE / pixels_per_radian)
        .unwrap_or(clearance_target);

    // Both constraints are upper bounds on acceptable sample spacing. Choose
    // the stricter one; binary quantization later selects at-or-finer.
    clearance_target
        .min(screen_target)
        .clamp(MIN_SAMPLE_SPACING_METRES, MAX_FINE_SAMPLE_SPACING_METRES)
}

pub(super) fn target_resolution_at_distance(
    finest: VoxelPresentationResolution,
    distance_metres: f64,
) -> VoxelPresentationResolution {
    let requested_spacing =
        (distance_metres / TARGET_CELLS_PER_DISTANCE).max(finest.sample_spacing_metres());
    let requested =
        VoxelPresentationResolution::at_most_metres(requested_spacing).unwrap_or(finest);
    requested.max(finest)
}

/// Cheap clipmap identity derivation.
///
/// Whole-body topology is semantic-body-owned. Observer position is stored only
/// as a refinement anchor and cannot change the coarsest body representation.
pub(in crate::voxel::resolution::live) fn derive_plan_input(
    field: CelestialVoxelField,
    observer_local: DVec3,
    pixels_per_radian: Option<f32>,
    observer_speed_metres_per_second: f64,
    expected_build_seconds: f64,
    visibility: ClipmapVisibilityDemand,
) -> Option<CelestialClipmapPlanInput> {
    if !observer_local.is_finite() {
        return None;
    }

    //
    // Keep TWO facts:
    // - actual observer_local owns 3D LOD/error/motion distance;
    // - nearest boundary owns the mandatory semantic-surface refinement branch.
    //
    // Collapsing these into one projected point silently discarded altitude.
    let (planning_anchor_local, signed_clearance_metres) =
        field.nearest_boundary_local_metres(observer_local, f64::MAX)?;
    let clearance = signed_clearance_metres.abs();
    if !clearance.is_finite() {
        return None;
    }

    //
    // Interaction Scale does not participate. Presentation quality is a
    // physical/screen-space problem.
    let desired_spacing = visual_target_spacing_metres(clearance, pixels_per_radian);
    let granularity = SpatialRealizationGranularityRequest::new(
        desired_spacing,
        MIN_SAMPLE_SPACING_METRES,
        MAX_FINE_SAMPLE_SPACING_METRES,
        BLOCK_SUBDIVISIONS as u32,
        CLIPMAP_VALIDITY_AGGREGATES_ACROSS,
        observer_speed_metres_per_second,
        expected_build_seconds,
        CLIPMAP_MIN_VALIDITY_SECONDS,
        CLIPMAP_MAX_VALIDITY_SECONDS,
        CLIPMAP_LATENCY_MULTIPLIER,
    )
    .solve();

    // The quality target is a MAXIMUM acceptable spacing. Quantize to
    // the closest binary level at-or-finer, never one step coarser.
    let finest = VoxelPresentationResolution::at_most_metres(granularity.target_spacing_metres())?;

    // Stable root resolution is a function of body diameter, not altitude.
    // At least one root-block extent spans the semantic diameter; multiple fixed
    // body-local root cells cover quadrants because the lattice origin is the
    // body's origin/corner boundary rather than an observer-relative origin.
    let body_diameter_metres =
        field.conservative_outer_radius_metres() * 2.0 * WHOLE_BODY_ROOT_MARGIN;
    let coarse_spacing_target =
        (body_diameter_metres / BLOCK_SUBDIVISIONS as f64).max(finest.sample_spacing_metres());
    let coarse_exp_f64 = coarse_spacing_target.log2().ceil();
    if coarse_exp_f64 < f64::from(i16::MIN) || coarse_exp_f64 > f64::from(i16::MAX) {
        return None;
    }
    let coarsest =
        VoxelPresentationResolution::new((coarse_exp_f64 as i16).max(finest.binary_exponent()));

    // The anchor is deliberately continuous rather than snapped to a bucket.
    // A live plan decides when this anchor has moved far enough to justify a
    // replacement; boundary crossing by itself is meaningless.
    let fine_extent = finest.sample_spacing_metres() * BLOCK_SUBDIVISIONS as f64;
    let validity_radius_metres = granularity.validity_radius_metres().max(fine_extent * 4.0);

    Some(CelestialClipmapPlanInput {
        key: CelestialClipmapPlanKey {
            finest_exponent: finest.binary_exponent(),
            coarsest_exponent: coarsest.binary_exponent(),
        },
        observer_anchor_local: observer_local,
        planning_anchor_local,
        validity_radius_metres,
        visibility,
        clearance_metres: clearance,
        finest,
        coarsest,
    })
}

const FOCUS_REPLAN_VALIDITY_FRACTION: f64 = 0.125;
const FOCUS_REPLAN_MIN_FINE_EXTENTS: f64 = 1.0;

pub(super) fn plan_requires_refresh(
    plan: &CelestialClipmapPlan,
    input: CelestialClipmapPlanInput,
    field: CelestialVoxelField,
) -> bool {
    if plan.key != input.key || plan.field != field {
        return true;
    }

    let observer_displacement = (input.observer_anchor_local - plan.observer_anchor_local).length();
    let surface_displacement = (input.planning_anchor_local - plan.planning_anchor_local).length();
    let displacement = observer_displacement.max(surface_displacement);
    let fine_extent = input.finest.sample_spacing_metres() * BLOCK_SUBDIVISIONS as f64;
    if plan
        .visibility
        .requires_refresh(&input.visibility, fine_extent)
    {
        return true;
    }

    //
    // Validity radius says how much already-built terrain remains useful; it is
    // not permission for the finest focus to wander across most of that region.
    let hold_radius = (plan.validity_radius_metres * FOCUS_REPLAN_VALIDITY_FRACTION)
        .max(fine_extent * FOCUS_REPLAN_MIN_FINE_EXTENTS);
    displacement > hold_radius
}

pub(in crate::voxel::resolution::live) fn should_schedule_plan_refresh(
    plan: &CelestialClipmapPlan,
    input: CelestialClipmapPlanInput,
    field: CelestialVoxelField,
) -> bool {
    //
    // Never finish obsolete deep refinement before allowing the moving focus to
    // replan. The previous committed frontier remains visible until the newer
    // balanced transaction is ready.
    plan_requires_refresh(plan, input, field)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::voxel::resolution::live) enum ClipmapPlanTaskRelevance {
    Fresh,
    RollingProgress,
    Stale,
}

pub(super) fn clipmap_plan_keys_structurally_compatible(
    built: CelestialClipmapPlanKey,
    current: CelestialClipmapPlanKey,
) -> bool {
    // Finest exponent is observer quality state, not semantic hierarchy
    // identity. A behind/finer/coarser result may still be useful intermediate
    // progress. Body-root topology must remain identical.
    built.coarsest_exponent == current.coarsest_exponent
}

pub(in crate::voxel::resolution::live) fn plan_task_relevance(
    built: CelestialClipmapPlanInput,
    current: CelestialClipmapPlanInput,
    built_field: CelestialVoxelField,
    current_field: CelestialVoxelField,
    committed_anchor_local: Option<DVec3>,
) -> ClipmapPlanTaskRelevance {
    if built_field != current_field
        || !clipmap_plan_keys_structurally_compatible(built.key, current.key)
        || !built
            .visibility
            .result_still_relevant_to(&current.visibility)
    {
        return ClipmapPlanTaskRelevance::Stale;
    }

    let built_lag = (current.observer_anchor_local - built.observer_anchor_local).length();
    if !built_lag.is_finite() {
        return ClipmapPlanTaskRelevance::Stale;
    }

    let built_fine_extent = built.finest.sample_spacing_metres() * BLOCK_SUBDIVISIONS as f64;
    let current_fine_extent = current.finest.sample_spacing_metres() * BLOCK_SUBDIVISIONS as f64;

    let fresh_radius = built
        .validity_radius_metres
        .max(current.validity_radius_metres)
        .max(built_fine_extent * 4.0)
        .max(current_fine_extent * 4.0);

    if built_lag <= fresh_radius * 1.5 {
        return ClipmapPlanTaskRelevance::Fresh;
    }

    // A worker result can be outside the nominal freshness envelope and still
    // be valuable if it advances the currently displayed frontier toward the
    // newest target. This is the key rolling-frontier distinction.
    let Some(committed_anchor_local) = committed_anchor_local else {
        return ClipmapPlanTaskRelevance::Stale;
    };
    let committed_lag = (current.observer_anchor_local - committed_anchor_local).length();
    if !committed_lag.is_finite() {
        return ClipmapPlanTaskRelevance::Stale;
    }

    let progress_margin = built_fine_extent.min(current_fine_extent).max(1.0);
    if built_lag + progress_margin < committed_lag {
        ClipmapPlanTaskRelevance::RollingProgress
    } else {
        ClipmapPlanTaskRelevance::Stale
    }
}
