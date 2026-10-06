//! Balanced binary frontier planning from semantic field evidence.
//!
//! ## Module map
//!
//! - `balance`: Transactional 2:1 leaf refinement and rollback.
//! - `frontier`: Sparse staged frontier construction and publication specs.
//! - `input`: Observer demand, plan identity and refresh relevance.
//!
//! Reexports here define the supported surface; child modules hold its implementation.
//!

mod balance;
mod frontier;
mod input;

pub(super) use balance::block_sort_key;
pub(super) use frontier::{build_plan, initial_stage_for_plan, sparse_frontier_leaf_budget};
pub(super) use input::{
    ClipmapPlanTaskRelevance, derive_plan_input, plan_task_relevance, should_schedule_plan_refresh,
};
