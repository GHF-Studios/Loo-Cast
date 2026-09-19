//! Domain-owned validation/commit path for thermal inspection requests.

use super::*;

/// Domain-owned commit path for generic inspection proposals.
pub(super) fn apply_thermal_inspection_edits(
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

pub(super) fn apply_thermal_inspection_actions(
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
