//! Thermal Structure and semantic inspection collection.

use super::*;

pub(super) fn collect_thermal_structure(
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

pub(super) fn collect_thermal_inspection(
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
