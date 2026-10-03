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
use crate::voxel::developer_policy::{
    presentation_surface_radius_bounds_metres,
};

use crate::{
    devtools::{
        DeveloperScalarPolicyRuntime, DeveloperScalarPolicySnapshot,
        DeveloperScriptWorkbench,
    },
    ecs::UsfPresentationProjectionOf,
    spatial::{
        SpatialRealizationGranularityRequest, SpatialScale, UsfPosition,
        UsfPrimaryInteractionSlice, UsfSemanticFrame,
        UsfSpatialSet, UsfViewContext, UsfViewDemandSnapshot,
        UsfViewRenderAnchor, UsfScaleLayer
    },
};

use super::{
    VoxelPresentationResolution, VoxelTransitionFace, VoxelTransitionFaces,
};
use super::super::{
    CelestialVoxelField, CelestialVoxelRealization, CelestialVoxelRealizationPolicy,
    MATERIALIZATION_CHUNK_SIZE, VoxelAuthority, VoxelWorld,
    manifestation::{
        VoxelMaterializationPresentation, VoxelMaterializationRuntime,
    },
    worker::{VoxelWorkerLane, VoxelWorkerPool, VoxelWorkerTask, VoxelWorkerTicket},
};

const BLOCK_SUBDIVISIONS: usize = 8;
const MIN_SAMPLE_SPACING_METRES: f64 = 2.0;
const MAX_FINE_SAMPLE_SPACING_METRES: f64 = 2_048.0;
const BASE_LOCAL_RADIUS_METRES: f64 = 64_000.0;
// whole-body-volumetric-clipmap-v1
//
// The coarsest binary bricks are allowed to span the semantic body. A small
// conservative margin absorbs canonical relief without inventing a second
// spherical surface representation.
const WHOLE_BODY_ROOT_MARGIN: f64 = 1.125;
const TARGET_CELLS_PER_DISTANCE: f64 = 16.0;
const MAX_INITIAL_LEAVES: usize = 224;
const MAX_BALANCED_LEAVES: usize = 512;
// small-coarse-first-refinement-waves-v1
const MAX_PRIMARY_REFINEMENTS_PER_STAGE: usize = 2;
const MAX_BUILDS_IN_FLIGHT: usize = 4;
const MAX_BUILD_ADMISSIONS_PER_FRAME: usize = 2;
const MAX_PUBLICATIONS_PER_FRAME: usize = 2;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CelestialClipmapPlanKey {
    observer_bucket: IVec3,
    finest_exponent: i16,
    coarsest_exponent: i16,
    validity_exponent: i16,
    policy_revision: u64,
}

// transactional-binary-refinement-frontiers-v1
#[derive(Debug)]
struct CelestialClipmapPlan {
    key: CelestialClipmapPlanKey,
    field: CelestialVoxelField,
    policy: Option<DeveloperScalarPolicySnapshot>,
    generation: u64,
    stages: Vec<Vec<CelestialClipmapBlockSpec>>,
    stage_index: usize,
    desired: Vec<CelestialClipmapBlockSpec>,
    completed: HashSet<CelestialClipmapBlockSpec>,
    meshful: HashSet<CelestialClipmapBlockSpec>,
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
    distance: f64,
    refinement_debt: i16,
    key: CelestialClipmapBlockKey,
}

impl PartialEq for ClipmapRefinementCandidate {
    fn eq(&self, other: &Self) -> bool {
        self.refinement_debt == other.refinement_debt
            && self.distance.total_cmp(&other.distance) == std::cmp::Ordering::Equal
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
        self.key
            .resolution
            .cmp(&other.key.resolution)
            .then_with(|| self.refinement_debt.cmp(&other.refinement_debt))
            .then_with(|| other.distance.total_cmp(&self.distance))
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
    (refinement_debt > 0).then_some(ClipmapRefinementCandidate {
        distance,
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

fn refine_leaf_indexed(
    leaves: &mut HashSet<CelestialClipmapBlockKey>,
    parent: CelestialClipmapBlockKey,
    field: CelestialVoxelField,
    policy: Option<&DeveloperScalarPolicyRuntime>,
    surface_cache: &mut CelestialClipmapSurfaceCache,
    inserted: &mut Vec<CelestialClipmapBlockKey>,
) -> bool {
    inserted.clear();
    if !leaves.remove(&parent) {
        return true;
    }

    let Some(children) = parent.children() else {
        leaves.insert(parent);
        return false;
    };

    for child in children {
        if surface_cache.intersects(field, child, policy) {
            leaves.insert(child);
            inserted.push(child);
        }
    }

    if inserted.is_empty() {
        leaves.insert(parent);
        return false;
    }

    true
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

            if leaves.len().saturating_add(7) > MAX_BALANCED_LEAVES {
                return false;
            }

            if !refine_leaf_indexed(
                leaves,
                neighbor,
                field,
                policy,
                surface_cache,
                &mut inserted,
            ) {
                return false;
            }

            queue.extend(inserted.iter().copied());
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

fn refine_leaf(
    leaves: &mut Vec<CelestialClipmapBlockKey>,
    index: usize,
    field: CelestialVoxelField,
    policy: Option<&DeveloperScalarPolicyRuntime>,
) -> bool {
    let parent = leaves.swap_remove(index);
    let Some(children) = parent.children() else {
        leaves.push(parent);
        return false;
    };

    for child in children {
        if block_intersects_semantic_surface(field, child, policy) {
            leaves.push(child);
        }
    }
    true
}

fn face_from_a_to_b(
    a: CelestialClipmapBlockKey,
    b: CelestialClipmapBlockKey,
) -> Option<VoxelTransitionFace> {
    let a_min = a.origin_local_metres();
    let a_max = a_min + DVec3::splat(a.extent_metres());
    let b_min = b.origin_local_metres();
    let b_max = b_min + DVec3::splat(b.extent_metres());
    let epsilon = a.spacing_metres().min(b.spacing_metres()) * 1.0e-6 + 1.0e-9;

    let overlap = |a0: f64, a1: f64, b0: f64, b1: f64| {
        a1.min(b1) - a0.max(b0) > epsilon
    };
    let same = |lhs: f64, rhs: f64| (lhs - rhs).abs() <= epsilon;

    if same(a_min.x, b_max.x)
        && overlap(a_min.y, a_max.y, b_min.y, b_max.y)
        && overlap(a_min.z, a_max.z, b_min.z, b_max.z)
    {
        return Some(VoxelTransitionFace::LowX);
    }
    if same(a_max.x, b_min.x)
        && overlap(a_min.y, a_max.y, b_min.y, b_max.y)
        && overlap(a_min.z, a_max.z, b_min.z, b_max.z)
    {
        return Some(VoxelTransitionFace::HighX);
    }
    if same(a_min.y, b_max.y)
        && overlap(a_min.x, a_max.x, b_min.x, b_max.x)
        && overlap(a_min.z, a_max.z, b_min.z, b_max.z)
    {
        return Some(VoxelTransitionFace::LowY);
    }
    if same(a_max.y, b_min.y)
        && overlap(a_min.x, a_max.x, b_min.x, b_max.x)
        && overlap(a_min.z, a_max.z, b_min.z, b_max.z)
    {
        return Some(VoxelTransitionFace::HighY);
    }
    if same(a_min.z, b_max.z)
        && overlap(a_min.x, a_max.x, b_min.x, b_max.x)
        && overlap(a_min.y, a_max.y, b_min.y, b_max.y)
    {
        return Some(VoxelTransitionFace::LowZ);
    }
    if same(a_max.z, b_min.z)
        && overlap(a_min.x, a_max.x, b_min.x, b_max.x)
        && overlap(a_min.y, a_max.y, b_min.y, b_max.y)
    {
        return Some(VoxelTransitionFace::HighZ);
    }

    None
}

fn balance_leaves_2_to_1(
    field: CelestialVoxelField,
    leaves: &mut Vec<CelestialClipmapBlockKey>,
    policy: Option<&DeveloperScalarPolicyRuntime>,
) -> bool {
    loop {
        let mut refine = None::<usize>;

        'pairs: for a_index in 0..leaves.len() {
            for b_index in (a_index + 1)..leaves.len() {
                if face_from_a_to_b(leaves[a_index], leaves[b_index]).is_none() {
                    continue;
                }

                let a_exp = leaves[a_index].resolution.binary_exponent();
                let b_exp = leaves[b_index].resolution.binary_exponent();
                if (i32::from(a_exp) - i32::from(b_exp)).abs() <= 1 {
                    continue;
                }

                refine = Some(if a_exp > b_exp { a_index } else { b_index });
                break 'pairs;
            }
        }

        let Some(index) = refine else {
            return true;
        };

        if leaves.len().saturating_add(7) > MAX_BALANCED_LEAVES {
            return false;
        }
        if !refine_leaf(leaves, index, field, policy) {
            return false;
        }
    }
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
/// This deliberately performs no terrain sampling/refinement. Stable frames only
/// need to know whether observer bucket, binary LOD bounds, field identity or
/// script revision changed; the expensive leaf plan is rebuilt only after that
/// identity changes.
fn derive_plan_input(
    field: CelestialVoxelField,
    observer_local: DVec3,
    observer_speed_metres_per_second: f64,
    expected_build_seconds: f64,
    policy_revision: u64,
) -> Option<CelestialClipmapPlanInput> {
    let observer_radius = observer_local.length();
    if !observer_radius.is_finite() {
        return None;
    }

    // Presentation follows the same canonical volumetric field as dense
    // realization. In a cave this measures cave-wall proximity rather than
    // distance to an unrelated radial outer shell.
    let clearance = field
        .signed_distance_local_metres(observer_local)?
        .abs();
    if !clearance.is_finite() {
        return None;
    }

    let desired_spacing = (clearance / 64.0).clamp(
        MIN_SAMPLE_SPACING_METRES,
        MAX_FINE_SAMPLE_SPACING_METRES,
    );
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
    ).solve();

    let finest = VoxelPresentationResolution::at_least_metres(
        granularity.target_spacing_metres(),
    )?;

    // A single hierarchy owns both the nearby aperture and whole-body context.
    // Make the coarsest root extent large enough to span from the observer past
    // the far side of the semantic body. This works on the same body-local
    // Cartesian lattice at the surface, underground and in distant space.
    let whole_body_reach =
        observer_radius
            + field.conservative_outer_radius_metres() * WHOLE_BODY_ROOT_MARGIN;
    let local_radius = (
        BASE_LOCAL_RADIUS_METRES
            + clearance * 2.0
            + granularity.validity_radius_metres()
    )
    .max(BASE_LOCAL_RADIUS_METRES)
    .max(whole_body_reach);

    let coarse_spacing_target =
        (local_radius / BLOCK_SUBDIVISIONS as f64)
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

    let fine_extent =
        finest.sample_spacing_metres() * BLOCK_SUBDIVISIONS as f64;
    let requested_validity_extent =
        (granularity.validity_radius_metres() * 2.0).max(fine_extent);
    let validity_level =
        VoxelPresentationResolution::at_least_metres(requested_validity_extent)?;
    let validity_extent = validity_level.sample_spacing_metres();
    let observer_bucket = checked_floor_coord(observer_local, validity_extent)?;
    let planning_anchor_local = DVec3::new(
        (f64::from(observer_bucket.x) + 0.5) * validity_extent,
        (f64::from(observer_bucket.y) + 0.5) * validity_extent,
        (f64::from(observer_bucket.z) + 0.5) * validity_extent,
    );

    Some(CelestialClipmapPlanInput {
        key: CelestialClipmapPlanKey {
            observer_bucket,
            finest_exponent: finest.binary_exponent(),
            coarsest_exponent: coarsest.binary_exponent(),
            validity_exponent: validity_level.binary_exponent(),
            policy_revision,
        },
        planning_anchor_local,
        validity_radius_metres: validity_extent * 0.5,
        clearance_metres: clearance,
        finest,
        coarsest,
    })
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

/// Expensive staged-frontier plan construction.
///
/// Coarse ancestry is runtime fallback, not disposable planning scratch state.
/// Record one balanced frontier after each binary refinement round.
fn build_plan(
    field: CelestialVoxelField,
    input: CelestialClipmapPlanInput,
    policy: Option<&DeveloperScalarPolicyRuntime>,
    surface_cache: &mut CelestialClipmapSurfaceCache,
) -> Option<Vec<Vec<CelestialClipmapBlockSpec>>> {
    surface_cache.begin_plan(field, input.key.policy_revision);

    let result = (|| {
        let observer_local = input.planning_anchor_local;
        let validity_radius_metres = input.validity_radius_metres;
        let finest = input.finest;
        let coarsest = input.coarsest;

        let root_extent =
            coarsest.sample_spacing_metres() * BLOCK_SUBDIVISIONS as f64;
        let root_center = checked_floor_coord(observer_local, root_extent)?;

        let mut leaves =
            HashSet::<CelestialClipmapBlockKey>::with_capacity(MAX_BALANCED_LEAVES);

        for z in -1..=1 {
            for y in -1..=1 {
                for x in -1..=1 {
                    let cx = root_center.x.checked_add(x)?;
                    let cy = root_center.y.checked_add(y)?;
                    let cz = root_center.z.checked_add(z)?;
                    let key = CelestialClipmapBlockKey {
                        resolution: coarsest,
                        coord: IVec3::new(cx, cy, cz),
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

        let mut stages = vec![specs_for_frontier(&leaves, observer_local)];

        loop {
            let mut candidates =
                BinaryHeap::<ClipmapRefinementCandidate>::with_capacity(
                    leaves.len(),
                );
            for &key in &leaves {
                if let Some(candidate) = refinement_candidate(
                    key,
                    finest,
                    observer_local,
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
            let mut refined_any = false;
            let mut primary_refinements = 0usize;

            // Keep each transaction small. Newly inserted children wait for the
            // next recorded stage instead of expanding one giant commit barrier.
            while let Some(candidate) = candidates.pop() {
                if primary_refinements >= MAX_PRIMARY_REFINEMENTS_PER_STAGE {
                    break;
                }
                if !leaves.contains(&candidate.key) {
                    continue;
                }
                if leaves.len().saturating_add(7) > MAX_INITIAL_LEAVES {
                    break;
                }

                if refine_leaf_indexed(
                    &mut leaves,
                    candidate.key,
                    field,
                    policy,
                    surface_cache,
                    &mut inserted,
                ) && !inserted.is_empty()
                {
                    refined_any = true;
                    primary_refinements += 1;
                }
            }

            if !refined_any {
                break;
            }

            if !balance_leaves_2_to_1_local(
                field,
                &mut leaves,
                policy,
                surface_cache,
            ) {
                leaves = previous;
                break;
            }

            let stage = specs_for_frontier(&leaves, observer_local);
            if stages.last().is_some_and(|current| *current == stage) {
                break;
            }
            stages.push(stage);
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
    mut coverage: ResMut<CelestialClipmapCoverageSnapshot>,
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
            let Some(input) = derive_plan_input(
                *field,
                observer_local,
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

        if build.field != current_field || build.input.key != current_input.key {
            continue;
        }

        let Some(stages) = output.stages else {
            continue;
        };
        let Some(desired) = stages.first().cloned() else {
            continue;
        };
        if desired.is_empty() {
            continue;
        }

        info!(
            authority = ?build.authority,
            canonical_clearance_metres = build.input.clearance_metres,
            finest_spacing_metres = build.input.finest.sample_spacing_metres(),
            coarsest_spacing_metres = build.input.coarsest.sample_spacing_metres(),
            refinement_stages = stages.len(),
            initial_blocks = desired.len(),
            final_blocks = stages.last().map_or(0, Vec::len),
            "celestial clipmap staged plan ready"
        );

        let (committed_generation, committed_specs) = registry
            .plans
            .get(&build.authority)
            .map(|plan| {
                (
                    plan.committed_generation,
                    plan.committed_specs.clone(),
                )
            })
            .unwrap_or((None, HashSet::new()));

        let generation = registry.next_generation();
        registry.plans.insert(
            build.authority,
            CelestialClipmapPlan {
                key: build.input.key,
                field: build.field,
                policy: build.policy.clone(),
                generation,
                stages,
                stage_index: 0,
                desired,
                completed: HashSet::new(),
                meshful: HashSet::new(),
                committed_specs,
                committed_generation,
            },
        );
        plans_changed = true;
    }

    let mut planning_slots =
        workers.available_slots(VoxelWorkerLane::PresentationPlanning);
    for (&authority, &(input, field)) in &current_inputs {
        let replace = registry.plans.get(&authority).is_none_or(|plan| {
            plan.key != input.key || plan.field != field
        });
        if !replace
            || planning_authorities.contains(&authority)
            || planning_slots == 0
        {
            continue;
        }

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

        commands.spawn((
            Name::new("Celestial Binary Clipmap Plan"),
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
    coverage.retain_authorities(&live_authorities);

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

            if publications >= MAX_PUBLICATIONS_PER_FRAME {
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
            }
            frame_budget.finish(work_token);
        }
    }

    // Only the committed visible frontier owns presentation coverage.
    {
        let _span =
            bevy::log::info_span!("celestial_clipmap.committed_coverage").entered();
        for (&authority, plan) in &registry.plans {
            let realized_specs = plan
                .committed_specs
                .iter()
                .copied()
                .filter(|spec| {
                    projected_specs.contains(&(
                        authority,
                        plan.key.policy_revision,
                        *spec,
                    ))
                })
                .collect::<Vec<_>>();
            coverage.replace_authority(
                authority,
                coverage_for_specs(&realized_specs),
            );
        }
    }

    {
        let _span = bevy::log::info_span!("celestial_clipmap.schedule_builds").entered();
        let mut admitted = 0usize;
        let mut worker_slots =
            workers.available_slots(VoxelWorkerLane::PresentationResolution);

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
                    || inflight.len() >= MAX_BUILDS_IN_FLIGHT
                    || admitted >= MAX_BUILD_ADMISSIONS_PER_FRAME
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

            if plan.meshful.is_empty()
                || !plan.meshful.iter().all(|spec| {
                    projected_specs.contains(&(
                        authority,
                        plan.key.policy_revision,
                        *spec,
                    ))
                })
            {
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
    interaction: Res<UsfPrimaryInteractionSlice>,
    registry: Res<CelestialClipmapRegistry>,
    authorities: Query<(&UsfPosition, &UsfSemanticFrame, &CelestialVoxelField)>,
    mut blocks: Query<(
        &mut CelestialClipmapBlock,
        &mut Transform,
        &mut Visibility,
    )>,
    mut logged_projection: Local<bool>,
) {
    // Clipmap vertices are authored in physical metres, which are S0-native
    // units. Presentation scale is observer-owned and deliberately independent
    // from the active interaction/physics chart.
    let metre_to_view = view.projection_factor(SpatialScale::ZERO);
    if !metre_to_view.is_finite() || metre_to_view <= 0.0 {
        for (mut block, _, mut visibility) in &mut blocks {
            block.projection_ready = false;
            *visibility = Visibility::Hidden;
        }
        return;
    }

    let mut projected_any = false;

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
            *visibility = Visibility::Hidden;
            continue;
        };

        // Whole-body blocks can be millions (or, for a distant body, far more)
        // metres from the observer. Preserve canonical precision in f64 until
        // the final view-chart projection instead of forcing the semantic delta
        // through an intermediate f32 S0 window.
        let Ok(relative_metres) = anchor.relative_at_scale_bounded_f64(
            view.anchor(),
            SpatialScale::ZERO,
            f64::MAX,
        ) else {
            *visibility = Visibility::Hidden;
            continue;
        };

        // clipmap-eye-relative-projection-v1
        // Mesh scale and block placement share the same view similarity frame.
        // The physical eye/boom offset is removed before view-chart scaling so
        // clipmap terrain and the local physical pass retain identical rays.
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
        let translation =
            view.presentation_origin() + projected_relative;
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

        *visibility = if committed {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
        projected_any = true;
    }

    if projected_any && !*logged_projection {
        info!(
            interaction_scale = %interaction.scale(),
            view_scale = %view.scale(),
            view_exponent = view.continuous_exponent(),
            metre_to_view,
            "celestial clipmap projected through view chart"
        );
        *logged_projection = true;
    }
}

// exclusive-celestial-presentation-aperture-v1
fn dense_base_presentation_yields_to_binary_hierarchy(
    realization: CelestialVoxelRealization,
    layer: &UsfScaleLayer,
    world: &VoxelWorld,
    runtime: &VoxelMaterializationRuntime,
    authorities: &Query<(&UsfPosition, &UsfSemanticFrame, &CelestialVoxelField)>,
    clipmap_coverage: &CelestialClipmapCoverageSnapshot,
) -> bool {
    let Ok((body_origin, body_frame, _field)) =
        authorities.get(realization.authority())
    else {
        return false;
    };
    let Ok(address) = world.materialization_address(runtime.key()) else {
        return false;
    };

    let half_native = MATERIALIZATION_CHUNK_SIZE as f32 * 0.5;
    let Ok(center) = address
        .query_origin()
        .usf()
        .translated_at_scale(layer.scale(), Vec3::splat(half_native))
    else {
        return false;
    };
    let Ok(center_local_metres) = body_frame.world_to_local_metres(
        body_origin,
        &center,
        SpatialScale::ZERO,
        f64::MAX,
    ) else {
        return false;
    };

    let maximum_replacement_spacing =
        layer.scale().metres_per_native() * 2.0;

    clipmap_coverage
        .for_authority(realization.authority())
        .iter()
        .copied()
        .any(|cell| {
            cell.sample_spacing_metres() <= maximum_replacement_spacing
                && cell.contains_local_point(center_local_metres)
        })
}

fn suppress_legacy_celestial_dense_presentation(
    interaction: Res<UsfPrimaryInteractionSlice>,
    clipmap_coverage: Res<CelestialClipmapCoverageSnapshot>,
    runtimes: Query<&VoxelMaterializationRuntime>,
    worlds: Query<(
        &CelestialVoxelRealization,
        &UsfScaleLayer,
        &VoxelWorld,
    )>,
    authorities: Query<(&UsfPosition, &UsfSemanticFrame, &CelestialVoxelField)>,
    mut presentations: Query<
        (&ChildOf, &mut Visibility),
        With<VoxelMaterializationPresentation>,
    >,
) {
    for (parent, mut visibility) in &mut presentations {
        let Ok(runtime) = runtimes.get(parent.0) else {
            continue;
        };
        let Ok((realization, layer, world)) = worlds.get(runtime.world()) else {
            continue;
        };

        if layer.scale() != interaction.scale() {
            *visibility = Visibility::Hidden;
            continue;
        }

        if dense_base_presentation_yields_to_binary_hierarchy(
            *realization,
            layer,
            world,
            runtime,
            &authorities,
            &clipmap_coverage,
        ) {
            *visibility = Visibility::Hidden;
        } else if runtime.active() {
            *visibility = Visibility::Inherited;
        }
    }
}

pub(super) fn configure(app: &mut App) {
    app.init_resource::<CelestialClipmapRegistry>()
        .init_resource::<CelestialClipmapPlannerPolicyCache>()
        .init_resource::<CelestialClipmapCoverageSnapshot>()
        .add_systems(Update, sync_celestial_clipmap_realizations)
        .add_systems(
            PostUpdate,
            sync_celestial_clipmap_transforms
                .in_set(UsfSpatialSet::ViewProjection),
        )
        .add_systems(
            PostUpdate,
            suppress_legacy_celestial_dense_presentation
                .after(UsfSpatialSet::ViewProjection),
        );
}

#[cfg(test)]
mod tests {
    use super::*;

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
