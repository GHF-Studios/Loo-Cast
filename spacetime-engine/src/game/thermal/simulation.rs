//! Thermal evolution and combustion propagation.

use std::collections::HashMap;

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

#[derive(Default)]
struct HeatScratch {
    positions_by_semantic: HashMap<Entity, Vec<Vec3>>,
    weights: Vec<(Entity, f32)>,
    energy_by_target: HashMap<Entity, f32>,
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
    mut scratch: Local<HeatScratch>,
) {
    let dt = time.delta_secs().max(0.0);
    if dt <= 0.0 {
        return;
    }

    let HeatScratch {
        positions_by_semantic,
        weights,
        energy_by_target,
    } = &mut *scratch;

    for positions in positions_by_semantic.values_mut() {
        positions.clear();
    }
    for (relation, transform) in &samples {
        positions_by_semantic
            .entry(relation.0)
            .or_default()
            .push(transform.translation);
    }
    positions_by_semantic.retain(|_, positions| !positions.is_empty());
    energy_by_target.clear();

    for (source, combustion, material) in &sources {
        let coupling = super::coupling::combustion_heat_coupling(combustion, material);
        *energy_by_target.entry(source).or_default() += coupling.self_heating_power_watts * dt;

        let Some(source_positions) = positions_by_semantic.get(&source) else {
            continue;
        };
        if coupling.radius_meters <= 0.0 || coupling.environmental_power_watts <= 0.0 {
            continue;
        }

        weights.clear();
        let mut total_weight = 0.0;
        let radius_squared = coupling.radius_meters * coupling.radius_meters;

        for (&target, target_positions) in positions_by_semantic.iter() {
            if target == source {
                continue;
            }

            let minimum_distance_squared = source_positions
                .iter()
                .flat_map(|source_position| {
                    target_positions
                        .iter()
                        .map(move |target_position| source_position.distance_squared(*target_position))
                })
                .fold(f32::INFINITY, f32::min);

            if minimum_distance_squared >= radius_squared {
                continue;
            }

            let weight = super::coupling::radial_heat_weight(
                minimum_distance_squared.sqrt(),
                coupling.radius_meters,
            );
            if weight > 0.0 {
                weights.push((target, weight));
                total_weight += weight;
            }
        }

        let normalization = total_weight.max(1.0);
        let environmental_energy = coupling.environmental_power_watts * dt;
        for &(target, weight) in weights.iter() {
            *energy_by_target.entry(target).or_default() +=
                environmental_energy * weight / normalization;
        }
    }

    for (&target, &energy_joules) in energy_by_target.iter() {
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
