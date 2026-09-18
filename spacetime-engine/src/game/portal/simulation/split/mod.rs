//! Shared portal-splitting mechanics and the character split adapter.
//!
//! Portal split policy is shared by every physical manifestation path:
//! candidate selection, aperture fit and center crossing do not belong to the
//! character implementation or the rigid-body implementation. Character-specific
//! lifecycle and collision materialization live under [`character`].

use bevy::prelude::*;

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

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum PortalSplitSet {
    Prepare,
    MaterializeBeforeMotor,
    Resolve,
    MaterializeAfterMotor,
}
