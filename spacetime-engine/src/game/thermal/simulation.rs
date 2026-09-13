//! Thermal evolution and combustion propagation.

use bevy::prelude::*;

use crate::{
    ecs::UsfManifestationOf,
    game::combat::Damage,
    physics::topology::{SpatialSplitPeer, SpatialSplitPeerActive},
};

use super::{
    CombustibleMaterial, Combustion, Fuel, ThermalBody, ThermalField, ThermalImpulse,
    ThermalInjury, ThermalMaterial, ThermalSet, ThermalSpatialSample,
};

pub(super) fn configure(app: &mut App) {
    app.add_systems(
        Update,
        (
            apply_thermal_impulses,
            update_combustion,
            propagate_combustion_heat,
            cool_thermal_bodies,
            emit_thermal_injury_damage,
        )
            .chain()
            .in_set(ThermalSet::Lumped),
    );
}

fn apply_thermal_impulses(
    mut impulses: MessageReader<ThermalImpulse>,
    manifestations: Query<&UsfManifestationOf>,
    mut thermal_bodies: Query<&mut ThermalBody>,
) {
    for impulse in impulses.read() {
        let target = manifestations
            .get(impulse.target)
            .map(|manifestation| manifestation.0)
            .unwrap_or(impulse.target);

        let Ok(mut body) = thermal_bodies.get_mut(target) else {
            continue;
        };

        body.add_energy_joules(impulse.energy_joules);
    }
}

/// Derives combustion from material + temperature + remaining fuel and consumes
/// chemical energy while combustion is active.
fn update_combustion(
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

#[derive(Debug)]
struct HeatEmission {
    source: Entity,
    positions: Vec<Vec3>,
    self_heating_power_watts: f32,
    environmental_power_watts: f32,
    radius_meters: f32,
}

/// Transfers combustion heat through registered manifestation-space samples.
///
/// This deliberately uses an O(n²) semantic-entity pass in the first slice.
/// The *interaction contract* is the important part; large worlds can replace
/// candidate discovery with chunk/spatial indexing without changing thermal
/// state or combustion semantics.
fn propagate_combustion_heat(
    time: Res<Time>,
    samples: Query<
        (&UsfManifestationOf, &Transform),
        (
            With<ThermalSpatialSample>,
            Or<(Without<SpatialSplitPeer>, With<SpatialSplitPeerActive>)>,
        ),
    >,
    sources: Query<(Entity, &Combustion, &CombustibleMaterial)>,
    mut targets: Query<&mut ThermalBody>,
) {
    let dt = time.delta_secs().max(0.0);
    if dt <= 0.0 {
        return;
    }

    let mut positions_by_semantic = std::collections::HashMap::<Entity, Vec<Vec3>>::new();
    for (relation, transform) in &samples {
        positions_by_semantic
            .entry(relation.0)
            .or_default()
            .push(transform.translation);
    }

    let emissions: Vec<HeatEmission> = sources
        .iter()
        .map(|(source, combustion, material)| {
            let coupling = super::coupling::combustion_heat_coupling(combustion, material);
            HeatEmission {
                source,
                positions: positions_by_semantic
                    .get(&source)
                    .cloned()
                    .unwrap_or_default(),
                self_heating_power_watts: coupling.self_heating_power_watts,
                environmental_power_watts: coupling.environmental_power_watts,
                radius_meters: coupling.radius_meters,
            }
        })
        .collect();

    let mut energy_by_target = std::collections::HashMap::<Entity, f32>::new();

    for emission in &emissions {
        *energy_by_target.entry(emission.source).or_default() +=
            emission.self_heating_power_watts * dt;

        if emission.positions.is_empty()
            || emission.radius_meters <= 0.0
            || emission.environmental_power_watts <= 0.0
        {
            continue;
        }

        let mut weights = Vec::new();
        let mut total_weight = 0.0;

        for (&target, target_positions) in &positions_by_semantic {
            if target == emission.source || target_positions.is_empty() {
                continue;
            }

            let minimum_distance_squared = emission
                .positions
                .iter()
                .flat_map(|source| {
                    target_positions
                        .iter()
                        .map(move |target| source.distance_squared(*target))
                })
                .fold(f32::INFINITY, f32::min);

            let radius_squared = emission.radius_meters * emission.radius_meters;
            if minimum_distance_squared >= radius_squared {
                continue;
            }

            let distance = minimum_distance_squared.sqrt();
            let weight = super::coupling::radial_heat_weight(
                distance,
                emission.radius_meters,
            );
            if weight > 0.0 {
                weights.push((target, weight));
                total_weight += weight;
            }
        }

        // Distance attenuates each coupling, while normalization only kicks in
        // when the combined weights would otherwise exceed the source's finite
        // environmental heat budget.
        let normalization = total_weight.max(1.0);
        let environmental_energy = emission.environmental_power_watts * dt;

        for (target, weight) in weights {
            *energy_by_target.entry(target).or_default() +=
                environmental_energy * weight / normalization;
        }
    }

    for (target, energy_joules) in energy_by_target {
        if let Ok(mut thermal) = targets.get_mut(target) {
            thermal.add_energy_joules(energy_joules);
        }
    }
}

fn cool_thermal_bodies(time: Res<Time>, mut bodies: Query<&mut ThermalBody>) {
    let dt = time.delta_secs().max(0.0);
    for mut body in &mut bodies {
        body.exchange_with_ambient(dt);
    }
}

/// Converts dangerous biological temperature into the generic combat Damage
/// protocol. Inanimate degradation gets its own adapter rather than pretending
/// every material has biology-style Health response.
fn emit_thermal_injury_damage(
    time: Res<Time>,
    bodies: Query<(Entity, &ThermalBody, &ThermalInjury)>,
    mut damage: MessageWriter<Damage>,
) {
    let dt = time.delta_secs().max(0.0);

    for (entity, body, injury) in &bodies {
        let excess = body.temperature_kelvin() - injury.damage_threshold_kelvin;
        if excess <= 0.0 {
            continue;
        }

        let amount = excess * injury.damage_per_second_per_kelvin.max(0.0) * dt;
        if amount > 0.0 && amount.is_finite() {
            damage.write(Damage {
                target: entity,
                instigator: None,
                amount,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::thermal::AMBIENT_TEMPERATURE_KELVIN;

    #[test]
    fn thermal_energy_changes_temperature_by_heat_capacity() {
        let mut body = ThermalBody::ambient(1_000.0, 0.0);
        body.add_energy_joules(100_000.0);
        assert!((body.temperature_kelvin() - (AMBIENT_TEMPERATURE_KELVIN + 100.0)).abs() < 1e-4);
    }
}
