//! Observational viewport visualization for focused thermal state.

use super::*;

/// Thermal's viewport gizmo is observational: it visualizes the focused thermal
/// samples and, when combustible, the authored heat-transfer radius. Mutation is
/// exposed separately through validated fields/actions in the contextual UI.
pub(super) fn collect_focused_thermal_gizmo(
    presentation: Res<PrimaryViewPresentation>,
    focus: Res<DeveloperFocus>,
    structure: Res<StructureSelection>,
    thermal_bodies: Query<&ThermalBody>,
    combustible: Query<&CombustibleMaterial>,
    semantic_entities: Query<&UsfManifestations>,
    thermal_samples: Query<&GlobalTransform, With<ThermalSpatialSample>>,
    transforms: Query<&GlobalTransform>,
    frame: Res<WorldDrawFrame>,
) {
    if !presentation.is_embedded() {
        return;
    }
    let Some(target) = focus.current() else {
        return;
    };
    if structure.item_for(target) != Some(THERMAL_STRUCTURE) {
        return;
    }
    let Ok(body) = thermal_bodies.get(target.semantic_entity) else {
        return;
    };

    let radius = combustible
        .get(target.semantic_entity)
        .ok()
        .map(|material| material.heat_transfer_radius_meters)
        .filter(|radius| *radius > 0.0);
    let color = thermal_color(body.temperature_kelvin());
    let mut batch = WorldDrawBatch::default();
    let mut drew_sample = false;

    if let Ok(manifestations) = semantic_entities.get(target.semantic_entity) {
        for manifestation in manifestations.iter() {
            let Ok(transform) = thermal_samples.get(manifestation) else {
                continue;
            };
            draw_thermal_sample(&mut batch, transform.translation(), radius, color);
            drew_sample = true;
        }
    }

    if !drew_sample && let Ok(transform) = transforms.get(target.spatial_entity) {
        draw_thermal_sample(&mut batch, transform.translation(), radius, color);
    }

    if !batch.primitives.is_empty() {
        frame.submit(batch);
    }
}

fn draw_thermal_sample(
    batch: &mut WorldDrawBatch,
    position: Vec3,
    radius: Option<f32>,
    color: Color,
) {
    batch.cross(
        Isometry3d::new(position, Quat::IDENTITY),
        0.18,
        color,
        DrawDepth::Overlay,
    );
    if let Some(radius) = radius {
        batch.sphere(
            Isometry3d::new(position, Quat::IDENTITY),
            radius,
            Color::srgba(1.0, 0.35, 0.08, 0.72),
            32,
            DrawDepth::Overlay,
        );
    }
}

fn thermal_color(temperature_kelvin: f32) -> Color {
    let normalized = ((temperature_kelvin - AMBIENT_TEMPERATURE_KELVIN) / 500.0).clamp(-1.0, 1.0);
    if normalized >= 0.0 {
        Color::srgb(1.0, 0.85 - normalized * 0.65, 0.2)
    } else {
        let cold = -normalized;
        Color::srgb(0.2, 0.75 - cold * 0.35, 1.0)
    }
}
