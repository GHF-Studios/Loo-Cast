//! Thermal/combustion domain state.
//!
//! The components in this module deliberately describe *state and material
//! response*, not presentation. A flame mesh is an observation of combustion;
//! it is never the authority that something is burning.

use bevy::prelude::*;

/// Reference room temperature used by the first thermal slice.
pub const AMBIENT_TEMPERATURE_KELVIN: f32 = 293.15;

/// Aggregate thermal state for one semantic entity.
///
/// Existing gameplay systems consume this well-mixed aggregate directly.
/// Bodies that need internal gradients may additionally carry
/// [`super::ThermalField`] + [`super::ThermalMaterial`]; the thermal simulation
/// keeps that spatial refinement reconciled with this aggregate state.
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct ThermalBody {
    temperature_kelvin: f32,
    pub heat_capacity_joules_per_kelvin: f32,
    pub cooling_watts_per_kelvin: f32,
}

impl ThermalBody {
    pub fn new(
        temperature_kelvin: f32,
        heat_capacity_joules_per_kelvin: f32,
        cooling_watts_per_kelvin: f32,
    ) -> Self {
        assert!(temperature_kelvin.is_finite() && temperature_kelvin > 0.0);
        assert!(
            heat_capacity_joules_per_kelvin.is_finite() && heat_capacity_joules_per_kelvin > 0.0
        );
        assert!(cooling_watts_per_kelvin.is_finite() && cooling_watts_per_kelvin >= 0.0);

        Self {
            temperature_kelvin,
            heat_capacity_joules_per_kelvin,
            cooling_watts_per_kelvin,
        }
    }

    pub fn ambient(heat_capacity_joules_per_kelvin: f32, cooling_watts_per_kelvin: f32) -> Self {
        Self::new(
            AMBIENT_TEMPERATURE_KELVIN,
            heat_capacity_joules_per_kelvin,
            cooling_watts_per_kelvin,
        )
    }

    pub fn temperature_kelvin(&self) -> f32 {
        self.temperature_kelvin
    }

    pub(crate) fn add_energy_joules(&mut self, energy_joules: f32) {
        if !energy_joules.is_finite() {
            return;
        }

        self.temperature_kelvin = (self.temperature_kelvin
            + energy_joules / self.heat_capacity_joules_per_kelvin)
            .max(1.0);
    }

    pub(crate) fn exchange_with_ambient(&mut self, delta_seconds: f32) {
        if delta_seconds <= 0.0 || self.cooling_watts_per_kelvin <= 0.0 {
            return;
        }

        let delta_kelvin = self.temperature_kelvin - AMBIENT_TEMPERATURE_KELVIN;
        if delta_kelvin.abs() <= f32::EPSILON {
            return;
        }

        let requested_energy = -delta_kelvin * self.cooling_watts_per_kelvin * delta_seconds;
        let energy_to_ambient = -delta_kelvin * self.heat_capacity_joules_per_kelvin;
        let applied = if requested_energy.signum() == energy_to_ambient.signum()
            && requested_energy.abs() > energy_to_ambient.abs()
        {
            energy_to_ambient
        } else {
            requested_energy
        };

        self.add_energy_joules(applied);
    }
}

/// Marks a concrete manifestation whose transform participates in spatial
/// thermal interactions for its semantic entity.
///
/// Semantic thermal state can have many visual/physical manifestations, but not
/// every reserved manifestation is necessarily spatially active. Explicit
/// samples keep propagation independent from portal-specific activity rules.
#[derive(Component, Reflect, Debug, Default, Clone, Copy)]
#[reflect(Component)]
pub struct ThermalSpatialSample;

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

/// Biological-style response to dangerous body temperature.
///
/// This is intentionally separate from [`CombustibleMaterial`]. A living body
/// can suffer thermal injury without itself being modeled as a lump of fuel,
/// while an inanimate fuel object can burn without having biological Health.
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct ThermalInjury {
    pub damage_threshold_kelvin: f32,
    pub damage_per_second_per_kelvin: f32,
}

impl ThermalInjury {
    pub fn human_like() -> Self {
        Self {
            damage_threshold_kelvin: 318.15,
            damage_per_second_per_kelvin: 0.075,
        }
    }
}

/// Instant signed thermal energy transfer into one entity.
///
/// Positive values heat; negative values cool. The target may be either a
/// semantic entity or one of its manifestations; simulation canonicalizes it.
#[derive(Message, Debug, Clone, Copy)]
pub struct ThermalImpulse {
    pub target: Entity,
    pub energy_joules: f32,
}

#[cfg(test)]
mod tests {
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
