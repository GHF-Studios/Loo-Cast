use bevy::prelude::*;

use super::*;

#[test]
fn material_heat_capacity_comes_from_density_volume_and_specific_heat() {
    let material = ThermalMaterial::copper();
    let field = ThermalField::uniform_box(
        Vec3::new(2.0, 0.5, 0.25),
        UVec3::new(4, 2, 1),
        300.0,
        &material,
    );
    let expected = 2.0
        * 0.5
        * 0.25
        * material.density_kg_per_cubic_meter
        * material.specific_heat_capacity_joules_per_kg_kelvin;
    assert!(
        (field.total_heat_capacity_joules_per_kelvin(&material) - expected).abs()
            < expected * 1.0e-6
    );
}

#[test]
fn localized_energy_creates_a_gradient_and_conduction_conserves_energy() {
    let material = ThermalMaterial::copper();
    let mut field = ThermalField::uniform_box(
        Vec3::new(0.30, 0.10, 0.10),
        UVec3::new(3, 1, 1),
        300.0,
        &material,
    );

    let applied =
        field.add_energy_at_local_position(&material, Vec3::new(-0.149, 0.0, 0.0), 50_000.0);
    let energy_before_conduction = field.total_energy_joules();
    let hot_before = field
        .cell_temperature_kelvin(UVec3::ZERO, &material)
        .unwrap();
    let far_before = field
        .cell_temperature_kelvin(UVec3::new(2, 0, 0), &material)
        .unwrap();

    assert!((applied - 50_000.0).abs() < 1.0e-3);
    assert!(hot_before > far_before);

    for _ in 0..600 {
        field.conduct_internal(&material, 1.0 / 60.0);
    }

    let hot_after = field
        .cell_temperature_kelvin(UVec3::ZERO, &material)
        .unwrap();
    let middle_after = field
        .cell_temperature_kelvin(UVec3::new(1, 0, 0), &material)
        .unwrap();
    let energy_after = field.total_energy_joules();

    assert!(hot_after < hot_before);
    assert!(middle_after > 300.0);
    assert!(
        (energy_after - energy_before_conduction).abs()
            < energy_before_conduction.abs().max(1.0) * 1.0e-5
    );
}
