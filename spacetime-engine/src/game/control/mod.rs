//! Semantic and local runtime control authority.
//!
//! A controller owns one semantic subject; local focus is an explicit adapter
//! over that relationship and its runtime manifestation.

use super::GameSet;
use bevy::prelude::*;

mod model;
mod runtime;

pub use model::*;
use runtime::{
    apply_local_control_transfers, audit_local_control_invariants, reconcile_local_control_focus,
    sync_controlled_interaction_scale_affinity,
};

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LocalControlAudit>()
            .register_type::<LocalController>()
            .register_type::<LocalControlSubject>()
            .register_type::<LocalViewTarget>()
            .add_message::<LocalControlTransferRequest>()
            .add_message::<LocalControlTransferApplied>()
            .add_message::<LocalControlTransferRejected>()
            .configure_sets(
                Update,
                (
                    ControlActionSet::Request,
                    ControlActionSet::Transfer,
                    ControlActionSet::Reconcile,
                    ControlActionSet::Validate,
                )
                    .chain()
                    .in_set(GameSet::Action),
            )
            .add_systems(
                Update,
                apply_local_control_transfers.in_set(ControlActionSet::Transfer),
            )
            .add_systems(
                Update,
                (
                    reconcile_local_control_focus,
                    sync_controlled_interaction_scale_affinity.after(reconcile_local_control_focus),
                )
                    .in_set(ControlActionSet::Reconcile),
            )
            .add_systems(
                Update,
                audit_local_control_invariants.in_set(ControlActionSet::Validate),
            );
    }
}
