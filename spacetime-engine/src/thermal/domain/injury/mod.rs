//! Biological-style response policy for dangerous body temperature.

use bevy::prelude::*;

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
