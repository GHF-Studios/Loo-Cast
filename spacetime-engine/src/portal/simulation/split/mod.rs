//! Shared portal-splitting mechanics and the character split adapter.
//!
//! Portal split policy is shared by every physical manifestation path:
//! candidate selection, aperture fit and center crossing do not belong to the
//! character implementation or the rigid-body implementation. Character-specific
//! lifecycle and collision materialization live under [`character`].

use bevy::prelude::*;

use crate::{
    ecs::{UsfAuthorityPartitionOf, UsfLogicalRealizationOf},
    portal::{PortalSplitTraveler, domain::ActivePortalSplit},
};

mod aperture;
mod candidate;
mod character;

pub(crate) use character::{
    materialize_portal_splits, prepare_portal_splits, resolve_portal_splits,
};

pub(super) use aperture::{box_fits_aperture_at, center_crossing_fraction};
pub(super) use candidate::{
    active_pair_is_valid, box_reaches_portal_this_tick, find_split_candidate,
};

/// Distance beyond the trailing support radius before an active split collapses.
pub(super) const CLEAR_MARGIN: f32 = 0.02;

pub(super) fn activate_split_partition(
    commands: &mut Commands,
    primary: &UsfLogicalRealizationOf,
    partitions: &Query<&UsfAuthorityPartitionOf>,
    peer: Entity,
    source: Entity,
    destination: Entity,
) -> Option<ActivePortalSplit> {
    let semantic = partitions.get(primary.0).ok()?.0;
    let partition = commands
        .spawn((
            Name::new("Portal Authority Partition"),
            UsfAuthorityPartitionOf(semantic),
        ))
        .id();

    commands
        .entity(peer)
        .insert(UsfLogicalRealizationOf(partition));

    Some(ActivePortalSplit {
        source,
        destination,
        partition,
    })
}

pub(in crate::portal) fn retire_split_partition(
    commands: &mut Commands,
    split: &mut PortalSplitTraveler,
) -> Option<ActivePortalSplit> {
    let active = split.active.take()?;
    commands
        .entity(split.peer())
        .remove::<UsfLogicalRealizationOf>();
    commands.entity(active.partition).despawn();
    Some(active)
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum PortalSplitSet {
    Prepare,
    MaterializeBeforeMotor,
    Resolve,
    MaterializeAfterMotor,
}
