//! Voxel demand, residency, and manifestation policy.

use serde::Deserialize;

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
    pub rebuild_budget_per_frame: Option<usize>,
    pub physics_interaction_radius_native: Option<f32>,
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
            generation_publish_budget_per_frame: 32,
            warm_inactive_materialization_limit: 512,
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

/// Runtime manifestation policy for derived voxel representations.
///
/// Presentation remains one-to-one with materialization surfaces. Collision
/// aggregation is a separate backend representation concern.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[serde(default)]
pub struct VoxelManifestationConfig {
    pub rebuild_budget_per_frame: usize,
    pub physics_interaction_radius_native: f32,
}

impl Default for VoxelManifestationConfig {
    fn default() -> Self {
        Self {
            rebuild_budget_per_frame: 32,
            physics_interaction_radius_native: 32.0,
        }
    }
}

impl VoxelManifestationConfig {
    pub(super) fn apply_overrides(&mut self, overrides: &VoxelManifestationConfigOverrides) {
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
        Ok(())
    }
}

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

fn require_positive(value: usize, field: &str) -> Result<(), String> {
    if value == 0 {
        Err(format!("{field} must be > 0"))
    } else {
        Ok(())
    }
}

fn validate_generation_group_edge(value: i32, field: &str) -> Result<(), String> {
    const MATERIALIZATION_CHUNKS_PER_USF_DIGIT: i32 = 100;

    if value <= 0
        || value > MATERIALIZATION_CHUNKS_PER_USF_DIGIT
        || MATERIALIZATION_CHUNKS_PER_USF_DIGIT % value != 0
    {
        return Err(format!(
            "{field} must be a positive divisor of {MATERIALIZATION_CHUNKS_PER_USF_DIGIT}; got {value}"
        ));
    }
    Ok(())
}
