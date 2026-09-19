//! Aggregate thermal state and manifestation-space sampling marker.

use bevy::prelude::*;

/// Reference room temperature used by the first thermal slice.
pub const AMBIENT_TEMPERATURE_KELVIN: f32 = 293.15;

/// Aggregate thermal state for one semantic entity.
///
/// Existing gameplay systems consume this well-mixed aggregate directly.
/// Bodies that need internal gradients may additionally carry
/// [`super::super::ThermalField`] + [`super::super::ThermalMaterial`]; the thermal simulation
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
