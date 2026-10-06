//! Combustion heat transfer from disposable spatial samples to semantic bodies.

use super::*;

#[derive(Default)]
pub(super) struct HeatScratch {
    positions_by_semantic: HashMap<Entity, Vec<DVec3>>,
    weights: Vec<(Entity, f32)>,
    energy_by_target: HashMap<Entity, f32>,
}

impl HeatScratch {
    fn accumulate_sources(
        &mut self,
        sources: &Query<(Entity, &Combustion, &CombustibleMaterial)>,
        dt: f32,
    ) {
        self.energy_by_target.clear();
        for (source, combustion, material) in sources {
            let coupling = super::super::coupling::combustion_heat_coupling(combustion, material);
            *self.energy_by_target.entry(source).or_default() +=
                coupling.self_heating_power_watts * dt;

            let Some(source_positions) = self.positions_by_semantic.get(&source) else {
                continue;
            };
            distribute_environmental_heat(
                source,
                source_positions,
                &self.positions_by_semantic,
                coupling.radius_meters,
                coupling.environmental_power_watts * dt,
                &mut self.weights,
                &mut self.energy_by_target,
            );
        }
    }
}

/// Transfers combustion heat through registered manifestation-space samples.
///
/// This deliberately uses an O(n²) semantic-entity pass in the first slice.
/// The *interaction contract* is the important part; large worlds can replace
/// candidate discovery with chunk/spatial indexing without changing thermal
/// state or combustion semantics.
pub(super) fn propagate_combustion_heat(
    time: Res<Time>,
    runtime_ownership: UsfRuntimeOwnershipQuery,
    layers: Query<&UsfScaleLayer>,
    anchor: Single<&UsfScaleLayer, With<UsfSpatialAnchor>>,
    samples: Query<
        (Entity, &Transform),
        (
            With<ThermalSpatialSample>,
            Or<(Without<SpatialSplitPeer>, With<SpatialSplitPeerActive>)>,
        ),
    >,
    sources: Query<(Entity, &Combustion, &CombustibleMaterial)>,
    mut targets: Query<&mut ThermalBody>,
    mut scratch: Local<HeatScratch>,
) {
    let dt = time.delta_secs().max(0.0);
    if dt <= 0.0 || sources.is_empty() {
        return;
    }

    for positions in scratch.positions_by_semantic.values_mut() {
        positions.clear();
    }
    for (runtime, transform) in &samples {
        let Some(semantic) = runtime_ownership.semantic_of(runtime) else {
            continue;
        };
        let scale = runtime_ownership.scale_of(runtime, &layers, anchor.scale());
        let position_metres = transform.translation.as_dvec3() * scale.metres_per_native();
        if !position_metres.is_finite() {
            continue;
        }
        scratch
            .positions_by_semantic
            .entry(semantic)
            .or_default()
            .push(position_metres);
    }
    scratch
        .positions_by_semantic
        .retain(|_, positions| !positions.is_empty());
    scratch.accumulate_sources(&sources, dt);

    for (&target, &energy_joules) in &scratch.energy_by_target {
        if let Ok(mut thermal) = targets.get_mut(target) {
            thermal.add_energy_joules(energy_joules);
        }
    }
}

/// Spatial samples are disposable interaction evidence. Energy is accumulated
/// per semantic target and only applied to authoritative bodies by the caller.
fn distribute_environmental_heat(
    source: Entity,
    source_positions: &[DVec3],
    positions_by_semantic: &HashMap<Entity, Vec<DVec3>>,
    radius_meters: f32,
    environmental_energy: f32,
    weights: &mut Vec<(Entity, f32)>,
    energy_by_target: &mut HashMap<Entity, f32>,
) {
    if radius_meters <= 0.0 || environmental_energy <= 0.0 {
        return;
    }
    weights.clear();
    let mut total_weight = 0.0;
    let radius_squared = f64::from(radius_meters).powi(2);
    for (&target, target_positions) in positions_by_semantic {
        if target == source {
            continue;
        }
        let distance_squared = source_positions
            .iter()
            .flat_map(|source_position| {
                target_positions
                    .iter()
                    .map(move |target_position| source_position.distance_squared(*target_position))
            })
            .fold(f64::INFINITY, f64::min);
        if distance_squared >= radius_squared {
            continue;
        }
        let weight = super::super::coupling::radial_heat_weight(
            distance_squared.sqrt() as f32,
            radius_meters,
        );
        if weight > 0.0 {
            weights.push((target, weight));
            total_weight += weight;
        }
    }
    let normalization = total_weight.max(1.0);
    for &(target, weight) in weights.iter() {
        *energy_by_target.entry(target).or_default() +=
            environmental_energy * weight / normalization;
    }
}
