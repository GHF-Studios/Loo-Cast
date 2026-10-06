//! ECS ordering for request collection and residency reconciliation.

use super::{UsfContextResidency, UsfResidencyRequestBuffer};
use crate::spatial::{SpatialDemandSet, SpatialDemandSnapshot};
use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UsfResidencySet {
    Reset,
    Collect,
    Reconcile,
}

fn clear_residency_requests(mut demand: ResMut<UsfResidencyRequestBuffer>) {
    demand.clear();
}

fn collect_spatial_residency_requests(
    spatial: Res<SpatialDemandSnapshot>,
    mut demand: ResMut<UsfResidencyRequestBuffer>,
) {
    for scope in spatial.iter() {
        demand.request(scope);
    }
}

fn reconcile_context_residency(
    demand: Res<UsfResidencyRequestBuffer>,
    mut residency: ResMut<UsfContextResidency>,
) {
    if let Err(error) = residency.reconcile_from_scopes(demand.iter()) {
        error!(
            ?error,
            "USF residency demand could not be represented canonically; retaining previous residency"
        );
    }
}

pub(in crate::spatial) fn configure(app: &mut App) {
    app.init_resource::<UsfResidencyRequestBuffer>()
        .init_resource::<UsfContextResidency>()
        .configure_sets(
            Update,
            UsfResidencySet::Reset.after(SpatialDemandSet::Collect),
        )
        .configure_sets(
            Update,
            UsfResidencySet::Collect.after(UsfResidencySet::Reset),
        )
        .configure_sets(
            Update,
            UsfResidencySet::Reconcile.after(UsfResidencySet::Collect),
        )
        .add_systems(
            Update,
            clear_residency_requests.in_set(UsfResidencySet::Reset),
        )
        .add_systems(
            Update,
            collect_spatial_residency_requests.in_set(UsfResidencySet::Collect),
        )
        .add_systems(
            Update,
            reconcile_context_residency.in_set(UsfResidencySet::Reconcile),
        );
}
