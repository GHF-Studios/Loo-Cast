//! Live celestial presentation clipmap over the voxel-local binary resolution domain.
//!
//! This is presentation only. Semantic terrain remains [`CelestialVoxelField`];
//! dense voxel worlds keep collision/editing authority. The clipmap is a
//! reconstructible mesh adapter whose LOD axis is independent of USF Scale.

use std::{
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
};

use bevy::{
    camera::{primitives::Aabb, visibility::{NoAutoAabb, RenderLayers}},
    light::{NotShadowCaster, NotShadowReceiver},
    math::DVec3,
    prelude::*,
    render::storage::ShaderBuffer,
};

use crate::reconstructible::{
    ReconstructibleFrameBudget, ReconstructibleWorkClass,
};
use crate::procedural_assets::{
    DEBUG_GRID_BASE_UV_METRES_PER_UNIT, ProceduralAssetLibrary,
};
use crate::view::USF_PRESENTATION_LAYER;
use crate::voxel::{
    MATERIALIZATION_CHUNK_SIZE, VoxelStreaming, VoxelWorld,
};

use crate::{
    ecs::UsfPresentationProjectionOf,
    spatial::{
         SpatialRealizationGranularityRequest, SpatialScale, UsfCapabilitySet,
        UsfPosition, UsfPrimaryInteractionSlice, UsfScaleLayer,
        UsfScaleRoleMask, UsfSemanticFrame, UsfSpatialSet,
        UsfViewContext, UsfViewDemandSnapshot,
        UsfViewRenderAnchor,
    },
};

use super::classification::{CelestialClipmapSurfaceCache, ClipmapBoundaryClassifier};
use super::visibility::ClipmapVisibilityDemand;
use super::topology::{
    BLOCK_SUBDIVISIONS, CLIPMAP_FACE_DIRECTIONS, CelestialClipmapBlockKey,
    block_contains_local_point, block_distance_squared_to_point, block_distance_to_point,
    leaf_containing_point, same_or_coarser_face_neighbor,
    transition_faces_for_frontier,
};

use super::{
    gpu::{
        allocation_mesh, descriptor_for_block, GpuTerrainBlock,
        GpuTerrainRuntime,
    },
    VoxelPresentationResolution, VoxelTransitionFaces,
};
use super::super::{
    presentation_palette::{DEBUG_BAND_COUNT, debug_band_rgb},
    CelestialVoxelField,
    CelestialVoxelRealization, CelestialVoxelRealizationPolicy, VoxelAuthority,
    manifestation::{
        create_voxel_render_material, VoxelMaterializationPresentation,
        VoxelMaterializationRuntime, VoxelPresentationFallbackRetireReady,
        VoxelRenderMaterial,
    },
    worker::{VoxelWorkerLane, VoxelWorkerPool, VoxelWorkerTicket},
};

// Binary presentation is independent from decimal USF interaction Scale.
const MIN_SAMPLE_SPACING_METRES: f64 = 1.0;
const MAX_FINE_SAMPLE_SPACING_METRES: f64 = 2_048.0;
//
// The coarsest binary bricks are allowed to span the semantic body. A small
// conservative margin absorbs canonical relief without inventing a second
// spherical surface representation.
const WHOLE_BODY_ROOT_MARGIN: f64 = 1.125;
const TARGET_CELLS_PER_DISTANCE: f64 = 16.0;
const TARGET_CELLS_PER_CLEARANCE: f64 = 128.0;
const TARGET_PIXELS_PER_BINARY_SAMPLE: f64 = 4.0;
// Inactive dense presentation is a bootstrap fallback, not a LOD layer or history buffer.
const DENSE_FALLBACK_RETENTION_CHUNKS: f32 = 4.0;

//
// Deep local detail is a sparse boundary aperture over persistent coarse body
// ancestry. The budget scales with requested binary depth; this hard ceiling is
// reconstructible protection, not a normal target.
const MIN_SPARSE_FRONTIER_LEAVES: usize = 4_096;
const MAX_SPARSE_FRONTIER_LEAVES: usize = 32_768;
const LEAVES_PER_REQUESTED_LEVEL: usize = 640;
// Each checkpoint remains a complete balanced Transvoxel frontier, but the
// changed region is deliberately small enough to become visible continuously.
//
// Tiny eight-refinement waves forced repeated balance passes and repeated
// whole-frontier materialization while changing almost nothing. Reconstructible
// planning now does substantial work per local balance transaction and publishes
// only a cold bootstrap plus the final balanced replacement frontier.
const MAX_PRIMARY_REFINEMENTS_PER_WAVE: usize = 256;
//
// Planning remains one deep sparse solve. Publication, however, must not jump
// from eight root bricks directly to a multi-thousand-leaf final transaction.
// Cheap hierarchy-only stage synthesis below exposes balanced intermediate
// frontiers without re-running canonical SDF planning.
const MAX_RECORDED_FRONTIER_STAGES: usize = 8;
const CLIPMAP_VALIDITY_AGGREGATES_ACROSS: u32 = 8;
const CLIPMAP_MIN_VALIDITY_SECONDS: f64 = 0.10;
const CLIPMAP_MAX_VALIDITY_SECONDS: f64 = 2.0;
const CLIPMAP_LATENCY_MULTIPLIER: f64 = 4.0;
// Reconstructible clipmap presentation shells are expensive to churn through
// Bevy's entity/asset lifecycle. Keep a bounded hot pool and mutate stable
// Mesh handles in place, matching the dense manifestation runtime.

#[derive(Resource, Default)]
struct CelestialClipmapBandDebugMaterials {
    by_base_and_relative_level:
        HashMap<(Handle<StandardMaterial>, i16), Handle<VoxelRenderMaterial>>,
}

//
// Keep the binary clipmap systems below Bevy's plain function-system parameter
// arity limit without hiding ownership behind globals. These five resources are
// one cohesive presentation-material dependency and can therefore travel as one
// ordinary ECS SystemParam.
#[derive(bevy::ecs::system::SystemParam)]
struct CelestialClipmapMaterialParams<'w> {
    standard_materials: Res<'w, Assets<StandardMaterial>>,
    render_materials: ResMut<'w, Assets<VoxelRenderMaterial>>,
    shader_buffers: ResMut<'w, Assets<ShaderBuffer>>,
    library: Res<'w, ProceduralAssetLibrary>,
    band_materials: ResMut<'w, CelestialClipmapBandDebugMaterials>,
}

// Sixteen adjacent binary LODs traverse one complete hue revolution. The
// palette deliberately advances slowly enough that neighboring resolution
// shells remain easy to distinguish while broad LOD structure reads as one
// continuous sweep rather than a seven-color repeating traffic light.

impl CelestialClipmapBandDebugMaterials {
    fn material_for(
        &mut self,
        standard_materials: &Assets<StandardMaterial>,
        render_materials: &mut Assets<VoxelRenderMaterial>,
        shader_buffers: &mut Assets<ShaderBuffer>,
        debug_grid: &Handle<StandardMaterial>,
        base: &Handle<StandardMaterial>,
        relative_level: i16,
    ) -> Option<Handle<VoxelRenderMaterial>> {
        let diagnostic_band = relative_level.rem_euclid(DEBUG_BAND_COUNT);
        let key = (base.clone(), diagnostic_band);
        if let Some(existing) = self.by_base_and_relative_level.get(&key) {
            return Some(existing.clone());
        }

        let mut tinted = standard_materials.get(base)?.clone();
        // The StandardMaterial tint remains the LOD diagnostic multiplier.
        // If this is the debug-grid marker, the shared voxel shader supplies
        // textureless analytical grid detail in physical metres.
        tinted.base_color = clipmap_band_debug_color(relative_level);
        let grid_uv_metres_per_unit = (base == debug_grid)
            .then_some(DEBUG_GRID_BASE_UV_METRES_PER_UNIT);
        let handle = create_voxel_render_material(
            tinted,
            grid_uv_metres_per_unit,
            shader_buffers,
            render_materials,
        );
        self.by_base_and_relative_level.insert(key, handle.clone());
        Some(handle)
    }
}

fn clipmap_band_debug_color(relative_level: i16) -> Color {
    let [r, g, b] = debug_band_rgb(relative_level);
    Color::srgb(r, g, b)
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CelestialClipmapBlockSpec {
    key: CelestialClipmapBlockKey,
    transition_faces: VoxelTransitionFaces,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CelestialClipmapPlanKey {
    // Observer coordinates are deliberately absent. They are refinement state,
    // not identity of the body-local presentation hierarchy.
    finest_exponent: i16,
    coarsest_exponent: i16,
}

#[derive(Debug)]
struct CelestialClipmapPlan {
    key: CelestialClipmapPlanKey,
    field: CelestialVoxelField,
    /// Predicted actual observer position in body-local SI metres.
    ///
    /// This is the 3D LOD/error/motion anchor. Never project it onto terrain:
    /// altitude and cave/interior motion are real components of observer-space
    /// presentation distance.
    observer_anchor_local: DVec3,
    /// Nearest canonical semantic boundary point to the observer.
    ///
    /// This owns the mandatory surface-refinement branch only. It does not own
    /// observer-distance LOD selection.
    planning_anchor_local: DVec3,
    validity_radius_metres: f64,
    visibility: ClipmapVisibilityDemand,
    generation: u64,
    stages: Vec<Vec<CelestialClipmapBlockSpec>>,
    stage_index: usize,
    desired: Vec<CelestialClipmapBlockSpec>,
    desired_set: HashSet<CelestialClipmapBlockSpec>,
    completed: HashSet<CelestialClipmapBlockSpec>,
    meshful: HashSet<CelestialClipmapBlockSpec>,
    committed_specs: HashSet<CelestialClipmapBlockSpec>,
    committed_generation: Option<u64>,
}


fn seed_clipmap_stage_completion(
    authority: Entity,
    plan: &mut CelestialClipmapPlan,
    active_entities: &HashMap<
        (Entity, CelestialClipmapBlockSpec),
        Entity,
    >,
) {
    plan.completed.clear();
    plan.meshful.clear();

    for &spec in &plan.desired {
        let key = (authority, spec);
        if active_entities.contains_key(&key) {
            plan.completed.insert(spec);
            plan.meshful.insert(spec);
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct ClipmapRefinementCandidate {
    inside_validity: bool,
    distance: f64,
    projected_error: f64,
    refinement_debt: i16,
    key: CelestialClipmapBlockKey,
}

impl PartialEq for ClipmapRefinementCandidate {
    fn eq(&self, other: &Self) -> bool {
        self.inside_validity == other.inside_validity
            && self.refinement_debt == other.refinement_debt
            && self.projected_error.total_cmp(&other.projected_error)
                == std::cmp::Ordering::Equal
            && self.distance.total_cmp(&other.distance)
                == std::cmp::Ordering::Equal
            && self.key == other.key
    }
}

impl Eq for ClipmapRefinementCandidate {}

impl PartialOrd for ClipmapRefinementCandidate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ClipmapRefinementCandidate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.inside_validity
            .cmp(&other.inside_validity)
            .then_with(|| self.refinement_debt.cmp(&other.refinement_debt))
            .then_with(|| self.projected_error.total_cmp(&other.projected_error))
            .then_with(|| other.distance.total_cmp(&self.distance))
            .then_with(|| self.key.resolution.cmp(&other.key.resolution))
            .then_with(|| block_sort_key(other.key).cmp(&block_sort_key(self.key)))
    }
}


fn block_sort_key(
    key: CelestialClipmapBlockKey,
) -> (i16, i32, i32, i32) {
    (
        key.resolution.binary_exponent(),
        key.coord.x,
        key.coord.y,
        key.coord.z,
    )
}

//
// LOD is a genuinely observer-centered 3D shell field. Predictive validity may
// inflate those shells slightly so useful work survives motion/build latency,
// but semantic surface clearance must NOT be subtracted from distance: doing so
// turns sqrt(h^2 + r^2) into sqrt(h^2 + r^2) - h, which makes the terrain
// footprint grow with altitude instead of shrink and eventually disappear.
#[inline]
fn effective_observer_lod_distance_metres(
    distance_metres: f64,
    validity_radius_metres: f64,
) -> f64 {
    (distance_metres - validity_radius_metres.max(0.0)).max(0.0)
}

fn refinement_candidate(
    key: CelestialClipmapBlockKey,
    finest: VoxelPresentationResolution,
    observer_anchor_local: DVec3,
    validity_radius_metres: f64,
) -> Option<ClipmapRefinementCandidate> {
    if key.resolution <= finest {
        return None;
    }

    let distance = block_distance_to_point(key, observer_anchor_local);
    let effective_distance = effective_observer_lod_distance_metres(
        distance,
        validity_radius_metres,
    );
    let target = target_resolution_at_distance(finest, effective_distance);
    let refinement_debt = key
        .resolution
        .binary_exponent()
        .saturating_sub(target.binary_exponent());
    let projected_error =
        key.spacing_metres() / distance.max(key.spacing_metres());

    (refinement_debt > 0).then_some(ClipmapRefinementCandidate {
        inside_validity: distance <= validity_radius_metres.max(0.0),
        distance,
        projected_error,
        refinement_debt,
        key,
    })
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LeafRefinementOutcome {
    Refined,
    Retained,
    Unavailable,
}


fn refine_leaf_indexed(
    input: CelestialClipmapPlanInput,
    leaves: &mut HashSet<CelestialClipmapBlockKey>,
    parent: CelestialClipmapBlockKey,
    surface_cache: &mut CelestialClipmapSurfaceCache,
    classifier: &ClipmapBoundaryClassifier<'_>,
    inserted: &mut Vec<CelestialClipmapBlockKey>,
) -> LeafRefinementOutcome {
    inserted.clear();
    if !leaves.remove(&parent) {
        return LeafRefinementOutcome::Unavailable;
    }

    let Some(children) = parent.children() else {
        leaves.insert(parent);
        return LeafRefinementOutcome::Unavailable;
    };

    for child in children {
        if !input.visibility.demands_block(child) {
            continue;
        }
        if block_contains_local_point(child, input.planning_anchor_local)
            || surface_cache.refinement_intersects(child, classifier) {
            leaves.insert(child);
            inserted.push(child);
        }
    }

    if inserted.is_empty() {
        leaves.insert(parent);
        LeafRefinementOutcome::Retained
    } else {
        LeafRefinementOutcome::Refined
    }
}



#[derive(Debug, Clone, Copy)]
struct LeafRefinementMutation {
    parent: CelestialClipmapBlockKey,
    children: [Option<CelestialClipmapBlockKey>; 8],
}


fn refine_leaf_transactional(
    input: CelestialClipmapPlanInput,
    leaves: &mut HashSet<CelestialClipmapBlockKey>,
    parent: CelestialClipmapBlockKey,
    surface_cache: &mut CelestialClipmapSurfaceCache,
    classifier: &ClipmapBoundaryClassifier<'_>,
    inserted: &mut Vec<CelestialClipmapBlockKey>,
    journal: &mut Vec<LeafRefinementMutation>,
) -> LeafRefinementOutcome {
    let outcome = refine_leaf_indexed(
        input,
        leaves,
        parent,
        surface_cache,
        classifier,
        inserted,
    );

    if outcome == LeafRefinementOutcome::Refined {
        let mut children = [None; 8];
        for (slot, child) in children.iter_mut().zip(inserted.iter().copied()) {
            *slot = Some(child);
        }
        journal.push(LeafRefinementMutation { parent, children });
    }
    outcome
}


fn rollback_leaf_refinements(
    leaves: &mut HashSet<CelestialClipmapBlockKey>,
    journal: &[LeafRefinementMutation],
) {
    for mutation in journal.iter().rev() {
        for child in mutation.children.into_iter().flatten() {
            leaves.remove(&child);
        }
        leaves.insert(mutation.parent);
    }
}

/// Restore the 2:1 invariant only around leaves changed by this wave.

fn balance_leaves_2_to_1_from_seeds(
    input: CelestialClipmapPlanInput,
    leaves: &mut HashSet<CelestialClipmapBlockKey>,
    surface_cache: &mut CelestialClipmapSurfaceCache,
    classifier: &ClipmapBoundaryClassifier<'_>,
    maximum_exponent: i16,
    maximum_leaves: usize,
    seeds: &[CelestialClipmapBlockKey],
    journal: &mut Vec<LeafRefinementMutation>,
    balanced_inserted: &mut Vec<CelestialClipmapBlockKey>,
) -> bool {
    let mut queue =
        VecDeque::<CelestialClipmapBlockKey>::with_capacity(seeds.len().max(64));
    let mut queued =
        HashSet::<CelestialClipmapBlockKey>::with_capacity(
            seeds.len().saturating_mul(2).max(64),
        );

    for &seed in seeds {
        if queued.insert(seed) {
            queue.push_back(seed);
        }
    }

    let mut inserted = Vec::<CelestialClipmapBlockKey>::with_capacity(8);
    balanced_inserted.clear();

    while let Some(key) = queue.pop_front() {
        queued.remove(&key);
        if !leaves.contains(&key) {
            continue;
        }

        for (face, _) in CLIPMAP_FACE_DIRECTIONS {
            let Some(neighbor) = same_or_coarser_face_neighbor(
                leaves,
                key,
                face,
                maximum_exponent,
            ) else {
                continue;
            };
            let difference =
                i32::from(neighbor.resolution.binary_exponent())
                    - i32::from(key.resolution.binary_exponent());
            if difference <= 1 {
                continue;
            }
            if leaves.len().saturating_add(7) > maximum_leaves {
                return false;
            }

            match refine_leaf_transactional(
                input,
                leaves,
                neighbor,
                surface_cache,
                classifier,
                &mut inserted,
                journal,
            ) {
                LeafRefinementOutcome::Refined => {
                    for &child in &inserted {
                        if queued.insert(child) {
                            queue.push_back(child);
                        }
                    }
                    balanced_inserted.extend(inserted.iter().copied());
                }
                LeafRefinementOutcome::Retained => return false,
                LeafRefinementOutcome::Unavailable => return false,
            }

            if queued.insert(key) {
                queue.push_back(key);
            }
            break;
        }
    }

    true
}


#[derive(Resource, Default)]
struct CelestialClipmapRegistry {
    next_generation: u64,
    gpu_admission_cursor: Option<Entity>,
    plans: HashMap<Entity, CelestialClipmapPlan>,
    planner_caches: HashMap<Entity, CelestialClipmapSurfaceCache>,
    // Stable presentation shells + async tickets live in this reconstructible
    // registry. They are runtime bookkeeping, not semantic ECS entities.
    active_entities:
        HashMap<(Entity, CelestialClipmapBlockSpec), Entity>,
    plan_tasks: Vec<CelestialClipmapPlanBuildTask>,
    build_tasks: Vec<CelestialClipmapBuildTask>,

    projection_epoch: u64,
    frontier_epoch: u64,
    projection_pending: Vec<Entity>,
}

impl CelestialClipmapRegistry {
    fn next_generation(&mut self) -> u64 {
        self.next_generation = self.next_generation.wrapping_add(1).max(1);
        self.next_generation
    }

    fn mark_projection_pending(&mut self, entity: Entity) {
        self.projection_pending.push(entity);
        self.projection_epoch = self.projection_epoch.wrapping_add(1).max(1);
    }

    fn mark_frontier_changed(&mut self) {
        self.frontier_epoch = self.frontier_epoch.wrapping_add(1).max(1);
    }
}

#[derive(Resource, Debug, Default, Clone)]
pub(in crate::voxel) struct CelestialClipmapTelemetry {
    plan_requests_total: u64,
    cold_plans_total: u64,
    warm_replans_total: u64,
    visible_blocks: usize,
    binary_primary_authorities: usize,
    visible_binary_levels: usize,
    finest_visible_spacing_metres: Option<f64>,
    coarsest_visible_spacing_metres: Option<f64>,
    boundary_clearance_metres: Option<f64>,
    requested_finest_spacing_metres: Option<f64>,
    planned_finest_spacing_metres: Option<f64>,
    planner_leaf_budget: usize,
    planner_final_leaves: usize,
    planner_budget_saturated: bool,
    dense_fallback_held: usize,
    dense_fallback_retire_ready: usize,
    dense_fallback_forced_retire: usize,
    committed_focus_lag_metres: Option<f64>,
    fresh_plan_accepts_total: u64,
    rolling_plan_accepts_total: u64,
    stale_plan_drops_total: u64,
}

impl CelestialClipmapTelemetry {
    fn record_plan_request(&mut self, warm: bool) {
        self.plan_requests_total = self.plan_requests_total.wrapping_add(1);
        if warm {
            self.warm_replans_total = self.warm_replans_total.wrapping_add(1);
        } else {
            self.cold_plans_total = self.cold_plans_total.wrapping_add(1);
        }
    }

    fn record_visible_frontier(
        &mut self,
        visible_blocks: usize,
        binary_primary_authorities: usize,
        visible_levels: &HashSet<i16>,
        finest_spacing: Option<f64>,
        coarsest_spacing: Option<f64>,
    ) {
        self.visible_blocks = visible_blocks;
        self.binary_primary_authorities = binary_primary_authorities;
        self.visible_binary_levels = visible_levels.len();
        self.finest_visible_spacing_metres = finest_spacing;
        self.coarsest_visible_spacing_metres = coarsest_spacing;
    }

    fn record_plan_quality(
        &mut self,
        input: CelestialClipmapPlanInput,
        stages: &[Vec<CelestialClipmapBlockSpec>],
    ) {
        self.boundary_clearance_metres = Some(input.clearance_metres);
        self.requested_finest_spacing_metres =
            Some(input.finest.sample_spacing_metres());
        self.planned_finest_spacing_metres = stages
            .last()
            .and_then(|stage| {
                stage
                    .iter()
                    .map(|spec| spec.key.spacing_metres())
                    .min_by(f64::total_cmp)
            });
        self.planner_leaf_budget = sparse_frontier_leaf_budget(input);
        self.planner_final_leaves = stages.last().map_or(0, Vec::len);
        self.planner_budget_saturated = self.planner_final_leaves
            >= self.planner_leaf_budget.saturating_sub(7)
            && self
                .planned_finest_spacing_metres
                .zip(self.requested_finest_spacing_metres)
                .is_some_and(|(planned, requested)| planned > requested * 1.001);
    }

    fn record_dense_fallbacks(
        &mut self,
        held: usize,
        retire_ready: usize,
        forced_retire: usize,
    ) {
        self.dense_fallback_held = held;
        self.dense_fallback_retire_ready = retire_ready;
        self.dense_fallback_forced_retire = forced_retire;
    }

    fn record_plan_task_relevance(
        &mut self,
        relevance: ClipmapPlanTaskRelevance,
    ) {
        match relevance {
            ClipmapPlanTaskRelevance::Fresh => {
                self.fresh_plan_accepts_total =
                    self.fresh_plan_accepts_total.saturating_add(1);
            }
            ClipmapPlanTaskRelevance::RollingProgress => {
                self.rolling_plan_accepts_total =
                    self.rolling_plan_accepts_total.saturating_add(1);
            }
            ClipmapPlanTaskRelevance::Stale => {
                self.stale_plan_drops_total =
                    self.stale_plan_drops_total.saturating_add(1);
            }
        }
    }

    fn record_committed_focus_lag(&mut self, lag_metres: Option<f64>) {
        self.committed_focus_lag_metres = lag_metres;
    }


    pub(in crate::voxel) fn summary(&self) -> String {
        format!(
            "plans={} cold={} warm={} visible={} binary_primary={} levels={} visible={}..{}m clearance={}m requested_finest={}m planned_finest={}m leaf_budget={} leaves={} saturated={} fallback_hold={} fallback_retire={} fallback_forced={} focus_lag={}m plan_fresh={} plan_rolling={} plan_drop={}",
            self.plan_requests_total,
            self.cold_plans_total,
            self.warm_replans_total,
            self.visible_blocks,
            self.binary_primary_authorities,
            self.visible_binary_levels,
            self.finest_visible_spacing_metres
                .map_or_else(|| "-".to_string(), |v| format!("{v:.1}")),
            self.coarsest_visible_spacing_metres
                .map_or_else(|| "-".to_string(), |v| format!("{v:.1}")),
            self.boundary_clearance_metres
                .map_or_else(|| "-".to_string(), |v| format!("{v:.1}")),
            self.requested_finest_spacing_metres
                .map_or_else(|| "-".to_string(), |v| format!("{v:.1}")),
            self.planned_finest_spacing_metres
                .map_or_else(|| "-".to_string(), |v| format!("{v:.1}")),
            self.planner_leaf_budget,
            self.planner_final_leaves,
            self.planner_budget_saturated,
            self.dense_fallback_held,
            self.dense_fallback_retire_ready,
            self.dense_fallback_forced_retire,
            self.committed_focus_lag_metres
                .map_or_else(|| "-".to_string(), |v| format!("{v:.1}")),
            self.fresh_plan_accepts_total,
            self.rolling_plan_accepts_total,
            self.stale_plan_drops_total,
        )
    }
}

#[derive(Component, Debug, Clone, Copy)]
struct CelestialClipmapBlock {
    authority: Entity,
    spec: CelestialClipmapBlockSpec,
    active: bool,
    committed: bool,
    projection_ready: bool,
    material_relative_level: i16,
}

struct CelestialClipmapBuildTask {
    authority: Entity,
    generation: u64,
    spec: CelestialClipmapBlockSpec,
    field: CelestialVoxelField,
    entity: Entity,
    build_id: u64,
}

#[derive(Clone, Copy)]
struct PendingGpuAdmission {
    authority: Entity,
    generation: u64,
    spec: CelestialClipmapBlockSpec,
    field: CelestialVoxelField,
    relative_level: i16,
    descriptor: super::gpu::GpuTerrainDescriptor,
}

struct CelestialClipmapPlanBuildTask {
    authority: Entity,
    input: CelestialClipmapPlanInput,
    field: CelestialVoxelField,
    task: VoxelWorkerTicket<CelestialClipmapPlanBuildOutput>,
}

struct CelestialClipmapPlanBuildOutput {
    stages: Option<Vec<Vec<CelestialClipmapBlockSpec>>>,
    surface_cache: CelestialClipmapSurfaceCache,
}

/// Presentation-only local coverage committed by the clipmap.
///
/// This is intentionally *not* [`crate::spatial::UsfCapabilityRealization`].
/// Regional presentation may use it to cull counterfeit coarse surface, but it
/// grants no collision/editing/semantic authority.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(in crate::voxel) struct CelestialClipmapCoverageCell {
    center_local_metres: DVec3,
    half_extent_metres: DVec3,
    sample_spacing_metres: f64,
}

impl CelestialClipmapCoverageCell {
    pub(in crate::voxel) const fn center_local_metres(self) -> DVec3 {
        self.center_local_metres
    }

    pub(in crate::voxel) const fn sample_spacing_metres(self) -> f64 {
        self.sample_spacing_metres
    }

    fn contains_local_point(self, point: DVec3) -> bool {
        let relative = (point - self.center_local_metres).abs();
        relative.x <= self.half_extent_metres.x
            && relative.y <= self.half_extent_metres.y
            && relative.z <= self.half_extent_metres.z
    }

    pub(in crate::voxel) fn inner_radius_metres(self) -> f64 {
        self.half_extent_metres.min_element().max(0.0)
    }

    pub(in crate::voxel) fn outer_radius_metres(self) -> f64 {
        self.half_extent_metres.length()
    }
}

/// Frame-local celestial presentation ownership.
///
/// Dense Surface-Nets terrain is a physical/editable Scale-local working
/// representation. It is allowed to bootstrap presentation only while no
/// coherent binary frontier is available; it is never a geometric LOD child.
///
/// The first committed binary frontier whose complete mesh set projects in
/// the current view takes presentation ownership for the entire authority.
/// Every later quality transition is binary -> binary through the clipmap's
/// own make-before-break frontier transaction. Decimal USF Scale meshes are
/// therefore never stitched, clipped, or locally arbitrated against binary
/// presentation LOD. If binary projection becomes incomplete, dense may
/// reappear only as a whole-authority emergency fallback for that frame.
///
/// This resource is presentation-only and never grants semantic/collision/edit
/// authority.
#[derive(Resource, Debug, Default)]
struct CelestialTerrainPresentationState {
    binary_primary: HashSet<Entity>,
}

impl CelestialTerrainPresentationState {
    fn is_binary_primary(&self, authority: Entity) -> bool {
        self.binary_primary.contains(&authority)
    }

    fn replace_binary_primary_from(&mut self, next: &HashSet<Entity>) {
        if self.binary_primary != *next {
            self.binary_primary.clone_from(next);
        }
    }

    fn clear(&mut self) {
        self.binary_primary.clear();
    }
}


#[derive(Resource, Debug, Default)]
pub(in crate::voxel) struct CelestialClipmapCoverageSnapshot {
    revision: u64,
    by_authority: HashMap<Entity, Vec<CelestialClipmapCoverageCell>>,
}

impl CelestialClipmapCoverageSnapshot {
    pub(in crate::voxel) const fn revision(&self) -> u64 {
        self.revision
    }

    pub(in crate::voxel) fn for_authority(
        &self,
        authority: Entity,
    ) -> &[CelestialClipmapCoverageCell] {
        self.by_authority
            .get(&authority)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    /// Finest actually-visible binary spacing covering one body-local point.
    ///
    /// This is presentation-query metadata only. It never arbitrates a
    /// decimal Scale-local renderer against the binary LOD hierarchy.
    pub(in crate::voxel) fn finest_spacing_covering(
        &self,
        authority: Entity,
        point_local_metres: DVec3,
    ) -> Option<f64> {
        self.for_authority(authority)
            .iter()
            .copied()
            .filter(|cell| cell.contains_local_point(point_local_metres))
            .map(CelestialClipmapCoverageCell::sample_spacing_metres)
            .min_by(f64::total_cmp)
    }

    fn replace_authority(
        &mut self,
        authority: Entity,
        mut coverage: Vec<CelestialClipmapCoverageCell>,
    ) {
        coverage.sort_by(|a, b| {
            a.center_local_metres
                .x
                .total_cmp(&b.center_local_metres.x)
                .then_with(|| {
                    a.center_local_metres
                        .y
                        .total_cmp(&b.center_local_metres.y)
                })
                .then_with(|| {
                    a.center_local_metres
                        .z
                        .total_cmp(&b.center_local_metres.z)
                })
        });

        if self.by_authority.get(&authority) == Some(&coverage) {
            return;
        }

        self.by_authority.insert(authority, coverage);
        self.revision = self.revision.wrapping_add(1).max(1);
    }

    fn replace_authority_from_slice(
        &mut self,
        authority: Entity,
        coverage: &[CelestialClipmapCoverageCell],
    ) {
        if self.by_authority.get(&authority).map(Vec::as_slice)
            == Some(coverage)
        {
            return;
        }
        let target = self.by_authority.entry(authority).or_default();
        target.clear();
        target.extend_from_slice(coverage);
        self.revision = self.revision.wrapping_add(1).max(1);
    }

    fn remove_authority(&mut self, authority: Entity) {
        if self.by_authority.remove(&authority).is_some() {
            self.revision = self.revision.wrapping_add(1).max(1);
        }
    }

    fn retain_authorities(&mut self, live: &HashSet<Entity>) {
        let before = self.by_authority.len();
        self.by_authority.retain(|authority, _| live.contains(authority));
        if self.by_authority.len() != before {
            self.revision = self.revision.wrapping_add(1).max(1);
        }
    }
}

fn visual_target_spacing_metres(
    clearance_metres: f64,
    pixels_per_radian: Option<f32>,
) -> f64 {
    let clearance = clearance_metres.abs().max(MIN_SAMPLE_SPACING_METRES);

    let clearance_target =
        clearance / TARGET_CELLS_PER_CLEARANCE;

    let screen_target = pixels_per_radian
        .map(f64::from)
        .filter(|value| value.is_finite() && *value > 0.0)
        .map(|pixels_per_radian| {
            clearance
                * TARGET_PIXELS_PER_BINARY_SAMPLE
                / pixels_per_radian
        })
        .unwrap_or(clearance_target);

    // Both constraints are upper bounds on acceptable sample spacing. Choose
    // the stricter one; binary quantization later selects at-or-finer.
    clearance_target
        .min(screen_target)
        .clamp(
            MIN_SAMPLE_SPACING_METRES,
            MAX_FINE_SAMPLE_SPACING_METRES,
        )
}

fn target_resolution_at_distance(
    finest: VoxelPresentationResolution,
    distance_metres: f64,
) -> VoxelPresentationResolution {
    let requested_spacing = (distance_metres / TARGET_CELLS_PER_DISTANCE)
        .max(finest.sample_spacing_metres());
    let requested =
        VoxelPresentationResolution::at_most_metres(requested_spacing)
            .unwrap_or(finest);
    requested.max(finest)
}

#[derive(Debug, Clone, Copy)]
struct CelestialClipmapPlanInput {
    key: CelestialClipmapPlanKey,
    observer_anchor_local: DVec3,
    planning_anchor_local: DVec3,
    validity_radius_metres: f64,
    visibility: ClipmapVisibilityDemand,
    clearance_metres: f64,
    finest: VoxelPresentationResolution,
    coarsest: VoxelPresentationResolution,
}

/// Cheap clipmap identity derivation.
///
/// Whole-body topology is semantic-body-owned. Observer position is stored only
/// as a refinement anchor and cannot change the coarsest body representation.
fn derive_plan_input(
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
    let desired_spacing =
        visual_target_spacing_metres(clearance, pixels_per_radian);
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
    let finest = VoxelPresentationResolution::at_most_metres(
        granularity.target_spacing_metres(),
    )?;

    // Stable root resolution is a function of body diameter, not altitude.
    // At least one root-block extent spans the semantic diameter; multiple fixed
    // body-local root cells cover quadrants because the lattice origin is the
    // body's origin/corner boundary rather than an observer-relative origin.
    let body_diameter_metres =
        field.conservative_outer_radius_metres()
            * 2.0
            * WHOLE_BODY_ROOT_MARGIN;
    let coarse_spacing_target =
        (body_diameter_metres / BLOCK_SUBDIVISIONS as f64)
            .max(finest.sample_spacing_metres());
    let coarse_exp_f64 = coarse_spacing_target.log2().ceil();
    if coarse_exp_f64 < f64::from(i16::MIN)
        || coarse_exp_f64 > f64::from(i16::MAX)
    {
        return None;
    }
    let coarsest = VoxelPresentationResolution::new(
        (coarse_exp_f64 as i16).max(finest.binary_exponent()),
    );

    // The anchor is deliberately continuous rather than snapped to a bucket.
    // A live plan decides when this anchor has moved far enough to justify a
    // replacement; boundary crossing by itself is meaningless.
    let fine_extent =
        finest.sample_spacing_metres() * BLOCK_SUBDIVISIONS as f64;
    let validity_radius_metres = granularity
        .validity_radius_metres()
        .max(fine_extent * 4.0);

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

fn plan_requires_refresh(
    plan: &CelestialClipmapPlan,
    input: CelestialClipmapPlanInput,
    field: CelestialVoxelField,
) -> bool {
    if plan.key != input.key || plan.field != field {
        return true;
    }

    let observer_displacement =
        (input.observer_anchor_local - plan.observer_anchor_local).length();
    let surface_displacement =
        (input.planning_anchor_local - plan.planning_anchor_local).length();
    let displacement = observer_displacement.max(surface_displacement);
    let fine_extent =
        input.finest.sample_spacing_metres() * BLOCK_SUBDIVISIONS as f64;
    if plan.visibility.requires_refresh(&input.visibility, fine_extent) {
        return true;
    }

    //
    // Validity radius says how much already-built terrain remains useful; it is
    // not permission for the finest focus to wander across most of that region.
    let hold_radius = (plan.validity_radius_metres
        * FOCUS_REPLAN_VALIDITY_FRACTION)
        .max(fine_extent * FOCUS_REPLAN_MIN_FINE_EXTENTS);
    displacement > hold_radius
}

fn should_schedule_plan_refresh(
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
enum ClipmapPlanTaskRelevance {
    Fresh,
    RollingProgress,
    Stale,
}

fn clipmap_plan_keys_structurally_compatible(
    built: CelestialClipmapPlanKey,
    current: CelestialClipmapPlanKey,
) -> bool {
    // Finest exponent is observer quality state, not semantic hierarchy
    // identity. A behind/finer/coarser result may still be useful intermediate
    // progress. Body-root topology must remain identical.
    built.coarsest_exponent == current.coarsest_exponent
}

fn plan_task_relevance(
    built: CelestialClipmapPlanInput,
    current: CelestialClipmapPlanInput,
    built_field: CelestialVoxelField,
    current_field: CelestialVoxelField,
    committed_anchor_local: Option<DVec3>,
) -> ClipmapPlanTaskRelevance {
    if built_field != current_field
        || !clipmap_plan_keys_structurally_compatible(
            built.key,
            current.key,
        )
        || !built
            .visibility
            .result_still_relevant_to(&current.visibility)
    {
        return ClipmapPlanTaskRelevance::Stale;
    }

    let built_lag =
        (current.observer_anchor_local - built.observer_anchor_local)
            .length();
    if !built_lag.is_finite() {
        return ClipmapPlanTaskRelevance::Stale;
    }

    let built_fine_extent =
        built.finest.sample_spacing_metres()
            * BLOCK_SUBDIVISIONS as f64;
    let current_fine_extent =
        current.finest.sample_spacing_metres()
            * BLOCK_SUBDIVISIONS as f64;

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
    let committed_lag =
        (current.observer_anchor_local - committed_anchor_local).length();
    if !committed_lag.is_finite() {
        return ClipmapPlanTaskRelevance::Stale;
    }

    let progress_margin =
        built_fine_extent.min(current_fine_extent).max(1.0);
    if built_lag + progress_margin < committed_lag {
        ClipmapPlanTaskRelevance::RollingProgress
    } else {
        ClipmapPlanTaskRelevance::Stale
    }
}

fn local_frontier_spacing<'a>(
    specs: impl IntoIterator<Item = &'a CelestialClipmapBlockSpec>,
    planning_anchor_local: DVec3,
    probe_radius_metres: f64,
) -> Option<f64> {
    specs
        .into_iter()
        .filter(|spec| {
            block_distance_to_point(spec.key, planning_anchor_local)
                <= probe_radius_metres
        })
        .map(|spec| spec.key.spacing_metres())
        .min_by(f64::total_cmp)
}

fn initial_stage_for_plan(
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
    let fine_extent =
        input.finest.sample_spacing_metres() * BLOCK_SUBDIVISIONS as f64;
    let probe_radius =
        input.validity_radius_metres.max(fine_extent * 4.0);

    let existing = local_frontier_spacing(
        committed_specs.iter(),
        input.planning_anchor_local,
        probe_radius,
    );

    // If the previous frontier does not cover the new focus, require a useful
    // bootstrap (within 8x target) before swapping. This is still much smaller
    // than a final-frontier global barrier.
    let maximum_acceptable = existing.unwrap_or(
        input.finest.sample_spacing_metres() * 8.0,
    );

    stages
        .iter()
        .position(|stage| {
            local_frontier_spacing(
                stage.iter(),
                input.planning_anchor_local,
                probe_radius,
            )
            .is_some_and(|spacing| {
                spacing <= maximum_acceptable * 1.001
            })
        })
        .or(Some(stages.len() - 1))
}

fn body_root_coordinate_bounds(
    field: CelestialVoxelField,
    coarsest: VoxelPresentationResolution,
) -> Option<(i32, i32)> {
    let extent = coarsest.sample_spacing_metres() * BLOCK_SUBDIVISIONS as f64;
    if !extent.is_finite() || extent <= 0.0 {
        return None;
    }
    let radius =
        field.conservative_outer_radius_metres() * WHOLE_BODY_ROOT_MARGIN;
    let low = (-radius / extent).floor();
    let high = (radius / extent).floor();
    if low < f64::from(i32::MIN)
        || high > f64::from(i32::MAX)
        || low > high
    {
        return None;
    }
    Some((low as i32, high as i32))
}


/// Converts one balanced leaf frontier into deterministic build specs.
///
/// Priority ordering only needs monotonic distance, so avoid square roots here.
/// More importantly, this is no longer called from the inner refinement loop.
fn specs_for_frontier(
    leaves: &HashSet<CelestialClipmapBlockKey>,
    planning_anchor_local: DVec3,
) -> Vec<CelestialClipmapBlockSpec> {
    let mut transitions = transition_faces_for_frontier(leaves);
    let mut ordered = leaves
        .iter()
        .copied()
        .map(|key| {
            let distance2 = block_distance_squared_to_point(
                key,
                planning_anchor_local,
            );
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
fn frontier_capped_at_resolution(
    final_leaves: &HashSet<CelestialClipmapBlockKey>,
    cap: VoxelPresentationResolution,
) -> HashSet<CelestialClipmapBlockKey> {
    let mut staged =
        HashSet::<CelestialClipmapBlockKey>::with_capacity(
            final_leaves.len(),
        );

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

fn append_progressive_frontier_stages(
    stages: &mut Vec<Vec<CelestialClipmapBlockSpec>>,
    final_leaves: &HashSet<CelestialClipmapBlockKey>,
    final_stage: &[CelestialClipmapBlockSpec],
    coarsest: VoxelPresentationResolution,
    finest: VoxelPresentationResolution,
    ordering_anchor_local: DVec3,
) {
    let stride = RECORDED_FRONTIER_BINARY_LEVEL_STRIDE.max(1);
    let mut exponent = coarsest
        .binary_exponent()
        .saturating_sub(stride);

    while exponent > finest.binary_exponent()
        && stages.len().saturating_add(1)
            < MAX_RECORDED_FRONTIER_STAGES
    {
        let cap = VoxelPresentationResolution::new(exponent);
        let staged_leaves =
            frontier_capped_at_resolution(final_leaves, cap);
        let staged_specs = {
            let _span = bevy::log::info_span!(
                "voxel.worker.presentation_planning.progressive_specs"
            )
            .entered();
            specs_for_frontier(
                &staged_leaves,
                ordering_anchor_local,
            )
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

fn sparse_frontier_leaf_budget(input: CelestialClipmapPlanInput) -> usize {
    let requested_levels =
        i32::from(input.coarsest.binary_exponent())
            .saturating_sub(i32::from(input.finest.binary_exponent()))
            .max(0) as usize;

    let fine_extent =
        input.finest.sample_spacing_metres() * BLOCK_SUBDIVISIONS as f64;
    let validity_blocks = if fine_extent.is_finite() && fine_extent > 0.0 {
        (input.validity_radius_metres / fine_extent)
            .ceil()
            .clamp(1.0, 64.0) as usize
    } else {
        1
    };

    MIN_SPARSE_FRONTIER_LEAVES
        .saturating_add(
            requested_levels.saturating_mul(LEAVES_PER_REQUESTED_LEVEL),
        )
        .saturating_add(validity_blocks.saturating_mul(64))
        .clamp(
            MIN_SPARSE_FRONTIER_LEAVES,
            MAX_SPARSE_FRONTIER_LEAVES,
        )
}

const RECORDED_FRONTIER_BINARY_LEVEL_STRIDE: i16 = 4;

fn push_refinement_candidates(
    candidates: &mut BinaryHeap<ClipmapRefinementCandidate>,
    keys: impl IntoIterator<Item = CelestialClipmapBlockKey>,
    finest: VoxelPresentationResolution,
    observer_anchor_local: DVec3,
    validity_radius_metres: f64,
) {
    for key in keys {
        if let Some(candidate) = refinement_candidate(
            key,
            finest,
            observer_anchor_local,
            validity_radius_metres,
        ) {
            candidates.push(candidate);
        }
    }
}

fn should_record_frontier_checkpoint(
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
    gained >= RECORDED_FRONTIER_BINARY_LEVEL_STRIDE
        || current <= finest
}

/// Expensive sparse staged-frontier construction.
///
/// The coarse whole-body ancestry is conservative. Fine detail is a sparse
/// boundary aperture: the planner refines only blocks with actual local SDF
/// boundary evidence, balances the local transition ring, and records a
/// transaction stage only when local LOD depth improves.

fn build_plan(
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
        input.observer_anchor_local.is_finite()
            && input.planning_anchor_local.is_finite(),
        "clipmap planner anchors must be finite"
    );

    surface_cache.begin_plan(field);
    let result = build_plan_inner(
        field,
        input,
        surface_cache,
        warm_replan,
    );
    surface_cache.finish_plan();
    result
}

fn build_plan_inner(
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
    let bootstrap = (!warm_replan).then(|| {
        specs_for_frontier(&leaves, input.observer_anchor_local)
    });

    refine_plan_frontier(
        &mut leaves,
        input,
        maximum_leaves,
        surface_cache,
        &classifier,
    );

    let final_stage =
        specs_for_frontier(&leaves, input.observer_anchor_local);
    if final_stage.is_empty() {
        return None;
    }

    Some(build_publication_stages(
        bootstrap,
        &leaves,
        final_stage,
        input,
    ))
}

fn planner_root_leaves(
    field: CelestialVoxelField,
    coarsest: VoxelPresentationResolution,
    maximum_leaves: usize,
    surface_cache: &mut CelestialClipmapSurfaceCache,
    classifier: &ClipmapBoundaryClassifier<'_>,
    visibility: ClipmapVisibilityDemand,
) -> Option<HashSet<CelestialClipmapBlockKey>> {
    let _span =
        bevy::log::info_span!("voxel.worker.presentation_planning.roots")
            .entered();
    let (root_low, root_high) =
        body_root_coordinate_bounds(field, coarsest)?;
    let mut leaves =
        HashSet::<CelestialClipmapBlockKey>::with_capacity(
            maximum_leaves.min(8_192),
        );

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

fn surface_focus_resolution(
    input: CelestialClipmapPlanInput,
) -> VoxelPresentationResolution {
    let distance =
        (input.observer_anchor_local - input.planning_anchor_local).length();
    target_resolution_at_distance(
        input.finest,
        effective_observer_lod_distance_metres(
            distance,
            input.validity_radius_metres,
        ),
    )
    .min(input.coarsest)
}

fn refine_plan_frontier(
    leaves: &mut HashSet<CelestialClipmapBlockKey>,
    input: CelestialClipmapPlanInput,
    maximum_leaves: usize,
    surface_cache: &mut CelestialClipmapSurfaceCache,
    classifier: &ClipmapBoundaryClassifier<'_>,
) {
    let _span =
        bevy::log::info_span!("voxel.worker.presentation_planning.refine")
            .entered();
    let focus_resolution = surface_focus_resolution(input);
    let mut candidates =
        BinaryHeap::<ClipmapRefinementCandidate>::with_capacity(
            leaves.len().min(8_192),
        );
    push_refinement_candidates(
        &mut candidates,
        leaves.iter().copied(),
        input.finest,
        input.observer_anchor_local,
        input.validity_radius_metres,
    );

    let mut journal =
        Vec::<LeafRefinementMutation>::with_capacity(
            MAX_PRIMARY_REFINEMENTS_PER_WAVE * 2,
        );
    let mut inserted = Vec::<CelestialClipmapBlockKey>::with_capacity(8);
    let mut balance_seeds =
        Vec::<CelestialClipmapBlockKey>::with_capacity(
            MAX_PRIMARY_REFINEMENTS_PER_WAVE * 8,
        );
    let mut balanced_inserted =
        Vec::<CelestialClipmapBlockKey>::with_capacity(
            MAX_PRIMARY_REFINEMENTS_PER_WAVE * 8,
        );

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

fn build_publication_stages(
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

    assert!(!stages.is_empty(), "planner must publish at least one stage");
    assert!(
        stages.len() <= MAX_RECORDED_FRONTIER_STAGES,
        "planner publication stage cap must be respected"
    );
    stages
}


fn park_clipmap_entity(
    commands: &mut Commands,
    entity: Entity,
    block: &mut CelestialClipmapBlock,
    visibility: &mut Visibility,
    registry: &mut CelestialClipmapRegistry,
) {
    if !block.active {
        return;
    }

    let affected_visible_frontier = block.committed;
    registry.active_entities.remove(&(block.authority, block.spec));
    block.active = false;
    block.committed = false;
    block.projection_ready = false;
    block.material_relative_level = i16::MIN;
    *visibility = Visibility::Hidden;

    //
    // GPU-native clipmap shells own substantial persistent MeshAllocator
    // ranges. Retaining retired shells kept those ranges alive and allowed the
    // old 4096-entity pool to consume gigabytes. Despawn instead: once the
    // Mesh3d handle drops, Bevy propagates AssetEvent::Unused to RenderAssets,
    // which releases the allocator ranges.
    commands.entity(entity).despawn();

    if affected_visible_frontier {
        registry.mark_frontier_changed();
    }
}


fn spawn_gpu_clipmap_entity(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    admission: PendingGpuAdmission,
    presentation_material: Handle<VoxelRenderMaterial>,
    build_id: u64,
) -> Entity {
    let extent = admission.spec.key.extent_metres() as f32;
    let transition_face_count =
        admission.spec.transition_faces.bits().count_ones();

    assert!(
        extent.is_finite() && extent > 0.0,
        "GPU clipmap extent must be finite and positive"
    );
    assert!(
        transition_face_count <= 6,
        "GPU clipmap transitions must describe at most six faces"
    );

    let bounds = Aabb::from_min_max(
        Vec3::ZERO,
        Vec3::splat(extent),
    );
    let mesh = meshes.add(allocation_mesh(transition_face_count));

    commands
        .spawn((
            Name::new("Celestial Binary Clipmap"),
            CelestialClipmapBlock {
                authority: admission.authority,
                spec: admission.spec,
                active: true,
                committed: false,
                projection_ready: false,
                material_relative_level: admission.relative_level,
            },
            UsfPresentationProjectionOf(admission.authority),
            Mesh3d(mesh.clone()),
            MeshMaterial3d(presentation_material),
            bounds,
            NoAutoAabb,
            GpuTerrainBlock::new(
                mesh,
                build_id,
                admission.descriptor,
            ),
            Transform::IDENTITY,
            RenderLayers::layer(USF_PRESENTATION_LAYER),
            NotShadowCaster,
            NotShadowReceiver,
            Visibility::Hidden,
        ))
        .id()
}

fn sync_celestial_clipmap_realizations(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut material_params: CelestialClipmapMaterialParams,
    views: Res<UsfViewDemandSnapshot>,
    workers: Res<VoxelWorkerPool>,
    authorities: Query<(
        Entity,
        Option<&Name>,
        &UsfPosition,
        &UsfSemanticFrame,
        &CelestialVoxelField,
        &VoxelAuthority,
        &CelestialVoxelRealizationPolicy,
    )>,
    mut blocks: Query<(
        Entity,
        &mut CelestialClipmapBlock,
        &mut Visibility,
    )>,
    mut registry: ResMut<CelestialClipmapRegistry>,
    mut telemetry: ResMut<CelestialClipmapTelemetry>,
    mut frame_budget: ResMut<ReconstructibleFrameBudget>,
    mut gpu_runtime: ResMut<GpuTerrainRuntime>,
) {
    let Some(view) = views.iter().next() else {
        return;
    };

    let observer_speed = view.velocity_metres_per_second().length();
    let expected_build_seconds =
        workers.estimated_latency_seconds(VoxelWorkerLane::PresentationPlanning);

    let mut live_authorities = HashSet::<Entity>::new();
    let mut current_inputs =
        HashMap::<Entity, (CelestialClipmapPlanInput, CelestialVoxelField)>::new();

    {
        let _span = bevy::log::info_span!("celestial_clipmap.plan_identity").entered();
        for (
            authority,
            _name,
            body_origin,
            body_frame,
            field,
            voxel_authority,
            _policy,
        ) in &authorities
        {
            if !voxel_authority.is_empty() {
                continue;
            }
            let Ok(observer_local) = body_frame.world_to_local_metres(
                body_origin,
                &view.anchor(),
                SpatialScale::ZERO,
                f64::MAX,
            ) else {
                continue;
            };

            //
            // Aim reconstructible presentation work where the observer is
            // expected to be when planning+mesh work drains, not at the point
            // already being left behind.
            let prediction_seconds =
                (expected_build_seconds * CLIPMAP_LATENCY_MULTIPLIER)
                    .clamp(0.0, CLIPMAP_MAX_VALIDITY_SECONDS);
            let local_velocity_metres_per_second =
                body_frame.orientation().conjugate()
                    * view.velocity_metres_per_second();
            let predicted_observer_local =
                observer_local
                    + local_velocity_metres_per_second
                        * prediction_seconds;

            let visibility = ClipmapVisibilityDemand::new(
                *field,
                observer_local,
                predicted_observer_local,
                body_frame,
                view,
            );
            let Some(input) = derive_plan_input(
                *field,
                predicted_observer_local,
                view.pixels_per_radian_for_presentation_resolution(),
                observer_speed,
                expected_build_seconds,
                visibility,
            ) else {
                continue;
            };
            live_authorities.insert(authority);
            current_inputs.insert(authority, (input, *field));
        }
    }

    let mut planning_authorities = HashSet::<Entity>::new();
    let mut plans_changed = false;

    let _poll_plan_tasks_span =
        bevy::log::info_span!("celestial_clipmap.poll_plans").entered();
    let mut pending_plan_tasks = Vec::with_capacity(registry.plan_tasks.len());
    for mut build in std::mem::take(&mut registry.plan_tasks) {
        let Some(&(current_input, current_field)) =
            current_inputs.get(&build.authority)
        else {
            continue;
        };
        planning_authorities.insert(build.authority);

        let output = match build.task.try_take() {
            Ok(None) => {
                pending_plan_tasks.push(build);
                continue;
            }
            Ok(Some(output)) => output,
            Err(failure) => {
                planning_authorities.remove(&build.authority);
                warn!(?failure, authority = ?build.authority, "celestial clipmap plan worker failed");
                continue;
            }
        };
        planning_authorities.remove(&build.authority);
        registry
            .planner_caches
            .insert(build.authority, output.surface_cache);

        let committed_anchor_local = registry
            .plans
            .get(&build.authority)
            .map(|plan| plan.observer_anchor_local);
        let relevance = plan_task_relevance(
            build.input,
            current_input,
            build.field,
            current_field,
            committed_anchor_local,
        );
        telemetry.record_plan_task_relevance(relevance);
        if relevance == ClipmapPlanTaskRelevance::Stale {
            continue;
        }

        let Some(stages) = output.stages else {
            continue;
        };

        let (committed_generation, committed_specs) =
            registry
                .plans
                .get(&build.authority)
                .map(|plan| {
                    (
                        plan.committed_generation,
                        plan.committed_specs.clone(),
                    )
                })
                .unwrap_or((
                    None,
                    HashSet::new(),
                ));

        let Some(stage_index) = initial_stage_for_plan(
            &stages,
            &committed_specs,
            build.input,
        ) else {
            continue;
        };
        let desired = stages[stage_index].clone();
        if desired.is_empty() {
            continue;
        }

        telemetry.record_plan_quality(build.input, &stages);
        let actual_finest_spacing_metres = stages
            .last()
            .and_then(|stage| {
                stage
                    .iter()
                    .map(|spec| spec.key.spacing_metres())
                    .min_by(f64::total_cmp)
            });

        let current_target_lag_metres =
            (current_input.planning_anchor_local
                - build.input.planning_anchor_local)
                .length();

        trace!(
            authority = ?build.authority,
            ?relevance,
            current_target_lag_metres,
            canonical_clearance_metres = build.input.clearance_metres,
            requested_finest_spacing_metres = build.input.finest.sample_spacing_metres(),
            actual_finest_spacing_metres = ?actual_finest_spacing_metres,
            coarsest_spacing_metres = build.input.coarsest.sample_spacing_metres(),
            refinement_stages = stages.len(),
            initial_blocks = desired.len(),
            final_blocks = stages.last().map_or(0, Vec::len),
            requested_target_reached =
                actual_finest_spacing_metres.is_some_and(|spacing| {
                    spacing
                        <= build.input.finest.sample_spacing_metres() * 1.001
                }),
            "celestial clipmap staged plan ready"
        );

        let generation = registry.next_generation();
        registry.plans.insert(
            build.authority,
            CelestialClipmapPlan {
                key: build.input.key,
                field: build.field,
                observer_anchor_local: build.input.observer_anchor_local,
                planning_anchor_local: build.input.planning_anchor_local,
                validity_radius_metres: build.input.validity_radius_metres,
                visibility: build.input.visibility,
                generation,
                stages,
                stage_index,
                desired_set: desired.iter().copied().collect(),
                desired,
                completed: HashSet::new(),
                meshful: HashSet::new(),
                committed_specs,
                committed_generation,
            },
        );

        {
            let CelestialClipmapRegistry {
                plans,
                active_entities,
                ..
            } = &mut *registry;
            if let Some(plan) = plans.get_mut(&build.authority) {
                seed_clipmap_stage_completion(
                    build.authority,
                    plan,
                    active_entities,
                );
            }
        }
        registry.mark_frontier_changed();
        plans_changed = true;
    }

    registry.plan_tasks = pending_plan_tasks;
    drop(_poll_plan_tasks_span);

    let _schedule_plan_span =
        bevy::log::info_span!("celestial_clipmap.schedule_plans").entered();
    let mut planning_slots =
        workers.available_slots(VoxelWorkerLane::PresentationPlanning);
    let committed_focus_lag = current_inputs
        .iter()
        .filter_map(|(authority, (input, _))| {
            registry.plans.get(authority).map(|plan| {
                (
                    input.observer_anchor_local
                        - plan.observer_anchor_local,
                )
                    .0
                    .length()
            })
        })
        .filter(|lag| lag.is_finite())
        .max_by(f64::total_cmp);
    telemetry.record_committed_focus_lag(committed_focus_lag);

    for (&authority, &(input, field)) in &current_inputs {
        let existing = registry.plans.get(&authority);
        let replace = existing
            .is_none_or(|plan| {
                should_schedule_plan_refresh(plan, input, field)
            });
        if !replace
            || planning_authorities.contains(&authority)
            || planning_slots == 0
        {
            continue;
        }
        let warm_replan = existing
            .is_some_and(|plan| !plan.committed_specs.is_empty());

        let Some(work_token) =
            frame_budget.begin(ReconstructibleWorkClass::Maintenance)
        else {
            break;
        };

        let mut surface_cache = registry
            .planner_caches
            .remove(&authority)
            .unwrap_or_default();

        let Some(task) = workers.try_submit(
            VoxelWorkerLane::PresentationPlanning,
            move || {
                let stages = build_plan(
                    field,
                    input,
                    &mut surface_cache,
                    warm_replan,
                );
                CelestialClipmapPlanBuildOutput {
                    stages,
                    surface_cache,
                }
            },
        ) else {
            frame_budget.finish(work_token);
            break;
        };

        telemetry.record_plan_request(warm_replan);
        registry.plan_tasks.push(CelestialClipmapPlanBuildTask {
            authority,
            input,
            field,
            task,
        });
        planning_authorities.insert(authority);
        planning_slots -= 1;
        frame_budget.finish(work_token);
    }
    drop(_schedule_plan_span);

    let plan_count_before_retain = registry.plans.len();
    registry
        .plans
        .retain(|authority, _| live_authorities.contains(authority));
    let plans_removed = registry.plans.len() != plan_count_before_retain;
    registry
        .planner_caches
        .retain(|authority, _| live_authorities.contains(authority));
    if plans_removed {
        let dead_entities = registry
            .active_entities
            .iter()
            .filter_map(|((authority, _), &entity)| {
                (!live_authorities.contains(authority)).then_some(entity)
            })
            .collect::<Vec<_>>();
        for entity in dead_entities {
            if let Ok((_, mut block, mut visibility)) =
                blocks.get_mut(entity)
            {
                park_clipmap_entity(
                    &mut commands,
                    entity,
                    &mut block,
                    &mut visibility,
                    &mut registry,
                );
            }
        }
    }

    let no_plan_tasks = registry.plan_tasks.is_empty();
    let no_build_tasks = registry.build_tasks.is_empty();
    let plans_settled = registry
        .plans
        .values()
        .all(|plan| plan.committed_generation == Some(plan.generation));
    if !plans_changed
        && !plans_removed
        && no_plan_tasks
        && no_build_tasks
        && plans_settled
    {
        return;
    }

    // Current-stage completion is seeded exactly once when a stage
    // becomes active and incrementally updated by worker results.


    //
    // CPU planning decides which semantic blocks are required. Binary density,
    // Transvoxel extraction and terrain-buffer writes are GPU-owned.
    //
    // RenderWorld returns only a tiny dispatch acknowledgement. This is not
    // density/geometry readback; it preserves the make-before-break projection
    // barrier before the previous committed frontier may retire.
    let completed_gpu_builds = gpu_runtime
        .drain_completed()
        .into_iter()
        .collect::<HashSet<_>>();
    let mut inflight =
        HashSet::<(Entity, CelestialClipmapBlockSpec)>::new();

    {
        let _span =
            bevy::log::info_span!("celestial_clipmap.poll_gpu_builds").entered();
        let mut pending_builds =
            Vec::with_capacity(registry.build_tasks.len());

        for build in std::mem::take(&mut registry.build_tasks) {
            let key = (build.authority, build.spec);

            let valid = registry
                .plans
                .get(&build.authority)
                .is_some_and(|plan| {
                    build.generation == plan.generation
                        && plan.desired_set.contains(&build.spec)
                        && build.field == plan.field
                });

            if !valid {
                if let Ok((_, mut block, mut visibility)) =
                    blocks.get_mut(build.entity)
                {
                    commands
                        .entity(build.entity)
                        .remove::<GpuTerrainBlock>();
                    park_clipmap_entity(
                        &mut commands,
                        build.entity,
                        &mut block,
                        &mut visibility,
                        &mut registry,
                    );
                }
                continue;
            }

            if !completed_gpu_builds.contains(&build.build_id) {
                inflight.insert(key);
                pending_builds.push(build);
                continue;
            }

            if blocks.get_mut(build.entity).is_err() {
                continue;
            }

            commands
                .entity(build.entity)
                .remove::<GpuTerrainBlock>();
            registry.active_entities.insert(key, build.entity);
            registry.mark_projection_pending(build.entity);

            if let Some(plan) = registry.plans.get_mut(&build.authority) {
                plan.completed.insert(build.spec);
                plan.meshful.insert(build.spec);
            }
        }

        registry.build_tasks = pending_builds;
    }

    let mut admissions = Vec::<PendingGpuAdmission>::new();

    {
        let _span =
            bevy::log::info_span!("celestial_clipmap.schedule_gpu_builds").entered();

        // Descriptor/publication admissions remain bounded even though there
        // are no CPU mesh jobs anymore, preventing allocator/entity bursts.
        const MAX_GPU_BUILDS_IN_FLIGHT: usize = 32;
        const MAX_GPU_ADMISSIONS_PER_FRAME: usize = 8;
        let mut admitted = 0usize;
        let mut next_admission_cursor = registry.gpu_admission_cursor;

        // Limited admission must not depend on HashMap iteration order. Resume
        // after the last successful authority even if plans changed meanwhile.
        let mut authorities = registry.plans.keys().copied().collect::<Vec<_>>();
        authorities.sort_unstable_by_key(|authority| authority.to_bits());
        if let Some(cursor) = registry.gpu_admission_cursor {
            let start = authorities.partition_point(|authority| {
                authority.to_bits() <= cursor.to_bits()
            });
            let count = authorities.len();
            if count != 0 {
                authorities.rotate_left(start % count);
            }
        }

        'authorities: for authority in authorities {
            let plan = &registry.plans[&authority];
            for &spec in &plan.desired {
                let key = (authority, spec);
                if plan.completed.contains(&spec)
                    || registry.active_entities.contains_key(&key)
                    || inflight.contains(&key)
                {
                    continue;
                }

                if inflight.len().saturating_add(admissions.len())
                    >= MAX_GPU_BUILDS_IN_FLIGHT
                    || admitted >= MAX_GPU_ADMISSIONS_PER_FRAME
                {
                    break 'authorities;
                }

                let Some(work_token) =
                    frame_budget.begin(ReconstructibleWorkClass::Maintenance)
                else {
                    break 'authorities;
                };

                let Some(descriptor) = descriptor_for_block(
                    plan.field,
                    spec.key.origin_local_metres(),
                    spec.key.extent_metres(),
                    spec.key.spacing_metres(),
                    spec.transition_faces.bits(),
                ) else {
                    frame_budget.finish(work_token);
                    continue;
                };

                admissions.push(PendingGpuAdmission {
                    authority,
                    generation: plan.generation,
                    spec,
                    field: plan.field,
                    relative_level: spec
                        .key
                        .resolution
                        .binary_exponent()
                        .saturating_sub(plan.key.finest_exponent),
                    descriptor,
                });
                admitted += 1;
                next_admission_cursor = Some(authority);
                frame_budget.finish(work_token);
            }
        }
        registry.gpu_admission_cursor = next_admission_cursor;
    }

    let mut scheduled_builds = Vec::<CelestialClipmapBuildTask>::new();

    for admission in admissions {
        let still_valid = registry
            .plans
            .get(&admission.authority)
            .is_some_and(|plan| {
                admission.generation == plan.generation
                    && plan.desired_set.contains(&admission.spec)
                    && admission.field == plan.field
            });
        if !still_valid {
            continue;
        }

        let Ok((_, _name, _, _, _, _, policy)) =
            authorities.get(admission.authority)
        else {
            continue;
        };

        let standard_materials = &material_params.standard_materials;
        let debug_grid = &material_params.library.debug_grid;
        let render_materials = &mut material_params.render_materials;
        let shader_buffers = &mut material_params.shader_buffers;
        let band_materials = &mut material_params.band_materials;

        let Some(presentation_material) = band_materials.material_for(
            standard_materials,
            render_materials,
            shader_buffers,
            debug_grid,
            policy.presentation_material(),
            admission.relative_level,
        ) else {
            continue;
        };

        let build_id = gpu_runtime.next_build_id();
        let entity = spawn_gpu_clipmap_entity(
            &mut commands,
            &mut meshes,
            admission,
            presentation_material,
            build_id,
        );

        let key = (admission.authority, admission.spec);
        inflight.insert(key);
        scheduled_builds.push(CelestialClipmapBuildTask {
            authority: admission.authority,
            generation: admission.generation,
            spec: admission.spec,
            field: admission.field,
            entity,
            build_id,
        });
    }

    registry.build_tasks.extend(scheduled_builds);

    {
        let _span = bevy::log::info_span!("celestial_clipmap.commit").entered();
        let mut retired_entities = Vec::<Entity>::new();

        let CelestialClipmapRegistry {
            plans, active_entities, ..
        } = &mut *registry;
        let mut committed_any_frontier = false;
        for (&authority, plan) in plans {
            if plan.committed_generation == Some(plan.generation) {
                continue;
            }
            if !plan
                .desired
                .iter()
                .all(|spec| plan.completed.contains(spec))
            {
                continue;
            }

            //
            // Already-committed unchanged specs are not a dependency of this
            // transaction. Only replacement/new meshful specs must prove that
            // they can project before make-before-break swaps the frontier.
            if !plan
                .meshful
                .iter()
                .filter(|spec| !plan.committed_specs.contains(*spec))
                .all(|spec| {
                    active_entities
                        .get(&(authority, *spec))
                        .and_then(|entity| blocks.get(*entity).ok())
                        .is_some_and(|(_, block, _)| {
                            block.active && block.projection_ready
                        })
                })
            {
                continue;
            }

            let added_blocks = plan
                .desired_set
                .difference(&plan.committed_specs)
                .count();
            let retired_blocks = plan
                .committed_specs
                .difference(&plan.desired_set)
                .count();

            for ((block_authority, _), &entity) in active_entities.iter() {
                if *block_authority != authority {
                    continue;
                }
                let Ok((_, mut block, _)) = blocks.get_mut(entity)
                else {
                    continue;
                };
                if !block.active {
                    continue;
                }
                if plan.desired_set.contains(&block.spec)
                {
                    block.committed = true;
                } else {
                    retired_entities.push(entity);
                }
            }

            std::mem::swap(
                &mut plan.committed_specs,
                &mut plan.desired_set,
            );

            let committed_generation = plan.generation;
            committed_any_frontier = true;

            if plan.stage_index + 1 < plan.stages.len() {
                plan.committed_generation = Some(committed_generation);
                plan.stage_index += 1;
                plan.generation =
                    plan.generation.wrapping_add(1).max(1);
                plan.desired.clone_from(&plan.stages[plan.stage_index]);
                plan.desired_set.clear();
                plan.desired_set
                    .extend(plan.desired.iter().copied());
                seed_clipmap_stage_completion(
                    authority,
                    plan,
                    active_entities,
                );

                trace!(
                    authority = ?authority,
                    committed_stage = plan.stage_index,
                    total_stages = plan.stages.len(),
                    committed_blocks = plan.committed_specs.len(),
                    transaction_added_blocks = added_blocks,
                    transaction_retired_blocks = retired_blocks,
                    next_blocks = plan.desired.len(),
                    "clipmap refinement frontier committed; arming next stage"
                );
            } else {
                plan.committed_generation = Some(committed_generation);
                plan.desired_set.clear();
                trace!(
                    authority = ?authority,
                    committed_stage = plan.stage_index + 1,
                    total_stages = plan.stages.len(),
                    committed_blocks = plan.committed_specs.len(),
                    transaction_added_blocks = added_blocks,
                    transaction_retired_blocks = retired_blocks,
                    "clipmap final refinement frontier committed"
                );
            }
        }

        // End the disjoint field borrows before touching the pool itself.
        let _ = plans;
        let _ = active_entities;
        if committed_any_frontier {
            registry.mark_frontier_changed();
        }
        for entity in retired_entities {
            if let Ok((_, mut block, mut visibility)) =
                blocks.get_mut(entity)
            {
                park_clipmap_entity(
                    &mut commands,
                    entity,
                    &mut block,
                    &mut visibility,
                    &mut registry,
                );
            }
        }
    }

}


fn binary_frontier_projection_complete(
    expected_meshes: usize,
    projected_meshes: usize,
) -> bool {
    expected_meshes > 0 && projected_meshes == expected_meshes
}

#[derive(Debug, Clone, Copy)]
struct ClipmapProjectedAuthorityFrame {
    relative_metres: DVec3,
    orientation: bevy::math::DQuat,
    rotation: Quat,
}

#[derive(Debug, Clone, PartialEq)]
struct ClipmapAuthorityProjectionStamp {
    origin: UsfPosition,
    orientation: bevy::math::DQuat,
    presentation_material: Handle<StandardMaterial>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct ClipmapViewProjectionStamp {
    anchor: UsfPosition,
    metre_to_view_f64: f64,
    projection_eye: DVec3,
    presentation_origin: Vec3,
}

#[derive(Default)]
struct CelestialClipmapTransformScratch {
    frames: HashMap<Entity, ClipmapProjectedAuthorityFrame>,
    counts: HashMap<Entity, (usize, usize)>,
    binary_primary: HashSet<Entity>,
    projected_committed: Vec<Entity>,
    visible_levels: HashSet<i16>,
    visible_by_authority:
        HashMap<Entity, Vec<CelestialClipmapCoverageCell>>,
    live_authorities: HashSet<Entity>,
    authority_stamps: HashMap<Entity, ClipmapAuthorityProjectionStamp>,
    next_authority_stamps: HashMap<Entity, ClipmapAuthorityProjectionStamp>,
    last_view_stamp: Option<ClipmapViewProjectionStamp>,
    last_projection_epoch: u64,
    last_frontier_epoch: u64,
    initialized: bool,
    pending_entities: Vec<Entity>,
}

#[inline]
fn project_clipmap_shell(
    block: &mut CelestialClipmapBlock,
    transform: &mut Transform,
    visibility: &mut Visibility,
    frame: ClipmapProjectedAuthorityFrame,
    projection_eye: DVec3,
    presentation_origin: Vec3,
    metre_to_view_f64: f64,
    metre_to_view: f32,
) -> bool {
    let local_origin = block.spec.key.origin_local_metres();
    let relative_metres =
        frame.relative_metres + frame.orientation * local_origin;
    let projected =
        (relative_metres - projection_eye) * metre_to_view_f64;
    if !projected.is_finite() {
        block.projection_ready = false;
        *visibility = Visibility::Hidden;
        return false;
    }

    let projected = Vec3::new(
        projected.x as f32,
        projected.y as f32,
        projected.z as f32,
    );
    let translation = presentation_origin + projected;
    if !translation.is_finite() {
        block.projection_ready = false;
        *visibility = Visibility::Hidden;
        return false;
    }

    transform.translation = translation;
    transform.rotation = frame.rotation;
    transform.scale = Vec3::splat(metre_to_view);
    block.projection_ready = true;
    true
}

fn sync_celestial_clipmap_transforms(
    view: Single<&UsfViewContext, With<UsfViewRenderAnchor>>,
    mut registry: ResMut<CelestialClipmapRegistry>,
    mut material_params: CelestialClipmapMaterialParams,
    authorities: Query<(
        &UsfPosition,
        &UsfSemanticFrame,
        &CelestialVoxelField,
        &CelestialVoxelRealizationPolicy,
    )>,
    mut coverage: ResMut<CelestialClipmapCoverageSnapshot>,
    mut presentation_state: ResMut<CelestialTerrainPresentationState>,
    mut telemetry: ResMut<CelestialClipmapTelemetry>,
    mut blocks: Query<(
        &mut CelestialClipmapBlock,
        &mut Transform,
        &mut Visibility,
        &mut MeshMaterial3d<VoxelRenderMaterial>,
    )>,
    mut scratch: Local<CelestialClipmapTransformScratch>,
    mut logged_projection: Local<bool>,
) {
    let Some(metre_to_view_f64) =
        view.projection_factor_f64(SpatialScale::ZERO)
    else {
        for &entity in registry.active_entities.values() {
            if let Ok((mut block, _, mut visibility, _)) =
                blocks.get_mut(entity)
            {
                block.projection_ready = false;
                *visibility = Visibility::Hidden;
            }
        }
        scratch.live_authorities.clear();
        scratch
            .live_authorities
            .extend(registry.plans.keys().copied());
        coverage.retain_authorities(&scratch.live_authorities);
        for &authority in &scratch.live_authorities {
            coverage.replace_authority_from_slice(authority, &[]);
        }
        presentation_state.clear();
        telemetry.record_visible_frontier(
            0,
            0,
            &HashSet::new(),
            None,
            None,
        );
        scratch.initialized = false;
        return;
    };

    let metre_to_view = metre_to_view_f64 as f32;
    if !metre_to_view.is_finite() || metre_to_view <= 0.0 {
        return;
    }

    let view_stamp = ClipmapViewProjectionStamp {
        anchor: *view.anchor(),
        metre_to_view_f64,
        projection_eye: view.projection_eye_offset_metres(),
        presentation_origin: view.presentation_origin(),
    };
    let view_changed =
        scratch.last_view_stamp != Some(view_stamp);

    let _dirty_span =
        bevy::log::info_span!(
            "celestial_clipmap.transform_dirty_check"
        )
        .entered();

    scratch.next_authority_stamps.clear();
    let mut authority_changed =
        scratch.authority_stamps.len() != registry.plans.len();
    for &authority in registry.plans.keys() {
        let Ok((body_origin, body_frame, _, policy)) =
            authorities.get(authority)
        else {
            authority_changed = true;
            continue;
        };
        let stamp = ClipmapAuthorityProjectionStamp {
            origin: *body_origin,
            orientation: body_frame.orientation(),
            presentation_material:
                policy.presentation_material().clone(),
        };
        if scratch.authority_stamps.get(&authority) != Some(&stamp) {
            authority_changed = true;
        }
        scratch.next_authority_stamps.insert(authority, stamp);
    }
    {
        let CelestialClipmapTransformScratch {
            authority_stamps,
            next_authority_stamps,
            ..
        } = &mut *scratch;
        std::mem::swap(
            authority_stamps,
            next_authority_stamps,
        );
    }

    let frontier_changed = !scratch.initialized
        || scratch.last_frontier_epoch != registry.frontier_epoch;
    let projection_changed = !scratch.initialized
        || scratch.last_projection_epoch != registry.projection_epoch
        || !registry.projection_pending.is_empty();

    drop(_dirty_span);

    if scratch.initialized
        && !view_changed
        && !authority_changed
        && !frontier_changed
        && !projection_changed
    {
        return;
    }

    if scratch.initialized
        && !view_changed
        && !authority_changed
        && !frontier_changed
        && projection_changed
    {
        let _span = bevy::log::info_span!(
            "celestial_clipmap.project_pending"
        )
        .entered();

        scratch.pending_entities.clear();
        std::mem::swap(
            &mut scratch.pending_entities,
            &mut registry.projection_pending,
        );

        // Move the reusable Vec out while iterating so the drain does not
        // retain a mutable borrow of `scratch` while we read `frames`.
        // Restore the now-empty allocation afterward for reuse next frame.
        let mut pending_entities =
            std::mem::take(&mut scratch.pending_entities);

        for entity in pending_entities.drain(..) {
            let Ok((mut block, mut transform, mut visibility, _)) =
                blocks.get_mut(entity)
            else {
                continue;
            };
            if !block.active || block.committed {
                continue;
            }
            let Some(frame) =
                scratch.frames.get(&block.authority).copied()
            else {
                continue;
            };
            project_clipmap_shell(
                &mut block,
                &mut transform,
                &mut visibility,
                frame,
                view_stamp.projection_eye,
                view_stamp.presentation_origin,
                metre_to_view_f64,
                metre_to_view,
            );
        }

        scratch.pending_entities = pending_entities;
        scratch.last_projection_epoch = registry.projection_epoch;
        return;
    }

    let _full_span =
        bevy::log::info_span!(
            "celestial_clipmap.transform_full"
        )
        .entered();

    scratch.pending_entities.clear();
    std::mem::swap(
        &mut scratch.pending_entities,
        &mut registry.projection_pending,
    );
    scratch.pending_entities.clear();

    scratch.frames.clear();
    scratch.counts.clear();
    scratch.binary_primary.clear();
    scratch.projected_committed.clear();
    scratch.visible_levels.clear();
    scratch.live_authorities.clear();
    scratch
        .live_authorities
        .extend(registry.plans.keys().copied());
    for cells in scratch.visible_by_authority.values_mut() {
        cells.clear();
    }

    {
        let _span = bevy::log::info_span!(
            "celestial_clipmap.transform_authority_frames"
        )
        .entered();

        for &authority in registry.plans.keys() {
            let Ok((body_origin, body_frame, _, _)) =
                authorities.get(authority)
            else {
                continue;
            };
            let Ok(relative_metres) =
                body_origin.relative_at_scale_bounded_f64(
                    view.anchor(),
                    SpatialScale::ZERO,
                    f64::MAX,
                )
            else {
                continue;
            };
            let orientation = body_frame.orientation();
            let rotation = Quat::from_xyzw(
                orientation.x as f32,
                orientation.y as f32,
                orientation.z as f32,
                orientation.w as f32,
            )
            .normalize();

            scratch.frames.insert(
                authority,
                ClipmapProjectedAuthorityFrame {
                    relative_metres,
                    orientation,
                    rotation,
                },
            );
            scratch.counts.insert(authority, (0, 0));
        }
    }

    let mut projected_any = false;

    {
        let _span = bevy::log::info_span!(
            "celestial_clipmap.transform_active_shells"
        )
        .entered();

        for &entity in registry.active_entities.values() {
            let Ok((
                mut block,
                mut transform,
                mut visibility,
                mut material,
            )) = blocks.get_mut(entity)
            else {
                continue;
            };
            if !block.active {
                continue;
            }

            let Some(frame) =
                scratch.frames.get(&block.authority).copied()
            else {
                block.projection_ready = false;
                *visibility = Visibility::Hidden;
                continue;
            };
            let Some(plan) = registry.plans.get(&block.authority) else {
                block.projection_ready = false;
                *visibility = Visibility::Hidden;
                continue;
            };

            let relative_level = block
                .spec
                .key
                .resolution
                .binary_exponent()
                .saturating_sub(plan.key.finest_exponent);
            if block.material_relative_level != relative_level {
                if let Ok((_, _, _, policy)) =
                    authorities.get(block.authority)
                {
                    let standard_materials =
                        &material_params.standard_materials;
                    let debug_grid =
                        &material_params.library.debug_grid;
                    let render_materials =
                        &mut material_params.render_materials;
                    let shader_buffers =
                        &mut material_params.shader_buffers;
                    let band_materials =
                        &mut material_params.band_materials;
                    if let Some(desired) =
                        band_materials.material_for(
                            standard_materials,
                            render_materials,
                            shader_buffers,
                            debug_grid,
                            policy.presentation_material(),
                            relative_level,
                        )
                    {
                        material.0 = desired;
                        block.material_relative_level = relative_level;
                    }
                }
            }

            if block.committed {
                let counts =
                    scratch.counts.entry(block.authority).or_default();
                counts.0 = counts.0.saturating_add(1);
            }

            if project_clipmap_shell(
                &mut block,
                &mut transform,
                &mut visibility,
                frame,
                view_stamp.projection_eye,
                view_stamp.presentation_origin,
                metre_to_view_f64,
                metre_to_view,
            ) {
                projected_any = true;
                if block.committed {
                    let counts =
                        scratch.counts.entry(block.authority).or_default();
                    counts.1 = counts.1.saturating_add(1);
                    scratch.projected_committed.push(entity);
                }
            }
        }
    }

    {
        let CelestialClipmapTransformScratch {
            counts,
            binary_primary,
            ..
        } = &mut *scratch;

        for (&authority, &(expected, projected)) in counts.iter() {
            if binary_frontier_projection_complete(expected, projected) {
                binary_primary.insert(authority);
            }
        }
    }
    let binary_primary_count = scratch.binary_primary.len();

    let mut visible_blocks = 0usize;
    let mut finest_visible_spacing = None::<f64>;
    let mut coarsest_visible_spacing = None::<f64>;

    {
        let _span = bevy::log::info_span!(
            "celestial_clipmap.transform_visibility_coverage"
        )
        .entered();

        let CelestialClipmapTransformScratch {
            projected_committed,
            binary_primary,
            visible_levels,
            visible_by_authority,
            ..
        } = &mut *scratch;

        for &entity in projected_committed.iter() {
            let Ok((block, _, mut visibility, _)) =
                blocks.get_mut(entity)
            else {
                continue;
            };
            let visible =
                binary_primary.contains(&block.authority);
            *visibility = if visible {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            };
            if !visible {
                continue;
            }

            let spacing = block.spec.key.spacing_metres();
            visible_blocks = visible_blocks.saturating_add(1);
            visible_levels.insert(
                block.spec.key.resolution.binary_exponent(),
            );
            finest_visible_spacing = Some(
                finest_visible_spacing
                    .map_or(spacing, |value| value.min(spacing)),
            );
            coarsest_visible_spacing = Some(
                coarsest_visible_spacing
                    .map_or(spacing, |value| value.max(spacing)),
            );
            visible_by_authority
                .entry(block.authority)
                .or_default()
                .push(CelestialClipmapCoverageCell {
                    center_local_metres:
                        block.spec.key.center_local_metres(),
                    half_extent_metres:
                        block.spec.key.half_extent_metres(),
                    sample_spacing_metres: spacing,
                });
        }
    }

    {
        let _span = bevy::log::info_span!(
            "celestial_clipmap.transform_publish_coverage"
        )
        .entered();

        coverage.retain_authorities(&scratch.live_authorities);
        for &authority in &scratch.live_authorities {
            let next = scratch
                .visible_by_authority
                .get(&authority)
                .map(Vec::as_slice)
                .unwrap_or(&[]);
            coverage.replace_authority_from_slice(authority, next);
        }

        presentation_state.replace_binary_primary_from(
            &scratch.binary_primary,
        );
        telemetry.record_visible_frontier(
            visible_blocks,
            binary_primary_count,
            &scratch.visible_levels,
            finest_visible_spacing,
            coarsest_visible_spacing,
        );
    }

    scratch.last_view_stamp = Some(view_stamp);
    scratch.last_projection_epoch = registry.projection_epoch;
    scratch.last_frontier_epoch = registry.frontier_epoch;
    scratch.initialized = true;

    if projected_any && !*logged_projection {
        info!(
            view_scale = %view.scale(),
            view_exponent = view.continuous_exponent(),
            metre_to_view,
            visible_binary_blocks = visible_blocks,
            binary_primary_authorities = binary_primary_count,
            visible_binary_levels = scratch.visible_levels.len(),
            "celestial binary presentation owns complete authority-level frontiers"
        );
        *logged_projection = true;
    }
}

/// Dense physical/current-interaction presentation is a bootstrap/emergency
/// fallback only. The interaction Scale chooses which physical dense cache is
/// available; it does not choose visual LOD. Once a complete binary frontier
/// owns the authority, all dense visual chunks yield together.
fn enforce_dense_interaction_presentation(
    mut commands: Commands,
    view: Single<&UsfViewContext, With<UsfViewRenderAnchor>>,
    view_demands: Res<UsfViewDemandSnapshot>,
    interaction: Res<UsfPrimaryInteractionSlice>,
    presentation_state: Res<CelestialTerrainPresentationState>,
    runtimes: Query<&VoxelMaterializationRuntime>,
    worlds: Query<(
        &CelestialVoxelRealization,
        &UsfScaleLayer,
        &VoxelWorld,
        Option<&VoxelStreaming>,
    )>,
    mut presentations: Query<
        (&ChildOf, &mut Visibility),
        With<VoxelMaterializationPresentation>,
    >,
    mut telemetry: ResMut<CelestialClipmapTelemetry>,
) {
    let primary_view_demand = view_demands.iter().next();
    let mut fallback_held = 0usize;
    let mut fallback_retire_ready = 0usize;
    let mut fallback_forced_retire = 0usize;

    for (parent, mut visibility) in &mut presentations {
        let Ok(runtime) = runtimes.get(parent.0) else {
            continue;
        };
        let Ok((realization, layer, world, streaming)) =
            worlds.get(runtime.world())
        else {
            continue;
        };

        let target_scale = interaction.target_scale();

        let center = world
            .materialization_address(runtime.key())
            .ok()
            .and_then(|address| address.center().ok());

        let presentation_requested = streaming.is_none_or(|streaming| {
            streaming
                .effective_roles(runtime.key())
                .contains(UsfScaleRoleMask::PRESENTATION)
        });

        if layer.scale() != target_scale || !runtime.active() {
            *visibility = Visibility::Hidden;
            commands
                .entity(parent.0)
                .insert(VoxelPresentationFallbackRetireReady);
            fallback_retire_ready =
                fallback_retire_ready.saturating_add(1);
            continue;
        }

        // Whole-authority handoff: dense never clips or fills individual binary
        // blocks. It is either the fallback renderer for this body or not.
        if presentation_state.is_binary_primary(realization.authority()) {
            *visibility = Visibility::Hidden;
            if presentation_requested {
                commands
                    .entity(parent.0)
                    .remove::<VoxelPresentationFallbackRetireReady>();
            } else {
                commands
                    .entity(parent.0)
                    .insert(VoxelPresentationFallbackRetireReady);
                fallback_retire_ready =
                    fallback_retire_ready.saturating_add(1);
            }
            continue;
        }

        let view_relevant = center.is_some_and(|center| {
            primary_view_demand.is_none_or(|demand| {
                demand.intersects_presentation_native_aabb(
                    layer.scale(),
                    &center,
                    Vec3::splat(
                        MATERIALIZATION_CHUNK_SIZE as f32 * 0.5,
                    ),
                )
            })
        });

        let fallback_frontier_local = center.is_some_and(|center| {
            let retention_native =
                MATERIALIZATION_CHUNK_SIZE as f32
                    * DENSE_FALLBACK_RETENTION_CHUNKS;
            center
                .relative_at_scale_bounded(
                    &view.anchor(),
                    layer.scale(),
                    retention_native
                        + MATERIALIZATION_CHUNK_SIZE as f32 * 2.0,
                )
                .ok()
                .is_some_and(|relative| {
                    relative.length() <= retention_native
                })
        });

        if presentation_requested {
            // Active physical/view demand keeps the cache reusable. Until the
            // binary authority-level transaction succeeds, dense is the visible
            // bootstrap representation rather than one member of a mixed LOD.
            commands
                .entity(parent.0)
                .remove::<VoxelPresentationFallbackRetireReady>();
            *visibility = Visibility::Inherited;
            continue;
        }

        // Historical dense presentation is not a cache of old visual LODs.
        // Retain only a short local bridge while binary authority is absent.
        let forced_by_frontier =
            view_relevant && !fallback_frontier_local;

        if !view_relevant || !fallback_frontier_local {
            *visibility = Visibility::Hidden;
            commands
                .entity(parent.0)
                .insert(VoxelPresentationFallbackRetireReady);
            fallback_retire_ready =
                fallback_retire_ready.saturating_add(1);
            if forced_by_frontier {
                fallback_forced_retire =
                    fallback_forced_retire.saturating_add(1);
            }
        } else {
            *visibility = Visibility::Inherited;
            commands
                .entity(parent.0)
                .remove::<VoxelPresentationFallbackRetireReady>();
            fallback_held = fallback_held.saturating_add(1);
        }
    }

    telemetry.record_dense_fallbacks(
        fallback_held,
        fallback_retire_ready,
        fallback_forced_retire,
    );
}

pub(super) fn configure(app: &mut App) {
    app.init_resource::<CelestialClipmapRegistry>()
        .init_resource::<CelestialClipmapBandDebugMaterials>()
        .init_resource::<CelestialClipmapCoverageSnapshot>()
        .init_resource::<CelestialTerrainPresentationState>()
        .init_resource::<CelestialClipmapTelemetry>()
        .add_systems(Update, sync_celestial_clipmap_realizations)
        .add_systems(
            PostUpdate,
            sync_celestial_clipmap_transforms
                .after(UsfCapabilitySet::ReconcileCoverage)
                .in_set(UsfSpatialSet::ViewProjection),
        )
        .add_systems(
            PostUpdate,
            enforce_dense_interaction_presentation
                .after(UsfSpatialSet::ViewProjection),
        );
}
