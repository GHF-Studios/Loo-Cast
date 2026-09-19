//! Test-game adaptation of dangerous temperature into generic health damage.

use bevy::prelude::*;

use crate::{
    game::SimulationSet,
    thermal::{ThermalBody, ThermalInjury, ThermalSet},
};

use super::Damage;

pub(super) fn configure(app: &mut App) {
    app.add_systems(
        Update,
        emit_thermal_injury_damage
            .in_set(SimulationSet::Phenomena)
            .after(ThermalSet::Lumped)
            .before(ThermalSet::SpatialOutput),
    );
}

/// Converts dangerous biological temperature into generic Damage
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
