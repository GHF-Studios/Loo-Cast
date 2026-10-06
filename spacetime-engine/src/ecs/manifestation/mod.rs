//! USF semantic-entity manifestation primitives.
//!
//! # Ownership graph
//!
//! Semantic identity, topological authority, scale-local simulation, and
//! presentation are separate runtime dimensions:
//!
//! ```text
//! UsfEntity
//!   -> authority partition(s)
//!       -> logical realization(s)
//!           -> presentation projection(s)
//! ```
//!
//! [`UsfEntity`] is the single semantic identity for an object.
//!
//! An entity carrying [`UsfAuthorityPartitionOf`] is one authoritative
//! topological portion of exactly one semantic entity. Partitions are peers;
//! there is no permanent "original" partition.
//!
//! An entity carrying [`UsfLogicalRealizationOf`] is one scale/backend-local
//! simulation realization of exactly one authority partition. Realizations are
//! disposable runtime representations: rebuilding one must not change semantic
//! or partition identity.
//!
//! An entity carrying [`UsfPresentationProjectionOf`] is presentation-only.
//! New generic-graph users target a logical realization. [`UsfPresentationViewOf`]
//! independently identifies the view that owns observer-specific presentation
//! state. Presentation lifetime and viewer count never manufacture semantic or
//! simulation authority.
//!
//! The relationship components themselves identify authority partitions and
//! logical realizations. Separate marker components would duplicate role state
//! and could drift out of sync with the ownership relationship.
//!
//! # Lifetime ownership
//!
//! - A semantic entity owns its authority partitions.
//! - An authority partition owns its logical realizations.
//! - Destroying a realization does not destroy its partition.
//! - Destroying a partition does not destroy its semantic entity or siblings.
//! - Presentation is not linked-spawn-owned by the logical realization.
//!
mod components;
mod query;

pub use components::*;
pub use query::UsfOwnershipQuery;
