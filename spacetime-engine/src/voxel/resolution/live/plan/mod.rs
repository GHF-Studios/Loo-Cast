//! Balanced binary frontier planning from semantic field evidence.

mod balance;
mod frontier;
mod input;

pub(super) use balance::block_sort_key;
pub(super) use frontier::{build_plan, initial_stage_for_plan, sparse_frontier_leaf_budget};
pub(super) use input::{
    ClipmapPlanTaskRelevance, derive_plan_input, plan_task_relevance, should_schedule_plan_refresh,
};
