//! Live celestial presentation clipmap over the voxel-local binary resolution domain.
//!
//! This is presentation only. Semantic terrain remains [`CelestialVoxelField`];
//! dense voxel worlds keep collision/editing authority. The clipmap is a
//! reconstructible mesh adapter whose LOD axis is independent of USF Scale.

use std::collections::{BinaryHeap, HashMap, HashSet};

use bevy::{
    asset::RenderAssetUsages,
    math::DVec3,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};

use transvoxel::prelude::{
    extract_from_field, Block, FieldCaching, GenericMeshBuilder, TransitionSide,
    TransitionSides,
};

use crate::voxel::developer_policy::presentation_surface_radius_metres;

use crate::{
    devtools::{
        DeveloperScalarPolicyRuntime, DeveloperScalarPolicySnapshot,
        DeveloperScriptWorkbench,
    },
    ecs::UsfPresentationProjectionOf,
    spatial::{
        SpatialScale, UsfPosition, UsfPrimaryInteractionSlice, UsfSemanticFrame,
        UsfSpatialFrame, UsfSpatialSet, UsfViewDemandSnapshot, UsfScaleLayer
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
    },
    worker::{VoxelWorkerLane, VoxelWorkerPool, VoxelWorkerTask, VoxelWorkerTicket},
};

const BLOCK_SUBDIVISIONS: usize = 8;
const MIN_SAMPLE_SPACING_METRES: f64 = 2.0;
const MAX_FINE_SAMPLE_SPACING_METRES: f64 = 2_048.0;
const BASE_LOCAL_RADIUS_METRES: f64 = 64_000.0;
const MAX_LOCAL_RADIUS_METRES: f64 = 320_000.0;
const MAX_CLIPMAP_CLEARANCE_METRES: f64 = 250_000.0;
const TARGET_CELLS_PER_DISTANCE: f64 = 16.0;
const MAX_INITIAL_LEAVES: usize = 224;
const MAX_BALANCED_LEAVES: usize = 512;
const MAX_BUILDS_IN_FLIGHT: usize = 4;
const MAX_BUILD_ADMISSIONS_PER_FRAME: usize = 2;
const MAX_PUBLICATIONS_PER_FRAME: usize = 2;
const RUNTIME_RELATIVE_BOUND_NATIVE: f32 = 16_384.0;

/// All clipmap resolution levels sample one semantic terrain function.
///
/// Resolution controls only *sampling density*. It must never select a different
/// semantic terrain band at a coarse/fine transition.
const SEMANTIC_SURFACE_SCALE: SpatialScale = SpatialScale::ZERO;

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
    policy_revision: u64,
}

#[derive(Debug)]
struct CelestialClipmapPlan {
    key: CelestialClipmapPlanKey,
    field: CelestialVoxelField,
    policy: Option<DeveloperScalarPolicySnapshot>,
    generation: u64,
    desired: Vec<CelestialClipmapBlockSpec>,
    completed: HashSet<CelestialClipmapBlockSpec>,
    committed_generation: Option<u64>,
}


// celestial-clipmap-planner-superpass-v1
//
// Planner state is deliberately reusable. Observer motion changes *which*
// presentation blocks are wanted; it does not change the semantic answer to
// "can this dyadic block intersect this body surface?" for a stable
// (field, script revision). Keep two generations of those classifications so
// ordinary movement pays mostly for the changed frontier without an unbounded
// spatial cache.
#[derive(Default)]
struct CelestialClipmapSurfaceCache {
    field: Option<CelestialVoxelField>,
    policy_revision: u64,
    hot: HashMap<CelestialClipmapBlockKey, bool>,
    warm: HashMap<CelestialClipmapBlockKey, bool>,
    next: HashMap<CelestialClipmapBlockKey, bool>,
    hits: usize,
    misses: usize,
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
    key: CelestialClipmapBlockKey,
}

impl PartialEq for ClipmapRefinementCandidate {
    fn eq(&self, other: &Self) -> bool {
        self.distance.total_cmp(&other.distance) == std::cmp::Ordering::Equal
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
        other
            .distance
            .total_cmp(&self.distance)
            .then_with(|| block_sort_key(other.key).cmp(&block_sort_key(self.key)))
    }
}

#[derive(Debug, Clone, Copy)]
struct ClipmapFaceRecord {
    axis: u8,
    plane: i128,
    low: bool,
    u0: i128,
    u1: i128,
    v0: i128,
    v1: i128,
    key: CelestialClipmapBlockKey,
    face: VoxelTransitionFace,
}

#[derive(Debug, Clone, Copy)]
struct ClipmapFaceAdjacency {
    a: CelestialClipmapBlockKey,
    b: CelestialClipmapBlockKey,
    face_from_a: VoxelTransitionFace,
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
    observer_local: DVec3,
) -> Option<ClipmapRefinementCandidate> {
    if key.resolution <= finest {
        return None;
    }

    let distance = block_distance_to_point(key, observer_local);
    let target = target_resolution_at_distance(finest, distance);
    (key.resolution > target).then_some(ClipmapRefinementCandidate {
        distance,
        key,
    })
}

fn refine_leaf_indexed(
    leaves: &mut HashSet<CelestialClipmapBlockKey>,
    parent: CelestialClipmapBlockKey,
    field: CelestialVoxelField,
    policy: Option<&DeveloperScalarPolicyRuntime>,
    surface_cache: &mut CelestialClipmapSurfaceCache,
) -> bool {
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
        }
    }

    true
}

fn normalized_axis_bounds(
    coordinate: i32,
    scale: i128,
) -> Option<(i128, i128)> {
    let low = i128::from(coordinate).checked_mul(scale)?;
    let high = i128::from(coordinate)
        .checked_add(1)?
        .checked_mul(scale)?;
    Some((low, high))
}

fn push_face_records(
    key: CelestialClipmapBlockKey,
    minimum_exponent: i16,
    records: &mut Vec<ClipmapFaceRecord>,
) -> bool {
    let shift =
        i32::from(key.resolution.binary_exponent())
            - i32::from(minimum_exponent);
    let Ok(shift) = u32::try_from(shift) else {
        return false;
    };
    let Some(scale) = 1_i128.checked_shl(shift) else {
        return false;
    };

    let Some((x0, x1)) = normalized_axis_bounds(key.coord.x, scale) else {
        return false;
    };
    let Some((y0, y1)) = normalized_axis_bounds(key.coord.y, scale) else {
        return false;
    };
    let Some((z0, z1)) = normalized_axis_bounds(key.coord.z, scale) else {
        return false;
    };

    records.extend_from_slice(&[
        ClipmapFaceRecord {
            axis: 0,
            plane: x0,
            low: true,
            u0: y0,
            u1: y1,
            v0: z0,
            v1: z1,
            key,
            face: VoxelTransitionFace::LowX,
        },
        ClipmapFaceRecord {
            axis: 0,
            plane: x1,
            low: false,
            u0: y0,
            u1: y1,
            v0: z0,
            v1: z1,
            key,
            face: VoxelTransitionFace::HighX,
        },
        ClipmapFaceRecord {
            axis: 1,
            plane: y0,
            low: true,
            u0: x0,
            u1: x1,
            v0: z0,
            v1: z1,
            key,
            face: VoxelTransitionFace::LowY,
        },
        ClipmapFaceRecord {
            axis: 1,
            plane: y1,
            low: false,
            u0: x0,
            u1: x1,
            v0: z0,
            v1: z1,
            key,
            face: VoxelTransitionFace::HighY,
        },
        ClipmapFaceRecord {
            axis: 2,
            plane: z0,
            low: true,
            u0: x0,
            u1: x1,
            v0: y0,
            v1: y1,
            key,
            face: VoxelTransitionFace::LowZ,
        },
        ClipmapFaceRecord {
            axis: 2,
            plane: z1,
            low: false,
            u0: x0,
            u1: x1,
            v0: y0,
            v1: y1,
            key,
            face: VoxelTransitionFace::HighZ,
        },
    ]);
    true
}

fn rebuild_face_adjacencies(
    leaves: &HashSet<CelestialClipmapBlockKey>,
    records: &mut Vec<ClipmapFaceRecord>,
    adjacencies: &mut Vec<ClipmapFaceAdjacency>,
) -> bool {
    records.clear();
    adjacencies.clear();

    let Some(minimum_exponent) = leaves
        .iter()
        .map(|key| key.resolution.binary_exponent())
        .min()
    else {
        return true;
    };

    records.reserve(leaves.len().saturating_mul(6));
    for &key in leaves {
        if !push_face_records(key, minimum_exponent, records) {
            return false;
        }
    }

    records.sort_unstable_by_key(|record| {
        (
            record.axis,
            record.plane,
            record.low,
            record.u0,
            record.v0,
            block_sort_key(record.key),
        )
    });

    let mut group_start = 0usize;
    while group_start < records.len() {
        let axis = records[group_start].axis;
        let plane = records[group_start].plane;
        let mut group_end = group_start + 1;
        while group_end < records.len()
            && records[group_end].axis == axis
            && records[group_end].plane == plane
        {
            group_end += 1;
        }

        let group = &records[group_start..group_end];
        let split = group
            .iter()
            .position(|record| record.low)
            .unwrap_or(group.len());
        let highs = &group[..split];
        let lows = &group[split..];

        let mut low_start = 0usize;
        for high in highs {
            while low_start < lows.len()
                && lows[low_start].u1 <= high.u0
            {
                low_start += 1;
            }

            for low in &lows[low_start..] {
                if low.u0 >= high.u1 {
                    break;
                }
                if high.v1.min(low.v1) <= high.v0.max(low.v0) {
                    continue;
                }

                adjacencies.push(ClipmapFaceAdjacency {
                    a: high.key,
                    b: low.key,
                    face_from_a: high.face,
                });
            }
        }

        group_start = group_end;
    }

    true
}

fn balance_leaves_2_to_1_indexed(
    field: CelestialVoxelField,
    leaves: &mut HashSet<CelestialClipmapBlockKey>,
    policy: Option<&DeveloperScalarPolicyRuntime>,
    surface_cache: &mut CelestialClipmapSurfaceCache,
) -> Option<Vec<ClipmapFaceAdjacency>> {
    let mut records =
        Vec::<ClipmapFaceRecord>::with_capacity(leaves.len().saturating_mul(6));
    let mut adjacencies =
        Vec::<ClipmapFaceAdjacency>::with_capacity(leaves.len().saturating_mul(6));
    let mut refine = Vec::<CelestialClipmapBlockKey>::new();

    loop {
        if !rebuild_face_adjacencies(
            leaves,
            &mut records,
            &mut adjacencies,
        ) {
            return None;
        }

        refine.clear();
        for adjacency in &adjacencies {
            let a_exp = adjacency.a.resolution.binary_exponent();
            let b_exp = adjacency.b.resolution.binary_exponent();
            if (i32::from(a_exp) - i32::from(b_exp)).abs() <= 1 {
                continue;
            }

            refine.push(if a_exp > b_exp {
                adjacency.a
            } else {
                adjacency.b
            });
        }

        if refine.is_empty() {
            return Some(adjacencies);
        }

        refine.sort_unstable_by_key(|key| block_sort_key(*key));
        refine.dedup();

        for key in refine.iter().copied() {
            if !leaves.contains(&key) {
                continue;
            }
            if leaves.len().saturating_add(7) > MAX_BALANCED_LEAVES {
                return None;
            }
            if !refine_leaf_indexed(
                leaves,
                key,
                field,
                policy,
                surface_cache,
            ) {
                return None;
            }
        }
    }
}

fn opposite_transition_face(
    face: VoxelTransitionFace,
) -> VoxelTransitionFace {
    match face {
        VoxelTransitionFace::LowX => VoxelTransitionFace::HighX,
        VoxelTransitionFace::HighX => VoxelTransitionFace::LowX,
        VoxelTransitionFace::LowY => VoxelTransitionFace::HighY,
        VoxelTransitionFace::HighY => VoxelTransitionFace::LowY,
        VoxelTransitionFace::LowZ => VoxelTransitionFace::HighZ,
        VoxelTransitionFace::HighZ => VoxelTransitionFace::LowZ,
    }
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
}

impl CelestialClipmapCoverageCell {
    pub(in crate::voxel) const fn center_local_metres(self) -> DVec3 {
        self.center_local_metres
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

    fn owns_presentation(&self, authority: Entity) -> bool {
        self.by_authority.contains_key(&authority)
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
    policy: Option<&DeveloperScalarPolicyRuntime>,
) -> bool {
    let center = key.center_local_metres();
    let radial = center.length();
    if !radial.is_finite() {
        return false;
    }

    let direction = if radial > f64::EPSILON {
        Vec3::new(
            (center.x / radial) as f32,
            (center.y / radial) as f32,
            (center.z / radial) as f32,
        )
        .normalize_or_zero()
    } else {
        Vec3::Y
    };

    let surface_radius = presentation_surface_radius_metres(
        field,
        direction,
        SEMANTIC_SURFACE_SCALE,
        policy,
    )
    .unwrap_or_else(|| field.radius_metres());
    if !surface_radius.is_finite() {
        return false;
    }

    let half_diagonal = key.half_extent_metres().length();
    let conservative_extra = key.extent_metres() * 0.35 + key.spacing_metres() * 2.0;
    (radial - surface_radius).abs() <= half_diagonal + conservative_extra
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
    observer_local: DVec3,
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
    policy_revision: u64,
) -> Option<CelestialClipmapPlanInput> {
    let observer_radius = observer_local.length();
    if !observer_radius.is_finite() {
        return None;
    }

    let clearance =
        (observer_radius - field.radius_metres()).abs();
    if clearance > MAX_CLIPMAP_CLEARANCE_METRES {
        return None;
    }

    let finest_spacing =
        (clearance / 64.0)
            .clamp(
                MIN_SAMPLE_SPACING_METRES,
                MAX_FINE_SAMPLE_SPACING_METRES,
            );
    let finest =
        VoxelPresentationResolution::at_most_metres(finest_spacing)?;

    let local_radius = (BASE_LOCAL_RADIUS_METRES + clearance * 2.0)
        .clamp(BASE_LOCAL_RADIUS_METRES, MAX_LOCAL_RADIUS_METRES);
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
        (coarse_exp_f64 as i16)
            .max(finest.binary_exponent()),
    );

    let fine_extent =
        finest.sample_spacing_metres() * BLOCK_SUBDIVISIONS as f64;
    let observer_bucket =
        checked_floor_coord(observer_local, fine_extent.max(1.0))?;

    Some(CelestialClipmapPlanInput {
        key: CelestialClipmapPlanKey {
            observer_bucket,
            finest_exponent: finest.binary_exponent(),
            coarsest_exponent: coarsest.binary_exponent(),
            policy_revision,
        },
        observer_local,
        finest,
        coarsest,
    })
}

/// Expensive leaf-plan construction.
///
/// Call only when [`derive_plan_input`] reports an identity different from the
/// currently cached plan. On stable frames this function must not run.
fn build_plan(
    field: CelestialVoxelField,
    input: CelestialClipmapPlanInput,
    policy: Option<&DeveloperScalarPolicyRuntime>,
    surface_cache: &mut CelestialClipmapSurfaceCache,
) -> Option<Vec<CelestialClipmapBlockSpec>> {
    surface_cache.begin_plan(field, input.key.policy_revision);

    let result = (|| {
        let observer_local = input.observer_local;
        let finest = input.finest;
        let coarsest = input.coarsest;

        let root_extent =
            coarsest.sample_spacing_metres() * BLOCK_SUBDIVISIONS as f64;
        let root_center = checked_floor_coord(observer_local, root_extent)?;

        let mut leaves =
            HashSet::<CelestialClipmapBlockKey>::with_capacity(MAX_BALANCED_LEAVES);

        {
            let _span =
                bevy::log::info_span!("celestial_clipmap.rebuild_plan.seed").entered();

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
        }

        if leaves.is_empty() {
            return None;
        }

        {
            let _span = bevy::log::info_span!(
                "celestial_clipmap.rebuild_plan.refine_heap"
            )
            .entered();

            let mut candidates =
                BinaryHeap::<ClipmapRefinementCandidate>::with_capacity(
                    MAX_INITIAL_LEAVES,
                );
            for &key in &leaves {
                if let Some(candidate) =
                    refinement_candidate(key, finest, observer_local)
                {
                    candidates.push(candidate);
                }
            }

            while let Some(candidate) = candidates.pop() {
                if !leaves.contains(&candidate.key) {
                    continue;
                }

                if leaves.len().saturating_add(7) > MAX_INITIAL_LEAVES {
                    break;
                }

                if !refine_leaf_indexed(
                    &mut leaves,
                    candidate.key,
                    field,
                    policy,
                    surface_cache,
                ) {
                    break;
                }

                if let Some(children) = candidate.key.children() {
                    for child in children {
                        if !leaves.contains(&child) {
                            continue;
                        }
                        if let Some(next) =
                            refinement_candidate(child, finest, observer_local)
                        {
                            candidates.push(next);
                        }
                    }
                }
            }
        }

        let adjacencies = {
            let _span = bevy::log::info_span!(
                "celestial_clipmap.rebuild_plan.balance_index"
            )
            .entered();

            balance_leaves_2_to_1_indexed(
                field,
                &mut leaves,
                policy,
                surface_cache,
            )?
        };

        let mut transitions =
            HashMap::<CelestialClipmapBlockKey, VoxelTransitionFaces>::with_capacity(
                leaves.len(),
            );

        {
            let _span = bevy::log::info_span!(
                "celestial_clipmap.rebuild_plan.transitions_index"
            )
            .entered();

            for adjacency in adjacencies {
                let a_exp = adjacency.a.resolution.binary_exponent();
                let b_exp = adjacency.b.resolution.binary_exponent();
                let difference = i32::from(a_exp) - i32::from(b_exp);

                if difference == 1 {
                    transitions
                        .entry(adjacency.a)
                        .or_default()
                        .insert(adjacency.face_from_a);
                } else if difference == -1 {
                    transitions
                        .entry(adjacency.b)
                        .or_default()
                        .insert(opposite_transition_face(adjacency.face_from_a));
                }
            }
        }

        let mut ordered = leaves.into_iter().collect::<Vec<_>>();
        ordered.sort_unstable_by_key(|key| block_sort_key(*key));

        let mut specs = Vec::with_capacity(ordered.len());
        for key in ordered {
            specs.push(CelestialClipmapBlockSpec {
                key,
                transition_faces: transitions.remove(&key).unwrap_or_default(),
            });
        }

        Some(specs)
    })();

    {
        let _span = bevy::log::info_span!(
            "celestial_clipmap.rebuild_plan.cache_stats",
            hits = surface_cache.hits as u64,
            misses = surface_cache.misses as u64,
            hot_entries = surface_cache.next.len() as u64,
        )
        .entered();
    }

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
    policy: Option<&DeveloperScalarPolicyRuntime>,
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
        let radial = point.length();
        if !radial.is_finite() || radial <= f64::EPSILON {
            return 1.0;
        }

        let direction = Vec3::new(
            (point.x / radial) as f32,
            (point.y / radial) as f32,
            (point.z / radial) as f32,
        )
        .normalize_or_zero();

        let surface_radius = presentation_surface_radius_metres(
            field,
            direction,
            SEMANTIC_SURFACE_SCALE,
            policy,
        )
        .unwrap_or_else(|| field.radius_metres());

        let signed = surface_radius - radial;
        if signed.is_finite() {
            signed.clamp(
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
        })
        .collect()
}

fn sync_celestial_clipmap_realizations(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    views: Res<UsfViewDemandSnapshot>,
    script_workbench: Res<DeveloperScriptWorkbench>,
    mut planner_policy: ResMut<CelestialClipmapPlannerPolicyCache>,
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
    mut build_tasks: Query<(Entity, &mut CelestialClipmapBuildTask)>,
    mut registry: ResMut<CelestialClipmapRegistry>,
    mut coverage: ResMut<CelestialClipmapCoverageSnapshot>,
) {
    let Some(view) = views.iter().next() else {
        return;
    };

    let presentation_policy = script_workbench.celestial_height_snapshot();
    let policy_revision =
        presentation_policy.as_ref().map_or(0, DeveloperScalarPolicySnapshot::revision);

    let mut live_authorities = HashSet::<Entity>::new();
    let mut plans_changed = false;

    {
        let _span =
            bevy::log::info_span!("celestial_clipmap.plan_identity").entered();

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
            // The first live handoff proves the base semantic field. Edited
            // terrain stays on dense presentation until cross-resolution edit
            // aggregation is explicitly implemented.
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
            let Some(input) =
                derive_plan_input(*field, observer_local, policy_revision)
            else {
                continue;
            };

            let replace = registry
                .plans
                .get(&authority)
                .is_none_or(|plan| plan.key != input.key || plan.field != *field);

            if !replace {
                live_authorities.insert(authority);
                continue;
            }

            let _rebuild_span = bevy::log::info_span!(
                "celestial_clipmap.rebuild_plan",
                ?authority,
            )
            .entered();

            let policy_runtime =
                planner_policy.runtime_for(presentation_policy.as_ref());
            let planner_cache =
                registry.planner_caches.entry(authority).or_default();

            let Some(desired) = build_plan(
                *field,
                input,
                policy_runtime,
                planner_cache,
            ) else {
                continue;
            };
            if desired.is_empty() {
                continue;
            }

            live_authorities.insert(authority);
            let committed_generation = registry
                .plans
                .get(&authority)
                .and_then(|plan| plan.committed_generation);
            let generation = registry.next_generation();
            registry.plans.insert(
                authority,
                CelestialClipmapPlan {
                    key: input.key,
                    field: *field,
                    policy: presentation_policy.clone(),
                    generation,
                    desired,
                    completed: HashSet::new(),
                    committed_generation,
                },
            );
            plans_changed = true;
        }
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

    // clipmap-stable-fast-path-v1
    //
    // A committed plan with no worker tasks is fully reconstructible and needs
    // no maintenance work. In particular, do not scan every clipmap block,
    // rebuild lookup maps, recompile Rhai, or re-evaluate surface planning on a
    // stable frame.
    let no_build_tasks = build_tasks.iter_mut().next().is_none();
    let plans_settled = registry.plans.values().all(|plan| {
        plan.committed_generation == Some(plan.generation)
    });
    if !plans_changed
        && !plans_removed
        && no_build_tasks
        && plans_settled
    {
        return;
    }

    let mut existing_by_spec =
        HashMap::<(Entity, u64, CelestialClipmapBlockSpec), Entity>::new();

    {
        let _span =
            bevy::log::info_span!("celestial_clipmap.index_blocks").entered();

        for (entity, block) in &blocks {
            if !live_authorities.contains(&block.authority) {
                commands.entity(entity).despawn();
                continue;
            }
            let key = (block.authority, block.policy_revision, block.spec);
            if existing_by_spec.contains_key(&key) {
                commands.entity(entity).despawn();
            } else {
                existing_by_spec.insert(key, entity);
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
                }
            }
        }
    }

    let mut inflight =
        HashSet::<(Entity, u64, CelestialClipmapBlockSpec)>::new();
    let mut publications = 0usize;

    {
        let _span =
            bevy::log::info_span!("celestial_clipmap.poll_builds").entered();

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
                commands.entity(task_entity).despawn();
                continue;
            }

            inflight.insert((
                build.authority,
                build.generation,
                build.spec,
            ));

            if publications >= MAX_PUBLICATIONS_PER_FRAME {
                continue;
            }

            let Some(result) = build.task.try_take() else {
                continue;
            };
            commands.entity(task_entity).despawn();
            publications += 1;
            plan.completed.insert(build.spec);

            let Some(mesh) = result else {
                continue;
            };

            let Ok((
                _,
                name,
                _,
                _,
                _,
                _,
                policy,
            )) = authorities.get(build.authority)
            else {
                continue;
            };

            let body_name = name
                .map(|value| value.as_str())
                .unwrap_or("Celestial Body");

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
                    },
                    UsfPresentationProjectionOf(build.authority),
                    Mesh3d(meshes.add(mesh.into_mesh())),
                    MeshMaterial3d(policy.presentation_material().clone()),
                    Transform::IDENTITY,
                    Visibility::Hidden,
                ))
                .id();

            existing_by_spec.insert(
                (build.authority, build.policy_revision, build.spec),
                entity,
            );
        }
    }

    {
        let _span =
            bevy::log::info_span!("celestial_clipmap.schedule_builds").entered();

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
                    || inflight.contains(&(authority, plan.generation, spec))
                {
                    continue;
                }

                if worker_slots == 0
                    || inflight.len() >= MAX_BUILDS_IN_FLIGHT
                    || admitted >= MAX_BUILD_ADMISSIONS_PER_FRAME
                {
                    break 'authorities;
                }

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
                inflight.insert((authority, plan.generation, spec));
                admitted += 1;
                worker_slots -= 1;
            }
        }
    }

    {
        let _span =
            bevy::log::info_span!("celestial_clipmap.commit").entered();

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

            // Do not suppress the proven legacy presentation if the new plan
            // accidentally contains no surface mesh at all.
            let visible_meshes = plan
                .desired
                .iter()
                .filter_map(|spec| {
                    existing_by_spec.get(&(
                        authority,
                        plan.key.policy_revision,
                        *spec,
                    ))
                })
                .count();
            if visible_meshes == 0 {
                continue;
            }

            let desired = plan
                .desired
                .iter()
                .copied()
                .collect::<HashSet<_>>();

            for (entity, block) in &blocks {
                if block.authority != authority {
                    continue;
                }
                if block.policy_revision == plan.key.policy_revision
                    && desired.contains(&block.spec)
                {
                    commands.entity(entity).insert(Visibility::Inherited);
                } else {
                    commands.entity(entity).despawn();
                }
            }
            for &spec in &plan.desired {
                if let Some(&entity) = existing_by_spec.get(&(
                    authority,
                    plan.key.policy_revision,
                    spec,
                )) {
                    commands.entity(entity).insert(Visibility::Inherited);
                }
            }

            coverage.replace_authority(
                authority,
                coverage_for_specs(&plan.desired),
            );
            plan.committed_generation = Some(plan.generation);
        }
    }

    // Stale/inactive build tasks were already retired in the mutable task pass.
}

fn sync_celestial_clipmap_transforms(
    frame: Res<UsfSpatialFrame>,
    interaction: Res<UsfPrimaryInteractionSlice>,
    authorities: Query<(&UsfPosition, &UsfSemanticFrame)>,
    mut blocks: Query<(
        &CelestialClipmapBlock,
        &mut Transform,
        &mut Visibility,
    )>,
) {
    let scale = interaction.scale();
    let metre_to_native = scale.metres_to_native_f32(1.0);
    if !metre_to_native.is_finite() || metre_to_native <= 0.0 {
        return;
    }

    for (block, mut transform, mut visibility) in &mut blocks {
        let Ok((body_origin, body_frame)) =
            authorities.get(block.authority)
        else {
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
        let Ok(translation) = anchor.relative_at_scale_bounded(
            frame.origin(),
            scale,
            RUNTIME_RELATIVE_BOUND_NATIVE,
        ) else {
            *visibility = Visibility::Hidden;
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

        transform.translation = translation;
        transform.rotation = rotation;
        transform.scale = Vec3::splat(metre_to_native);
    }
}

fn suppress_legacy_celestial_dense_presentation(
    coverage: Res<CelestialClipmapCoverageSnapshot>,
    interaction: Res<UsfPrimaryInteractionSlice>,
    runtimes: Query<&VoxelMaterializationRuntime>,
    worlds: Query<(&CelestialVoxelRealization, &UsfScaleLayer)>,
    mut presentations: Query<
        (&ChildOf, &mut Visibility),
        With<VoxelMaterializationPresentation>,
    >,
) {
    for (parent, mut visibility) in &mut presentations {
        let Ok(runtime) = runtimes.get(parent.0) else {
            continue;
        };
        let Ok((realization, layer)) = worlds.get(runtime.world()) else {
            continue;
        };

        // Celestial dense Scale-Slice worlds are no longer a contextual LOD
        // stack. Before clipmap commit only the current interaction slice may
        // remain as physical fallback. After commit the binary x2 clipmap owns
        // local/intermediate presentation entirely.
        let contextual_decimal_slice =
            layer.scale() != interaction.scale();
        let replaced_by_clipmap =
            coverage.owns_presentation(realization.authority());

        if contextual_decimal_slice || replaced_by_clipmap {
            *visibility = Visibility::Hidden;
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
                .in_set(UsfSpatialSet::RuntimeProjection),
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
