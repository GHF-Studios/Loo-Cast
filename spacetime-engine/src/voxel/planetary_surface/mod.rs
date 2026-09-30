//! Adaptive regional whole-body presentation for semantic celestial fields.
//!
//! Dense voxel materializations are intentionally not used to draw an entire
//! planet. This representation selects bounded cubed-sphere patches from
//! observer significance and derives every vertex from `CelestialVoxelField`.
//! Patches are presentation-only projections of semantic authority.

use std::collections::{HashMap, HashSet};

use bevy::{math::DVec3, prelude::*};

use crate::{
    ecs::UsfPresentationProjectionOf,
    spatial::{
        SpatialScale, UsfScaleCoverageSnapshot, UsfScaleRoleMask,
        UsfSceneryPresentation, UsfSemanticFrame, UsfSpatialSet,
        UsfViewDemand, UsfViewDemandSnapshot, UsfPosition,
    },
};

use super::{
    CelestialVoxelField, CelestialVoxelRealizationPolicy,
};

mod mesh;
use mesh::build_planetary_surface_patch;

const PLANETARY_SAMPLE_SCALE: i8 = 4;
const MAX_PATCH_LEVEL: u8 = 12;
const PROJECTED_ERROR_RATIO: f64 = 0.20;
const PATCH_BOUND_MARGIN: f64 = 1.30;

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
/// Patch hierarchy is representation refinement only. `level` is deliberately
/// not a USF Scale.
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct PatchKey {
    authority: Entity,
    patch: PlanetarySurfacePatchId,
}

pub(super) fn sync_planetary_surface_realizations(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    views: Res<UsfViewDemandSnapshot>,
    coverage: Res<UsfScaleCoverageSnapshot>,
    authorities: Query<(
        Entity,
        Option<&Name>,
        &UsfPosition,
        &UsfSemanticFrame,
        &CelestialVoxelField,
        &CelestialVoxelRealizationPolicy,
    )>,
    existing: Query<(Entity, &PlanetarySurfaceRealization)>,
) {
    let Some(view) = views.iter().next() else {
        // View capture is downstream presentation state. A transient missing
        // snapshot must not tear down the last valid whole-body context.
        return;
    };
    let mut existing_by_key = HashMap::<PatchKey, Entity>::new();
    for (entity, realization) in &existing {
        let key = PatchKey {
            authority: realization.authority(),
            patch: realization.patch(),
        };
        if existing_by_key.insert(key, entity).is_some() {
            commands.entity(entity).despawn();
        }
    }

    let mut desired = HashSet::<PatchKey>::new();
    let mut replacements_ready = true;

    for (authority, name, body_origin, body_frame, field, policy) in &authorities {
        let sample_scale = planetary_sample_scale(*field);
        let selected = select_adaptive_patches(|patch| {
            evaluate_patch(
                authority,
                *body_origin,
                *body_frame,
                *field,
                sample_scale,
                patch,
                view,
                &coverage,
            )
        });

        for patch in selected {
            let key = PatchKey { authority, patch };
            desired.insert(key);

            if existing_by_key.contains_key(&key) {
                continue;
            }

            let Some(mesh) = build_planetary_surface_patch(
                *field,
                *body_origin,
                *body_frame,
                patch,
                sample_scale,
            ) else {
                replacements_ready = false;
                continue;
            };

            let body_name = name.map(|value| value.as_str()).unwrap_or("Celestial Body");
            let orientation = body_frame.orientation();
            let rotation = Quat::from_xyzw(
                orientation.x as f32,
                orientation.y as f32,
                orientation.z as f32,
                orientation.w as f32,
            )
            .normalize();

            commands.spawn((
                Name::new(format!(
                    "{body_name} Planetary Patch {:?} L{} ({},{})",
                    patch.face, patch.level, patch.x, patch.y,
                )),
                PlanetarySurfaceRealization {
                    authority,
                    patch,
                    scale: sample_scale,
                    revision: 1,
                },
                UsfPresentationProjectionOf(authority),
                UsfSceneryPresentation::from_anchor(*body_origin, sample_scale),
                Mesh3d(meshes.add(mesh)),
                MeshMaterial3d(policy.presentation_material().clone()),
                Transform::from_rotation(rotation),
                Visibility::Inherited,
            ));
        }
    }

    // Missing children are built synchronously above. Retire stale parents only
    // after every desired replacement has been queued, preserving make-before-
    // break at the ECS publication boundary.
    if replacements_ready {
        for (key, entity) in existing_by_key {
            if !desired.contains(&key) {
                commands.entity(entity).despawn();
            }
        }
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

fn select_adaptive_patches(
    mut evaluate: impl FnMut(PlanetarySurfacePatchId) -> PatchDecision,
) -> Vec<PlanetarySurfacePatchId> {
    fn visit(
        patch: PlanetarySurfacePatchId,
        evaluate: &mut impl FnMut(PlanetarySurfacePatchId) -> PatchDecision,
        selected: &mut Vec<PlanetarySurfacePatchId>,
    ) {
        match evaluate(patch) {
            PatchDecision::Cull => {}
            PatchDecision::Keep => selected.push(patch),
            PatchDecision::Refine => {
                for child in patch.children() {
                    visit(child, evaluate, selected);
                }
            }
        }
    }

    let mut selected = Vec::new();
    for root in PlanetarySurfacePatchId::roots() {
        visit(root, &mut evaluate, &mut selected);
    }
    selected
}

fn evaluate_patch(
    authority: Entity,
    body_origin: UsfPosition,
    body_frame: UsfSemanticFrame,
    field: CelestialVoxelField,
    sample_scale: SpatialScale,
    patch: PlanetarySurfacePatchId,
    view: &UsfViewDemand,
    coverage: &UsfScaleCoverageSnapshot,
) -> PatchDecision {
    let direction = patch.center_direction();
    let radius_metres = patch.approximate_radius_metres(field.radius_metres());

    // Conservative spherical-horizon rejection prevents a near-surface view
    // from recursively refining the entire back side of the planet. The patch
    // angular bound keeps edge/horizon patches until subdivision resolves them.
    if let Ok(observer_local) = body_frame.world_to_local_metres(
        &body_origin,
        &view.anchor(),
        sample_scale,
        f64::MAX,
    ) {
        let observer_distance = observer_local.length();
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
    }

    let Ok(center) =
        field.surface_position(&body_origin, body_frame, direction, sample_scale)
    else {
        return PatchDecision::Cull;
    };

    let radius_native = sample_scale.metres_to_native_f64(radius_metres);
    if !radius_native.is_finite()
        || radius_native <= 0.0
        || radius_native > f64::from(f32::MAX)
    {
        return PatchDecision::Cull;
    }

    if !view.intersects_presentation_native_aabb(
        sample_scale,
        &center,
        Vec3::splat(radius_native as f32),
    ) {
        return PatchDecision::Cull;
    }

    match dense_coverage_relation(authority, center, radius_metres, coverage) {
        DenseCoverageRelation::Full => return PatchDecision::Cull,
        DenseCoverageRelation::Partial if patch.level < MAX_PATCH_LEVEL => {
            return PatchDecision::Refine;
        }
        DenseCoverageRelation::Partial | DenseCoverageRelation::None => {}
    }

    if patch.level >= MAX_PATCH_LEVEL {
        return PatchDecision::Keep;
    }

    let Ok(observer_delta) = center.relative_at_scale_bounded_f64(
        &view.anchor(),
        sample_scale,
        f64::MAX,
    ) else {
        return PatchDecision::Keep;
    };
    let distance_metres =
        observer_delta.length() * sample_scale.metres_per_native();
    let projected_error =
        radius_metres / distance_metres.max(radius_metres * 0.25);

    if projected_error > PROJECTED_ERROR_RATIO {
        PatchDecision::Refine
    } else {
        PatchDecision::Keep
    }
}

fn dense_coverage_relation(
    authority: Entity,
    patch_center: UsfPosition,
    patch_radius_metres: f64,
    coverage: &UsfScaleCoverageSnapshot,
) -> DenseCoverageRelation {
    let mut partial = false;

    for realized in coverage.iter().filter(|realized| {
        realized.authority() == authority
            && realized.roles().contains(UsfScaleRoleMask::PRESENTATION)
    }) {
        let radius_native =
            realized.scale().metres_to_native_f64(patch_radius_metres);
        if !radius_native.is_finite() || radius_native < 0.0 {
            continue;
        }

        let half = realized.half_extent_native();
        let half = DVec3::new(
            f64::from(half.x),
            f64::from(half.y),
            f64::from(half.z),
        );
        let bound = half.length() + radius_native + 1.0;
        let Ok(relative) = patch_center.relative_at_scale_bounded_f64(
            &realized.center(),
            realized.scale(),
            bound,
        ) else {
            continue;
        };

        let abs = relative.abs();
        if abs.x + radius_native <= half.x
            && abs.y + radius_native <= half.y
            && abs.z + radius_native <= half.z
        {
            return DenseCoverageRelation::Full;
        }

        let nearest = DVec3::new(
            relative.x.clamp(-half.x, half.x),
            relative.y.clamp(-half.y, half.y),
            relative.z.clamp(-half.z, half.z),
        );
        if (relative - nearest).length_squared()
            <= radius_native * radius_native
        {
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
    fn distant_whole_body_can_remain_six_bounded_root_patches() {
        let selected = select_adaptive_patches(|_| PatchDecision::Keep);
        assert_eq!(selected.len(), 6);
    }

    #[test]
    fn refining_one_aperture_does_not_refine_the_entire_planet() {
        const DEPTH: u8 = 10;
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
        assert!(selected.len() < 64);
    }
}
