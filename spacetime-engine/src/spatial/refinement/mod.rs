//! Current-relative multiscale refinement planning.
//!
//! Refinement is runtime policy over canonical topology. It derives the
//! ancestor-first branch and bounded working-set taper requested by capability
//! planners. Realized capability state and coverage live in `spatial::capability`.
//!
//! ## Module map
//!
//! - `demand`: Controlled request for additional capability refinement.
//! - `plan`: Current-relative multiscale refinement planning.
//!
//! Reexports here define the supported surface; child modules hold its implementation.
//!

mod demand;
mod plan;

pub use demand::SpatialRefinementDemand;
pub use plan::{UsfRefinementPlan, UsfRefinementStep};
