//! Read-only physical-surface proximity telemetry.
//!
//! This domain does not choose navigation targets, gravity frames or collision
//! contacts. It independently observes the nearest authored celestial surface
//! to a runtime subject for HUD/safety telemetry.
//!
//! Actual contact authority remains the collision system:
//! character grounding and spacecraft landing use collision queries directly.

use bevy::{math::DVec3, prelude::*};

use crate::{
    ecs::UsfOwnershipQuery,
    physics::PhysicalBoxHull,
    spatial::{
        UsfPosition, UsfScaleCoverageSnapshot, UsfScaleLayer, UsfScaleRoleMask,
        UsfSemanticFrame, UsfSpatialFrame, UsfSpatialSet,
    },
    voxel::{CelestialVoxelField, VoxelScaleDomain},
};

#[derive(Reflect, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum SurfaceReference {
    #[default]
    None,
    Procedural,
    Nominal,
}

/// Nearest currently observable authored surface.
///
/// `radial_outward` is body-center radial direction only. It is deliberately
/// not called `up`: gravity-up, walkable contact normal and procedural surface
/// normal are independent concepts.
///
/// `clearance_metres` subtracts this subject's oriented detailed physical-hull
/// support radius from center altitude. It remains telemetry; a collision query
/// decides whether physical contact actually exists.
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct SurfaceContext {
    body: Option<Entity>,
    reference: SurfaceReference,
    radial_outward: Vec3,
    center_altitude_metres: f64,
    clearance_metres: f64,
    collision_ready: bool,
}

impl Default for SurfaceContext {
    fn default() -> Self {
        Self {
            body: None,
            reference: SurfaceReference::None,
            radial_outward: Vec3::ZERO,
            center_altitude_metres: f64::INFINITY,
            clearance_metres: f64::INFINITY,
            collision_ready: false,
        }
    }
}

impl SurfaceContext {
    pub const fn body(self) -> Option<Entity> {
        self.body
    }

    pub const fn reference(self) -> SurfaceReference {
        self.reference
    }

    pub const fn radial_outward(self) -> Vec3 {
        self.radial_outward
    }

    pub fn center_altitude_metres(self) -> Option<f64> {
        self.body.map(|_| self.center_altitude_metres)
    }

    pub fn clearance_metres(self) -> Option<f64> {
        self.body.map(|_| self.clearance_metres)
    }

    pub const fn collision_ready(self) -> bool {
        self.collision_ready
    }
}

#[derive(Debug, Clone, Copy)]
struct SurfaceCandidate {
    body: Entity,
    reference: SurfaceReference,
    radial_outward: Vec3,
    center_altitude_metres: f64,
}

fn sample_surface_candidate(
    position: &crate::spatial::UsfPosition,
    subject_scale: crate::spatial::SpatialScale,
    body: Entity,
    body_origin: UsfPosition,
    body_frame: UsfSemanticFrame,
    field: CelestialVoxelField,
    domain: VoxelScaleDomain,
) -> Option<SurfaceCandidate> {
    // Direction/distance must never be normalized in an arbitrarily coarse
    // runtime f32 chart. At S+35 an Earth-radius displacement is ~6e-29 native
    // units; squaring that underflows f32 length to zero and falsely makes the
    // body disappear from telemetry.
    //
    // Measure in the body's own coarse semantic chart using f64 instead. This
    // remains comfortably bounded for the body while preserving enough dynamic
    // range to normalize robustly from any observer interaction Scale.
    let measurement_scale = field.coarsest_detail_scale();
    let relative = position
        .relative_at_scale_bounded_f64(
            &body_origin,
            measurement_scale,
            f64::MAX,
        )
        .ok()?;
    let radial = relative.length();
    if !radial.is_finite() || radial <= f64::EPSILON {
        return None;
    }

    let radial_outward = Vec3::new(
        (relative.x / radial) as f32,
        (relative.y / radial) as f32,
        (relative.z / radial) as f32,
    )
    .normalize_or_zero();
    if radial_outward == Vec3::ZERO {
        return None;
    }

    let center_distance_metres =
        radial * measurement_scale.metres_per_native();

    let (center_altitude_metres, reference) = if domain.realizes(subject_scale) {
        // Surface telemetry consumes the same canonical procedural surface as
        // voxel demand/bootstrap. The body radius remains semantic; only the
        // bounded displacement from the resolved surface enters this chart.
        let local_outward = body_frame.world_direction_to_local(radial_outward);
        let surface = field
            .surface_position(&body_origin, body_frame, local_outward)
            .ok()?;
        let relative_to_surface = position
            .relative_at_scale_bounded_f64(
                &surface,
                measurement_scale,
                f64::MAX,
            )
            .ok()?;

        let world_outward = DVec3::new(
            f64::from(radial_outward.x),
            f64::from(radial_outward.y),
            f64::from(radial_outward.z),
        );
        let signed_native = relative_to_surface.dot(world_outward);

        (
            signed_native * measurement_scale.metres_per_native(),
            SurfaceReference::Procedural,
        )
    } else {
        (
            center_distance_metres - field.radius_metres(),
            SurfaceReference::Nominal,
        )
    };

    Some(SurfaceCandidate {
        body,
        reference,
        radial_outward,
        center_altitude_metres,
    })
}

fn sync_surface_contexts(
    coverage: Res<UsfScaleCoverageSnapshot>,
    ownership: UsfOwnershipQuery,
    semantic_positions: Query<&UsfPosition>,
    fields: Query<(
        Entity,
        &UsfPosition,
        &UsfSemanticFrame,
        &CelestialVoxelField,
        &VoxelScaleDomain,
    )>,
    mut subjects: Query<(
        Entity,
        &Transform,
        &UsfScaleLayer,
        &PhysicalBoxHull,
        &mut SurfaceContext,
    )>,
) {
    for (entity, transform, layer, hull, mut surface) in &mut subjects {
        *surface = SurfaceContext::default();

        let _candidate_span =
            bevy::log::info_span!("surface_context.candidate").entered();

        // Surface telemetry observes semantic position directly. Runtime
        // Transform is a bounded/rebased projection and may intentionally
        // remain near the chart origin while the subject travels globally.
        let Some(semantic_entity) = ownership.semantic_of(entity) else {
            continue;
        };
        let Ok(position) = semantic_positions.get(semantic_entity) else {
            continue;
        };

        let nearest = fields
            .iter()
            .filter_map(|(entity, body_origin, body_frame, field, domain)| {
                sample_surface_candidate(
                    &position,
                    layer.scale(),
                    entity,
                    *body_origin,
                    *body_frame,
                    *field,
                    *domain,
                )
            })
            .min_by(|a, b| {
                a.center_altitude_metres
                    .abs()
                    .total_cmp(&b.center_altitude_metres.abs())
            });

        let Some(candidate) = nearest else {
            continue;
        };

        let support_metres = f64::from(
            hull.projection_radius_metres(transform.rotation, candidate.radial_outward),
        );
        let clearance_metres = candidate.center_altitude_metres - support_metres;

        let collision_probe_metres = (support_metres + 0.5).max(0.5);
        let collision_probe_native =
            layer.scale().metres_to_native_f32(collision_probe_metres as f32);

        drop(_candidate_span);
        let _coverage_span =
            bevy::log::info_span!("surface_context.coverage").entered();
        let collision_ready = coverage.has_near_for_authority(
            candidate.body,
            layer.scale(),
            &position,
            UsfScaleRoleMask::COLLISION,
            collision_probe_native,
        );

        *surface = SurfaceContext {
            body: Some(candidate.body),
            reference: candidate.reference,
            radial_outward: candidate.radial_outward,
            center_altitude_metres: candidate.center_altitude_metres,
            clearance_metres,
            collision_ready,
        };
    }
}

pub struct SurfacePlugin;

impl Plugin for SurfacePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SurfaceReference>()
            .register_type::<SurfaceContext>()
            .add_systems(
                PostUpdate,
                sync_surface_contexts
                    .after(UsfSpatialSet::SyncSemantic)
                    .before(UsfSpatialSet::Rebase),
            );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn physical_clearance_subtracts_canonical_hull_support() {
        let hull = PhysicalBoxHull::from_size_metres(Vec3::new(2.0, 4.0, 2.0));
        let support = hull.projection_radius_metres(Quat::IDENTITY, Vec3::Y);
        assert!((support - 2.0).abs() < 1.0e-6);
        assert!((3.0_f64 - f64::from(support) - 1.0).abs() < 1.0e-9);
    }
}
