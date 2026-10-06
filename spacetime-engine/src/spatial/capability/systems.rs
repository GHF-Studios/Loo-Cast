//! ECS publication of the persistent coverage snapshot.

use super::*;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UsfCapabilitySet {
    /// Capability subsystems publish/update live realization components here.
    Publish,
    /// Spatial runtime rebuilds persistent coverage from those live facts.
    ReconcileCoverage,
}

fn reconcile_capability_coverage(
    realizations: Query<(Entity, &UsfCapabilityRealization)>,
    batches: Query<(Entity, &UsfCapabilityCoverageBatch)>,
    changed: Query<(), Changed<UsfCapabilityRealization>>,
    changed_batches: Query<(), Changed<UsfCapabilityCoverageBatch>>,
    mut removed: RemovedComponents<UsfCapabilityRealization>,
    mut removed_batches: RemovedComponents<UsfCapabilityCoverageBatch>,
    mut snapshot: ResMut<UsfScaleCoverageSnapshot>,
) {
    let changed_any = changed.iter().next().is_some();
    let changed_batch = changed_batches.iter().next().is_some();
    let removed_any = removed.read().next().is_some();
    let removed_batch = removed_batches.read().next().is_some();
    if !changed_any && !changed_batch && !removed_any && !removed_batch {
        return;
    }

    let _span = bevy::log::info_span!("usf_capability.rebuild_snapshot").entered();
    let batched_count = batches
        .iter()
        .map(|(_, batch)| batch.records().len())
        .sum::<usize>();
    let mut next = Vec::with_capacity(realizations.iter().len() + batched_count);

    for (entity, realization) in &realizations {
        if let Some(coverage) = realization.coverage(entity) {
            next.push(coverage);
        }
    }

    for (producer, batch) in &batches {
        for &record in batch.records() {
            if let Some(coverage) = record.coverage(producer) {
                next.push(coverage);
            }
        }
    }

    snapshot.reconcile(next);
}

pub(in crate::spatial) fn configure(app: &mut App) {
    app.init_resource::<UsfScaleCoverageSnapshot>()
        .configure_sets(
            PostUpdate,
            (
                UsfCapabilitySet::Publish,
                UsfCapabilitySet::ReconcileCoverage,
            )
                .chain()
                .before(UsfSpatialSet::SyncSemantic),
        )
        .add_systems(
            PostUpdate,
            reconcile_capability_coverage.in_set(UsfCapabilitySet::ReconcileCoverage),
        );
}
