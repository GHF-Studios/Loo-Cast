//! Pure thermal coupling functions shared by simulation and observability.
//!
//! The simulation remains authoritative, but debug tooling may query these pure
//! transfer semantics instead of re-implementing them and silently drifting.

use super::{CombustibleMaterial, Combustion};

#[derive(Debug, Clone, Copy)]
pub(crate) struct CombustionHeatCoupling {
    pub self_heating_power_watts: f32,
    pub environmental_power_watts: f32,
    pub radius_meters: f32,
}

pub(crate) fn combustion_heat_coupling(
    combustion: &Combustion,
    material: &CombustibleMaterial,
) -> CombustionHeatCoupling {
    let power_watts = combustion.power_watts().max(0.0);
    CombustionHeatCoupling {
        self_heating_power_watts: power_watts * material.self_heating_fraction.clamp(0.0, 1.0),
        environmental_power_watts: power_watts
            * material.environmental_transfer_fraction.clamp(0.0, 1.0),
        radius_meters: material.heat_transfer_radius_meters.max(0.0),
    }
}

/// Distance kernel used by combustion heat propagation.
///
/// Returns zero outside the radius and a quadratic falloff inside it.
pub(crate) fn radial_heat_weight(distance: f32, radius: f32) -> f32 {
    if !distance.is_finite() || !radius.is_finite() || radius <= 0.0 || distance >= radius {
        return 0.0;
    }
    let normalized = (1.0 - distance.max(0.0) / radius).clamp(0.0, 1.0);
    normalized * normalized
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heat_weight_is_bounded_and_reaches_zero_at_radius() {
        assert!((radial_heat_weight(0.0, 10.0) - 1.0).abs() < 1.0e-6);
        assert!(radial_heat_weight(5.0, 10.0) > 0.0);
        assert_eq!(radial_heat_weight(10.0, 10.0), 0.0);
        assert_eq!(radial_heat_weight(20.0, 10.0), 0.0);
    }

    #[test]
    fn combustion_coupling_clamps_material_fractions_and_radius() {
        let combustion = Combustion::new(10_000.0);
        let material = CombustibleMaterial {
            ignition_temperature_kelvin: 400.0,
            extinction_temperature_kelvin: 350.0,
            burn_power_watts: 10_000.0,
            self_heating_fraction: 1.5,
            environmental_transfer_fraction: -1.0,
            heat_transfer_radius_meters: -3.0,
        };
        let coupling = combustion_heat_coupling(&combustion, &material);
        assert_eq!(coupling.self_heating_power_watts, 10_000.0);
        assert_eq!(coupling.environmental_power_watts, 0.0);
        assert_eq!(coupling.radius_meters, 0.0);
    }
}
