//! Voxel-owned runtime policy.

use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(default)]
pub struct VoxelConfig {
    pub streaming: VoxelStreamingConfig,
    pub manifestation: VoxelManifestationConfig,
}

impl Default for VoxelConfig {
    fn default() -> Self {
        Self {
            streaming: VoxelStreamingConfig::default(),
            manifestation: VoxelManifestationConfig::default(),
        }
    }
}

impl VoxelConfig {
    pub(super) fn apply_overrides(&mut self, overrides: &VoxelConfigOverrides) {
        self.streaming.apply_overrides(&overrides.streaming);
        self.manifestation.apply_overrides(&overrides.manifestation);
    }

    pub(super) fn validate(&self) -> Result<(), String> {
        self.streaming.validate()?;
        self.manifestation.validate()
    }
}

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
    fn apply_overrides(&mut self, overrides: &VoxelStreamingConfigOverrides) {
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

    fn validate(self) -> Result<(), String> {
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
        validate_aligned_group_edge(
            self.generation_group_base_chunks_per_axis,
            "voxel.streaming.generation_group_base_chunks_per_axis",
        )
    }
}

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
    fn apply_overrides(&mut self, overrides: &VoxelManifestationConfigOverrides) {
        self.grouping.apply_overrides(&overrides.grouping);
        if let Some(value) = overrides.rebuild_budget_per_frame {
            self.rebuild_budget_per_frame = value;
        }
        if let Some(value) = overrides.physics_interaction_radius_native {
            self.physics_interaction_radius_native = value;
        }
    }

    fn validate(self) -> Result<(), String> {
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

fn require_positive(value: usize, field: &str) -> Result<(), String> {
    if value == 0 {
        Err(format!("{field} must be > 0"))
    } else {
        Ok(())
    }
}

fn validate_aligned_group_edge(value: i32, field: &str) -> Result<(), String> {
    const MATERIALIZATION_ATOMS_PER_USF_DIGIT: i32 = 100;

    if value <= 0
        || value > MATERIALIZATION_ATOMS_PER_USF_DIGIT
        || MATERIALIZATION_ATOMS_PER_USF_DIGIT % value != 0
    {
        return Err(format!(
            "{field} must be one of the positive divisors of 100 \
             (for example 1, 2, 4, 5, 10, 20, 25, 50, 100); got {value}"
        ));
    }
    Ok(())
}
