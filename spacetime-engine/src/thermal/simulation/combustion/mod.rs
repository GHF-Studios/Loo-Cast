//! Combustion lifecycle and manifestation-space heat propagation.

use super::*;

/// Derives combustion from material + temperature + remaining fuel and consumes
/// chemical energy while combustion is active.
pub(super) fn update_combustion(
    mut commands: Commands,
    time: Res<Time>,
    spatial_fields: Query<(&ThermalField, &ThermalMaterial)>,
    mut candidates: Query<(
        Entity,
        &ThermalBody,
        &CombustibleMaterial,
        &mut Fuel,
        Option<&Combustion>,
    )>,
) {
    let dt = time.delta_secs().max(0.0);

    for (entity, body, material, mut fuel, combustion) in &mut candidates {
        let has_fuel = fuel.remaining_energy_joules() > 0.0;
        let temperature = spatial_fields
            .get(entity)
            .map(|(field, material)| field.maximum_temperature_kelvin(material))
            .unwrap_or_else(|_| body.temperature_kelvin());

        match combustion {
            Some(combustion) => {
                let consumed = fuel.consume(combustion.power_watts() * dt);
                if consumed <= 0.0
                    || fuel.remaining_energy_joules() <= 0.0
                    || temperature < material.extinction_temperature_kelvin
                {
                    commands.entity(entity).remove::<Combustion>();
                }
            }
            None => {
                if has_fuel && temperature >= material.ignition_temperature_kelvin {
                    commands
                        .entity(entity)
                        .insert(Combustion::new(material.burn_power_watts));
                }
            }
        }
    }
}
