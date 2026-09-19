use crate::spatial::{SpatialScale, UsfPosition};
use bevy::prelude::Vec3;

use super::*;

#[test]
fn root_exists_before_any_finer_context_is_allocated() {
    let registry = PhenomenonRegistry::default();
    let mut store = WorldgenStore::new(5);
    let epoch = WorldgenEpoch::present_day_bootstrap();
    let target = UsfPosition::default();

    let root = store
        .bootstrap_root(target, TemporalScale::WORLDGEN_SNAPSHOT, epoch, &registry)
        .unwrap();
    assert_eq!(store.len(), 1);
    assert_eq!(root.scope().scale(), SpatialScale::MAX);

    let leaf = store
        .contextualize_to(root, target, SpatialScale::ZERO, &registry)
        .unwrap()
        .unwrap();
    assert_eq!(store.len(), 36);
    assert_eq!(leaf.scope().scale(), SpatialScale::ZERO);
}

#[test]
fn one_scale_zero_request_builds_every_nonnegative_scale() {
    let registry = PhenomenonRegistry::default();
    let mut store = WorldgenStore::new(7);
    let epoch = WorldgenEpoch::present_day_bootstrap();
    let leaf = store
        .ensure_branch(
            UsfPosition::default(),
            SpatialScale::ZERO,
            TemporalScale::WORLDGEN_SNAPSHOT,
            epoch,
            &registry,
        )
        .unwrap();

    let lineage = store.lineage(leaf);
    assert_eq!(lineage.len(), 36);
    assert_eq!(
        lineage.first().unwrap().context().spatial_scale(),
        SpatialScale::ZERO
    );
    assert_eq!(
        lineage.last().unwrap().context().spatial_scale(),
        SpatialScale::MAX
    );
    assert!(lineage.iter().all(|node| node.phenomena().len() > 0));
    assert!(
        store
            .state::<GeologyClimateHydrologyState>(leaf, GEOLOGY_CLIMATE_HYDROLOGY)
            .is_some()
    );
    assert!(store.state::<EcologyState>(leaf, ECOLOGY).is_some());
    assert!(
        store
            .state::<MaterialSubstrateState>(leaf, MATERIAL_SUBSTRATE)
            .is_some()
    );
}

#[test]
fn nearby_scale_zero_branch_reuses_all_shared_ancestors() {
    let registry = PhenomenonRegistry::default();
    let mut store = WorldgenStore::new(11);
    let epoch = WorldgenEpoch::present_day_bootstrap();
    let temporal = TemporalScale::WORLDGEN_SNAPSHOT;
    store
        .ensure_branch(
            UsfPosition::default(),
            SpatialScale::ZERO,
            temporal,
            epoch,
            &registry,
        )
        .unwrap();
    assert_eq!(store.len(), 36);

    let neighbor = UsfPosition::default()
        .translated_native(Vec3::new(1_500.0, 0.0, 0.0))
        .unwrap();
    store
        .ensure_branch(neighbor, SpatialScale::ZERO, temporal, epoch, &registry)
        .unwrap();

    // 1.5 km changes the S0 chunk while remaining in the same S+1 parent.
    assert_eq!(store.len(), 37);
}

#[test]
fn temporal_parameterizations_have_distinct_cache_identity() {
    let registry = PhenomenonRegistry::default();
    let mut store = WorldgenStore::new(13);
    let epoch = WorldgenEpoch::present_day_bootstrap();
    let target = UsfPosition::default();
    store
        .ensure_branch(
            target,
            SpatialScale::ZERO,
            TemporalScale::WORLDGEN_SNAPSHOT,
            epoch,
            &registry,
        )
        .unwrap();
    store
        .ensure_branch(
            target,
            SpatialScale::ZERO,
            TemporalScale::new(1),
            epoch,
            &registry,
        )
        .unwrap();

    assert_eq!(store.len(), 72);
}
}
