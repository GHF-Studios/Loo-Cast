//! Direct thermal-energy impulses and ambient exchange.

use super::*;

pub(super) fn apply_thermal_impulses(
    mut impulses: MessageReader<ThermalImpulse>,
    runtime_ownership: UsfRuntimeOwnershipQuery,
    mut thermal_bodies: Query<&mut ThermalBody>,
) {
    for impulse in impulses.read() {
        let target = runtime_ownership
            .semantic_of(impulse.target)
            .unwrap_or(impulse.target);

        let Ok(mut body) = thermal_bodies.get_mut(target) else {
            continue;
        };

        body.add_energy_joules(impulse.energy_joules);
    }
}

pub(super) fn cool_thermal_bodies(time: Res<Time>, mut bodies: Query<&mut ThermalBody>) {
    let dt = time.delta_secs().max(0.0);
    for mut body in &mut bodies {
        body.exchange_with_ambient(dt);
    }
}
