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
