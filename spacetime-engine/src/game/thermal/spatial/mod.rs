//! Spatial thermal refinement for finite solid bodies.
//!
//! [`ThermalBody`] remains the semantic aggregate state used by existing game
//! systems. A [`ThermalField`] refines that state into local finite-volume cells
//! whose conserved energy is redistributed by Fourier conduction. This keeps the
//! first spatial slice compatible with existing combustion/injury code without
//! making ECS entities out of individual thermal cells.

use bevy::prelude::*;

use crate::ecs::UsfManifestationOf;

use super::{AMBIENT_TEMPERATURE_KELVIN, ThermalBody, ThermalSet};

const MINIMUM_TEMPERATURE_KELVIN: f32 = 1.0;
const CONDUCTION_STEP_SECONDS: f32 = 1.0 / 60.0;
const MAX_ACCUMULATED_SECONDS: f32 = 0.25;
const STABILITY_SAFETY_FACTOR: f32 = 0.45;

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

    fn is_valid(&self) -> bool {
        self.density_kg_per_cubic_meter.is_finite()
            && self.density_kg_per_cubic_meter > 0.0
            && self.specific_heat_capacity_joules_per_kg_kelvin.is_finite()
            && self.specific_heat_capacity_joules_per_kg_kelvin > 0.0
            && self.thermal_conductivity_watts_per_meter_kelvin.is_finite()
            && self.thermal_conductivity_watts_per_meter_kelvin >= 0.0
    }
}

/// One diagnostic sample from a [`ThermalField`].
#[derive(Debug, Clone, Copy)]
pub struct ThermalCellSample {
    pub index: UVec3,
    pub local_center: Vec3,
    pub energy_joules: f32,
    pub temperature_kelvin: f32,
}

/// Compact local finite-volume temperature refinement for one semantic body.
///
/// Cells store sensible thermal energy in joules. Temperature is derived from
/// material density, specific heat and cell volume. Internal conduction only
/// moves energy between cells, so total field energy is conserved apart from
/// explicit external transfers.
#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct ThermalField {
    size_meters: Vec3,
    resolution: UVec3,
    cell_energy_joules: Vec<f32>,
    #[reflect(ignore)]
    temperature_scratch: Vec<f32>,
    #[reflect(ignore)]
    energy_delta_scratch: Vec<f32>,
}

impl ThermalField {
    pub fn uniform_box(
        size_meters: Vec3,
        resolution: UVec3,
        temperature_kelvin: f32,
        material: &ThermalMaterial,
    ) -> Self {
        assert!(size_meters.is_finite() && size_meters.min_element() > 0.0);
        assert!(resolution.min_element() > 0);
        assert!(temperature_kelvin.is_finite() && temperature_kelvin >= MINIMUM_TEMPERATURE_KELVIN);
        assert!(material.is_valid());

        let cell_count = resolution.x as usize * resolution.y as usize * resolution.z as usize;
        let cell_size = size_meters / resolution.as_vec3();
        let cell_volume = cell_size.x * cell_size.y * cell_size.z;
        let cell_capacity = material.density_kg_per_cubic_meter
            * cell_volume
            * material.specific_heat_capacity_joules_per_kg_kelvin;

        Self {
            size_meters,
            resolution,
            cell_energy_joules: vec![cell_capacity * temperature_kelvin; cell_count],
            temperature_scratch: Vec::with_capacity(cell_count),
            energy_delta_scratch: vec![0.0; cell_count],
        }
    }

    pub fn ambient_box(size_meters: Vec3, resolution: UVec3, material: &ThermalMaterial) -> Self {
        Self::uniform_box(
            size_meters,
            resolution,
            AMBIENT_TEMPERATURE_KELVIN,
            material,
        )
    }

    pub fn size_meters(&self) -> Vec3 {
        self.size_meters
    }

    pub fn resolution(&self) -> UVec3 {
        self.resolution
    }

    pub fn cell_size_meters(&self) -> Vec3 {
        self.size_meters / self.resolution.as_vec3()
    }

    pub fn total_energy_joules(&self) -> f32 {
        self.cell_energy_joules.iter().copied().sum()
    }

    pub fn total_heat_capacity_joules_per_kelvin(&self, material: &ThermalMaterial) -> f32 {
        self.cell_heat_capacity_joules_per_kelvin(material) * self.cell_energy_joules.len() as f32
    }

    pub fn average_temperature_kelvin(&self, material: &ThermalMaterial) -> f32 {
        self.total_energy_joules() / self.total_heat_capacity_joules_per_kelvin(material)
    }

    pub fn minimum_temperature_kelvin(&self, material: &ThermalMaterial) -> f32 {
        let capacity = self.cell_heat_capacity_joules_per_kelvin(material);
        self.cell_energy_joules
            .iter()
            .map(|energy| *energy / capacity)
            .fold(f32::INFINITY, f32::min)
    }

    pub fn maximum_temperature_kelvin(&self, material: &ThermalMaterial) -> f32 {
        let capacity = self.cell_heat_capacity_joules_per_kelvin(material);
        self.cell_energy_joules
            .iter()
            .map(|energy| *energy / capacity)
            .fold(f32::NEG_INFINITY, f32::max)
    }

    pub fn cell_temperature_kelvin(&self, index: UVec3, material: &ThermalMaterial) -> Option<f32> {
        self.linear_index(index).map(|linear| {
            self.cell_energy_joules[linear] / self.cell_heat_capacity_joules_per_kelvin(material)
        })
    }

    pub fn cell_samples<'a>(
        &'a self,
        material: &'a ThermalMaterial,
    ) -> impl Iterator<Item = ThermalCellSample> + 'a {
        let capacity = self.cell_heat_capacity_joules_per_kelvin(material);
        self.cell_energy_joules
            .iter()
            .copied()
            .enumerate()
            .map(move |(linear, energy_joules)| {
                let index = self.coordinates(linear);
                ThermalCellSample {
                    index,
                    local_center: self.local_cell_center(index),
                    energy_joules,
                    temperature_kelvin: energy_joules / capacity,
                }
            })
    }

    /// Applies external energy at one local-space point and returns the energy
    /// actually accepted after the absolute-temperature floor is respected.
    pub fn add_energy_at_local_position(
        &mut self,
        material: &ThermalMaterial,
        local_position: Vec3,
        energy_joules: f32,
    ) -> f32 {
        if !energy_joules.is_finite() {
            return 0.0;
        }
        let index = self.cell_at_local_position(local_position);
        self.add_energy_to_cell(material, index, energy_joules)
    }

    /// Applies energy evenly to all cells. This is used to reconcile older
    /// lumped-only thermal mechanisms with the spatial refinement.
    pub fn add_energy_uniform(&mut self, material: &ThermalMaterial, energy_joules: f32) -> f32 {
        if !energy_joules.is_finite() || energy_joules == 0.0 {
            return 0.0;
        }

        let per_cell = energy_joules / self.cell_energy_joules.len() as f32;
        let capacity = self.cell_heat_capacity_joules_per_kelvin(material);
        let minimum_energy = capacity * MINIMUM_TEMPERATURE_KELVIN;
        let mut applied = 0.0;

        for cell in &mut self.cell_energy_joules {
            let before = *cell;
            *cell = (*cell + per_cell).max(minimum_energy);
            applied += *cell - before;
        }

        applied
    }

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

    fn cell_volume_cubic_meters(&self) -> f32 {
        let size = self.cell_size_meters();
        size.x * size.y * size.z
    }

    fn cell_heat_capacity_joules_per_kelvin(&self, material: &ThermalMaterial) -> f32 {
        material.density_kg_per_cubic_meter
            * self.cell_volume_cubic_meters()
            * material.specific_heat_capacity_joules_per_kg_kelvin
    }

    fn add_energy_to_cell(
        &mut self,
        material: &ThermalMaterial,
        index: UVec3,
        energy_joules: f32,
    ) -> f32 {
        let linear = self.linear_index_unchecked(index);
        let capacity = self.cell_heat_capacity_joules_per_kelvin(material);
        let minimum_energy = capacity * MINIMUM_TEMPERATURE_KELVIN;
        let before = self.cell_energy_joules[linear];
        self.cell_energy_joules[linear] = (before + energy_joules).max(minimum_energy);
        self.cell_energy_joules[linear] - before
    }

    fn cell_at_local_position(&self, local_position: Vec3) -> UVec3 {
        let epsilon = 1.0e-6;
        let uv = (local_position / self.size_meters + Vec3::splat(0.5))
            .clamp(Vec3::ZERO, Vec3::splat(1.0 - epsilon));
        let scaled = (uv * self.resolution.as_vec3()).floor();
        UVec3::new(scaled.x as u32, scaled.y as u32, scaled.z as u32)
            .min(self.resolution - UVec3::ONE)
    }

    fn local_cell_center(&self, index: UVec3) -> Vec3 {
        let uv = (index.as_vec3() + Vec3::splat(0.5)) / self.resolution.as_vec3();
        (uv - Vec3::splat(0.5)) * self.size_meters
    }

    fn linear_index(&self, index: UVec3) -> Option<usize> {
        (index.x < self.resolution.x && index.y < self.resolution.y && index.z < self.resolution.z)
            .then(|| self.linear_index_unchecked(index))
    }

    fn linear_index_unchecked(&self, index: UVec3) -> usize {
        (index.x + self.resolution.x * (index.y + self.resolution.y * index.z)) as usize
    }

    fn coordinates(&self, linear: usize) -> UVec3 {
        let x_count = self.resolution.x as usize;
        let y_count = self.resolution.y as usize;
        let x = linear % x_count;
        let yz = linear / x_count;
        let y = yz % y_count;
        let z = yz / y_count;
        UVec3::new(x as u32, y as u32, z as u32)
    }
}

/// Localized external thermal-energy transfer.
///
/// `target` is normally a spatial manifestation. The simulation resolves its
/// semantic thermal state while retaining the hit point in manifestation-local
/// space so only the struck cell receives the impulse.
#[derive(Message, Debug, Clone, Copy)]
pub struct ThermalPointImpulse {
    pub target: Entity,
    pub world_position: Vec3,
    pub energy_joules: f32,
}

#[derive(Resource, Debug, Default)]
struct ThermalConductionClock {
    accumulator_seconds: f32,
}

pub(super) fn configure(app: &mut App) {
    app.add_message::<ThermalPointImpulse>()
        .register_type::<ThermalMaterial>()
        .register_type::<ThermalField>()
        .init_resource::<ThermalConductionClock>()
        .add_systems(
            Update,
            (apply_point_impulses, advance_internal_conduction)
                .chain()
                .in_set(ThermalSet::SpatialInput),
        )
        .add_systems(
            Update,
            reconcile_aggregate_energy.in_set(ThermalSet::SpatialOutput),
        );
}

fn apply_point_impulses(
    mut impulses: MessageReader<ThermalPointImpulse>,
    manifestations: Query<(&UsfManifestationOf, &Transform)>,
    mut bodies: Query<(&mut ThermalBody, &ThermalMaterial, &mut ThermalField)>,
) {
    for impulse in impulses.read() {
        let (semantic, local_position) =
            if let Ok((relation, transform)) = manifestations.get(impulse.target) {
                (
                    relation.0,
                    world_to_local_point(transform, impulse.world_position),
                )
            } else {
                (impulse.target, Vec3::ZERO)
            };

        let Ok((mut body, material, mut field)) = bodies.get_mut(semantic) else {
            continue;
        };

        let applied =
            field.add_energy_at_local_position(material, local_position, impulse.energy_joules);
        body.add_energy_joules(applied);
    }
}

fn advance_internal_conduction(
    time: Res<Time>,
    mut clock: ResMut<ThermalConductionClock>,
    mut fields: Query<(&ThermalMaterial, &mut ThermalField)>,
) {
    let elapsed = time.delta_secs().max(0.0).min(MAX_ACCUMULATED_SECONDS);
    clock.accumulator_seconds = (clock.accumulator_seconds + elapsed).min(MAX_ACCUMULATED_SECONDS);

    let steps = (clock.accumulator_seconds / CONDUCTION_STEP_SECONDS).floor() as usize;
    if steps == 0 {
        return;
    }

    for _ in 0..steps {
        for (material, mut field) in &mut fields {
            field.conduct_internal(material, CONDUCTION_STEP_SECONDS);
        }
    }

    clock.accumulator_seconds -= steps as f32 * CONDUCTION_STEP_SECONDS;
}

/// Existing thermal mechanisms still operate on the aggregate `ThermalBody`.
/// Any net aggregate energy change is projected uniformly back into the spatial
/// refinement after those systems run. Localized mechanisms should write the
/// field directly so their spatial information is retained.
fn reconcile_aggregate_energy(
    mut fields: Query<(&ThermalBody, &ThermalMaterial, &mut ThermalField)>,
) {
    for (body, material, mut field) in &mut fields {
        let desired_energy =
            field.total_heat_capacity_joules_per_kelvin(material) * body.temperature_kelvin();
        let difference = desired_energy - field.total_energy_joules();
        let tolerance = desired_energy.abs().max(1.0) * 1.0e-5;
        if difference.abs() > tolerance {
            field.add_energy_uniform(material, difference);
        }
    }
}

fn world_to_local_point(transform: &Transform, world_position: Vec3) -> Vec3 {
    let translated = world_position - transform.translation;
    let unrotated = transform.rotation.inverse() * translated;
    let scale = Vec3::new(
        nonzero_scale(transform.scale.x),
        nonzero_scale(transform.scale.y),
        nonzero_scale(transform.scale.z),
    );
    unrotated / scale
}

fn nonzero_scale(value: f32) -> f32 {
    if value.abs() > f32::EPSILON {
        value
    } else {
        1.0
    }
}

#[cfg(test)]
mod tests {
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
}
