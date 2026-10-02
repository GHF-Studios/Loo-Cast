//! Live celestial presentation clipmap over the voxel-local binary resolution domain.
//!
//! This is presentation only. Semantic terrain remains [`CelestialVoxelField`];
//! dense voxel worlds keep collision/editing authority. The clipmap is a
//! reconstructible mesh adapter whose LOD axis is independent of USF Scale.

use std::collections::{HashMap, HashSet};

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

use crate::{
    ecs::UsfPresentationProjectionOf,
    spatial::{
        SpatialScale, UsfPosition, UsfPrimaryInteractionSlice, UsfSemanticFrame,
        UsfSpatialFrame, UsfSpatialSet, UsfViewDemandSnapshot,
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
}

#[derive(Debug)]
struct CelestialClipmapPlan {
    key: CelestialClipmapPlanKey,
    field: CelestialVoxelField,
    generation: u64,
    desired: Vec<CelestialClipmapBlockSpec>,
    completed: HashSet<CelestialClipmapBlockSpec>,
    committed_generation: Option<u64>,
}

#[derive(Resource, Default)]
struct CelestialClipmapRegistry {
    next_generation: u64,
    plans: HashMap<Entity, CelestialClipmapPlan>,
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
    spec: CelestialClipmapBlockSpec,
}

#[derive(Component)]
struct CelestialClipmapBuildTask {
    authority: Entity,
    generation: u64,
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

    let surface_radius = field
        .surface_local_metres(direction, SEMANTIC_SURFACE_SCALE)
        .map(|point| point.length())
        .unwrap_or_else(|_| field.radius_metres());
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
) -> bool {
    let parent = leaves.swap_remove(index);
    let Some(children) = parent.children() else {
        leaves.push(parent);
        return false;
    };

    for child in children {
        if block_intersects_semantic_surface(field, child) {
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
        if !refine_leaf(leaves, index, field) {
            return false;
        }
    }
}

fn build_plan(
    field: CelestialVoxelField,
    observer_local: DVec3,
) -> Option<(CelestialClipmapPlanKey, Vec<CelestialClipmapBlockSpec>)> {
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

    let root_extent =
        coarsest.sample_spacing_metres() * BLOCK_SUBDIVISIONS as f64;
    let root_center = checked_floor_coord(observer_local, root_extent)?;

    let mut leaves = Vec::with_capacity(64);
    for z in -1..=1 {
        for y in -1..=1 {
            for x in -1..=1 {
                let Some(cx) = root_center.x.checked_add(x) else {
                    return None;
                };
                let Some(cy) = root_center.y.checked_add(y) else {
                    return None;
                };
                let Some(cz) = root_center.z.checked_add(z) else {
                    return None;
                };
                let coord = IVec3::new(cx, cy, cz);
                let key = CelestialClipmapBlockKey {
                    resolution: coarsest,
                    coord,
                };
                if block_intersects_semantic_surface(field, key) {
                    leaves.push(key);
                }
            }
        }
    }

    if leaves.is_empty() {
        return None;
    }

    loop {
        let mut best = None::<(usize, f64)>;
        for (index, &key) in leaves.iter().enumerate() {
            if key.resolution <= finest {
                continue;
            }

            let distance = block_distance_to_point(key, observer_local);
            let target = target_resolution_at_distance(finest, distance);
            if key.resolution <= target {
                continue;
            }

            if best.is_none_or(|(_, current)| distance < current) {
                best = Some((index, distance));
            }
        }

        let Some((index, _)) = best else {
            break;
        };
        if leaves.len().saturating_add(7) > MAX_INITIAL_LEAVES {
            break;
        }
        if !refine_leaf(&mut leaves, index, field) {
            break;
        }
    }

    if !balance_leaves_2_to_1(field, &mut leaves) {
        return None;
    }

    leaves.sort_unstable_by_key(|key| {
        (
            key.resolution.binary_exponent(),
            key.coord.x,
            key.coord.y,
            key.coord.z,
        )
    });

    let mut specs = Vec::with_capacity(leaves.len());
    for (index, &key) in leaves.iter().enumerate() {
        let mut transition_faces = VoxelTransitionFaces::default();

        for (other_index, &other) in leaves.iter().enumerate() {
            if index == other_index {
                continue;
            }
            if other
                .resolution
                .binary_exponent()
                .checked_add(1)
                != Some(key.resolution.binary_exponent())
            {
                continue;
            }
            if let Some(face) = face_from_a_to_b(key, other) {
                transition_faces.insert(face);
            }
        }

        specs.push(CelestialClipmapBlockSpec {
            key,
            transition_faces,
        });
    }

    let fine_extent =
        finest.sample_spacing_metres() * BLOCK_SUBDIVISIONS as f64;
    let observer_bucket =
        checked_floor_coord(observer_local, fine_extent.max(1.0))?;

    Some((
        CelestialClipmapPlanKey {
            observer_bucket,
            finest_exponent: finest.binary_exponent(),
            coarsest_exponent: coarsest.binary_exponent(),
        },
        specs,
    ))
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

        let surface_radius = field
            .surface_local_metres(direction, SEMANTIC_SURFACE_SCALE)
            .map(|surface| surface.length())
            .unwrap_or_else(|_| field.radius_metres());

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

    let mut live_authorities = HashSet::<Entity>::new();
    let mut authority_names = HashMap::<Entity, String>::new();
    let mut authority_materials =
        HashMap::<Entity, Handle<StandardMaterial>>::new();

    for (
        authority,
        name,
        body_origin,
        body_frame,
        field,
        voxel_authority,
        policy,
    ) in &authorities
    {
        // The first live handoff proves the base semantic field. Edited terrain
        // stays on the existing dense presentation until cross-resolution edit
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
        let Some((key, desired)) = build_plan(*field, observer_local) else {
            continue;
        };
        if desired.is_empty() {
            continue;
        }

        live_authorities.insert(authority);
        authority_names.insert(
            authority,
            name.map(|value| value.as_str().to_string())
                .unwrap_or_else(|| "Celestial Body".to_string()),
        );
        authority_materials.insert(
            authority,
            policy.presentation_material().clone(),
        );

        let replace = registry
            .plans
            .get(&authority)
            .is_none_or(|plan| plan.key != key || plan.field != *field);

        if replace {
            let committed_generation = registry
                .plans
                .get(&authority)
                .and_then(|plan| plan.committed_generation);
            let generation = registry.next_generation();
            registry.plans.insert(
                authority,
                CelestialClipmapPlan {
                    key,
                    field: *field,
                    generation,
                    desired,
                    completed: HashSet::new(),
                    committed_generation,
                },
            );
        }
    }

    registry
        .plans
        .retain(|authority, _| live_authorities.contains(authority));

    let mut existing_by_spec =
        HashMap::<(Entity, CelestialClipmapBlockSpec), Entity>::new();
    for (entity, block) in &blocks {
        if !live_authorities.contains(&block.authority) {
            commands.entity(entity).despawn();
            continue;
        }
        let key = (block.authority, block.spec);
        if existing_by_spec.contains_key(&key) {
            commands.entity(entity).despawn();
        } else {
            existing_by_spec.insert(key, entity);
        }
    }

    for (&authority, plan) in &mut registry.plans {
        for &spec in &plan.desired {
            if existing_by_spec.contains_key(&(authority, spec)) {
                plan.completed.insert(spec);
            }
        }
    }

    let mut inflight =
        HashSet::<(Entity, u64, CelestialClipmapBlockSpec)>::new();
    let mut publications = 0usize;

    for (task_entity, mut build) in &mut build_tasks {
        let Some(plan) = registry.plans.get_mut(&build.authority) else {
            commands.entity(task_entity).despawn();
            continue;
        };

        if build.generation != plan.generation
            || !plan.desired.contains(&build.spec)
            || build.field != plan.field
        {
            commands.entity(task_entity).despawn();
            continue;
        }

        if existing_by_spec.contains_key(&(build.authority, build.spec)) {
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
        let Some(material) =
            authority_materials.get(&build.authority).cloned()
        else {
            continue;
        };
        let body_name = authority_names
            .get(&build.authority)
            .map(String::as_str)
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
                    spec: build.spec,
                },
                UsfPresentationProjectionOf(build.authority),
                Mesh3d(meshes.add(mesh.into_mesh())),
                MeshMaterial3d(material),
                Transform::IDENTITY,
                Visibility::Hidden,
            ))
            .id();

        existing_by_spec.insert((build.authority, build.spec), entity);
    }

    let mut admitted = 0usize;
    let mut worker_slots =
        workers.available_slots(VoxelWorkerLane::PresentationResolution);

    'authorities: for (&authority, plan) in &registry.plans {
        for &spec in &plan.desired {
            if plan.completed.contains(&spec)
                || existing_by_spec.contains_key(&(authority, spec))
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
            let Some(task) = workers.try_submit(
                VoxelWorkerLane::PresentationResolution,
                move || build_clipmap_mesh(field, spec),
            ) else {
                break 'authorities;
            };

            commands.spawn((
                Name::new("Celestial Binary Clipmap Build"),
                VoxelWorkerTask,
                CelestialClipmapBuildTask {
                    authority,
                    generation: plan.generation,
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
            .filter_map(|spec| existing_by_spec.get(&(authority, *spec)))
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
            if desired.contains(&block.spec) {
                commands.entity(entity).insert(Visibility::Inherited);
            } else {
                commands.entity(entity).despawn();
            }
        }
        for &spec in &plan.desired {
            if let Some(&entity) = existing_by_spec.get(&(authority, spec)) {
                commands.entity(entity).insert(Visibility::Inherited);
            }
        }

        coverage.replace_authority(
            authority,
            coverage_for_specs(&plan.desired),
        );
        plan.committed_generation = Some(plan.generation);
    }

    // Stale/inactive build tasks were already retired in the mutable task pass
    // above; do not attempt a second immutable iteration over a Query carrying
    // `&mut CelestialClipmapBuildTask`.
    coverage.retain_authorities(&live_authorities);
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
    runtimes: Query<&VoxelMaterializationRuntime>,
    worlds: Query<&CelestialVoxelRealization>,
    mut presentations: Query<
        (&ChildOf, &mut Visibility),
        With<VoxelMaterializationPresentation>,
    >,
) {
    for (parent, mut visibility) in &mut presentations {
        let Ok(runtime) = runtimes.get(parent.0) else {
            continue;
        };
        let Ok(realization) = worlds.get(runtime.world()) else {
            continue;
        };

        if coverage.owns_presentation(realization.authority()) {
            *visibility = Visibility::Hidden;
        }
    }
}

pub(super) fn configure(app: &mut App) {
    app.init_resource::<CelestialClipmapRegistry>()
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
