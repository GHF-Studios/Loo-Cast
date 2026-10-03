//! Bounded adaptive regional whole-body presentation for semantic celestial fields.
//!
//! Dense voxel materializations do not draw whole planets. This representation
//! derives a small body-local cubed-sphere frontier from observer geometry and
//! samples the outer radial boundary derived from `CelestialVoxelField`.
//!
//! This is deliberately a whole-body outer-shell approximation. Volumetric
//! topology such as caves/overhangs belongs to local clipmap/dense realizers;
//! this adapter must never claim local collision or physical terrain authority.
//!
//! Critical work bound: selection is a bounded frontier, never an unbounded
//! recursive quadtree walk. Camera/view state never creates dense voxel demand.

use std::{collections::{HashMap, HashSet}, sync::Arc};

use bevy::{
    math::DVec3,
    prelude::*,
};

use crate::{
    reconstructible::{ReconstructibleFrameBudget, ReconstructibleWorkClass},
    devtools::{
        DeveloperScalarPolicyRuntime, DeveloperScalarPolicySnapshot,
        DeveloperScriptWorkbench,
    },
    ecs::UsfPresentationProjectionOf,
    spatial::{
        SpatialRealizationGranularityRequest, SpatialScale,
        UsfScaleCoverageSnapshot, UsfScaleRoleMask,
        UsfSceneryPresentation, UsfSemanticFrame, UsfViewDemand, UsfViewDemandSnapshot,
        UsfPosition,
    },
};

use super::{
    CelestialVoxelField, CelestialVoxelRealizationPolicy,
    developer_policy::presentation_surface_local_metres,
    resolution::{CelestialClipmapCoverageCell, CelestialClipmapCoverageSnapshot},
    worker::{VoxelWorkerLane, VoxelWorkerPool, VoxelWorkerTask, VoxelWorkerTicket},
};

mod mesh;
use mesh::{PATCH_GRID_RESOLUTION, build_planetary_surface_patch};

const PLANETARY_SAMPLE_SCALE: i8 = 4;

/// Hard representation-work ceiling per celestial authority.
///
/// Refining one frontier leaf replaces it with four children (+3 leaves). The
/// selector refuses a refinement that would cross this bound and keeps the
/// parent instead. Correctness therefore degrades by representation error, not
/// by unbounded frame work.
const MAX_PATCH_LEAVES: usize = 64;

/// Absolute safety ceiling. The semantic sample spacing normally produces a
/// much lower body-specific maximum (Earth at S+4 resolves to about L7).
const MAX_ABSOLUTE_PATCH_LEVEL: u8 = 10;

// regional-aperture-cutout-v1
//
// Semantic terrain bandwidth and presentation ownership are separate axes.
// Around committed dense/clipmap coverage, regional patches may subdivide a
// few extra levels solely to localize the replacement boundary. They still
// sample the same regional semantic field and remain inside the existing hard
// absolute-depth and leaf-count work bounds.
const APERTURE_CUTOUT_EXTRA_LEVELS: u8 = 3;

const PROJECTED_ERROR_RATIO: f64 = 0.24;
const PATCH_BOUND_MARGIN: f64 = 1.30;
const PLANETARY_VALIDITY_AGGREGATES_ACROSS: u32 = 4;
const PLANETARY_MIN_VALIDITY_SECONDS: f64 = 0.25;
const PLANETARY_MAX_VALIDITY_SECONDS: f64 = 4.0;
const PLANETARY_LATENCY_MULTIPLIER: f64 = 4.0;

const MAX_PATCH_BUILDS_IN_FLIGHT: usize = 2;
const MAX_PATCH_BUILD_ADMISSIONS_PER_FRAME: usize = 2;
const MAX_PATCH_PUBLICATIONS_PER_FRAME: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlanetarySurfaceFace {
    PositiveX,
    NegativeX,
    PositiveY,
    NegativeY,
    PositiveZ,
    NegativeZ,
}

impl PlanetarySurfaceFace {
    const ALL: [Self; 6] = [
        Self::PositiveX,
        Self::NegativeX,
        Self::PositiveY,
        Self::NegativeY,
        Self::PositiveZ,
        Self::NegativeZ,
    ];

    fn cube_point(self, u: f32, v: f32) -> Vec3 {
        match self {
            Self::PositiveX => Vec3::new(1.0, v, -u),
            Self::NegativeX => Vec3::new(-1.0, v, u),
            Self::PositiveY => Vec3::new(u, 1.0, -v),
            Self::NegativeY => Vec3::new(u, -1.0, v),
            Self::PositiveZ => Vec3::new(u, v, 1.0),
            Self::NegativeZ => Vec3::new(-u, v, -1.0),
        }
    }
}

/// Cubed-sphere regional representation identity.
///
/// Patch level is representation refinement only; it is not a USF Scale.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlanetarySurfacePatchId {
    pub face: PlanetarySurfaceFace,
    pub level: u8,
    pub x: u32,
    pub y: u32,
}

impl PlanetarySurfacePatchId {
    fn root(face: PlanetarySurfaceFace) -> Self {
        Self {
            face,
            level: 0,
            x: 0,
            y: 0,
        }
    }

    fn roots() -> impl Iterator<Item = Self> {
        PlanetarySurfaceFace::ALL.into_iter().map(Self::root)
    }

    fn children(self) -> [Self; 4] {
        let next = self.level + 1;
        let x = self.x * 2;
        let y = self.y * 2;
        [
            Self { face: self.face, level: next, x, y },
            Self { face: self.face, level: next, x: x + 1, y },
            Self { face: self.face, level: next, x, y: y + 1 },
            Self { face: self.face, level: next, x: x + 1, y: y + 1 },
        ]
    }

    fn uv_bounds(self) -> (f32, f32, f32, f32) {
        let subdivisions = 1_u32 << self.level;
        let size = 2.0 / subdivisions as f32;
        let u0 = -1.0 + self.x as f32 * size;
        let v0 = -1.0 + self.y as f32 * size;
        (u0, u0 + size, v0, v0 + size)
    }

    pub fn direction_at(self, u: f32, v: f32) -> Vec3 {
        let (u0, u1, v0, v1) = self.uv_bounds();
        let u = u0 + (u1 - u0) * u.clamp(0.0, 1.0);
        let v = v0 + (v1 - v0) * v.clamp(0.0, 1.0);
        self.face.cube_point(u, v).normalize()
    }

    fn center_direction(self) -> Vec3 {
        self.direction_at(0.5, 0.5)
    }

    fn approximate_radius_metres(self, body_radius_metres: f64) -> f64 {
        let subdivisions = 2.0_f64.powi(i32::from(self.level));
        body_radius_metres * (2.0 / subdivisions) * PATCH_BOUND_MARGIN
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlanetarySurfaceRealization {
    authority: Entity,
    patch: PlanetarySurfacePatchId,
    scale: SpatialScale,
    revision: u64,
}

impl PlanetarySurfaceRealization {
    pub const fn authority(self) -> Entity {
        self.authority
    }

    pub const fn patch(self) -> PlanetarySurfacePatchId {
        self.patch
    }

    pub const fn scale(self) -> SpatialScale {
        self.scale
    }

    pub const fn revision(self) -> u64 {
        self.revision
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PatchDecision {
    Cull,
    Keep,
    Refine,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DenseCoverageRelation {
    None,
    Partial,
    Full,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PlanetaryObserverKey {
    bucket: [i64; 3],
    validity_exponent: i16,
    detail_exponent: i16,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct PlanetaryDenseCoverageGeometry {
    realization: Entity,
    scale: SpatialScale,
    center: UsfPosition,
    half_extent_native: Vec3,
}

/// One ready dense aperture projected once into the semantic body's local SI
/// frame.
///
/// Exact materialization geometry remains canonical elsewhere. Regional
/// presentation only needs conservative overlap: an inscribed sphere proves
/// full replacement, while a circumscribed sphere conservatively detects
/// partial overlap. Patch selection therefore never performs canonical
/// cross-scale arithmetic in its patch × materialization inner loop.
#[derive(Debug, Clone, Copy)]
struct PlanetaryDenseCoverageLocal {
    geometry: PlanetaryDenseCoverageGeometry,
    center_local_metres: DVec3,
    inner_radius_metres: f64,
    outer_radius_metres: f64,
}

#[derive(Debug, Clone, Copy)]
struct PlanetaryDenseCoverageBounds {
    min: DVec3,
    max: DVec3,
}

impl PlanetaryDenseCoverageBounds {
    fn around(coverage: PlanetaryDenseCoverageLocal) -> Self {
        let radius = DVec3::splat(coverage.outer_radius_metres);
        Self {
            min: coverage.center_local_metres - radius,
            max: coverage.center_local_metres + radius,
        }
    }

    fn include(&mut self, coverage: PlanetaryDenseCoverageLocal) {
        let radius = DVec3::splat(coverage.outer_radius_metres);
        self.min = self.min.min(coverage.center_local_metres - radius);
        self.max = self.max.max(coverage.center_local_metres + radius);
    }

    fn intersects_sphere(self, center: DVec3, radius: f64) -> bool {
        let nearest = center.clamp(self.min, self.max);
        (center - nearest).length_squared() <= radius * radius
    }
}

#[derive(Debug)]
struct PlanetarySurfacePlanState {
    observer: PlanetaryObserverKey,
    field: CelestialVoxelField,
    policy_revision: u64,
    policy: Option<DeveloperScalarPolicySnapshot>,
    body_origin: UsfPosition,
    body_frame: UsfSemanticFrame,
    dense_coverage: Vec<PlanetaryDenseCoverageGeometry>,
    dense_local: Arc<HashMap<Entity, PlanetaryDenseCoverageLocal>>,
    dense_bounds: Option<PlanetaryDenseCoverageBounds>,
    clipmap_coverage: Vec<CelestialClipmapCoverageCell>,
    desired: Vec<PlanetarySurfacePatchId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct PlanetarySurfacePatchKey {
    authority: Entity,
    patch: PlanetarySurfacePatchId,
    scale: SpatialScale,
    policy_revision: u64,
}

#[derive(Component)]
pub(super) struct PlanetarySurfaceBuildTask {
    authority: Entity,
    patch: PlanetarySurfacePatchId,
    field: CelestialVoxelField,
    sample_scale: SpatialScale,
    policy_revision: u64,
    task: VoxelWorkerTicket<Option<Mesh>>,
}

#[derive(Default)]
pub(super) struct PlanetarySurfacePlanCache {
    plans: HashMap<Entity, PlanetarySurfacePlanState>,
    coverage_revision: u64,
    coverage_by_authority:
        HashMap<Entity, Vec<PlanetaryDenseCoverageGeometry>>,
}

/// Reconcile presentation patches only when body-relative observer geometry,
/// semantic field definition or dense presentation coverage changes materially.
///
/// Tiny continuous observer motion stays within a quantized observer key, so a
/// stable planet does not pay for a full regional replan every Update.
pub(super) fn sync_planetary_surface_realizations(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    views: Res<UsfViewDemandSnapshot>,
    script_workbench: Res<DeveloperScriptWorkbench>,
    coverage: Res<UsfScaleCoverageSnapshot>,
    clipmap_coverage: Res<CelestialClipmapCoverageSnapshot>,
    workers: Res<VoxelWorkerPool>,
    authorities: Query<(
        Entity,
        Option<&Name>,
        &UsfPosition,
        &UsfSemanticFrame,
        &CelestialVoxelField,
        &CelestialVoxelRealizationPolicy,
    )>,
    existing: Query<(Entity, &PlanetarySurfaceRealization)>,
    mut build_tasks: Query<(Entity, &mut PlanetarySurfaceBuildTask)>,
    mut cache: Local<PlanetarySurfacePlanCache>,
    mut frame_budget: ResMut<ReconstructibleFrameBudget>,
) {
    let Some(view) = views.iter().next() else { return; };
    let presentation_policy = script_workbench.celestial_height_snapshot();
    let policy_revision =
        presentation_policy.as_ref().map_or(0, DeveloperScalarPolicySnapshot::revision);
    let policy_runtime = presentation_policy
        .as_ref()
        .and_then(|snapshot| snapshot.compile_runtime().ok());
    let mut live_authorities = HashSet::new();

    {
        let _span = bevy::log::info_span!("planetary_surface.plan").entered();

        // Capability topology usually stays stable for many frames. Rebuild the
        // authority grouping only when the generic coverage snapshot revision
        // actually changes.
        if cache.coverage_revision != coverage.revision() {
            let _span =
                bevy::log::info_span!("planetary_surface.plan.coverage_collect").entered();
            cache.coverage_by_authority.clear();
            for entry in coverage.iter() {
                if !entry.roles().contains(UsfScaleRoleMask::PRESENTATION) {
                    continue;
                }
                cache
                    .coverage_by_authority
                    .entry(entry.authority())
                    .or_default()
                    .push(PlanetaryDenseCoverageGeometry {
                        realization: entry.realization(),
                        scale: entry.scale(),
                        center: entry.center(),
                        half_extent_native: entry.half_extent_native(),
                    });
            }
            cache.coverage_revision = coverage.revision();
        }

        for (authority, _name, body_origin, body_frame, field, _policy) in &authorities {
            live_authorities.insert(authority);
            let sample_scale = planetary_sample_scale(*field);
            let expected_build_seconds = workers
                .estimated_latency_seconds(VoxelWorkerLane::PlanetarySurface);
            let Some((observer_key, observer_local, max_level)) =
                observer_plan_key(
                    *body_origin,
                    *body_frame,
                    *field,
                    sample_scale,
                    view,
                    expected_build_seconds,
                )
            else {
                continue;
            };

            let dense_geometry = cache
                .coverage_by_authority
                .get(&authority)
                .cloned()
                .unwrap_or_default();
            let clipmap_geometry =
                clipmap_coverage.for_authority(authority).to_vec();

            let previous = cache.plans.get(&authority);
            let coverage_changed = previous.is_none_or(|plan| {
                plan.dense_coverage != dense_geometry
                    || plan.clipmap_coverage != clipmap_geometry
                    || plan.body_origin != *body_origin
                    || plan.body_frame != *body_frame
            });
            let selection_changed = coverage_changed || previous.is_none_or(|plan| {
                plan.observer != observer_key
                    || plan.field != *field
                    || plan.policy_revision != policy_revision
            });
            if !selection_changed {
                continue;
            }

            let Some(plan_token) =
                frame_budget.begin(ReconstructibleWorkClass::Planning)
            else {
                continue;
            };

            // Canonical -> body-local projection happens at most once for each
            // changed materialization geometry. Unchanged ready chunks reuse the
            // cached projection across observer bucket changes and later plans.
            let (dense_local, dense_bounds) = if coverage_changed {
                let _span =
                    bevy::log::info_span!("planetary_surface.plan.coverage_sync").entered();
                let mut next =
                    HashMap::<Entity, PlanetaryDenseCoverageLocal>::with_capacity(
                        dense_geometry.len(),
                    );
                let mut bounds = None::<PlanetaryDenseCoverageBounds>;

                for geometry in dense_geometry.iter().copied() {
                    let local = if let Some(cached) = previous
                        .and_then(|plan| plan.dense_local.get(&geometry.realization))
                        .copied()
                        .filter(|cached| {
                            cached.geometry == geometry
                                && previous.is_some_and(|plan| {
                                    plan.body_origin == *body_origin
                                        && plan.body_frame == *body_frame
                                })
                        })
                    {
                        cached
                    } else {
                        let Ok(center_local_metres) = body_frame.world_to_local_metres(
                            body_origin,
                            &geometry.center,
                            geometry.scale,
                            f64::MAX,
                        ) else {
                            continue;
                        };

                        let metres_per_native = geometry.scale.metres_per_native();
                        let half = geometry.half_extent_native;
                        let half_metres = DVec3::new(
                            f64::from(half.x) * metres_per_native,
                            f64::from(half.y) * metres_per_native,
                            f64::from(half.z) * metres_per_native,
                        );
                        if !center_local_metres.is_finite() || !half_metres.is_finite() {
                            continue;
                        }

                        PlanetaryDenseCoverageLocal {
                            geometry,
                            center_local_metres,
                            inner_radius_metres: half_metres.min_element().max(0.0),
                            outer_radius_metres: half_metres.length(),
                        }
                    };

                    if let Some(bounds) = bounds.as_mut() {
                        bounds.include(local);
                    } else {
                        bounds = Some(PlanetaryDenseCoverageBounds::around(local));
                    }
                    next.insert(geometry.realization, local);
                }

                (Arc::new(next), bounds)
            } else {
                let previous =
                    previous.expect("unchanged coverage requires an existing plan");
                (Arc::clone(&previous.dense_local), previous.dense_bounds)
            };

            let selected = {
                let _span =
                    bevy::log::info_span!("planetary_surface.plan.select").entered();
                select_adaptive_patches(|patch| {
                    evaluate_patch(
                        *field,
                        sample_scale,
                        patch,
                        observer_local,
                        max_level,
                        dense_local.as_ref(),
                        dense_bounds,
                        &clipmap_geometry,
                        policy_runtime.as_ref(),
                    )
                })
            };

            cache.plans.insert(authority, PlanetarySurfacePlanState {
                observer: observer_key,
                field: *field,
                policy_revision,
                policy: presentation_policy.clone(),
                body_origin: *body_origin,
                body_frame: *body_frame,
                dense_coverage: dense_geometry,
                dense_local,
                dense_bounds,
                clipmap_coverage: clipmap_geometry,
                desired: selected,
            });
            frame_budget.finish(plan_token);
        }
    }

    let mut existing_by_key = HashMap::<PlanetarySurfacePatchKey, Entity>::new();
    for (entity, realization) in &existing {
        let key = PlanetarySurfacePatchKey {
            authority: realization.authority(),
            patch: realization.patch(),
            scale: realization.scale(),
            policy_revision: realization.revision(),
        };
        if existing_by_key.contains_key(&key) {
            commands.entity(entity).despawn();
        } else {
            existing_by_key.insert(key, entity);
        }
    }

    let mut inflight_keys = HashSet::<PlanetarySurfacePatchKey>::new();
    let mut published_keys = HashSet::<PlanetarySurfacePatchKey>::new();
    let mut published = 0usize;

    {
        let _span = bevy::log::info_span!("planetary_surface.publish").entered();
        for (task_entity, mut build) in &mut build_tasks {
            let key = PlanetarySurfacePatchKey {
                authority: build.authority,
                patch: build.patch,
                scale: build.sample_scale,
                policy_revision: build.policy_revision,
            };
            let still_desired = cache.plans.get(&build.authority).is_some_and(|plan| {
                plan.field == build.field
                    && plan.policy_revision == build.policy_revision
                    && planetary_sample_scale(plan.field) == build.sample_scale
                    && plan.desired.contains(&build.patch)
            });
            if !still_desired || !live_authorities.contains(&build.authority) {
                commands.entity(task_entity).despawn();
                continue;
            }
            if existing_by_key.contains_key(&key) {
                commands.entity(task_entity).despawn();
                continue;
            }

            if published < MAX_PATCH_PUBLICATIONS_PER_FRAME {
                let Some(work_token) =
                    frame_budget.begin(ReconstructibleWorkClass::Publication)
                else {
                    inflight_keys.insert(key);
                    continue;
                };
                if let Some(result) = build.task.try_take() {
                    commands.entity(task_entity).despawn();
                    let Some(mesh) = result else { continue; };
                    let Ok((_, name, body_origin, body_frame, current_field, policy)) =
                        authorities.get(build.authority)
                    else { continue; };
                    if *current_field != build.field { continue; }

                    let body_name = name.map(|v| v.as_str()).unwrap_or("Celestial Body");
                    let q = body_frame.orientation();
                    let rotation = Quat::from_xyzw(
                        q.x as f32, q.y as f32, q.z as f32, q.w as f32,
                    ).normalize();
                    commands.spawn((
                        Name::new(format!(
                            "{body_name} Planetary Patch {:?} L{} ({},{})",
                            build.patch.face, build.patch.level, build.patch.x, build.patch.y,
                        )),
                        // planetary-surface-policy-revision-publication-fix-v1
                        //
                        // Publication identity must exactly match the scheduler/build key.
                        // A hardcoded revision prevents existing_by_key from recognizing
                        // freshly published patches, so make-before-break retirement never
                        // completes and old regional patch entities accumulate while moving.
                        PlanetarySurfaceRealization {
                            authority: build.authority,
                            patch: build.patch,
                            scale: build.sample_scale,
                            revision: build.policy_revision,
                        },
                        UsfPresentationProjectionOf(build.authority),
                        UsfSceneryPresentation::from_anchor(*body_origin, build.sample_scale),
                        Mesh3d(meshes.add(mesh)),
                        MeshMaterial3d(policy.presentation_material().clone()),
                        Transform::from_rotation(rotation),
                        Visibility::Inherited,
                    ));
                    published_keys.insert(key);
                    published += 1;
                    frame_budget.finish(work_token);
                    continue;
                }
                frame_budget.finish(work_token);
            }
            inflight_keys.insert(key);
        }
    }

    {
        let _span = bevy::log::info_span!("planetary_surface.schedule").entered();
        let mut admitted = 0usize;
        let mut worker_slots = workers.available_slots(VoxelWorkerLane::PlanetarySurface);
        'authorities: for (&authority, plan) in &cache.plans {
            let sample_scale = planetary_sample_scale(plan.field);
            for &patch in &plan.desired {
                let key = PlanetarySurfacePatchKey {
                    authority,
                    patch,
                    scale: sample_scale,
                    policy_revision: plan.policy_revision,
                };
                if existing_by_key.contains_key(&key)
                    || published_keys.contains(&key)
                    || inflight_keys.contains(&key)
                { continue; }
                if worker_slots == 0
                    || inflight_keys.len() >= MAX_PATCH_BUILDS_IN_FLIGHT
                    || admitted >= MAX_PATCH_BUILD_ADMISSIONS_PER_FRAME
                { break 'authorities; }

                let Some(work_token) =
                    frame_budget.begin(ReconstructibleWorkClass::Maintenance)
                else {
                    break 'authorities;
                };
                let field = plan.field;
                let policy = plan.policy.clone();
                let policy_revision = plan.policy_revision;
                let Some(task) = workers.try_submit(
                    VoxelWorkerLane::PlanetarySurface,
                    move || {
                        let runtime = policy
                            .as_ref()
                            .and_then(|snapshot| snapshot.compile_runtime().ok());
                        build_planetary_surface_patch(
                            field,
                            patch,
                            sample_scale,
                            runtime.as_ref(),
                        )
                    },
                ) else {
                    break 'authorities;
                };
                commands.spawn((
                    Name::new("Planetary Surface Patch Build"),
                    VoxelWorkerTask,
                    PlanetarySurfaceBuildTask {
                        authority,
                        patch,
                        field,
                        sample_scale,
                        policy_revision,
                        task,
                    },
                ));
                inflight_keys.insert(key);
                admitted += 1;
                worker_slots -= 1;
                frame_budget.finish(work_token);
            }
        }
    }

    {
        let _span = bevy::log::info_span!("planetary_surface.retire").entered();
        for (&authority, plan) in &cache.plans {
            let sample_scale = planetary_sample_scale(plan.field);
            let replacements_ready = plan.desired.iter().all(|&patch| {
                let key = PlanetarySurfacePatchKey {
                    authority,
                    patch,
                    scale: sample_scale,
                    policy_revision: plan.policy_revision,
                };
                existing_by_key.contains_key(&key) || published_keys.contains(&key)
            });
            if !replacements_ready { continue; }
            for (&key, &entity) in &existing_by_key {
                if key.authority != authority { continue; }
                if key.scale != sample_scale
                    || key.policy_revision != plan.policy_revision
                    || !plan.desired.contains(&key.patch)
                {
                    commands.entity(entity).despawn();
                }
            }
        }
        for (&key, &entity) in &existing_by_key {
            if !live_authorities.contains(&key.authority) {
                commands.entity(entity).despawn();
            }
        }
        cache.plans.retain(|authority, _| live_authorities.contains(authority));
    }
}

pub(super) fn sync_planetary_surface_projection_state(
    authorities: Query<(&UsfPosition, &UsfSemanticFrame)>,
    mut patches: Query<(
        &PlanetarySurfaceRealization,
        &mut UsfSceneryPresentation,
        &mut Transform,
    )>,
) {
    for (realization, mut presentation, mut transform) in &mut patches {
        let Ok((body_origin, body_frame)) = authorities.get(realization.authority()) else {
            continue;
        };

        presentation.set_anchor(*body_origin);

        let orientation = body_frame.orientation();
        let rotation = Quat::from_xyzw(
            orientation.x as f32,
            orientation.y as f32,
            orientation.z as f32,
            orientation.w as f32,
        )
        .normalize();
        if transform.rotation != rotation {
            transform.rotation = rotation;
        }
    }
}

fn planetary_sample_scale(field: CelestialVoxelField) -> SpatialScale {
    let preferred =
        SpatialScale::new(PLANETARY_SAMPLE_SCALE).expect("planetary sample scale is valid");
    field.coarsest_detail_scale().min(preferred)
}

/// Body-specific representation ceiling derived from semantic sample spacing.
///
/// Refining a regional mesh below roughly one sample-scale native unit per mesh
/// segment cannot reveal additional semantic terrain owned by this band; doing
/// so would be pure representation churn.
fn maximum_patch_level_for_spacing(
    field: CelestialVoxelField,
    minimum_segment_metres: f64,
) -> u8 {
    let root_span_metres = field.radius_metres() * 2.0;
    let minimum_segment_metres =
        minimum_segment_metres.max(f64::MIN_POSITIVE);
    let useful_subdivisions =
        root_span_metres
            / (minimum_segment_metres
                * f64::from(PATCH_GRID_RESOLUTION));
    if !useful_subdivisions.is_finite() || useful_subdivisions <= 1.0 {
        return 0;
    }

    useful_subdivisions
        .log2()
        .floor()
        .clamp(0.0, f64::from(MAX_ABSOLUTE_PATCH_LEVEL)) as u8
}



/// Maximum regional depth allowed solely to carve a local replacement aperture.
///
/// Ordinary significance/detail refinement remains capped by the semantic
/// sample-spacing ceiling. This extra depth changes ownership granularity, not
/// terrain information.
fn maximum_aperture_patch_level(detail_max_level: u8) -> u8 {
    detail_max_level
        .saturating_add(APERTURE_CUTOUT_EXTRA_LEVELS)
        .min(MAX_ABSOLUTE_PATCH_LEVEL)
}

fn observer_plan_key(
    body_origin: UsfPosition,
    body_frame: UsfSemanticFrame,
    field: CelestialVoxelField,
    sample_scale: SpatialScale,
    view: &UsfViewDemand,
    expected_build_seconds: f64,
) -> Option<(PlanetaryObserverKey, DVec3, u8)> {
    let local = body_frame
        .world_to_local_metres(
            &body_origin,
            &view.anchor(),
            sample_scale,
            f64::MAX,
        )
        .ok()?;
    if !local.is_finite() {
        return None;
    }

    let semantic_spacing = sample_scale.metres_per_native();
    let granularity = SpatialRealizationGranularityRequest::new(
        semantic_spacing,
        semantic_spacing,
        field.radius_metres().max(semantic_spacing),
        PATCH_GRID_RESOLUTION,
        PLANETARY_VALIDITY_AGGREGATES_ACROSS,
        view.velocity_metres_per_second().length(),
        expected_build_seconds,
        PLANETARY_MIN_VALIDITY_SECONDS,
        PLANETARY_MAX_VALIDITY_SECONDS,
        PLANETARY_LATENCY_MULTIPLIER,
    ).solve();

    let detail_exponent_f =
        granularity.target_spacing_metres().log2().ceil();
    if detail_exponent_f < f64::from(i16::MIN)
        || detail_exponent_f > f64::from(i16::MAX)
    {
        return None;
    }
    let detail_exponent = detail_exponent_f as i16;
    let detail_spacing = 2.0_f64.powi(i32::from(detail_exponent));

    let requested_validity_extent =
        (granularity.validity_radius_metres() * 2.0)
            .max(detail_spacing * f64::from(PATCH_GRID_RESOLUTION));
    let validity_exponent_f =
        requested_validity_extent.log2().ceil();
    if validity_exponent_f < f64::from(i16::MIN)
        || validity_exponent_f > f64::from(i16::MAX)
    {
        return None;
    }
    let validity_exponent = validity_exponent_f as i16;
    let validity_extent = 2.0_f64.powi(i32::from(validity_exponent));

    let bucket_component = |value: f64| -> Option<i64> {
        let value = (value / validity_extent).floor();
        if !value.is_finite()
            || value < i64::MIN as f64
            || value > i64::MAX as f64
        {
            None
        } else {
            Some(value as i64)
        }
    };
    let bucket = [
        bucket_component(local.x)?,
        bucket_component(local.y)?,
        bucket_component(local.z)?,
    ];
    let planning_anchor_local = DVec3::new(
        (bucket[0] as f64 + 0.5) * validity_extent,
        (bucket[1] as f64 + 0.5) * validity_extent,
        (bucket[2] as f64 + 0.5) * validity_extent,
    );
    let max_level =
        maximum_patch_level_for_spacing(field, detail_spacing);

    Some((
        PlanetaryObserverKey {
            bucket,
            validity_exponent,
            detail_exponent,
        },
        planning_anchor_local,
        max_level,
    ))
}


/// Select one non-overlapping regional frontier with an absolute leaf bound.
///
/// The stack itself is the unresolved frontier. Refining one leaf is allowed
/// only when replacing it with four children keeps
/// `selected + unresolved <= MAX_PATCH_LEAVES`. Otherwise the parent remains as
/// the valid coarse representation.
fn select_adaptive_patches(
    mut evaluate: impl FnMut(PlanetarySurfacePatchId) -> PatchDecision,
) -> Vec<PlanetarySurfacePatchId> {
    let mut unresolved = PlanetarySurfacePatchId::roots().collect::<Vec<_>>();
    unresolved.reverse();
    let mut selected = Vec::with_capacity(MAX_PATCH_LEAVES);

    while let Some(patch) = unresolved.pop() {
        match evaluate(patch) {
            PatchDecision::Cull => {}
            PatchDecision::Keep => selected.push(patch),
            PatchDecision::Refine
                if selected.len() + unresolved.len() + 4 <= MAX_PATCH_LEAVES =>
            {
                for child in patch.children().into_iter().rev() {
                    unresolved.push(child);
                }
            }
            PatchDecision::Refine => {
                // Budget exhaustion degrades representation error by retaining
                // the parent; it never spills unbounded work into the frame.
                selected.push(patch);
            }
        }
    }

    debug_assert!(selected.len() <= MAX_PATCH_LEAVES);
    selected
}

fn evaluate_patch(
    field: CelestialVoxelField,
    _sample_scale: SpatialScale,
    patch: PlanetarySurfacePatchId,
    observer_local: DVec3,
    max_level: u8,
    dense_coverage: &HashMap<Entity, PlanetaryDenseCoverageLocal>,
    dense_bounds: Option<PlanetaryDenseCoverageBounds>,
    clipmap_coverage: &[CelestialClipmapCoverageCell],
    policy: Option<&DeveloperScalarPolicyRuntime>,
) -> PatchDecision {
    let direction = patch.center_direction();
    let radius_metres = patch.approximate_radius_metres(field.radius_metres());
    let observer_distance = observer_local.length();

    // Conservative spherical-horizon rejection. Root/large patches carry a
    // deliberately broad angular margin and are only rejected once subdivision
    // makes their region unambiguously back-facing.
    if observer_distance > field.radius_metres() {
        let observer_direction = Vec3::new(
            (observer_local.x / observer_distance) as f32,
            (observer_local.y / observer_distance) as f32,
            (observer_local.z / observer_distance) as f32,
        )
        .normalize_or_zero();
        let horizon_cos =
            (field.radius_metres() / observer_distance).clamp(0.0, 1.0) as f32;
        let angular_margin =
            (radius_metres / field.radius_metres()).min(2.0) as f32;
        if direction.dot(observer_direction) + angular_margin < horizon_cos {
            return PatchDecision::Cull;
        }
    }

    // Local replacement is sparse and presentation-only. Dense capability
    // coverage and the committed binary clipmap are independent evidence
    // sources; either may refine/cull the regional approximation.
    if !dense_coverage.is_empty() || !clipmap_coverage.is_empty() {
        let Some(center_local_metres) =
            presentation_surface_local_metres(field, direction, policy)
        else {
            return PatchDecision::Keep;
        };

        let mut relation = DenseCoverageRelation::None;

        if !dense_coverage.is_empty()
            && dense_bounds.is_none_or(|bounds| {
                bounds.intersects_sphere(center_local_metres, radius_metres)
            })
        {
            relation = dense_coverage_relation(
                center_local_metres,
                radius_metres,
                dense_coverage,
            );
        }

        if relation != DenseCoverageRelation::Full
            && !clipmap_coverage.is_empty()
        {
            let clipmap_relation = clipmap_coverage_relation(
                center_local_metres,
                radius_metres,
                clipmap_coverage,
            );
            relation = match (relation, clipmap_relation) {
                (DenseCoverageRelation::Full, _)
                | (_, DenseCoverageRelation::Full) => DenseCoverageRelation::Full,
                (DenseCoverageRelation::Partial, _)
                | (_, DenseCoverageRelation::Partial) => DenseCoverageRelation::Partial,
                _ => DenseCoverageRelation::None,
            };
        }

        match relation {
            DenseCoverageRelation::Full => return PatchDecision::Cull,
            DenseCoverageRelation::Partial
                if patch.level < maximum_aperture_patch_level(max_level) =>
            {
                // Refine ownership farther than ordinary semantic-detail LOD
                // when necessary so fully replaced children can disappear.
                // This does not invent finer regional terrain information.
                return PatchDecision::Refine;
            }
            DenseCoverageRelation::Partial | DenseCoverageRelation::None => {}
        }
    }

    if patch.level >= max_level {
        return PatchDecision::Keep;
    }

    // Significance is body-local and cheap: no canonical terrain sampling is
    // needed to decide whether a patch deserves more representation detail.
    let direction64 = DVec3::new(
        f64::from(direction.x),
        f64::from(direction.y),
        f64::from(direction.z),
    );
    let approximate_surface = direction64 * field.radius_metres();
    let distance_metres = (observer_local - approximate_surface).length();
    let projected_error =
        radius_metres / distance_metres.max(radius_metres * 0.5);

    if projected_error > PROJECTED_ERROR_RATIO {
        PatchDecision::Refine
    } else {
        PatchDecision::Keep
    }
}

fn clipmap_coverage_relation(
    patch_center_local_metres: DVec3,
    patch_radius_metres: f64,
    clipmap_coverage: &[CelestialClipmapCoverageCell],
) -> DenseCoverageRelation {
    let mut partial = false;

    for realized in clipmap_coverage {
        let delta =
            patch_center_local_metres - realized.center_local_metres();
        let distance_squared = delta.length_squared();
        let inner = realized.inner_radius_metres();
        let outer = realized.outer_radius_metres();

        let full_radius = (inner - patch_radius_metres).max(0.0);
        if inner >= patch_radius_metres
            && distance_squared <= full_radius * full_radius
        {
            return DenseCoverageRelation::Full;
        }

        let partial_radius = outer + patch_radius_metres;
        if distance_squared <= partial_radius * partial_radius {
            partial = true;
        }
    }

    if partial {
        DenseCoverageRelation::Partial
    } else {
        DenseCoverageRelation::None
    }
}

fn dense_coverage_relation(
    patch_center_local_metres: DVec3,
    patch_radius_metres: f64,
    dense_coverage: &HashMap<Entity, PlanetaryDenseCoverageLocal>,
) -> DenseCoverageRelation {
    let mut partial = false;

    for realized in dense_coverage.values() {
        let delta = patch_center_local_metres - realized.center_local_metres;
        let distance_squared = delta.length_squared();

        // The inscribed sphere lies entirely inside the original coverage box.
        // Cull only when it proves the regional patch sphere is fully covered.
        let full_radius =
            (realized.inner_radius_metres - patch_radius_metres).max(0.0);
        if realized.inner_radius_metres >= patch_radius_metres
            && distance_squared <= full_radius * full_radius
        {
            return DenseCoverageRelation::Full;
        }

        // The circumscribed sphere contains the original coverage box. Treating
        // its overlap as partial may over-refine a regional patch, but can never
        // incorrectly remove coarse presentation.
        let partial_radius =
            realized.outer_radius_metres + patch_radius_metres;
        if distance_squared <= partial_radius * partial_radius {
            partial = true;
        }
    }

    if partial {
        DenseCoverageRelation::Partial
    } else {
        DenseCoverageRelation::None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn six_root_faces_cover_the_cardinal_directions() {
        let directions = PlanetarySurfacePatchId::roots()
            .map(PlanetarySurfacePatchId::center_direction)
            .collect::<Vec<_>>();

        for axis in [Vec3::X, -Vec3::X, Vec3::Y, -Vec3::Y, Vec3::Z, -Vec3::Z] {
            assert!(
                directions.iter().any(|direction| direction.dot(axis) > 0.999),
                "missing cubed-sphere root direction {axis:?}",
            );
        }
    }

    #[test]
    fn four_children_exactly_partition_parent_uv_domain() {
        let parent = PlanetarySurfacePatchId {
            face: PlanetarySurfaceFace::PositiveZ,
            level: 3,
            x: 4,
            y: 2,
        };
        let (pu0, pu1, pv0, pv1) = parent.uv_bounds();
        let children = parent.children();

        let area = |patch: PlanetarySurfacePatchId| {
            let (u0, u1, v0, v1) = patch.uv_bounds();
            (u1 - u0) * (v1 - v0)
        };
        let parent_area = (pu1 - pu0) * (pv1 - pv0);
        let child_area: f32 = children.into_iter().map(area).sum();

        assert!((parent_area - child_area).abs() < 1.0e-6);
        for child in parent.children() {
            let (u0, u1, v0, v1) = child.uv_bounds();
            assert!(u0 >= pu0 && u1 <= pu1 && v0 >= pv0 && v1 <= pv1);
        }
    }

    #[test]
    fn shared_cube_face_edge_generates_identical_directions() {
        let pos_x = PlanetarySurfacePatchId::root(PlanetarySurfaceFace::PositiveX);
        let pos_z = PlanetarySurfacePatchId::root(PlanetarySurfaceFace::PositiveZ);

        for step in 0..=16 {
            let t = step as f32 / 16.0;
            let x_edge = pos_x.face.cube_point(-1.0, -1.0 + 2.0 * t).normalize();
            let z_edge = pos_z.face.cube_point(1.0, -1.0 + 2.0 * t).normalize();
            assert!((x_edge - z_edge).length() < 1.0e-6);
        }
    }

    #[test]
    fn pathological_refine_everything_is_still_hard_bounded() {
        let selected = select_adaptive_patches(|_| PatchDecision::Refine);
        assert!(selected.len() <= MAX_PATCH_LEAVES);
        assert!(selected.len() >= MAX_PATCH_LEAVES.saturating_sub(3));
    }

    #[test]
    fn one_refinement_path_grows_linearly_not_planet_wide() {
        const DEPTH: u8 = 9;
        let selected = select_adaptive_patches(|patch| {
            if patch.face == PlanetarySurfaceFace::PositiveX
                && patch.x == 0
                && patch.y == 0
                && patch.level < DEPTH
            {
                PatchDecision::Refine
            } else {
                PatchDecision::Keep
            }
        });

        assert_eq!(selected.len(), 6 + 3 * DEPTH as usize);
        assert!(selected.len() < MAX_PATCH_LEAVES);
    }

    #[test]
    fn semantic_sample_spacing_caps_earth_s4_regional_depth() {
        let field = CelestialVoxelField::new(
            6_371_000.0,
            SpatialScale::new(6).unwrap(), SpatialScale::ZERO,
            0x4541_5254,
            crate::voxel::CelestialBodyProfile::Rocky,
        );
        assert_eq!(
            maximum_patch_level(field, SpatialScale::new(4).unwrap()),
            7,
        );
    }
}

#[cfg(test)]
mod aperture_cutout_tests {
    use super::*;

    #[test]
    fn partial_local_coverage_refines_beyond_semantic_detail_ceiling() {
        let field = CelestialVoxelField::new(
            6_371_000.0,
            SpatialScale::new(6).unwrap(),
            SpatialScale::ZERO,
            0x4541_5254,
            crate::voxel::CelestialBodyProfile::Rocky,
        );
        let sample_scale = SpatialScale::new(4).unwrap();
        let detail_max_level = maximum_patch_level_for_spacing(
            field,
            sample_scale.metres_per_native(),
        );
        assert!(
            detail_max_level < maximum_aperture_patch_level(detail_max_level),
            "Earth regional ownership must have cutout headroom beyond semantic detail",
        );

        let patch = PlanetarySurfacePatchId {
            face: PlanetarySurfaceFace::PositiveY,
            level: detail_max_level,
            x: 0,
            y: 0,
        };
        let direction = patch.center_direction();
        let center_local_metres =
            presentation_surface_local_metres(field, direction, None).unwrap();
        let patch_radius_metres =
            patch.approximate_radius_metres(field.radius_metres());

        let local_coverage = PlanetaryDenseCoverageLocal {
            geometry: PlanetaryDenseCoverageGeometry {
                realization: Entity::PLACEHOLDER,
                scale: SpatialScale::ZERO,
                center: UsfPosition::zero(SpatialScale::ZERO),
                half_extent_native: Vec3::ONE,
            },
            center_local_metres,
            inner_radius_metres: patch_radius_metres * 0.25,
            outer_radius_metres: patch_radius_metres * 0.25,
        };
        let dense_coverage = std::collections::HashMap::from([(
            Entity::PLACEHOLDER,
            local_coverage,
        )]);
        let dense_bounds =
            Some(PlanetaryDenseCoverageBounds::around(local_coverage));

        let radial = DVec3::new(
            f64::from(direction.x),
            f64::from(direction.y),
            f64::from(direction.z),
        );
        let observer_local = center_local_metres + radial * 100.0;

        assert_eq!(
            evaluate_patch(
                field,
                sample_scale,
                patch,
                observer_local,
                detail_max_level,
                &dense_coverage,
                dense_bounds,
                &[],
                None,
            ),
            PatchDecision::Refine,
        );
    }
}
