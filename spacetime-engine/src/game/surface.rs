//! Gameplay-facing physical surface context.
//!
//! Navigation supplies coarse semantic body geometry for regime selection.
//! Player-facing altitude/contact instead references the actual procedural
//! surface definition; physical fields are queried independently.

use bevy::prelude::*;

use crate::{
    game::{
        locomotion::DetailedInteractionScale,
        navigation::PrimaryBodyContext,
    },
    physics::topology::SpatialSplitBox,
    spatial::{
        UsfScaleCoverageSnapshot, UsfScaleLayer, UsfScaleRoleMask, UsfSpatialFrame,
        UsfSpatialSet,
    },
    voxel::CelestialVoxelField,
};

#[derive(Reflect, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum SurfaceReference {
    #[default]
    None,
    Procedural,
    Nominal,
}

/// Stable gameplay-facing physical-surface snapshot.
///
/// `center_altitude_metres` measures the subject origin above terrain.
/// `clearance_metres` subtracts the subject's oriented physical support radius,
/// so a standing/landed hull approaches zero clearance at contact.
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct SurfaceContext {
    body: Option<Entity>,
    reference: SurfaceReference,
    up: Vec3,
    center_altitude_metres: f64,
    clearance_metres: f64,
    collision_ready: bool,
}

impl Default for SurfaceContext {
    fn default() -> Self {
        Self {
            body: None,
            reference: SurfaceReference::None,
            up: Vec3::Y,
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

    pub const fn up(self) -> Vec3 {
        self.up
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

fn sync_surface_contexts(
    frame: Res<UsfSpatialFrame>,
    coverage: Res<UsfScaleCoverageSnapshot>,
    fields: Query<&CelestialVoxelField>,
    mut subjects: Query<(
        &Transform,
        &UsfScaleLayer,
        &DetailedInteractionScale,
        &PrimaryBodyContext,
        &SpatialSplitBox,
        &mut SurfaceContext,
    )>,
) {
    for (transform, layer, detailed, primary, hull, mut surface) in &mut subjects {
        *surface = SurfaceContext::default();

        let Some(body) = primary.entity() else {
            continue;
        };

        let Ok(position) = frame
            .origin()
            .translated_at_scale(layer.scale(), transform.translation)
        else {
            continue;
        };

        let reference_scale = primary.reference_scale();
        let bound_metres = (primary.center_distance_metres().max(primary.radius_metres())
            + primary.radius_metres())
            .max(1.0);
        let bound_native = reference_scale
            .scale0_to_native_f64(bound_metres)
            .min(f64::from(f32::MAX)) as f32;

        let Ok(relative) = position.relative_at_scale_bounded(
            &primary.center(),
            reference_scale,
            bound_native,
        ) else {
            continue;
        };

        let up = relative.normalize_or_zero();
        if up == Vec3::ZERO {
            continue;
        }

        let center_distance_metres =
            f64::from(relative.length()) * reference_scale.scale0_units_per_native();

        let (surface_radius_metres, reference) = if let Ok(field) = fields.get(body) {
            let realization = field.realization(detailed.0);
            (
                f64::from(realization.surface_radius_native(up))
                    * detailed.0.scale0_units_per_native(),
                SurfaceReference::Procedural,
            )
        } else {
            (primary.radius_metres(), SurfaceReference::Nominal)
        };

        let center_altitude_metres = center_distance_metres - surface_radius_metres;
        let support_metres = f64::from(hull.projection_radius(transform.rotation, up));
        let clearance_metres = center_altitude_metres - support_metres;

        let collision_probe_metres = (support_metres + 0.5).max(0.5);
        let collision_probe_native =
            layer.scale().metres_to_native_f32(collision_probe_metres as f32);

        let collision_ready = coverage.has_near_for_authority(
            body,
            layer.scale(),
            &position,
            UsfScaleRoleMask::COLLISION,
            collision_probe_native,
        );

        *surface = SurfaceContext {
            body: Some(body),
            reference,
            up,
            center_altitude_metres,
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
    fn physical_clearance_subtracts_body_support_from_center_altitude() {
        let hull = SpatialSplitBox::from_size(Vec3::new(2.0, 4.0, 2.0));
        let support = hull.projection_radius(Quat::IDENTITY, Vec3::Y);
        assert!((support - 2.0).abs() < 1.0e-6);
        assert!((3.0_f64 - f64::from(support) - 1.0).abs() < 1.0e-9);
    }
}
