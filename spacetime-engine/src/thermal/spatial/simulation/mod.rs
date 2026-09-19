//! ECS adapter for localized thermal input and aggregate-state reconciliation.

use bevy::prelude::*;

use crate::ecs::UsfManifestationOf;

use super::{ThermalField, ThermalMaterial};
use super::super::{ThermalBody, ThermalSet};

const CONDUCTION_STEP_SECONDS: f32 = 1.0 / 60.0;
const MAX_ACCUMULATED_SECONDS: f32 = 0.25;

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

pub(in crate::thermal) fn configure(app: &mut App) {
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
