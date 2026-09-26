//! Current-relative multiscale refinement planning.
//!
//! Refinement is runtime policy over canonical topology. It derives the
//! ancestor-first branch and bounded working-set taper requested by capability
//! planners. Realized capability state and coverage live in `spatial::capability`.

mod plan;

pub use plan::{UsfRefinementPlan, UsfRefinementStep};
