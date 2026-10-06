//! Conservative face exchanges for the thermal field.

use bevy::prelude::*;

use super::{MINIMUM_TEMPERATURE_KELVIN, ThermalField};
use crate::thermal::ThermalMaterial;

const STABILITY_SAFETY_FACTOR: f32 = 0.45;

impl ThermalField {
    /// Advances isotropic Fourier conduction for `delta_seconds`.
    ///
    /// The explicit finite-volume solve automatically subdivides the requested
    /// interval to satisfy the 3D diffusion stability bound. Every face exchange
    /// is accumulated as equal-and-opposite energy deltas, preserving total
    /// energy up to floating-point roundoff.
    pub fn conduct_internal(&mut self, material: &ThermalMaterial, delta_seconds: f32) {
        if !delta_seconds.is_finite()
            || delta_seconds <= 0.0
            || material.thermal_conductivity_watts_per_meter_kelvin <= 0.0
        {
            return;
        }

        let maximum_step = self.maximum_stable_step_seconds(material);
        let substeps = if maximum_step.is_finite() && delta_seconds > maximum_step {
            (delta_seconds / maximum_step).ceil() as usize
        } else {
            1
        };
        let substep_seconds = delta_seconds / substeps as f32;

        for _ in 0..substeps {
            self.conduct_substep(material, substep_seconds);
        }
    }

    fn conduct_substep(&mut self, material: &ThermalMaterial, delta_seconds: f32) {
        let capacity = self.cell_heat_capacity_joules_per_kelvin(material);
        let cell_size = self.cell_size_meters();
        let conductivity = material.thermal_conductivity_watts_per_meter_kelvin;
        let resolution = self.resolution;

        let conductance = Vec3::new(
            conductivity * cell_size.y * cell_size.z / cell_size.x,
            conductivity * cell_size.x * cell_size.z / cell_size.y,
            conductivity * cell_size.x * cell_size.y / cell_size.z,
        );

        self.temperature_scratch.clear();
        self.temperature_scratch.extend(
            self.cell_energy_joules
                .iter()
                .map(|energy| *energy / capacity),
        );
        self.energy_delta_scratch
            .resize(self.cell_energy_joules.len(), 0.0);
        self.energy_delta_scratch.fill(0.0);

        let linear_index =
            |index: UVec3| (index.x + resolution.x * (index.y + resolution.y * index.z)) as usize;
        let temperatures = &self.temperature_scratch;
        let energy_delta = &mut self.energy_delta_scratch;

        for z in 0..resolution.z {
            for y in 0..resolution.y {
                for x in 0..resolution.x {
                    let cell = UVec3::new(x, y, z);
                    let source = linear_index(cell);

                    for (neighbor, face_conductance) in [
                        (UVec3::new(x + 1, y, z), conductance.x),
                        (UVec3::new(x, y + 1, z), conductance.y),
                        (UVec3::new(x, y, z + 1), conductance.z),
                    ] {
                        if neighbor.x >= resolution.x
                            || neighbor.y >= resolution.y
                            || neighbor.z >= resolution.z
                        {
                            continue;
                        }

                        let target = linear_index(neighbor);
                        let transferred = face_conductance
                            * (temperatures[source] - temperatures[target])
                            * delta_seconds;
                        energy_delta[source] -= transferred;
                        energy_delta[target] += transferred;
                    }
                }
            }
        }

        let minimum_energy = capacity * MINIMUM_TEMPERATURE_KELVIN;
        for (energy, delta) in self
            .cell_energy_joules
            .iter_mut()
            .zip(self.energy_delta_scratch.iter().copied())
        {
            *energy = (*energy + delta).max(minimum_energy);
        }
    }

    fn maximum_stable_step_seconds(&self, material: &ThermalMaterial) -> f32 {
        let diffusivity = material.thermal_diffusivity_square_meters_per_second();
        if diffusivity <= 0.0 || !diffusivity.is_finite() {
            return f32::INFINITY;
        }

        let cell = self.cell_size_meters();
        let inverse_square_sum =
            1.0 / (cell.x * cell.x) + 1.0 / (cell.y * cell.y) + 1.0 / (cell.z * cell.z);
        STABILITY_SAFETY_FACTOR / (diffusivity * inverse_square_sum)
    }
}
