//! Portal splitting for ordinary dynamic rigid bodies.
//!
//! One authoritative rigid body owns mass, inertia, gravity and external forces.
//! The destination-side peer is a disposable physical manifestation. Candidate
//! selection and aperture geometry are shared with the character split path;
//! this module owns only rigid-body manifestation, solver coupling and rigid
//! reconciliation.

mod character_contact;
mod peer;
mod preparation;
mod reconciliation;
mod solver;

pub(crate) use character_contact::{
    apply_peer_character_pushes, receive_peer_dynamic_contact_pushes,
};
pub(crate) use preparation::prepare_rigid_splits;
pub(crate) use reconciliation::reconcile_rigid_splits;
pub(crate) use solver::{
    couple_rigid_split_solver_peers, sync_rigid_split_solver_peers,
};
