use super::*;
use crate::game::thermal::AMBIENT_TEMPERATURE_KELVIN;

#[test]
fn thermal_energy_changes_temperature_by_heat_capacity() {
    let mut body = ThermalBody::ambient(1_000.0, 0.0);
    body.add_energy_joules(100_000.0);
    assert!((body.temperature_kelvin() - (AMBIENT_TEMPERATURE_KELVIN + 100.0)).abs() < 1e-4);
}
