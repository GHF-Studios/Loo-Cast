//! Voxel demand, residency and background-generation policy.

use serde::Deserialize;

use super::{require_positive, validate_generation_group_edge};
use super::overrides::VoxelStreamingConfigOverrides;

/// Demand/residency and background-generation policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(default)]
pub struct VoxelStreamingConfig {
    pub default_load_budget_per_frame: usize,
    pub generation_publish_budget_per_frame: usize,
    pub warm_inactive_materialization_limit: usize,
    pub generation_group_base_chunks_per_axis: i32,
    pub max_chunks_per_generation_task: usize,
}

impl Default for VoxelStreamingConfig {
    fn default() -> Self {
        Self {
            default_load_budget_per_frame: 24,
            generation_publish_budget_per_frame: 16,
            warm_inactive_materialization_limit: 4096,
            generation_group_base_chunks_per_axis: 10,
            max_chunks_per_generation_task: 4,
        }
    }
}

impl VoxelStreamingConfig {
    pub(super) fn apply_overrides(&mut self, overrides: &VoxelStreamingConfigOverrides) {
        if let Some(value) = overrides.default_load_budget_per_frame {
            self.default_load_budget_per_frame = value;
        }
        if let Some(value) = overrides.generation_publish_budget_per_frame {
            self.generation_publish_budget_per_frame = value;
        }
        if let Some(value) = overrides.warm_inactive_materialization_limit {
            self.warm_inactive_materialization_limit = value;
        }
        if let Some(value) = overrides.generation_group_base_chunks_per_axis {
            self.generation_group_base_chunks_per_axis = value;
        }
        if let Some(value) = overrides.max_chunks_per_generation_task {
            self.max_chunks_per_generation_task = value;
        }
    }

    pub(super) fn validate(self) -> Result<(), String> {
        require_positive(
            self.default_load_budget_per_frame,
            "voxel.streaming.default_load_budget_per_frame",
        )?;
        require_positive(
            self.generation_publish_budget_per_frame,
            "voxel.streaming.generation_publish_budget_per_frame",
        )?;
        require_positive(
            self.max_chunks_per_generation_task,
            "voxel.streaming.max_chunks_per_generation_task",
        )?;
        validate_generation_group_edge(
            self.generation_group_base_chunks_per_axis,
            "voxel.streaming.generation_group_base_chunks_per_axis",
        )
    }
}
