//! Shared ECS identity, manifestation, constituency, and component-conflict facilities.
//!
//! ## Module map
//!
//! - `component_conflict`: Register and report incompatible ECS component combinations.
//! - `constituency`: Recursive semantic entity constituency.
//! - `devtools`: USF manifestation developer visualization.
//! - `manifestation`: USF semantic-entity manifestation primitives.
//!
//! Reexports here define the supported surface; child modules hold its implementation.
//!

pub mod component_conflict;
pub mod constituency;
pub(crate) mod devtools;
pub mod manifestation;

pub use constituency::{UsfConstituentOf, UsfConstituents};
pub use manifestation::{
    UsfAuthorityPartitionOf, UsfAuthorityPartitions, UsfEntity, UsfLogicalRealizationOf,
    UsfLogicalRealizations, UsfOwnershipQuery, UsfPresentationProjectionOf,
    UsfPresentationProjections, UsfPresentationView, UsfPresentationViewOf, UsfViewPresentations,
};
