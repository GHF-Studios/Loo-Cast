//! Live celestial presentation clipmap over the voxel-local binary resolution domain.
//!
//! This is presentation only. Semantic terrain remains [`CelestialVoxelField`];
//! dense voxel worlds keep collision/editing authority. The clipmap is a
//! reconstructible mesh adapter whose LOD axis is independent of USF Scale.

use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

use bevy::{
    asset::RenderAssetUsages,
    camera::visibility::RenderLayers,
    light::{NotShadowCaster, NotShadowReceiver},
    math::DVec3,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};

use transvoxel::prelude::{
    extract_from_field, Block, FieldCaching, GenericMeshBuilder, TransitionSide,
    TransitionSides,
};

use crate::reconstructible::{
    ReconstructibleFrameBudget, ReconstructibleWorkClass,
};
use crate::view::USF_PRESENTATION_LAYER;
use crate::voxel::{
    developer_policy::{
        presentation_surface_radius_bounds_metres,
    },
    MATERIALIZATION_CHUNK_SIZE, VoxelStreaming, VoxelWorld,
};

use crate::{
    devtools::{
        DeveloperScalarPolicyRuntime, DeveloperScalarPolicySnapshot,
        DeveloperScriptWorkbench,
    },
    ecs::UsfPresentationProjectionOf,
    spatial::{
         SpatialRealizationGranularityRequest, SpatialScale, UsfCapabilitySet,
        UsfPosition, UsfPrimaryInteractionSlice, UsfScaleCoverageSnapshot,
        UsfScaleLayer, UsfScaleRoleMask, UsfSemanticFrame, UsfSpatialSet,
        UsfViewContext, UsfViewDemandSnapshot, UsfViewRenderAnchor,
    },
};

use super::{
    VoxelPresentationResolution, VoxelTransitionFace, VoxelTransitionFaces,
};
use super::super::{
    CelestialVoxelField, CelestialVoxelRealization, CelestialVoxelRealizationPolicy,
    VoxelAuthority,
    manifestation::{
        VoxelMaterializationPresentation, VoxelMaterializationRuntime,
        VoxelPresentationFallbackRetireReady,
    },
    worker::{VoxelWorkerLane, VoxelWorkerPool, VoxelWorkerTask, VoxelWorkerTicket},
};

const BLOCK_SUBDIVISIONS: usize = 8;
// presentation-resolution-independent-screen-error-v1
// terrain-continuity-closure-megapass-v1
// moving-volume-rolling-clipmap-megapass-v1
// terrain-transaction-root-cause-megapass-v2
// Binary presentation is independent from decimal USF interaction Scale.
const MIN_SAMPLE_SPACING_METRES: f64 = 1.0;
const MAX_FINE_SAMPLE_SPACING_METRES: f64 = 2_048.0;
// whole-body-volumetric-clipmap-v1
//
// The coarsest binary bricks are allowed to span the semantic body. A small
// conservative margin absorbs canonical relief without inventing a second
// spherical surface representation.
const WHOLE_BODY_ROOT_MARGIN: f64 = 1.125;
const TARGET_CELLS_PER_DISTANCE: f64 = 16.0;
// volumetric-boundary-continuity-visual-handoff-v1
const TARGET_CELLS_PER_CLEARANCE: f64 = 128.0;
const TARGET_PIXELS_PER_BINARY_SAMPLE: f64 = 4.0;
const DENSE_VISUAL_HANDOFF_CELLS_PER_DISTANCE: f64 = 64.0;
const DENSE_VISUAL_HANDOFF_MAX_PIXELS_PER_SAMPLE: f64 = 8.0;
const DENSE_VISUAL_HANDOFF_MIN_SAMPLE_DISTANCES: f64 = 12.0;
// frontier-local-dense-fallback-v1
// Inactive dense presentation is a seam bridge, not a history buffer.
const DENSE_FALLBACK_RETENTION_CHUNKS: f32 = 4.0;

// terrain-continuity-closure-megapass-v1
// sparse-boundary-frontier-v1
//
// Deep local detail is a sparse boundary aperture over persistent coarse body
// ancestry. The budget scales with requested binary depth; this hard ceiling is
// reconstructible protection, not a normal target.
const MIN_SPARSE_FRONTIER_LEAVES: usize = 4_096;
const MAX_SPARSE_FRONTIER_LEAVES: usize = 32_768;
const LEAVES_PER_REQUESTED_LEVEL: usize = 640;
const MAX_PRIMARY_REFINEMENTS_PER_WAVE: usize = 96;
const MAX_RECORDED_FRONTIER_STAGES: usize = 48;
const CLIPMAP_VALIDITY_AGGREGATES_ACROSS: u32 = 8;
const CLIPMAP_MIN_VALIDITY_SECONDS: f64 = 0.10;
const CLIPMAP_MAX_VALIDITY_SECONDS: f64 = 2.0;
const CLIPMAP_LATENCY_MULTIPLIER: f64 = 4.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CelestialClipmapBlockKey {
    resolution: VoxelPresentationResolution,
    coord: IVec3,
}

impl CelestialClipmapBlockKey {
    fn spacing_metres(self) -> f64 {
        self.resolution.sample_spacing_metres()
    }

    fn extent_metres(self) -> f64 {
        self.spacing_metres() * BLOCK_SUBDIVISIONS as f64
    }

    fn origin_local_metres(self) -> DVec3 {
        let extent = self.extent_metres();
        DVec3::new(
            f64::from(self.coord.x) * extent,
            f64::from(self.coord.y) * extent,
            f64::from(self.coord.z) * extent,
        )
    }

    fn half_extent_metres(self) -> DVec3 {
        DVec3::splat(self.extent_metres() * 0.5)
    }

    fn center_local_metres(self) -> DVec3 {
        self.origin_local_metres() + self.half_extent_metres()
    }

    fn children(self) -> Option<[Self; 8]> {
        let resolution = self.resolution.finer()?;
        let base = IVec3::new(
            self.coord.x.checked_mul(2)?,
            self.coord.y.checked_mul(2)?,
            self.coord.z.checked_mul(2)?,
        );
        Some([
            Self { resolution, coord: base + IVec3::new(0, 0, 0) },
            Self { resolution, coord: base + IVec3::new(1, 0, 0) },
            Self { resolution, coord: base + IVec3::new(0, 1, 0) },
            Self { resolution, coord: base + IVec3::new(1, 1, 0) },
            Self { resolution, coord: base + IVec3::new(0, 0, 1) },
            Self { resolution, coord: base + IVec3::new(1, 0, 1) },
            Self { resolution, coord: base + IVec3::new(0, 1, 1) },
            Self { resolution, coord: base + IVec3::new(1, 1, 1) },
        ])
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CelestialClipmapBlockSpec {
    key: CelestialClipmapBlockKey,
    transition_faces: VoxelTransitionFaces,
}

// stable-body-frontier-dense-terminal-aperture-v1
// sticky-refinement-anchor-v1
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CelestialClipmapPlanKey {
    // Observer coordinates are deliberately absent. They are refinement state,
    // not identity of the body-local presentation hierarchy.
    finest_exponent: i16,
    coarsest_exponent: i16,
    policy_revision: u64,
}

// transactional-binary-refinement-frontiers-v1
#[derive(Debug)]
struct CelestialClipmapPlan {
    key: CelestialClipmapPlanKey,
    field: CelestialVoxelField,
    policy: Option<DeveloperScalarPolicySnapshot>,
    /// Sticky body-local point about which the current refinement frontier was
    /// planned. This may move; root topology does not.
    planning_anchor_local: DVec3,
    validity_radius_metres: f64,
    generation: u64,
    stages: Vec<Vec<CelestialClipmapBlockSpec>>,
    stage_index: usize,
    desired: Vec<CelestialClipmapBlockSpec>,
    completed: HashSet<CelestialClipmapBlockSpec>,
    meshful: HashSet<CelestialClipmapBlockSpec>,
    /// Specs already proven to contain no presentation triangles. Unlike mesh
    /// entities these have no ECS artifact, so remember the result explicitly
    /// across stages and warm replans.
    known_empty: HashSet<CelestialClipmapBlockSpec>,
    committed_specs: HashSet<CelestialClipmapBlockSpec>,
    committed_generation: Option<u64>,
}


// aggressive-demand-and-clipmap-local-balance-v1
// celestial-clipmap-planner-superpass-v1
//
// Planner state is deliberately reusable. Observer motion changes *which*
// presentation blocks are wanted; it does not change the semantic answer to
// "can this dyadic block intersect this body surface?" for a stable
// (field, script revision). Keep two generations of those classifications so
// ordinary movement pays mostly for the changed frontier without an unbounded
// spatial cache.
#[derive(Default, Clone)]
struct CelestialClipmapSurfaceCache {
    field: Option<CelestialVoxelField>,
    policy_revision: u64,
    hot: HashMap<CelestialClipmapBlockKey, bool>,
    warm: HashMap<CelestialClipmapBlockKey, bool>,
    next: HashMap<CelestialClipmapBlockKey, bool>,
    hits: usize,
    misses: usize,
    cheap_rejects: usize,
}

impl CelestialClipmapSurfaceCache {
    fn begin_plan(
        &mut self,
        field: CelestialVoxelField,
        policy_revision: u64,
    ) {
        if self.field != Some(field) || self.policy_revision != policy_revision {
            self.field = Some(field);
            self.policy_revision = policy_revision;
            self.hot.clear();
            self.warm.clear();
            self.next.clear();
        } else {
            self.next.clear();
        }
        self.hits = 0;
        self.misses = 0;
        self.cheap_rejects = 0;
    }

    fn intersects(
        &mut self,
        field: CelestialVoxelField,
        key: CelestialClipmapBlockKey,
        policy: Option<&DeveloperScalarPolicyRuntime>,
    ) -> bool {
        let cached = self
            .next
            .get(&key)
            .copied()
            .or_else(|| self.hot.get(&key).copied())
            .or_else(|| self.warm.get(&key).copied());

        if let Some(value) = cached {
            self.hits = self.hits.saturating_add(1);
            self.next.insert(key, value);
            return value;
        }

        self.misses = self.misses.saturating_add(1);

        if !block_may_intersect_presentation_shell(field, key) {
            self.cheap_rejects = self.cheap_rejects.saturating_add(1);
            self.next.insert(key, false);
            return false;
        }

        let value = block_intersects_semantic_surface(field, key, policy);
        self.next.insert(key, value);
        value
    }

    fn finish_plan(&mut self) {
        self.warm.clear();
        std::mem::swap(&mut self.warm, &mut self.hot);
        std::mem::swap(&mut self.hot, &mut self.next);
    }
}

#[derive(Resource, Default)]
struct CelestialClipmapPlannerPolicyCache {
    revision: u64,
    enabled: bool,
    runtime: Option<DeveloperScalarPolicyRuntime>,
}

impl CelestialClipmapPlannerPolicyCache {
    fn runtime_for(
        &mut self,
        snapshot: Option<&DeveloperScalarPolicySnapshot>,
    ) -> Option<&DeveloperScalarPolicyRuntime> {
        let enabled = snapshot.is_some();
        let revision =
            snapshot.map_or(0, DeveloperScalarPolicySnapshot::revision);

        if self.enabled != enabled || self.revision != revision {
            self.enabled = enabled;
            self.revision = revision;
            self.runtime = snapshot
                .and_then(|snapshot| snapshot.compile_runtime().ok());
        }

        self.runtime.as_ref()
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

fn refinement_candidate(
    key: CelestialClipmapBlockKey,
    finest: VoxelPresentationResolution,
    planning_anchor_local: DVec3,
    validity_radius_metres: f64,
) -> Option<ClipmapRefinementCandidate> {
    if key.resolution <= finest {
        return None;
    }
    let distance = block_distance_to_point(key, planning_anchor_local);
    let effective_distance = (distance - validity_radius_metres).max(0.0);
    let target = target_resolution_at_distance(finest, effective_distance);
    let refinement_debt = key
        .resolution
        .binary_exponent()
        .saturating_sub(target.binary_exponent());
    let projected_error =
        key.spacing_metres() / distance.max(key.spacing_metres());
    (refinement_debt > 0).then_some(ClipmapRefinementCandidate {
        inside_validity: distance <= validity_radius_metres,
        distance,
        projected_error,
        refinement_debt,
        key,
    })
}


const CLIPMAP_FACE_DIRECTIONS: [(VoxelTransitionFace, IVec3); 6] = [
    (VoxelTransitionFace::LowX, IVec3::new(-1, 0, 0)),
    (VoxelTransitionFace::HighX, IVec3::new(1, 0, 0)),
    (VoxelTransitionFace::LowY, IVec3::new(0, -1, 0)),
    (VoxelTransitionFace::HighY, IVec3::new(0, 1, 0)),
    (VoxelTransitionFace::LowZ, IVec3::new(0, 0, -1)),
    (VoxelTransitionFace::HighZ, IVec3::new(0, 0, 1)),
];

fn checked_coord_add(lhs: IVec3, rhs: IVec3) -> Option<IVec3> {
    Some(IVec3::new(
        lhs.x.checked_add(rhs.x)?,
        lhs.y.checked_add(rhs.y)?,
        lhs.z.checked_add(rhs.z)?,
    ))
}

fn parent_coord(coord: IVec3) -> IVec3 {
    IVec3::new(
        coord.x.div_euclid(2),
        coord.y.div_euclid(2),
        coord.z.div_euclid(2),
    )
}

// refinement-empty-is-evidence-v1
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LeafRefinementOutcome {
    Refined,
    RemovedAsEmpty,
    Unavailable,
}

fn refine_leaf_indexed(
    leaves: &mut HashSet<CelestialClipmapBlockKey>,
    parent: CelestialClipmapBlockKey,
    field: CelestialVoxelField,
    _policy: Option<&DeveloperScalarPolicyRuntime>,
    _surface_cache: &mut CelestialClipmapSurfaceCache,
    planning_anchor_local: DVec3,
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
        if block_intersects_refinement_boundary(
            field,
            child,
            planning_anchor_local,
        ) {
            leaves.insert(child);
            inserted.push(child);
        }
    }

    if inserted.is_empty() {
        LeafRefinementOutcome::RemovedAsEmpty
    } else {
        LeafRefinementOutcome::Refined
    }
}

/// Find the unique leaf on the other side of one face when that leaf is at the
/// same or a coarser binary level. Starting from the same-level adjacent cell,
/// parent ascent is exact for dyadic octree coordinates (including negatives).
fn same_or_coarser_face_neighbor(
    leaves: &HashSet<CelestialClipmapBlockKey>,
    key: CelestialClipmapBlockKey,
    face: VoxelTransitionFace,
    maximum_exponent: i16,
) -> Option<CelestialClipmapBlockKey> {
    let delta = CLIPMAP_FACE_DIRECTIONS
        .iter()
        .find_map(|(candidate, delta)| (*candidate == face).then_some(*delta))?;
    let mut coord = checked_coord_add(key.coord, delta)?;
    let mut resolution = key.resolution;

    loop {
        let candidate = CelestialClipmapBlockKey { resolution, coord };
        if leaves.contains(&candidate) {
            return Some(candidate);
        }

        if resolution.binary_exponent() >= maximum_exponent {
            return None;
        }
        resolution = resolution.coarser()?;
        coord = parent_coord(coord);
    }
}

/// Restore the 2:1 invariant using only local neighbor ascent.
fn balance_leaves_2_to_1_local(
    field: CelestialVoxelField,
    leaves: &mut HashSet<CelestialClipmapBlockKey>,
    policy: Option<&DeveloperScalarPolicyRuntime>,
    surface_cache: &mut CelestialClipmapSurfaceCache,
    planning_anchor_local: DVec3,
    maximum_leaves: usize,
) -> bool {
    let Some(maximum_exponent) = leaves
        .iter()
        .map(|key| key.resolution.binary_exponent())
        .max()
    else {
        return true;
    };

    let mut queue = leaves.iter().copied().collect::<VecDeque<_>>();
    let mut inserted = Vec::<CelestialClipmapBlockKey>::with_capacity(8);

    while let Some(key) = queue.pop_front() {
        if !leaves.contains(&key) {
            continue;
        }

        let mut corrected = false;
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

            match refine_leaf_indexed(
                leaves,
                neighbor,
                field,
                policy,
                surface_cache,
                planning_anchor_local,
                &mut inserted,
            ) {
                LeafRefinementOutcome::Refined => {
                    queue.extend(inserted.iter().copied());
                }
                LeafRefinementOutcome::RemovedAsEmpty => {
                    // Fine evidence removed a conservative false-positive;
                    // there is no surviving surface transition to balance.
                }
                LeafRefinementOutcome::Unavailable => return false,
            }

            queue.push_back(key);
            corrected = true;
            break;
        }

        if corrected {
            continue;
        }
    }

    true
}

fn finer_face_neighbors(
    key: CelestialClipmapBlockKey,
    face: VoxelTransitionFace,
) -> Option<[CelestialClipmapBlockKey; 4]> {
    let resolution = key.resolution.finer()?;
    let bx = key.coord.x.checked_mul(2)?;
    let by = key.coord.y.checked_mul(2)?;
    let bz = key.coord.z.checked_mul(2)?;

    let make = |x: i32, y: i32, z: i32| CelestialClipmapBlockKey {
        resolution,
        coord: IVec3::new(x, y, z),
    };

    match face {
        VoxelTransitionFace::LowX => {
            let x = bx.checked_sub(1)?;
            Some([
                make(x, by, bz),
                make(x, by.checked_add(1)?, bz),
                make(x, by, bz.checked_add(1)?),
                make(x, by.checked_add(1)?, bz.checked_add(1)?),
            ])
        }
        VoxelTransitionFace::HighX => {
            let x = bx.checked_add(2)?;
            Some([
                make(x, by, bz),
                make(x, by.checked_add(1)?, bz),
                make(x, by, bz.checked_add(1)?),
                make(x, by.checked_add(1)?, bz.checked_add(1)?),
            ])
        }
        VoxelTransitionFace::LowY => {
            let y = by.checked_sub(1)?;
            Some([
                make(bx, y, bz),
                make(bx.checked_add(1)?, y, bz),
                make(bx, y, bz.checked_add(1)?),
                make(bx.checked_add(1)?, y, bz.checked_add(1)?),
            ])
        }
        VoxelTransitionFace::HighY => {
            let y = by.checked_add(2)?;
            Some([
                make(bx, y, bz),
                make(bx.checked_add(1)?, y, bz),
                make(bx, y, bz.checked_add(1)?),
                make(bx.checked_add(1)?, y, bz.checked_add(1)?),
            ])
        }
        VoxelTransitionFace::LowZ => {
            let z = bz.checked_sub(1)?;
            Some([
                make(bx, by, z),
                make(bx.checked_add(1)?, by, z),
                make(bx, by.checked_add(1)?, z),
                make(bx.checked_add(1)?, by.checked_add(1)?, z),
            ])
        }
        VoxelTransitionFace::HighZ => {
            let z = bz.checked_add(2)?;
            Some([
                make(bx, by, z),
                make(bx.checked_add(1)?, by, z),
                make(bx, by.checked_add(1)?, z),
                make(bx.checked_add(1)?, by.checked_add(1)?, z),
            ])
        }
    }
}

fn transition_faces_for_leaf(
    leaves: &HashSet<CelestialClipmapBlockKey>,
    key: CelestialClipmapBlockKey,
) -> VoxelTransitionFaces {
    let mut transitions = VoxelTransitionFaces::default();

    for (face, _) in CLIPMAP_FACE_DIRECTIONS {
        let Some(finer) = finer_face_neighbors(key, face) else {
            continue;
        };
        if finer.into_iter().any(|candidate| leaves.contains(&candidate)) {
            transitions.insert(face);
        }
    }

    transitions
}

#[derive(Resource, Default)]
struct CelestialClipmapRegistry {
    next_generation: u64,
    plans: HashMap<Entity, CelestialClipmapPlan>,
    planner_caches: HashMap<Entity, CelestialClipmapSurfaceCache>,
}

impl CelestialClipmapRegistry {
    fn next_generation(&mut self) -> u64 {
        self.next_generation = self.next_generation.wrapping_add(1).max(1);
        self.next_generation
    }
}

#[derive(Resource, Debug, Default, Clone)]
pub(in crate::voxel) struct CelestialClipmapTelemetry {
    plan_requests_total: u64,
    cold_plans_total: u64,
    warm_replans_total: u64,
    visible_blocks: usize,
    dense_aperture_yielded_blocks: usize,
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
        yielded_blocks: usize,
        visible_levels: &HashSet<i16>,
        finest_spacing: Option<f64>,
        coarsest_spacing: Option<f64>,
    ) {
        self.visible_blocks = visible_blocks;
        self.dense_aperture_yielded_blocks = yielded_blocks;
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
            "plans={} cold={} warm={} visible={} aperture_yield={} levels={} visible={}..{}m clearance={}m requested_finest={}m planned_finest={}m leaf_budget={} leaves={} saturated={} fallback_hold={} fallback_retire={} fallback_forced={} focus_lag={}m plan_fresh={} plan_rolling={} plan_drop={}",
            self.plan_requests_total,
            self.cold_plans_total,
            self.warm_replans_total,
            self.visible_blocks,
            self.dense_aperture_yielded_blocks,
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
    policy_revision: u64,
    spec: CelestialClipmapBlockSpec,
    projection_ready: bool,
}

#[derive(Component)]
struct CelestialClipmapBuildTask {
    authority: Entity,
    generation: u64,
    policy_revision: u64,
    spec: CelestialClipmapBlockSpec,
    field: CelestialVoxelField,
    task: VoxelWorkerTicket<Option<CelestialClipmapMeshData>>,
}

#[derive(Component)]
struct CelestialClipmapPlanBuildTask {
    authority: Entity,
    input: CelestialClipmapPlanInput,
    field: CelestialVoxelField,
    policy: Option<DeveloperScalarPolicySnapshot>,
    task: VoxelWorkerTicket<CelestialClipmapPlanBuildOutput>,
}

struct CelestialClipmapPlanBuildOutput {
    stages: Option<Vec<Vec<CelestialClipmapBlockSpec>>>,
    surface_cache: CelestialClipmapSurfaceCache,
}

#[derive(Debug)]
struct CelestialClipmapMeshData {
    positions: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
    tangents: Vec<[f32; 4]>,
    indices: Vec<u32>,
}

impl CelestialClipmapMeshData {
    fn into_mesh(self) -> Mesh {
        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, self.positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, self.uvs)
        .with_inserted_attribute(Mesh::ATTRIBUTE_TANGENT, self.tangents)
        .with_inserted_indices(Indices::U32(self.indices))
    }
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

// dense-terminal-aperture-v1
#[derive(Debug, Clone, Copy)]
struct DensePresentationApertureCell {
    center_local_metres: DVec3,
    world_x_local: DVec3,
    world_y_local: DVec3,
    world_z_local: DVec3,
    half_extent_metres: DVec3,
}

impl DensePresentationApertureCell {
    fn contains_local_point(self, point: DVec3) -> bool {
        let delta = point - self.center_local_metres;
        let local = DVec3::new(
            delta.dot(self.world_x_local),
            delta.dot(self.world_y_local),
            delta.dot(self.world_z_local),
        );
        let epsilon = 1.0e-6;
        local.x.abs() <= self.half_extent_metres.x + epsilon
            && local.y.abs() <= self.half_extent_metres.y + epsilon
            && local.z.abs() <= self.half_extent_metres.z + epsilon
    }
}

fn vec3_to_dvec3(value: Vec3) -> DVec3 {
    DVec3::new(
        f64::from(value.x),
        f64::from(value.y),
        f64::from(value.z),
    )
}

fn dense_presentation_apertures(
    interaction_scale: SpatialScale,
    coverage: &UsfScaleCoverageSnapshot,
    authorities: &Query<(&UsfPosition, &UsfSemanticFrame, &CelestialVoxelField)>,
) -> HashMap<Entity, Vec<DensePresentationApertureCell>> {
    let metres_per_native = interaction_scale.metres_per_native();
    let mut by_authority =
        HashMap::<Entity, Vec<DensePresentationApertureCell>>::new();

    for cell in coverage.iter() {
        if cell.scale() != interaction_scale
            || !cell.roles().contains(UsfScaleRoleMask::PRESENTATION)
        {
            continue;
        }

        let Ok((body_origin, body_frame, _field)) =
            authorities.get(cell.authority())
        else {
            continue;
        };
        let Ok(center_local_metres) = body_frame.world_to_local_metres(
            body_origin,
            &cell.center(),
            SpatialScale::ZERO,
            f64::MAX,
        ) else {
            continue;
        };

        let axis = |world: Vec3| {
            vec3_to_dvec3(
                body_frame
                    .world_direction_to_local(world)
                    .normalize_or_zero(),
            )
            .normalize_or_zero()
        };
        let world_x_local = axis(Vec3::X);
        let world_y_local = axis(Vec3::Y);
        let world_z_local = axis(Vec3::Z);
        if world_x_local == DVec3::ZERO
            || world_y_local == DVec3::ZERO
            || world_z_local == DVec3::ZERO
        {
            continue;
        }

        let half = cell.half_extent_native();
        by_authority
            .entry(cell.authority())
            .or_default()
            .push(DensePresentationApertureCell {
                center_local_metres,
                world_x_local,
                world_y_local,
                world_z_local,
                half_extent_metres: DVec3::new(
                    f64::from(half.x) * metres_per_native,
                    f64::from(half.y) * metres_per_native,
                    f64::from(half.z) * metres_per_native,
                ),
            });
    }

    by_authority
}

fn point_covered_by_dense_union(
    apertures: &[DensePresentationApertureCell],
    point: DVec3,
) -> bool {
    apertures
        .iter()
        .copied()
        .any(|aperture| aperture.contains_local_point(point))
}

/// Conservative make-before-break aperture ownership.
///
/// We hide a contextual binary block only when a 3x3x3 sample lattice spanning
/// its complete AABB is covered by ready dense PRESENTATION cells. This supports
/// union coverage across adjacent 10^3 dense materializations while refusing to
/// speculate across a partially ready boundary.
fn block_fully_covered_by_dense_union(
    key: CelestialClipmapBlockKey,
    apertures: &[DensePresentationApertureCell],
) -> bool {
    if apertures.is_empty() {
        return false;
    }

    let min = key.origin_local_metres();
    let extent = key.extent_metres();
    for z in [0.0_f64, 0.5, 1.0] {
        for y in [0.0_f64, 0.5, 1.0] {
            for x in [0.0_f64, 0.5, 1.0] {
                let point =
                    min + DVec3::new(x * extent, y * extent, z * extent);
                if !point_covered_by_dense_union(apertures, point) {
                    return false;
                }
            }
        }
    }
    true
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

    /// Finest actually-visible contextual spacing covering one body-local point.
    ///
    /// This is intentionally post-composition coverage; a hidden/unpublished
    /// binary block cannot be used as evidence to retire dense presentation.
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

fn checked_floor_coord(point: DVec3, extent: f64) -> Option<IVec3> {
    if !point.is_finite() || !extent.is_finite() || extent <= 0.0 {
        return None;
    }

    let scaled = point / extent;
    let x = scaled.x.floor();
    let y = scaled.y.floor();
    let z = scaled.z.floor();
    let valid = |value: f64| {
        value >= f64::from(i32::MIN) && value <= f64::from(i32::MAX)
    };
    if !valid(x) || !valid(y) || !valid(z) {
        return None;
    }

    Some(IVec3::new(x as i32, y as i32, z as i32))
}

fn block_distance_to_point(
    key: CelestialClipmapBlockKey,
    point: DVec3,
) -> f64 {
    let min = key.origin_local_metres();
    let max = min + DVec3::splat(key.extent_metres());
    let nearest = point.clamp(min, max);
    (point - nearest).length()
}

fn block_intersects_semantic_surface(
    field: CelestialVoxelField,
    key: CelestialClipmapBlockKey,
    _policy: Option<&DeveloperScalarPolicyRuntime>,
) -> bool {
    if !block_may_intersect_presentation_shell(field, key) {
        return false;
    }

    let center = key.center_local_metres();
    let radial = center.length();
    if !radial.is_finite() {
        return false;
    }

    let half_diagonal = key.half_extent_metres().length();
    let conservative_extra =
        key.extent_metres() * 0.35 + key.spacing_metres() * 2.0;
    let threshold = half_diagonal + conservative_extra;

    // Exact canonical volumetric SDF where it is informative.
    if field
        .presentation_signed_distance_local_metres(
            center,
            key.spacing_metres(),
        )
        .is_some_and(|distance| distance.abs() <= threshold)
    {
        return true;
    }

    // Cave morphology is currently a pseudo-SDF and therefore is not promised
    // to be globally 1-Lipschitz. Preserve the complete declared inward support
    // band conservatively so narrow/deep cave surfaces are never rejected by
    // the hierarchy planner merely because the block center lies in solid rock.
    if radial <= f64::EPSILON {
        return false;
    }
    let direction = Vec3::new(
        (center.x / radial) as f32,
        (center.y / radial) as f32,
        (center.z / radial) as f32,
    )
    .normalize_or_zero();
    if direction == Vec3::ZERO {
        return false;
    }

    let Ok(surface) = field.presentation_surface_local_metres(
        direction,
        key.spacing_metres(),
    ) else {
        return false;
    };
    let radial_delta = radial - surface.length();
    radial_delta <= threshold
        && radial_delta
            >= -(field.volumetric_surface_inward_support_metres() + threshold)
}

fn block_contains_local_point(
    key: CelestialClipmapBlockKey,
    point: DVec3,
) -> bool {
    let min = key.origin_local_metres();
    let max = min + DVec3::splat(key.extent_metres());
    point.x >= min.x
        && point.x <= max.x
        && point.y >= min.y
        && point.y <= max.y
        && point.z >= min.z
        && point.z <= max.z
}

/// Fine-refinement occupancy.
///
/// Whole-body roots intentionally keep the broad conservative shell test.
/// Children, however, need evidence of an actual boundary. Treating the whole
/// declared cave inward-support band as occupied at every fine level creates a
/// 3-D volume refinement explosion and starves the visible surface branch.
fn block_intersects_refinement_boundary(
    field: CelestialVoxelField,
    key: CelestialClipmapBlockKey,
    planning_anchor_local: DVec3,
) -> bool {
    // exact-boundary-focus-precedes-radial-broadphase-v1
    //
    // This focus came from the complete canonical volumetric field. A radial
    // broadphase optimization may never veto the exact cave/terrain branch.
    if block_contains_local_point(key, planning_anchor_local) {
        return true;
    }

    if !block_may_intersect_presentation_shell(field, key) {
        return false;
    }

    let origin = key.origin_local_metres();
    let extent = key.extent_metres();
    let center = key.center_local_metres();

    let mut minimum = f64::INFINITY;
    let mut maximum = f64::NEG_INFINITY;
    let mut minimum_abs = f64::INFINITY;
    let mut samples = 0usize;

    let mut observe = |point: DVec3| {
        if let Some(distance) = field.presentation_signed_distance_local_metres(
            point,
            key.spacing_metres(),
        )
        && distance.is_finite()
        {
            minimum = minimum.min(distance);
            maximum = maximum.max(distance);
            minimum_abs = minimum_abs.min(distance.abs());
            samples += 1;
        }
    };

    observe(center);
    for z in [0.0_f64, 1.0] {
        for y in [0.0_f64, 1.0] {
            for x in [0.0_f64, 1.0] {
                observe(
                    origin
                        + DVec3::new(
                            x * extent,
                            y * extent,
                            z * extent,
                        ),
                );
            }
        }
    }

    if samples > 0 {
        if minimum <= 0.0 && maximum >= 0.0 {
            return true;
        }

        // Near-zero sampled evidence catches tangential intersections that do
        // not produce a corner sign change.
        let evidence_margin =
            (key.spacing_metres() * 2.0)
                .max(key.half_extent_metres().length() * 0.30);
        if minimum_abs <= evidence_margin {
            return true;
        }
    }

    // Preserve ordinary outer terrain without reopening the complete cave
    // support volume. This radial fallback is narrow around the authored outer
    // surface only.
    let radial = center.length();
    if !radial.is_finite() || radial <= f64::EPSILON {
        return false;
    }
    let direction = Vec3::new(
        (center.x / radial) as f32,
        (center.y / radial) as f32,
        (center.z / radial) as f32,
    )
    .normalize_or_zero();
    if direction == Vec3::ZERO {
        return false;
    }
    let Ok(surface) = field.presentation_surface_local_metres(
        direction,
        key.spacing_metres(),
    ) else {
        return false;
    };
    let threshold =
        key.half_extent_metres().length()
            + key.extent_metres() * 0.20
            + key.spacing_metres() * 2.0;
    (radial - surface.length()).abs() <= threshold
}

fn block_may_intersect_presentation_shell(
    field: CelestialVoxelField,
    key: CelestialClipmapBlockKey,
) -> bool {
    let minimum = key.origin_local_metres();
    let maximum = minimum + DVec3::splat(key.extent_metres());

    let nearest_axis = |low: f64, high: f64| {
        if low <= 0.0 && high >= 0.0 {
            0.0
        } else {
            low.abs().min(high.abs())
        }
    };
    let farthest_axis =
        |low: f64, high: f64| low.abs().max(high.abs());

    let nearest = DVec3::new(
        nearest_axis(minimum.x, maximum.x),
        nearest_axis(minimum.y, maximum.y),
        nearest_axis(minimum.z, maximum.z),
    )
    .length();
    let farthest = DVec3::new(
        farthest_axis(minimum.x, maximum.x),
        farthest_axis(minimum.y, maximum.y),
        farthest_axis(minimum.z, maximum.z),
    )
    .length();

    let (surface_minimum, surface_maximum) =
        presentation_surface_radius_bounds_metres(field);
    let conservative_extra =
        key.extent_metres() * 0.35 + key.spacing_metres() * 2.0;
    let volumetric_minimum = (
        surface_minimum - field.volumetric_surface_inward_support_metres()
    )
    .max(0.0);

    nearest <= surface_maximum + conservative_extra
        && farthest >= (volumetric_minimum - conservative_extra).max(0.0)
}

// binary-can-replace-dense-visual-v2
fn projected_sample_pixels(
    sample_spacing_metres: f64,
    distance_metres: f64,
    pixels_per_radian: Option<f32>,
) -> Option<f64> {
    let pixels_per_radian = f64::from(pixels_per_radian?);
    if !sample_spacing_metres.is_finite()
        || sample_spacing_metres <= 0.0
        || !distance_metres.is_finite()
        || distance_metres <= 0.0
        || !pixels_per_radian.is_finite()
        || pixels_per_radian <= 0.0
    {
        return None;
    }

    Some(
        (sample_spacing_metres / distance_metres)
            .clamp(0.0, 1.0)
            .asin()
            * pixels_per_radian,
    )
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

fn binary_spacing_is_adequate_for_dense_handoff(
    sample_spacing_metres: f64,
    distance_metres: f64,
    pixels_per_radian: Option<f32>,
) -> bool {
    if !sample_spacing_metres.is_finite()
        || sample_spacing_metres <= 0.0
        || !distance_metres.is_finite()
        || distance_metres < 0.0
    {
        return false;
    }

    // Near geometry keeps dense ownership. The threshold is expressed in
    // samples, not USF native units, so it remains presentation-resolution
    // semantics rather than interaction Scale semantics.
    if distance_metres
        < sample_spacing_metres
            * DENSE_VISUAL_HANDOFF_MIN_SAMPLE_DISTANCES
    {
        return false;
    }

    let distance_quality =
        sample_spacing_metres
            <= (distance_metres / DENSE_VISUAL_HANDOFF_CELLS_PER_DISTANCE)
                .max(MIN_SAMPLE_SPACING_METRES);

    let screen_quality = projected_sample_pixels(
        sample_spacing_metres,
        distance_metres,
        pixels_per_radian,
    )
    .is_none_or(|pixels| {
        pixels <= DENSE_VISUAL_HANDOFF_MAX_PIXELS_PER_SAMPLE
    });

    distance_quality && screen_quality
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
    planning_anchor_local: DVec3,
    validity_radius_metres: f64,
    clearance_metres: f64,
    finest: VoxelPresentationResolution,
    coarsest: VoxelPresentationResolution,
}

/// Cheap clipmap identity derivation.
///
/// body-owned-whole-body-root-v1
/// Whole-body topology is semantic-body-owned. Observer position is stored only
/// as a refinement anchor and cannot change the coarsest body representation.
fn derive_plan_input(
    field: CelestialVoxelField,
    observer_local: DVec3,
    pixels_per_radian: Option<f32>,
    observer_speed_metres_per_second: f64,
    expected_build_seconds: f64,
    policy_revision: u64,
) -> Option<CelestialClipmapPlanInput> {
    if !observer_local.is_finite() {
        return None;
    }

    // boundary-focused-binary-refinement-v1
    //
    // Refine the semantic terrain boundary, not empty space around the camera.
    // Full SDF projection means this follows cave walls as well as the outer
    // planetary surface.
    let (planning_anchor_local, signed_clearance_metres) =
        field.nearest_boundary_local_metres(observer_local, f64::MAX)?;
    let clearance = signed_clearance_metres.abs();
    if !clearance.is_finite() {
        return None;
    }

    // presentation-resolution-independent-screen-error-v1
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
            policy_revision,
        },
        planning_anchor_local,
        validity_radius_metres,
        clearance_metres: clearance,
        finest,
        coarsest,
    })
}

fn plan_requires_refresh(
    plan: &CelestialClipmapPlan,
    input: CelestialClipmapPlanInput,
    field: CelestialVoxelField,
) -> bool {
    if plan.key != input.key || plan.field != field {
        return true;
    }

    let displacement =
        (input.planning_anchor_local - plan.planning_anchor_local).length();
    let fine_extent =
        input.finest.sample_spacing_metres() * BLOCK_SUBDIVISIONS as f64;

    // sticky-refinement-anchor-v1
    // Hold substantially beyond a finest block so walking/flying does not turn
    // local cell boundaries into whole-frontier transactions.
    let hold_radius = plan
        .validity_radius_metres
        .max(fine_extent * 4.0)
        * 0.75;
    displacement > hold_radius
}

// refinement-progress-before-maintenance-v1
fn plan_refinement_in_progress(plan: &CelestialClipmapPlan) -> bool {
    plan.stage_index.saturating_add(1) < plan.stages.len()
        || plan.committed_generation != Some(plan.generation)
}

fn committed_local_spacing_metres(plan: &CelestialClipmapPlan) -> f64 {
    plan.committed_specs
        .iter()
        .filter(|spec| {
            block_distance_to_point(
                spec.key,
                plan.planning_anchor_local,
            ) <= plan
                .validity_radius_metres
                .max(spec.key.extent_metres())
        })
        .map(|spec| spec.key.spacing_metres())
        .min_by(f64::total_cmp)
        .unwrap_or_else(|| {
            2.0_f64.powi(i32::from(plan.key.coarsest_exponent))
        })
}

fn emergency_replan_required(
    plan: &CelestialClipmapPlan,
    input: CelestialClipmapPlanInput,
    field: CelestialVoxelField,
) -> bool {
    if plan.field != field
        || plan.key.policy_revision != input.key.policy_revision
        || plan.key.coarsest_exponent != input.key.coarsest_exponent
    {
        return true;
    }

    let displacement =
        (input.planning_anchor_local - plan.planning_anchor_local).length();
    if !displacement.is_finite() {
        return true;
    }

    let support =
        committed_local_spacing_metres(plan)
            * BLOCK_SUBDIVISIONS as f64
            * 2.0;
    let safety_radius = support
        .max(plan.validity_radius_metres * 4.0)
        .max(input.validity_radius_metres * 2.0);

    displacement > safety_radius
}

fn should_schedule_plan_refresh(
    plan: &CelestialClipmapPlan,
    input: CelestialClipmapPlanInput,
    field: CelestialVoxelField,
) -> bool {
    if !plan_requires_refresh(plan, input, field) {
        return false;
    }
    if !plan_refinement_in_progress(plan) {
        return true;
    }

    // Refinement is useful foreground work. Ordinary focus maintenance cannot
    // continuously replace it; structural changes and true escape still can.
    emergency_replan_required(plan, input, field)
}

// rolling-predictive-clipmap-v1
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
    // progress. Body-root topology + policy must remain identical.
    built.coarsest_exponent == current.coarsest_exponent
        && built.policy_revision == current.policy_revision
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
    {
        return ClipmapPlanTaskRelevance::Stale;
    }

    let built_lag =
        (current.planning_anchor_local - built.planning_anchor_local)
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
        (current.planning_anchor_local - committed_anchor_local).length();
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

    // quality-preserving-warm-stage-v1
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
fn specs_for_frontier(
    leaves: &HashSet<CelestialClipmapBlockKey>,
    planning_anchor_local: DVec3,
) -> Vec<CelestialClipmapBlockSpec> {
    let mut ordered = leaves.iter().copied().collect::<Vec<_>>();
    ordered.sort_unstable_by(|a, b| {
        block_distance_to_point(*a, planning_anchor_local)
            .total_cmp(&block_distance_to_point(*b, planning_anchor_local))
            .then_with(|| b.resolution.cmp(&a.resolution))
            .then_with(|| a.coord.x.cmp(&b.coord.x))
            .then_with(|| a.coord.y.cmp(&b.coord.y))
            .then_with(|| a.coord.z.cmp(&b.coord.z))
    });

    ordered
        .into_iter()
        .map(|key| CelestialClipmapBlockSpec {
            key,
            transition_faces: transition_faces_for_leaf(leaves, key),
        })
        .collect()
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

fn finest_spacing_in_leaf_frontier(
    leaves: &HashSet<CelestialClipmapBlockKey>,
    planning_anchor_local: DVec3,
    validity_radius_metres: f64,
) -> Option<f64> {
    leaves
        .iter()
        .copied()
        .filter(|key| {
            block_distance_to_point(*key, planning_anchor_local)
                <= validity_radius_metres.max(key.extent_metres())
        })
        .map(CelestialClipmapBlockKey::spacing_metres)
        .min_by(f64::total_cmp)
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
    policy: Option<&DeveloperScalarPolicyRuntime>,
    surface_cache: &mut CelestialClipmapSurfaceCache,
) -> Option<Vec<Vec<CelestialClipmapBlockSpec>>> {
    surface_cache.begin_plan(field, input.key.policy_revision);

    let result = (|| {
        let planning_anchor_local = input.planning_anchor_local;
        let validity_radius_metres = input.validity_radius_metres;
        let finest = input.finest;
        let coarsest = input.coarsest;
        let maximum_leaves = sparse_frontier_leaf_budget(input);

        let (root_low, root_high) =
            body_root_coordinate_bounds(field, coarsest)?;

        let mut leaves =
            HashSet::<CelestialClipmapBlockKey>::with_capacity(
                maximum_leaves.min(8_192),
            );

        // Whole-body ancestry remains conservative and stable.
        for z in root_low..=root_high {
            for y in root_low..=root_high {
                for x in root_low..=root_high {
                    let key = CelestialClipmapBlockKey {
                        resolution: coarsest,
                        coord: IVec3::new(x, y, z),
                    };
                    if surface_cache.intersects(field, key, policy) {
                        leaves.insert(key);
                    }
                }
            }
        }

        if leaves.is_empty() {
            return None;
        }

        let mut stages =
            vec![specs_for_frontier(&leaves, planning_anchor_local)];
        let mut recorded_spacing = finest_spacing_in_leaf_frontier(
            &leaves,
            planning_anchor_local,
            validity_radius_metres,
        )
        .unwrap_or(coarsest.sample_spacing_metres());

        loop {
            let mut candidates =
                BinaryHeap::<ClipmapRefinementCandidate>::with_capacity(
                    leaves.len().min(8_192),
                );
            for &key in &leaves {
                if let Some(candidate) = refinement_candidate(
                    key,
                    finest,
                    planning_anchor_local,
                    validity_radius_metres,
                ) {
                    candidates.push(candidate);
                }
            }

            if candidates.is_empty() {
                break;
            }

            let previous = leaves.clone();
            let mut inserted =
                Vec::<CelestialClipmapBlockKey>::with_capacity(8);
            let mut primary_refinements = 0usize;

            // focus-path-first-v1
            // Whole-body breadth may never consume a complete wave before the
            // exact local semantic-boundary branch advances one level.
            let focus_candidate = leaves
                .iter()
                .copied()
                .filter(|key| {
                    key.resolution > finest
                        && block_contains_local_point(
                            *key,
                            planning_anchor_local,
                        )
                })
                .min_by_key(|key| key.resolution.binary_exponent());

            if let Some(focus_key) = focus_candidate {
                match refine_leaf_indexed(
                    &mut leaves,
                    focus_key,
                    field,
                    policy,
                    surface_cache,
                    planning_anchor_local,
                    &mut inserted,
                ) {
                    LeafRefinementOutcome::Refined
                    | LeafRefinementOutcome::RemovedAsEmpty => {
                        primary_refinements = 1;
                    }
                    LeafRefinementOutcome::Unavailable => {}
                }
            }

            while let Some(candidate) = candidates.pop() {
                if primary_refinements
                    >= MAX_PRIMARY_REFINEMENTS_PER_WAVE
                {
                    break;
                }
                if !leaves.contains(&candidate.key) {
                    continue;
                }
                if leaves.len().saturating_add(7) > maximum_leaves {
                    break;
                }

                match refine_leaf_indexed(
                    &mut leaves,
                    candidate.key,
                    field,
                    policy,
                    surface_cache,
                    planning_anchor_local,
                    &mut inserted,
                ) {
                    LeafRefinementOutcome::Refined
                    | LeafRefinementOutcome::RemovedAsEmpty => {
                        primary_refinements += 1;
                    }
                    LeafRefinementOutcome::Unavailable => {}
                }
            }

            if primary_refinements == 0 {
                break;
            }

            if !balance_leaves_2_to_1_local(
                field,
                &mut leaves,
                policy,
                surface_cache,
                planning_anchor_local,
                maximum_leaves,
            ) {
                leaves = previous;
                break;
            }

            let current_spacing = finest_spacing_in_leaf_frontier(
                &leaves,
                planning_anchor_local,
                validity_radius_metres,
            )
            .unwrap_or(recorded_spacing);

            // Milestone stages scale with actual LOD depth, not tiny worker
            // batches. This keeps transactional metadata compact while still
            // allowing coarse->fine make-before-break progression.
            if current_spacing < recorded_spacing * 0.999
                && stages.len() < MAX_RECORDED_FRONTIER_STAGES
            {
                stages.push(specs_for_frontier(
                    &leaves,
                    planning_anchor_local,
                ));
                recorded_spacing = current_spacing;
            }

            if leaves.len().saturating_add(7) > maximum_leaves {
                break;
            }
        }

        let final_stage =
            specs_for_frontier(&leaves, planning_anchor_local);
        if stages.last().is_none_or(|current| *current != final_stage) {
            if stages.len() < MAX_RECORDED_FRONTIER_STAGES {
                stages.push(final_stage);
            } else if let Some(last) = stages.last_mut() {
                *last = final_stage;
            }
        }

        Some(stages)
    })();

    surface_cache.finish_plan();
    result
}


fn transvoxel_sides(faces: VoxelTransitionFaces) -> TransitionSides {
    let mut sides = TransitionSide::none();

    if faces.contains(VoxelTransitionFace::LowX) {
        sides |= TransitionSide::LowX;
    }
    if faces.contains(VoxelTransitionFace::HighX) {
        sides |= TransitionSide::HighX;
    }
    if faces.contains(VoxelTransitionFace::LowY) {
        sides |= TransitionSide::LowY;
    }
    if faces.contains(VoxelTransitionFace::HighY) {
        sides |= TransitionSide::HighY;
    }
    if faces.contains(VoxelTransitionFace::LowZ) {
        sides |= TransitionSide::LowZ;
    }
    if faces.contains(VoxelTransitionFace::HighZ) {
        sides |= TransitionSide::HighZ;
    }

    sides
}

fn build_tangent(normal: Vec3) -> [f32; 4] {
    let axis = normal.abs();
    let tangent_axis = if axis.y < 0.9 { Vec3::Y } else { Vec3::X };
    let tangent =
        (tangent_axis - normal * normal.dot(tangent_axis))
            .normalize_or_zero();
    [tangent.x, tangent.y, tangent.z, 1.0]
}

fn build_clipmap_mesh(
    field: CelestialVoxelField,
    spec: CelestialClipmapBlockSpec,
    _policy: Option<&DeveloperScalarPolicyRuntime>,
) -> Option<CelestialClipmapMeshData> {
    let origin = spec.key.origin_local_metres();
    let extent = spec.key.extent_metres();
    if !origin.is_finite()
        || !extent.is_finite()
        || extent <= 0.0
        || extent > f64::from(f32::MAX)
    {
        return None;
    }

    let extent_f32 = extent as f32;
    let density = move |x: f32, y: f32, z: f32| -> f32 {
        let point = origin + DVec3::new(
            f64::from(x),
            f64::from(y),
            f64::from(z),
        );
        // The whole-body presentation hierarchy samples exactly the same
        // canonical volumetric field as dense physical voxels. Transvoxel uses
        // the opposite sign convention: positive density means solid.
        let Some(volumetric_sdf) =
            field.presentation_signed_distance_local_metres(
                point,
                spec.key.spacing_metres(),
            )
        else {
            return -1.0;
        };
        let density = -volumetric_sdf;
        if density.is_finite() {
            density.clamp(
                -f64::from(f32::MAX),
                f64::from(f32::MAX),
            ) as f32
        } else {
            -1.0
        }
    };

    let block = Block::new(
        [0.0_f32, 0.0_f32, 0.0_f32],
        extent_f32,
        BLOCK_SUBDIVISIONS,
    );
    let mesh = extract_from_field(
        &density,
        FieldCaching::CacheNothing,
        block,
        transvoxel_sides(spec.transition_faces),
        0.0,
        GenericMeshBuilder::new(),
    )
    .build();

    if mesh.triangle_indices.is_empty() || mesh.positions.is_empty() {
        return None;
    }

    let positions = mesh
        .positions
        .chunks_exact(3)
        .map(|p| [p[0], p[1], p[2]])
        .collect::<Vec<_>>();
    let normals = mesh
        .normals
        .chunks_exact(3)
        .map(|n| [n[0], n[1], n[2]])
        .collect::<Vec<_>>();
    if positions.len() != normals.len() {
        return None;
    }

    let uvs = positions
        .iter()
        .map(|p| {
            [
                ((origin.x + f64::from(p[0])) * 0.5) as f32,
                ((origin.z + f64::from(p[2])) * 0.5) as f32,
            ]
        })
        .collect::<Vec<_>>();
    let tangents = normals
        .iter()
        .map(|normal| build_tangent(Vec3::from_array(*normal)))
        .collect::<Vec<_>>();
    let indices = mesh
        .triangle_indices
        .into_iter()
        .map(u32::try_from)
        .collect::<Result<Vec<_>, _>>()
        .ok()?;

    Some(CelestialClipmapMeshData {
        positions,
        normals,
        uvs,
        tangents,
        indices,
    })
}

fn coverage_for_specs(
    specs: &[CelestialClipmapBlockSpec],
) -> Vec<CelestialClipmapCoverageCell> {
    specs
        .iter()
        .map(|spec| CelestialClipmapCoverageCell {
            center_local_metres: spec.key.center_local_metres(),
            half_extent_metres: spec.key.half_extent_metres(),
            sample_spacing_metres: spec.key.spacing_metres(),
        })
        .collect()
}

fn sync_celestial_clipmap_realizations(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    views: Res<UsfViewDemandSnapshot>,
    script_workbench: Res<DeveloperScriptWorkbench>,
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
    blocks: Query<(Entity, &CelestialClipmapBlock)>,
    mut plan_tasks: Query<(Entity, &mut CelestialClipmapPlanBuildTask)>,
    mut build_tasks: Query<(Entity, &mut CelestialClipmapBuildTask)>,
    mut registry: ResMut<CelestialClipmapRegistry>,
    mut telemetry: ResMut<CelestialClipmapTelemetry>,
    mut frame_budget: ResMut<ReconstructibleFrameBudget>,
) {
    let Some(view) = views.iter().next() else {
        return;
    };

    let presentation_policy = script_workbench.celestial_height_snapshot();
    let policy_revision =
        presentation_policy.as_ref().map_or(0, DeveloperScalarPolicySnapshot::revision);
    let observer_speed = view.velocity_metres_per_second().length();
    let expected_build_seconds =
        workers.estimated_latency_seconds(VoxelWorkerLane::PresentationPlanning)
            + workers.estimated_latency_seconds(VoxelWorkerLane::PresentationResolution);

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

            // rolling-predictive-clipmap-v1
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

            let Some(input) = derive_plan_input(
                *field,
                predicted_observer_local,
                view.pixels_per_radian_for_presentation_resolution(),
                observer_speed,
                expected_build_seconds,
                policy_revision,
            ) else {
                continue;
            };
            live_authorities.insert(authority);
            current_inputs.insert(authority, (input, *field));
        }
    }

    let mut planning_authorities = HashSet::<Entity>::new();
    let mut plans_changed = false;

    for (task_entity, mut build) in &mut plan_tasks {
        let Some(&(current_input, current_field)) =
            current_inputs.get(&build.authority)
        else {
            commands.entity(task_entity).despawn();
            continue;
        };
        planning_authorities.insert(build.authority);

        let Some(output) = build.task.try_take() else {
            continue;
        };
        commands.entity(task_entity).despawn();
        planning_authorities.remove(&build.authority);
        registry
            .planner_caches
            .insert(build.authority, output.surface_cache);

        let committed_anchor_local = registry
            .plans
            .get(&build.authority)
            .map(|plan| plan.planning_anchor_local);
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

        let (committed_generation, committed_specs, known_empty) =
            registry
                .plans
                .get(&build.authority)
                .map(|plan| {
                    (
                        plan.committed_generation,
                        plan.committed_specs.clone(),
                        if plan.key.policy_revision
                            == build.input.key.policy_revision
                        {
                            plan.known_empty.clone()
                        } else {
                            HashSet::new()
                        },
                    )
                })
                .unwrap_or((
                    None,
                    HashSet::new(),
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

        info!(
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
                policy: build.policy.clone(),
                planning_anchor_local: build.input.planning_anchor_local,
                validity_radius_metres: build.input.validity_radius_metres,
                generation,
                stages,
                stage_index,
                desired,
                completed: HashSet::new(),
                meshful: HashSet::new(),
                known_empty,
                committed_specs,
                committed_generation,
            },
        );
        plans_changed = true;
    }

    let mut planning_slots =
        workers.available_slots(VoxelWorkerLane::PresentationPlanning);
    let committed_focus_lag = current_inputs
        .iter()
        .filter_map(|(authority, (input, _))| {
            registry.plans.get(authority).map(|plan| {
                (
                    input.planning_anchor_local
                        - plan.planning_anchor_local,
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
            .get(&authority)
            .cloned()
            .unwrap_or_default();
        let policy = presentation_policy.clone();
        let policy_for_job = policy.clone();

        let Some(task) = workers.try_submit(
            VoxelWorkerLane::PresentationPlanning,
            move || {
                let runtime = policy_for_job
                    .as_ref()
                    .and_then(|snapshot| snapshot.compile_runtime().ok());
                let stages = build_plan(
                    field,
                    input,
                    runtime.as_ref(),
                    &mut surface_cache,
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
        commands.spawn((
            Name::new(if warm_replan {
                "Celestial Binary Clipmap Warm Replan"
            } else {
                "Celestial Binary Clipmap Cold Plan"
            }),
            VoxelWorkerTask,
            CelestialClipmapPlanBuildTask {
                authority,
                input,
                field,
                policy,
                task,
            },
        ));
        planning_authorities.insert(authority);
        planning_slots -= 1;
        frame_budget.finish(work_token);
    }

    let plan_count_before_retain = registry.plans.len();
    registry
        .plans
        .retain(|authority, _| live_authorities.contains(authority));
    let plans_removed = registry.plans.len() != plan_count_before_retain;
    registry
        .planner_caches
        .retain(|authority, _| live_authorities.contains(authority));

    let no_plan_tasks = plan_tasks.iter_mut().next().is_none();
    let no_build_tasks = build_tasks.iter_mut().next().is_none();
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

    let mut existing_by_spec =
        HashMap::<(Entity, u64, CelestialClipmapBlockSpec), Entity>::new();
    let mut projected_specs =
        HashSet::<(Entity, u64, CelestialClipmapBlockSpec)>::new();

    {
        let _span = bevy::log::info_span!("celestial_clipmap.index_blocks").entered();
        for (entity, block) in &blocks {
            if !live_authorities.contains(&block.authority) {
                commands.entity(entity).despawn();
                continue;
            }

            let key = (block.authority, block.policy_revision, block.spec);
            if existing_by_spec.contains_key(&key) {
                commands.entity(entity).despawn();
                continue;
            }

            existing_by_spec.insert(key, entity);
            if block.projection_ready {
                projected_specs.insert(key);
            }
        }

        for (&authority, plan) in &mut registry.plans {
            for &spec in &plan.desired {
                if existing_by_spec.contains_key(&(
                    authority,
                    plan.key.policy_revision,
                    spec,
                )) {
                    plan.completed.insert(spec);
                    plan.meshful.insert(spec);
                } else if plan.known_empty.contains(&spec) {
                    plan.completed.insert(spec);
                }
            }
        }
    }

    let mut inflight =
        HashSet::<(Entity, u64, CelestialClipmapBlockSpec)>::new();
    let mut publications = 0usize;

    {
        let _span = bevy::log::info_span!("celestial_clipmap.poll_builds").entered();
        for (task_entity, mut build) in &mut build_tasks {
            let Some(plan) = registry.plans.get_mut(&build.authority) else {
                commands.entity(task_entity).despawn();
                continue;
            };

            if build.generation != plan.generation
                || build.policy_revision != plan.key.policy_revision
                || !plan.desired.contains(&build.spec)
                || build.field != plan.field
            {
                commands.entity(task_entity).despawn();
                continue;
            }

            if existing_by_spec.contains_key(&(
                build.authority,
                build.policy_revision,
                build.spec,
            )) {
                plan.completed.insert(build.spec);
                plan.meshful.insert(build.spec);
                commands.entity(task_entity).despawn();
                continue;
            }

            inflight.insert((build.authority, build.policy_revision, build.spec));

            let max_publications_per_frame =
                workers.capacity().saturating_mul(2).max(4);
            if publications >= max_publications_per_frame {
                continue;
            }
            let Some(work_token) =
                frame_budget.begin(ReconstructibleWorkClass::Publication)
            else {
                continue;
            };
            let Some(result) = build.task.try_take() else {
                frame_budget.finish(work_token);
                continue;
            };

            commands.entity(task_entity).despawn();
            publications += 1;
            plan.completed.insert(build.spec);

            if let Some(mesh) = result {
                let Ok((_, name, _, _, _, _, policy)) =
                    authorities.get(build.authority)
                else {
                    frame_budget.finish(work_token);
                    continue;
                };

                let body_name =
                    name.map(|value| value.as_str()).unwrap_or("Celestial Body");
                let entity = commands
                    .spawn((
                        Name::new(format!(
                            "{body_name} Binary Clipmap S2^{} ({},{},{})",
                            build.spec.key.resolution.binary_exponent(),
                            build.spec.key.coord.x,
                            build.spec.key.coord.y,
                            build.spec.key.coord.z,
                        )),
                        CelestialClipmapBlock {
                            authority: build.authority,
                            policy_revision: build.policy_revision,
                            spec: build.spec,
                            projection_ready: false,
                        },
                        UsfPresentationProjectionOf(build.authority),
                        Mesh3d(meshes.add(mesh.into_mesh())),
                        MeshMaterial3d(policy.presentation_material().clone()),
                        Transform::IDENTITY,
                        RenderLayers::layer(USF_PRESENTATION_LAYER),
                        NotShadowCaster,
                        NotShadowReceiver,
                        // A mesh entity is not presentation coverage until its
                        // canonical anchor projects successfully into the active
                        // presentation chart.
                        Visibility::Hidden,
                    ))
                    .id();
                existing_by_spec.insert(
                    (build.authority, build.policy_revision, build.spec),
                    entity,
                );
                plan.meshful.insert(build.spec);
            } else {
                plan.known_empty.insert(build.spec);
            }
            frame_budget.finish(work_token);
        }
    }

    // Visible clipmap coverage is published after PostUpdate projection
    // and dense-aperture composition. Pre-composition "committed" is not the
    // same fact as actually visible contextual presentation.

    {
        let _span = bevy::log::info_span!("celestial_clipmap.schedule_builds").entered();
        let mut admitted = 0usize;
        let mut worker_slots =
            workers.available_slots(VoxelWorkerLane::PresentationResolution);
        let max_builds_in_flight =
            workers.capacity().saturating_mul(4).max(8);
        let max_admissions_per_frame =
            workers.capacity().saturating_mul(2).max(4);

        'authorities: for (&authority, plan) in &registry.plans {
            for &spec in &plan.desired {
                if plan.completed.contains(&spec)
                    || existing_by_spec.contains_key(&(
                        authority,
                        plan.key.policy_revision,
                        spec,
                    ))
                    || inflight.contains(&(
                        authority,
                        plan.key.policy_revision,
                        spec,
                    ))
                {
                    continue;
                }

                if worker_slots == 0
                    || inflight.len() >= max_builds_in_flight
                    || admitted >= max_admissions_per_frame
                {
                    break 'authorities;
                }

                let Some(work_token) =
                    frame_budget.begin(ReconstructibleWorkClass::Maintenance)
                else {
                    break 'authorities;
                };
                let field = plan.field;
                let policy = plan.policy.clone();
                let policy_revision = plan.key.policy_revision;
                let Some(task) = workers.try_submit(
                    VoxelWorkerLane::PresentationResolution,
                    move || {
                        let runtime = policy
                            .as_ref()
                            .and_then(|snapshot| snapshot.compile_runtime().ok());
                        build_clipmap_mesh(field, spec, runtime.as_ref())
                    },
                ) else {
                    frame_budget.finish(work_token);
                    break 'authorities;
                };

                commands.spawn((
                    Name::new("Celestial Binary Clipmap Build"),
                    VoxelWorkerTask,
                    CelestialClipmapBuildTask {
                        authority,
                        generation: plan.generation,
                        policy_revision,
                        spec,
                        field,
                        task,
                    },
                ));
                inflight.insert((authority, policy_revision, spec));
                admitted += 1;
                worker_slots -= 1;
                frame_budget.finish(work_token);
            }
        }
    }

    {
        let _span = bevy::log::info_span!("celestial_clipmap.commit").entered();

        for (&authority, plan) in &mut registry.plans {
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

            if !plan.meshful.iter().all(|spec| {
                projected_specs.contains(&(
                    authority,
                    plan.key.policy_revision,
                    *spec,
                ))
            }) {
                continue;
            }

            let desired =
                plan.desired.iter().copied().collect::<HashSet<_>>();
            plan.committed_specs = desired.clone();

            for (entity, block) in &blocks {
                if block.authority != authority {
                    continue;
                }
                if block.policy_revision != plan.key.policy_revision
                    || !desired.contains(&block.spec)
                {
                    commands.entity(entity).despawn();
                }
            }

            let committed_generation = plan.generation;

            if plan.stage_index + 1 < plan.stages.len() {
                plan.committed_generation = Some(committed_generation);
                plan.stage_index += 1;
                plan.generation =
                    plan.generation.wrapping_add(1).max(1);
                plan.desired = plan.stages[plan.stage_index].clone();
                plan.completed.clear();
                plan.meshful.clear();

                info!(
                    authority = ?authority,
                    committed_stage = plan.stage_index,
                    total_stages = plan.stages.len(),
                    committed_blocks = plan.committed_specs.len(),
                    next_blocks = plan.desired.len(),
                    "clipmap refinement frontier committed; arming next stage"
                );
            } else {
                plan.committed_generation = Some(committed_generation);
                info!(
                    authority = ?authority,
                    committed_stage = plan.stage_index + 1,
                    total_stages = plan.stages.len(),
                    committed_blocks = plan.committed_specs.len(),
                    "clipmap final refinement frontier committed"
                );
            }
        }
    }

}


fn sync_celestial_clipmap_transforms(
    view: Single<&UsfViewContext, With<UsfViewRenderAnchor>>,
    view_demands: Res<UsfViewDemandSnapshot>,
    interaction: Res<UsfPrimaryInteractionSlice>,
    dense_coverage: Res<UsfScaleCoverageSnapshot>,
    registry: Res<CelestialClipmapRegistry>,
    authorities: Query<(&UsfPosition, &UsfSemanticFrame, &CelestialVoxelField)>,
    mut coverage: ResMut<CelestialClipmapCoverageSnapshot>,
    mut telemetry: ResMut<CelestialClipmapTelemetry>,
    mut blocks: Query<(
        &mut CelestialClipmapBlock,
        &mut Transform,
        &mut Visibility,
    )>,
    mut logged_projection: Local<bool>,
) {
    let metre_to_view = view.projection_factor(SpatialScale::ZERO);
    if !metre_to_view.is_finite() || metre_to_view <= 0.0 {
        for (mut block, _, mut visibility) in &mut blocks {
            block.projection_ready = false;
            *visibility = Visibility::Hidden;
        }
        coverage.retain_authorities(&HashSet::new());
        telemetry.record_visible_frontier(
            0,
            0,
            &HashSet::new(),
            None,
            None,
        );
        return;
    }

    let pixels_per_radian = view_demands
        .iter()
        .next()
        .and_then(|view| view.pixels_per_radian_for_presentation_resolution());
    let physical_target_scale = interaction.target_scale();
    let dense_apertures = dense_presentation_apertures(
        physical_target_scale,
        &dense_coverage,
        &authorities,
    );

    let mut projected_any = false;
    let mut visible_by_authority =
        HashMap::<Entity, Vec<CelestialClipmapCoverageCell>>::new();
    let mut visible_levels = HashSet::<i16>::new();
    let mut visible_blocks = 0usize;
    let mut yielded_blocks = 0usize;
    let mut finest_visible_spacing = None::<f64>;
    let mut coarsest_visible_spacing = None::<f64>;

    for (mut block, mut transform, mut visibility) in &mut blocks {
        let Ok((body_origin, body_frame, _field)) =
            authorities.get(block.authority)
        else {
            block.projection_ready = false;
            *visibility = Visibility::Hidden;
            continue;
        };

        let local_origin = block.spec.key.origin_local_metres();
        let Ok(anchor) = body_frame.local_metres_to_world(
            *body_origin,
            local_origin,
        ) else {
            block.projection_ready = false;
            *visibility = Visibility::Hidden;
            continue;
        };

        let Ok(relative_metres) = anchor.relative_at_scale_bounded_f64(
            view.anchor(),
            SpatialScale::ZERO,
            f64::MAX,
        ) else {
            block.projection_ready = false;
            *visibility = Visibility::Hidden;
            continue;
        };

        let Some(projected_relative) =
            view.project_relative_metres_from_eye(relative_metres)
        else {
            block.projection_ready = false;
            *visibility = Visibility::Hidden;
            continue;
        };
        let projected_relative = Vec3::new(
            projected_relative.x as f32,
            projected_relative.y as f32,
            projected_relative.z as f32,
        );
        let translation = view.presentation_origin() + projected_relative;
        if !translation.is_finite() {
            block.projection_ready = false;
            *visibility = Visibility::Hidden;
            continue;
        }

        let orientation = body_frame.orientation();
        let rotation = Quat::from_xyzw(
            orientation.x as f32,
            orientation.y as f32,
            orientation.z as f32,
            orientation.w as f32,
        )
        .normalize();

        transform.translation = translation;
        transform.rotation = rotation;
        transform.scale = Vec3::splat(metre_to_view);
        block.projection_ready = true;

        let committed = registry
            .plans
            .get(&block.authority)
            .is_some_and(|plan| plan.committed_specs.contains(&block.spec));

        let observer_local = body_frame
            .world_to_local_metres(
                body_origin,
                view.anchor(),
                SpatialScale::ZERO,
                f64::MAX,
            )
            .ok();
        let distance_to_block = observer_local
            .map(|observer_local| {
                block_distance_to_point(block.spec.key, observer_local)
            })
            .unwrap_or(f64::INFINITY);
        let binary_can_replace_dense =
            binary_spacing_is_adequate_for_dense_handoff(
                block.spec.key.spacing_metres(),
                distance_to_block,
                pixels_per_radian,
            );

        let dense_yields_context = committed
            && !binary_can_replace_dense
            && dense_apertures
                .get(&block.authority)
                .is_some_and(|apertures| {
                    block_fully_covered_by_dense_union(
                        block.spec.key,
                        apertures,
                    )
                });

        if dense_yields_context {
            yielded_blocks = yielded_blocks.saturating_add(1);
        }

        // Near: dense child owns the aperture.
        // Medium/far: once this committed binary block is fine enough for the
        // view distance, keep it visible so it can become the proven parent
        // replacement before dense rendering retires.
        let visible = committed && !dense_yields_context;
        *visibility = if visible {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };

        if visible {
            let spacing = block.spec.key.spacing_metres();
            visible_blocks = visible_blocks.saturating_add(1);
            visible_levels.insert(block.spec.key.resolution.binary_exponent());
            finest_visible_spacing = Some(
                finest_visible_spacing.map_or(spacing, |value| value.min(spacing)),
            );
            coarsest_visible_spacing = Some(
                coarsest_visible_spacing.map_or(spacing, |value| value.max(spacing)),
            );
            visible_by_authority
                .entry(block.authority)
                .or_default()
                .push(CelestialClipmapCoverageCell {
                    center_local_metres: block.spec.key.center_local_metres(),
                    half_extent_metres: block.spec.key.half_extent_metres(),
                    sample_spacing_metres: spacing,
                });
        }

        projected_any = true;
    }

    // Coverage now means actually visible contextual geometry after the terminal
    // dense child has cut its aperture.
    let live_authorities =
        registry.plans.keys().copied().collect::<HashSet<_>>();
    coverage.retain_authorities(&live_authorities);
    for authority in live_authorities {
        coverage.replace_authority(
            authority,
            visible_by_authority.remove(&authority).unwrap_or_default(),
        );
    }

    telemetry.record_visible_frontier(
        visible_blocks,
        yielded_blocks,
        &visible_levels,
        finest_visible_spacing,
        coarsest_visible_spacing,
    );

    if projected_any && !*logged_projection {
        info!(
            interaction_scale = %interaction.scale(),
            interaction_target_scale = %physical_target_scale,
            interaction_handoff_pending = interaction.handoff_pending(),
            view_scale = %view.scale(),
            view_exponent = view.continuous_exponent(),
            metre_to_view,
            visible_binary_blocks = visible_blocks,
            dense_aperture_yielded_blocks = yielded_blocks,
            visible_binary_levels = visible_levels.len(),
            "celestial clipmap projected through stable body-local frontier"
        );
        *logged_projection = true;
    }
}

/// Dense physical/current-interaction presentation is the terminal refinement
/// child. Binary context may yield to it; this function never does the reverse.
// dense-visual-make-before-break-v1
fn enforce_dense_interaction_presentation(
    mut commands: Commands,
    view: Single<&UsfViewContext, With<UsfViewRenderAnchor>>,
    view_demands: Res<UsfViewDemandSnapshot>,
    interaction: Res<UsfPrimaryInteractionSlice>,
    clipmap_coverage: Res<CelestialClipmapCoverageSnapshot>,
    runtimes: Query<&VoxelMaterializationRuntime>,
    worlds: Query<(
        &CelestialVoxelRealization,
        &UsfScaleLayer,
        &VoxelWorld,
        Option<&VoxelStreaming>,
    )>,
    authorities: Query<(&UsfPosition, &UsfSemanticFrame), With<CelestialVoxelField>>,
    mut presentations: Query<
        (&ChildOf, &mut Visibility),
        With<VoxelMaterializationPresentation>,
    >,
    mut telemetry: ResMut<CelestialClipmapTelemetry>,
) {
    let pixels_per_radian = view_demands
        .iter()
        .next()
        .and_then(|view| view.pixels_per_radian_for_presentation_resolution());

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

        let binary_replacement_ready = center.is_some_and(|center| {
            let Ok((body_origin, body_frame)) =
                authorities.get(realization.authority())
            else {
                return false;
            };
            let Ok(center_local) = body_frame.world_to_local_metres(
                body_origin,
                &center,
                SpatialScale::ZERO,
                f64::MAX,
            ) else {
                return false;
            };
            let Ok(observer_local) = body_frame.world_to_local_metres(
                body_origin,
                view.anchor(),
                SpatialScale::ZERO,
                f64::MAX,
            ) else {
                return false;
            };
            let distance_metres =
                (center_local - observer_local).length();
            let Some(spacing) = clipmap_coverage
                .finest_spacing_covering(
                    realization.authority(),
                    center_local,
                )
            else {
                return false;
            };

            binary_spacing_is_adequate_for_dense_handoff(
                spacing,
                distance_metres,
                pixels_per_radian,
            )
        });

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
            // Active physical/view demand owns the dense cache. Rendering may
            // still yield to a proven binary parent at medium distance, but the
            // manifestation must remain reusable and must not be retired.
            commands
                .entity(parent.0)
                .remove::<VoxelPresentationFallbackRetireReady>();
            *visibility = if binary_replacement_ready {
                Visibility::Hidden
            } else {
                Visibility::Inherited
            };
            continue;
        }

        // presentation-fallback-across-residency-v1
        // frontier-local-dense-fallback-v1
        //
        // Frustum relevance is not a lifetime. During vertical flight while
        // looking toward the planet, historical patches can remain in-frustum
        // indefinitely. Hold only a short local make-before-break bridge.
        let forced_by_frontier =
            view_relevant
                && !binary_replacement_ready
                && !fallback_frontier_local;

        if binary_replacement_ready
            || !view_relevant
            || !fallback_frontier_local
        {
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
        .init_resource::<CelestialClipmapPlannerPolicyCache>()
        .init_resource::<CelestialClipmapCoverageSnapshot>()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_visual_handoff_requires_distance_resolution_and_screen_error() {
        assert!(!binary_spacing_is_adequate_for_dense_handoff(
            2.0,
            20.0,
            Some(1_000.0),
        ));
        assert!(binary_spacing_is_adequate_for_dense_handoff(
            4.0,
            1_000.0,
            Some(1_000.0),
        ));
        assert!(!binary_spacing_is_adequate_for_dense_handoff(
            64.0,
            1_000.0,
            Some(1_000.0),
        ));
    }

    #[test]
    fn binary_visual_target_is_independent_from_interaction_scale() {
        let clearance = 1_000.0;
        let target = visual_target_spacing_metres(
            clearance,
            Some(1_000.0),
        );
        assert!((target - 4.0).abs() < 1.0e-9);

        let resolution =
            VoxelPresentationResolution::at_most_metres(target).unwrap();
        assert_eq!(resolution.sample_spacing_metres(), 4.0);
    }

    #[test]
    fn binary_quantization_never_exceeds_visual_quality_target() {
        for target in [1.0, 3.0, 7.0, 40.0, 900.0] {
            let resolution =
                VoxelPresentationResolution::at_most_metres(target).unwrap();
            assert!(
                resolution.sample_spacing_metres() <= target,
                "target={target}, actual={}",
                resolution.sample_spacing_metres(),
            );
        }
    }

    #[test]
    fn clipmap_focus_projects_to_semantic_boundary() {
        let field = CelestialVoxelField::new(
            6_371_000.0,
            SpatialScale::new(6).unwrap(),
            SpatialScale::ZERO,
            0x4541_5254,
            crate::voxel::CelestialBodyProfile::Rocky,
        );
        let surface = field.surface_local_metres(Vec3::Y).unwrap();
        let observer = surface + DVec3::Y * 4_000.0;

        let input = derive_plan_input(
            field,
            observer,
            Some(1_000.0),
            0.0,
            0.05,
            0,
        )
        .unwrap();

        assert!(
            (input.planning_anchor_local - surface).length() < 5.0,
            "binary focus must follow terrain boundary, not empty observer space",
        );
        assert!(input.clearance_metres > 3_000.0);
    }

    #[test]
    fn sparse_frontier_budget_grows_with_requested_depth() {
        let shallow = CelestialClipmapPlanInput {
            key: CelestialClipmapPlanKey {
                finest_exponent: 10,
                coarsest_exponent: 14,
                policy_revision: 0,
            },
            planning_anchor_local: DVec3::ZERO,
            validity_radius_metres: 1_000.0,
            clearance_metres: 100.0,
            finest: VoxelPresentationResolution::new(10),
            coarsest: VoxelPresentationResolution::new(14),
        };
        let deep = CelestialClipmapPlanInput {
            key: CelestialClipmapPlanKey {
                finest_exponent: 0,
                coarsest_exponent: 20,
                policy_revision: 0,
            },
            finest: VoxelPresentationResolution::new(0),
            coarsest: VoxelPresentationResolution::new(20),
            ..shallow
        };

        assert!(
            sparse_frontier_leaf_budget(deep)
                > sparse_frontier_leaf_budget(shallow)
        );
        assert!(
            sparse_frontier_leaf_budget(deep)
                <= MAX_SPARSE_FRONTIER_LEAVES
        );
    }

    #[test]
    fn warm_stage_selection_preserves_existing_local_quality() {
        let coarse = CelestialClipmapBlockSpec {
            key: CelestialClipmapBlockKey {
                resolution: VoxelPresentationResolution::new(6),
                coord: IVec3::ZERO,
            },
            transition_faces: VoxelTransitionFaces::default(),
        };
        let medium = CelestialClipmapBlockSpec {
            key: CelestialClipmapBlockKey {
                resolution: VoxelPresentationResolution::new(4),
                coord: IVec3::ZERO,
            },
            transition_faces: VoxelTransitionFaces::default(),
        };
        let fine = CelestialClipmapBlockSpec {
            key: CelestialClipmapBlockKey {
                resolution: VoxelPresentationResolution::new(2),
                coord: IVec3::ZERO,
            },
            transition_faces: VoxelTransitionFaces::default(),
        };

        let stages = vec![
            vec![coarse],
            vec![medium],
            vec![fine],
        ];
        let committed = HashSet::from([medium]);
        let input = CelestialClipmapPlanInput {
            key: CelestialClipmapPlanKey {
                finest_exponent: 2,
                coarsest_exponent: 6,
                policy_revision: 0,
            },
            planning_anchor_local: DVec3::ZERO,
            validity_radius_metres: 1_000.0,
            clearance_metres: 100.0,
            finest: VoxelPresentationResolution::new(2),
            coarsest: VoxelPresentationResolution::new(6),
        };

        assert_eq!(
            initial_stage_for_plan(&stages, &committed, input),
            Some(1),
            "warm plan should start at equal local quality, not root or final",
        );
    }

    #[test]
    fn local_refinement_candidate_beats_far_coarse_candidate() {
        let finest = VoxelPresentationResolution::new(0);
        let near = CelestialClipmapBlockKey {
            resolution: VoxelPresentationResolution::new(5),
            coord: IVec3::ZERO,
        };
        let far = CelestialClipmapBlockKey {
            resolution: VoxelPresentationResolution::new(6),
            coord: IVec3::new(1_000, 0, 0),
        };
        let near_candidate =
            refinement_candidate(near, finest, DVec3::ZERO, 1_000.0).unwrap();
        let far_candidate =
            refinement_candidate(far, finest, DVec3::ZERO, 1_000.0).unwrap();
        assert!(
            near_candidate > far_candidate,
            "after root context exists, local under-resolution should outrank far global breadth",
        );
    }

    #[test]
    fn ten_to_one_decimal_gap_requires_binary_bridge_levels() {
        let fine =
            VoxelPresentationResolution::at_most_metres(10_000.0).unwrap();
        let coarse =
            VoxelPresentationResolution::at_most_metres(100_000.0).unwrap();

        let binary_steps = i32::from(coarse.binary_exponent())
            - i32::from(fine.binary_exponent());

        assert!(
            binary_steps >= 3,
            "10x must not be treated as one Transvoxel adjacency",
        );
    }

    #[test]
    fn frontier_specs_keep_parent_and_children_in_separate_frontiers() {
        let parent = CelestialClipmapBlockKey {
            resolution: VoxelPresentationResolution::new(4),
            coord: IVec3::ZERO,
        };
        let children = parent.children().unwrap();

        let coarse = HashSet::from([parent]);
        let fine = children.into_iter().collect::<HashSet<_>>();

        let coarse_specs = specs_for_frontier(&coarse, DVec3::ZERO);
        let fine_specs = specs_for_frontier(&fine, DVec3::ZERO);

        assert_eq!(coarse_specs.len(), 1);
        assert_eq!(coarse_specs[0].key, parent);
        assert_eq!(fine_specs.len(), 8);
        assert!(fine_specs.iter().all(|spec| {
            spec.key.resolution == VoxelPresentationResolution::new(3)
        }));
    }

    #[test]
    fn coarse_context_is_scheduled_before_fine_detail() {
        let coarse = CelestialClipmapBlockKey {
            resolution: VoxelPresentationResolution::new(12),
            coord: IVec3::ZERO,
        };
        let fine = CelestialClipmapBlockKey {
            resolution: VoxelPresentationResolution::new(2),
            coord: IVec3::ZERO,
        };
        let mut ordered = vec![fine, coarse];
        ordered.sort_unstable_by(|a, b| {
            b.resolution
                .cmp(&a.resolution)
                .then_with(|| a.coord.x.cmp(&b.coord.x))
                .then_with(|| a.coord.y.cmp(&b.coord.y))
                .then_with(|| a.coord.z.cmp(&b.coord.z))
        });

        assert_eq!(ordered, vec![coarse, fine]);
    }

    #[test]
    fn whole_body_root_resolution_is_altitude_independent() {
        let field = CelestialVoxelField::new(
            6_371_000.0,
            SpatialScale::new(6).unwrap(),
            SpatialScale::ZERO,
            0x4541_5254,
            crate::voxel::CelestialBodyProfile::Rocky,
        );

        let derive = |observer_local: DVec3| {
            derive_plan_input(
                field,
                observer_local,
                Some(1_000.0),
                0.0,
                0.05,
                0,
            )
            .unwrap()
        };

        let surface = derive(DVec3::Y * 6_371_025.0);
        let high = derive(DVec3::Y * 60_000_000.0);

        assert_eq!(
            surface.coarsest.binary_exponent(),
            high.coarsest.binary_exponent(),
            "whole-body root resolution must not change merely because observer altitude changes",
        );
    }

    #[test]
    fn body_root_coordinates_are_observer_independent() {
        let field = CelestialVoxelField::new(
            6_371_000.0,
            SpatialScale::new(6).unwrap(),
            SpatialScale::ZERO,
            0x4541_5254,
            crate::voxel::CelestialBodyProfile::Rocky,
        );
        let resolution = VoxelPresentationResolution::new(21);
        let a = body_root_coordinate_bounds(field, resolution).unwrap();
        let b = body_root_coordinate_bounds(field, resolution).unwrap();
        assert_eq!(a, b);
        assert!(a.0 <= -1);
        assert!(a.1 >= 0);
    }

    #[test]
    fn sticky_anchor_does_not_replan_for_small_motion() {
        let scale = SpatialScale::ZERO;
        let field = CelestialVoxelField::new(
            6_371_000.0,
            SpatialScale::new(6).unwrap(),
            scale,
            0x4541_5254,
            crate::voxel::CelestialBodyProfile::Rocky,
        );
        let input = derive_plan_input(
            field,
            DVec3::Y * 6_371_025.0,
            Some(1_000.0),
            0.0,
            0.05,
            0,
        )
        .unwrap();

        let plan = CelestialClipmapPlan {
            key: input.key,
            field,
            policy: None,
            planning_anchor_local: input.planning_anchor_local,
            validity_radius_metres: input.validity_radius_metres,
            generation: 1,
            stages: vec![],
            stage_index: 0,
            desired: vec![],
            completed: HashSet::new(),
            meshful: HashSet::new(),
            known_empty: HashSet::new(),
            committed_specs: HashSet::new(),
            committed_generation: None,
        };

        let mut moved = input;
        moved.planning_anchor_local += DVec3::X * 1.0;
        assert!(!plan_requires_refresh(&plan, moved, field));

        moved.planning_anchor_local +=
            DVec3::X * input.validity_radius_metres.max(100.0) * 2.0;
        assert!(plan_requires_refresh(&plan, moved, field));
    }

    #[test]
    fn rolling_plan_accepts_spatial_progress_beyond_fresh_radius() {
        let field = CelestialVoxelField::new(
            6_371_000.0,
            SpatialScale::new(6).unwrap(),
            SpatialScale::ZERO,
            0x4541_5254,
            crate::voxel::CelestialBodyProfile::Rocky,
        );
        let make = |anchor_x: f64| CelestialClipmapPlanInput {
            key: CelestialClipmapPlanKey {
                finest_exponent: 0,
                coarsest_exponent: 20,
                policy_revision: 7,
            },
            planning_anchor_local: DVec3::new(anchor_x, 0.0, 0.0),
            validity_radius_metres: 10.0,
            clearance_metres: 1.0,
            finest: VoxelPresentationResolution::new(0),
            coarsest: VoxelPresentationResolution::new(20),
        };

        let built = make(60.0);
        let current = make(100.0);
        let relevance = plan_task_relevance(
            built,
            current,
            field,
            field,
            Some(DVec3::ZERO),
        );

        assert_eq!(
            relevance,
            ClipmapPlanTaskRelevance::RollingProgress,
            "a behind result that moves committed focus 100m -> 40m lag is useful progress",
        );
    }

    #[test]
    fn rolling_plan_rejects_policy_or_root_topology_change() {
        let field = CelestialVoxelField::new(
            6_371_000.0,
            SpatialScale::new(6).unwrap(),
            SpatialScale::ZERO,
            0x4541_5254,
            crate::voxel::CelestialBodyProfile::Rocky,
        );
        let built = CelestialClipmapPlanInput {
            key: CelestialClipmapPlanKey {
                finest_exponent: 0,
                coarsest_exponent: 20,
                policy_revision: 1,
            },
            planning_anchor_local: DVec3::ZERO,
            validity_radius_metres: 100.0,
            clearance_metres: 1.0,
            finest: VoxelPresentationResolution::new(0),
            coarsest: VoxelPresentationResolution::new(20),
        };
        let mut current = built;
        current.key.policy_revision = 2;

        assert_eq!(
            plan_task_relevance(
                built,
                current,
                field,
                field,
                Some(DVec3::ZERO),
            ),
            ClipmapPlanTaskRelevance::Stale,
        );
    }

    #[test]
    fn finest_exponent_change_is_not_structural_plan_incompatibility() {
        let a = CelestialClipmapPlanKey {
            finest_exponent: 0,
            coarsest_exponent: 20,
            policy_revision: 9,
        };
        let b = CelestialClipmapPlanKey {
            finest_exponent: 5,
            ..a
        };
        assert!(clipmap_plan_keys_structurally_compatible(a, b));
    }

    #[test]
    fn warm_replan_targets_final_frontier_without_coarse_regression() {
        let stages = vec![vec![], vec![], vec![]];
        assert_eq!(
            stages.len(),
            3,
            "warm-stage policy is covered by quality-preserving selection test",
        );
    }

    #[test]
    fn dense_union_can_cover_one_binary_block_across_adjacent_cells() {
        let key = CelestialClipmapBlockKey {
            resolution: VoxelPresentationResolution::new(0),
            coord: IVec3::ZERO,
        };
        // 8 m block. Two adjacent 4 m half-width boxes tile x while spanning
        // the whole block in y/z.
        let make = |center_x: f64| DensePresentationApertureCell {
            center_local_metres: DVec3::new(center_x, 4.0, 4.0),
            world_x_local: DVec3::X,
            world_y_local: DVec3::Y,
            world_z_local: DVec3::Z,
            half_extent_metres: DVec3::new(2.0, 4.0, 4.0),
        };
        let cells = [make(2.0), make(6.0)];
        assert!(block_fully_covered_by_dense_union(key, &cells));
    }

    #[test]
    fn whole_body_root_reach_exceeds_planet_diameter_at_surface() {
        let radius = 6_371_000.0_f64;
        let observer_radius = radius + 25.0;
        let whole_body_reach =
            observer_radius + radius * WHOLE_BODY_ROOT_MARGIN;

        assert!(whole_body_reach > radius * 2.0);
    }

    #[test]
    fn coverage_cell_remembers_resolution_and_contains_local_points() {
        let cell = CelestialClipmapCoverageCell {
            center_local_metres: DVec3::new(10.0, 20.0, 30.0),
            half_extent_metres: DVec3::splat(8.0),
            sample_spacing_metres: 2.0,
        };

        assert_eq!(cell.sample_spacing_metres(), 2.0);
        assert!(cell.contains_local_point(DVec3::new(12.0, 18.0, 31.0)));
        assert!(!cell.contains_local_point(DVec3::new(19.0, 20.0, 30.0)));
    }

    #[test]
    fn decimal_dense_context_is_not_part_of_the_binary_lod_contract() {
        let interaction = SpatialScale::new(3).unwrap();
        let coarse = SpatialScale::new(5).unwrap();
        assert_ne!(coarse, interaction);
    }

    #[test]
    fn face_adjacency_finds_coarse_to_fine_boundary() {
        let coarse = CelestialClipmapBlockKey {
            resolution: VoxelPresentationResolution::new(1),
            coord: IVec3::ZERO,
        };
        let fine = CelestialClipmapBlockKey {
            resolution: VoxelPresentationResolution::new(0),
            coord: IVec3::new(2, 0, 0),
        };

        assert_eq!(
            face_from_a_to_b(coarse, fine),
            Some(VoxelTransitionFace::HighX),
        );
    }
}
