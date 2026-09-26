//! Read-only physical-surface proximity telemetry.
//!
//! This domain does not choose navigation targets, gravity frames or collision
//! contacts. It independently observes the nearest authored celestial surface
//! to a runtime subject for HUD/safety telemetry.
//!
//! Actual contact authority remains the collision system:
//! character grounding and spacecraft landing use collision queries directly.

use bevy::prelude::*;

use crate::{
    physics::PhysicalBoxHull,
    spatial::{
        UsfScaleCoverageSnapshot, UsfScaleLayer, UsfScaleRoleMask, UsfSpatialFrame,
        UsfSpatialSet,
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
    field: CelestialVoxelField,
    domain: VoxelScaleDomain,
) -> Option<SurfaceCandidate> {
    let measurement_scale = field.coarsest_detail_scale().max(position.leaf_scale());
    let relative = position
        .relative_at_scale_bounded(&field.center(), measurement_scale, f32::MAX)
        .ok()?;
    let radial_outward = relative.normalize_or_zero();
    if radial_outward == Vec3::ZERO {
        return None;
    }

    let center_distance_metres =
        f64::from(relative.length()) * measurement_scale.metres_per_native();

    let (center_altitude_metres, reference) = if domain.realizes(subject_scale) {
        // Surface telemetry consumes the same canonical procedural surface as
        // voxel demand/bootstrap. The body radius remains semantic; only the
        // bounded displacement from the resolved surface enters this chart.
        let surface = field
            .surface_position(radial_outward, subject_scale)
            .ok()?;
        let relative_to_surface = position
            .relative_at_scale_bounded_f64(&surface, subject_scale, f64::MAX)
            .ok()?;
        let signed_native = relative_to_surface.x * f64::from(radial_outward.x)
            + relative_to_surface.y * f64::from(radial_outward.y)
            + relative_to_surface.z * f64::from(radial_outward.z);

        (
            signed_native * subject_scale.metres_per_native(),
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
    frame: Res<UsfSpatialFrame>,
    coverage: Res<UsfScaleCoverageSnapshot>,
    fields: Query<(Entity, &CelestialVoxelField, &VoxelScaleDomain)>,
    mut subjects: Query<(
        &Transform,
        &UsfScaleLayer,
        &PhysicalBoxHull,
        &mut SurfaceContext,
    )>,
) {
    for (transform, layer, hull, mut surface) in &mut subjects {
        *surface = SurfaceContext::default();

        let Ok(position) = frame
            .origin()
            .translated_at_scale(layer.scale(), transform.translation)
        else {
            continue;
        };

        let nearest = fields
            .iter()
            .filter_map(|(entity, field, domain)| {
                sample_surface_candidate(
                    &position,
                    layer.scale(),
                    entity,
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
