//! Thermal state and heat-transfer influence rendered through the generic debug substrate.

use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    debug::{
        AppDebugExt, DebugCamera, DebugColorRamp, DebugGizmos, DebugOverlayGizmos,
        DebugScalarField2d, DebugScalarRange, DebugView, DebugViews, billboard_text,
    },
    ecs::UsfManifestationOf,
    physics::topology::{SpatialSplitPeer, SpatialSplitPeerActive},
};

use super::{CombustibleMaterial, Combustion, ThermalBody, ThermalSpatialSample};

struct ThermalScalarDebugView;

impl DebugView for ThermalScalarDebugView {
    const NAME: &'static str = "Thermal / Scalar Samples";
    const DESCRIPTION: &'static str = "Show each spatial thermal sample's lumped temperature.";
}

struct ThermalInfluenceFieldDebugView;

impl DebugView for ThermalInfluenceFieldDebugView {
    const NAME: &'static str = "Thermal / Heat Influence Field";
    const DESCRIPTION: &'static str =
        "Sample combustion heat-transfer influence on a plane around the debug camera.";
    const ENABLED_BY_DEFAULT: bool = false;
}

#[derive(Component)]
struct ThermalInfluenceFieldEntity;

pub(crate) fn configure(app: &mut App) {
    app.register_debug_view::<ThermalScalarDebugView>()
        .register_debug_view::<ThermalInfluenceFieldDebugView>()
        .add_systems(
            PostUpdate,
            (draw_thermal_scalar_debug, update_thermal_influence_field),
        );
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

#[derive(Debug)]
struct HeatInfluenceSource {
    positions: Vec<Vec3>,
    environmental_power_watts: f32,
    radius_meters: f32,
}

fn update_thermal_influence_field(
    mut commands: Commands,
    views: Res<DebugViews>,
    camera: Query<
        &Transform,
        (With<DebugCamera>, Without<ThermalInfluenceFieldEntity>),
    >,
    samples: Query<
        (&UsfManifestationOf, &Transform),
        (
            With<ThermalSpatialSample>,
            Without<ThermalInfluenceFieldEntity>,
            Or<(Without<SpatialSplitPeer>, With<SpatialSplitPeerActive>)>,
        ),
    >,
    sources: Query<(Entity, &Combustion, &CombustibleMaterial)>,
    mut existing: Query<
        (Entity, &mut DebugScalarField2d, &mut Transform),
        With<ThermalInfluenceFieldEntity>,
    >,
) {
    if !views.enabled::<ThermalInfluenceFieldDebugView>() {
        for (entity, _, _) in &mut existing {
            commands.entity(entity).despawn();
        }
        return;
    }

    let Some(camera) = camera.iter().next() else {
        return;
    };

    let mut positions_by_semantic = HashMap::<Entity, Vec<Vec3>>::new();
    for (relation, transform) in &samples {
        positions_by_semantic
            .entry(relation.0)
            .or_default()
            .push(transform.translation);
    }

    let influence_sources = sources
        .iter()
        .filter_map(|(entity, combustion, material)| {
            let positions = positions_by_semantic.get(&entity)?.clone();
            let environmental_power_watts = combustion.power_watts()
                * material.environmental_transfer_fraction.clamp(0.0, 1.0);
            let radius_meters = material.heat_transfer_radius_meters.max(0.0);
            (!positions.is_empty()
                && environmental_power_watts > 0.0
                && radius_meters > 0.0)
                .then_some(HeatInfluenceSource {
                    positions,
                    environmental_power_watts,
                    radius_meters,
                })
        })
        .collect::<Vec<_>>();

    if influence_sources.is_empty() {
        for (entity, _, _) in &mut existing {
            commands.entity(entity).despawn();
        }
        return;
    }

    const SIZE: f32 = 20.0;
    const RESOLUTION: UVec2 = UVec2::new(28, 28);
    let size = Vec2::splat(SIZE);
    let cell = size / RESOLUTION.as_vec2();
    let minimum = -size * 0.5;
    let transform = Transform {
        translation: Vec3::new(camera.translation.x, 0.035, camera.translation.z),
        rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
        ..default()
    };

    let mut values = Vec::with_capacity((RESOLUTION.x * RESOLUTION.y) as usize);
    let mut maximum = 0.0_f32;

    for y in 0..RESOLUTION.y {
        for x in 0..RESOLUTION.x {
            let local = Vec3::new(
                minimum.x + (x as f32 + 0.5) * cell.x,
                minimum.y + (y as f32 + 0.5) * cell.y,
                0.0,
            );
            let point = transform.translation + transform.rotation * local;
            let mut influence = 0.0;

            for source in &influence_sources {
                // The simulation treats all manifestations of one thermal source
                // as alternate spatial access points and uses the nearest one,
                // rather than multiplying source power by manifestation count.
                let distance = source
                    .positions
                    .iter()
                    .map(|position| position.distance(point))
                    .fold(f32::INFINITY, f32::min);
                if distance >= source.radius_meters {
                    continue;
                }

                let normalized = 1.0 - distance / source.radius_meters;
                influence += source.environmental_power_watts * normalized * normalized;
            }

            maximum = maximum.max(influence);
            values.push(influence);
        }
    }

    let field = DebugScalarField2d::new(
        size,
        RESOLUTION,
        values,
        DebugScalarRange::new(0.0, maximum.max(1.0)),
        DebugColorRamp::THERMAL,
    )
    .with_opacity(0.58);

    if let Some((_, mut current_field, mut current_transform)) = existing.iter_mut().next() {
        *current_field = field;
        *current_transform = transform;
    } else {
        commands.spawn((
            Name::new("Debug Thermal Heat Influence Field"),
            ThermalInfluenceFieldEntity,
            field,
            transform,
        ));
    }
}
