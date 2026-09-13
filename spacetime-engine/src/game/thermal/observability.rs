//! Thermal developer visualizations.

use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    devtools::{
        AppDeveloperToolsExt, ColorRamp, DeveloperSet, DeveloperTools, DeveloperView, DrawDepth,
        DrawId, ScalarFieldMode, ScalarRange, VisualizationId, VisualizationSpec, WorldDrawBatch,
        WorldDrawFrame, WorldScalarField,
    },
    ecs::UsfManifestationOf,
    physics::topology::{SpatialSplitPeer, SpatialSplitPeerActive},
};

use super::{
    coupling::{combustion_heat_coupling, radial_heat_weight}, CombustibleMaterial, Combustion,
    ThermalBody, ThermalField, ThermalMaterial, ThermalSpatialSample,
};

const CELLS_VISUALIZATION: VisualizationId = VisualizationId("world.thermal.cells");
const COUPLING_FIELD_VISUALIZATION: VisualizationId =
    VisualizationId("world.thermal.coupling_field");

const FIELD_OBSERVATION: DrawId = DrawId("observation.thermal.heat_coupling");
const FIELD_SIZE_METERS: f32 = 20.0;
const FIELD_RESOLUTION: u32 = 28;
const FIELD_OPACITY: f32 = 0.58;

pub(crate) fn configure(app: &mut App) {
    app.register_developer_visualization(VisualizationSpec::new(
        CELLS_VISUALIZATION,
        "Thermal bodies / cells",
        10,
        false,
    ))
    .register_developer_visualization(VisualizationSpec::new(
        COUPLING_FIELD_VISUALIZATION,
        "Thermal coupling field",
        11,
        false,
    ))
    .add_systems(
        PostUpdate,
        collect_thermal_observations.in_set(DeveloperSet::CollectWorldDraw),
    );
}

fn collect_thermal_observations(
    tools: Res<DeveloperTools>,
    view: Res<DeveloperView>,
    transforms: Query<&GlobalTransform>,
    thermal_bodies: Query<&ThermalBody>,
    thermal_fields: Query<(&ThermalField, &ThermalMaterial)>,
    samples: Query<
        (&UsfManifestationOf, &GlobalTransform),
        (
            With<ThermalSpatialSample>,
            Or<(Without<SpatialSplitPeer>, With<SpatialSplitPeerActive>)>,
        ),
    >,
    sources: Query<(Entity, &Combustion, &CombustibleMaterial)>,
    frame: Res<WorldDrawFrame>,
) {
    let cells_enabled = tools.visualization_enabled(CELLS_VISUALIZATION);
    let field_enabled = tools.visualization_enabled(COUPLING_FIELD_VISUALIZATION);
    if !cells_enabled && !field_enabled {
        return;
    }

    let mut batch = WorldDrawBatch::default();

    if cells_enabled {
        let range = ScalarRange::new(273.15, 800.0);
        for (relation, transform) in &samples {
            let Ok(body) = thermal_bodies.get(relation.0) else {
                continue;
            };

            let temperature = body.temperature_kelvin();
            let color = ColorRamp::THERMAL.sample_scalar(range, temperature);
            let position = transform.translation();

            batch.cross(
                Isometry3d::new(position, Quat::IDENTITY),
                0.35,
                color,
                DrawDepth::World,
            );
            batch.sphere(
                Isometry3d::new(position, Quat::IDENTITY),
                0.28,
                color,
                12,
                DrawDepth::World,
            );

            if let Ok((field, material)) = thermal_fields.get(relation.0) {
                let minimum = field.minimum_temperature_kelvin(material);
                let maximum = field.maximum_temperature_kelvin(material);
                let cell_range = ScalarRange::new(minimum, maximum.max(minimum + 1.0));
                let radius = (field.cell_size_meters().min_element() * 0.16).clamp(0.015, 0.12);

                for cell in field.cell_samples(material) {
                    let cell_position = transform.affine().transform_point3(cell.local_center);
                    let cell_color = ColorRamp::THERMAL
                        .sample_scalar(cell_range, cell.temperature_kelvin);
                    batch.sphere(
                        Isometry3d::new(cell_position, Quat::IDENTITY),
                        radius,
                        cell_color,
                        6,
                        DrawDepth::World,
                    );
                }
            }
        }
    }

    if !field_enabled {
        frame.submit(batch);
        return;
    }

    let Some(observer) = view
        .observer()
        .and_then(|entity| transforms.get(entity).ok())
        .map(GlobalTransform::compute_transform)
    else {
        frame.submit(batch);
        return;
    };

    let mut positions_by_semantic = HashMap::<Entity, Vec<Vec3>>::new();
    for (relation, transform) in &samples {
        positions_by_semantic
            .entry(relation.0)
            .or_default()
            .push(transform.translation());
    }

    let influence_sources = sources
        .iter()
        .filter_map(|(entity, combustion, material)| {
            let positions = positions_by_semantic.get(&entity)?.clone();
            let coupling = combustion_heat_coupling(combustion, material);

            (!positions.is_empty()
                && coupling.environmental_power_watts > 0.0
                && coupling.radius_meters > 0.0)
                .then_some((
                    positions,
                    coupling.environmental_power_watts,
                    coupling.radius_meters,
                ))
        })
        .collect::<Vec<_>>();

    if influence_sources.is_empty() {
        frame.submit(batch);
        return;
    }

    let resolution = UVec2::splat(FIELD_RESOLUTION);
    let transform = Transform {
        translation: Vec3::new(observer.translation.x, 0.035, observer.translation.z),
        rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
        ..default()
    };

    let minimum = -Vec2::splat(FIELD_SIZE_METERS) * 0.5;
    let denominator = (resolution - UVec2::ONE).as_vec2();
    let mut values = Vec::with_capacity((resolution.x * resolution.y) as usize);
    let mut maximum = 0.0_f32;

    for y in 0..resolution.y {
        for x in 0..resolution.x {
            let uv = Vec2::new(x as f32, y as f32) / denominator;
            let local_xy = minimum + Vec2::splat(FIELD_SIZE_METERS) * uv;
            let local = Vec3::new(local_xy.x, local_xy.y, 0.0);
            let point = transform.transform_point(local);
            let mut influence = 0.0_f32;

            for (positions, power, radius) in &influence_sources {
                let distance = positions
                    .iter()
                    .map(|position| position.distance(point))
                    .fold(f32::INFINITY, f32::min);
                influence += *power * radial_heat_weight(distance, *radius);
            }

            maximum = maximum.max(influence);
            values.push(influence);
        }
    }

    batch.scalar_field(WorldScalarField {
        id: FIELD_OBSERVATION,
        transform,
        size: Vec2::splat(FIELD_SIZE_METERS),
        resolution,
        values,
        range: ScalarRange::new(0.0, maximum.max(1.0)),
        ramp: ColorRamp::THERMAL,
        opacity: FIELD_OPACITY,
        mode: ScalarFieldMode::Heatmap,
        height_scale: 0.0,
    });
    frame.submit(batch);
}
