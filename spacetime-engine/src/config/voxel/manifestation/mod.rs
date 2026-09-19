//! Runtime grouping, rebuild and physics policy for voxel manifestations.

use serde::Deserialize;

use super::{require_positive, validate_aligned_group_edge};
use super::overrides::{
    VoxelManifestationConfigOverrides, VoxelManifestationGroupingConfigOverrides,
};

/// Runtime manifestation policy for derived voxel representations.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[serde(default)]
pub struct VoxelManifestationConfig {
    pub grouping: VoxelManifestationGroupingConfig,
    pub rebuild_budget_per_frame: usize,
    pub physics_interaction_radius_native: f32,
}

impl Default for VoxelManifestationConfig {
    fn default() -> Self {
        Self {
            grouping: VoxelManifestationGroupingConfig::default(),
            rebuild_budget_per_frame: 8,
            physics_interaction_radius_native: 32.0,
        }
    }
}

impl VoxelManifestationConfig {
    pub(super) fn apply_overrides(&mut self, overrides: &VoxelManifestationConfigOverrides) {
        self.grouping.apply_overrides(&overrides.grouping);
        if let Some(value) = overrides.rebuild_budget_per_frame {
            self.rebuild_budget_per_frame = value;
        }
        if let Some(value) = overrides.physics_interaction_radius_native {
            self.physics_interaction_radius_native = value;
        }
    }

    pub(super) fn validate(self) -> Result<(), String> {
        require_positive(
            self.rebuild_budget_per_frame,
            "voxel.manifestation.rebuild_budget_per_frame",
        )?;
        if !self.physics_interaction_radius_native.is_finite()
            || self.physics_interaction_radius_native <= 0.0
        {
            return Err(
                "voxel.manifestation.physics_interaction_radius_native must be finite and > 0"
                    .into(),
            );
        }
        self.grouping.validate()
    }
}

/// Strategy used to partition virtual surface patches into manifestations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Default)]
pub enum VoxelGroupingStrategy {
    #[default]
    AlignedRegions,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(default)]
pub struct VoxelManifestationGroupingConfig {
    pub strategy: VoxelGroupingStrategy,
    pub base_chunks_per_axis: i32,
}

impl Default for VoxelManifestationGroupingConfig {
    fn default() -> Self {
        Self {
            strategy: VoxelGroupingStrategy::AlignedRegions,
            base_chunks_per_axis: 10,
        }
    }
}

impl VoxelManifestationGroupingConfig {
    fn apply_overrides(&mut self, overrides: &VoxelManifestationGroupingConfigOverrides) {
        if let Some(value) = overrides.strategy {
            self.strategy = value;
        }
        if let Some(value) = overrides.base_chunks_per_axis {
            self.base_chunks_per_axis = value;
        }
    }

    fn validate(self) -> Result<(), String> {
        match self.strategy {
            VoxelGroupingStrategy::AlignedRegions => validate_aligned_group_edge(
                self.base_chunks_per_axis,
                "voxel.manifestation.grouping.base_chunks_per_axis",
            ),
        }
    }
}
