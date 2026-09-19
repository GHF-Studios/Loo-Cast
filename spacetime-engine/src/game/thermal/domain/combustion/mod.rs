//! Combustible material behavior, consumable fuel and derived combustion state.

use bevy::prelude::*;

/// Material behavior required for self-sustaining combustion.
///
/// This describes *how* available fuel burns. The remaining amount of fuel is
/// separate [`Fuel`] state so material properties and consumable state do not
/// become one giant catch-all component.
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct CombustibleMaterial {
    pub ignition_temperature_kelvin: f32,
    pub extinction_temperature_kelvin: f32,
    pub burn_power_watts: f32,
    pub self_heating_fraction: f32,
    pub environmental_transfer_fraction: f32,
    pub heat_transfer_radius_meters: f32,
}

impl CombustibleMaterial {
    /// A deliberately game-visible wood-like preset for the first vertical
    /// slice. It is qualitative, not a claim of measured material fidelity.
    pub fn wood_like() -> Self {
        Self {
            ignition_temperature_kelvin: 430.0,
            extinction_temperature_kelvin: 370.0,
            burn_power_watts: 45_000.0,
            self_heating_fraction: 0.18,
            environmental_transfer_fraction: 0.50,
            heat_transfer_radius_meters: 3.5,
        }
    }
}

/// Chemical energy still available to combustion.
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct Fuel {
    remaining_energy_joules: f32,
}

impl Fuel {
    pub fn new(energy_joules: f32) -> Self {
        assert!(energy_joules.is_finite() && energy_joules >= 0.0);
        Self {
            remaining_energy_joules: energy_joules,
        }
    }

    pub fn remaining_energy_joules(&self) -> f32 {
        self.remaining_energy_joules
    }

    pub(crate) fn consume(&mut self, requested_joules: f32) -> f32 {
        if requested_joules <= 0.0 || !requested_joules.is_finite() {
            return 0.0;
        }

        let consumed = self.remaining_energy_joules.min(requested_joules);
        self.remaining_energy_joules -= consumed;
        consumed
    }
}

/// Runtime fact that a semantic entity is currently combusting.
///
/// This component is derived from thermal state + combustible material + fuel.
/// Presentation and propagation observe it; tools never insert it directly.
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct Combustion {
    power_watts: f32,
}

impl Combustion {
    pub fn power_watts(&self) -> f32 {
        self.power_watts
    }

    pub(crate) fn new(power_watts: f32) -> Self {
        Self {
            power_watts: power_watts.max(0.0),
        }
    }
}
