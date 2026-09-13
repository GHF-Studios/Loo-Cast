//! Thermal observability: semantic samples plus configurable scalar slices.

use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    ecs::UsfManifestationOf,
    observability::{
        AppObservabilityExt, DebugChoiceOption, DebugColorRamp, DebugContext,
        DebugControlSpec, DebugControls, DebugDepth, DebugFrame, DebugFrameBatch, DebugId,
        DebugScalarField, DebugScalarFieldMode, DebugScalarRange, ObservabilitySet,
        CATEGORY_WORLD,
    },
    physics::topology::{SpatialSplitPeer, SpatialSplitPeerActive},
};

use super::{
    coupling::{combustion_heat_coupling, radial_heat_weight}, CombustibleMaterial, Combustion,
    ThermalBody, ThermalField, ThermalMaterial, ThermalSpatialSample,
};

const TOOL: DebugId = DebugId("world.thermal");
const SAMPLES: DebugId = DebugId("world.thermal.samples");
const SAMPLE_LABELS: DebugId = DebugId("world.thermal.sample_labels");
const INTERNAL_CELLS: DebugId = DebugId("world.thermal.internal_cells");

const FIELD: DebugId = DebugId("world.thermal.field");
const FIELD_SLICE: DebugId = DebugId("world.thermal.field.slice");
const FIELD_VIEW_DISTANCE: DebugId = DebugId("world.thermal.field.view_distance");
const FIELD_MODE: DebugId = DebugId("world.thermal.field.mode");
const FIELD_RANGE: DebugId = DebugId("world.thermal.field.range");
const FIELD_FIXED_MAX: DebugId = DebugId("world.thermal.field.fixed_max");
const FIELD_SIZE: DebugId = DebugId("world.thermal.field.size");
const FIELD_RESOLUTION: DebugId = DebugId("world.thermal.field.resolution");
const FIELD_OPACITY: DebugId = DebugId("world.thermal.field.opacity");
const FIELD_HEIGHT: DebugId = DebugId("world.thermal.field.height");

const FIELD_OBSERVATION: DebugId = DebugId("observation.thermal.heat_coupling");

pub(crate) fn configure(app: &mut App) {
    app.register_debug_control(
        DebugControlSpec::tool(
            TOOL,
            Some(CATEGORY_WORLD),
            "Thermal",
            20,
            false,
        )
        .described("Aggregate temperatures, internal solid-energy gradients and combustion heat coupling."),
    )
    .register_debug_control(DebugControlSpec::toggle(
        SAMPLES,
        Some(TOOL),
        "Spatial samples",
        0,
        true,
    ))
    .register_debug_control(DebugControlSpec::toggle(
        SAMPLE_LABELS,
        Some(SAMPLES),
        "Temperature labels",
        0,
        true,
    ))
    .register_debug_control(DebugControlSpec::toggle(
        INTERNAL_CELLS,
        Some(SAMPLES),
        "Internal thermal cells",
        1,
        true,
    ))
    .register_debug_control(
        DebugControlSpec::toggle(
            FIELD,
            Some(TOOL),
            "Heat-coupling field",
            10,
            false,
        )
        .described(
            "Continuous pre-normalization coupling potential using the exact simulation distance kernel.",
        ),
    )
    .register_debug_control(DebugControlSpec::choice(
        FIELD_SLICE,
        Some(FIELD),
        "Slice plane",
        0,
        [
            DebugChoiceOption::new("ground_xz", "Ground XZ (world y=0)"),
            DebugChoiceOption::new("observer_xz", "Horizontal XZ at observer"),
            DebugChoiceOption::new("view", "View plane ahead of observer"),
        ],
        0,
    ))
    .register_debug_control(
        DebugControlSpec::scalar(
            FIELD_VIEW_DISTANCE,
            Some(FIELD),
            "View-plane distance",
            1,
            5.0,
            1.0,
            30.0,
            1.0,
            "m",
        )
        .when_choice(FIELD_SLICE, "view"),
    )
    .register_debug_control(DebugControlSpec::choice(
        FIELD_MODE,
        Some(FIELD),
        "Representation",
        2,
        [
            DebugChoiceOption::new("heatmap", "Heatmap"),
            DebugChoiceOption::new("height", "3D height field"),
        ],
        0,
    ))
    .register_debug_control(DebugControlSpec::choice(
        FIELD_RANGE,
        Some(FIELD),
        "Value range",
        3,
        [
            DebugChoiceOption::new("auto", "Auto"),
            DebugChoiceOption::new("fixed", "Fixed"),
        ],
        0,
    ))
    .register_debug_control(
        DebugControlSpec::scalar(
            FIELD_FIXED_MAX,
            Some(FIELD),
            "Fixed maximum",
            4,
            25_000.0,
            1_000.0,
            200_000.0,
            1_000.0,
            "W",
        )
        .when_choice(FIELD_RANGE, "fixed"),
    )
    .register_debug_control(DebugControlSpec::scalar(
        FIELD_SIZE,
        Some(FIELD),
        "Slice size",
        5,
        20.0,
        4.0,
        80.0,
        2.0,
        "m",
    ))
    .register_debug_control(DebugControlSpec::integer(
        FIELD_RESOLUTION,
        Some(FIELD),
        "Resolution",
        6,
        28,
        8,
        64,
        4,
        "",
    ))
    .register_debug_control(DebugControlSpec::scalar(
        FIELD_OPACITY,
        Some(FIELD),
        "Opacity",
        7,
        0.58,
        0.10,
        1.0,
        0.05,
        "",
    ))
    .register_debug_control(
        DebugControlSpec::scalar(
            FIELD_HEIGHT,
            Some(FIELD),
            "Height scale",
            8,
            2.5,
            0.25,
            10.0,
            0.25,
            "m",
        )
        .when_choice(FIELD_MODE, "height"),
    )
    .add_systems(
        PostUpdate,
        collect_thermal_observations.in_set(ObservabilitySet::Collect),
    );
}

fn collect_thermal_observations(
    controls: Res<DebugControls>,
    context: Res<DebugContext>,
    transforms: Query<&GlobalTransform>,
    thermal_bodies: Query<&ThermalBody>,
    thermal_fields: Query<(&ThermalField, &ThermalMaterial)>,
    combustions: Query<(), With<Combustion>>,
    samples: Query<
        (&UsfManifestationOf, &GlobalTransform),
        (
            With<ThermalSpatialSample>,
            Or<(Without<SpatialSplitPeer>, With<SpatialSplitPeerActive>)>,
        ),
    >,
    sources: Query<(Entity, &Combustion, &CombustibleMaterial)>,
    frame: Res<DebugFrame>,
) {
    if !controls.active(TOOL) {
        return;
    }

    let mut batch = DebugFrameBatch::default();

    if controls.active(SAMPLES) {
        let range = DebugScalarRange::new(273.15, 800.0);
        for (relation, transform) in &samples {
            let Ok(body) = thermal_bodies.get(relation.0) else {
                continue;
            };

            let temperature = body.temperature_kelvin();
            let color = DebugColorRamp::THERMAL.sample_scalar(range, temperature);
            let position = transform.translation();

            batch.cross(
                Isometry3d::new(position, Quat::IDENTITY),
                0.35,
                color,
                DebugDepth::World,
            );
            batch.sphere(
                Isometry3d::new(position, Quat::IDENTITY),
                0.28,
                color,
                12,
                DebugDepth::World,
            );

            let spatial = thermal_fields.get(relation.0).ok();
            if controls.active(INTERNAL_CELLS) {
                if let Some((field, material)) = spatial {
                    let minimum = field.minimum_temperature_kelvin(material);
                    let maximum = field.maximum_temperature_kelvin(material);
                    let cell_range = DebugScalarRange::new(minimum, maximum.max(minimum + 1.0));
                    let radius = (field.cell_size_meters().min_element() * 0.16)
                        .clamp(0.015, 0.12);

                    for cell in field.cell_samples(material) {
                        let cell_position = transform.affine().transform_point3(cell.local_center);
                        let cell_color = DebugColorRamp::THERMAL
                            .sample_scalar(cell_range, cell.temperature_kelvin);
                        batch.sphere(
                            Isometry3d::new(cell_position, Quat::IDENTITY),
                            radius,
                            cell_color,
                            6,
                            DebugDepth::World,
                        );
                    }
                }
            }

            if controls.active(SAMPLE_LABELS) {
                let spatial_summary = spatial
                    .map(|(field, material)| {
                        format!(
                            "\nmin {:.1} / max {:.1} K\nrho {:.0} kg/m^3, cp {:.0} J/(kg K), k {:.3} W/(m K)",
                            field.minimum_temperature_kelvin(material),
                            field.maximum_temperature_kelvin(material),
                            material.density_kg_per_cubic_meter,
                            material.specific_heat_capacity_joules_per_kg_kelvin,
                            material.thermal_conductivity_watts_per_meter_kelvin,
                        )
                    })
                    .unwrap_or_default();
                batch.label(
                    position + Vec3::Y * 0.65,
                    format!(
                        "{temperature:.1} K{spatial_summary}{}",
                        if combustions.contains(relation.0) {
                            " [burning]"
                        } else {
                            ""
                        },
                    ),
                    DebugFrameBatch::DEFAULT_LABEL_FONT_SIZE,
                    color,
                );
            }
        }
    }

    if !controls.active(FIELD) {
        frame.submit(batch);
        return;
    }

    let Some(observer) = context
        .observer
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

    let size = controls.scalar_value(FIELD_SIZE).unwrap_or(20.0);
    let resolution = controls.integer_value(FIELD_RESOLUTION).unwrap_or(28).max(2);
    let resolution = UVec2::splat(resolution);
    let transform = slice_transform(
        controls.choice_value(FIELD_SLICE).unwrap_or("ground_xz"),
        &observer,
        controls.scalar_value(FIELD_VIEW_DISTANCE).unwrap_or(5.0),
    );

    let minimum = -Vec2::splat(size) * 0.5;
    let denominator = (resolution - UVec2::ONE).as_vec2();
    let mut values = Vec::with_capacity((resolution.x * resolution.y) as usize);
    let mut maximum = 0.0_f32;

    for y in 0..resolution.y {
        for x in 0..resolution.x {
            let uv = Vec2::new(x as f32, y as f32) / denominator;
            let local_xy = minimum + Vec2::splat(size) * uv;
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

    let maximum = if controls.choice_value(FIELD_RANGE) == Some("fixed") {
        controls.scalar_value(FIELD_FIXED_MAX).unwrap_or(25_000.0)
    } else {
        maximum.max(1.0)
    };

    batch.scalar_field(DebugScalarField {
        id: FIELD_OBSERVATION,
        transform,
        size: Vec2::splat(size),
        resolution,
        values,
        range: DebugScalarRange::new(0.0, maximum),
        ramp: DebugColorRamp::THERMAL,
        opacity: controls.scalar_value(FIELD_OPACITY).unwrap_or(0.58),
        mode: if controls.choice_value(FIELD_MODE) == Some("height") {
            DebugScalarFieldMode::HeightField
        } else {
            DebugScalarFieldMode::Heatmap
        },
        height_scale: controls.scalar_value(FIELD_HEIGHT).unwrap_or(2.5),
        title: "Combustion heat-coupling potential".to_owned(),
        unit: "W",
        legend: true,
    });
    frame.submit(batch);
}

fn slice_transform(mode: &str, observer: &Transform, view_distance: f32) -> Transform {
    match mode {
        "observer_xz" => Transform {
            translation: observer.translation,
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
            ..default()
        },
        "view" => Transform {
            translation: observer.translation
                + observer.rotation * Vec3::NEG_Z * view_distance.max(0.0),
            rotation: observer.rotation,
            ..default()
        },
        _ => Transform {
            translation: Vec3::new(observer.translation.x, 0.035, observer.translation.z),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
            ..default()
        },
    }
}
