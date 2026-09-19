//! Adaptation of dangerous temperature into generic combat damage.

use super::*;

/// Converts dangerous biological temperature into the generic combat Damage
/// protocol. Inanimate degradation gets its own adapter rather than pretending
/// every material has biology-style Health response.
pub(super) fn emit_thermal_injury_damage(
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
