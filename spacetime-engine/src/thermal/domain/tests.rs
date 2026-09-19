use super::*;

#[test]
fn fuel_consumption_is_bounded_and_observable() {
    let mut fuel = Fuel::new(100.0);
    assert_eq!(fuel.consume(30.0), 30.0);
    assert_eq!(fuel.remaining_energy_joules(), 70.0);
    assert_eq!(fuel.consume(100.0), 70.0);
    assert_eq!(fuel.remaining_energy_joules(), 0.0);
}

#[test]
fn combustion_power_is_readable_without_mutating_simulation() {
    let combustion = Combustion::new(12_000.0);
    assert_eq!(combustion.power_watts(), 12_000.0);
}
}
