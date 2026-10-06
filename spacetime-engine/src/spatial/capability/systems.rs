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
    publications: Query<(Entity, &UsfCapabilityCoveragePublication)>,
    changed: Query<(), Changed<UsfCapabilityRealization>>,
    changed_publications: Query<(), Changed<UsfCapabilityCoveragePublication>>,
    mut removed: RemovedComponents<UsfCapabilityRealization>,
    mut removed_publications: RemovedComponents<UsfCapabilityCoveragePublication>,
    mut snapshot: ResMut<UsfScaleCoverageSnapshot>,
) {
    let changed_any = changed.iter().next().is_some();
    let changed_publication = changed_publications.iter().next().is_some();
    let removed_any = removed.read().next().is_some();
    let removed_publication = removed_publications.read().next().is_some();
    if !changed_any && !changed_publication && !removed_any && !removed_publication {
        return;
    }

    let _span = bevy::log::info_span!("usf_capability.rebuild_snapshot").entered();
    let published_count = publications
        .iter()
        .map(|(_, batch)| batch.facts().len())
        .sum::<usize>();
    let mut next = Vec::with_capacity(realizations.iter().len() + published_count);

    for (entity, realization) in &realizations {
        if let Some(coverage) = realization.coverage(entity) {
            next.push(coverage);
        }
    }

    for (producer, batch) in &publications {
        for &fact in batch.facts() {
            if let Some(coverage) = fact.coverage(producer) {
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
