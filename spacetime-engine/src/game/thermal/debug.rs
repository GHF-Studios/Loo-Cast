//! Thermal state rendered as generic scalar samples.

use bevy::prelude::*;

use crate::{
    debug::{
        AppDebugExt, DebugCamera, DebugColorRamp, DebugGizmos, DebugOverlayGizmos,
        DebugScalarRange, DebugView, DebugViews, billboard_text,
    },
    ecs::UsfManifestationOf,
    physics::topology::{SpatialSplitPeer, SpatialSplitPeerActive},
};

use super::{Combustion, ThermalBody, ThermalSpatialSample};

struct ThermalScalarDebugView;

impl DebugView for ThermalScalarDebugView {
    const NAME: &'static str = "Thermal / Scalar Samples";
}

pub(crate) fn configure(app: &mut App) {
    app.register_debug_view::<ThermalScalarDebugView>()
        .add_systems(PostUpdate, draw_thermal_scalar_debug);
}

fn draw_thermal_scalar_debug(
    views: Res<DebugViews>,
    camera: Query<&Transform, With<DebugCamera>>,
    thermal_bodies: Query<&ThermalBody>,
    combustions: Query<(), With<Combustion>>,
    samples: Query<
        (&UsfManifestationOf, &Transform),
        (
            With<ThermalSpatialSample>,
            Or<(Without<SpatialSplitPeer>, With<SpatialSplitPeerActive>)>,
        ),
    >,
    mut gizmos: Gizmos<DebugGizmos>,
    mut overlay: Gizmos<DebugOverlayGizmos>,
) {
    if !views.enabled::<ThermalScalarDebugView>() {
        return;
    }
    let Some(camera) = camera.iter().next() else {
        return;
    };

    // Deliberately a visualization range, not a simulation threshold. The same
    // scalar/ramp machinery can later be fed by sampled metric maps or fields.
    let range = DebugScalarRange::new(273.15, 800.0);

    for (relation, transform) in &samples {
        let Ok(body) = thermal_bodies.get(relation.0) else {
            continue;
        };

        let temperature = body.temperature_kelvin();
        let color = DebugColorRamp::THERMAL.sample_scalar(range, temperature);
        let position = transform.translation;

        gizmos.cross(Isometry3d::new(position, Quat::IDENTITY), 0.35, color);
        gizmos
            .sphere(Isometry3d::new(position, Quat::IDENTITY), 0.28, color)
            .resolution(12);

        let burning = combustions.contains(relation.0);
        let label = format!(
            "{:.1} K{}",
            temperature,
            if burning { " [burning]" } else { "" },
        );
        billboard_text(
            &mut overlay,
            camera,
            position + Vec3::Y * 0.65,
            &label,
            13.0,
            Vec2::new(-0.5, -0.5),
            color,
        );
    }
}
