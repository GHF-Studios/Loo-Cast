//! Structured semantic inspection and contextual gizmo support for thermal state.
//!
//! Thermal is intentionally the second proof case after Transform: most runtime
//! facts are observational, selected parameters use validated domain-owned edits,
//! and heat/cool/reset are explicit actions rather than fake mutable fields.

use bevy::prelude::*;

use crate::{
    devtools::{
        DeveloperFocus, DeveloperSet, DrawDepth, InspectAccess, InspectAction, InspectActionId,
        InspectActionRequest, InspectEditRequest, InspectField, InspectFieldId,
        InspectNumberFormat, InspectNumberInput, InspectSection, InspectSectionId, InspectUnit,
        InspectValue, InspectionFrame, StructureFrame, StructureItem, StructureItemId,
        StructureSelection, WorldDrawBatch, WorldDrawFrame,
    },
    ecs::UsfManifestations,
    view::PrimaryViewPresentation,
};

use super::{
    AMBIENT_TEMPERATURE_KELVIN, CombustibleMaterial, Combustion, Fuel, ThermalBody, ThermalField,
    ThermalImpulse, ThermalMaterial, ThermalSpatialSample,
};

const THERMAL_SECTION: InspectSectionId = InspectSectionId("thermal");
const MATERIAL_SECTION: InspectSectionId = InspectSectionId("thermal.material");
const COMBUSTION_SECTION: InspectSectionId = InspectSectionId("thermal.combustion");
pub(crate) const THERMAL_STRUCTURE: StructureItemId = StructureItemId("thermal.state");

const HEAT_CAPACITY_FIELD: InspectFieldId = InspectFieldId("thermal.body.heat_capacity");
const COOLING_FIELD: InspectFieldId = InspectFieldId("thermal.body.cooling");
const IGNITION_FIELD: InspectFieldId = InspectFieldId("thermal.combustion.ignition");
const EXTINCTION_FIELD: InspectFieldId = InspectFieldId("thermal.combustion.extinction");
const BURN_POWER_FIELD: InspectFieldId = InspectFieldId("thermal.combustion.power");
const SELF_HEATING_FIELD: InspectFieldId = InspectFieldId("thermal.combustion.self_heating");
const ENVIRONMENTAL_TRANSFER_FIELD: InspectFieldId =
    InspectFieldId("thermal.combustion.environmental_transfer");
const TRANSFER_RADIUS_FIELD: InspectFieldId = InspectFieldId("thermal.combustion.transfer_radius");

const HEAT_ACTION: InspectActionId = InspectActionId("thermal.heat");
const COOL_ACTION: InspectActionId = InspectActionId("thermal.cool");
const RESET_AMBIENT_ACTION: InspectActionId = InspectActionId("thermal.reset_ambient");
const THERMAL_NUDGE_JOULES: f32 = 10_000.0;

pub(crate) fn configure(app: &mut App) {
    app.add_systems(
        PreUpdate,
        (
            apply_thermal_inspection_edits,
            apply_thermal_inspection_actions,
        ),
    )
    .add_systems(
        PostUpdate,
        collect_thermal_structure.in_set(DeveloperSet::CollectStructure),
    )
    .add_systems(
        PostUpdate,
        collect_thermal_inspection.in_set(DeveloperSet::CollectInspection),
    )
    .add_systems(
        PostUpdate,
        collect_focused_thermal_gizmo.in_set(DeveloperSet::CollectWorldDraw),
    );
}

fn collect_thermal_structure(
    focus: Res<DeveloperFocus>,
    thermal_bodies: Query<(), With<ThermalBody>>,
    mut frame: ResMut<StructureFrame>,
) {
    let Some(target) = focus.current() else {
        return;
    };
    if !thermal_bodies.contains(target.semantic_entity) {
        return;
    }

    frame.submit(
        StructureItem::new(THERMAL_STRUCTURE, "Thermal", 20)
            .detail("temperature, energy, material response and combustion"),
    );
}

fn collect_thermal_inspection(
    focus: Res<DeveloperFocus>,
    thermal_bodies: Query<&ThermalBody>,
    thermal_fields: Query<(&ThermalField, &ThermalMaterial)>,
    combustible_materials: Query<&CombustibleMaterial>,
    fuels: Query<&Fuel>,
    combustions: Query<&Combustion>,
    mut frame: ResMut<InspectionFrame>,
) {
    let Some(target) = focus.current() else {
        return;
    };
    let Ok(body) = thermal_bodies.get(target.semantic_entity) else {
        return;
    };

    let five = InspectNumberFormat::significant_digits(5);
    let four = InspectNumberFormat::significant_digits(4);

    let mut thermal = InspectSection::new(THERMAL_SECTION, "Thermal state", 20)
        .for_structure(THERMAL_STRUCTURE)
        .contextual_gizmo()
        .field(
            InspectField::new(
                "Temperature",
                InspectValue::Quantity {
                    value: body.temperature_kelvin() as f64,
                    unit: InspectUnit::KELVIN,
                    format: five,
                },
            )
            .hint("Simulation-owned aggregate temperature; use thermal actions to change energy."),
        )
        .field(
            InspectField::new(
                "Heat capacity",
                InspectValue::Quantity {
                    value: body.heat_capacity_joules_per_kelvin as f64,
                    unit: InspectUnit::HEAT_CAPACITY,
                    format: five,
                },
            )
            .editable(HEAT_CAPACITY_FIELD, InspectAccess::Validated)
            .number_input(InspectNumberInput::speed(100.0).range(0.001, f32::MAX as f64))
            .hint("Validated runtime parameter; must remain finite and positive."),
        )
        .field(
            InspectField::new(
                "Cooling",
                InspectValue::Quantity {
                    value: body.cooling_watts_per_kelvin as f64,
                    unit: InspectUnit::COOLING_COEFFICIENT,
                    format: five,
                },
            )
            .editable(COOLING_FIELD, InspectAccess::Validated)
            .number_input(InspectNumberInput::speed(1.0).range(0.0, f32::MAX as f64))
            .hint("Validated runtime parameter; zero disables ambient cooling."),
        )
        .field(InspectField::new(
            "Burning",
            InspectValue::Bool(combustions.contains(target.semantic_entity)),
        ))
        .action(
            InspectAction::new(HEAT_ACTION, "+10 kJ heat")
                .hint("Adds thermal energy through ThermalImpulse."),
        )
        .action(
            InspectAction::new(COOL_ACTION, "−10 kJ cool")
                .hint("Removes thermal energy through ThermalImpulse, respecting simulation floors."),
        )
        .action(
            InspectAction::new(RESET_AMBIENT_ACTION, "Reset to ambient")
                .hint("Requests the exact aggregate energy delta required to return to ambient temperature."),
        );

    if let Ok((field, material)) = thermal_fields.get(target.semantic_entity) {
        thermal = thermal
            .field(InspectField::new(
                "Internal temperature",
                InspectValue::Range {
                    minimum: field.minimum_temperature_kelvin(material) as f64,
                    maximum: field.maximum_temperature_kelvin(material) as f64,
                    unit: InspectUnit::KELVIN,
                    format: five,
                },
            ))
            .field(InspectField::new(
                "Internal energy",
                InspectValue::Quantity {
                    value: field.total_energy_joules() as f64,
                    unit: InspectUnit::JOULE,
                    format: five,
                },
            ))
            .field(InspectField::new(
                "Grid",
                InspectValue::text(format!(
                    "{} × {} × {} cells",
                    field.resolution().x,
                    field.resolution().y,
                    field.resolution().z,
                )),
            ));

        frame.submit(thermal);
        frame.submit(
            InspectSection::new(MATERIAL_SECTION, "Thermal material", 21)
                .for_structure(THERMAL_STRUCTURE)
                .contextual_gizmo()
                .field(
                    InspectField::new(
                        "Density",
                        InspectValue::Quantity {
                            value: material.density_kg_per_cubic_meter as f64,
                            unit: InspectUnit::DENSITY,
                            format: five,
                        },
                    )
                    .symbol("ρ"),
                )
                .field(
                    InspectField::new(
                        "Specific heat",
                        InspectValue::Quantity {
                            value: material.specific_heat_capacity_joules_per_kg_kelvin as f64,
                            unit: InspectUnit::SPECIFIC_HEAT_CAPACITY,
                            format: five,
                        },
                    )
                    .symbol("cₚ"),
                )
                .field(
                    InspectField::new(
                        "Conductivity",
                        InspectValue::Quantity {
                            value: material.thermal_conductivity_watts_per_meter_kelvin as f64,
                            unit: InspectUnit::THERMAL_CONDUCTIVITY,
                            format: five,
                        },
                    )
                    .symbol("k"),
                )
                .field(
                    InspectField::new(
                        "Diffusivity",
                        InspectValue::Quantity {
                            value: material.thermal_diffusivity_square_meters_per_second() as f64,
                            unit: InspectUnit::THERMAL_DIFFUSIVITY,
                            format: four,
                        },
                    )
                    .symbol("α"),
                ),
        );
    } else {
        frame.submit(thermal);
    }

    if let Ok(material) = combustible_materials.get(target.semantic_entity) {
        let mut combustion = InspectSection::new(COMBUSTION_SECTION, "Combustion", 22)
            .for_structure(THERMAL_STRUCTURE)
            .contextual_gizmo()
            .field(
                InspectField::new(
                    "Ignition temperature",
                    InspectValue::quantity(
                        material.ignition_temperature_kelvin as f64,
                        InspectUnit::KELVIN,
                    ),
                )
                .editable(IGNITION_FIELD, InspectAccess::Validated)
                .number_input(InspectNumberInput::speed(1.0).range(1.0, f32::MAX as f64))
                .hint("Must remain greater than or equal to the extinction threshold."),
            )
            .field(
                InspectField::new(
                    "Extinction temperature",
                    InspectValue::quantity(
                        material.extinction_temperature_kelvin as f64,
                        InspectUnit::KELVIN,
                    ),
                )
                .editable(EXTINCTION_FIELD, InspectAccess::Validated)
                .number_input(InspectNumberInput::speed(1.0).range(1.0, f32::MAX as f64))
                .hint("Must remain less than or equal to the ignition threshold."),
            )
            .field(
                InspectField::new(
                    "Burn power",
                    InspectValue::quantity(material.burn_power_watts as f64, InspectUnit::WATT),
                )
                .editable(BURN_POWER_FIELD, InspectAccess::Validated)
                .number_input(InspectNumberInput::speed(1_000.0).range(0.0, f32::MAX as f64)),
            )
            .field(
                InspectField::new(
                    "Self heating fraction",
                    InspectValue::Number {
                        value: material.self_heating_fraction as f64,
                        format: four,
                    },
                )
                .editable(SELF_HEATING_FIELD, InspectAccess::Validated)
                .number_input(InspectNumberInput::speed(0.01).range(0.0, 1.0)),
            )
            .field(
                InspectField::new(
                    "Environmental transfer fraction",
                    InspectValue::Number {
                        value: material.environmental_transfer_fraction as f64,
                        format: four,
                    },
                )
                .editable(ENVIRONMENTAL_TRANSFER_FIELD, InspectAccess::Validated)
                .number_input(InspectNumberInput::speed(0.01).range(0.0, 1.0)),
            )
            .field(
                InspectField::new(
                    "Heat transfer radius",
                    InspectValue::quantity(
                        material.heat_transfer_radius_meters as f64,
                        InspectUnit::METER,
                    ),
                )
                .editable(TRANSFER_RADIUS_FIELD, InspectAccess::Validated)
                .number_input(InspectNumberInput::speed(0.05).range(0.0, f32::MAX as f64)),
            );

        if let Ok(fuel) = fuels.get(target.semantic_entity) {
            combustion = combustion.field(InspectField::new(
                "Fuel remaining",
                InspectValue::quantity(fuel.remaining_energy_joules() as f64, InspectUnit::JOULE),
            ));
        }
        if let Ok(state) = combustions.get(target.semantic_entity) {
            combustion = combustion.field(InspectField::new(
                "Current combustion power",
                InspectValue::quantity(state.power_watts() as f64, InspectUnit::WATT),
            ));
        }
        frame.submit(combustion);
    }
}

/// Domain-owned commit path for generic inspection proposals.
fn apply_thermal_inspection_edits(
    mut requests: MessageReader<InspectEditRequest>,
    mut bodies: Query<&mut ThermalBody>,
    mut combustible_materials: Query<&mut CombustibleMaterial>,
) {
    for request in requests.read() {
        let entity = request.target.semantic_entity;
        let Some(value) = request.value.as_number().map(|value| value as f32) else {
            continue;
        };
        if !value.is_finite() {
            continue;
        }

        match (request.section, request.field) {
            (THERMAL_SECTION, HEAT_CAPACITY_FIELD) => {
                if value > 0.0
                    && let Ok(mut body) = bodies.get_mut(entity)
                {
                    body.heat_capacity_joules_per_kelvin = value;
                }
            }
            (THERMAL_SECTION, COOLING_FIELD) => {
                if value >= 0.0
                    && let Ok(mut body) = bodies.get_mut(entity)
                {
                    body.cooling_watts_per_kelvin = value;
                }
            }
            (COMBUSTION_SECTION, field) => {
                let Ok(mut material) = combustible_materials.get_mut(entity) else {
                    continue;
                };
                match field {
                    IGNITION_FIELD
                        if value >= 1.0 && value >= material.extinction_temperature_kelvin =>
                    {
                        material.ignition_temperature_kelvin = value;
                    }
                    EXTINCTION_FIELD
                        if value >= 1.0 && value <= material.ignition_temperature_kelvin =>
                    {
                        material.extinction_temperature_kelvin = value;
                    }
                    BURN_POWER_FIELD if value >= 0.0 => material.burn_power_watts = value,
                    SELF_HEATING_FIELD if (0.0..=1.0).contains(&value) => {
                        material.self_heating_fraction = value;
                    }
                    ENVIRONMENTAL_TRANSFER_FIELD if (0.0..=1.0).contains(&value) => {
                        material.environmental_transfer_fraction = value;
                    }
                    TRANSFER_RADIUS_FIELD if value >= 0.0 => {
                        material.heat_transfer_radius_meters = value;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

fn apply_thermal_inspection_actions(
    mut requests: MessageReader<InspectActionRequest>,
    bodies: Query<&ThermalBody>,
    mut impulses: MessageWriter<ThermalImpulse>,
) {
    for request in requests.read() {
        if request.section != THERMAL_SECTION {
            continue;
        }
        let entity = request.target.semantic_entity;
        let energy_joules = match request.action {
            HEAT_ACTION => THERMAL_NUDGE_JOULES,
            COOL_ACTION => -THERMAL_NUDGE_JOULES,
            RESET_AMBIENT_ACTION => {
                let Ok(body) = bodies.get(entity) else {
                    continue;
                };
                (AMBIENT_TEMPERATURE_KELVIN - body.temperature_kelvin())
                    * body.heat_capacity_joules_per_kelvin
            }
            _ => continue,
        };

        if energy_joules.is_finite() && energy_joules.abs() > f32::EPSILON {
            impulses.write(ThermalImpulse {
                target: entity,
                energy_joules,
            });
        }
    }
}

/// Thermal's viewport gizmo is observational: it visualizes the focused thermal
/// samples and, when combustible, the authored heat-transfer radius. Mutation is
/// exposed separately through validated fields/actions in the contextual UI.
fn collect_focused_thermal_gizmo(
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
