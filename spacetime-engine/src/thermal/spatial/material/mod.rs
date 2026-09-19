//! Thermodynamic material properties used by spatial thermal refinement.

use bevy::prelude::*;

/// Thermodynamic properties of a homogeneous solid at approximately room
/// temperature.
///
/// These are intentionally the material properties required by the current
/// conduction model, not a universal rendering/physics/material abstraction.
/// Real materials vary with alloy, moisture, grain, temperature and direction;
/// presets are representative engineering values rather than exact identities.
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct ThermalMaterial {
    /// Mass density, kg/m^3.
    pub density_kg_per_cubic_meter: f32,
    /// Specific heat capacity, J/(kg K).
    pub specific_heat_capacity_joules_per_kg_kelvin: f32,
    /// Isotropic thermal conductivity, W/(m K).
    pub thermal_conductivity_watts_per_meter_kelvin: f32,
}

impl ThermalMaterial {
    pub fn new(
        density_kg_per_cubic_meter: f32,
        specific_heat_capacity_joules_per_kg_kelvin: f32,
        thermal_conductivity_watts_per_meter_kelvin: f32,
    ) -> Self {
        let material = Self {
            density_kg_per_cubic_meter,
            specific_heat_capacity_joules_per_kg_kelvin,
            thermal_conductivity_watts_per_meter_kelvin,
        };
        assert!(material.is_valid());
        material
    }

    /// Representative dry wood values. Wood is strongly anisotropic in reality;
    /// the first solver intentionally uses one isotropic conductivity.
    pub const fn dry_wood() -> Self {
        Self {
            density_kg_per_cubic_meter: 500.0,
            specific_heat_capacity_joules_per_kg_kelvin: 1_700.0,
            thermal_conductivity_watts_per_meter_kelvin: 0.12,
        }
    }

    pub const fn copper() -> Self {
        Self {
            density_kg_per_cubic_meter: 8_960.0,
            specific_heat_capacity_joules_per_kg_kelvin: 385.0,
            thermal_conductivity_watts_per_meter_kelvin: 401.0,
        }
    }

    pub const fn aluminum() -> Self {
        Self {
            density_kg_per_cubic_meter: 2_700.0,
            specific_heat_capacity_joules_per_kg_kelvin: 897.0,
            thermal_conductivity_watts_per_meter_kelvin: 237.0,
        }
    }

    pub const fn carbon_steel() -> Self {
        Self {
            density_kg_per_cubic_meter: 7_850.0,
            specific_heat_capacity_joules_per_kg_kelvin: 486.0,
            thermal_conductivity_watts_per_meter_kelvin: 50.0,
        }
    }

    pub fn thermal_diffusivity_square_meters_per_second(&self) -> f32 {
        self.thermal_conductivity_watts_per_meter_kelvin
            / (self.density_kg_per_cubic_meter * self.specific_heat_capacity_joules_per_kg_kelvin)
    }

    pub(super) fn is_valid(&self) -> bool {
        self.density_kg_per_cubic_meter.is_finite()
            && self.density_kg_per_cubic_meter > 0.0
            && self.specific_heat_capacity_joules_per_kg_kelvin.is_finite()
            && self.specific_heat_capacity_joules_per_kg_kelvin > 0.0
            && self.thermal_conductivity_watts_per_meter_kelvin.is_finite()
            && self.thermal_conductivity_watts_per_meter_kelvin >= 0.0
    }
}
