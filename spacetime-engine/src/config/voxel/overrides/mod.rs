//! Typed runtime/developer overrides for voxel policy.

use super::manifestation::VoxelGroupingStrategy;

#[derive(Debug, Clone, Default)]
pub struct VoxelConfigOverrides {
    pub streaming: VoxelStreamingConfigOverrides,
    pub manifestation: VoxelManifestationConfigOverrides,
}

#[derive(Debug, Clone, Default)]
pub struct VoxelStreamingConfigOverrides {
    pub default_load_budget_per_frame: Option<usize>,
    pub generation_publish_budget_per_frame: Option<usize>,
    pub warm_inactive_materialization_limit: Option<usize>,
    pub generation_group_base_chunks_per_axis: Option<i32>,
    pub max_chunks_per_generation_task: Option<usize>,
}

#[derive(Debug, Clone, Default)]
pub struct VoxelManifestationConfigOverrides {
    pub grouping: VoxelManifestationGroupingConfigOverrides,
    pub rebuild_budget_per_frame: Option<usize>,
    pub physics_interaction_radius_native: Option<f32>,
}

#[derive(Debug, Clone, Default)]
pub struct VoxelManifestationGroupingConfigOverrides {
    pub strategy: Option<VoxelGroupingStrategy>,
    pub base_chunks_per_axis: Option<i32>,
}
