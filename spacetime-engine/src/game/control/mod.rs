//! Semantic and local runtime control authority.
//!
//! A controller owns one semantic subject; local focus is an explicit adapter
//! over that relationship and its runtime manifestation.
//!
//! ## Integration
//!
//! Transfer requests resolve semantic control first. Runtime adapters then reconcile local subject,
//! view, and interaction focus from the resolved owner.
//!
//! ## Module map
//!
//! - `model`: Control relationships, transfer messages, and invariant reports.
//! - `runtime`: Ordered local-control transfer transaction and adapter reconciliation.
//!
//! The plugin or configure entrypoint here wires the child systems and resources.
//!

use super::GameSet;
use bevy::prelude::*;

mod model;
mod runtime;

pub use model::*;
use runtime::{
    audit_local_control_invariants, reconcile_local_control_adapters,
    refresh_controlled_interaction_requirement, resolve_local_control_transfers,
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
                resolve_local_control_transfers.in_set(ControlActionSet::Transfer),
            )
            .add_systems(
                Update,
                (
                    reconcile_local_control_adapters,
                    refresh_controlled_interaction_requirement
                        .after(reconcile_local_control_adapters),
                )
                    .in_set(ControlActionSet::Reconcile),
            )
            .add_systems(
                Update,
                audit_local_control_invariants.in_set(ControlActionSet::Validate),
            );
    }
}
