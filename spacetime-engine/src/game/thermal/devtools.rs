//! Structured developer inspection for thermal state.
//!
//! This is intentionally independent from thermal world visualization toggles.

use bevy::prelude::*;

use crate::devtools::{
    DeveloperFocus, DeveloperSet, DeveloperTools, InspectField, InspectNumberFormat,
    InspectSection, InspectSectionId, InspectUnit, InspectValue, InspectionFrame,
};

use super::{Combustion, ThermalBody, ThermalField, ThermalMaterial};

const THERMAL_SECTION: InspectSectionId = InspectSectionId("thermal");
const MATERIAL_SECTION: InspectSectionId = InspectSectionId("thermal.material");

pub(crate) fn configure(app: &mut App) {
    app.add_systems(
        PostUpdate,
        collect_thermal_inspection.in_set(DeveloperSet::CollectInspection),
    );
}

fn collect_thermal_inspection(
    tools: Res<DeveloperTools>,
    focus: Res<DeveloperFocus>,
    thermal_bodies: Query<&ThermalBody>,
    thermal_fields: Query<(&ThermalField, &ThermalMaterial)>,
    combustions: Query<(), With<Combustion>>,
    mut frame: ResMut<InspectionFrame>,
) {
    if !tools.enabled() {
        return;
    }
    let Some(target) = focus.current() else {
        return;
    };
    let Ok(body) = thermal_bodies.get(target.semantic_entity) else {
        return;
    };

    let five = InspectNumberFormat::significant_digits(5);
    let four = InspectNumberFormat::significant_digits(4);

    let mut thermal = InspectSection::new(THERMAL_SECTION, "Thermal", 20)
        .field(InspectField::new(
            "Temperature",
            InspectValue::Quantity {
                value: body.temperature_kelvin() as f64,
                unit: InspectUnit::KELVIN,
                format: five,
            },
        ))
        .field(InspectField::new(
            "Burning",
            InspectValue::Bool(combustions.contains(target.semantic_entity)),
        ));

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
}
